//! IP-to-location lookup types and the [`Provider`] trait.
//!
//! This module defines the data model returned by any geo-IP lookup: [`IpGeoData`] is the
//! top-level result, containing a [`Location`] (continent, country, city …), an optional
//! [`Locale`] (timezone, currency, language), and an optional [`Asn`] (autonomous system).
//!
//! Concrete provider implementations live in [`providers`].
//!
//! # Data model
//!
//! ```text
//! IpGeoData
//! ├── ip_address: IpAddr
//! ├── location: Location
//! │   ├── continent: Code<ContinentCode>
//! │   ├── country:   Code<CountryCode>
//! │   └── location:  Option<GeoLocation>  (coordinate + accuracy)
//! ├── hostname: Option<String>
//! ├── locale:   Option<Locale>            (timezone, currency, language)
//! └── asn:      Option<Asn>              (AS number, name, org)
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

use crate::error::GeoResult;
use lat_long::{Coordinate, Latitude, Longitude};
use rfham_core::{CountryCode, error::CoreError};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::{Debug, Display},
    net::IpAddr,
    str::FromStr,
};
use uom::si::f64::Length;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A geo-IP lookup service that maps an IP address to location data.
pub trait Provider {
    /// Look up location data for `address`. Returns `None` if the provider has no data
    /// for the given address (e.g. a private/reserved range).
    fn lookup(&self, address: &IpAddr) -> GeoResult<Option<IpGeoData>>;

    /// Describes the licence under which this provider's data is available.
    fn license(&self) -> ProviderDataLicense;
}

/// Indicates how the data returned by a provider may be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProviderDataLicense {
    /// Freely accessible with no licence restrictions (e.g. a public API).
    Public,
    /// Requires a service agreement with the data provider.
    ServiceLicensed,
    /// Requires the client application to hold a licence.
    ClientLicensed,
}

/// The result of a geo-IP lookup: location, locale, and network information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IpGeoData {
    ip_address: IpAddr,
    location: Location,
    hostname: Option<String>,
    locale: Option<Locale>,
    asn: Option<Asn>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Location {
    continent: Code<ContinentCode>,
    country: Code<CountryCode>,
    location: Option<GeoLocation>,
    region: Option<String>,
    city: Option<String>,
    district: Option<String>,
    postal_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeoLocation {
    coordinate: Coordinate,
    accuracy: Option<Length>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Locale {
    timezone: Option<String>,
    currency: Option<Code<CurrencyCode>>,
    language: Option<Code<LanguageCode>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Asn {
    number: u64,
    name: String,
    organization: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Code<T>
where
    T: Clone + Debug + Display + PartialEq + Eq,
{
    code: T,
    label: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
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

/// ISO 4217 three-letter currency code (e.g. `"USD"`, `"EUR"`).
#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct CurrencyCode(String);

/// ISO 639-1 two-letter language code (e.g. `"en"`, `"ja"`).
#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct LanguageCode(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Structures
// ------------------------------------------------------------------------------------------------

impl IpGeoData {
    pub const fn new(ip_address: IpAddr, location: Location) -> Self {
        Self {
            ip_address,
            location,
            hostname: None,
            locale: None,
            asn: None,
        }
    }

    pub fn with_hostname<S: Into<String>>(mut self, hostname: S) -> Self {
        self.hostname = Some(hostname.into());
        self
    }

    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = Some(locale);
        self
    }

    pub fn with_asn(mut self, asn: Asn) -> Self {
        self.asn = Some(asn);
        self
    }

    pub const fn ip_address(&self) -> &IpAddr {
        &self.ip_address
    }

    pub const fn location(&self) -> &Location {
        &self.location
    }

    pub const fn locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub const fn asn(&self) -> Option<&Asn> {
        self.asn.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Location {
    pub const fn new(continent: Code<ContinentCode>, country: Code<CountryCode>) -> Self {
        Self {
            continent,
            country,
            location: None,
            region: None,
            city: None,
            district: None,
            postal_code: None,
        }
    }
    pub fn with_location(mut self, location: GeoLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_region<S: Into<String>>(mut self, region: S) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn with_city<S: Into<String>>(mut self, city: S) -> Self {
        self.city = Some(city.into());
        self
    }

    pub fn with_district<S: Into<String>>(mut self, district: S) -> Self {
        self.district = Some(district.into());
        self
    }

    pub fn with_postal_code<S: Into<String>>(mut self, postal_code: S) -> Self {
        self.postal_code = Some(postal_code.into());
        self
    }

    pub const fn continent(&self) -> &Code<ContinentCode> {
        &self.continent
    }

    pub const fn country(&self) -> &Code<CountryCode> {
        &self.country
    }

    pub const fn location(&self) -> Option<&GeoLocation> {
        self.location.as_ref()
    }

    pub const fn region(&self) -> Option<&String> {
        self.region.as_ref()
    }

    pub const fn city(&self) -> Option<&String> {
        self.city.as_ref()
    }

    pub const fn district(&self) -> Option<&String> {
        self.district.as_ref()
    }

    pub const fn postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Coordinate> for GeoLocation {
    fn from(value: Coordinate) -> Self {
        Self::new(value)
    }
}

impl GeoLocation {
    pub const fn new(coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            accuracy: None,
        }
    }

    pub fn with_accuracy(mut self, accuracy: Length) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    pub const fn coordinate(&self) -> Coordinate {
        self.coordinate
    }

    pub const fn longitude(&self) -> Longitude {
        self.coordinate.longitude()
    }

    pub const fn latitude(&self) -> Latitude {
        self.coordinate.latitude()
    }

    pub const fn accuracy(&self) -> Option<&Length> {
        self.accuracy.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Locale {
    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    pub fn with_currency(mut self, currency: Code<CurrencyCode>) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_language(mut self, language: Code<LanguageCode>) -> Self {
        self.language = Some(language);
        self
    }

    pub const fn timezone(&self) -> Option<&String> {
        self.timezone.as_ref()
    }

    pub const fn currency(&self) -> Option<&Code<CurrencyCode>> {
        self.currency.as_ref()
    }

    pub const fn language(&self) -> Option<&Code<LanguageCode>> {
        self.language.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Asn {
    pub const fn new(number: u64, name: String, organization: String) -> Self {
        Self {
            number,
            name,
            organization,
        }
    }

    pub const fn number(&self) -> u64 {
        self.number
    }

    pub const fn name(&self) -> &String {
        &self.name
    }

    pub const fn organization(&self) -> &String {
        &self.organization
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Codes
// ------------------------------------------------------------------------------------------------

impl<T> Display for Code<T>
where
    T: Clone + Debug + Display + PartialEq + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                format!("{}: {}", self.code, self.label)
            } else {
                self.label.to_string()
            }
        )
    }
}

impl<T> Code<T>
where
    T: Clone + Debug + Display + PartialEq + Eq,
{
    pub fn new<S: Into<String>>(code: T, label: S) -> Self {
        Self {
            code,
            label: label.into(),
        }
    }

    pub const fn code(&self) -> &T {
        &self.code
    }

    pub const fn label(&self) -> &String {
        &self.label
    }
}
// ------------------------------------------------------------------------------------------------

impl Display for ContinentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AF => "AF",
                Self::AN => "AN",
                Self::AS => "AS",
                Self::EU => "EU",
                Self::NA => "NA",
                Self::OC => "OC",
                Self::SA => "SA",
            }
        )
    }
}

impl FromStr for ContinentCode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AF" => Ok(Self::AF),
            "AN" => Ok(Self::AN),
            "AS" => Ok(Self::AS),
            "EU" => Ok(Self::EU),
            "NA" => Ok(Self::NA),
            "OC" => Ok(Self::OC),
            "SA" => Ok(Self::SA),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "ContinentCode",
            )),
        }
    }
}

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

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CurrencyCode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "CurrencyCode",
            ))
        }
    }
}

impl CurrencyCode {
    pub fn is_valid(s: &str) -> bool {
        s.len() == 3 && s.chars().all(|c| c.is_ascii_uppercase())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LanguageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for LanguageCode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "LanguageCode",
            ))
        }
    }
}

impl LanguageCode {
    pub fn is_valid(s: &str) -> bool {
        s.len() == 2 && s.chars().all(|c| c.is_ascii_lowercase())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

pub mod providers;

#[cfg(test)]
mod tests {
    use super::*;
    use lat_long::{Latitude, Longitude};
    use pretty_assertions::assert_eq;
    use serde_json::to_string_pretty;

    #[test]
    fn test_serialize_roundtrip() {
        let data = IpGeoData::new(
            IpAddr::from_str("23.64.167.34").unwrap(),
            Location {
                continent: Code {
                    code: ContinentCode::NA,
                    label: "North America".to_string(),
                },
                country: Code {
                    code: "US".parse().unwrap(),
                    label: "United States".to_string(),
                },
                location: Some(GeoLocation {
                    coordinate: Coordinate::new(
                        Latitude::from_str("32.814").unwrap(),
                        Longitude::from_str("-96.9489").unwrap(),
                    ),
                    accuracy: None,
                }),
                region: Some("Texas".to_string()),
                city: Some("Irving".to_string()),
                district: None,
                postal_code: None,
            },
        )
        .with_locale(Locale {
            timezone: Some("America/Chicago".to_string()),
            currency: Some(Code {
                code: CurrencyCode::from_str("USD").unwrap(),
                label: "United States Dollar".to_string(),
            }),
            language: Some(Code {
                code: LanguageCode::from_str("en").unwrap(),
                label: "English".to_string(),
            }),
        });

        let json = to_string_pretty(&data).unwrap();
        assert!(json.contains("23.64.167.34"));
        assert!(json.contains("Texas"));

        let deserialized: IpGeoData = serde_json::from_str(&json).unwrap();
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

    #[test]
    fn test_currency_code_valid() {
        assert!(CurrencyCode::from_str("USD").is_ok());
        assert!(CurrencyCode::from_str("EUR").is_ok());
        assert!(CurrencyCode::from_str("JPY").is_ok());
    }

    #[test]
    fn test_currency_code_invalid() {
        assert!(CurrencyCode::from_str("us").is_err());   // lowercase
        assert!(CurrencyCode::from_str("USDD").is_err()); // 4 chars
        assert!(CurrencyCode::from_str("US").is_err());   // 2 chars
    }

    #[test]
    fn test_language_code_valid() {
        assert!(LanguageCode::from_str("en").is_ok());
        assert!(LanguageCode::from_str("ja").is_ok());
    }

    #[test]
    fn test_language_code_invalid() {
        assert!(LanguageCode::from_str("EN").is_err()); // uppercase
        assert!(LanguageCode::from_str("eng").is_err()); // 3 chars
    }

    #[test]
    fn test_code_display() {
        let c = Code::new(ContinentCode::EU, "Europe");
        assert_eq!(c.to_string(), "Europe");
        assert_eq!(format!("{c:#}"), "EU: Europe");
    }

    #[test]
    fn test_ip_geo_data_accessors() {
        let location = Location::new(
            Code::new(ContinentCode::NA, "North America"),
            Code::new("US".parse::<rfham_core::CountryCode>().unwrap(), "United States"),
        );
        let data = IpGeoData::new("203.0.113.1".parse::<IpAddr>().unwrap(), location);
        assert_eq!(data.ip_address().to_string(), "203.0.113.1");
        assert_eq!(data.location().continent().code(), &ContinentCode::NA);
        assert!(data.locale().is_none());
        assert!(data.asn().is_none());
    }
}
