//! Concrete [`Provider`](crate::geoip::Provider) implementations.
//!
//! | Type | Source | License |
//! |------|--------|---------|
//! | [`GeoIpLookup`] | `json.geoiplookup.io` public REST API | Public |
//! | [`NoOp`] | Always returns `None` — useful for tests | Public |
//!
//! The [`local`] sub-module additionally provides [`local::IpNetwork`] for CIDR-range
//! matching, which is used by local lookup tables that may be added in the future.

use crate::{
    error::{GeoError, GeoResult},
    geoip::{Asn, Code, GeoLocation, IpGeoData, Locale, Location, Provider, ProviderDataLicense},
};
use lat_long::{Coordinate, Latitude, Longitude};
use rfham_core::error::CoreError;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, net::IpAddr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Describe this struct.
///
/// # Examples
///
/// ```rust,no_run
/// use rfham_geo::geoip::{Provider, providers::GeoIpLookup};
/// use std::{net::IpAddr, str::FromStr};
///
/// let service = GeoIpLookup();
/// match service.lookup(&IpAddr::from_str("23.64.167.34").unwrap()) {
///     Ok(Some(data)) => println!("Found data: {data:#?}"),
///     Ok(None) => println!("No data for IP address"),
///     Err(e) => println!("Service error: {e}"),
/// }
///
/// ```
///
/// This uses the publicly accessible API at `https://json.geoiplookup.io/{ip}` which returns a
/// structure as shown below.
///
/// ```json
/// {
///     "ip": "23.64.167.34",
///     "isp": "Akamai Technologies, Inc.",
///     "org": "Akamai Technologies, Inc.",
///     "hostname": "",
///     "latitude": 32.814,
///     "longitude": -96.9489,
///     "postal_code": "",
///     "city": "Irving",
///     "country_code": "US",
///     "country_name": "United States",
///     "continent_code": "NA",
///     "continent_name": "North America",
///     "region": "Texas",
///     "district": "",
///     "timezone_name": "America/Chicago",
///     "connection_type": "Corporate",
///     "asn_number": 16625,
///     "asn_org": "Akamai Technologies, Inc.",
///     "asn": "AS16625 - Akamai Technologies, Inc.",
///     "currency_code": "USD",
///     "currency_name": "United States Dollar",
///     "language_code": "en",
///     "language_name": "English",
///     "success": true,
///     "premium": false
/// }
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GeoIpLookup();

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoOp();

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct GeoIpResponse {
    ip: IpAddr,
    isp: String,
    org: String,
    hostname: String,
    latitude: Latitude,
    longitude: Longitude,
    postal_code: String,
    city: String,
    country_code: String,
    country_name: String,
    continent_code: String,
    continent_name: String,
    region: String,
    district: String,
    timezone_name: String,
    connection_type: String,
    asn_number: u64,
    asn_org: String,
    asn: String,
    currency_name: String,
    currency_code: String,
    language_code: String,
    language_name: String,
    success: bool,
    premium: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const GEOIP_LOOKUP_URL: &str = "https://json.geoiplookup.io/";

impl Provider for GeoIpLookup {
    fn lookup(&self, address: &std::net::IpAddr) -> GeoResult<Option<IpGeoData>> {
        let response = reqwest::blocking::get(format!("{GEOIP_LOOKUP_URL}{address}"))?;
        if response.status().is_success() {
            let body = response.text()?;
            let parsed: GeoIpResponse = serde_json::from_str(&body)?;
            if parsed.success {
                Ok(Some(parsed.try_into()?))
            } else {
                Ok(None)
            }
        } else {
            Err(GeoError::Http(response.status()))
        }
    }

    fn license(&self) -> ProviderDataLicense {
        ProviderDataLicense::Public
    }
}

// ------------------------------------------------------------------------------------------------

impl TryFrom<GeoIpResponse> for IpGeoData {
    type Error = CoreError;

    fn try_from(response: GeoIpResponse) -> Result<Self, Self::Error> {
        let location = Location::new(
            Code::new(response.continent_code.parse()?, response.continent_name),
            Code::new(response.country_code.parse()?, response.country_name),
        );
        let location = if !response.region.is_empty() {
            location.with_region(response.region)
        } else {
            location
        };
        let location = if !response.city.is_empty() {
            location.with_city(response.city)
        } else {
            location
        };
        let location = if !response.district.is_empty() {
            location.with_district(response.district)
        } else {
            location
        };
        let location = if !response.postal_code.is_empty() {
            location.with_postal_code(response.postal_code)
        } else {
            location
        };
        let location = location.with_location(GeoLocation::new(Coordinate::new(
            response.latitude,
            response.longitude,
        )));

        let data = IpGeoData::new(response.ip, location);

        let data = if !response.timezone_name.is_empty()
            || !response.currency_code.is_empty()
            || !response.language_code.is_empty()
        {
            let locale = Locale::default();

            let locale = if !response.timezone_name.is_empty() {
                locale.with_timezone(response.timezone_name)
            } else {
                locale
            };
            let locale = if !response.currency_code.is_empty() {
                locale.with_currency(Code::new(
                    response.currency_code.parse()?,
                    response.currency_name,
                ))
            } else {
                locale
            };
            let locale = if !response.language_code.is_empty() {
                locale.with_language(Code::new(
                    response.language_code.parse()?,
                    response.language_name,
                ))
            } else {
                locale
            };
            data.with_locale(locale)
        } else {
            data
        };

        let data = if response.asn_number != 0 {
            data.with_asn(Asn::new(
                response.asn_number,
                response.asn,
                response.asn_org,
            ))
        } else {
            data
        };

        Ok(data)
    }
}

// ------------------------------------------------------------------------------------------------

impl Provider for NoOp {
    fn lookup(&self, _: &std::net::IpAddr) -> GeoResult<Option<IpGeoData>> {
        Ok(None)
    }

    fn license(&self) -> ProviderDataLicense {
        ProviderDataLicense::Public
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod local;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_geoip_response() {
        const RESPONSE: &str = r##"{
    "ip": "23.64.167.34",
    "isp": "Akamai Technologies, Inc.",
    "org": "Akamai Technologies, Inc.",
    "hostname": "",
    "latitude": 32.814,
    "longitude": -96.9489,
    "postal_code": "",
    "city": "Irving",
    "country_code": "US",
    "country_name": "United States",
    "continent_code": "NA",
    "continent_name": "North America",
    "region": "Texas",
    "district": "",
    "timezone_name": "America/Chicago",
    "connection_type": "Corporate",
    "asn_number": 16625,
    "asn_org": "Akamai Technologies, Inc.",
    "asn": "AS16625 - Akamai Technologies, Inc.",
    "currency_code": "USD",
    "currency_name": "United States Dollar",
    "language_code": "en",
    "language_name": "English",
    "success": true,
    "premium": false
}"##;
        let parsed: Result<GeoIpResponse, serde_json::Error> = serde_json::from_str(RESPONSE);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        assert_eq!("23.64.167.34".to_string(), parsed.ip.to_string());
    }

    #[test]
    fn test_geoip_response_to_data() {
        const RESPONSE: &str = r##"{
    "ip": "23.64.167.34",
    "isp": "Akamai Technologies, Inc.",
    "org": "Akamai Technologies, Inc.",
    "hostname": "",
    "latitude": 32.814,
    "longitude": -96.9489,
    "postal_code": "",
    "city": "Irving",
    "country_code": "US",
    "country_name": "United States",
    "continent_code": "NA",
    "continent_name": "North America",
    "region": "Texas",
    "district": "",
    "timezone_name": "America/Chicago",
    "connection_type": "Corporate",
    "asn_number": 16625,
    "asn_org": "Akamai Technologies, Inc.",
    "asn": "AS16625 - Akamai Technologies, Inc.",
    "currency_code": "USD",
    "currency_name": "United States Dollar",
    "language_code": "en",
    "language_name": "English",
    "success": true,
    "premium": false
}"##;
        let parsed: GeoIpResponse = serde_json::from_str(RESPONSE).unwrap();
        let data: IpGeoData = parsed.try_into().unwrap();
        assert_eq!("NA", data.location().continent().code().to_string());
        assert_eq!("US", data.location().country().code().to_string());
        assert_eq!(16625, data.asn().unwrap().number());
    }

    #[test]
    fn test_noop_provider() {
        let provider = NoOp::default();
        let ip_address: IpAddr = "23.64.167.34".parse().unwrap();
        assert_eq!(None, provider.lookup(&ip_address).unwrap())
    }
}
