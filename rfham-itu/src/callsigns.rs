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

use core::{fmt::Display, str::FromStr};
use rfham_core::{
    callsign::CallSign,
    country::{CountryCode, CountryCodeNumeric},
    error::CoreError,
};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::BTreeMap, sync::LazyLock};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItuSeriesAllocation {
    Country(CountryCode),
    Organization(ItuInternationalOrganization),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub enum ItuInternationalOrganization {
    InternationalCivilAviationOrganization,
    UnitedNations,
    WorldMeteorologicalOrganization,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum ItuSeries {
    Country(CountryCodeNumeric),
    SubLevel(ItuSeriesMap),
}

type ItuSeriesMap = BTreeMap<char, ItuSeries>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ItuSeriesAllocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::Country(country_code), false) => country_code.to_string(),
                (Self::Country(country_code), true) => format!("Country: {country_code:#}"),
                (Self::Organization(itu_international_organization), false) =>
                    itu_international_organization.to_string(),
                (Self::Organization(itu_international_organization), true) =>
                    format!("Organization: {itu_international_organization:#}"),
            }
        )
    }
}

impl FromStr for ItuSeriesAllocation {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(organization) = ItuInternationalOrganization::from_str(s) {
            Ok(Self::Organization(organization))
        } else if let Ok(country) = CountryCode::from_str(s) {
            Ok(Self::Country(country))
        } else {
            Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "ItuSeriesAllocation",
            ))
        }
    }
}

impl From<CountryCode> for ItuSeriesAllocation {
    fn from(value: CountryCode) -> Self {
        Self::Country(value)
    }
}

impl From<ItuInternationalOrganization> for ItuSeriesAllocation {
    fn from(value: ItuInternationalOrganization) -> Self {
        Self::Organization(value)
    }
}

impl ItuSeriesAllocation {
    pub fn from_callsign(callsign: &CallSign) -> Option<Self> {
        if ItuInternationalOrganization::is_valid(callsign.prefix()) {
            Some(ItuSeriesAllocation::Organization(
                ItuInternationalOrganization::from_str(callsign.prefix()).unwrap(),
            ))
        } else {
            itu_map_lookup(callsign.prefix())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ItuInternationalOrganization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::InternationalCivilAviationOrganization, true) =>
                    "International Civil Aviation Organization (XA)".to_string(),
                (Self::InternationalCivilAviationOrganization, false) => "XA".to_string(),
                (Self::UnitedNations, true) => "United Nations (XU)".to_string(),
                (Self::UnitedNations, false) => "XU".to_string(),
                (Self::WorldMeteorologicalOrganization, true) =>
                    "World Meteorological Organization (XM)".to_string(),
                (Self::WorldMeteorologicalOrganization, false) => "XM".to_string(),
            }
        )
    }
}

impl FromStr for ItuInternationalOrganization {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XA" => Ok(Self::InternationalCivilAviationOrganization),
            "XM" => Ok(Self::WorldMeteorologicalOrganization),
            "XU" => Ok(Self::UnitedNations),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "ItuInternationalOrganization",
            )),
        }
    }
}

impl ItuInternationalOrganization {
    pub fn is_valid(s: &str) -> bool {
        ["XA", "XM", "XU"].contains(&s)
    }

    pub fn is_valid_code(code: CountryCodeNumeric) -> bool {
        [5888, 5900, 5908].contains(&code)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// https://www.itu.int/en/ITU-R/terrestrial/fmd/Pages/call_sign_series.aspx
static ITU_PREFIX_COUNTRY_DATA: &str = include_str!("../data/itu-prefix.csv");

static ITU_PREFIX_COUNTRY_MAP: LazyLock<ItuSeriesMap> = LazyLock::new(|| {
    let mut mapping = ItuSeriesMap::default();
    for line in ITU_PREFIX_COUNTRY_DATA.split('\n') {
        let line = line.trim();
        let line = if let Some(line) = line.strip_prefix('\u{feff}') {
            // remove the UTF-8 BOM
            line
        } else {
            line
        };

        if !line.is_empty() {
            let columns: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            assert!(columns.len() == 2);
            let prefix = columns[0];
            let country = columns[1];
            assert!(prefix.is_ascii());
            assert!(country.is_ascii());
            add_to_itu_map(prefix, country, &mut mapping);
        }
    }
    mapping
});

fn add_to_itu_map(prefix: &str, country: &str, map: &mut ItuSeriesMap) {
    let country_code = CountryCode::from_str(country).unwrap().to_numeric();
    if prefix.contains('-') {
        let range: Vec<&str> = prefix.split('-').collect();
        assert_eq!(2, range.len());
        assert_eq!(
            range[0][..range[0].len() - 1],
            range[1][..range[1].len() - 1]
        );
        let prefix = &range[0][..range[0].len() - 1];
        let first = range[0].chars().last().unwrap();
        let last: char = range[0].chars().last().unwrap();
        for last in first..last {
            add_to_itu_map(&format!("{prefix}{last}"), country, map);
        }
    } else {
        let chars: Vec<char> = prefix.chars().collect();
        if chars.len() == 1 {
            map.insert(chars[0], ItuSeries::Country(country_code));
        } else {
            map.entry(chars[0])
                .or_insert_with(|| ItuSeries::SubLevel(Default::default()));
            if let Some(ItuSeries::SubLevel(next)) = map.get_mut(&chars[0]) {
                add_to_itu_map(&chars[1..].iter().collect::<String>(), country, next);
            } else {
                unreachable!();
            }
        }
    }
}

pub(crate) fn itu_map_lookup(prefix: &str) -> Option<ItuSeriesAllocation> {
    itu_map_lookup_inner(prefix, &ITU_PREFIX_COUNTRY_MAP)
}

fn itu_map_lookup_inner(prefix: &str, map: &ItuSeriesMap) -> Option<ItuSeriesAllocation> {
    if prefix.is_empty() {
        return None;
    }
    #[allow(clippy::iter_nth_zero)]
    let prefix_char = prefix.chars().nth(0).unwrap();
    match map.get(&prefix_char) {
        Some(ItuSeries::Country(code)) => {
            let decoded = CountryCode::try_from(*code).unwrap();
            if ItuInternationalOrganization::is_valid(decoded.as_str()) {
                Some(ItuSeriesAllocation::Organization(
                    ItuInternationalOrganization::from_str(decoded.as_str()).unwrap(),
                ))
            } else {
                Some(ItuSeriesAllocation::Country(decoded))
            }
        }
        Some(ItuSeries::SubLevel(next)) => itu_map_lookup_inner(&prefix[1..], next),
        _ => None,
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::ItuSeriesAllocation;
    use pretty_assertions::assert_eq;
    use rfham_core::{callsign::CallSign, country::CountryCode};
    use std::str::FromStr;

    #[test]
    fn test_callsign_components() {
        let callsign = CallSign::from_str("K7SKJ/M").unwrap();
        assert_eq!(None, callsign.ancillary_prefix());
        assert_eq!("K", callsign.prefix().as_str());
        assert_eq!(7, callsign.separator_numeral());
        assert_eq!("SKJ", callsign.suffix().as_str());
        assert_eq!(Some("M"), callsign.ancillary_suffix().map(|s| s.as_str()));

        assert_eq!(
            Some(ItuSeriesAllocation::Country(
                CountryCode::from_str("US").unwrap()
            )),
            ItuSeriesAllocation::from_callsign(&callsign)
        );

        assert!(!callsign.is_special());
    }
}
