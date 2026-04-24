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
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use uom::{
    fmt::DisplayStyle,
    si::{f64::Power as BasePower, power as power_unit},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Power(BasePower);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            if self.value() < 0.001 {
                self.0
                    .into_format_args(power_unit::milliwatt, DisplayStyle::Description)
                    .fmt(f)
            } else if self.value() >= 1000.0 {
                self.0
                    .into_format_args(power_unit::kilowatt, DisplayStyle::Description)
                    .fmt(f)
            } else {
                self.0
                    .into_format_args(power_unit::watt, DisplayStyle::Description)
                    .fmt(f)
            }
        } else {
            self.0
                .into_format_args(power_unit::watt, DisplayStyle::Abbreviation)
                .fmt(f)
        }
    }
}

impl FromStr for Power {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BasePower::from_str(s)
            .map(Self)
            .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "Power"))
    }
}

impl From<BasePower> for Power {
    fn from(value: BasePower) -> Self {
        Self(value)
    }
}

impl From<f64> for Power {
    fn from(value: f64) -> Self {
        Self::watts(value)
    }
}

impl From<Power> for BasePower {
    fn from(value: Power) -> Self {
        value.0
    }
}

impl From<Power> for f64 {
    fn from(value: Power) -> Self {
        value.0.value
    }
}

impl AsRef<BasePower> for Power {
    fn as_ref(&self) -> &BasePower {
        &self.0
    }
}

impl Power {
    #[inline(always)]
    pub fn milliwatts(value: f64) -> Self {
        Self(BasePower::new::<power_unit::milliwatt>(value))
    }

    #[inline(always)]
    pub fn watts(value: f64) -> Self {
        Self(BasePower::new::<power_unit::watt>(value))
    }

    #[inline(always)]
    pub fn kilowatts(value: f64) -> Self {
        Self(BasePower::new::<power_unit::kilowatt>(value))
    }

    #[inline(always)]
    pub fn from_dc_circuit(voltage: f64, current: f64) -> Self {
        Self::watts(voltage * current)
    }

    #[inline(always)]
    pub fn from_ac_circuit(voltage: f64, current: f64, factor: f64) -> Self {
        assert!((0.0..=1.0).contains(&factor));
        Self::watts(voltage * current * factor)
    }

    pub const fn value(&self) -> f64 {
        self.0.value
    }
}
