//! Trait abstractions for geographic grid locator systems.
//!
//! A *grid system* divides the Earth's surface into named cells. This module defines the
//! three traits that every concrete grid system must implement:
//!
//! - [`GridIdentifier`] — a validated, string-like cell identifier (e.g. `"CN87"`).
//! - [`GridPolygon`] — the bounding polygon and centroid for a grid cell.
//! - [`GridSystem`] — the registry that converts identifiers or coordinates to polygons.
//!
//! Concrete implementations (e.g. Maidenhead / QTH locator) live in separate crates and
//! depend on this module for the shared interface.
//!
//! # Implementing a grid system
//!
//! ```rust,ignore
//! use rfham_geo::grid::{GridIdentifier, GridPolygon, GridSystem};
//!
//! struct MyId(String);
//! // impl GridIdentifier for MyId { … }
//!
//! struct MyPoly { id: MyId, /* … */ }
//! // impl GridPolygon for MyPoly { … }
//!
//! struct MySystem;
//! // impl GridSystem for MySystem { … }
//! ```

use crate::error::GeoError;
use lat_long::Coordinate;
use rfham_core::Agency;
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A validated string-like identifier for a single cell in a geographic grid system.
///
/// Identifiers must round-trip through `Display` / `FromStr` and can be constructed from
/// a [`Coordinate`] via `TryFrom`.
pub trait GridIdentifier:
    Clone
    + Display
    + FromStr
    + PartialEq
    + Eq
    + Into<String>
    + AsRef<str>
    + TryFrom<Coordinate, Error = GeoError>
{
    /// Returns `true` if `s` is a syntactically valid identifier for this grid system.
    fn is_valid(s: &str) -> bool;

    /// Returns the identifier as a `&str` slice.
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

/// The bounding polygon and centroid of a single grid cell.
pub trait GridPolygon: Clone + PartialEq + Eq + TryFrom<Self::Identifier> {
    type Identifier: GridIdentifier;

    /// Returns a reference to this cell's identifier.
    fn id(&self) -> &Self::Identifier;

    /// Returns the corner vertices of this cell in order.
    fn vertices(&self) -> Vec<Coordinate>;

    /// Returns the geographic centroid of this cell.
    fn centroid(&self) -> Coordinate;
}

/// A grid system that can convert identifiers and coordinates to cell polygons.
pub trait GridSystem: Default {
    type Identifier: GridIdentifier;
    type Poly: GridPolygon<Identifier = Self::Identifier>;

    /// The human-readable name of this grid system (e.g. `"Maidenhead Locator System"`).
    fn name(&self) -> &str;

    /// The agency that defines or maintains this grid system.
    fn defining_agency(&self) -> Agency;

    /// Look up the polygon for a given cell identifier, returning `None` if not found.
    fn lookup_id(&self, id: &Self::Identifier) -> Result<Option<Self::Poly>, GeoError>;

    /// Look up the polygon for the cell that contains `point`.
    ///
    /// The default implementation converts the coordinate to an identifier and delegates
    /// to [`lookup_id`](Self::lookup_id).
    fn lookup_point(&self, point: &Coordinate) -> Result<Option<Self::Poly>, GeoError> {
        self.lookup_id(&Self::Identifier::try_from(*point)?)
    }
}
