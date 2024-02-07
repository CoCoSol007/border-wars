//! A networking library for Bevy.

use std::collections::LinkedList;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes128Gcm, Key, Nonce};

/// A non-blocking tcp connection.
///
/// # Example
///
/// ```rust
/// use std::io;
///
/// use bevnet::{Connection, Listener};
///
/// # fn main() -> io::Result<()> {
/// let secret_key = Connection::generate_key();
/// let listener = Listener::bind("127.0.0.1:23732", &secret_key)?;
/// let mut connection = Connection::connect("127.0.0.1:23732", &secret_key)?;
///
/// // The accept operation is not blocking. So we need to loop here.
/// let mut server_connection;
/// loop {
///     if let Some(new_connection) = listener.accept()? {
///         server_connection = new_connection;
///         break;
///     }
/// }
///
/// // We don't need to loop here because the send operation just appends to the send buffer.
/// connection.send(b"Hello, world!")?;
///
/// // To be sure the message has been sent, we need to update the connection.
/// while !connection.update()? {
///     // Wait until the connection is updated.
/// }
///
/// // The receive operation is not blocking. So we need to loop here.
/// loop {
///     if let Some(message) = server_connection.receive()? {
///         assert_eq!(message, b"Hello, world!");
///         break;
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct Connection {
    /// The underlying [TcpStream] used for the connection.
    stream: TcpStream,

    /// Contains the buffers that are not yet being sent.
    send_buffers: LinkedList<(usize, Vec<u8>)>,

    /// The length of the next message to be received.
    ///
    /// `None` if the message length is not yet received.
    receive_message_len: Option<u16>,

    /// The nonce used for encryption.
    receive_message_nonce: Option<Vec<u8>>,

    /// The length of the received byte block.
    ///
    /// Used by [Connection::receive_partial] to determine if the block is
    /// complete.
    receive_filled_len: usize,

    /// The buffer used to receive a byte block.
    receive_buffer: Vec<u8>,

    /// The secret key used for encryption.
    secret_key: Aes128Gcm,
}

impl Connection {
    /// Generates a new secret key.
    pub fn generate_key() -> [u8; 16] {
        Aes128Gcm::generate_key(OsRng).into()
    }

    /// Creates a new [Connection] from a [TcpStream].
    fn new(stream: TcpStream, secret_key: &Key<Aes128Gcm>) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Self {
            stream,
            send_buffers: LinkedList::new(),
            receive_message_len: None,
            receive_message_nonce: None,
            receive_filled_len: 0,
            receive_buffer: Vec::new(),
            secret_key: Aes128Gcm::new(secret_key),
        })
    }

    /// Creates a new [Connection] that connects to the given address.
    ///
    /// This function is blocking.
    pub fn connect(address: impl ToSocketAddrs, secret_key: &[u8; 16]) -> io::Result<Self> {
        Self::new(TcpStream::connect(address)?, secret_key.into())
    }

    /// Sends a message over the connection.
    ///
    /// Returns `true` if the message has been sent directly and `false`
    /// if the message is still in the send queue.
    ///
    /// This function is not blocking.
    pub fn send(&mut self, message: &[u8]) -> io::Result<bool> {
        // Encrypt the message.
        let nonce = Aes128Gcm::generate_nonce(OsRng);
        let message = self.secret_key.encrypt(&nonce, message).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to encrypt message: {}", e),
            )
        })?;

        // Get the length of the message as a u16.
        let message_len: u16 = match message.len().try_into() {
            Ok(len) => len,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("message length is too large: {}", message.len()),
                ));
            }
        };

        // Add a new buffer to the send queue.
        self.send_buffers
            .push_back((0, message_len.to_ne_bytes().to_vec()));
        self.send_buffers.push_back((0, nonce.to_vec()));
        self.send_buffers.push_back((0, message));

        // Update the connection.
        self.update()
    }

    /// Updates the connection.
    ///
    /// This function sends any pending messages that have not been sent yet.
    /// It returns `true` if there is no remaining data to send after updating
    /// the connection and `false` otherwise.
    ///
    /// This function is not blocking.
    pub fn update(&mut self) -> io::Result<bool> {
        // Looping over the send buffers.
        while let Some((offset, buffer)) = self.send_buffers.front_mut() {
            // Writing the buffer to the stream.
            match self.stream.write(&buffer[*offset..]) {
                Ok(n) => *offset += n,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }

            // Removing the buffer if it is fully sent.
            if *offset >= buffer.len() {
                self.send_buffers.pop_front();
            }
        }

        // Returning success.
        Ok(self.send_buffers.is_empty())
    }

    /// Receives a byte block from the connection.
    ///
    /// This function fills the receive buffer and returns `true` if the
    /// buffer is successfully filled with `len` bytes and `false` if the
    /// buffer is not filled yet.
    ///
    /// This function mustn't be called for different byte block sequentially
    /// because the function can only process one byte block at a time.
    ///
    /// This function is not blocking.
    fn receive_partial(&mut self, len: u16) -> io::Result<bool> {
        let len = len as usize;

        // Resizing the buffer if it is not large enough.
        if self.receive_buffer.len() < len {
            self.receive_buffer.resize(len, 0);
        }

        // Checking if the buffer is already filled.
        if self.receive_filled_len >= len {
            self.receive_filled_len = 0;
            return Ok(true);
        }

        // Reading from the stream.
        let start_index = self.receive_filled_len;
        let receive_buffer = &mut self.receive_buffer[start_index..start_index + len];
        let received_len = self.stream.read(receive_buffer);
        self.receive_filled_len += match received_len {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "connection closed by remote peer",
                ));
            }
            Ok(n) => n,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(false),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => return Ok(false),
            Err(e) => return Err(e),
        };

        // Checking if the buffer is filled.
        if self.receive_filled_len >= len {
            self.receive_filled_len = 0;
            return Ok(true);
        }
        Ok(false)
    }

    /// Receives a message from the connection.
    ///
    /// If no message is available, returns `None`.
    ///
    /// This function is not blocking.
    pub fn receive(&mut self) -> io::Result<Option<Vec<u8>>> {
        // Receiving the message length.
        let message_len = match self.receive_message_len {
            Some(message_len) => message_len,
            None => {
                // If the message length is not received yet, return `None`.
                if !self.receive_partial(2)? {
                    return Ok(None);
                }

                // Setting the message length.
                let message_len =
                    u16::from_ne_bytes(self.receive_buffer[..2].try_into().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid message length")
                    })?);
                self.receive_message_len = Some(message_len);

                // Returning the message length.
                message_len
            }
        };

        if self.receive_message_nonce.is_none() {
            // If the nonce is not received yet, return `None`.
            if !self.receive_partial(12)? {
                return Ok(None);
            }

            // Setting the nonce.
            self.receive_message_nonce = Some(self.receive_buffer[..12].to_vec());
        }

        // Receiving the message.
        if !self.receive_partial(message_len)? {
            return Ok(None);
        }
        let message = &self.receive_buffer[..message_len as usize];

        // Getting the nonce.
        let nonce = self
            .receive_message_nonce
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing nonce"))?;

        // Decrypting the message.
        let message = self
            .secret_key
            .decrypt(Nonce::from_slice(nonce), message)
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("failed to decrypt message: {}", e),
                )
            })?;

        // Resetting the message length and nonce.
        self.receive_message_len = None;
        self.receive_message_nonce = None;

        // Returning the message.
        Ok(Some(message))
    }
}

/// A non-blocking tcp listener.
///
/// ```rust
/// use std::io;
///
/// use bevnet::{Connection, Listener};
///
/// # fn main() -> io::Result<()> {
/// let secret_key = Connection::generate_key();
/// let listener = Listener::bind("127.0.0.1:23732", &secret_key)?;
/// let mut connection = Connection::connect("127.0.0.1:23732", &secret_key)?;
///
/// // The accept operation is not blocking. So we need to loop here.
/// let mut server_connection;
/// loop {
///     if let Some(new_connection) = listener.accept()? {
///         server_connection = new_connection;
///         break;
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct Listener(TcpListener, Key<Aes128Gcm>);

impl Listener {
    /// Creates a new listener.
    pub fn bind(addr: impl ToSocketAddrs, secret_key: &[u8; 16]) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(Self(
            listener,
            Key::<Aes128Gcm>::from_slice(secret_key).to_owned(),
        ))
    }

    /// Accepts a new [Connection].
    ///
    /// This function is not blocking.
    pub fn accept(&self) -> io::Result<Option<Connection>> {
        match self.0.accept() {
            Ok((stream, _)) => Connection::new(stream, &self.1).map(Some),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => Ok(None),
            Err(e) => Err(e),
        }
    }
}
