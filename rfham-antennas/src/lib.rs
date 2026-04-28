//! Antenna types and calculations for RF-Ham.
//!
//! This crate provides the [`AntennaForm`] enum classifying common amateur antenna
//! styles, and the [`dipole`] module with [`dipole::SimpleDipole`] — a classical
//! half-wave dipole calculator that can produce Markdown documentation of its design.
//!
//! # Examples
//!
//! ```rust
//! use rfham_antennas::SimpleDipole;
//! use rfham_itu::allocations::FrequencyAllocation;
//!
//! let dipole = SimpleDipole::new(FrequencyAllocation::Band2M);
//! // Half-wave length for 2m mid-band (≈146 MHz) is ~1.027 m
//! let length = dipole.antenna_length().unwrap();
//! assert!(length.value() > 1.0 && length.value() < 1.1);
//! ```

use rfham_core::error::CoreError;
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AntennaForm {
    Dipole,
    Dish,
    Vertical,
    EndFed,
    RandomWire,
    Yagi,
    Other,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for AntennaForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dipole => "dipole",
                Self::Dish => "dish",
                Self::Vertical => "vertical",
                Self::EndFed => "efhw",
                Self::RandomWire => "random",
                Self::Yagi => "yagi",
                Self::Other => "other",
            }
        )
    }
}

impl FromStr for AntennaForm {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dipole" => Ok(Self::Dipole),
            "dish" => Ok(Self::Dish),
            "vertical" => Ok(Self::Vertical),
            "efhw" => Ok(Self::EndFed),
            "random" => Ok(Self::RandomWire),
            "yagi" => Ok(Self::Yagi),
            "other" => Ok(Self::Other),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "AntennaForm")),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod dipoles;
pub use dipoles::SimpleDipole;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::AntennaForm;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_display_roundtrip() {
        for (s, form) in [
            ("dipole", AntennaForm::Dipole),
            ("vertical", AntennaForm::Vertical),
            ("efhw", AntennaForm::EndFed),
            ("yagi", AntennaForm::Yagi),
        ] {
            assert_eq!(s, form.to_string());
            assert_eq!(form, AntennaForm::from_str(s).unwrap());
        }
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("loop".parse::<AntennaForm>().is_err());
    }
}
