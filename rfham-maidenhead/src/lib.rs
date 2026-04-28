//!
//! Provides an implementation of a grid system for the IARU Maidenhead Locator System.
//!
//! The grid traits are defined in the `rfham-geo` crate allowing the implementation of multiple grid systems.
//!
//! # The Maidenhead System
//!
//! The Earth's surface is divided into `18 ⨉ 18 = 324` *Fields*, each one `20°` longitude by `10°` latitude.
//! Each Field is divided into `10 * 10 = 100` *Squares*, each one `2°` longitude by `1°` latitude.
//! Each square is finally divided into `24 * 24 = 576` *Subsquares*, each one `5′` longitude by `2.5′` latitude.
//! The Fields are indicated by two letters `AA` - `RR`, the Squares by two digits `00` - `99` and the Subsquares
//! by two letters `AA` - `XX`.
//! The first character is the longitude and the second character is the latitude on each level.
//! The numbering is always West to East and North to South.
//! The complete locator is the sum of all 6 characters, for example "FN43MJ".
//!
//! See [Maidenhead Locator System](https://en.wikipedia.org/wiki/Maidenhead_Locator_System).
//!
//! ```text
//!                Latitude   Longitude
//! ------------------------|------------------------
//!                         B <-- Field
//!               Field --> L
//!                         1 <-- Square
//!              Square --> 1
//!                         b <-- Sub-square
//!          Sub-square --> h
//!                         1 <-- Extended square
//!     Extended square --> 6
//!                  --- optional ---
//!                         0 <-- Extended sub-square
//! Extended sub-square --> 0
//! ```
//!
//! # Examples
//!
//! ```rust
//! use rfham_geo::grid::GridIdentifier;
//! use rfham_maidenhead::{MaidenheadPrecision, MaidenheadLocator};
//! use lat_long::{Coordinate, Latitude, Longitude};
//!
//! assert_eq!(
//!     "CN97hk",
//!     MaidenheadLocator::from_point_with_precision(
//!         Coordinate::new(
//!             Latitude::try_from(47.421375).unwrap(),
//!             Longitude::try_from(-121.410118).unwrap()
//!         ),
//!         MaidenheadPrecision::SubSquare
//!     )
//!     .unwrap()
//!     .as_ref()
//! );
//! ```
//!
//! ```rust
//! use rfham_geo::grid::GridIdentifier;
//! use rfham_maidenhead::{MaidenheadPrecision, MaidenheadLocator};
//! use lat_long::{Coordinate, Latitude, Longitude};
//! use std::str::FromStr;
//!
//!  for s in [
//!      "CN87",
//!      "CN87aa",
//!      "CN87aa00",
//!      "CN87aa00aa",
//!  ] {
//!      let loc = MaidenheadLocator::from_str(s).unwrap();
//!      println!(
//!          "Example: {s:>10} => point: {}, center: {}",
//!          loc.to_point().unwrap(),
//!          loc.center().unwrap(),
//!      )
//!  }
//! ```
//!
//! Results in the following output.
//!
//! ```text
//! Example:       CN87 => point: 47.00000000, -124.00000000, center: 46.50000000, -123.00000000
//! Example:     CN87aa => point: 47.00000000, -124.00000000, center: 46.97916667, -123.95833333
//! Example:   CN87aa00 => point: 47.00000000, -124.00000000, center: 46.99791667, -123.99583333
//! Example: CN87aa00aa => point: 47.00000000, -124.00000000, center: 46.99991319, -123.99982639
//! Example: CN87aa00mm => point: 47.00208333, -123.99583333, center: 47.00199653, -123.99565972
//! Example: CN87aa55mm => point: 47.02291667, -123.95416667, center: 47.02282986, -123.95399306
//! ```
//! # Features
//!
//! TBD
//!
//!

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc as std;

use lat_long::{Angle, Coordinate, Latitude, Longitude};
use rfham_core::{Agency, agencies::agency_iaru, error::CoreError};
use rfham_geo::{
    error::GeoError,
    grid::{GridIdentifier, GridPolygon, GridSystem},
};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, marker::PhantomData, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This String newtype implements the Locator, or *loc* type holding the string encoded square
/// identifier. Maidenhead locators can be specified at multiple levels of precision with increasing
/// specificity, and these levels are described in [`MaidenheadPrecision`] and used in the methods
/// [`from_point_with_precision`] and [`precision`].
///
/// This type also implements the Rf-Ham geo `GridIdentifier` trait.
///
/// # Examples
///
/// ```
/// use rfham_geo::grid::GridIdentifier;
/// use rfham_maidenhead::{MaidenheadPrecision, MaidenheadLocator};
/// use lat_long::{Coordinate, Latitude, Longitude};
/// use std::str::FromStr;
///
/// let loc = MaidenheadLocator::from_str("CN87").unwrap();
/// println!("loc: {loc}");
/// // Output: "loc: CN87"
/// println!("point: {}", loc.to_point().unwrap());
/// // Output: "point: 47.00000000, -124.00000000"
/// println!("center: {}", loc.center().unwrap());
/// // Output: "point: 46.50000000, -123.00000000"
/// ```
///
#[derive(Clone, Debug, DeserializeFromStr, SerializeDisplay)]
pub struct MaidenheadLocator(String);

///
/// This type describes the differing levels of precision provided by different lengths of
/// locator strings. Note that the minimum precision is `Square`, it is meaningless, or at least
/// of no practical use, to use a field-only locator string.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum MaidenheadPrecision {
    /// A locator with *square* precision, a 4-character string.
    Square = 4,
    /// A locator with *sub-square* precision, a 6-character string.
    #[default]
    SubSquare = 6,
    /// A locator with *extended square* precision, a 8-character string.
    ExtendedSquare = 8,
    /// A locator with *extended sub-square* precision, a 10-character string.
    ExtendedSubSquare = 10,
}

///
/// This type also implements the Rf-Ham geo `GridPolygon` trait.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaidenheadSquare {
    loc: MaidenheadLocator,
    south_west: Coordinate,
}

///
/// This type also implements the Rf-Ham geo `GridSystem` trait.
///
/// # Examples
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Maidenhead {
    // Ensures clients **must** use Default to construct.
    private: PhantomData<u32>,
}

// ------------------------------------------------------------------------------------------------
// Constants
// ------------------------------------------------------------------------------------------------

const DEGREES_PER_CIRCLE: f64 = 360.0;
const DEGREES_PER_HALF_CIRCLE: f64 = DEGREES_PER_CIRCLE / 2.0;
const DEGREES_PER_QUARTER_CIRCLE: f64 = DEGREES_PER_HALF_CIRCLE / 2.0;

const FIELDS_PER: f64 = 18.0;
const SQUARES_PER: f64 = 10.0;
const SUB_SQUARES_PER: f64 = 24.0;

const LATITUDE_DEGREES_PER_FIELD: f64 = DEGREES_PER_HALF_CIRCLE / FIELDS_PER;
const LATITUDE_DEGREES_PER_SQUARE: f64 = LATITUDE_DEGREES_PER_FIELD / SQUARES_PER;
const LATITUDE_DEGREES_PER_SUB_SQUARE: f64 = LATITUDE_DEGREES_PER_SQUARE / SUB_SQUARES_PER;
const LATITUDE_DEGREES_PER_EXT_SQUARE: f64 = LATITUDE_DEGREES_PER_SUB_SQUARE / SQUARES_PER;
const LATITUDE_DEGREES_PER_EXT_SUB_SQUARE: f64 = LATITUDE_DEGREES_PER_EXT_SQUARE / SUB_SQUARES_PER;

const LONGITUDE_DEGREES_PER_FIELD: f64 = DEGREES_PER_CIRCLE / FIELDS_PER;
const LONGITUDE_DEGREES_PER_SQUARE: f64 = LONGITUDE_DEGREES_PER_FIELD / SQUARES_PER;
const LONGITUDE_DEGREES_PER_SUB_SQUARE: f64 = LONGITUDE_DEGREES_PER_SQUARE / SUB_SQUARES_PER;
const LONGITUDE_DEGREES_PER_EXT_SQUARE: f64 = LONGITUDE_DEGREES_PER_SUB_SQUARE / SQUARES_PER;
const LONGITUDE_DEGREES_PER_EXT_SUB_SQUARE: f64 =
    LONGITUDE_DEGREES_PER_EXT_SQUARE / SUB_SQUARES_PER;

const NORTH_POLE_LATITUDE: f64 = 180.0;
const SOUTH_POLE_LATITUDE: f64 = 0.0;

const FIELD_LETTERS: [char; 18] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
];

const SQUARE_LETTERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const SUB_SQUARE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const LETTER_A_ASCII_VALUE: u32 = 'A' as u32;

const LETTER_0_ASCII_VALUE: u32 = '0' as u32;

const SQUARE_CORNER_SUFFIX: &str = "00";
const SUB_SQUARE_CORNER_SUFFIX: &str = "aa";

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Grid Precision
// ------------------------------------------------------------------------------------------------

impl Display for MaidenheadPrecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Square => "square",
                Self::SubSquare => "sub-square",
                Self::ExtendedSquare => "extended square",
                Self::ExtendedSubSquare => "extended sub-square",
            }
        )
    }
}

impl TryFrom<usize> for MaidenheadPrecision {
    type Error = GeoError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::Square),
            6 => Ok(Self::SubSquare),
            8 => Ok(Self::ExtendedSquare),
            10 => Ok(Self::ExtendedSubSquare),
            _ => Err(GeoError::Core(CoreError::InvalidValue(
                value.to_string(),
                "GridPrecision",
            ))),
        }
    }
}

impl MaidenheadPrecision {
    /// Returns `true` if the precision is an *extended* value, else `false`.
    pub fn is_extended(&self) -> bool {
        *self > Self::SubSquare
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Grid Identifier
// ------------------------------------------------------------------------------------------------

impl Display for MaidenheadLocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MaidenheadLocator {
    type Err = GeoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(GeoError::Core(CoreError::InvalidValueFromStr(
                s.to_string(),
                "MaidenheadLocator",
            )))
        }
    }
}

impl TryFrom<Coordinate> for MaidenheadLocator {
    type Error = GeoError;

    fn try_from(point: Coordinate) -> Result<Self, Self::Error> {
        Self::from_point_with_precision(point, Default::default())
    }
}

impl From<MaidenheadLocator> for String {
    fn from(value: MaidenheadLocator) -> Self {
        value.0
    }
}

impl TryFrom<MaidenheadLocator> for Coordinate {
    type Error = GeoError;

    fn try_from(locator: MaidenheadLocator) -> Result<Self, Self::Error> {
        locator.to_point()
    }
}

impl AsRef<str> for MaidenheadLocator {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for MaidenheadLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str().eq_ignore_ascii_case(other.0.as_str())
    }
}

impl Eq for MaidenheadLocator {}

impl GridIdentifier for MaidenheadLocator {
    fn is_valid(s: &str) -> bool {
        s.len() >= MaidenheadPrecision::Square as usize
            && s.chars().enumerate().all(|(i, c)| match (i, c) {
                (0..=1, 'A'..='R' | 'a'..='r') => true, // field
                (2..=3, '0'..='9') => true,             // square
                (4..=5, 'A'..='X' | 'a'..='x') => true, // sub-square
                (6..=7, '0'..='9') => true,             // extended square
                (8..=9, 'A'..='X' | 'a'..='x') => true, // extended sub-square
                _ => false,
            })
    }
}

impl MaidenheadLocator {
    pub fn from_point_with_precision(
        point: Coordinate,
        precision: MaidenheadPrecision,
    ) -> Result<Self, GeoError> {
        let latitude: f64 = f64::from(point.latitude()) + DEGREES_PER_QUARTER_CIRCLE;
        let longitude: f64 = f64::from(point.longitude()) + DEGREES_PER_HALF_CIRCLE;

        if latitude == NORTH_POLE_LATITUDE || latitude == SOUTH_POLE_LATITUDE {
            Err(GeoError::NoPolarGrid)
        } else {
            let grid_locator_string = format!(
                "{}{}",
                grid_field_string(latitude, longitude),
                grid_square_string(latitude, longitude)
            );
            let grid_locator_string = if precision >= MaidenheadPrecision::SubSquare {
                format!(
                    "{}{}",
                    grid_locator_string,
                    grid_sub_square_string(latitude, longitude)
                )
            } else {
                grid_locator_string
            };
            let grid_locator_string = if precision >= MaidenheadPrecision::ExtendedSquare {
                format!(
                    "{}{}",
                    grid_locator_string,
                    grid_extended_square_string(latitude, longitude)
                )
            } else {
                grid_locator_string
            };
            let grid_locator_string = if precision >= MaidenheadPrecision::ExtendedSubSquare {
                format!(
                    "{}{}",
                    grid_locator_string,
                    grid_extended_sub_square_string(latitude, longitude)
                )
            } else {
                grid_locator_string
            };
            Ok(Self(grid_locator_string))
        }
    }

    pub fn to_point(&self) -> Result<Coordinate, GeoError> {
        fn char_to_value(c: char) -> f64 {
            (c.to_ascii_uppercase() as u32 - LETTER_A_ASCII_VALUE) as f64
        }

        fn numeric_char_to_value(c: char) -> f64 {
            (c as u32 - LETTER_0_ASCII_VALUE) as f64
        }

        let locator_string = &self.0.to_ascii_uppercase();

        // Add a *central* value for any missing
        let locator_string = match locator_string.len() {
            4 => format!(
                "{}{}{}{}",
                locator_string,
                SUB_SQUARE_CORNER_SUFFIX,
                SQUARE_CORNER_SUFFIX,
                SUB_SQUARE_CORNER_SUFFIX
            ),
            6 => format!(
                "{}{}{}",
                locator_string, SQUARE_CORNER_SUFFIX, SUB_SQUARE_CORNER_SUFFIX
            ),
            8 => format!("{}{}", locator_string, SUB_SQUARE_CORNER_SUFFIX),
            10 => locator_string.clone(),
            _ => {
                return Err(GeoError::Core(CoreError::InvalidValueFromStr(
                    locator_string.to_string(),
                    "MaidenheadLocator",
                )));
            }
        };

        let long_chars: Vec<char> = locator_string
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, c)| c)
            .collect();
        let longitude: f64 = -DEGREES_PER_HALF_CIRCLE
            + (char_to_value(long_chars[0]) * LONGITUDE_DEGREES_PER_FIELD)
            + (numeric_char_to_value(long_chars[1]) * LONGITUDE_DEGREES_PER_SQUARE)
            + (char_to_value(long_chars[2]) / 12.0)
            + (numeric_char_to_value(long_chars[3]) / 120.0)
            + (char_to_value(long_chars[4]) / 2880.0);

        let lat_chars: Vec<char> = locator_string
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, c)| c)
            .collect();
        let latitude: f64 = -DEGREES_PER_QUARTER_CIRCLE
            + (char_to_value(lat_chars[0]) * LATITUDE_DEGREES_PER_FIELD)
            + (numeric_char_to_value(lat_chars[1]) * LATITUDE_DEGREES_PER_SQUARE)
            + (char_to_value(lat_chars[2]) / 24.0)
            + (numeric_char_to_value(lat_chars[3]) / 240.0)
            + (char_to_value(lat_chars[4]) / 5760.0);

        Ok(Coordinate::new(
            Latitude::try_from(latitude)?,
            Longitude::try_from(longitude)?,
        ))
    }

    pub fn precision(&self) -> MaidenheadPrecision {
        MaidenheadPrecision::try_from(self.0.len()).unwrap()
    }

    pub fn width(&self) -> f64 {
        match self.precision() {
            MaidenheadPrecision::Square => LONGITUDE_DEGREES_PER_SQUARE,
            MaidenheadPrecision::SubSquare => LONGITUDE_DEGREES_PER_SUB_SQUARE,
            MaidenheadPrecision::ExtendedSquare => LONGITUDE_DEGREES_PER_EXT_SQUARE,
            MaidenheadPrecision::ExtendedSubSquare => LONGITUDE_DEGREES_PER_EXT_SUB_SQUARE,
        }
    }

    pub fn height(&self) -> f64 {
        match self.precision() {
            MaidenheadPrecision::Square => LATITUDE_DEGREES_PER_SQUARE,
            MaidenheadPrecision::SubSquare => LATITUDE_DEGREES_PER_SUB_SQUARE,
            MaidenheadPrecision::ExtendedSquare => LATITUDE_DEGREES_PER_EXT_SQUARE,
            MaidenheadPrecision::ExtendedSubSquare => LATITUDE_DEGREES_PER_EXT_SUB_SQUARE,
        }
    }

    pub fn center(&self) -> Result<Coordinate, GeoError> {
        let south_west = self.to_point()?;
        Ok(Coordinate::new(
            Latitude::try_from(south_west.latitude().as_float() - (self.height() / 2.0))?,
            Longitude::try_from(south_west.longitude().as_float() + (self.width() / 2.0))?,
        ))
    }

    pub fn field(&self) -> &str {
        &self.0[0..2]
    }
    pub fn square(&self) -> &str {
        &self.0[0..MaidenheadPrecision::Square as usize]
    }

    pub fn sub_square(&self) -> Option<&str> {
        if self.precision() >= MaidenheadPrecision::SubSquare {
            Some(&self.0[0..MaidenheadPrecision::SubSquare as usize])
        } else {
            None
        }
    }

    pub fn is_extended(&self) -> bool {
        self.precision() > MaidenheadPrecision::SubSquare
    }

    pub fn extended_square(&self) -> Option<&str> {
        if self.precision() >= MaidenheadPrecision::ExtendedSquare {
            Some(&self.0[0..MaidenheadPrecision::ExtendedSquare as usize])
        } else {
            None
        }
    }

    pub fn extended_sub_square(&self) -> Option<&str> {
        if self.precision() >= MaidenheadPrecision::ExtendedSubSquare {
            Some(&self.0[0..MaidenheadPrecision::ExtendedSubSquare as usize])
        } else {
            None
        }
    }

    pub fn trim_to_precision(&self, precision: MaidenheadPrecision) -> Self {
        if self.precision() > precision {
            Self(self.0[0..precision as usize].to_string())
        } else {
            self.clone()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Grid Polygon
// ------------------------------------------------------------------------------------------------

impl Display for MaidenheadSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                format!("{} ({:?})", self.loc, self.vertices())
            } else {
                self.loc.to_string()
            }
        )
    }
}

impl FromStr for MaidenheadSquare {
    type Err = GeoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let loc = MaidenheadLocator::from_str(s)?;
        Self::try_from(loc)
    }
}

impl GridPolygon for MaidenheadSquare {
    type Identifier = MaidenheadLocator;

    fn id(&self) -> &MaidenheadLocator {
        &self.loc
    }

    fn vertices(&self) -> Vec<Coordinate> {
        vec![
            // origin
            self.south_west,
            // south east
            Coordinate::new(
                self.south_west.latitude().clone(),
                Longitude::try_from(self.south_west.longitude().as_float() - self.loc.width())
                    .unwrap(),
            ),
            // north west
            Coordinate::new(
                Latitude::try_from(self.south_west.latitude().as_float() + self.loc.height())
                    .unwrap(),
                self.south_west.longitude().clone(),
            ),
            // north east
            Coordinate::new(
                Latitude::try_from(self.south_west.latitude().as_float() + self.loc.height())
                    .unwrap(),
                Longitude::try_from(self.south_west.longitude().as_float() - self.loc.width())
                    .unwrap(),
            ),
        ]
    }

    fn centroid(&self) -> Coordinate {
        Coordinate::new(
            Latitude::try_from(self.south_west.latitude().as_float() - (self.loc.height() / 2.0))
                .unwrap(),
            Longitude::try_from(self.south_west.longitude().as_float() + (self.loc.width() / 2.0))
                .unwrap(),
        )
    }
}

impl TryFrom<MaidenheadLocator> for MaidenheadSquare {
    type Error = GeoError;

    fn try_from(loc: MaidenheadLocator) -> Result<Self, Self::Error> {
        let south_west = loc.to_point()?;
        Ok(Self { loc, south_west })
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Grid System
// ------------------------------------------------------------------------------------------------

impl Display for Maidenhead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name(), self.defining_agency())
    }
}

impl GridSystem for Maidenhead {
    type Identifier = MaidenheadLocator;
    type Poly = MaidenheadSquare;

    fn name(&self) -> &str {
        "Maidenhead Locator System"
    }

    fn defining_agency(&self) -> Agency {
        agency_iaru()
    }

    fn lookup_id(&self, _id: &MaidenheadLocator) -> Result<Option<Self::Poly>, GeoError> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn grid_field_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        FIELD_LETTERS[(longitude / LONGITUDE_DEGREES_PER_FIELD).floor() as usize],
        FIELD_LETTERS[(latitude / LATITUDE_DEGREES_PER_FIELD).floor() as usize],
    )
}

#[inline(always)]
fn grid_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SQUARE_LETTERS[((longitude / LONGITUDE_DEGREES_PER_SQUARE) % SQUARES_PER).floor() as usize],
        SQUARE_LETTERS[((latitude / LATITUDE_DEGREES_PER_SQUARE) % SQUARES_PER).floor() as usize],
    )
}

#[inline(always)]
fn grid_sub_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SUB_SQUARE_LETTERS[((longitude * 12.0) % SUB_SQUARES_PER).floor() as usize],
        SUB_SQUARE_LETTERS[((latitude * 24.0) % SUB_SQUARES_PER).floor() as usize],
    )
}

#[inline(always)]
fn grid_extended_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SQUARE_LETTERS[((longitude * 120.0) % SQUARES_PER).floor() as usize],
        SQUARE_LETTERS[((latitude * 240.0) % SQUARES_PER).floor() as usize],
    )
}

#[inline(always)]
fn grid_extended_sub_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SUB_SQUARE_LETTERS[((longitude * 2880.0) % SUB_SQUARES_PER).floor() as usize],
        SUB_SQUARE_LETTERS[((latitude * 5760.0) % SUB_SQUARES_PER).floor() as usize],
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

///
/// This module exists only to re-export the traits from the `rfham-geo` crate and save an
/// additional dependency.
///
pub mod traits {
    pub use rfham_geo::{
        error::GeoError,
        grid::{GridIdentifier, GridPolygon, GridSystem},
    };
}
