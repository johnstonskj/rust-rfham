//! Error and result types for `rfham-radios`.
//!

#[cfg(feature = "entity-api")]
use crate::api::modes::OperatingMode;

#[cfg(feature = "entity-api")]
use rfham_core::Name;

use std::{fmt::Display, num::ParseIntError, sync::PoisonError};
use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum RigError {
    // --------------------------------------------------------------------------------------------
    // Wrapped Dependency Errors
    // --------------------------------------------------------------------------------------------
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serial port I/O error: {0}")]
    Serial(#[from] serialport::Error),

    #[error("Toml parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    // --------------------------------------------------------------------------------------------
    // Pseudo-Wrapped Dependency Errors
    // --------------------------------------------------------------------------------------------
    #[error("Could not convert IpConnection to socket address; connection: {socket_addr}")]
    SocketAddress { socket_addr: String },

    #[error("Synchronization poison error: {message}")]
    Poison { message: String },

    // --------------------------------------------------------------------------------------------
    // Inter-Layer Errors
    // --------------------------------------------------------------------------------------------
    #[cfg(feature = "entity-api")]
    #[error("Function not supported by target rig; operation: {function_name}, rig: {rig_name}")]
    UnsupportedFunction {
        function_name: String,
        rig_name: String,
    },

    #[cfg(feature = "entity-api")]
    #[error("Mode not supported by target rig; operation: {mode}, rig: {rig_name}")]
    UnsupportedMode { mode: OperatingMode, rig_name: Name },

    // --------------------------------------------------------------------------------------------
    // Argument Validation Errors
    // --------------------------------------------------------------------------------------------
    #[error(
        "Invalid length for argument {argument_name}; expecting {expecting:?} bytes, given {given}"
    )]
    InvalidArgumentLength {
        argument_name: &'static str,
        expecting: ranges::Ranges<usize>,
        given: usize,
    },

    #[error("Invalid value {value:?} of type `{type_name}` for argument {argument_name}")]
    InvalidArgumentValue {
        argument_name: &'static str,
        type_name: &'static str,
        value: String,
    },

    // --------------------------------------------------------------------------------------------
    // Response Validation Errors
    // --------------------------------------------------------------------------------------------
    #[error("Invalid response message length; expecting {expecting} bytes, given {given}")]
    InvalidResponseLength {
        expecting: ranges::Ranges<usize>,
        given: usize,
    },

    #[error("Invalid response command identification string; expecting {expecting:?}")]
    InvalidResponseCommandId { expecting: Vec<u8> },

    #[error("Invalid response data, could not parse {data:?}")]
    InvalidResponseData { data: Vec<u8> },

    #[error("Invalid response, missing message terminator byte or bytes; expecting {expecting:?}")]
    InvalidResponseTerminator { expecting: Vec<u8> },

    // --------------------------------------------------------------------------------------------
    // Response Parsing (Lower Level) Errors
    // --------------------------------------------------------------------------------------------
    #[error(
        "Invalid string representation of a Frequency; could not parse {value:?}, error: {error}"
    )]
    ParseFrequency { value: String, error: ParseIntError },

    #[error(
        "Invalid string representation for enum type {type_name}, not a known variant; could not parse {value:?}"
    )]
    ParseEnum {
        type_name: &'static str,
        value: String,
    },

    #[error("Invalid string representation of a boolean; could not parse {value:?}")]
    ParseBoolean { value: u8 },

    #[error("Invalid string representation of a byte; could not parse {value:?}")]
    ParseByte { value: Vec<u8> },

    #[error(
        "Invalid numeric sign character, expecting '+' or '-', or space if loose validation, given {value:?}"
    )]
    ParseSign { value: u8 },

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
// Public Functions ❯ Pseudo-Wrapped Dependency Errors
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn lock_poisoned<T>(error: PoisonError<T>) -> RigError {
    log_rig_error!(Poison => message: error.to_string())
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Inter-Layer Errors
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "entity-api")]
#[inline(always)]
pub fn unsupported_function<S1, S2>(function_name: S1, rig_name: S2) -> RigError
where
    S1: Display,
    S2: Display,
{
    log_rig_error!(UnsupportedFunction =>
        function_name: function_name.to_string(),
        rig_name: rig_name.to_string()
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Argument Validation Errors
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn invalid_argument_length<R>(
    argument_name: &'static str,
    expecting: R,
    given: usize,
) -> RigError
where
    R: Into<ranges::Ranges<usize>>,
{
    log_rig_error!(InvalidArgumentLength =>
        argument_name,
        expecting: expecting.into(),
        given
    )
}

#[inline(always)]
pub fn invalid_argument_value<V>(
    argument_name: &'static str,
    type_name: &'static str,
    value: V,
) -> RigError
where
    V: Display,
{
    log_rig_error!(InvalidArgumentValue =>
        argument_name,
        type_name,
        value: value.to_string()
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Response Validation Errors
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn invalid_response_length<R>(expecting: R, given: usize) -> RigError
where
    R: Into<ranges::Ranges<usize>>,
{
    log_rig_error!(InvalidResponseLength =>
        expecting: expecting.into(),
        given
    )
}

#[inline(always)]
pub fn invalid_response_data<V>(data: V) -> RigError
where
    V: Into<Vec<u8>>,
{
    log_rig_error!(InvalidResponseData => data: data.into() )
}

#[inline(always)]
pub fn invalid_response_command_id<V>(expecting: V) -> RigError
where
    V: Into<Vec<u8>>,
{
    log_rig_error!(InvalidResponseCommandId =>
        expecting: expecting.into()
    )
}

#[inline(always)]
pub fn invalid_response_terminator<V>(expecting: V) -> RigError
where
    V: Into<Vec<u8>>,
{
    log_rig_error!(InvalidResponseTerminator =>
        expecting: expecting.into()
    )
}

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Response Parsing (Lower Level)
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn enum_try_from<S>(value: S, type_name: &'static str, _: strum::ParseError) -> RigError
where
    S: Into<String>,
{
    log_rig_error!(RigError::ParseEnum {
        type_name,
        value: value.into()
    })
}

#[inline(always)]
pub fn enum_parse<S>(value: S, type_name: &'static str) -> RigError
where
    S: Display,
{
    log_rig_error!(RigError::ParseEnum {
        type_name,
        value: value.to_string()
    })
}

#[inline(always)]
pub fn invalid_bcd_digit(byte: u8) -> RigError {
    log_rig_error!(InvalidBcdDigit => byte)
}

pub mod ranges {
    use core::{
        fmt::Display,
        ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    };
    use num::{Integer, Unsigned};
    use strum::EnumIs;

    #[derive(Debug, Clone, PartialEq, Eq, EnumIs)]
    pub enum Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        Just(T),
        Range(Range<T>),
        RangeFrom(RangeFrom<T>),
        RangeFull(RangeFull),
        RangeInclusive(RangeInclusive<T>),
        RangeTo(RangeTo<T>),
        RangeToInclusive(RangeToInclusive<T>),
    }

    impl<T> Display for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.is_singleton() {
                self.start().unwrap().fmt(f)?;
            }
            format!(
                "{}..{}",
                self.start().map(|v| v.to_string()).unwrap_or_default(),
                self.end().map(|v| v.to_string()).unwrap_or_default()
            )
            .fmt(f)
        }
    }

    impl<T> From<T> for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn from(value: T) -> Self {
            Ranges::Just(value)
        }
    }

    impl<T> From<Range<T>> for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn from(range: Range<T>) -> Self {
            Ranges::Range(range)
        }
    }

    impl<T> From<RangeFrom<T>> for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn from(range: RangeFrom<T>) -> Self {
            Ranges::RangeFrom(range)
        }
    }

    impl<T> From<RangeInclusive<T>> for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn from(range: RangeInclusive<T>) -> Self {
            Ranges::RangeInclusive(range)
        }
    }

    impl<T> From<RangeTo<T>> for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn from(range: RangeTo<T>) -> Self {
            Ranges::RangeTo(range)
        }
    }

    impl<T> From<RangeToInclusive<T>> for Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        fn from(range: RangeToInclusive<T>) -> Self {
            Ranges::RangeToInclusive(range)
        }
    }

    impl<T> Ranges<T>
    where
        T: Unsigned + Integer + Copy + Display,
    {
        pub const fn just(value: T) -> Self {
            Ranges::Just(value)
        }

        pub const fn from_to(range: Range<T>) -> Self {
            Ranges::Range(range)
        }

        pub const fn from(range: RangeFrom<T>) -> Self {
            Ranges::RangeFrom(range)
        }

        pub const fn full(range: RangeFull) -> Self {
            Ranges::RangeFull(range)
        }

        pub const fn from_to_inclusive(range: RangeInclusive<T>) -> Self {
            Ranges::RangeInclusive(range)
        }

        pub const fn to(range: RangeTo<T>) -> Self {
            Ranges::RangeTo(range)
        }

        pub const fn to_inclusive(range: RangeToInclusive<T>) -> Self {
            Ranges::RangeToInclusive(range)
        }

        pub fn contains(&self, value: T) -> bool {
            match self {
                Ranges::Just(v) => *v == value,
                Ranges::Range(r) => r.contains(&value),
                Ranges::RangeFrom(r) => r.contains(&value),
                Ranges::RangeFull(_) => true,
                Ranges::RangeInclusive(r) => r.contains(&value),
                Ranges::RangeTo(r) => r.contains(&value),
                Ranges::RangeToInclusive(r) => r.contains(&value),
            }
        }

        pub const fn start(&self) -> Option<T> {
            match self {
                Ranges::Just(v) => Some(*v),
                Ranges::Range(r) => Some(r.start),
                Ranges::RangeFrom(r) => Some(r.start),
                Ranges::RangeFull(_) => None,
                Ranges::RangeInclusive(r) => Some(*r.start()),
                Ranges::RangeTo(_) => None,
                Ranges::RangeToInclusive(_) => None,
            }
        }

        pub const fn end(&self) -> Option<T> {
            match self {
                Ranges::Just(v) => Some(*v),
                Ranges::Range(r) => Some(r.end),
                Ranges::RangeFrom(_) => None,
                Ranges::RangeFull(_) => None,
                Ranges::RangeInclusive(r) => Some(*r.end()),
                Ranges::RangeTo(r) => Some(r.end),
                Ranges::RangeToInclusive(r) => Some(r.end),
            }
        }

        pub const fn is_bounded(&self) -> bool {
            match self {
                Ranges::Just(_) => true,
                Ranges::Range(_) => true,
                Ranges::RangeFrom(_) => false,
                Ranges::RangeFull(_) => false,
                Ranges::RangeInclusive(_) => true,
                Ranges::RangeTo(_) => false,
                Ranges::RangeToInclusive(_) => false,
            }
        }
        pub fn is_singleton(&self) -> bool {
            match self {
                Ranges::Just(_) => true,
                Ranges::Range(r) => r.start == r.end,
                Ranges::RangeFrom(_) => false,
                Ranges::RangeFull(_) => false,
                Ranges::RangeInclusive(r) => r.start() == r.end(),
                Ranges::RangeTo(_) => false,
                Ranges::RangeToInclusive(_) => false,
            }
        }
    }
}
