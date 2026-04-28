//! The three ITU world regions used in frequency allocation tables.
//!
//! [`Region`] is a `#[repr(u32)]` enum with values 1, 2, and 3 corresponding to the
//! geographic zones defined by the ITU Radio Regulations. The default `Display` format
//! is the numeral `"1"`, `"2"`, or `"3"`; the alternate (`{:#}`) format is the full
//! name `"IARU/ITU Region N"`.
//!
//! # Examples
//!
//! ```rust
//! use rfham_itu::regions::Region;
//! use std::str::FromStr;
//!
//! let r: Region = "2".parse().unwrap();
//! assert_eq!(Region::Two, r);
//! assert_eq!("2", r.to_string());
//! assert_eq!("IARU/ITU Region 2", format!("{r:#}"));
//! assert!("4".parse::<Region>().is_err());
//! ```

use core::{fmt::Display, str::FromStr};
use rfham_core::error::CoreError;
use serde_with::{DeserializeFromStr, SerializeDisplay};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay)]
#[repr(u32)]
pub enum Region {
    /// Comprises Europe, Africa, the Commonwealth of Independent States (former Soviet Union),
    /// Mongolia, and the Middle East west of the Persian Gulf, including Iraq.
    One = 1,
    /// Covers the Americas, including Greenland and some eastern Pacific Islands.
    Two = 2,
    /// Contains most of non-FSU Asia (east of and including Iran), Australia, New Zealand, and
    /// most of the Pacific.
    Three = 3,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Region::One, true) => "IARU/ITU Region 1",
                (Region::One, false) => "1",
                (Region::Two, true) => "IARU/ITU Region 2",
                (Region::Two, false) => "2",
                (Region::Three, true) => "IARU/ITU Region 3",
                (Region::Three, false) => "3",
            }
        )
    }
}

impl FromStr for Region {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "one" => Ok(Self::One),
            "2" | "two" => Ok(Self::Two),
            "3" | "three" => Ok(Self::Three),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "Region")),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Region;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_display_numeral() {
        assert_eq!("1", Region::One.to_string());
        assert_eq!("3", Region::Three.to_string());
    }

    #[test]
    fn test_display_alternate() {
        assert_eq!("IARU/ITU Region 2", format!("{:#}", Region::Two));
    }

    #[test]
    fn test_from_str_numeric() {
        assert_eq!(Region::One, "1".parse().unwrap());
        assert_eq!(Region::Two, "2".parse().unwrap());
        assert_eq!(Region::Three, "3".parse().unwrap());
    }

    #[test]
    fn test_from_str_word() {
        assert_eq!(Region::One, Region::from_str("one").unwrap());
        assert_eq!(Region::Two, Region::from_str("two").unwrap());
        assert_eq!(Region::Three, Region::from_str("three").unwrap());
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("4".parse::<Region>().is_err());
        assert!("zero".parse::<Region>().is_err());
    }
}
