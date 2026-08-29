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

use crate::error::{RigError, enum_parse, lock_poisoned};
use rfham_config::connections::{Connection, Host, IpConnection, SerialConnection};
use serialport::{
    DataBits, Error as SerialError, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits,
};
use std::{
    fmt::{Debug, Display},
    io::{Error as IoError, ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::{Arc, RwLock, RwLockWriteGuard},
    time::Duration,
};
use strum::{AsRefStr, EnumIs, EnumTryAs, FromRepr};

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

///
/// In telecommunications and electronics, baud is a common unit of measurement of symbol rate,
/// which is one of the components that determine the speed of communication over a data channel.
///
/// It is the unit for symbol rate or modulation rate in symbols per second or pulses per second.
/// It is the number of distinct symbol changes (signalling events) made to the transmission medium
/// per second in a digitally modulated signal or a bd rate line code.
///
/// Baud is related to gross bit rate, which can be expressed in bits per second (bit/s).
/// If there are precisely two symbols in the system (typically 0 and 1), then baud and bits per
/// second are equivalent.
///
/// Its symbol is uppercase (Bd), but when the unit is spelled out, it should be written in
/// lowercase (baud) except when it begins a sentence or is capitalized for another reason, such as
/// in title case. It was defined by the CCITT (now the ITU-T) in November 1926.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsRefStr, EnumIs, FromRepr)]
#[repr(u32)]
pub enum BaudRate {
    /// Bell 103 modem or ITU-T V.21 modem.
    Bd300 = 300,
    /// Bell 202, Bell 212A, orITU-T V.22 modem.
    Bd1200 = 1200,
    /// ITU-T V.22bis modem.
    Bd2400 = 2400,
    /// ITU-T V.27ter modem.
    Bd4800 = 4800,
    /// ITU-T V.32 modem.
    Bd9600 = 9600,
    /// ITU-T V.32bis modem.
    Bd14000 = 14000,
    Bd19200 = 19200,
    Bd38400 = 38400,
    /// ITU-T V.90/V.92 modem.
    Bd56000 = 56000,
    /// ITU-T V.32bis modem with V.42bis compression.
    Bd57600 = 57600,
    /// ITU-T V.34 modem with V.42bis compression, low cost serial V.90/V.92 modem with V.42bis or V.44 compression.
    Bd115200 = 115200,
    /// ISO 11898-3 CAN bus.
    Bd125000 = 125000,
    /// Basic Rate Interface ISDN terminal adapter.
    Bd128000 = 128000,
    /// LocalTalk, Econet, high end serial V.90/V.92 modem with V.42bis or V.44 compression.
    Bd230400 = 230400,
    /// DMX512, stage lighting and effects network.
    Bd250000 = 250000,
}

const DEFAULT_SERIAL_TIMEOUT: Duration = Duration::from_millis(200);
const DEFAULT_IP_CONNECT_TIMEOUT: Duration = Duration::new(15, 0);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for BaudRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl TryFrom<u32> for BaudRate {
    type Error = RigError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or_else(|| enum_parse(value, "BaudRate"))
    }
}

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
        self.inner.write().map_err(lock_poisoned)
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

pub fn to_serial_port_builder(conn: &SerialConnection) -> SerialPortBuilder {
    serialport::new(conn.path().display().to_string(), conn.baud_rate())
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
