//!
//! Provides the lowest layer abstractions for serial and IP communication.
//!
//! The abstractions are simple, and minimal:
//!
//! - [`Message`]; a type-safe wrapper around a byte buffer.
//! - [`Transport`]; a trait defining the interface for transport connections. Transport
//!   implementations are responsible for managing the underlying connection and implementing the
//!   standard `Read` and `Write` traits.
//! - [`connect`] ; a function that establishes a transport connection based on the provided
//!   configuration.
//!
//!
//! # Example
//!
//! The following example demonstrates how to establish a transport connection using a serial
//! connection. The connection configuration is parsed from a string representation, commonly
//! provided as a command-line argument or environment variable.
//!
//! ```rust,no_run
//! use rfham_config::connections::{Connection, SerialConnection};
//! use rfham_rigs::transport::connect;
//!
//! let conn: Connection = SerialConnection::from_str(
//!     "/dev/cu.usbserial-A10KMJZB:38400;stop-bits=Two",
//! ).unwrap().into();
//!
//! let transport = rfham_rigs::transport::connect(&conn).unwrap();
//! println!("Transport connected ({transport:?})");
//! ```
//!
//! The following example demonstrates how to send a message using the current transport connection.
//! The message is constructed using the `Message::new` method, from a byte string and corresponds
//! to the CAT no-op command. Generally clients use the [`protocol`](crate::protocol) layer for
//! constructing and parsing protocol-specific messages.
//!
//! ```rust,no_run
//! # fn get_current_transport() -> impl Transport {
//! #     unimplemented!()
//! # }
//! use rfham_rigs::transport::message::Message;
//! let message = Message::new(b";"); // Common CAT 'No-op' command.
//! println!("Sending: {message:#}"); // "Sending: Message [ ;]"
//!
//! let transport = get_current_transport();
//! transport.write_message(&message).unwrap();
//! ```
//!

use crate::error::RigError;
use rfham_config::connections::{Connection, Host, IpConnection, SerialConnection};
use serialport::{
    DataBits, Error as SerialError, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits,
};
use std::{
    fmt::Debug,
    io::{Error as IoError, ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::Mutex,
    time::Duration,
};
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This is the interface provided by the transport layer for communication with connected devices.
///
/// The transport implements `Read` and `Write` traits so clients can perform standard I/O
/// operations directly against it, although the higher-level message functions are preferred.
///
pub trait Transport: Debug + Read + Write {
    ///
    /// Writes a message to the transport.
    ///
    /// This method takes a reference to a `Message` and writes its bytes to the underlying
    /// transport. It uses the `Write` trait's `write_all` method to ensure the entire message is
    /// written, and then flushes the transport.
    ///
    fn write_message(&mut self, message: &message::Message<'_>) -> std::io::Result<()> {
        self.write_all(message.as_bytes())?;
        self.flush()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Connect, and return, a transport described in a [`Connection`] object.
///
/// # Example
///
/// ```rust
/// use rfham_rigs::transport::connect;
/// use rfham_config::connections::{CBaudRate, onnection, SerialConnection};
///
/// let connection: Connection = Connection::Serial(
///     SerialConnection::new(
///         "/dev/ttyUSB0",
///         BaudRate::Bd38400
///     )
/// );
/// let transport = connect(&connection);
/// ```
///
pub fn connect(config: &Connection) -> Result<impl Transport, RigError> {
    Ok(ConnectedTransport::new(ConnectedTransportKind::connect(
        config,
    )?))
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct ConnectedTransport(Mutex<ConnectedTransportKind>);

#[derive(Debug, EnumIs, EnumTryAs)]
enum ConnectedTransportKind {
    Serial { port: Box<dyn SerialPort> },
    Ip { stream: TcpStream },
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const DEFAULT_SERIAL_TIMEOUT: Duration = Duration::from_millis(200);
const DEFAULT_IP_CONNECT_TIMEOUT: Duration = Duration::new(15, 0);

impl Read for ConnectedTransportKind {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Serial { port } => port.read(buf),
            Self::Ip { stream } => stream.read(buf),
        }
    }
}

impl Write for ConnectedTransportKind {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Serial { port } => port.write(buf),
            Self::Ip { stream } => stream.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Serial { port } => port.flush(),
            Self::Ip { stream } => stream.flush(),
        }
    }
}

impl ConnectedTransportKind {
    fn connect(connection: &Connection) -> Result<Self, RigError> {
        Ok(match connection {
            Connection::Serial(conn) => {
                let port = to_serial_port(conn)?;

                Self::Serial { port }
            }
            Connection::Ip(conn) => {
                if let Some(addr) = to_socket_address(conn)? {
                    let timeout = conn.connect_timeout().unwrap_or(DEFAULT_IP_CONNECT_TIMEOUT);
                    let stream = TcpStream::connect_timeout(&addr, timeout)?;
                    stream.set_read_timeout(conn.read_timeout())?;
                    stream.set_write_timeout(conn.write_timeout())?;
                    // stream.set_keepalive(true)? #[feature()] nightly
                    Self::Ip { stream }
                } else {
                    return Err(log_rig_error!(SocketAddress => socket_addr: conn.to_string()));
                }
            }
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl Read for ConnectedTransport {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        inner.read(buf)
    }
}

impl Write for ConnectedTransport {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        inner.flush()
    }
}

impl Transport for ConnectedTransport {}

impl ConnectedTransport {
    fn new(inner: ConnectedTransportKind) -> Self {
        Self(Mutex::new(inner))
    }

    #[allow(dead_code)]
    fn read_all(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        let expected = buf.len();
        let mut total_read = 0;
        while total_read < expected {
            match inner.read(&mut buf[total_read..]) {
                Ok(0) => break,
                Ok(n) => total_read += n,
                Err(e) => return Err(e),
            }
        }
        Ok(total_read)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn to_socket_address(conn: &IpConnection) -> Result<Option<SocketAddr>, IoError> {
    match conn.host() {
        Host::HostName(name) => (name.as_str(), conn.port())
            .to_socket_addrs()
            .map(|addrs| addrs.into_iter().next()),
        Host::Address(addr) => (*addr, conn.port())
            .to_socket_addrs()
            .map(|addrs| addrs.into_iter().next()),
    }
}

fn to_serial_port_builder(conn: &SerialConnection) -> SerialPortBuilder {
    serialport::new(conn.path().display().to_string(), conn.baud_rate() as u32)
        .data_bits(conn.data_bits().unwrap_or(DataBits::Eight))
        .flow_control(conn.flow_control().unwrap_or(FlowControl::None))
        .parity(conn.parity().unwrap_or(Parity::None))
        .stop_bits(conn.stop_bits().unwrap_or(StopBits::One))
        .timeout(conn.io_timeout().unwrap_or(DEFAULT_SERIAL_TIMEOUT))
}

fn to_serial_port(conn: &SerialConnection) -> Result<Box<dyn SerialPort>, SerialError> {
    to_serial_port_builder(conn).open()
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod message;
pub use message::Message;
