//! ITU frequency band designations (ELF through THF).
//!
//! [`FrequencyBand`] encodes the twelve ITU bands numbered 1–12. Each band has an
//! abbreviation (`"VHF"`), a full name (`"Very High Frequency"`), an ITU band number,
//! and a [`FrequencyRange`](rfham_core::frequency::FrequencyRange).
//!
//! # Examples
//!
//! ```rust
//! use rfham_itu::bands::FrequencyBand;
//! use std::str::FromStr;
//!
//! let band = FrequencyBand::VeryHigh;
//! assert_eq!("VHF", band.abbreviation());
//! assert_eq!("Very High Frequency", band.name());
//! assert_eq!(8, band.number());
//!
//! let parsed: FrequencyBand = "UHF".parse().unwrap();
//! assert_eq!(FrequencyBand::UltraHigh, parsed);
//! assert!("XYZ".parse::<FrequencyBand>().is_err());
//! ```

use core::{fmt::Display, str::FromStr};
use rfham_core::{
    error::CoreError,
    frequencies::{FrequencyRange, gigahertz, hertz, kilohertz, megahertz},
};
use serde_with::{DeserializeFromStr, SerializeDisplay};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay)]
#[repr(u32)]
pub enum FrequencyBand {
    ExtremelyLow = 1,
    SuperLow = 2,
    UltraLow = 3,
    VeryLow = 4,
    Low = 5,
    Medium = 6,
    High = 7,
    VeryHigh = 8,
    UltraHigh = 9,
    SuperHigh = 10,
    ExtremelyHigh = 11,
    TremendouslyHigh = 12,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for FrequencyBand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                format!("{} ({})", self.name(), self.abbreviation())
            } else {
                self.abbreviation().to_string()
            }
        )
    }
}

impl FromStr for FrequencyBand {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ELF" => Ok(Self::ExtremelyLow),
            "SLF" => Ok(Self::SuperLow),
            "ULF" => Ok(Self::UltraLow),
            "VLF" => Ok(Self::VeryLow),
            "LF" => Ok(Self::Low),
            "MF" => Ok(Self::Medium),
            "HF" => Ok(Self::High),
            "VHF" => Ok(Self::VeryHigh),
            "UHF" => Ok(Self::UltraHigh),
            "SHF" => Ok(Self::SuperHigh),
            "EHF" => Ok(Self::ExtremelyHigh),
            "THF" => Ok(Self::TremendouslyHigh),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "FrequencyBand",
            )),
        }
    }
}

impl FrequencyBand {
    pub const fn name(&self) -> &str {
        match self {
            Self::ExtremelyLow => "Extremely Low Frequency",
            Self::SuperLow => "Super Low Frequency",
            Self::UltraLow => "Ultra Low Frequency",
            Self::VeryLow => "Very Low Frequency",
            Self::Low => "Low Frequency",
            Self::Medium => "Medium Frequency",
            Self::High => "High Frequency",
            Self::VeryHigh => "Very High Frequency",
            Self::UltraHigh => "Ultra High Frequency",
            Self::SuperHigh => "Super High Frequency",
            Self::ExtremelyHigh => "Extremely High Frequency",
            Self::TremendouslyHigh => "Tremendously High Frequency",
        }
    }
    pub const fn abbreviation(&self) -> &str {
        match self {
            Self::ExtremelyLow => "ELF",
            Self::SuperLow => "SLF",
            Self::UltraLow => "ULF",
            Self::VeryLow => "VLF",
            Self::Low => "LF",
            Self::Medium => "MF",
            Self::High => "HF",
            Self::VeryHigh => "VHF",
            Self::UltraHigh => "UHF",
            Self::SuperHigh => "SHF",
            Self::ExtremelyHigh => "EHF",
            Self::TremendouslyHigh => "THF",
        }
    }
    pub const fn number(&self) -> u32 {
        *self as u32
    }

    pub fn range(&self) -> FrequencyRange {
        match self {
            Self::ExtremelyLow => FrequencyRange::new(hertz(3.0), hertz(30.0)),
            Self::SuperLow => FrequencyRange::new(hertz(30.0), hertz(300.0)),
            Self::UltraLow => FrequencyRange::new(hertz(300.0), hertz(3000.0)),
            Self::VeryLow => FrequencyRange::new(kilohertz(3.0), kilohertz(30.0)),
            Self::Low => FrequencyRange::new(kilohertz(30.0), kilohertz(300.0)),
            Self::Medium => FrequencyRange::new(kilohertz(300.0), kilohertz(3000.0)),
            Self::High => FrequencyRange::new(megahertz(3.0), megahertz(30.0)),
            Self::VeryHigh => FrequencyRange::new(megahertz(30.0), megahertz(300.0)),
            Self::UltraHigh => FrequencyRange::new(megahertz(300.0), megahertz(3000.0)),
            Self::SuperHigh => FrequencyRange::new(gigahertz(3.0), gigahertz(30.0)),
            Self::ExtremelyHigh => FrequencyRange::new(gigahertz(30.0), gigahertz(300.0)),
            Self::TremendouslyHigh => FrequencyRange::new(gigahertz(300.0), gigahertz(3000.0)),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::FrequencyBand;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_display_abbreviation() {
        assert_eq!("HF", FrequencyBand::High.to_string());
        assert_eq!("VHF", FrequencyBand::VeryHigh.to_string());
    }

    #[test]
    fn test_display_alternate() {
        assert_eq!(
            "Very High Frequency (VHF)",
            format!("{:#}", FrequencyBand::VeryHigh)
        );
    }

    #[test]
    fn test_from_str_valid() {
        assert_eq!(
            FrequencyBand::VeryHigh,
            FrequencyBand::from_str("VHF").unwrap()
        );
        assert_eq!(
            FrequencyBand::UltraHigh,
            FrequencyBand::from_str("UHF").unwrap()
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("XYZ".parse::<FrequencyBand>().is_err());
    }

    #[test]
    fn test_number() {
        assert_eq!(7, FrequencyBand::High.number());
        assert_eq!(8, FrequencyBand::VeryHigh.number());
        assert_eq!(9, FrequencyBand::UltraHigh.number());
    }

    #[test]
    fn test_range_contains() {
        let vhf = FrequencyBand::VeryHigh.range();
        assert!(vhf.contains_mhz(146.0));
        assert!(!vhf.contains_mhz(500.0));
    }
}
