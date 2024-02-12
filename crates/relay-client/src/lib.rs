//! A library containing a client to use a relay server.

use std::borrow::Cow;
use std::io::{self};
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

use log::warn;
use mio::net::TcpStream;
use rand::seq::SliceRandom;
use tungstenite::handshake::MidHandshake;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{ClientHandshake, HandshakeError, Message, WebSocket};

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

    /// The [Connection] is connected.
    Active(WebSocket<MaybeTlsStream<TcpStream>>),
}

/// A connection to a relay server.
pub struct Connection {
    /// The address list corresponding to the relay server.
    address_list: Vec<SocketAddr>,

    /// The domain of the relay server.
    domain: String,

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
    receive_receiver: Receiver<(u32, Vec<u8>)>,

    /// The sender part of the send channel.
    ///
    /// This is used in [Connection::update] to store messages that have
    /// been received from the relay server.
    receive_sender: Sender<(u32, Vec<u8>)>,

    /// The state of the connection.
    state: ConnectionState,
}

impl Connection {
    /// Create a new [Connection].
    pub fn new<'a>(domain: impl Into<Cow<'a, str>>) -> io::Result<Self> {
        let domain = domain.into();
        let (send_sender, send_receiver) = channel();
        let (receive_sender, receive_receiver) = channel();
        Ok(Self {
            address_list: (domain.as_ref(), 443).to_socket_addrs()?.collect(),
            domain: domain.into_owned(),
            send_receiver,
            send_sender,
            receive_receiver,
            receive_sender,
            state: ConnectionState::Disconnected,
        })
    }

    /// Send a message to the target client.
    pub fn send(&self, target_id: u32, message: Cow<[u8]>) {
        let mut data = message.into_owned();
        data.extend_from_slice(&target_id.to_be_bytes());
        self.send_sender.send(Message::Binary(data)).ok();
    }

    /// Receive a message from the target client.
    pub fn read(&self) -> Option<(u32, Vec<u8>)> {
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
            Ok((socket, _)) => ConnectionState::Active(socket),
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
            Ok((socket, _)) => ConnectionState::Active(socket),
            Err(HandshakeError::Interrupted(handshake)) => ConnectionState::Handshaking(handshake),
            Err(HandshakeError::Failure(e)) => {
                warn!("handshake failed with the relay server: {e}");
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
                    if data.len() < 4 {
                        warn!("received malformed message with length: {}", data.len());
                        continue;
                    }

                    // Extract the sender ID.
                    let id_start = data.len() - 4;
                    let sender_id = u32::from_be_bytes(
                        data[id_start..]
                            .try_into()
                            .unwrap_or_else(|_| unreachable!()),
                    );
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
            ConnectionState::Active(socket) => self.update_connection(socket),
        }
    }
}
