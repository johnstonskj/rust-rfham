//! RF frequency, wavelength, and frequency-range types.
//!
//! [`Frequency`] and [`Wavelength`] are thin wrappers around [`uom`](https://docs.rs/uom)
//! SI quantities. They add ham-radio-centric constructors, a smart `Display` that chooses
//! the most readable unit, and implements the bidirectional relationship $λ = \frac{v}{f}$.
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
//! use rfham_core::frequencies::Frequency;
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
//! use rfham_core::frequencies::{Frequency, FrequencyRange};
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

use crate::{Measure, error::CoreError};
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
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This type represents the frequency of a radio signal usingthe SI unit of frequency
/// (symbol: $f$), the Hertz (symbol: $\text{Hz}$).
///
/// # Definition
///
/// Frequency is the number of occurrences of a repeating event per unit of time.
/// Frequency is an important parameter used in science and engineering to specify the rate of
/// oscillatory and vibratory phenomena, such as mechanical vibrations, radio waves, and light.
///
/// # Representation
///
/// The datatype for Frequency is a non-negative `f64` guaranteed to be finite and **not** a NaN.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Frequency(BaseFrequency);

///
///
/// This type represents the wavelength (symbol: $\lambda$) of a radio signal using the SI unit
/// of length, the meter (symbol: $\text{m}$).
///
/// # Definition
///
/// The wavelength (symbol: $\lambda$) of a sine wave can be measured between any two points
/// with the same phase, such as between crests (on top), or troughs (on bottom), or
/// corresponding zero crossings. The wavelength $\lambda$ of a sinusoidal waveform traveling at
/// constant speed $v$, and frequency $f$ is given by $\lambda = \frac{v}{f}$. Given the
/// assumption that radio waves travel at approximately the speed of light (symbol: $c$), this
/// can be expressed as $\lambda = \frac{c}{f}$.
///
/// # Representation
///
/// The datatype for Wavelength is a non-negative `f64` guaranteed to be finite and **not** a NaN.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Wavelength(BaseLength);

///
/// The universal constant, $c$, the speed of light in a vacuum; defined as $299792458\text{ m/s}$.
///
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

///
/// Represents a range (Symbol: $Rf$) of frequencies from `start` (symbol: $f_s$) to `end`
///  (symbol: $f_e$).
///
/// $$Rf = \big[f_s,f_e\big)$$
///
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FrequencyRange(Range<Frequency>);

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

impl Measure for Frequency {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn is_valid(value: f64) -> bool {
        value.is_sign_positive() && value.is_finite() && !value.is_nan()
    }

    fn is_magnitude() -> bool {
        true
    }
}

impl Frequency {
    ///
    /// Construct a new Frequency measured in GigaHertz.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Frequency;
    ///
    /// let v = Frequency::gigahertz(4.0);
    /// assert_eq!("4000 MHz ≣ 4 gigahertz".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn gigahertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::gigahertz>(value))
    }

    ///
    /// Construct a new Frequency measured in MegaHertz.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Frequency;
    ///
    /// let v = Frequency::megahertz(4.0);
    /// assert_eq!("4 MHz ≣ 4 megahertz".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn megahertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::megahertz>(value))
    }

    ///
    /// Construct a new Frequency measured in KiloHertz.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Frequency;
    ///
    /// let v = Frequency::kilohertz(4.0);
    /// assert_eq!("0.004 MHz ≣ 4 kilohertz".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn kilohertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::kilohertz>(value))
    }

    ///
    /// Construct a new Frequency measured in Hertz.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Frequency;
    ///
    /// let v = Frequency::hertz(4.0);
    /// assert_eq!("0.000004 MHz ≣ 4 hertz".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn hertz(value: f64) -> Self {
        Self(BaseFrequency::new::<frequency_unit::hertz>(value))
    }

    ///
    /// The interval of time between events is called the period $T = \frac{1}{f}$. It is the reciprocal of
    /// the frequency and is returned as a number of seconds.
    ///
    pub fn period(&self) -> f64 {
        1.0 / self.value()
    }

    ///
    /// Implements the natural relationship between frequencies and wave lengths, specifically
    /// $\lambda = \frac{c}{f}$. where $f$ is the frequency in Hertz, $c$ is the speed of light
    /// and $\lambda$ is the wavelength in meters.
    ///
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

    /// The start value ()infimum, $f_s$ of this range.
    pub const fn start(&self) -> Frequency {
        self.0.start
    }

    /// The end value (supremum), $f_e$ of this range.
    pub const fn end(&self) -> Frequency {
        self.0.end
    }

    ///
    /// The bandwidth (symbol: $b$) is the magnitude of the range
    /// $$b = f_e - f_s$$
    ///
    pub fn bandwidth(&self) -> Frequency {
        Frequency::hertz(self.0.end.value() - self.0.start.value())
    }

    ///
    /// The mid-point of the band
    /// $$f_m = f_s + \frac{f_e - f_s}{2}$$
    ///
    pub fn mid_band(&self) -> Frequency {
        Frequency::hertz(self.0.start.value() + (self.bandwidth().value() / 2.0))
    }

    ///
    /// Returns `true` if the given frequency is within this range.
    ///
    /// Denoted by the in relation $\in$
    ///
    /// $$f \in Rf \implies f_s \leq f < f_s$$
    ///
    pub fn contains(&self, frequency: Frequency) -> bool {
        self.0.contains(&frequency)
    }

    ///
    /// Returns `true` if the given frequency value in MHz is within this range.
    ///
    /// Denoted by the in relation $\in$
    ///
    /// $$f \in Rf \implies f_s \leq f < f_s$$
    ///
    pub fn contains_mhz(&self, frequency: f64) -> bool {
        self.0.contains(&Frequency::megahertz(frequency))
    }

    ///
    /// Returns `true` if the range is empty. $f_s = f_e$
    ///
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Returns `true` if the ending value of this range is before the starting of `other`.
    ///
    /// Denoted by the less-than relation $<$
    ///
    /// $$Rf_1 < Rf_2 \implies f_{e_1} < f_{s_2}$$
    ///
    pub fn before(&self, other: &Self) -> bool {
        self.end() < other.start()
    }

    ///
    /// Returns `true` if the starting value of this range is after the ending of `other`,
    ///
    /// Denoted by the greater-than relation $>$
    ///
    /// $$Rf_1 > Rf_2 \implies f_{s_1} > f_{e_2}$$
    ///
    pub fn after(&self, other: &Self) -> bool {
        self.start() > other.end()
    }

    ///
    /// Returns `true` if the start value of this range is before the start of `other`,
    /// regardless of the value of either ending value.
    ///
    /// Denoted by the greater-than-similar relation $\lesssim$
    ///
    /// $$Rf_1 \lesssim Rf_2 \implies f_{s_1} < f_{s_2}$$
    ///
    pub fn starts_before(&self, other: &Self) -> bool {
        self.start() < other.start()
    }

    ///
    /// Returns `true` if the range `other` is contained within this range and both have
    /// the same starting value.
    ///
    /// Denoted by the right-triangle relation $\triangleright$
    ///
    /// $$Rf_1 \triangleright Rf_2 \implies f_{s_1} = f_{s_2} \land f_{e_2} \in Rf_1$$
    ///
    pub fn starts_with(&self, other: &Self) -> bool {
        self.start() == other.start() && self.contains(other.end())
    }

    ///
    /// Returns `true` if the end value of this range is before the end of `other`,
    /// regardless of the value of either starting value.
    ///
    /// Denoted by the less-than-similar relation $\gtrsim$
    ///
    /// $$Rf_1 \gtrsim Rf_2 \implies f_{e_1} < f_{e_2}$$
    ///
    pub fn ends_before(&self, other: &Self) -> bool {
        self.end() < other.end()
    }

    ///
    /// Returns `true` if the range `other` is contained within this range and both have
    /// the same ending value.
    ///
    /// Denoted by the left-triangle relation $\triangleleft$
    ///
    /// $$Rf_1 \triangleleft Rf_2 \implies f_{e_1} = f_{e_2} \land f_{s_2} \in Rf_1$$
    ///
    pub fn ends_with(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.end() == other.end()
    }

    ///
    /// Returns `true` if *both* the start and end value of `other` are contained within
    /// this range.
    ///
    /// Denoted by the sub-set relation $\subset$
    ///
    /// $$Rf_1 \subset Rf_2 \implies f_{s_2} \in Rf_1 \land f_{e_2} \in Rf_1$$
    ///
    pub fn is_subrange(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    ///
    /// Returns `true` if *both* the start and end value of this range are contained within
    /// `other`.
    ///
    /// Denoted by the super-set relation $\supset$
    ///
    /// $$Rf_1 \supset Rf_2 \implies f_{s_1} \in Rf_2 \land f_{e_1} \in Rf_2$$
    ///
    pub fn is_superrange(&self, other: &Self) -> bool {
        other.contains(self.start()) && self.contains(self.end())
    }

    ///
    /// Returns `true` if *either* the start and end value of `other` are contained within
    /// this range.
    ///
    /// Denoted by the intersection relation $\cap$
    ///
    /// $$Rf_1 \cap Rf_2 \implies f_{s_2} \in Rf_1 \lor f_{e_2} \in Rf_1$$
    ///
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

impl Measure for Wavelength {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn is_valid(value: f64) -> bool {
        value.is_sign_positive() && value.is_finite() && !value.is_nan()
    }

    fn is_magnitude() -> bool {
        true
    }
}

impl Wavelength {
    ///
    /// Construct a new Wavelength measured in millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Wavelength;
    ///
    /// let v = Wavelength::millimeters(4.0);
    /// assert_eq!("0.004 m ≣ 4 millimeters".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn millimeters(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::millimeter>(value))
    }

    ///
    /// Construct a new Wavelength measured in centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Wavelength;
    ///
    /// let v = Wavelength::centimeters(4.0);
    /// assert_eq!("0.04 m ≣ 4 centimeters".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn centimeters(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::centimeter>(value))
    }

    ///
    /// Construct a new Wavelength measured in meters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Wavelength;
    ///
    /// let v = Wavelength::meters(4.0);
    /// assert_eq!("4 m ≣ 4 meters".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn meters(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::meter>(value))
    }

    ///
    /// Construct a new Wavelength measured in kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::frequencies::Wavelength;
    ///
    /// let v = Wavelength::kilometers(4.0);
    /// assert_eq!("4000 m ≣ 4 kilometers".to_string(), format!("{v} ≣ {v:#}"));
    /// ```
    ///
    pub fn kilometers(value: f64) -> Self {
        Self(BaseLength::new::<length_unit::kilometer>(value))
    }

    ///
    /// Return the underlying value in meters as an `f64`.
    ///
    pub const fn value(&self) -> f64 {
        self.0.value
    }

    ///
    /// Implements the natural relationship between wave lengths and frequencies, specifically
    /// $f = \frac{c}{\lambda}$. where $f$ is the frequency in Hertz, $c$ is the speed of light
    /// and $\lambda$ is the wavelength in meters.
    ///
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
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        Measure,
        frequencies::{Frequency, FrequencyRange},
    };
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
