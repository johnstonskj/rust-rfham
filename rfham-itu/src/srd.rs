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

use rfham_core::error::CoreError;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::BTreeMap, fmt::Display, str::FromStr, sync::LazyLock};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, DeserializeFromStr, SerializeDisplay,
)]
pub struct SpecialRegionDesignator(String);

// ------------------------------------------------------------------------------------------------
// Data
// ------------------------------------------------------------------------------------------------

type RegionNameMap = BTreeMap<String, String>;

static REGION_NAME_DATA: &str = include_str!("../data/itu-special-region-designators.csv");
static REGION_NAMES: LazyLock<RegionNameMap> = LazyLock::new(|| {
    REGION_NAME_DATA
        .lines()
        .filter_map(|line| {
            let parts = line.split_once(',')?;
            if SpecialRegionDesignator::is_valid(parts.0) {
                Some((parts.0.to_string(), parts.1.to_string()))
            } else {
                None
            }
        })
        .collect::<BTreeMap<_, _>>()
});

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SpecialRegionDesignator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for SpecialRegionDesignator {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<SpecialRegionDesignator> for String {
    fn from(value: SpecialRegionDesignator) -> Self {
        value.0
    }
}

impl FromStr for SpecialRegionDesignator {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "SpecialRegionDesignator",
            ))
        }
    }
}

impl SpecialRegionDesignator {
    pub fn is_valid(s: &str) -> bool {
        REGION_NAMES.contains_key(s)
    }

    pub fn name(&self) -> Option<&str> {
        REGION_NAMES.get(self.as_ref()).map(|s| s.as_str())
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_srd_name_map() {
        assert_eq!(
            REGION_NAMES.get(&"AFG".to_string()),
            Some(&"Afganistan".to_string())
        );
        assert_eq!(
            REGION_NAMES.get(&"ATA".to_string()),
            Some(&"Antartic".to_string())
        );
    }

    #[test]
    fn test_srd_name() {
        assert_eq!(
            SpecialRegionDesignator::from_str("AFG").unwrap().name(),
            Some("Afganistan")
        );
        assert_eq!(
            SpecialRegionDesignator::from_str("ATA").unwrap().name(),
            Some("Antartic")
        );
    }
}
