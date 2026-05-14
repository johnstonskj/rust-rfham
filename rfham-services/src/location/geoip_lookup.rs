use crate::{
    error::ServiceError,
    location::{IpGeoService, Provider},
};
use lat_long::{Coordinate, Latitude, Longitude};
use rfham_core::CountryCode;
use rfham_geo::geoip::{Continent, ContinentCode, Country, Location, TimeZone};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tracing::{error, info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Lookup;

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
    country_code: CountryCode,
    country_name: String,
    continent_code: ContinentCode,
    continent_name: String,
    region: String,
    district: String,
    timezone_name: String,
    connection_type: String,
    asn_number: u64, // Autonomous System Numbers
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
// Implementations ❯ Service
// ------------------------------------------------------------------------------------------------

const GEOIP_LOOKUP_URL: &str = "https://json.geoiplookup.io/";

impl IpGeoService for Lookup {
    fn lookup(&self) -> Result<Location, ServiceError> {
        trace!("Lookup::lookup(): {GEOIP_LOOKUP_URL}");
        // TODO: set user-agent
        let response = reqwest::blocking::get(format!("{GEOIP_LOOKUP_URL}"))?;
        if response.status().is_success() {
            let status = response.status();
            let body = response.text()?;
            let parsed: GeoIpResponse = serde_json::from_str(&body)?;
            info!("Lookup::lookup => result({status}): {parsed:?}");
            if parsed.success {
                Ok(parsed.try_into()?)
            } else {
                error!("Legacy::lookup => status: {}", status);
                Err(ServiceError::NotFound(self.provider().to_string(), None))
            }
        } else {
            error!("IpGeoService::lookup => status: {}", response.status());
            Err(ServiceError::Http(response.status()))
        }
    }

    fn provider(&self) -> Provider {
        Provider::GeoIpLookup
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Response
// ------------------------------------------------------------------------------------------------

impl TryFrom<GeoIpResponse> for Location {
    type Error = ServiceError;

    fn try_from(response: GeoIpResponse) -> Result<Self, Self::Error> {
        Ok(Location::new(
            Some(Continent::new(
                response.continent_code,
                response.continent_name,
            )),
            Country::new(response.country_code, response.country_name),
            response.region,
            response.city,
            response.postal_code,
            TimeZone::new(response.timezone_name),
            Coordinate::new(response.latitude, response.longitude).into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::GeoIpResponse;
    use lat_long::{Coordinate, Latitude, Longitude};
    use pretty_assertions::assert_eq;
    use rfham_core::CountryCode;
    use rfham_geo::geoip::{Continent, ContinentCode, Country, Location, TimeZone};
    use serde_json::from_str;
    use std::str::FromStr;

    const EXAMPLE_JSON: &str = r##"{ 
    "ip":"76.22.38.217",
    "isp":"Comcast Cable Communications, LLC",
    "org":"Comcast Cable Communications, Inc.",
    "hostname":"",
    "latitude":47.6101,
    "longitude":-122.202,
    "postal_code":"",
    "city":"Bellevue",
    "country_code":"US",
    "country_name":"United States",
    "continent_code":"NA",
    "continent_name":"North America",
    "region":"Washington",
    "district":"",
    "timezone_name":"America/Los_Angeles",
    "connection_type":"Cable/DSL",
    "asn_number":7922,
    "asn_org":"Comcast Cable Communications, LLC",
    "asn":"AS7922 - Comcast Cable Communications, LLC",
    "currency_code":"USD",
    "currency_name":"United States Dollar",
    "language_code":"en",
    "language_name":"English",
    "success":true,
    "premium":false
}"##;

    #[test]
    fn test_deserialize_and_into() {
        let parsed: GeoIpResponse = from_str(&EXAMPLE_JSON).unwrap();
        let location: Location = parsed.try_into().unwrap();
        assert_eq!(
            Location::new(
                Some(Continent::new(
                    ContinentCode::from_str("NA").unwrap(),
                    "North America".to_string(),
                )),
                Country::new(
                    CountryCode::from_str("US").unwrap(),
                    "United States".to_string()
                ),
                "Washington".to_string(),
                "Bellevue".to_string(),
                "".to_string(),
                TimeZone::new("America/Los_Angeles".to_string()),
                Coordinate::new(
                    Latitude::from_str("47.6101").unwrap(),
                    Longitude::from_str("-122.202").unwrap()
                )
                .into(),
            ),
            location
        );
    }
}
