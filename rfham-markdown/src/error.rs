//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum MarkdownError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type MarkdownResult<T> = std::result::Result<T, MarkdownError>;
