//! Error and result types for `rfham-radios`.
//!

use crate::OperatingMode;
use rfham_core::Name;
use std::{num::ParseIntError, sync::PoisonError};
use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum RigError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Synchronization poison error: {message}")]
    Poison { message: String },

    #[error("Serial port I/O error: {0}")]
    Serial(#[from] serialport::Error),

    #[error("Toml parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Could not convert IpConnection to socket address; connection: {socket_addr}")]
    SocketAddress { socket_addr: String },

    #[error("Function not supported by target rig; operation: {function_name}, rig: {rig_name}")]
    UnsupportedFunction {
        function_name: String,
        rig_name: String,
    },

    #[error("Mode not supported by target rig; operation: {mode}, rig: {rig_name}")]
    UnsupportedMode { mode: OperatingMode, rig_name: Name },

    #[error(
        "Index value `{index}` out of bounds for argument {argument_name}; operation: {function_name}, rig: {rig_name}"
    )]
    IndexOutOfBounds {
        index: usize,
        argument_name: String,
        function_name: String,
        rig_name: String,
    },

    #[error(
        "Character value `{character}` invalid for argument {argument_name}; operation: {function_name}, rig: {rig_name}"
    )]
    InvalidChar {
        character: char,
        argument_name: String,
        function_name: String,
        rig_name: String,
    },

    #[error("Invalid response message length; expecting {expecting} bytes, given {given}")]
    InvalidResponseLength { expecting: usize, given: usize },

    #[error("Invalid response command identification string; expecting {expecting:?}")]
    InvalidResponseCommandString { expecting: String },

    #[error("Invalid response data, could not parse {data:?}")]
    InvalidResponseData { data: Vec<u8> },

    #[error("Invalid response, missing terminating ';' character")]
    InvalidResponseTerminator,

    #[error(
        "Invalid string representation of a Frequency; could not parse {frequency:?}, error: {error}"
    )]
    ParseFrequency {
        frequency: String,
        error: ParseIntError,
    },

    #[error("Invalid string representation of a Level; could not parse {level:?}, error: {error}")]
    ParseLevel { level: String, error: ParseIntError },

    #[error("Byte value `0x{byte:02X}` is not in range 0..=9 for a BCD value")]
    InvalidBcdDigit { byte: u8 },
}

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! log_rig_error {
    ($message:expr, $error:ident => $( $name:ident $( : $value:expr)? ),+ ) => {
        log_rig_error!($message, $crate::error::RigError::$error {
            $(
                $name $( : $value )?,
            )+
        })
    };
    ($error:ident => $( $name:ident $( : $value:expr)? ),+ ) => {
        log_rig_error!($crate::error::RigError::$error {
            $(
                $name $( : $value )?,
            )+
        })
    };
    ($message:expr, $error:expr) => {{
        let error = $error;
        ::tracing::error!("{message}; error: {error}");
        error
    }};
    ($error:expr) => {{
        let error = $error;
        ::tracing::error!("{error}");
        error
    }};
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn lock_poisoned<T>(error: PoisonError<T>) -> RigError {
    log_rig_error!(Poison => message: error.to_string())
}

#[inline(always)]
pub fn unsupported_function<S1, S2>(function_name: S1, rig_name: S2) -> RigError
where
    S1: Into<String>,
    S2: Into<String>,
{
    log_rig_error!(UnsupportedFunction =>
        function_name: function_name.into(),
        rig_name: rig_name.into()
    )
}

#[inline(always)]
pub fn index_out_of_bounds<S1, S2, S3>(
    index: usize,
    argument_name: S1,
    function_name: S2,
    rig_name: S3,
) -> RigError
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    log_rig_error!(IndexOutOfBounds =>
        index,
        argument_name: argument_name.into(),
        function_name: function_name.into(),
        rig_name: rig_name.into()
    )
}

#[inline(always)]
pub fn invalid_character<S1, S2, S3>(
    character: char,
    argument_name: S1,
    function_name: S2,
    rig_name: S3,
) -> RigError
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    log_rig_error!(InvalidChar =>
        character,
        argument_name: argument_name.into(),
        function_name: function_name.into(),
        rig_name: rig_name.into()
    )
}

#[inline(always)]
pub fn invalid_bcd_digit(byte: u8) -> RigError {
    log_rig_error!(InvalidBcdDigit => byte)
}
