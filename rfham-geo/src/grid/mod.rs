//!
//! Provides the traits [`GridIdentifier`], [`GridPoly`], and [`GridSystem`].
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::error::GeoError;
use lat_long::Coordinate;
use rfham_core::Agency;
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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
    fn is_valid(s: &str) -> bool;
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

pub trait GridPolygon {
    type Identifier: GridIdentifier;
    fn id(&self) -> &Self::Identifier;
    fn vertices(&self) -> Vec<Coordinate>;
}

pub trait GridSystem {
    type Identifier: GridIdentifier;
    type Poly: GridPolygon<Identifier = Self::Identifier>;

    fn name(&self) -> &str;

    fn defining_agency(&self) -> Agency;

    fn lookup_id(&self, id: &Self::Identifier) -> Result<Option<Self::Poly>, GeoError>;

    fn lookup_point(&self, point: &Coordinate) -> Result<Option<Self::Poly>, GeoError> {
        self.lookup_id(&Self::Identifier::try_from(*point)?)
    }
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod maidenhead;
