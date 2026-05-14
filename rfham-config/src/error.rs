//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use rfham_markdown::error::MarkdownError;
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

    #[error("A Lock, or similar sync structure, was poisoned; error: {0}")]
    LockPoison(String),

    #[error("An error occured writing markdown output; error {0}")]
    Markdown(#[from] MarkdownError),

    #[error("The name `{0}` is not a valid config path component in `{1}`")]
    InvalidPathComponent(String, &'static str, Vec<&'static str>),

    #[error("The name `{0}` is a valid config path component in `{1}` but expects more components")]
    PathTooShort(String, &'static str, Vec<&'static str>),

    #[error("Config paths cannot reference credentials")]
    RestrictedPath,

    #[error("Error accessing credential store; reason: {0}")]
    CredentialStore(String),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;
