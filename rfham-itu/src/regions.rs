//!
//! Provides the [`Region`] type used to denote one of the three ITU regions worldwide.
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

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
