//! RF frequency, wavelength, and frequency-range types.
//!
//! [`Frequency`] and [`Wavelength`] are thin wrappers around [`uom`](https://docs.rs/uom)
//! SI quantities. They add ham-radio-centric constructors, a smart `Display` that chooses
//! the most readable unit, and bidirectional conversion via λ = c / f.
//!
//! [`FrequencyRange`] represents a contiguous band segment and supports overlap and
//! containment queries.
//!
//! # Display formats
//!
//! The default formatter (`{}`) always uses MHz. The alternate formatter (`{:#}`) selects
//! the most natural unit based on the value:
//!
//! ```rust
//! use rfham_core::frequency::Frequency;
//!
//! assert_eq!(format!("{:#}", Frequency::hertz(440.0)),      "440 hertz");
//! assert_eq!(format!("{:#}", Frequency::kilohertz(7.074)),  "7.074 kilohertz");
//! assert_eq!(format!("{:#}", Frequency::megahertz(146.52)), "146.52 megahertz");
//! assert_eq!(format!("{:#}", Frequency::gigahertz(2.4)),    "2.4 gigahertz");
//! ```
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::frequency::{Frequency, FrequencyRange};
//!
//! // Construct and display
//! let f = Frequency::megahertz(144.0);
//! assert_eq!(f.to_string(), "144 MHz");
//!
//! // Convert to wavelength (~2 m band)
//! let wl = f.to_wavelength();
//! assert!((wl.value() - 2.082).abs() < 0.001);
//!
//! // Range queries
//! let band = FrequencyRange::new_mhz(144.0, 148.0);
//! assert!(band.contains(Frequency::megahertz(146.52)));
//! assert!(!band.contains(Frequency::megahertz(150.0)));
//! ```

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
        Frequency::megahertz(value)
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
    pub fn gigahertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::gigahertz>(value))
    }
    pub fn megahertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::megahertz>(value))
    }
    pub fn kilohertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::kilohertz>(value))
    }
    pub fn hertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::hertz>(value))
    }

    pub const fn value(&self) -> f64 {
        self.0.value
    }

    pub fn to_wavelength(&self) -> Wavelength {
        Wavelength::meters(SPEED_OF_LIGHT / self.value())
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
        Self::new(
            Frequency::megahertz(range.start),
            Frequency::megahertz(range.end),
        )
    }
}

impl From<(Frequency, Frequency)> for FrequencyRange {
    fn from(range: (Frequency, Frequency)) -> Self {
        Self::new(range.0, range.1)
    }
}

impl From<(f64, f64)> for FrequencyRange {
    fn from(range: (f64, f64)) -> Self {
        Self::new(Frequency::megahertz(range.0), Frequency::megahertz(range.1))
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
        Self::new(Frequency::megahertz(start), Frequency::megahertz(end))
    }

    pub const fn start(&self) -> Frequency {
        self.0.start
    }

    pub const fn end(&self) -> Frequency {
        self.0.end
    }

    pub fn bandwidth(&self) -> Frequency {
        Frequency::hertz(self.0.end.value() - self.0.start.value())
    }

    pub fn mid_band(&self) -> Frequency {
        Frequency::hertz(self.0.start.value() + (self.bandwidth().value() / 2.0))
    }

    pub fn contains(&self, frequency: Frequency) -> bool {
        self.0.contains(&frequency)
    }

    pub fn contains_mhz(&self, frequency: f64) -> bool {
        self.0.contains(&Frequency::megahertz(frequency))
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
        Wavelength::meters(value)
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
    pub fn millimeters(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::millimeter>(value))
    }

    pub fn centimeters(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::centimeter>(value))
    }

    pub fn meters(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::meter>(value))
    }

    pub fn kilometers(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::kilometer>(value))
    }

    pub const fn value(&self) -> f64 {
        self.0.value
    }

    pub fn to_frequency(&self) -> Frequency {
        Frequency::hertz(SPEED_OF_LIGHT / self.value())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn gigahertz(value: f64) -> Frequency {
    Frequency::gigahertz(value)
}

pub fn megahertz(value: f64) -> Frequency {
    Frequency::megahertz(value)
}

pub fn kilohertz(value: f64) -> Frequency {
    Frequency::kilohertz(value)
}

pub fn hertz(value: f64) -> Frequency {
    Frequency::hertz(value)
}

pub fn meters(value: f64) -> Wavelength {
    Wavelength::meters(value)
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
    use super::{Frequency, FrequencyRange};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_fmt_frequency() {
        assert_eq!("0.000001 MHz", &Frequency::hertz(1.0).to_string());
        assert_eq!("0.001 MHz", &Frequency::kilohertz(1.0).to_string());
        assert_eq!("1 MHz", &Frequency::megahertz(1.0).to_string());
        assert_eq!("1000 MHz", &Frequency::gigahertz(1.0).to_string());
    }

    #[test]
    fn test_default_fmt_frequency_precision() {
        assert_eq!("0.000001 MHz", &format!("{:.6}", Frequency::hertz(1.0)));
        assert_eq!("0.001000 MHz", &format!("{:.6}", Frequency::kilohertz(1.0)));
        assert_eq!("1.000000 MHz", &format!("{:.6}", Frequency::megahertz(1.0)));
        assert_eq!(
            "1000.000000 MHz",
            &format!("{:.6}", Frequency::gigahertz(1.0))
        );
    }

    #[test]
    fn test_default_fmt_frequency_width_and_precision() {
        assert_eq!(
            "      0.000001 MHz",
            &format!("{:>14.6}", Frequency::hertz(1.0))
        );
        assert_eq!(
            "      0.001000 MHz",
            &format!("{:>14.6}", Frequency::kilohertz(1.0))
        );
        assert_eq!(
            "      1.000000 MHz",
            &format!("{:>14.6}", Frequency::megahertz(1.0))
        );
        assert_eq!(
            "   1000.000000 MHz",
            &format!("{:>14.6}", Frequency::gigahertz(1.0))
        );
    }

    #[test]
    fn test_alternate_fmt_frequency() {
        assert_eq!("1 hertz", &format!("{:#}", Frequency::hertz(1.0)));
        assert_eq!("1 kilohertz", &format!("{:#}", Frequency::kilohertz(1.0)));
        assert_eq!("1 megahertz", &format!("{:#}", Frequency::megahertz(1.0)));
        assert_eq!("1 gigahertz", &format!("{:#}", Frequency::gigahertz(1.0)));
    }

    #[test]
    fn test_default_fmt_range() {
        assert_eq!(
            "0.000001 MHz - 0.00001 MHz",
            &FrequencyRange::new(Frequency::hertz(1.0), Frequency::hertz(10.0)).to_string()
        );
        assert_eq!(
            "0.001 MHz - 0.01 MHz",
            &FrequencyRange::new(Frequency::kilohertz(1.0), Frequency::kilohertz(10.0)).to_string()
        );
        assert_eq!(
            "1 MHz - 10 MHz",
            &FrequencyRange::new(Frequency::megahertz(1.0), Frequency::megahertz(10.0)).to_string()
        );
        assert_eq!(
            "1000 MHz - 10000 MHz",
            &FrequencyRange::new(Frequency::gigahertz(1.0), Frequency::gigahertz(10.0)).to_string()
        );
    }

    #[test]
    fn test_frequency_wavelength_roundtrip() {
        let f = Frequency::megahertz(144.0);
        let wl = f.to_wavelength();
        let f2 = wl.to_frequency();
        assert!((f.value() - f2.value()).abs() < 1e-6);
    }

    #[test]
    fn test_frequency_range_bandwidth() {
        let r = FrequencyRange::new_mhz(144.0, 148.0);
        // Internally stored in Hz; 4 MHz = 4_000_000 Hz
        assert!((r.bandwidth().value() - 4_000_000.0).abs() < 1.0);
    }

    #[test]
    fn test_frequency_range_mid_band() {
        let r = FrequencyRange::new_mhz(144.0, 148.0);
        // Mid-band at 146 MHz = 146_000_000 Hz
        assert!((r.mid_band().value() - 146_000_000.0).abs() < 1.0);
    }

    #[test]
    fn test_frequency_range_contains() {
        let r = FrequencyRange::new_mhz(144.0, 148.0);
        assert!(r.contains_mhz(144.0));
        assert!(r.contains_mhz(146.52));
        assert!(!r.contains_mhz(148.0)); // Range end is exclusive
        assert!(!r.contains_mhz(150.0));
    }

    #[test]
    fn test_frequency_range_overlap() {
        let a = FrequencyRange::new_mhz(144.0, 148.0);
        let b = FrequencyRange::new_mhz(146.0, 150.0);
        let c = FrequencyRange::new_mhz(150.0, 160.0);
        assert!(a.is_overlapping(&b));
        assert!(!a.is_overlapping(&c));
    }

    #[test]
    fn test_frequency_range_subrange() {
        let outer = FrequencyRange::new_mhz(144.0, 148.0);
        let inner = FrequencyRange::new_mhz(145.0, 147.0);
        assert!(outer.is_subrange(&inner));
        assert!(!inner.is_subrange(&outer));
    }

    #[test]
    fn test_frequency_from_f64_is_megahertz() {
        let f: Frequency = 146.52_f64.into();
        assert_eq!(f.to_string(), "146.52 MHz");
    }
}
