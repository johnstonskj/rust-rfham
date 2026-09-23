//!
//! Provides the lowest layer abstractions for serial and IP communication.
//!
//! The abstractions are simple, and minimal:
//!
//! - [`Message`]; a type-safe wrapper around a byte buffer.
//! - [`Transport`]; a trait defining the interface for transport connections. Transport
//!   implementations are responsible for managing the underlying connection and implementing the
//!   standard `Read` and `Write` traits.
//!   - [`Statistics`]; a trait defining the interface for transport statistics.
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
//! # Logging
//! 
//! Note that each transport will log it's statistic periodically based on a configured count.
//! The count is based on the number of successful operations, every *n* messages sent or
//! received an *info* level log record is emitted with the transport identifier and statistics.
//! This count can be configured via the `RFHAM_LOG_TRANSPORT_STATS_COUNT` environment variable,
//! otherwise it's default is 100.
//! 
//! ```text
//! 2026-09-23T16:16:15Z [INFO] statistics /dev/cu.usbserial-A10KMJZB:38400, 4, 42, 0, 0, 6, 83, 0, 0
//! ```
//!

use crate::error::RigError;
use rfham_config::connections::{Connection, Host, IpConnection, SerialConnection};
use serialport::{
    DataBits, Error as SerialError, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits,
};
use std::{
    env,
    fmt::{Debug, Display},
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

    /// 
    /// Return a reference to the transport's statistics.
    ///
    /// This allows clients to query various metrics about the transport's usage, such as the
    /// number of commands sent, bytes transmitted, and any errors or timeouts encountered.
    /// 
    fn statistics(&self) -> impl Statistics;
}

pub trait Statistics {
    /// 
    /// Number of messages successfully sent.
    /// 
    fn messages_sent(&self) -> u64;
    /// 
    /// Number of bytes successfully sent.
    /// 
    fn bytes_sent(&self) -> u64;
    /// 
    /// Number of write timeouts encountered.
    /// 
    fn write_timeouts(&self) -> u64;
    /// 
    /// Number of write errors encountered.
    /// 
    fn write_errors(&self) -> u64;
    /// 
    /// Number of messages successfully received.
    /// 
    fn messages_received(&self) -> u64;
    /// 
    /// Number of bytes successfully received.
    /// 
    fn bytes_received(&self) -> u64;
    /// 
    /// Number of read timeouts encountered.
    /// 
    fn read_timeouts(&self) -> u64;
    /// 
    /// Number of read errors encountered.
    /// 
    fn read_errors(&self) -> u64;

    /// 
    /// Return a snapshot of the transport's statistics as an array of `u64` values.
    /// 
    /// This allows a client to quickly capture the current state of the transport's statistics
    /// without having to individually query each statistic. It also allows for metrics to be easily
    /// logged or transmitted for analysis.
    /// 
    /// The order of the values in the array is: `messages_sent`, `bytes_sent`, `write_timeouts`, 
    /// `write_errors`, `messages_received`, `bytes_received`, `read_timeouts`, `read_errors`.
    /// 
    fn snapshot(&self) -> [u64;8];
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
    Ok(ConnectedTransport::new(
        ConnectedTransportKind::connect(
            config,
        )?,
        match config {
            Connection::Serial(conn) => format!("{}:{}", conn.path().display(), conn.baud_rate()),
            Connection::Ip(conn) => match conn.host() {
                Host::HostName(name) => format!("{}:{}",name, conn.port()),
                Host::Address(addr) => format!("{}:{}", addr, conn.port()),
            },
        },
    ))
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct ConnectedTransport(Mutex<Inner>);

#[derive(Debug)]
struct Inner {
    conn: ConnectedTransportKind,
    label: String,
    stats: Stats,
}

#[derive(Debug, EnumIs, EnumTryAs)]
enum ConnectedTransportKind {
    Serial { port: Box<dyn SerialPort> },
    Ip { stream: TcpStream },
}

#[derive(Copy, Clone, Debug, Default)]
struct Stats {
    messages_sent: u64,
    bytes_sent: u64,
    write_timeouts: u64,
    write_errors: u64,
    messages_received: u64,
    bytes_received: u64,
    read_timeouts: u64,
    read_errors: u64,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const DEFAULT_SERIAL_TIMEOUT: Duration = Duration::from_millis(200);
const DEFAULT_IP_CONNECT_TIMEOUT: Duration = Duration::new(15, 0);

const LOG_TRANSPORT_STATS_COUNT: u64 = 100;

// ------------------------------------------------------------------------------------------------

impl Read for ConnectedTransport {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        match inner.conn.read(buf) {
            Ok(bytes_read) => {
                inner.stats.messages_received += 1;
                inner.stats.bytes_received += bytes_read as u64;
                self.log_statistics();
                Ok(bytes_read)
            } 
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                inner.stats.read_timeouts += 1;
                Err(e)
            }
            Err(e) => {
                inner.stats.read_errors += 1;
                Err(e)
            }
        }
    }
}

impl Write for ConnectedTransport {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        match inner.conn.write(buf) {
            Ok(bytes_written) => {
                inner.stats.messages_sent += 1;
                inner.stats.bytes_sent += bytes_written as u64;
                self.log_statistics();
                Ok(bytes_written)
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                inner.stats.write_timeouts += 1;
                Err(e)
            }
            Err(e) => {
                inner.stats.write_errors += 1;
                Err(e)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        inner.conn.flush()
    }
}

impl Transport for ConnectedTransport {
    fn statistics(&self) -> impl Statistics {
        let transport = self.0.lock().map_err(|_| ErrorKind::Other).unwrap();
        transport.stats
    }
}

impl ConnectedTransport {
    fn new(conn: ConnectedTransportKind, label: String) -> Self {
        Self(Mutex::new(Inner { conn, label, stats: Default::default() }))
    }

    #[allow(dead_code)]
    fn read_all(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().map_err(|_| ErrorKind::Other)?;
        let expected = buf.len();
        let mut total_read = 0;
        while total_read < expected {
            match inner.conn.read(&mut buf[total_read..]) {
                Ok(0) => break,
                Ok(n) => total_read += n,
                Err(e) if e.kind() == ErrorKind::TimedOut => {
                    inner.stats.read_timeouts += 1;
                    return Err(e);
                }
                Err(e) => {
                    inner.stats.read_errors += 1;
                    return Err(e);
                }
            }
        }
        inner.stats.bytes_received += total_read as u64;
        inner.stats.messages_received += 1;
        self.log_statistics();
        Ok(total_read)
    }

    fn log_statistics(&self) {
        let log_trace_count = env::var("RFHAM_LOG_TRANSPORT_STATS_COUNT")
            .map(|v| v.parse::<u64>().unwrap_or(LOG_TRANSPORT_STATS_COUNT))
            .unwrap_or(LOG_TRANSPORT_STATS_COUNT);
        let transport = self.0.lock().map_err(|_| ErrorKind::Other).unwrap();
        if (transport.stats.messages_received + transport.stats.messages_sent) % log_trace_count == 0 {
            tracing::info!(
                "statistics {}, {}", 
                transport.label, 
                transport.stats.snapshot()
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
}

// ------------------------------------------------------------------------------------------------

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

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Statistics {{ Read: {}, {}, {}, {}, Write: {}, {}, {}, {} }}",
            self.messages_received(),
            self.bytes_received(),
            self.read_errors(),
            self.read_timeouts(),
            self.messages_sent(),
            self.bytes_sent(),
            self.write_errors(),
            self.write_timeouts()
        )
    }
}

impl Statistics for Stats {
    #[inline(always)]
    fn messages_received(&self) -> u64 {
        self.messages_received
    }

    #[inline(always)]
    fn bytes_received(&self) -> u64 {
        self.bytes_received
    }

    #[inline(always)]
    fn read_errors(&self) -> u64 {
        self.read_errors
    }

    #[inline(always)]
    fn read_timeouts(&self) -> u64 {
        self.read_timeouts
    }

    #[inline(always)]
    fn messages_sent(&self) -> u64 {
        self.messages_sent
    }

    #[inline(always)]
    fn bytes_sent(&self) -> u64 {
        self.bytes_sent
    }

    #[inline(always)]
    fn write_errors(&self) -> u64 {
        self.write_errors
    }

    #[inline(always)]
    fn write_timeouts(&self) -> u64 {
        self.write_timeouts
    }

    #[inline(always)]
    fn snapshot(&self) -> [u64; 8] {
        [
            self.messages_received,
            self.bytes_received,
            self.read_errors,
            self.read_timeouts,
            self.messages_sent,
            self.bytes_sent,
            self.write_errors,
            self.write_timeouts,
        ]
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
