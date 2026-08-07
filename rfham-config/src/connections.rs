//! Rig connection types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fields::{
        CFG_FIELD_ADDRESS, CFG_FIELD_BAUD_RATE, CFG_FIELD_CONNECTION, CFG_FIELD_CONNECTIONS,
        CFG_FIELD_DATA_BITS, CFG_FIELD_FLOW_CONTROL, CFG_FIELD_HOST_NAME, CFG_FIELD_PARITY,
        CFG_FIELD_PATH, CFG_FIELD_PORT, CFG_FIELD_STOP_BITS, CFG_FIELD_TIMEOUT, CFG_FIELD_TYPE,
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
use strum::{EnumIs, EnumTryAs};
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SerialConnection {
    path: PathBuf,
    baud_rate: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop_bits: Option<StopBits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    data_bits: Option<DataBits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    flow_control: Option<FlowControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parity: Option<Parity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout: Option<Duration>,
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
    timeout: Option<Duration>,
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
}

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
                (Self::Serial(conn), name) if name == CFG_FIELD_TIMEOUT => Ok(conn
                    .timeout
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
                (Self::Ip(conn), name) if name == CFG_FIELD_TIMEOUT => Ok(conn
                    .timeout
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
                        CFG_FIELD_TIMEOUT,
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
                        CFG_FIELD_TIMEOUT,
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
            // IP
            CFG_FIELD_HOST_NAME,
            CFG_FIELD_ADDRESS,
            CFG_FIELD_PORT,
            // Common
            CFG_FIELD_TYPE,
            CFG_FIELD_TIMEOUT,
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
            let mut connection =
                SerialConnection::new(PathBuf::from(required[0]), u32::from_str(required[1])?);

            for optional in top_parts.into_iter().skip(1) {
                if let Some((name, value)) = optional.split_once('=') {
                    match name {
                        CFG_FIELD_STOP_BITS => {
                            connection.stop_bits = Some(match value {
                                "One" => StopBits::One,
                                "Two" => StopBits::Two,
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
                                "Five" => DataBits::Five,
                                "Six" => DataBits::Six,
                                "Seven" => DataBits::Seven,
                                "Eight" => DataBits::Eight,
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
                        CFG_FIELD_TIMEOUT => {
                            connection.timeout =
                                Some(Duration::from_secs_f64(f64::from_str(value)?))
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
                if let Some(timeout) = &self.timeout {
                    bulleted_list_item(
                        writer,
                        1,
                        format!("Timeout: {} seconds", timeout.as_secs_f64()),
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
    pub fn new<P, R>(port_name_or_path: P, baud_rate: R) -> Self
    where
        P: Into<PathBuf>,
        R: Into<u32>,
    {
        Self {
            path: port_name_or_path.into(),
            baud_rate: baud_rate.into(),
            stop_bits: None,
            data_bits: None,
            flow_control: None,
            parity: None,
            timeout: None,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn baud_rate(&self) -> u32 {
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

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
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
            if let Some(timeout) = &self.timeout {
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
            if top_parts.len() > 2 {
                return Err(ParseConnectionString::OptionalPartCount {
                    expected: 1,
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
            Ok(IpConnection {
                host: Host::from_str(required[0])?,
                port: u16::from_str(required[1])?,
                timeout: if top_parts.len() == 1 {
                    None
                } else if let Some(s) = top_parts[1].strip_prefix("timeout=") {
                    Some(Duration::from_secs_f64(f64::from_str(s)?))
                } else {
                    None
                },
            })
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
                if let Some(timeout) = &self.timeout {
                    bulleted_list_item(
                        writer,
                        1,
                        format!("Timeout: {} seconds", timeout.as_secs_f64()),
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
            timeout: None,
        }
    }

    pub fn new_with_timeout<H>(host: H, port: u16, timeout: Duration) -> Self
    where
        H: Into<Host>,
    {
        Self {
            host: host.into(),
            port,
            timeout: Some(timeout),
        }
    }

    pub fn host(&self) -> &Host {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
