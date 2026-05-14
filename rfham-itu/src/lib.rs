//! ITU frequency allocation, band, region, and callsign-series data.
//!
//! `rfham-itu` encodes the radio-frequency spectrum as defined by the International
//! Telecommunication Union (ITU) and the International Amateur Radio Union (IARU).
//!
//! # Examples
//!
//! ```rust
//! use rfham_itu::allocations::FrequencyAllocation;
//! use rfham_itu::bands::FrequencyBand;
//! use rfham_core::frequencies::Frequency;
//!
//! let band = FrequencyAllocation::classify(Frequency::megahertz(146.52));
//! assert_eq!(Some(FrequencyAllocation::Band2M), band);
//!
//! assert_eq!("VHF", FrequencyBand::VeryHigh.abbreviation());
//! ```
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod allocations;
pub mod bands;
pub mod callsigns;
pub mod regions;
