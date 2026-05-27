//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use rfham_core::error::CoreError;
use rfham_markdown::error::MarkdownError;
use std::{io::Error as IoError, num::ParseIntError};
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

    #[error("An error occured in the core library; error {0}")]
    Core(#[from] CoreError),

    #[error("Could not determine the location of the configuration directory.")]
    ConfigDir,

    #[error("An error occured parsing the configuration file; error {0}")]
    Parser(#[from] ParserError),

    #[error("An error occured parsing the input as an integer value; error: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("An error occured serializing the configuration file; error {0}")]
    Serializer(#[from] SerializerError),

    #[error("A Lock, or similar sync structure, was poisoned; error: {0}")]
    LockPoison(String),

    #[error("An error occured writing markdown output; error {0}")]
    Markdown(#[from] MarkdownError),

    #[error(
        "The name `{0}` is not a valid config path component in `{1}`, expecting one of: {2:?}"
    )]
    InvalidPathComponent(String, &'static str, Vec<&'static str>),

    #[error("The value `{0}` is not a valid Name path element as expected in `{1}`")]
    InvalidPathElementName(String, &'static str),

    #[error("The value `{0}` is not a valid Index path element as expected in `{1}`")]
    InvalidPathElementIndex(String, &'static str),

    #[error("The index `{0}` is not a valid index for the list `{1}`")]
    InvalidPathIndex(usize, &'static str),

    #[error(
        "The name `{0}` is a valid config path component in `{1}` but expects more components; one of: {2:?}"
    )]
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
