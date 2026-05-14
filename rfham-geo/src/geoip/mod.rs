//! IP-to-location lookup types and the [`Provider`] trait.
//!
//! This module defines the data model returned by any geo-IP lookup: [`Location`] is the
//! top-level result, continent, country, city, etc.
//!
//!
//! # Data model
//!
//! ```text
//! location:            Location
//! ├── continent:       Continent
//! │   └── code:        ContinentCode
//! │   └── name:        String
//! ├── country:         Country
//! │   └── code:        CountryCode,
//! │   └── name:        Option<String>
//! ├── region:          String
//! ├── city:            String
//! ├── postal_code:     String
//! ├── timezone:        TimeZone
//! │   └── name:        String
//! └── geo:             GeoLocation
//!     ├── latitidue:   Latitude
//!     ├── longitude:   Longitude
//!     └── accuracy:    Option<Length>
//! ```
//!
//! # Examples
//!
//! ```rust
//! use rfham_geo::geoip::{IpGeoData, Location, Code, ContinentCode, GeoLocation};
//! use rfham_core::CountryCode;
//! use lat_long::{Coordinate, Latitude, Longitude};
//! use std::{net::IpAddr, str::FromStr};
//!
//! let location = Location::new(
//!     Code::new(ContinentCode::NA, "North America"),
//!     Code::new(CountryCode::from_str("US").unwrap(), "United States"),
//! );
//! let data = IpGeoData::new("203.0.113.1".parse::<IpAddr>().unwrap(), location);
//! assert_eq!(data.location().continent().code(), &ContinentCode::NA);
//! assert_eq!(data.location().country().code().to_string(), "US");
//! ```
//!

use lat_long::{Coordinate, Latitude, Longitude};
use rfham_core::CountryCode;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::fmt::{Debug, Display};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use uom::si::f64::Length;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Location {
    continent: Option<Continent>,
    country: Country,
    region: String,
    city: String,
    postal_code: String,
    timezone: TimeZone,
    geo: GeoLocation,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeoLocation {
    point: Coordinate,
    accuracy: Option<Length>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Country {
    code: CountryCode,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TimeZone {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Continent {
    code: ContinentCode,
    name: String,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    EnumDisplay,
    EnumIs,
    EnumIter,
    EnumString,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum ContinentCode {
    /// Africa
    AF,
    /// North America
    NA,
    /// Oceania
    OC,
    /// Antarctica
    AN,
    /// Asia
    AS,
    /// Europe
    EU,
    /// South America
    SA,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Structures
// ------------------------------------------------------------------------------------------------

impl Location {
    pub const fn new(
        continent: Option<Continent>,
        country: Country,
        region: String,
        city: String,
        postal_code: String,
        timezone: TimeZone,
        geo: GeoLocation,
    ) -> Self {
        Self {
            continent,
            country,
            region,
            city,
            postal_code,
            timezone,
            geo,
        }
    }

    pub const fn continent(&self) -> Option<&Continent> {
        self.continent.as_ref()
    }

    pub const fn country(&self) -> &Country {
        &self.country
    }

    pub const fn region(&self) -> &str {
        self.region.as_str()
    }

    pub const fn city(&self) -> &str {
        self.city.as_str()
    }

    pub const fn postal_code(&self) -> &str {
        self.postal_code.as_str()
    }

    pub const fn geo(&self) -> &GeoLocation {
        &self.geo
    }

    pub const fn timezone(&self) -> &TimeZone {
        &self.timezone
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Coordinate> for GeoLocation {
    fn from(value: Coordinate) -> Self {
        Self::new(value)
    }
}

impl GeoLocation {
    pub const fn new(point: Coordinate) -> Self {
        Self {
            point,
            accuracy: None,
        }
    }

    pub fn with_accuracy(mut self, accuracy: Length) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    pub const fn point(&self) -> Coordinate {
        self.point
    }

    pub const fn longitude(&self) -> Longitude {
        self.point.longitude()
    }

    pub const fn latitude(&self) -> Latitude {
        self.point.latitude()
    }

    pub const fn accuracy(&self) -> Option<&Length> {
        self.accuracy.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl ContinentCode {
    pub fn name(&self) -> &str {
        match self {
            Self::AF => "Africa",
            Self::AN => "Antarctica",
            Self::AS => "Asia",
            Self::EU => "Europe",
            Self::NA => "North America",
            Self::OC => "Oceania",
            Self::SA => "South America",
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Continent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

impl From<ContinentCode> for Continent {
    fn from(value: ContinentCode) -> Self {
        Self::new(value, value.name().to_string())
    }
}

impl Continent {
    pub fn new<S: Into<String>>(code: ContinentCode, name: S) -> Self {
        Self {
            code,
            name: name.into(),
        }
    }

    pub fn code(&self) -> &ContinentCode {
        &self.code
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

impl From<CountryCode> for Country {
    fn from(value: CountryCode) -> Self {
        Self::new(value.clone(), value.to_string())
    }
}

impl Country {
    pub fn new<S: Into<String>>(code: CountryCode, name: S) -> Self {
        Self {
            code,
            name: name.into(),
        }
    }

    pub fn code(&self) -> &CountryCode {
        &self.code
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for TimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TimeZone {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lat_long::{Latitude, Longitude};
    use pretty_assertions::assert_eq;
    use serde_json::to_string_pretty;
    use std::str::FromStr;

    #[test]
    fn test_serialize_roundtrip() {
        let data = Location::new(
            ContinentCode::NA.into(),
            Country::new("US".parse().unwrap(), "United States"),
            "Texas".to_string(),
            "Irving".to_string(),
            "994894".to_string(),
            TimeZone::new("America/Chicago"),
            Coordinate::new(
                Latitude::from_str("32.814").unwrap(),
                Longitude::from_str("-96.9489").unwrap(),
            )
            .into(),
        );

        let json = to_string_pretty(&data).unwrap();
        assert!(json.contains("23.64.167.34"));
        assert!(json.contains("Texas"));

        let deserialized: Location = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_continent_code_roundtrip() {
        for (s, code) in [
            ("AF", ContinentCode::AF),
            ("AN", ContinentCode::AN),
            ("AS", ContinentCode::AS),
            ("EU", ContinentCode::EU),
            ("NA", ContinentCode::NA),
            ("OC", ContinentCode::OC),
            ("SA", ContinentCode::SA),
        ] {
            assert_eq!(code.to_string(), s);
            assert_eq!(ContinentCode::from_str(s).unwrap(), code);
        }
    }

    #[test]
    fn test_continent_code_name() {
        assert_eq!(ContinentCode::NA.name(), "North America");
        assert_eq!(ContinentCode::EU.name(), "Europe");
    }

    #[test]
    fn test_continent_code_invalid() {
        assert!(ContinentCode::from_str("XX").is_err());
    }
}
