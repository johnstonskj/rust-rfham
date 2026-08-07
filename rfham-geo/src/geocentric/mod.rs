//! This module provides types for geocentric, or Earth-centered, Earth-fixed coordinate system
//! (acronym ECEF), coordinate systems.
//!
//! The following diagram demonstrates the difference between geocentric `X,Y,Z` coordinates
//! and geodetic `φ,λ,h` coordinates.
//!
//! ![Geocentric and Geographic Coordinates](https://raw.githubusercontent.com/johnstonskj/rust-rfham/refs/heads/main/rustdoc/geosphir.gif)
//!
//! The [`CartesianCoordinate`] structure is a 3 dimensional coordinate having
//! a common origin. This origin is *usually* the datum associated with a specific coordinate system.
//!
//! A [`Datum`] trait representing the values necessarily provided by a coordinate system to convert
//! from geodetic or geographic coordinates in latitude, longitude and elevation.
//!
//! Additionally functions are provided for the conversion from geographic and geocentric coordinates.
//!
//! # Examples
//!
//! The following demonstrates the conversion from a geographic coordinate, the latitude and logitude
//! of a location within London (UK) into geocentric coordinates.
//!
//! ```rust
//! use rfham_geo::geocentric::{
//!     CartesianCoordinate, geographic_to_geocentric,
//!     wgs::Wgs84Datum,
//! };
//! use lat_long::{CoordinateWithElevation, Elevation, Latitude, Longitude};
//!
//! let london = CoordinateWithElevation::new_from(
//!     Latitude::try_from(51.5072222).unwrap(),
//!     Longitude::try_from(0.1275000).unwrap(),
//!     Elevation::try_from(42.0).unwrap(),
//! );
//!
//! let converted = geographic_to_geocentric(&Wgs84Datum, &london);
//!
//! assert_eq!(
//!     CartesianCoordinate::new_raw(
//!         3978035.9444535594,
//!         8852.317298701908,
//!         4968895.497702952
//!     ),
//!     converted
//! )
//! ```
//!
//! The following demonstrates the reverse conversion from geocentric coordinates to geographic.
//!
//! ```rust
//! use crate::geocentric::{
//!     CartesianCoordinate, geocentric_to_geographic,
//!     wgs::Wgs84Datum,
//! };
//! use lat_long::{CoordinateWithElevation, Elevation, Latitude, Longitude};
//! use pretty_assertions::assert_eq;
//!
//! let london = CartesianCoordinate::new_raw(
//!     3978035.9444535594, 8852.317298701908, 4968895.497702952
//! );
//!
//! let converted = geocentric_to_geographic(&Wgs84Datum, &london);
//!
//! assert_eq!(
//!     CoordinateWithElevation::new_from(
//!         Latitude::try_from(51.50722220000001).unwrap(),
//!         Longitude::try_from(0.1275000).unwrap(),
//!         Elevation::try_from(42.0).unwrap(),
//!     ),
//!     converted
//! )
//! ```

use crate::error::GeoError;
use lat_long::{
    Angle, CoordinateWithElevation as GeographicCoordinate, Elevation, Latitude, Longitude,
};
use rfham_core::error::CoreError;
use std::{fmt::Display, str::FromStr};
use tracing::error;
use uom::si::{f64::Length, length::meter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A three-dimensional cartesian coordinate denoting an Earth-centered, Earth-fixed coordinate
/// coordinate.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct CartesianCoordinate {
    x: Length,
    y: Length,
    z: Length,
}

///
/// A Datum is a reference point used by a Geocentric Coordinate System in calculating coordinates.
///
/// In conversion from Latitude, Longitude, and Elevation (LLE) it is necessary for a Datum to
/// provide a value for the semi-major axis and the flattening value.
///
pub trait Datum {
    ///
    /// The equitorial radius of the Earth, in formula it is denoted as $a$.
    ///
    fn semi_major_axis(&self) -> Length;

    ///
    /// The flattening of the Earth, in formula it is denoted as $f$.
    ///
    fn flattening(&self) -> f64;

    ///
    /// The polar radius of the Earth, in formula it is denoted as $b$, and calculated from $a$ as
    /// $b = a \cdot (1 - f)$.
    ///
    fn semi_minor_axis(&self) -> Length {
        Length::new::<meter>(self.semi_major_axis().value * (1.0 - self.flattening()))
    }

    ///
    /// The Inverse Flattening, or $\frac{1}{f}$.
    ///
    #[inline(always)]
    fn inverse_flattening(&self) -> f64 {
        1.0 / self.flattening()
    }

    ///
    /// The First Eccentricity Squared, in formula denoted as $e^2$, and calculated as $2 f - f^2$.
    ///
    fn first_eccentricity_squared(&self) -> f64 {
        (self.flattening() * 2.0) - (self.flattening().powi(2))
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Convert a Geographic Latitude, Logitude, Elevation (LLE) coordinate to an Earth-Centered Earth-Fixed
/// (ECEF) Cartesian coordinate.
///
/// # The Math
///
/// Latitude and Longitude must be in radians, so if necessary convert from degrees first.
///
/// $$\varphi = \varphi_{deg} \times \frac{\pi}{180}$$
/// $$\lambda = \lambda_{deg} \times \frac{\pi}{180}$$
///
/// First, calculate the radius of curvature ($N$) using the prime vertical:
///
/// $$N = \frac{a}{\sqrt{1-e^2sin(\varphi)^2}}$$
///
/// Where $e^2 = 2f - f^2$ (the first eccentricity squared), and $\varphi$ is the latitude.
///
/// Then, compute the ECEF cartesian coordinates ($X$, $Y$, $Z$) using your latitude ($\varphi$),
/// longitude ($\lambda$), and elevation ($h$).
///
/// $$ X = (N + h) cos(\varphi) cos(\lambda)$$
/// $$ Y = (N + h) cos(\varphi) sin(\lambda)$$
/// $$ Z = (N (1 - e^2) + h) sin(\varphi)$$
///
pub fn geographic_to_geocentric(
    datum: &impl Datum,
    coord: &GeographicCoordinate,
) -> CartesianCoordinate {
    let phi = coord.point().latitude().to_radians();
    let lambda = coord.point().longitude().to_radians();

    let h = coord.elevation().value();

    let n = datum.semi_major_axis().value
        / (1.0 - datum.first_eccentricity_squared() * phi.sin().powi(2)).sqrt();

    let x = (n + h) * phi.cos() * lambda.cos();
    let y = (n + h) * phi.cos() * lambda.sin();
    let z = (n * (1.0 - datum.first_eccentricity_squared()) + h) * phi.sin();

    CartesianCoordinate::new_raw(x, y, z)
}

///
/// Convert a Geographic Latitude, Logitude, Elevation (LLE) coordinate to an Earth-Centered Earth-Fixed
/// (ECEF) Cartesian coordinate.
///
/// # The Math
///
/// Longitude ($\lambda$) is calculated directly using the arctangent of ($Y$) and ($X$):
///
/// $$\lambda = arctan2(Y, X)$$
///
/// To find latitude (\varphi$) and elevation ($h$), first compute the distance from the Z-axis:
///
/// $$p = \sqrt{X^2 + Y^2}$$
///
/// Then, calculate the initial parameter ($\theta$) (sometimes referred to as the parametric latitude):
///
/// $$\theta = arctan2(Z \cdot a, p \cdot b)$$
///
/// Use $\theta$ to compute latitude ($\varphi$) via the closed-form Bowring / Zhu equation:
///
/// $$\varphi = arctan2\left(Z + eps \cdot b \cdot sin(\theta)^3, p-e^2 \cdot a \cdot cos(\theta)^3\right)$$
///
/// Finally, calculate the prime vertical radius of curvature ($N$) and then the elevation ($h$):
///
/// $$N = \frac{a}{\sqrt{1 - e^2 \cdot sin(\varphi)^2}}$$
///
/// $$h = \frac{p}{cos(\varphi)} - N$$
///
pub fn geocentric_to_geographic(
    datum: &impl Datum,
    coord: &CartesianCoordinate,
) -> GeographicCoordinate {
    // Input values unwrapped.
    let x: f64 = coord.x_raw();
    let y: f64 = coord.y_raw();
    let z: f64 = coord.z_raw();

    // Datum values unwrapped.
    let a: f64 = datum.semi_major_axis().value;
    let b: f64 = datum.semi_minor_axis().value;
    let e2: f64 = datum.first_eccentricity_squared();

    // Handle polar conditions.
    let p = (x.powi(2) + y.powi(2)).sqrt();
    if p == 0.0 {
        GeographicCoordinate::new(
            (
                Latitude::try_from(if z > 0.0 { 90.0 } else { -90.0 }).unwrap(),
                Longitude::try_from(0.0).unwrap(),
            )
                .into(),
            Elevation::try_from(Length::new::<meter>(z.abs() - b)).unwrap(),
        )
    } else {
        // Intermediate values
        let eps = e2 / (1.0 - e2);
        let q = (z * a).atan2(p * b);

        // Latitude
        let phi = (z + eps * b * q.sin().powi(3)).atan2(p - e2 * a * q.cos().powi(3));

        // Longitude
        let lambda = y.atan2(x);

        // Elevation
        let v = a / (1.0 - e2 * phi.sin().powi(2)).sqrt();
        let h = (p / phi.cos()) - v;

        GeographicCoordinate::new(
            (
                Latitude::try_from(phi.to_degrees()).unwrap(),
                Longitude::try_from(lambda.to_degrees()).unwrap(),
            )
                .into(),
            Elevation::try_from(Length::new::<meter>(h)).unwrap(),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for CartesianCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x.value, self.y.value, self.z.value)
    }
}

impl FromStr for CartesianCoordinate {
    type Err = GeoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            error!(
                "Invalid number of components for a CartesianCoordinate; value: {s}, components: {}, expecting: 3",
                parts.len()
            );
            Err(CoreError::InvalidValueFromStr(s.to_string(), "CartesianCoordinate").into())
        } else {
            Ok(Self {
                x: Length::new::<meter>(f64::from_str(parts[0].trim()).map_err(|e| {
                    error!(
                        "Invalid representation of an f64 value parsing `x`; value: {s}, error: {e}"
                    );
                    CoreError::InvalidValueFromStr(s.to_string(), "CartesianCoordinate")
                })?),
                y: Length::new::<meter>(f64::from_str(parts[1].trim()).map_err(|e| {
                    error!(
                        "Invalid representation of an f64 value parsing `y`; value: {s}, error: {e}"
                    );
                    CoreError::InvalidValueFromStr(s.to_string(), "CartesianCoordinate")
                })?),
                z: Length::new::<meter>(f64::from_str(parts[2].trim()).map_err(|e| {
                    error!(
                        "Invalid representation of an f64 value parsing `z`; value: {s}, error: {e}"
                    );
                    CoreError::InvalidValueFromStr(s.to_string(), "CartesianCoordinate")
                })?),
            })
        }
    }
}

impl From<(Length, Length, Length)> for CartesianCoordinate {
    fn from(value: (Length, Length, Length)) -> Self {
        CartesianCoordinate::new(value.0, value.1, value.2)
    }
}

impl From<(f64, f64, f64)> for CartesianCoordinate {
    fn from(value: (f64, f64, f64)) -> Self {
        CartesianCoordinate::new_raw(value.0, value.1, value.2)
    }
}

impl From<CartesianCoordinate> for (Length, Length, Length) {
    fn from(value: CartesianCoordinate) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<CartesianCoordinate> for (f64, f64, f64) {
    fn from(value: CartesianCoordinate) -> Self {
        (value.x.value, value.y.value, value.z.value)
    }
}

impl CartesianCoordinate {
    pub fn new(x: Length, y: Length, z: Length) -> Self {
        Self { x, y, z }
    }

    pub fn new_raw(x: f64, y: f64, z: f64) -> Self {
        Self::new(
            Length::new::<meter>(x),
            Length::new::<meter>(y),
            Length::new::<meter>(z),
        )
    }

    pub fn x(&self) -> Length {
        self.x
    }

    pub fn x_raw(&self) -> f64 {
        self.x.value
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn y_raw(&self) -> f64 {
        self.y.value
    }

    pub fn z(&self) -> Length {
        self.z
    }

    pub fn z_raw(&self) -> f64 {
        self.z.value
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod nad;

pub mod wgs;
