//!
//! One-line description.
//!
//! More detailed description.
//!
//! # Examples
//!
//! ```rust
//! ```
//!
//! # Features
//!
//! - **feature-name**; Feature description
//!
//!

#[allow(unused_extern_crates)]
extern crate alloc;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod agency;
pub use agency::Agency;
pub mod callsign;
pub mod conversions;
pub mod country;
pub use country::CountryCode;
pub mod fmt;
pub mod frequency;
pub use frequency::Frequency;
pub mod id;
pub use id::Name;
pub mod power;
pub use power::Power;
