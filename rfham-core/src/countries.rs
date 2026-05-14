//! ISO 3166-1 alpha-2 country codes.
//!
//! [`CountryCode`] is a two-uppercase-letter code validated at construction time.
//! It can be read from an environment variable ([`ENVVAR_COUNTRY_CODE`]) for
//! locale-aware behaviour in library consumers.
//!
//! Internally the two letters are packed into a [`u16`] via [`CountryCode::to_numeric`]
//! and unpacked with [`CountryCode::try_from`], which allows compact storage when needed.
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::countries::CountryCode;
//! use std::str::FromStr;
//!
//! let us = CountryCode::from_str("US").unwrap();
//! assert_eq!(us.to_string(), "US");
//!
//! // Numeric round-trip
//! let n: u16 = us.to_numeric();
//! let back = CountryCode::try_from(n).unwrap();
//! assert_eq!(us, back);
//! ```
//!
//! Invalid codes are rejected:
//!
//! ```rust
//! use rfham_core::countries::CountryCode;
//! use std::str::FromStr;
//!
//! assert!("us".parse::<CountryCode>().is_err());   // must be uppercase
//! assert!("USA".parse::<CountryCode>().is_err());  // must be exactly 2 chars
//! assert!("1X".parse::<CountryCode>().is_err());   // must be letters
//! ```
//!
//! Unknown countries are also rejected, even if they are well-formed:
//!
//! ```rust
//! use rfham_core::countries::CountryCode;
//! use std::str::FromStr;
//!
//! assert!("AA".parse::<CountryCode>().is_err());
//! assert!("JG".parse::<CountryCode>().is_err());
//! ```
//!

use crate::{StringLike, error::CoreError};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::HashSet, env::VarError, fmt::Display, str::FromStr, sync::LazyLock};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A struct representing an ISO 3166-1 alpha-2 country code.
///
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, DeserializeFromStr, SerializeDisplay,
)]
pub struct CountryCode(String);

pub const ENVVAR_COUNTRY_CODE: &str = "RFHAM_COUNTRY";

///
/// A Type providing a unique numeric representation of a country codes.
/// This is not intended for general use, but can be useful for compact storage of country codes
///
pub type CountryCodeNumeric = u16;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return a `CountryCode` instance for the United States (US).
///
pub fn country_code_us() -> CountryCode {
    CountryCode::new_unchecked("US")
}

///
/// Return a `CountryCode` instance for the United Kingdom (UK).
///
pub fn country_code_uk() -> CountryCode {
    CountryCode::new_unchecked("UK")
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

static COUNTRY_CODE_DATA: &str = include_str!("../data/country-codes-ordered.csv");
static VALID_COUNTRY_CODES: LazyLock<HashSet<CountryCodeNumeric>> = LazyLock::new(|| {
    let mut countries = COUNTRY_CODE_DATA
        .lines()
        .filter_map(|line| {
            let code = line.split(',').next()?;
            Some(country_code_coded(code.trim()))
        })
        .collect::<HashSet<_, _>>();

    countries.extend(vec![
        country_code_coded("XA"), // International Civil Aviation Organization
        country_code_coded("XM"), // World Meteorological Organization
        country_code_coded("XU"), // United Nations
    ]);

    countries
});

impl Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CountryCode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_known_country(s) {
            Ok(Self(s.to_string()))
        } else if Self::is_valid_format(s) {
            Err(CoreError::InvalidValue(s.to_string(), "CountryCode"))
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

impl StringLike for CountryCode {
    const MAX_LENGTH: usize = 2;

    fn new_unchecked<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn is_valid(s: &str) -> bool {
        Self::is_known_country(s)
    }
}

impl CountryCode {
    ///
    /// Attempt to read a `CountryCode` from the environment variable specified by
    /// [`ENVVAR_COUNTRY_CODE`].
    ///
    pub fn from_env() -> Result<Option<Self>, CoreError> {
        Self::from_env_named(ENVVAR_COUNTRY_CODE)
    }

    ///
    /// Attempt to read a `CountryCode` from the environment variable specified by
    /// `envvar_name`.
    ///
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

    ///
    /// Convert this string country code to its compact numeric representation.
    ///
    pub fn to_numeric(&self) -> CountryCodeNumeric {
        country_code_coded(self.0.as_str())
    }

    ///
    /// Returns `true` if the string `s` is in a valid format, but does not imply that
    /// it is a valid, known, ISO country code.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::countries::CountryCode;
    ///
    /// println!("Is 'US' valid format? {}", CountryCode::is_valid_format("US"));
    /// println!("Is 'AA' valid format? {}", CountryCode::is_valid_format("AA"));
    /// println!("Is 'USA' valid format? {}", CountryCode::is_valid_format("USA"));
    /// ```
    pub fn is_valid_format(s: &str) -> bool {
        s.len() == 2 && s.chars().all(|c| c.is_ascii_uppercase())
    }

    ///
    /// Returns `true` if the string `s` is a known, valid ISO country code. This implies
    /// that it is in a valid format.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::countries::CountryCode;
    ///
    /// println!("Is 'US' a known country? {}", CountryCode::is_known_country("US"));
    /// println!("Is 'AA' a known country? {}", CountryCode::is_known_country("AA"));
    /// ```
    ///
    pub fn is_known_country(s: &str) -> bool {
        Self::is_valid_format(s) && VALID_COUNTRY_CODES.contains(&country_code_coded(s))
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
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::{CountryCode, CountryCodeNumeric, country_code_coded, country_code_decoded};
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

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

    #[test]
    fn country_code_from_str_valid() {
        assert!(CountryCode::from_str("US").is_ok());
        assert!(CountryCode::from_str("JP").is_ok());
        assert!(CountryCode::from_str("DE").is_ok());
    }

    #[test]
    fn country_code_from_str_invalid() {
        assert!("us".parse::<CountryCode>().is_err()); // lowercase
        assert!("USA".parse::<CountryCode>().is_err()); // 3 chars
        assert!("1X".parse::<CountryCode>().is_err()); // leading digit
        assert!("".parse::<CountryCode>().is_err()); // empty
    }

    #[test]
    fn country_code_numeric_roundtrip() {
        for code in ["US", "GB", "JP", "CN", "DE"] {
            let cc = CountryCode::from_str(code).unwrap();
            let n = cc.to_numeric();
            assert_eq!(
                cc,
                CountryCode::try_from(n).unwrap(),
                "roundtrip failed for {code}"
            );
        }
    }

    #[test]
    fn country_code_from_env_absent() {
        // Use a name that is guaranteed to be unset rather than mutating the environment.
        assert_eq!(
            CountryCode::from_env_named("RFHAM_COUNTRY_ABSENT_TEST_ONLY").unwrap(),
            None
        );
    }
}
