//! Transmit / receive power type.
//!
//! [`Power`] wraps a [`uom`](https://docs.rs/uom) SI watt quantity with ham-radio-centric
//! constructors and a smart `Display` that selects the most readable unit.
//!
//! # Display formats
//!
//! The default formatter (`{}`) uses watts. The alternate formatter (`{:#}`) selects milliwatts,
//! watts, or kilowatts based on the value:
//!
//! ```rust
//! use rfham_core::power::Power;
//!
//! assert_eq!(Power::watts(5.0).to_string(),        "5 W");
//! assert_eq!(format!("{:#}", Power::watts(5.0)),    "5 watts");
//! assert_eq!(format!("{:#}", Power::milliwatts(0.5)), "0.5 milliwatts"); // < 0.001 W
//! assert_eq!(format!("{:#}", Power::kilowatts(1.5)), "1.5 kilowatts");
//! ```
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::power::Power;
//!
//! // Derived from DC circuit: P = V × I
//! let p = Power::from_dc_circuit(13.8, 10.0); // 13.8 V × 10 A
//! assert!((p.value() - 138.0).abs() < 1e-9);
//!
//! // Derived from AC circuit: P = V × I × PF
//! let p = Power::from_ac_circuit(120.0, 5.0, 0.85);
//! assert!((p.value() - 510.0).abs() < 1e-9);
//! ```

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

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn watts(value: f64) -> Power {
    Power::watts(value)
}

pub fn milliwatts(value: f64) -> Power {
    Power::milliwatts(value)
}

pub fn kilowatts(value: f64) -> Power {
    Power::kilowatts(value)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Power;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display_watts() {
        assert_eq!("5 W", &Power::watts(5.0).to_string());
        assert_eq!("100 W", &Power::watts(100.0).to_string());
    }

    #[test]
    fn test_alternate_display() {
        // milliwatt branch: value < 0.001 W
        assert_eq!("0.5 milliwatts", &format!("{:#}", Power::milliwatts(0.5)));
        // watt branch: 0.001 W ≤ value < 1000 W
        assert_eq!("5 watts", &format!("{:#}", Power::watts(5.0)));
        // kilowatt branch: value ≥ 1000 W
        assert_eq!("1.5 kilowatts", &format!("{:#}", Power::kilowatts(1.5)));
    }

    #[test]
    fn test_from_dc_circuit() {
        let p = Power::from_dc_circuit(13.8, 10.0);
        assert!((p.value() - 138.0).abs() < 1e-9);
    }

    #[test]
    fn test_from_ac_circuit() {
        let p = Power::from_ac_circuit(120.0, 5.0, 0.85);
        assert!((p.value() - 510.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic]
    fn test_from_ac_circuit_factor_above_one_panics() {
        Power::from_ac_circuit(120.0, 5.0, 1.5);
    }

    #[test]
    fn test_from_f64_is_watts() {
        let p: Power = 100.0_f64.into();
        assert_eq!(p.value(), Power::watts(100.0).value());
    }
}
