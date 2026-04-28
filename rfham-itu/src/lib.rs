//! ITU frequency allocation, band, region, and callsign-series data.
//!
//! `rfham-itu` encodes the radio-frequency spectrum as defined by the International
//! Telecommunication Union (ITU) and the International Amateur Radio Union (IARU).
//!
//! | Module | Key types | Purpose |
//! |--------|-----------|---------|
//! | [`allocations`] | [`allocations::FrequencyAllocation`] | IARU amateur band allocations per region |
//! | [`bands`] | [`bands::FrequencyBand`] | ITU frequency band names (ELF … THF) |
//! | [`regions`] | [`regions::Region`] | ITU Regions 1, 2, and 3 |
//! | [`callsigns`] | [`callsigns::ItuSeriesAllocation`] | ITU callsign prefix–country mapping |
//!
//! # Examples
//!
//! ```rust
//! use rfham_itu::allocations::FrequencyAllocation;
//! use rfham_itu::bands::FrequencyBand;
//! use rfham_core::frequency::megahertz;
//!
//! let band = FrequencyAllocation::classify(megahertz(146.52));
//! assert_eq!(Some(FrequencyAllocation::Band2M), band);
//!
//! assert_eq!("VHF", FrequencyBand::VeryHigh.abbreviation());
//! ```

// use statements

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod allocations;
pub mod bands;
pub mod callsigns;
pub mod regions;
