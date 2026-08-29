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

use lat_long::{Latitude, Longitude};
use rfham_core::{CountryCode, countries::DivisionCode, error::CoreError};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::BTreeMap, fmt::Display, str::FromStr};
use std::{collections::HashMap, iter::FromIterator, sync::LazyLock};
use strum::{Display as EnumDisplay, EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// CIRAF Zones - Conferencia Internacional de Radiodifusión por Altas Frecuencias.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay)]
pub struct Zone(u8);

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ZoneDefinitions(BTreeMap<String, ZoneDefinition>);

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, EnumDisplay, EnumIs, EnumTryAs, Deserialize, Serialize,
)]
#[serde(untagged)]
pub enum ZoneDefinition {
    Assertion(ZoneAssertion),
    Disjunction(ZoneDisjunction),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ZoneAssertion {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    latitude: Option<LatitudeAssertion>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    longitude: Option<LongitudeAssertion>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    region: Option<RegionAssertion>,
    zone: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ZoneDisjunction(Vec<ZoneAssertion>);

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, EnumDisplay, EnumIs, EnumTryAs, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum LatitudeAssertion {
    NorthOf(Latitude),
    SouthOf(Latitude),
    #[serde(rename = "lat-between")]
    Between(Latitude, Latitude),
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, EnumDisplay, EnumIs, EnumTryAs, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum LongitudeAssertion {
    EastOf(Longitude),
    WestOf(Longitude),
    #[serde(rename = "long-between")]
    Between(Longitude, Longitude),
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, EnumDisplay, EnumIs, EnumTryAs, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum RegionAssertion {
    Is(RegionIdentity),
    IsNot(RegionIdentity),
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, EnumDisplay, EnumIs, EnumTryAs, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum RegionIdentity {
    Country(CountryCode),
    Division(DivisionCode),
    Description(String),
}

// ------------------------------------------------------------------------------------------------
// Data Files
// ------------------------------------------------------------------------------------------------

type ZoneNameMap = HashMap<u8, String>;

static ZONE_NAME_DATA: &str = include_str!("../data/itu-zone-names.csv");
static ZONE_NAMES: LazyLock<ZoneNameMap> = LazyLock::new(|| {
    ZONE_NAME_DATA
        .lines()
        .filter_map(|line| {
            let parts = line.split_once(',')?;
            Some((
                u8::from_str(parts.0.trim()).ok()?,
                parts.1.trim().to_string(),
            ))
        })
        .collect::<HashMap<_, _>>()
});

#[allow(dead_code)]
static ZONE_MAPPING_JSON: &str = include_str!("../data/itu-zone-mapping.json");
#[allow(dead_code)]
static ZONE_MAPPING: LazyLock<ZoneDefinitions> = LazyLock::new(|| {
    serde_json::from_str(ZONE_MAPPING_JSON).expect("Failed to parse ITU prefix->zone mapping JSON")
});

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TryFrom<u8> for Zone {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(CoreError::InvalidValue(value.to_string(), "Zone"))
        }
    }
}

impl From<Zone> for u8 {
    fn from(value: Zone) -> Self {
        value.0
    }
}

impl FromStr for Zone {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u8>()
            .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "Zone"))
            .and_then(Self::try_from)
    }
}

impl Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Zone {
    pub fn is_valid(v: u8) -> bool {
        if ZONE_NAMES.is_empty() {
            (1..=85).contains(&v)
        } else {
            (1..=ZONE_NAMES.len() as u8).contains(&v)
        }
    }

    pub fn name(&self) -> Option<&str> {
        ZONE_NAMES.get(&self.0).map(|s| s.as_str())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<BTreeMap<String, ZoneDefinition>> for ZoneDefinitions {
    fn from(definitions: BTreeMap<String, ZoneDefinition>) -> Self {
        // TODO: validate that the keys are valid ITU country prefixes
        Self(definitions)
    }
}

impl From<ZoneDefinitions> for BTreeMap<String, ZoneDefinition> {
    fn from(definitions: ZoneDefinitions) -> Self {
        // TODO: validate that the keys are valid ITU country prefixes
        definitions.0
    }
}

impl AsRef<BTreeMap<String, ZoneDefinition>> for ZoneDefinitions {
    fn as_ref(&self) -> &BTreeMap<String, ZoneDefinition> {
        &self.0
    }
}

impl ZoneDefinitions {}

// ------------------------------------------------------------------------------------------------

impl From<ZoneAssertion> for ZoneDefinition {
    fn from(assertion: ZoneAssertion) -> Self {
        Self::is(assertion)
    }
}

impl From<ZoneDisjunction> for ZoneDefinition {
    fn from(disjunction: ZoneDisjunction) -> Self {
        Self::one_of(disjunction)
    }
}

impl ZoneDefinition {
    pub fn is(assertion: ZoneAssertion) -> Self {
        Self::Assertion(assertion)
    }

    pub fn one_of(assertions: ZoneDisjunction) -> Self {
        Self::Disjunction(assertions)
    }
}

// ------------------------------------------------------------------------------------------------

impl ZoneAssertion {
    pub fn new(zone: u8) -> Self {
        Self {
            latitude: None,
            longitude: None,
            region: None,
            zone,
            comment: None,
        }
    }

    pub fn with_latitude(mut self, latitude: LatitudeAssertion) -> Self {
        self.latitude = Some(latitude);
        self
    }

    pub fn with_north_of(mut self, latitude: Latitude) -> Self {
        self.latitude = Some(LatitudeAssertion::NorthOf(latitude));
        self
    }

    pub fn with_south_of(mut self, latitude: Latitude) -> Self {
        self.latitude = Some(LatitudeAssertion::SouthOf(latitude));
        self
    }

    pub fn with_latitude_between(mut self, lhs: Latitude, rhs: Latitude) -> Self {
        self.latitude = Some(LatitudeAssertion::Between(lhs, rhs));
        self
    }

    pub fn with_longitude(mut self, longitude: LongitudeAssertion) -> Self {
        self.longitude = Some(longitude);
        self
    }

    pub fn with_east_of(mut self, longitude: Longitude) -> Self {
        self.longitude = Some(LongitudeAssertion::EastOf(longitude));
        self
    }

    pub fn with_west_of(mut self, longitude: Longitude) -> Self {
        self.longitude = Some(LongitudeAssertion::WestOf(longitude));
        self
    }

    pub fn with_longitude_between(mut self, lhs: Longitude, rhs: Longitude) -> Self {
        self.longitude = Some(LongitudeAssertion::Between(lhs, rhs));
        self
    }

    pub fn with_region(mut self, region: RegionAssertion) -> Self {
        self.region = Some(region);
        self
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Vec<ZoneAssertion>> for ZoneDisjunction {
    fn from(assertions: Vec<ZoneAssertion>) -> Self {
        Self(assertions)
    }
}

impl From<&[ZoneAssertion]> for ZoneDisjunction {
    fn from(assertions: &[ZoneAssertion]) -> Self {
        Self(assertions.to_vec())
    }
}

impl FromIterator<ZoneAssertion> for ZoneDisjunction {
    fn from_iter<T: IntoIterator<Item = ZoneAssertion>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<ZoneDisjunction> for Vec<ZoneAssertion> {
    fn from(value: ZoneDisjunction) -> Self {
        value.0
    }
}

impl AsRef<Vec<ZoneAssertion>> for ZoneDisjunction {
    fn as_ref(&self) -> &Vec<ZoneAssertion> {
        &self.0
    }
}

impl AsRef<[ZoneAssertion]> for ZoneDisjunction {
    fn as_ref(&self) -> &[ZoneAssertion] {
        self.0.as_slice()
    }
}

impl ZoneDisjunction {}

// ------------------------------------------------------------------------------------------------

impl LatitudeAssertion {}

// ------------------------------------------------------------------------------------------------

impl LongitudeAssertion {}

// ------------------------------------------------------------------------------------------------

impl RegionAssertion {
    pub fn is<R: Into<RegionIdentity>>(identity: R) -> Self {
        Self::Is(identity.into())
    }
    pub fn is_not<R: Into<RegionIdentity>>(identity: R) -> Self {
        Self::IsNot(identity.into())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<CountryCode> for RegionIdentity {
    fn from(country: CountryCode) -> Self {
        Self::Country(country)
    }
}

impl From<DivisionCode> for RegionIdentity {
    fn from(division: DivisionCode) -> Self {
        Self::Division(division)
    }
}

impl From<String> for RegionIdentity {
    fn from(description: String) -> Self {
        Self::Description(description)
    }
}

impl RegionIdentity {}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lat_long::{Latitude, Longitude};
    use pretty_assertions::assert_eq;
    use rfham_core::countries::{CountryCode, DivisionCode};
    use serde_json::to_string_pretty;

    #[test]
    fn test_zone_name_map() {
        assert_eq!(ZONE_NAMES.get(&1), Some(&"Alaska".to_string()));
        assert_eq!(ZONE_NAMES.get(&2), Some(&"Western Canada".to_string()));
        assert_eq!(ZONE_NAMES.get(&3), Some(&"Central Canada west".to_string()));
        assert_eq!(ZONE_NAMES.get(&17), Some(&"Iceland".to_string()));
        assert_eq!(
            ZONE_NAMES.get(&36),
            Some(&"Azores, Canary Island, Madeira".to_string())
        );
        assert_eq!(ZONE_NAMES.get(&61), Some(&"Hawaii".to_string()));
        assert_eq!(ZONE_NAMES.get(&70), Some(&"Antarctica".to_string()));
    }

    #[test]
    fn test_zone_name() {
        assert_eq!(Zone::try_from(1).unwrap().name(), Some("Alaska"));
        assert_eq!(Zone::try_from(2).unwrap().name(), Some("Western Canada"));
        assert_eq!(
            Zone::try_from(3).unwrap().name(),
            Some("Central Canada west")
        );
        assert_eq!(Zone::try_from(17).unwrap().name(), Some("Iceland"));
        assert_eq!(
            ZONE_NAMES.get(&36),
            Some(&"Azores, Canary Island, Madeira".to_string())
        );
        assert_eq!(Zone::try_from(61).unwrap().name(), Some("Hawaii"));
        assert_eq!(Zone::try_from(70).unwrap().name(), Some("Antarctica"));
    }

    #[test]
    fn test_zone_definition_serialization() {}

    #[test]
    fn test_region_identity_serialization() {
        // Variant Country
        assert_eq!(
            to_string_pretty(&RegionIdentity::Country(
                CountryCode::from_str("US").unwrap()
            ))
            .unwrap(),
            r#"{
  "country": "US"
}"#
        );
        // Variant Division
        assert_eq!(
            to_string_pretty(&RegionIdentity::Division(
                DivisionCode::from_str("US-CA").unwrap()
            ))
            .unwrap(),
            r#"{
  "division": "US-CA"
}"#
        );
        // Variant Description
        assert_eq!(
            to_string_pretty(&RegionIdentity::Description("California".to_string())).unwrap(),
            r#"{
  "description": "California"
}"#
        );
    }

    #[test]
    fn test_region_assertion_serialization() {
        // Variant Is => Country
        assert_eq!(
            to_string_pretty(&RegionAssertion::is(CountryCode::from_str("US").unwrap())).unwrap(),
            r#"{
  "is": {
    "country": "US"
  }
}"#
        );
        // Variant Is => Division
        assert_eq!(
            to_string_pretty(&RegionAssertion::is(
                DivisionCode::from_str("US-CA").unwrap()
            ))
            .unwrap(),
            r#"{
  "is": {
    "division": "US-CA"
  }
}"#
        );
        // Variant Is => Description
        assert_eq!(
            to_string_pretty(&RegionAssertion::is("California".to_string())).unwrap(),
            r#"{
  "is": {
    "description": "California"
  }
}"#
        );
        // Variant IsNot => Country
        assert_eq!(
            to_string_pretty(&RegionAssertion::is_not(
                CountryCode::from_str("US").unwrap()
            ))
            .unwrap(),
            r#"{
  "is-not": {
    "country": "US"
  }
}"#
        );
        // Variant IsNot => Division
        assert_eq!(
            to_string_pretty(&RegionAssertion::is_not(
                DivisionCode::from_str("US-CA").unwrap()
            ))
            .unwrap(),
            r#"{
  "is-not": {
    "division": "US-CA"
  }
}"#
        );
        // Variant IsNot => Description
        assert_eq!(
            to_string_pretty(&RegionAssertion::is_not("California".to_string())).unwrap(),
            r#"{
  "is-not": {
    "description": "California"
  }
}"#
        );
    }

    #[test]
    fn test_longitude_serialization() {
        // Variant EastOf
        assert_eq!(
            to_string_pretty(&LongitudeAssertion::EastOf(
                Longitude::from_str("30E").unwrap()
            ))
            .unwrap(),
            r#"{
  "east-of": 30.0
}"#
        );
        // Variant WestOf
        assert_eq!(
            to_string_pretty(&LongitudeAssertion::WestOf(
                Longitude::from_str("30W").unwrap()
            ))
            .unwrap(),
            r#"{
  "west-of": -30.0
}"#
        );
        // Variant Between
        assert_eq!(
            to_string_pretty(&LongitudeAssertion::Between(
                Longitude::from_str("30W").unwrap(),
                Longitude::from_str("20W").unwrap()
            ))
            .unwrap(),
            r#"{
  "long-between": [
    -30.0,
    -20.0
  ]
}"#
        );
    }

    #[test]
    fn test_latitude_serialization() {
        // Variant NorthOf
        assert_eq!(
            to_string_pretty(&LatitudeAssertion::NorthOf(
                Latitude::from_str("30N").unwrap()
            ))
            .unwrap(),
            r#"{
  "north-of": 30.0
}"#
        );
        // Variant SouthOf
        assert_eq!(
            to_string_pretty(&LatitudeAssertion::SouthOf(
                Latitude::from_str("30S").unwrap()
            ))
            .unwrap(),
            r#"{
  "south-of": -30.0
}"#
        );
        // Variant Between
        assert_eq!(
            to_string_pretty(&LatitudeAssertion::Between(
                Latitude::from_str("30N").unwrap(),
                Latitude::from_str("20S").unwrap()
            ))
            .unwrap(),
            r#"{
  "lat-between": [
    30.0,
    -20.0
  ]
}"#
        );
    }

    #[test]
    fn test_zone_disjunction_serialization() {
        assert_eq!(
            to_string_pretty(&ZoneDisjunction::from(vec![
                ZoneAssertion::new(1).with_latitude(LatitudeAssertion::NorthOf(
                    Latitude::from_str("30N").unwrap()
                )),
                ZoneAssertion::new(2).with_longitude(LongitudeAssertion::EastOf(
                    Longitude::from_str("30E").unwrap()
                )),
            ]))
            .unwrap(),
            r#"[
  {
    "north-of": 30.0,
    "zone": 1
  },
  {
    "east-of": 30.0,
    "zone": 2
  }
]"#
        );
    }

    #[test]
    fn test_zone_definitions_serialization() {
        let mut definitions = BTreeMap::new();
        definitions.insert("KL".to_string(), ZoneDefinition::is(ZoneAssertion::new(1)));
        definitions.insert("VE6".to_string(), ZoneDefinition::is(ZoneAssertion::new(2)));
        definitions.insert(
            "VE8".to_string(),
            ZoneDefinition::one_of(
                vec![
                    ZoneAssertion::new(2)
                        .with_south_of(Latitude::from_str("80").unwrap().into())
                        .with_west_of(Longitude::from_str("110W").unwrap().into()),
                    ZoneAssertion::new(3)
                        .with_south_of(Latitude::from_str("80").unwrap().into())
                        .with_longitude_between(
                            Longitude::from_str("90W").unwrap().into(),
                            Longitude::from_str("110W").unwrap().into(),
                        ),
                    ZoneAssertion::new(4)
                        .with_south_of(Latitude::from_str("80").unwrap().into())
                        .with_longitude_between(
                            Longitude::from_str("70W").unwrap().into(),
                            Longitude::from_str("90W").unwrap().into(),
                        ),
                ]
                .into(),
            ),
        );
        assert_eq!(
            to_string_pretty(&ZoneDefinitions(definitions)).unwrap(),
            r#"{
  "KL": {
    "zone": 1
  },
  "VE6": {
    "zone": 2
  },
  "VE8": [
    {
      "south-of": 80.0,
      "west-of": -110.0,
      "zone": 2
    },
    {
      "south-of": 80.0,
      "long-between": [
        -90.0,
        -110.0
      ],
      "zone": 3
    },
    {
      "south-of": 80.0,
      "long-between": [
        -70.0,
        -90.0
      ],
      "zone": 4
    }
  ]
}"#
        );
    }
}
