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
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigPath(Vec<PathElement>);

#[derive(Clone, Debug, PartialEq, Eq, EnumIs, EnumTryAs)]
pub enum PathElement {
    Name(Name),
    Index(usize),
}

pub trait PathTarget: Debug {
    fn path_name() -> Option<Name>;

    fn value_names() -> impl Iterator<Item = &'static str>;

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError>;
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

impl From<Name> for PathElement {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}

impl From<usize> for PathElement {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

impl Display for PathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Index(v) => v.to_string(),
                Self::Name(v) => v.to_string(),
            }
        )
    }
}

impl FromStr for PathElement {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self::Index(usize::from_str(s)?))
        } else {
            Ok(Self::Name(Name::from_str(s)?))
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ConfigPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

impl FromStr for ConfigPath {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Result<Vec<PathElement>, ConfigError> =
            s.split('.').map(PathElement::from_str).collect();
        let values = results?;
        if !values.is_empty() {
            Ok(Self(values))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "ConfigPath").into())
        }
    }
}

impl From<Name> for ConfigPath {
    fn from(value: Name) -> Self {
        Self::from(PathElement::from(value))
    }
}

impl From<usize> for ConfigPath {
    fn from(value: usize) -> Self {
        Self::from(PathElement::from(value))
    }
}

impl From<PathElement> for ConfigPath {
    fn from(value: PathElement) -> Self {
        Self::from(vec![value])
    }
}

impl From<Vec<PathElement>> for ConfigPath {
    fn from(values: Vec<PathElement>) -> Self {
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

    pub fn iter(&self) -> impl Iterator<Item = &PathElement> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PathElement> {
        self.0.iter_mut()
    }

    pub fn push(&mut self, name: PathElement) {
        self.0.push(name);
    }

    pub fn pop(&mut self) -> Option<PathElement> {
        if !self.is_single() {
            self.0.pop()
        } else {
            None
        }
    }

    pub fn head(&self) -> &PathElement {
        &self.0[0]
    }

    pub fn tail(&self) -> Option<ConfigPath> {
        if !self.is_single() {
            Some(ConfigPath(self.0[1..].to_vec()))
        } else {
            None
        }
    }

    pub fn last(&self) -> &PathElement {
        self.0.last().unwrap()
    }

    pub fn split(&self) -> (&PathElement, Option<ConfigPath>) {
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
