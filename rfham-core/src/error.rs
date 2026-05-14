//! Error and result types for `rfham-core`.
//!
//! [`CoreError`] is the single error enum used across all modules in this crate.
//! [`Result<T>`] is a type alias for `std::result::Result<T, CoreError>`.

use std::num::ParseFloatError;
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

    #[error("Value `{0}` is not valid for type `{1}`, in the context of `{2}`")]
    InvalidValueCtx(String, &'static str, &'static str),

    #[error("Unable to convert from UTF-8 to string; error: {0}")]
    FromUtf(#[from] FromUtf8Error),

    #[cfg(feature = "std")]
    #[error("An I/O error occurred; error: {0}")]
    Io(#[from] IoError),

    #[error("Unable to parse float value from string; error: {0}")]
    ParseFloat(#[from] ParseFloatError),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type CoreResult<T> = std::result::Result<T, CoreError>;
