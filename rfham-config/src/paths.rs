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

use crate::error::ConfigError;
use rfham_core::{Name, error::CoreError};
use std::{
    fmt::{Debug, Display},
    path::PathBuf,
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigPath(Vec<Name>);

pub trait PathTarget: Debug {
    fn path_name(&self) -> Option<Name>;

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError>;

    fn value_names(&self) -> impl Iterator<Item = &'static str>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Path(PathBuf),
    EnumValue(String),
    None,
}

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

impl ConfigPath {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_single(&self) -> bool {
        self.0.len() == 1
    }

    pub fn iter(&self) -> impl Iterator<Item = &Name> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Name> {
        self.0.iter_mut()
    }

    pub fn push(&mut self, name: Name) {
        self.0.push(name);
    }

    pub fn pop(&mut self) -> Option<Name> {
        if !self.is_single() {
            self.0.pop()
        } else {
            None
        }
    }

    pub fn head(&self) -> &Name {
        &self.0[0]
    }

    pub fn tail(&self) -> Option<ConfigPath> {
        if !self.is_single() {
            Some(ConfigPath(self.0[1..].to_vec()))
        } else {
            None
        }
    }

    pub fn field_name(&self) -> &Name {
        self.0.last().unwrap()
    }

    pub fn split(&self) -> (&Name, Option<ConfigPath>) {
        (self.head(), self.tail())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean(v) => v.to_string(),
                Self::EnumValue(v) => v.to_string(),
                Self::Float(v) => v.to_string(),
                Self::Integer(v) => v.to_string(),
                Self::None => "".to_string(),
                Self::Path(v) => v.display().to_string(),
                Self::String(v) => format!("{:?}", v),
            }
        )
    }
}

impl Value {
    pub fn type_label(&self) -> &'static str {
        match self {
            Self::Boolean(_) => "bool",
            Self::EnumValue(_) => "enum value",
            Self::Float(_) => "f64",
            Self::Integer(_) => "i64",
            Self::None => "not set",
            Self::Path(_) => "path",
            Self::String(_) => "string",
        }
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
