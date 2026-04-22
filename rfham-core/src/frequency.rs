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
use std::{fmt::Display, ops::Range, str::FromStr};
use uom::{
    fmt::DisplayStyle,
    si::{
        f64::{Frequency as BaseFrequency, Length as BaseLength},
        frequency as frequency_unit, length as length_unit,
    },
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Frequency(BaseFrequency);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Wavelength(BaseLength);

pub const SPEED_OF_LIGHT: f64 = 299792458.0; // m/s

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FrequencyRange(Range<Frequency>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn gigahertz(value: f64) -> Frequency {
    Frequency(BaseFrequency::new::<frequency_unit::gigahertz>(value))
}
pub fn megahertz(value: f64) -> Frequency {
    Frequency(BaseFrequency::new::<frequency_unit::megahertz>(value))
}
pub fn kilohertz(value: f64) -> Frequency {
    Frequency(BaseFrequency::new::<frequency_unit::kilohertz>(value))
}
pub fn hertz(value: f64) -> Frequency {
    Frequency(BaseFrequency::new::<frequency_unit::hertz>(value))
}

pub fn millimeters(value: f64) -> Wavelength {
    Wavelength(BaseLength::new::<length_unit::millimeter>(value))
}

pub fn centimeters(value: f64) -> Wavelength {
    Wavelength(BaseLength::new::<length_unit::centimeter>(value))
}

pub fn meters(value: f64) -> Wavelength {
    Wavelength(BaseLength::new::<length_unit::meter>(value))
}

pub fn kilometers(value: f64) -> Wavelength {
    Wavelength(BaseLength::new::<length_unit::kilometer>(value))
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Frequency
// ------------------------------------------------------------------------------------------------

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.value() {
                0.0..1_000.0 => self
                    .0
                    .into_format_args(frequency_unit::hertz, DisplayStyle::Description)
                    .fmt(f),
                1_000.0..1_000_000.0 => self
                    .0
                    .into_format_args(frequency_unit::kilohertz, DisplayStyle::Description)
                    .fmt(f),
                1_000_000.0..1_000_000_000.0 => self
                    .0
                    .into_format_args(frequency_unit::megahertz, DisplayStyle::Description)
                    .fmt(f),
                1_000_000_000.0..1_000_000_000_000.0 => self
                    .0
                    .into_format_args(frequency_unit::gigahertz, DisplayStyle::Description)
                    .fmt(f),
                _ => self
                    .0
                    .into_format_args(frequency_unit::terahertz, DisplayStyle::Description)
                    .fmt(f),
            }
        } else {
            self.0
                .into_format_args(frequency_unit::megahertz, DisplayStyle::Abbreviation)
                .fmt(f)
        }
    }
}

impl FromStr for Frequency {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BaseFrequency::from_str(s)
            .map(Self)
            .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "Frequency"))
    }
}

impl From<BaseFrequency> for Frequency {
    fn from(value: BaseFrequency) -> Self {
        Self(value)
    }
}

impl From<f64> for Frequency {
    fn from(value: f64) -> Self {
        megahertz(value)
    }
}

impl From<Frequency> for BaseFrequency {
    fn from(value: Frequency) -> Self {
        value.0
    }
}

impl From<Frequency> for f64 {
    fn from(value: Frequency) -> Self {
        value.0.value
    }
}

impl AsRef<BaseFrequency> for Frequency {
    fn as_ref(&self) -> &BaseFrequency {
        &self.0
    }
}

impl Frequency {
    pub fn value(&self) -> f64 {
        self.0.value
    }

    pub fn to_wavelength(&self) -> Wavelength {
        meters(SPEED_OF_LIGHT / self.value())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ FrequencyRange
// ------------------------------------------------------------------------------------------------

impl Display for FrequencyRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.start.fmt(f)?;
        write!(f, " {} ", if f.alternate() { "to" } else { "-" },)?;
        self.0.end.fmt(f)
    }
}

impl From<Range<Frequency>> for FrequencyRange {
    fn from(range: Range<Frequency>) -> Self {
        Self(range)
    }
}

impl From<Range<f64>> for FrequencyRange {
    fn from(range: Range<f64>) -> Self {
        Self::new(megahertz(range.start), megahertz(range.end))
    }
}

impl From<(Frequency, Frequency)> for FrequencyRange {
    fn from(range: (Frequency, Frequency)) -> Self {
        Self::new(range.0, range.1)
    }
}

impl From<(f64, f64)> for FrequencyRange {
    fn from(range: (f64, f64)) -> Self {
        Self::new(megahertz(range.0), megahertz(range.1))
    }
}

impl From<FrequencyRange> for Range<Frequency> {
    fn from(range: FrequencyRange) -> Self {
        range.0
    }
}

impl From<FrequencyRange> for Range<f64> {
    fn from(range: FrequencyRange) -> Self {
        Range {
            start: range.0.start.value(),
            end: range.0.end.value(),
        }
    }
}

impl FrequencyRange {
    pub fn new(start: Frequency, end: Frequency) -> Self {
        assert!(start <= end);
        Self(Range { start, end })
    }
    pub fn new_mhz(start: f64, end: f64) -> Self {
        Self::new(megahertz(start), megahertz(end))
    }

    pub const fn start(&self) -> Frequency {
        self.0.start
    }

    pub const fn end(&self) -> Frequency {
        self.0.end
    }

    pub fn mid_band(&self) -> Frequency {
        hertz(self.0.start.value() + ((self.0.end.value() - self.0.start.value()) / 2.0))
    }

    pub fn contains(&self, frequency: Frequency) -> bool {
        self.0.contains(&frequency)
    }

    pub fn contains_mhz(&self, frequency: f64) -> bool {
        self.0.contains(&megahertz(frequency))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn starts_before(&self, other: &Self) -> bool {
        self.start() < other.start()
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        self.start() == other.start() && self.contains(other.end())
    }

    pub fn ends_before(&self, other: &Self) -> bool {
        self.end() < other.end()
    }

    pub fn ends_with(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.end() == other.end()
    }

    pub fn is_subrange(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    pub fn is_overlapping(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Wavelength
// ------------------------------------------------------------------------------------------------

impl Display for Wavelength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.value() {
                0.001..0.01 => self
                    .0
                    .into_format_args(length_unit::millimeter, DisplayStyle::Description)
                    .fmt(f),
                0.01..1.0 => self
                    .0
                    .into_format_args(length_unit::centimeter, DisplayStyle::Description)
                    .fmt(f),
                1_000.0.. => self
                    .0
                    .into_format_args(length_unit::kilometer, DisplayStyle::Description)
                    .fmt(f),
                _ => self
                    .0
                    .into_format_args(length_unit::meter, DisplayStyle::Description)
                    .fmt(f),
            }
        } else {
            self.0
                .into_format_args(length_unit::meter, DisplayStyle::Abbreviation)
                .fmt(f)
        }
    }
}

impl FromStr for Wavelength {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BaseLength::from_str(s)
            .map(Self)
            .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "Wavelength"))
    }
}

impl From<BaseLength> for Wavelength {
    fn from(value: BaseLength) -> Self {
        Self(value)
    }
}

impl From<f64> for Wavelength {
    fn from(value: f64) -> Self {
        meters(value)
    }
}

impl From<Wavelength> for BaseLength {
    fn from(value: Wavelength) -> Self {
        value.0
    }
}

impl From<Wavelength> for f64 {
    fn from(value: Wavelength) -> Self {
        value.0.value
    }
}

impl AsRef<BaseLength> for Wavelength {
    fn as_ref(&self) -> &BaseLength {
        &self.0
    }
}

impl Wavelength {
    pub fn value(&self) -> f64 {
        self.0.value
    }

    pub fn to_frequency(&self) -> Frequency {
        megahertz(SPEED_OF_LIGHT / self.value())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::FrequencyRange;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_fmt_frequency() {
        assert_eq!("0.000001 MHz", &super::hertz(1.0).to_string());
        assert_eq!("0.001 MHz", &super::kilohertz(1.0).to_string());
        assert_eq!("1 MHz", &super::megahertz(1.0).to_string());
        assert_eq!("1000 MHz", &super::gigahertz(1.0).to_string());
    }

    #[test]
    fn test_default_fmt_frequency_precision() {
        assert_eq!("0.000001 MHz", &format!("{:.6}", super::hertz(1.0)));
        assert_eq!("0.001000 MHz", &format!("{:.6}", super::kilohertz(1.0)));
        assert_eq!("1.000000 MHz", &format!("{:.6}", super::megahertz(1.0)));
        assert_eq!("1000.000000 MHz", &format!("{:.6}", super::gigahertz(1.0)));
    }

    #[test]
    fn test_default_fmt_frequency_width_and_precision() {
        assert_eq!(
            "      0.000001 MHz",
            &format!("{:>14.6}", super::hertz(1.0))
        );
        assert_eq!(
            "      0.001000 MHz",
            &format!("{:>14.6}", super::kilohertz(1.0))
        );
        assert_eq!(
            "      1.000000 MHz",
            &format!("{:>14.6}", super::megahertz(1.0))
        );
        assert_eq!(
            "   1000.000000 MHz",
            &format!("{:>14.6}", super::gigahertz(1.0))
        );
    }

    #[test]
    fn test_alternate_fmt_frequency() {
        assert_eq!("1 hertz", &format!("{:#}", super::hertz(1.0)));
        assert_eq!("1 kilohertz", &format!("{:#}", super::kilohertz(1.0)));
        assert_eq!("1 megahertz", &format!("{:#}", super::megahertz(1.0)));
        assert_eq!("1 gigahertz", &format!("{:#}", super::gigahertz(1.0)));
    }

    #[test]
    fn test_default_fmt_range() {
        assert_eq!(
            "0.000001 MHz - 0.00001 MHz",
            &FrequencyRange::new(super::hertz(1.0), super::hertz(10.0)).to_string()
        );
        assert_eq!(
            "0.001 MHz - 0.01 MHz",
            &FrequencyRange::new(super::kilohertz(1.0), super::kilohertz(10.0)).to_string()
        );
        assert_eq!(
            "1 MHz - 10 MHz",
            &FrequencyRange::new(super::megahertz(1.0), super::megahertz(10.0)).to_string()
        );
        assert_eq!(
            "1000 MHz - 10000 MHz",
            &FrequencyRange::new(super::gigahertz(1.0), super::gigahertz(10.0)).to_string()
        );
    }
}
