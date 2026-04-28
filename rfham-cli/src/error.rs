//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use core::fmt::Error as FmtError;
use rfham_config::error::ConfigError;
use rfham_core::error::CoreError;
use rfham_markdown::MarkdownError;
use thiserror::Error;
use tracing_subscriber::filter::FromEnvError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum CliError {
    #[error("An error occured in the rfham libraries; error: {0}")]
    Core(#[from] CoreError),

    #[error(
        "Could not retrieve value from environment variable for command-line argument; error: {0}"
    )]
    EnvError(#[from] FromEnvError),

    #[error("An error occured during formatting; error: {0}")]
    FmtError(#[from] FmtError),

    #[error("An error occured loading or initializing the configuration; error: {0}")]
    Config(#[from] ConfigError),

    #[error("An error occured writing markdown output; error: {0}")]
    Markdown(#[from] MarkdownError),
}
