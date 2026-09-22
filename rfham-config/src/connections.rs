//! Rig connection types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fields::{
        CFG_FIELD_ADDRESS, CFG_FIELD_BAUD_RATE, CFG_FIELD_CONNECT_TIMEOUT, CFG_FIELD_CONNECTION,
        CFG_FIELD_CONNECTIONS, CFG_FIELD_DATA_BITS, CFG_FIELD_FLOW_CONTROL, CFG_FIELD_HOST_NAME,
        CFG_FIELD_IO_TIMEOUT, CFG_FIELD_PARITY, CFG_FIELD_PATH, CFG_FIELD_PORT,
        CFG_FIELD_READ_TIMEOUT, CFG_FIELD_STOP_BITS, CFG_FIELD_TYPE, CFG_FIELD_WRITE_TIMEOUT,
    },
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathElement, PathTarget, Value},
};
use rfham_core::{StringLike, fmt::FormattedWriter, names::Name};
use rfham_markdown::{blank_line, bulleted_list_item};
use serde::{Deserialize, Serialize};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::{
    fmt::Display,
    io::Write,
    iter::IntoIterator,
    net::IpAddr,
    num::{ParseFloatError, ParseIntError},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};
use strum::{EnumIs, EnumTryAs, FromRepr};
use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Connections
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, EnumIs, EnumTryAs)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum Connection {
    Serial(SerialConnection),
    Ip(IpConnection),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Serial Connected Rigs
// ------------------------------------------------------------------------------------------------

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
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    EnumIs,
    FromRepr,
)]
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SerialConnection {
    path: PathBuf,
    baud_rate: BaudRate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop_bits: Option<StopBits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    data_bits: Option<DataBits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    flow_control: Option<FlowControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parity: Option<Parity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    io_timeout: Option<Duration>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ IP Connected Rigs
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct IpConnection {
    #[serde(flatten)]
    host: Host,
    port: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    connect_timeout: Option<Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    read_timeout: Option<Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    write_timeout: Option<Duration>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Host {
    HostName(String),
    Address(IpAddr),
}

#[derive(Debug, Error)]
pub enum ParseConnectionString {
    #[error("String value may not be empty")]
    Empty,

    #[error("Connection strings have {expected} required components, parser found {given}.")]
    RequiredPartCount { expected: usize, given: usize },

    #[error(
        "Connection strings have a maximum {expected} optional components, parser found {given}."
    )]
    OptionalPartCount { expected: usize, given: usize },

    #[error("Optional field name {0} is not known for the current connection type.")]
    UnknownOptionalPartName(String),

    #[error("An error occured parsing the input for optional part {name}; value: {value}")]
    ParseOptionalPart { name: &'static str, value: String },

    #[error("An error occured parsing the input as an integer value; error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("An error occured parsing the input as a float value; error: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("An error occured parsing the input as a variant of enum `{name}`; value: {value};")]
    ParseEnum { name: &'static str, value: String },
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ BaudRate
// ------------------------------------------------------------------------------------------------

impl Display for BaudRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl TryFrom<u32> for BaudRate {
    type Error = ConfigError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or_else(|| ConfigError::ParseEnum {
            type_name: "BaudRate",
            value: value.to_string(),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Connections
// ------------------------------------------------------------------------------------------------

impl FormattedWriter for Connection {
    type Options = FormatterOptions;
    type Error = ConfigError;

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Ip(conn) => conn.write_with(writer, options),
            Self::Serial(conn) => conn.write_with(writer, options),
        }
    }
}

impl PathTarget for Connection {
    fn path_name() -> Option<rfham_core::Name> {
        Some(Name::new_unchecked(CFG_FIELD_CONNECTIONS))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let (head, _tail) = path.split();
        if let PathElement::Name(name) = head {
            match (self, name.as_str()) {
                // Serial
                (Self::Serial(_), name) if name == CFG_FIELD_TYPE => {
                    Ok(Value::String("serial".to_string()))
                }
                (Self::Serial(conn), name) if name == CFG_FIELD_PATH => {
                    Ok(Value::Path(conn.path.clone()))
                }
                (Self::Serial(conn), name) if name == CFG_FIELD_BAUD_RATE => {
                    Ok(Value::Integer(conn.baud_rate as i64))
                }
                (Self::Serial(conn), name) if name == CFG_FIELD_DATA_BITS => Ok(conn
                    .data_bits
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::None)),
                (Self::Serial(conn), name) if name == CFG_FIELD_STOP_BITS => Ok(conn
                    .stop_bits
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::None)),
                (Self::Serial(conn), name) if name == CFG_FIELD_FLOW_CONTROL => Ok(conn
                    .flow_control
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::None)),
                (Self::Serial(conn), name) if name == CFG_FIELD_PARITY => Ok(conn
                    .parity
                    .map(|v| Value::String(v.to_string()))
                    .unwrap_or(Value::None)),
                (Self::Serial(conn), name) if name == CFG_FIELD_IO_TIMEOUT => Ok(conn
                    .io_timeout
                    .map(|v| Value::Float(v.as_secs_f64()))
                    .unwrap_or(Value::None)),
                // IP
                (Self::Ip(_), name) if name == CFG_FIELD_TYPE => {
                    Ok(Value::String("ip".to_string()))
                }
                (Self::Ip(conn), name) if name == CFG_FIELD_ADDRESS => {
                    Ok(Value::String(conn.host.to_string()))
                }
                (Self::Ip(conn), name) if name == CFG_FIELD_HOST_NAME => {
                    Ok(Value::String(conn.host.to_string()))
                }
                (Self::Ip(conn), name) if name == CFG_FIELD_PORT => {
                    Ok(Value::Integer(conn.port as i64))
                }
                (Self::Ip(conn), name) if name == CFG_FIELD_CONNECT_TIMEOUT => Ok(conn
                    .connect_timeout
                    .map(|v| Value::Float(v.as_secs_f64()))
                    .unwrap_or(Value::None)),
                (Self::Ip(conn), name) if name == CFG_FIELD_READ_TIMEOUT => Ok(conn
                    .read_timeout
                    .map(|v| Value::Float(v.as_secs_f64()))
                    .unwrap_or(Value::None)),
                (Self::Ip(conn), name) if name == CFG_FIELD_WRITE_TIMEOUT => Ok(conn
                    .write_timeout
                    .map(|v| Value::Float(v.as_secs_f64()))
                    .unwrap_or(Value::None)),
                // Error
                (Self::Serial(_), name) => Err(ConfigError::InvalidPathComponent(
                    name.to_string(),
                    CFG_FIELD_CONNECTION,
                    vec![
                        CFG_FIELD_TYPE,
                        CFG_FIELD_PATH,
                        CFG_FIELD_BAUD_RATE,
                        CFG_FIELD_STOP_BITS,
                        CFG_FIELD_DATA_BITS,
                        CFG_FIELD_FLOW_CONTROL,
                        CFG_FIELD_PARITY,
                        CFG_FIELD_IO_TIMEOUT,
                    ],
                )),
                (Self::Ip(_), name) => Err(ConfigError::InvalidPathComponent(
                    name.to_string(),
                    CFG_FIELD_CONNECTION,
                    vec![
                        CFG_FIELD_TYPE,
                        CFG_FIELD_HOST_NAME,
                        CFG_FIELD_ADDRESS,
                        CFG_FIELD_PORT,
                        CFG_FIELD_CONNECT_TIMEOUT,
                        CFG_FIELD_READ_TIMEOUT,
                        CFG_FIELD_WRITE_TIMEOUT,
                    ],
                )),
            }
        } else {
            Err(ConfigError::InvalidPathElementName(
                head.to_string(),
                CFG_FIELD_CONNECTION,
            ))
        }
    }

    fn value_names() -> impl Iterator<Item = &'static str> {
        [
            // Serial
            CFG_FIELD_PATH,
            CFG_FIELD_BAUD_RATE,
            CFG_FIELD_STOP_BITS,
            CFG_FIELD_DATA_BITS,
            CFG_FIELD_FLOW_CONTROL,
            CFG_FIELD_PARITY,
            CFG_FIELD_IO_TIMEOUT,
            // IP
            CFG_FIELD_HOST_NAME,
            CFG_FIELD_ADDRESS,
            CFG_FIELD_PORT,
            CFG_FIELD_CONNECT_TIMEOUT,
            CFG_FIELD_READ_TIMEOUT,
            CFG_FIELD_WRITE_TIMEOUT,
            // Common
            CFG_FIELD_TYPE,
        ]
        .into_iter()
    }
}

impl From<SerialConnection> for Connection {
    fn from(value: SerialConnection) -> Self {
        Self::Serial(value)
    }
}

impl From<IpConnection> for Connection {
    fn from(value: IpConnection) -> Self {
        Self::Ip(value)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Serial Connected Rigs
// ------------------------------------------------------------------------------------------------

impl FromStr for SerialConnection {
    type Err = ParseConnectionString;

    ///
    /// Connection string:
    ///
    /// <host-or-ip>:<port>(;timeout=<timeout>)?
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseConnectionString::Empty)
        } else {
            let top_parts = s.split(';').collect::<Vec<_>>();
            if top_parts.len() > 6 {
                return Err(ParseConnectionString::OptionalPartCount {
                    expected: 5,
                    given: top_parts.len() - 1,
                });
            }
            let required = top_parts[0].split(':').collect::<Vec<_>>();
            if required.len() != 2 {
                return Err(ParseConnectionString::RequiredPartCount {
                    expected: 2,
                    given: required.len(),
                });
            }
            let mut connection = SerialConnection::new(
                PathBuf::from(required[0]),
                BaudRate::from_repr(u32::from_str(required[1])?).ok_or(
                    ParseConnectionString::ParseEnum {
                        name: "BaudRate",
                        value: required[1].to_string(),
                    },
                )?,
            );

            for optional in top_parts.into_iter().skip(1) {
                if let Some((name, value)) = optional.split_once('=') {
                    match name {
                        CFG_FIELD_STOP_BITS => {
                            connection.stop_bits = Some(match value {
                                "1" | "One" => StopBits::One,
                                "2" | "Two" => StopBits::Two,
                                _ => {
                                    return Err(ParseConnectionString::ParseOptionalPart {
                                        name: CFG_FIELD_STOP_BITS,
                                        value: value.to_string(),
                                    });
                                }
                            })
                        }
                        CFG_FIELD_DATA_BITS => {
                            connection.data_bits = Some(match value {
                                "5" | "Five" => DataBits::Five,
                                "6" | "Six" => DataBits::Six,
                                "7" | "Seven" => DataBits::Seven,
                                "8" | "Eight" => DataBits::Eight,
                                _ => {
                                    return Err(ParseConnectionString::ParseOptionalPart {
                                        name: CFG_FIELD_DATA_BITS,
                                        value: value.to_string(),
                                    });
                                }
                            })
                        }
                        CFG_FIELD_FLOW_CONTROL => {
                            connection.flow_control = Some(match value {
                                "None" => FlowControl::None,
                                "Hardware" => FlowControl::Hardware,
                                "Software" => FlowControl::Software,
                                _ => {
                                    return Err(ParseConnectionString::ParseOptionalPart {
                                        name: CFG_FIELD_FLOW_CONTROL,
                                        value: value.to_string(),
                                    });
                                }
                            })
                        }
                        CFG_FIELD_PARITY => {
                            connection.parity = Some(match value {
                                "None" => Parity::None,
                                "Even" => Parity::Even,
                                "Odd" => Parity::Odd,
                                _ => {
                                    return Err(ParseConnectionString::ParseOptionalPart {
                                        name: CFG_FIELD_PARITY,
                                        value: value.to_string(),
                                    });
                                }
                            })
                        }
                        CFG_FIELD_IO_TIMEOUT => {
                            connection.io_timeout =
                                Some(Duration::from_millis(u64::from_str(value)?))
                        }
                        _ => panic!(),
                    }
                } else {
                    panic!()
                }
            }

            Ok(connection)
        }
    }
}

impl FormattedWriter for SerialConnection {
    type Options = FormatterOptions;
    type Error = ConfigError;

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        match options.output_kind() {
            OutputKind::MarkdownList => {
                blank_line(writer)?;
                bulleted_list_item(writer, 1, format!("Port path: {:?}", self.path))?;
                bulleted_list_item(writer, 1, format!("Baud rate: {}", self.baud_rate))?;
                if let Some(data_bits) = &self.data_bits {
                    bulleted_list_item(writer, 1, format!("Data bits: {data_bits}"))?;
                }
                if let Some(stop_bits) = &self.stop_bits {
                    bulleted_list_item(writer, 1, format!("Stop bits: {stop_bits}"))?;
                }
                if let Some(flow_control) = &self.flow_control {
                    bulleted_list_item(writer, 1, format!("Flow control: {flow_control}"))?;
                }
                if let Some(parity) = &self.parity {
                    bulleted_list_item(writer, 1, format!("Parity: {parity}"))?;
                }
                if let Some(timeout) = &self.io_timeout {
                    bulleted_list_item(
                        writer,
                        1,
                        format!("I/O timeout: {} seconds", timeout.as_secs_f64()),
                    )?;
                }
            }
            OutputKind::MarkdownTable => {
                todo!()
            }
            OutputKind::Toml => writer.write_all(toml::to_string_pretty(self)?.as_bytes())?,
        }
        Ok(())
    }
}

impl SerialConnection {
    pub fn new<P>(port_name_or_path: P, baud_rate: BaudRate) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: port_name_or_path.into(),
            baud_rate,
            stop_bits: None,
            data_bits: None,
            flow_control: None,
            parity: None,
            io_timeout: None,
        }
    }

    pub fn with_data_bits(mut self, data_bits: DataBits) -> Self {
        self.data_bits = Some(data_bits);
        self
    }

    pub fn with_stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = Some(stop_bits);
        self
    }

    pub fn with_flow_control(mut self, flow_control: FlowControl) -> Self {
        self.flow_control = Some(flow_control);
        self
    }

    pub fn with_parity(mut self, parity: Parity) -> Self {
        self.parity = Some(parity);
        self
    }

    pub fn with_io_timeout(mut self, timeout: Duration) -> Self {
        self.io_timeout = Some(timeout);
        self
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn baud_rate(&self) -> BaudRate {
        self.baud_rate
    }

    pub fn data_bits(&self) -> Option<DataBits> {
        self.data_bits
    }

    pub fn stop_bits(&self) -> Option<StopBits> {
        self.stop_bits
    }

    pub fn flow_control(&self) -> Option<FlowControl> {
        self.flow_control
    }

    pub fn parity(&self) -> Option<Parity> {
        self.parity
    }

    pub fn io_timeout(&self) -> Option<Duration> {
        self.io_timeout
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ IP Connected Rigs
// ------------------------------------------------------------------------------------------------

impl Display for IpConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}{}",
            self.host,
            self.port,
            if let Some(timeout) = &self.connect_timeout {
                if timeout.is_zero() {
                    String::new()
                } else {
                    format!(";timeout={}", timeout.as_secs_f64())
                }
            } else {
                String::new()
            }
        )
    }
}

impl FromStr for IpConnection {
    type Err = ParseConnectionString;

    ///
    /// Connection string:
    ///
    /// <host-or-ip>:<port>(;timeout=<timeout>)?
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseConnectionString::Empty)
        } else {
            let top_parts = s.split(';').collect::<Vec<_>>();
            if top_parts.len() > 4 {
                return Err(ParseConnectionString::OptionalPartCount {
                    expected: 3,
                    given: top_parts.len() - 1,
                });
            }
            let required = top_parts[0].split(':').collect::<Vec<_>>();
            if required.len() != 2 {
                return Err(ParseConnectionString::RequiredPartCount {
                    expected: 2,
                    given: required.len(),
                });
            }

            let mut connection =
                IpConnection::new(Host::from_str(required[0])?, u16::from_str(required[1])?);

            for optional in top_parts.into_iter().skip(1) {
                if let Some((name, value)) = optional.split_once('=') {
                    match name {
                        CFG_FIELD_CONNECT_TIMEOUT => {
                            connection.connect_timeout =
                                Some(Duration::from_millis(u64::from_str(value)?))
                        }
                        CFG_FIELD_READ_TIMEOUT => {
                            connection.read_timeout =
                                Some(Duration::from_millis(u64::from_str(value)?))
                        }
                        CFG_FIELD_WRITE_TIMEOUT => {
                            connection.write_timeout =
                                Some(Duration::from_millis(u64::from_str(value)?))
                        }
                        _ => panic!(),
                    }
                } else {
                    panic!()
                }
            }

            Ok(connection)
        }
    }
}

impl FormattedWriter for IpConnection {
    type Options = FormatterOptions;
    type Error = ConfigError;

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        match options.output_kind() {
            OutputKind::MarkdownList => {
                blank_line(writer)?;
                match &self.host {
                    Host::HostName(name) => {
                        bulleted_list_item(writer, 1, format!("Host name: {}", name))?
                    }
                    Host::Address(address) => {
                        bulleted_list_item(writer, 1, format!("IP address: {}", address))?
                    }
                }
                bulleted_list_item(writer, 1, format!("Port: {}", self.port))?;
                if let Some(timeout) = &self.connect_timeout {
                    bulleted_list_item(
                        writer,
                        1,
                        format!("Connect timeout: {} seconds", timeout.as_secs_f64()),
                    )?;
                }
                if let Some(timeout) = &self.read_timeout {
                    bulleted_list_item(
                        writer,
                        1,
                        format!("Read timeout: {} seconds", timeout.as_secs_f64()),
                    )?;
                }
                if let Some(timeout) = &self.write_timeout {
                    bulleted_list_item(
                        writer,
                        1,
                        format!("Write timeout: {} seconds", timeout.as_secs_f64()),
                    )?;
                }
            }
            OutputKind::MarkdownTable => {
                todo!()
            }
            OutputKind::Toml => writer.write_all(toml::to_string_pretty(self)?.as_bytes())?,
        }
        Ok(())
    }
}

impl IpConnection {
    pub fn new<H>(host: H, port: u16) -> Self
    where
        H: Into<Host>,
    {
        Self {
            host: host.into(),
            port,
            connect_timeout: None,
            read_timeout: None,
            write_timeout: None,
        }
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self
    }

    pub fn with_write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = Some(timeout);
        self
    }

    pub fn host(&self) -> &Host {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn connect_timeout(&self) -> Option<Duration> {
        self.connect_timeout
    }

    pub fn read_timeout(&self) -> Option<Duration> {
        self.read_timeout
    }

    pub fn write_timeout(&self) -> Option<Duration> {
        self.write_timeout
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address(v) => v.fmt(f),
            Self::HostName(v) => v.fmt(f),
        }
    }
}

impl From<&str> for Host {
    fn from(value: &str) -> Self {
        Self::HostName(value.to_string())
    }
}

impl From<String> for Host {
    fn from(value: String) -> Self {
        Self::HostName(value)
    }
}

impl From<&String> for Host {
    fn from(value: &String) -> Self {
        Self::HostName(value.clone())
    }
}

impl From<IpAddr> for Host {
    fn from(value: IpAddr) -> Self {
        Self::Address(value)
    }
}

impl FromStr for Host {
    type Err = ParseConnectionString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseConnectionString::Empty)
        } else {
            Ok(if let Ok(addr) = IpAddr::from_str(s) {
                Self::Address(addr)
            } else {
                Self::HostName(s.to_string())
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::IpAddr, str::FromStr};

    #[test]
    fn test_serial_connection_from_str() {
        assert_eq!(
            SerialConnection::new("/dev/cu.usbserial-A10KMJZB", BaudRate::Bd9600)
                .with_data_bits(DataBits::Eight)
                .with_flow_control(FlowControl::None)
                .with_io_timeout(std::time::Duration::from_millis(250)),
            SerialConnection::from_str(
                "/dev/cu.usbserial-A10KMJZB:9600;io-timeout=250;data-bits=8;flow-control=None"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_ip_connection_from_str() {
        assert_eq!(
            IpConnection::new(IpAddr::from_str("127.0.0.1").unwrap(), 8080)
                .with_connect_timeout(std::time::Duration::from_millis(1000))
                .with_read_timeout(std::time::Duration::from_millis(500))
                .with_write_timeout(std::time::Duration::from_millis(250)),
            IpConnection::from_str(
                "127.0.0.1:8080;connect-timeout=1000;read-timeout=500;write-timeout=250"
            )
            .unwrap()
        );
    }
}
