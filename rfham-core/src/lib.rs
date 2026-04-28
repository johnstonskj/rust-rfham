//! Core data types for RF-Ham libraries.
//!
//! `rfham-core` provides foundational types shared across all crates in the `rust-rfham`
//! ecosystem: callsign parsing, frequency and power measurements, country codes, regulatory
//! agencies, and string-typed identifiers.
//!
//! # Type overview
//!
//! | Module | Key types | Purpose |
//! |--------|-----------|---------|
//! | [`callsign`] | [`callsign::CallSign`] | ITU-format amateur radio callsigns |
//! | [`frequency`] | [`Frequency`], [`frequency::Wavelength`], [`frequency::FrequencyRange`] | RF frequency values and ranges |
//! | [`power`] | [`Power`] | Transmit / receive power levels |
//! | [`country`] | [`CountryCode`] | ISO 3166-1 alpha-2 country codes |
//! | [`agency`] | [`Agency`] | Regulatory and standards bodies |
//! | [`id`] | [`Name`], [`id::DisplayName`], [`id::Tag`] | Validated string identifiers |
//! | [`fmt`] | [`fmt::Formatter`] | Custom formatting trait |
//! | [`conversions`] | [`conversions::LengthInFeet`] | Imperial length representation |
//!
//! # Quick start
//!
//! ```rust
//! use rfham_core::{
//!     callsign::CallSign,
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
//! # Features
//!
//! - **`std`** *(default)*: enables `std`-backed dependencies (I/O errors, `LazyLock`, etc.).
//!   Disable for `no_std` + `alloc` environments.

#[allow(unused_extern_crates)]
extern crate alloc;

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
pub mod names;
pub use names::Name;
pub mod power;
pub use power::Power;
