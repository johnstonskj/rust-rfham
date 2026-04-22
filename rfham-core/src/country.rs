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

use crate::error::CoreError;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{env::VarError, fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct CountryCode(String);

pub const ENVVAR_COUNTRY_CODE: &str = "RFHAM_COUNTRY";

pub type CountryCodeNumeric = u16;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn country_code_us() -> CountryCode {
    CountryCode::new_unchecked("US")
}

pub fn country_code_uk() -> CountryCode {
    CountryCode::new_unchecked("UK")
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CountryCode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "CountryCode"))
        }
    }
}

impl From<CountryCode> for String {
    fn from(value: CountryCode) -> Self {
        value.0
    }
}

impl AsRef<str> for CountryCode {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl CountryCode {
    pub(crate) fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn from_env() -> Result<Option<Self>, CoreError> {
        Self::from_env_named(ENVVAR_COUNTRY_CODE)
    }

    pub fn from_env_named(envvar_name: &str) -> Result<Option<Self>, CoreError> {
        match std::env::var(envvar_name) {
            Ok(value) => Ok(Some(Self::from_str(&value)?)),
            Err(VarError::NotPresent) => Ok(None),
            Err(VarError::NotUnicode(value)) => Err(CoreError::InvalidValueFromStr(
                format!("{:#?}", value),
                "CountryCode",
            )),
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_numeric(&self) -> CountryCodeNumeric {
        country_code_coded(self.0.as_str())
    }

    pub fn is_valid(s: &str) -> bool {
        // TODO: validate correct combinations, this is good enough for now.
        s.len() == 2 && s.chars().all(|c| c.is_ascii_uppercase())
    }
}

// ------------------------------------------------------------------------------------------------

impl From<CountryCode> for CountryCodeNumeric {
    fn from(country_code: CountryCode) -> Self {
        country_code_coded(country_code.as_str())
    }
}

impl TryFrom<CountryCodeNumeric> for CountryCode {
    type Error = CoreError;

    fn try_from(value: CountryCodeNumeric) -> Result<Self, Self::Error> {
        let country_code = country_code_decoded(value)?;
        CountryCode::from_str(&country_code)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const CC_CODE_BASIS: u32 = 'A' as u32;

// Internal only as this does not do any validation whatsoever.
pub(crate) fn country_code_coded(s: &str) -> CountryCodeNumeric {
    let pair: Vec<u16> = s
        .chars()
        .map(|c| (c as u32 - CC_CODE_BASIS) as u16)
        .collect();
    (pair[0] << 8) + pair[1]
}

// Internal only as this does not do any validation whatsoever.
fn country_code_decoded(country_code: CountryCodeNumeric) -> Result<String, CoreError> {
    Ok(vec![
        char::from_u32((country_code >> 8) as u32 + CC_CODE_BASIS).ok_or(
            CoreError::InvalidValue(country_code.to_string(), "CountryCode"),
        )?,
        char::from_u32((country_code & 0b11111111) as u32 + CC_CODE_BASIS).ok_or(
            CoreError::InvalidValue(country_code.to_string(), "CountryCode"),
        )?,
    ]
    .into_iter()
    .collect::<String>())
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::{CountryCodeNumeric, country_code_coded, country_code_decoded};
    use pretty_assertions::assert_eq;

    const VALID_MAPPINGS: &[(&str, CountryCodeNumeric)] =
        &[("US", 5138_u16), ("GB", 1537), ("CN", 525)];

    #[test]
    fn country_code_to_number() {
        for (string, numeric) in VALID_MAPPINGS {
            assert_eq!(*numeric, country_code_coded(string));
        }
    }

    #[test]
    fn country_code_to_string() {
        for (string, numeric) in VALID_MAPPINGS {
            assert_eq!(string, &country_code_decoded(*numeric).unwrap().as_str());
        }
    }
}
