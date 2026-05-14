//! Geographic data types and lookup services for RF-Ham libraries.
//!
//! `rfham-geo` provides:
//!
//! - **Grid systems** ([`grid`]) — trait abstractions for identifier-based locator systems
//!   (e.g. Maidenhead / QTH, implemented in separate crates).
//! - **Geo-IP lookup** ([`geoip`]) — map an IP address to location, locale, and ASN data
//!   via pluggable [`geoip::Provider`] implementations.
//!
//! # Features
//!
//! - **`std`** *(default)*: enables standard-library support. Disable for `no_std` + `alloc`
//!   environments (note: the HTTP provider requires `std`).

// https://nominatim.org
// https://nominatim.openstreetmap.org/search?q=Bellevue+WA&limit=1&format=jsonv2
// https://nominatim.openstreetmap.org/reverse?lat=<value>&lon=<value>&<params>
// https://geocoding-api.open-meteo.com/v1/search?name=Seattle

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod geoip;

pub mod grid;

pub use lat_long::{Coordinate, Latitude, Longitude};
