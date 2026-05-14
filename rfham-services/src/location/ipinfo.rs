use crate::{
    error::ServiceError,
    location::{IpGeoService, Provider},
};
use lat_long::Coordinate;
use rfham_core::CountryCode;
use rfham_geo::geoip::{Location, TimeZone};
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, str::FromStr};
use tracing::{error, info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
///
/// # Example API Response
///
/// ```json
/// {
///   "ip": "...",
///   "hostname": "...",
///   "city": "...",
///   "region": "...",
///   "country": "US",
///   "loc": "36.1329,-94.1655",
///   "org": "...",
///   "postal": "...",
///   "timezone": "America/Chicago",
///   "readme": "https://ipinfo.io/missingauth"
/// }
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Legacy;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct LegacyLocation {
    ip: IpAddr,
    hostname: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    postal: String,
    timezone: String,
    readme: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Service
// ------------------------------------------------------------------------------------------------

const IPINFO_LEGACY_URI: &str = "https://ipinfo.io/";

impl IpGeoService for Legacy {
    fn lookup(&self) -> Result<Location, ServiceError> {
        trace!("Legacy::lookup(): {IPINFO_LEGACY_URI}");
        // TODO: set user-agent
        let response = reqwest::blocking::get(format!("{IPINFO_LEGACY_URI}"))?;
        if response.status().is_success() {
            let body = response.text()?;
            let parsed: LegacyLocation = serde_json::from_str(&body)?;
            info!("Legacy::lookup => result: {parsed:?}");
            Ok(Location::try_from(parsed)?)
        } else {
            error!("Legacy::lookup => status: {}", response.status());
            Err(ServiceError::Http(response.status()))
        }
    }

    fn provider(&self) -> Provider {
        Provider::IpInfoLegacy
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Response
// ------------------------------------------------------------------------------------------------

impl TryFrom<LegacyLocation> for Location {
    type Error = ServiceError;

    fn try_from(value: LegacyLocation) -> Result<Self, Self::Error> {
        Ok(Location::new(
            None,
            CountryCode::from_str(&value.country)?.into(),
            value.region,
            value.city,
            value.postal,
            TimeZone::new(value.timezone),
            Coordinate::from_str(&value.loc)?.into(),
        ))
    }
}
