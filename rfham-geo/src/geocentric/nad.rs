//! The North American Datum (NAD) is a geometric coordinate system used for mapping and
//! surveying in the U.S., Canada, Mexico, and Central America. It ties coordinates to
//! the North American tectonic plate.
//!
//! Defined by the
//! [National Geodetic Survey](https://geodesy.noaa.gov/datums/horizontal/north-american-datum-1983.shtml).
//!

use crate::geocentric::Datum;
use uom::si::{f64::Length, length::meter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// NAD27 (North American Datum of 1927) is a legacy geodetic reference system used for mapping
/// North America.
///
/// # Parameters
///
/// - $a = 6378206.4 m$
/// - $f = \frac{1}{294.9786982}$
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nad27Datum;

///
/// The North American Datum of 1983 (NAD 83) is the horizontal and geometric control datum for
/// the United States, Canada, Mexico, and Central America. NAD 83 was released in 1986.
///
/// # Parameters
///
/// - $a = 6378137.0 m$
/// - $f = \frac{1}{298.257222101}$
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nad83Datum;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Datum for Nad27Datum {
    #[inline(always)]
    fn semi_major_axis(&self) -> Length {
        Length::new::<meter>(6378206.4)
    }

    #[inline(always)]
    fn flattening(&self) -> f64 {
        1.0 / 294.9786982
    }
}

impl Datum for Nad83Datum {
    #[inline(always)]
    fn semi_major_axis(&self) -> Length {
        Length::new::<meter>(6378137.0)
    }

    #[inline(always)]
    fn flattening(&self) -> f64 {
        1.0 / 298.257222101
    }
}
