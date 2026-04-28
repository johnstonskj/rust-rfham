//! Geographic data types and lookup services for RF-Ham libraries.
//!
//! `rfham-geo` provides:
//!
//! - **Grid systems** ([`grid`]) — trait abstractions for identifier-based locator systems
//!   (e.g. Maidenhead / QTH, implemented in separate crates).
//! - **Geo-IP lookup** ([`geoip`]) — map an IP address to location, locale, and ASN data
//!   via pluggable [`geoip::Provider`] implementations.
//!
//! # Module overview
//!
//! | Module | Key types | Purpose |
//! |--------|-----------|---------|
//! | [`grid`] | [`grid::GridIdentifier`], [`grid::GridPolygon`], [`grid::GridSystem`] | Locator-grid trait abstractions |
//! | [`geoip`] | [`geoip::IpGeoData`], [`geoip::Provider`] | IP-to-location resolution |
//! | [`geoip::providers`] | [`geoip::providers::GeoIpLookup`], [`geoip::providers::NoOp`] | Concrete provider implementations |
//! | [`error`] | [`error::GeoError`], [`error::GeoResult`] | Crate-wide error and result types |
//!
//! # Features
//!
//! - **`std`** *(default)*: enables standard-library support. Disable for `no_std` + `alloc`
//!   environments (note: the HTTP provider requires `std`).

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod geoip;

pub mod grid;
