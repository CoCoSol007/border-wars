//! A library containing a client to use a relay server.

use std::borrow::Cow;
use std::fs;
use std::io::{self};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

use log::warn;
use mio::net::TcpStream;
use rand::seq::SliceRandom;
use tungstenite::handshake::MidHandshake;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{ClientHandshake, HandshakeError, Message, WebSocket};
use uuid::Uuid;

/// The state of a [Connection].
#[derive(Debug)]
enum ConnectionState {
    /// The [Connection] is not connected.
    Disconnected,

    /// The underlying [TcpStream] is connecting.
    Connecting(TcpStream, Instant),

    /// The underlying [TcpStream] is connected.
    Connected(TcpStream),

    /// The websocket handshake is in progress.
    Handshaking(MidHandshake<ClientHandshake<MaybeTlsStream<TcpStream>>>),

    /// The websocket handshake is finished.
    Handshaked(WebSocket<MaybeTlsStream<TcpStream>>),

    /// The [Connection] is registering with the relay server.
    Registering(WebSocket<MaybeTlsStream<TcpStream>>),

    /// The [Connection] is connected.
    Active(WebSocket<MaybeTlsStream<TcpStream>>),
}

/// A connection to a relay server.
pub struct Connection {
    /// The address list corresponding to the relay server.
    address_list: Vec<SocketAddr>,

    /// The domain of the relay server.
    domain: String,

    /// The path to the file where the identifier and secret key are stored.
    data_path: PathBuf,

    /// The identifier of the connection for the relay server.
    identifier: Option<Uuid>,

    /// The secret key used to authenticate with the relay server.
    secret: Option<Uuid>,

    /// The receiver part of the send channel.
    ///
    /// This is used in [Connection::update] to get messages that need to
    /// be sent to the relay server.
    send_receiver: Receiver<Message>,

    /// The sender part of the receive channel.
    ///
    /// This is used in [Connection::send] to store messages that need to
    /// be sent to the relay server.
    send_sender: Sender<Message>,

    /// The receiver part of the receive channel.
    ///
    /// This is used in [Connection::read] to get messages that have been
    /// received from the relay server.
    receive_receiver: Receiver<(Uuid, Vec<u8>)>,

    /// The sender part of the send channel.
    ///
    /// This is used in [Connection::update] to store messages that have
    /// been received from the relay server.
    receive_sender: Sender<(Uuid, Vec<u8>)>,

    /// The state of the connection.
    state: ConnectionState,
}

impl Connection {
    /// Create a new [Connection].
    pub fn new<'a>(domain: impl Into<Cow<'a, str>>) -> io::Result<Self> {
        let domain = domain.into();

        // Loads the identifier and secret key from disk.
        let (data_path, identifier, secret) = {
            // Find the relay data file path.
            let mut path = home::home_dir().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "could not find home directory")
            })?;
            path.push(".relay-data");

            // Check if the file exists.
            match path.exists() {
                true => {
                    // Read the file and parse the identifier and secret key.
                    let contents = fs::read(&path)?;
                    if contents.len() != 32 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "invalid data in .relay-data",
                        ));
                    }
                    let identifier = Uuid::from_slice(&contents[..16]).map_err(io::Error::other)?;
                    let secret = Uuid::from_slice(&contents[16..]).map_err(io::Error::other)?;
                    (path, Some(identifier), Some(secret))
                }
                false => (path, None, None),
            }
        };

        // Create the communication channels.
        let (send_sender, send_receiver) = channel();
        let (receive_sender, receive_receiver) = channel();

        // Create the connection and return it.
        Ok(Self {
            address_list: (domain.as_ref(), 443).to_socket_addrs()?.collect(),
            domain: domain.into_owned(),
            data_path,
            identifier,
            secret,
            send_receiver,
            send_sender,
            receive_receiver,
            receive_sender,
            state: ConnectionState::Disconnected,
        })
    }

    /// Get the identifier of the connection.
    pub const fn identifier(&self) -> Option<Uuid> {
        self.identifier
    }

    /// Send a message to the target client.
    pub fn send<'a>(&self, target_id: Uuid, message: impl Into<Cow<'a, [u8]>>) {
        let mut data = message.into().into_owned();
        data.extend_from_slice(target_id.as_bytes());
        self.send_sender.send(Message::Binary(data)).ok();
    }

    /// Receive a message from the relay connection.
    pub fn read(&self) -> Option<(Uuid, Vec<u8>)> {
        self.receive_receiver.try_recv().ok()
    }

    /// Create a new [TcpStream] to the relay server.
    fn create_stream(&mut self) -> ConnectionState {
        // Take a random relay address.
        let Some(address) = self.address_list.choose(&mut rand::thread_rng()) else {
            warn!("no relay address available");
            return ConnectionState::Disconnected;
        };

        // Create the new TCP stream.
        match TcpStream::connect(address.to_owned()) {
            Ok(stream) => ConnectionState::Connecting(stream, Instant::now()),
            Err(e) => {
                warn!("failed to start connection to the relay server: {e}");
                ConnectionState::Disconnected
            }
        }
    }

    /// Check if the [TcpStream] of the [Connection] is connected.
    fn check_connection(&mut self, stream: TcpStream, start: Instant) -> ConnectionState {
        // Check for connection errors.
        if let Err(e) = stream.take_error() {
            warn!("failed to connect to the relay server: {e}");
            return ConnectionState::Disconnected;
        }

        // Check if the stream is connected.
        let connected = match stream.peek(&mut [0]) {
            Ok(_) => true,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => true,
            Err(ref e) if e.kind() == io::ErrorKind::NotConnected => false,
            Err(e) => {
                warn!("failed to connect to the relay server: {e}");
                return ConnectionState::Disconnected;
            }
        };

        // Check if the connection has timed out.
        let elapsed = start.elapsed();
        if elapsed > Duration::from_secs(5) {
            warn!("connection to the relay server timed out");
            return ConnectionState::Disconnected;
        }

        // Update the connection state if connected.
        match connected {
            true => ConnectionState::Connected(stream),
            false => ConnectionState::Connecting(stream, start),
        }
    }

    /// Start the websocket handshake.
    fn start_handshake(&mut self, stream: TcpStream) -> ConnectionState {
        match tungstenite::client_tls(format!("wss://{}", self.domain), stream) {
            Ok((socket, _)) => ConnectionState::Handshaked(socket),
            Err(HandshakeError::Interrupted(handshake)) => ConnectionState::Handshaking(handshake),
            Err(HandshakeError::Failure(e)) => {
                warn!("handshake failed with the relay server: {e}");
                ConnectionState::Disconnected
            }
        }
    }

    /// Continue the websocket handshake.
    fn continue_handshake(
        &mut self,
        handshake: MidHandshake<ClientHandshake<MaybeTlsStream<TcpStream>>>,
    ) -> ConnectionState {
        match handshake.handshake() {
            Ok((socket, _)) => ConnectionState::Handshaked(socket),
            Err(HandshakeError::Interrupted(handshake)) => ConnectionState::Handshaking(handshake),
            Err(HandshakeError::Failure(e)) => {
                warn!("handshake failed with the relay server: {e}");
                ConnectionState::Disconnected
            }
        }
    }

    /// Start authentication with the relay server.
    fn start_authentication(
        &mut self,
        mut socket: WebSocket<MaybeTlsStream<TcpStream>>,
    ) -> ConnectionState {
        match (self.identifier, self.secret) {
            (Some(identifier), Some(secret)) => {
                // Create the authentication message.
                let mut data = Vec::with_capacity(32);
                data.extend(identifier.as_bytes());
                data.extend(secret.as_bytes());

                // Send the authentication message.
                match socket.send(Message::Binary(data)) {
                    Ok(()) => ConnectionState::Active(socket),
                    Err(e) => {
                        warn!("failed to send authentication message: {e}");
                        ConnectionState::Disconnected
                    }
                }
            }
            _ => {
                // Send empty authentication message to request a new identifier and secret key.
                match socket.send(Message::Binary(vec![])) {
                    Ok(()) => ConnectionState::Registering(socket),
                    Err(e) => {
                        warn!("failed to send registration message: {e}");
                        ConnectionState::Disconnected
                    }
                }
            }
        }
    }

    /// Wait for the registration response.
    fn get_registration_response(
        &mut self,
        mut socket: WebSocket<MaybeTlsStream<TcpStream>>,
    ) -> ConnectionState {
        match socket.read() {
            Ok(message) => {
                // Check the message length.
                let data = message.into_data();
                if data.len() != 32 {
                    warn!("received malformed registration response");
                    return ConnectionState::Disconnected;
                }

                // Extract the client identifier and secret.
                self.identifier = Some(Uuid::from_slice(&data[..16]).expect("invalid identifier"));
                self.secret = Some(Uuid::from_slice(&data[16..]).expect("invalid secret"));

                // Save the client identifier and secret.
                fs::write(&self.data_path, data).ok();

                // Activate the connection.
                ConnectionState::Active(socket)
            }
            Err(tungstenite::Error::Io(ref e))
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::Interrupted =>
            {
                ConnectionState::Registering(socket)
            }
            Err(e) => {
                warn!("failed to receive registration response: {e}");
                ConnectionState::Disconnected
            }
        }
    }

    /// Update the [Connection] by receiving and sending messages.
    fn update_connection(
        &mut self,
        mut socket: WebSocket<MaybeTlsStream<TcpStream>>,
    ) -> ConnectionState {
        // Send messages from the send channel to the socket.
        while let Ok(message) = self.send_receiver.try_recv() {
            match socket.send(message) {
                Ok(()) => (),
                Err(tungstenite::Error::Io(ref e))
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::Interrupted =>
                {
                    break;
                }
                Err(e) => {
                    warn!("relay connection closed: {e}");
                    return ConnectionState::Disconnected;
                }
            }
        }

        // Receive messages from the socket and send them to the receive channel.
        loop {
            match socket.read() {
                Ok(message) => {
                    // Check the message length.
                    let mut data = message.into_data();
                    if data.len() < 16 {
                        warn!("received malformed message with length: {}", data.len());
                        continue;
                    }

                    // Extract the sender ID.
                    let id_start = data.len() - 16;
                    let sender_id = Uuid::from_slice(&data[id_start..]).expect("invalid sender id");
                    data.truncate(id_start);

                    // Send the message to the receive channel.
                    self.receive_sender.send((sender_id, data)).ok();
                }
                Err(tungstenite::Error::Io(ref e))
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::Interrupted =>
                {
                    break;
                }
                Err(e) => {
                    warn!("relay connection closed: {e}");
                    return ConnectionState::Disconnected;
                }
            }
        }

        // Keep the connection connected.
        ConnectionState::Active(socket)
    }

    /// Update the [Connection].
    ///
    /// This function will connect to the relay server if it's not already
    /// connected, and will send and receive messages from the relay server
    /// if it's connected.
    ///
    /// This function will not block the current thread.
    pub fn update(&mut self) {
        self.state = match std::mem::replace(&mut self.state, ConnectionState::Disconnected) {
            ConnectionState::Disconnected => self.create_stream(),
            ConnectionState::Connecting(stream, start) => self.check_connection(stream, start),
            ConnectionState::Connected(stream) => self.start_handshake(stream),
            ConnectionState::Handshaking(handshake) => self.continue_handshake(handshake),
            ConnectionState::Handshaked(socket) => self.start_authentication(socket),
            ConnectionState::Registering(socket) => self.get_registration_response(socket),
            ConnectionState::Active(socket) => self.update_connection(socket),
        }
    }
}
