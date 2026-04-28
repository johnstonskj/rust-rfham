//! Error and result types for `rfham-geo`.
//!
//! [`GeoError`] is the single error enum used across all modules in this crate.
//! [`GeoResult<T>`] is a type alias for `std::result::Result<T, GeoError>`.

use lat_long::Error as LatLongError;
use reqwest::Error as RequestError;
use reqwest::StatusCode;
use rfham_core::error::CoreError;
use serde_json::Error as JsonError;
use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum GeoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("An error occured converting latitude/longitude value; error: {0}")]
    LatLong(#[from] LatLongError),

    #[error("Cannot produce a Maidenhead grid locator for either North/South poles")]
    NoPolarGrid,

    #[error("An error occured in a Geo-IP provider; error: {0}")]
    GeoIpProvider(String),

    #[error("An HTTP service returned non-success code ; status-code: {0}")]
    Http(StatusCode),

    #[error("An error occured calling an HTTP service; status-code: {0}")]
    Reqwest(#[from] RequestError),

    #[error("Core library error detected; error: {0}")]
    Core(#[from] CoreError),

    #[error("An error occured serializing/deserializing JSON; error: {0}")]
    Serialization(#[from] JsonError),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type GeoResult<T> = std::result::Result<T, GeoError>;
