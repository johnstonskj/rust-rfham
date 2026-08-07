//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::error::{RigError, lock_poisoned};
use rfham_config::connections::{Connection, Host, IpConnection, SerialConnection};
use serialport::{
    DataBits, Error as SerialError, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits,
};
use std::{
    fmt::Debug,
    io::{Error as IoError, ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::{Arc, RwLock, RwLockWriteGuard},
    time::Duration,
};
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ActiveConnection {
    serial: bool,
    inner: Arc<RwLock<ActiveConnectionKind>>,
}

#[derive(Debug, EnumIs, EnumTryAs)]
pub enum ActiveConnectionKind {
    Serial { port: Box<dyn SerialPort> },
    Ip { stream: TcpStream },
}

const DEFAULT_SERIAL_TIMEOUT: Duration = Duration::from_millis(200);
const DEFAULT_IP_CONNECT_TIMEOUT: Duration = Duration::new(15, 0);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Read for ActiveConnectionKind {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Serial { port } => port.read(buf),
            Self::Ip { stream } => stream.read(buf),
        }
    }
}

impl Write for ActiveConnectionKind {
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

impl TryFrom<&Connection> for ActiveConnectionKind {
    type Error = RigError;

    fn try_from(connection: &Connection) -> Result<Self, Self::Error> {
        Ok(match connection {
            Connection::Serial(conn) => Self::Serial {
                port: to_serial_port(conn)?,
            },
            Connection::Ip(conn) => Self::Ip {
                stream: if let Some(result) = to_socket_address(conn)?.map(|addr| {
                    TcpStream::connect_timeout(
                        &addr,
                        conn.timeout().unwrap_or(DEFAULT_IP_CONNECT_TIMEOUT),
                    )
                }) {
                    result?
                } else {
                    return Err(log_rig_error!(SocketAddress => socket_addr: conn.to_string()));
                },
            },
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl Read for ActiveConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut inner = self.inner.write().map_err(|_| ErrorKind::Other)?;
        inner.read(buf)
    }
}

impl Write for ActiveConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut inner = self.inner.write().map_err(|_| ErrorKind::Other)?;
        inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut inner = self.inner.write().map_err(|_| ErrorKind::Other)?;
        inner.flush()
    }
}

impl ActiveConnection {
    pub fn new(inner: ActiveConnectionKind) -> Self {
        Self {
            serial: inner.is_serial(),
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn is_serial(&self) -> bool {
        self.serial
    }

    pub fn is_ip(&self) -> bool {
        !self.serial
    }

    pub fn inner(&mut self) -> Result<RwLockWriteGuard<'_, ActiveConnectionKind>, RigError> {
        self.inner.write().map_err(|e| lock_poisoned(e))
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
        Host::Address(addr) => (addr.clone(), conn.port())
            .to_socket_addrs()
            .map(|addrs| addrs.into_iter().next()),
    }
}

pub fn to_serial_port_builder(conn: &SerialConnection) -> SerialPortBuilder {
    serialport::new(&conn.path().display().to_string(), conn.baud_rate())
        .data_bits(conn.data_bits().unwrap_or(DataBits::Eight))
        .flow_control(conn.flow_control().unwrap_or(FlowControl::None))
        .parity(conn.parity().unwrap_or(Parity::None))
        .stop_bits(conn.stop_bits().unwrap_or(StopBits::One))
        .timeout(conn.timeout().unwrap_or(DEFAULT_SERIAL_TIMEOUT))
}

pub fn to_serial_port(conn: &SerialConnection) -> Result<Box<dyn SerialPort>, SerialError> {
    to_serial_port_builder(conn).open()
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod log;
