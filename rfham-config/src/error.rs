//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use std::io::Error as IoError;
use thiserror::Error;
use toml::{de::Error as ParserError, ser::Error as SerializerError};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("An error occured in an I/O operation; error: {0}")]
    Io(#[from] IoError),

    #[error("Could not determine the location of the configuration directory.")]
    ConfigDir,

    #[error("An error occured parsing the configuration file; error {0}")]
    Parser(#[from] ParserError),

    #[error("An error occured serializing the configuration file; error {0}")]
    Serializer(#[from] SerializerError),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;
