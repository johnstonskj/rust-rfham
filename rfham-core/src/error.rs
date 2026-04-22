//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use thiserror::Error;

#[cfg(feature = "std")]
use std::io::Error as IoError;

#[cfg(feature = "std")]
use std::string::FromUtf8Error;

#[cfg(not(feature = "std"))]
use alloc::string::FromUtf8Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Unable to parse string as `{1}`; value: {0:?}")]
    InvalidValueFromStr(String, &'static str),

    #[error("Value `{0}` is not valid for type `{1}`")]
    InvalidValue(String, &'static str),

    #[error("Unable to convert from UTF-8 to string; error: {0}")]
    FromUtf(#[from] FromUtf8Error),

    #[cfg(feature = "std")]
    #[error("An I/O error occurred; error: {0}")]
    Io(#[from] IoError),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = std::result::Result<T, CoreError>;
