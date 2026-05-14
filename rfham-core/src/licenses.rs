//!
//! Provides a descriptive `LicenseClass` structure to describe classes of amateur license.
//!
//! License classes are tiered and
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::{StringLike, error::CoreError};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LicenseKey(String);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LicenseClass {
    key: LicenseKey,
    ordinal: u32,
    name: String,
    is_active: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for LicenseKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<LicenseKey> for String {
    fn from(value: LicenseKey) -> Self {
        value.0
    }
}

impl AsRef<str> for LicenseKey {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for LicenseKey {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "LicenseKey"))
        }
    }
}

impl TryFrom<char> for LicenseKey {
    type Error = CoreError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c.is_ascii_alphanumeric() {
            Ok(Self(c.to_string()))
        } else {
            Err(CoreError::InvalidValueFromStr(c.to_string(), "LicenseKey"))
        }
    }
}

impl StringLike for LicenseKey {
    const MAX_LENGTH: usize = 3;

    fn new_unchecked<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty() && s.len() < Self::MAX_LENGTH && s.chars().all(|c| c.is_ascii_alphanumeric())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LicenseClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                format!(
                    "{}:{:02}: {} ({})",
                    self.key,
                    self.ordinal,
                    self.name,
                    if self.is_active { "active" } else { "inactive" }
                )
            } else {
                format!(
                    "{}: {}{}",
                    self.key,
                    self.name,
                    if self.is_active { "" } else { " (inactive)" }
                )
            }
        )
    }
}

impl PartialOrd for LicenseClass {
    fn partial_cmp(&self, other: &LicenseClass) -> Option<Ordering> {
        self.ordinal.partial_cmp(&other.ordinal)
    }
}

impl LicenseClass {
    pub fn new<S: Into<String>>(key: LicenseKey, ordinal: u32, name: S, is_active: bool) -> Self {
        Self {
            key,
            name: name.into(),
            ordinal,
            is_active,
        }
    }

    pub fn active<S: Into<String>>(key: LicenseKey, ordinal: u32, name: S) -> Self {
        Self::new(key, ordinal, name, true)
    }

    pub fn inactive<S: Into<String>>(key: LicenseKey, ordinal: u32, name: S) -> Self {
        Self::new(key, ordinal, name, false)
    }

    pub fn key(&self) -> &LicenseKey {
        &self.key
    }

    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
