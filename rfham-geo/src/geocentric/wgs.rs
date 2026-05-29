//! The World Geodetic System (WGS) is a standard used in cartography, geodesy, and
//! satellite navigation including GPS.
//!

use crate::geocentric::Datum;
use uom::si::{f64::Length, length::meter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// WGS 66 (World Geodetic System 1972) is an obsolete global geodetic datum and coordinate
/// system developed by the U.S. Department of Defense. Replaced by WGS 72.
/// # Parameters
///
/// - $a = 6378137 m$
/// - $f = \frac{1}{298.257223563}$
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wgs66Datum;

///
/// WGS 72 (World Geodetic System 1972) is an obsolete global geodetic datum and coordinate
/// system developed by the U.S. Department of Defense. Replaced by WGS 84.
///
/// # Parameters
///
/// - $a = 6378137 m$
/// - $f = \frac{1}{298.257223563}$
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wgs72Datum;

///
/// WGS84 (World Geodetic System 1984) is the globally accepted standard coordinate reference
/// frame used for mapping, geodesy, and satellite navigation, including GPS. It defines
/// Earth's shape, gravity, and coordinate grid using the Earth's center of mass as its exact
/// origin.
///
/// # Parameters
///
/// - $a = 6378137 m$
/// - $f = \frac{1}{298.257223563}$
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wgs84Datum;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Datum for Wgs66Datum {
    #[inline(always)]
    fn semi_major_axis(&self) -> Length {
        Length::new::<meter>(6378145.0)
    }

    #[inline(always)]
    fn flattening(&self) -> f64 {
        1.0 / 298.25
    }
}

impl Datum for Wgs72Datum {
    #[inline(always)]
    fn semi_major_axis(&self) -> Length {
        Length::new::<meter>(6378135.0)
    }

    #[inline(always)]
    fn flattening(&self) -> f64 {
        1.0 / 298.26
    }
}

impl Datum for Wgs84Datum {
    #[inline(always)]
    fn semi_major_axis(&self) -> Length {
        Length::new::<meter>(6378137.0)
    }

    #[inline(always)]
    fn flattening(&self) -> f64 {
        1.0 / 298.257223563
    }
}
