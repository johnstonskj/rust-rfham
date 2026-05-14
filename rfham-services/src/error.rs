//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use lat_long::Error as LatLongError;
use reqwest::{Error as RequestError, StatusCode};
use rfham_config::error::ConfigError;
use rfham_core::error::CoreError;
use serde_json::Error as JsonError;
use serde_xml_rs::Error as XmlError;
use std::io::Error as IoError;
use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("An I/O error occured; error: {0}")]
    Io(#[from] IoError),

    #[error("An rfham-core error occured; error: {0}")]
    Core(#[from] CoreError),

    #[error("An rfham-config error occured; error: {0}")]
    Config(#[from] ConfigError),

    #[error("An error occured converting latitude/longitude value; error: {0}")]
    LatLong(#[from] LatLongError),

    #[error("An HTTP service returned non-success code ; status-code: {0}")]
    Http(StatusCode),

    #[error("An error occured calling an HTTP service; status-code: {0}")]
    Reqwest(#[from] RequestError),

    #[error("An error occured serializing/deserializing JSON; error: {0}")]
    Json(#[from] JsonError),

    #[error("An error occured serializing/deserializing XML; error: {0}")]
    Xml(#[from] XmlError),

    #[error("An error occured trying to use a poisoned lock; error: {0}")]
    Poison(String),

    #[error("An error occurred trying to authenticate with the {0} service; error: {1}")]
    Authentication(String, String),

    #[error("An error occurred trying to retrieve credentals for the {0} service")]
    MissingCredentials(String),

    #[error("Service {0} returned no data for request; key: {1:?}")]
    NotFound(String, Option<String>),

    #[error("Could not open browser to URL {0}")]
    BrowserOpen(String),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
