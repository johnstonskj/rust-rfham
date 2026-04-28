//! Dot-separated configuration key paths for `rfham-config`.
//!
//! [`ConfigPath`] is a validated sequence of [`Name`](rfham_core::id::Name) segments
//! joined by `.`. It parses from strings like `"station.callsign"` and displays back
//! in the same form.
//!
//! # Examples
//!
//! ```rust
//! use rfham_config::paths::ConfigPath;
//!
//! let path: ConfigPath = "station.callsign".parse().unwrap();
//! assert_eq!("station.callsign", path.to_string());
//! assert!("".parse::<ConfigPath>().is_err());
//! ```

use core::{fmt::Display, str::FromStr};
use rfham_core::{Name, error::CoreError};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigPath(Vec<Name>);

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
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ConfigPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(Name::to_string)
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

impl FromStr for ConfigPath {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Result<Vec<Name>, CoreError> = s.split('.').map(Name::from_str).collect();
        let values = results?;
        if !values.is_empty() {
            Ok(Self(values))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "ConfigPath"))
        }
    }
}

impl From<Name> for ConfigPath {
    fn from(value: Name) -> Self {
        Self::from(vec![value])
    }
}

impl From<Vec<Name>> for ConfigPath {
    fn from(values: Vec<Name>) -> Self {
        assert!(
            !values.is_empty(),
            "ConfigPath must have at least one component"
        );
        Self(values)
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

#[cfg(test)]
mod tests {
    use super::ConfigPath;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_component() {
        let path: ConfigPath = "station".parse().unwrap();
        assert_eq!("station", path.to_string());
    }

    #[test]
    fn test_multi_component() {
        let path: ConfigPath = "station.callsign".parse().unwrap();
        assert_eq!("station.callsign", path.to_string());
    }

    #[test]
    fn test_empty_string_is_error() {
        assert!("".parse::<ConfigPath>().is_err());
    }

    #[test]
    fn test_invalid_component_is_error() {
        // Names cannot contain spaces
        assert!("station.has space".parse::<ConfigPath>().is_err());
    }
}
