//!
//! One-line description.
//!
//! More detailed description.
//!
//! # Features
//!
//! - **std**; (default) Enables use of the standard library.
//! - **no-color**; Disables the coloring of Markdown output.
//!

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

pub mod dipole;
