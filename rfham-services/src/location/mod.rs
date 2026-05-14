//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::error::ServiceError;
use rfham_core::{StringLike, names::Name};
use rfham_geo::geoip::Location;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{AsRefStr, Display as EnumDisplay, EnumIs, EnumIter, EnumString};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait IpGeoService {
    fn lookup(&self) -> Result<Location, ServiceError>;
    fn provider(&self) -> Provider;
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    AsRefStr,
    EnumDisplay,
    EnumIs,
    EnumIter,
    EnumString,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Provider {
    #[default]
    #[strum(serialize = "ipinfo-legacy")]
    IpInfoLegacy,
    #[strum(serialize = "geoiplookup")]
    GeoIpLookup,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn get_default_provider() -> Result<Box<dyn IpGeoService>, ServiceError> {
    get_provider(Provider::default())
}

pub fn get_provider(provider: Provider) -> Result<Box<dyn IpGeoService>, ServiceError> {
    match provider {
        Provider::IpInfoLegacy => Ok(Box::new(ipinfo::Legacy)),
        Provider::GeoIpLookup => Ok(Box::new(geoip_lookup::Lookup)),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Provider> for Name {
    fn from(value: Provider) -> Self {
        Self::new_unchecked(value.as_ref())
    }
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

mod geoip_lookup;
mod ipinfo;
