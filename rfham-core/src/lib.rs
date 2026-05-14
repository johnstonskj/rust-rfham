//! Core data types for RF-Ham libraries.
//!
//! `rfham-core` provides foundational types shared across all crates in the `rust-rfham`
//! ecosystem: callsign parsing, frequency and power measurements, country codes, regulatory
//! agencies, and string-typed identifiers.
//!
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::{
//!     callsigns::CallSign,
//!     Frequency,
//!     Power,
//! };
//!
//! let callsign: CallSign = "K7SKJ/M".parse().unwrap();
//! assert_eq!(callsign.prefix(), "K");
//! assert!(callsign.is_mobile());
//!
//! let freq = Frequency::megahertz(146.52);
//! let power = Power::watts(5.0);
//! println!("{callsign} on {freq} at {power}");
//! ```
//!
//! # Feature flags
#![doc = document_features::document_features!()]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

use std::{fmt::Display, hash::Hash, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

///
/// A trait for types that are primarily represented as strings, and generally implemented as a
/// newtype wrapper around `String`.
///
pub trait StringLike:
    Clone + Display + PartialEq + Eq + PartialOrd + Ord + Hash + FromStr + AsRef<str> + Into<String>
{
    ///
    /// Maximum allowed length of the string value, used for validation and display purposes.
    const MAX_LENGTH: usize;

    ///
    /// Creates a new instance from a string without validation. Use with care.
    ///
    fn new_unchecked<S: Into<String>>(name: S) -> Self;

    ///
    /// Returns the inner value as a string slice.
    ///
    fn as_str(&self) -> &str;

    ///
    /// Validates whether a given string is a valid value for this type.
    ///
    fn is_valid(s: &str) -> bool;
}

///
/// A measure is a floating-point wrapper for key physical units such as frequency, wavelength, and
/// power.
///
/// Measures **must** support `Display` and `FromStr` as they have string representations which
/// may include a unit-of-measure string. The addition of `TryFrom<f64>` indicates that not all float
/// values are valid for the measure; see [`Measure::is_valid`].
///
pub trait Measure: Display + FromStr + TryFrom<f64> + Into<f64> {
    ///
    /// Returns the value of this measure as an `f64`.
    ///
    fn value(&self) -> f64;

    ///
    /// Returns `true` if the provided `f64` value is a valid representation for this measure.
    ///
    /// Most measures **must** be finite (`f64::is_finite`), not NaNs (`f64::is_nan`), and some
    /// may only be positive values (`f64:is_sign_positive`) as they are magnitudes.
    ///
    fn is_valid(value: f64) -> bool;

    ///
    /// Returns `true` if the values of this measure represent a *magnitude* and therefore will
    /// **not** be negative.
    ///
    /// An implementation may choose to treat a negative value as an error, or store the absolute
    /// value, depending on common usage.
    ///
    fn is_magnitude() -> bool;
}

///
/// A bi-directional measure is **not** a manitude but may represent positive and negative values as
/// *forward* and *reverse* directions.
///
/// For example, power in Watts watts can be negative. In electrical systems, negative watts indicate
/// that power is flowing backwards, meaning a device is supplying or generating power rather than
/// consuming it. This is common in solar power systems, battery charging, or when a meter is
/// installed incorrectly.
///
pub trait BidirectionalMeasure: Measure {
    ///
    /// Returns `true` if this value is flowing *forward*, generally meaning a device is consuming it.
    ///
    fn is_forward(&self) -> bool {
        self.value().is_sign_positive()
    }

    ///
    /// Returns `true` if power is flowing *backward*, generally meaning a device is supplying or
    /// generating the value.
    ///
    fn is_backward(&self) -> bool {
        self.value().is_sign_negative()
    }

    fn magnitude(&self) -> Self
    where
        Self: Sized,
    {
        Self::try_from(self.value().abs()).map_err(|_| ()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod agencies;
pub use agencies::Agency;
pub mod callsigns;
pub mod countries;
pub mod non_si;
pub use countries::CountryCode;
pub mod fmt;
pub mod frequencies;
pub use frequencies::Frequency;
pub mod licenses;
pub mod names;
pub use names::Name;
pub mod power;
pub use power::Power;
