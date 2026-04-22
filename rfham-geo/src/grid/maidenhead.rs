//!
//! Provides an implementation of a grid system for the IARU
//! [Maidenhead Locator System](https://en.wikipedia.org/wiki/Maidenhead_Locator_System).
//!
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
//! use rfham_geo::grid::{
//!     GridIdentifier,
//!     maidenhead::{GridPrecision, MaidenheadLocator},
//! };
//! use lat_long::{Coordinate, Latitude, Longitude};
//!
//! assert_eq!(
//!     "CN97hk",
//!     MaidenheadLocator::from_point_with_precision(
//!         Coordinate::new(
//!             Latitude::try_from(47.421375).unwrap(),
//!             Longitude::try_from(-121.410118).unwrap()
//!         ),
//!         GridPrecision::SubSquare
//!     )
//!     .as_ref()
//! );
//! ```
//!

use crate::{
    error::GeoError,
    grid::{GridIdentifier, GridPolygon, GridSystem},
};
use lat_long::{Coordinate, Latitude, Longitude};
use rfham_core::{Agency, agency::agency_iaru, error::CoreError};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct MaidenheadLocator(String);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Maidenhead {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum GridPrecision {
    Square = 4,
    #[default]
    SubSquare = 6,
    ExtendedSquare = 8,
    ExtendedSubSquare = 10,
}

/// Describe this struct.
///
/// # Fields
///
/// - `name` (`String`) - Describe this field.
/// - `top_left` (`Coordinate`) - Describe this field.
/// - `bottom_right` (`Coordinate`) - Describe this field.
///
/// # Examples
///
/// ```rust,ignore
/// use crate::...;
///
/// let s = MaidenheadSquare {
///     name: value,
///     top_left: value,
///     bottom_right: value,
/// };
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct MaidenheadSquare {
    name: MaidenheadLocator,
    top_left: Coordinate,
    bottom_right: Coordinate,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Grid Precision
// ------------------------------------------------------------------------------------------------

impl Display for GridPrecision {
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

impl TryFrom<usize> for GridPrecision {
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

impl GridPrecision {
    /// Returns `true` if the precision is an *extended* value, else `false`.
    pub fn is_extended(&self) -> bool {
        *self > Self::SubSquare
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Grid Identifier
// ------------------------------------------------------------------------------------------------

const FIELD_LETTERS: [char; 18] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
];

const SQUARE_LETTERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const SUB_SQUARE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

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

impl From<Coordinate> for MaidenheadLocator {
    fn from(point: Coordinate) -> Self {
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

impl GridIdentifier for MaidenheadLocator {
    fn is_valid(s: &str) -> bool {
        s.len() >= GridPrecision::Square as usize
            && s.chars().enumerate().all(|(i, c)| match (i, c) {
                (0..=1, 'A'..='R' | 'a'..='r') => true, // field
                (2..=3, '0'..='9') => true,             // square
                (4..=5, 'A'..='X' | 'a'..='x') => true, // sub-square
                (6..=9, '0'..='9') => true,             // extended ...
                _ => {
                    println!("MaidenheadLocator::from_str not happy; i: {i}, c: {c}");
                    false
                }
            })
    }
}

impl MaidenheadLocator {
    pub fn from_point_with_precision(point: Coordinate, precision: GridPrecision) -> Self {
        let latitude: f64 = f64::from(point.latitude()) + 90.0;
        let longitude: f64 = f64::from(point.longitude()) + 180.0;

        let grid_locator_string = format!(
            "{}{}",
            grid_field_string(latitude, longitude),
            grid_square_string(latitude, longitude)
        );
        let grid_locator_string = if precision >= GridPrecision::SubSquare {
            format!(
                "{}{}",
                grid_locator_string,
                grid_sub_square_string(latitude, longitude)
            )
        } else {
            grid_locator_string
        };
        let grid_locator_string = if precision >= GridPrecision::ExtendedSquare {
            format!(
                "{}{}",
                grid_locator_string,
                grid_extended_square_string(latitude, longitude)
            )
        } else {
            grid_locator_string
        };
        let grid_locator_string = if precision >= GridPrecision::ExtendedSubSquare {
            format!(
                "{}{}",
                grid_locator_string,
                grid_extended_sub_square_string(latitude, longitude)
            )
        } else {
            grid_locator_string
        };
        Self(grid_locator_string)
    }

    pub fn to_point(&self) -> Result<Coordinate, GeoError> {
        const LETTER_A: u32 = 'A' as u32;
        const LETTER_0: u32 = '0' as u32;

        fn char_to_value(c: char) -> f64 {
            (c.to_ascii_uppercase() as u32 - LETTER_A) as f64
        }

        fn numeric_char_to_value(c: char) -> f64 {
            (c as u32 - LETTER_0) as f64
        }

        let locator_string = &self.0.to_ascii_uppercase();

        // Add a *central* value for any missing
        let locator_string = match locator_string.len() {
            4 => format!("{}MM00AA", locator_string),
            6 => format!("{}55AA", locator_string),
            8 => format!("{}MM", locator_string),
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
        let longitude: f64 = -180.0
            + (char_to_value(long_chars[0]) * 20.0)
            + (numeric_char_to_value(long_chars[1]) * 2.0)
            + (char_to_value(long_chars[2]) / 12.0)
            + (numeric_char_to_value(long_chars[3]) / 120.0)
            + (char_to_value(long_chars[4]) / 2880.0)
            + 0.000174;

        let lat_chars: Vec<char> = locator_string
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, c)| c)
            .collect();
        let latitude: f64 = -90.0
            + (char_to_value(lat_chars[0]) * 10.0)
            + numeric_char_to_value(lat_chars[1])
            + (char_to_value(lat_chars[2]) / 24.0)
            + (numeric_char_to_value(lat_chars[3]) / 240.0)
            + (char_to_value(lat_chars[4]) / 5760.0)
            + 0.0000868;

        Ok(Coordinate::new(
            Latitude::try_from(latitude)?,
            Longitude::try_from(longitude)?,
        ))
    }

    pub fn precision(&self) -> GridPrecision {
        GridPrecision::try_from(self.0.len()).unwrap()
    }

    pub fn field(&self) -> &str {
        &self.0[0..2]
    }

    pub fn square(&self) -> &str {
        &self.0[0..GridPrecision::Square as usize]
    }

    pub fn sub_square(&self) -> Option<&str> {
        if self.precision() >= GridPrecision::SubSquare {
            Some(&self.0[0..GridPrecision::SubSquare as usize])
        } else {
            None
        }
    }

    pub fn is_extended(&self) -> bool {
        self.precision() > GridPrecision::SubSquare
    }

    pub fn extended_square(&self) -> Option<&str> {
        if self.precision() >= GridPrecision::ExtendedSquare {
            Some(&self.0[0..GridPrecision::ExtendedSquare as usize])
        } else {
            None
        }
    }

    pub fn extended_sub_square(&self) -> Option<&str> {
        if self.precision() >= GridPrecision::ExtendedSubSquare {
            Some(&self.0[0..GridPrecision::ExtendedSubSquare as usize])
        } else {
            None
        }
    }

    pub fn trim_to_precision(&self, precision: GridPrecision) -> Self {
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
                format!("{} ({:?})", self.name, self.vertices())
            } else {
                self.name.to_string()
            }
        )
    }
}

impl FromStr for MaidenheadSquare {
    type Err = GeoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: MaidenheadLocator::from_str(s)?,
            top_left: Default::default(),
            bottom_right: Default::default(),
        })
    }
}

impl GridPolygon for MaidenheadSquare {
    type Identifier = MaidenheadLocator;

    fn id(&self) -> &MaidenheadLocator {
        &self.name
    }

    fn vertices(&self) -> Vec<Coordinate> {
        vec![
            self.top_left,
            Coordinate::new(self.top_left.latitude(), self.bottom_right.longitude()),
            self.bottom_right,
            Coordinate::new(self.bottom_right.latitude(), self.top_left.longitude()),
        ]
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

    fn lookup_id(&self, _id: &MaidenheadLocator) -> Option<Self::Poly> {
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
        FIELD_LETTERS[(longitude / 20.0).floor() as usize],
        FIELD_LETTERS[(latitude / 10.0).floor() as usize],
    )
}

#[inline(always)]
fn grid_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SQUARE_LETTERS[((longitude / 2.0) % 10.0).floor() as usize],
        SQUARE_LETTERS[(latitude % 10.0).floor() as usize],
    )
}

#[inline(always)]
fn grid_sub_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SUB_SQUARE_LETTERS[((longitude * 12.0) % 24.0).floor() as usize],
        SUB_SQUARE_LETTERS[((latitude * 24.0) % 24.0).floor() as usize],
    )
}

#[inline(always)]
fn grid_extended_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SQUARE_LETTERS[((longitude * 120.0) % 10.0).floor() as usize],
        SQUARE_LETTERS[((latitude * 240.0) % 10.0).floor() as usize],
    )
}

#[inline(always)]
fn grid_extended_sub_square_string(latitude: f64, longitude: f64) -> String {
    format!(
        "{}{}",
        SQUARE_LETTERS[((longitude * 2880.0) % 24.0).floor() as usize],
        SQUARE_LETTERS[((latitude * 5760.0) % 24.0).floor() as usize],
    )
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{GridPrecision, MaidenheadLocator};
    use lat_long::{Coordinate, Latitude, Longitude};
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_locator_point_to_string() {
        assert_eq!(
            "CN97hk",
            MaidenheadLocator::from_point_with_precision(
                Coordinate::new(
                    Latitude::try_from(47.421375).unwrap(),
                    Longitude::try_from(-121.410118).unwrap()
                ),
                GridPrecision::SubSquare
            )
            .as_ref()
        );
        assert_eq!(
            "CN97hk01",
            MaidenheadLocator::from_point_with_precision(
                Coordinate::new(
                    Latitude::try_from(47.421375).unwrap(),
                    Longitude::try_from(-121.410118).unwrap()
                ),
                GridPrecision::ExtendedSquare
            )
            .as_ref()
        );
    }

    #[test]
    fn locator_string_to_point() {
        assert_eq!(
            Coordinate::new(
                Latitude::try_from(47.4375868).unwrap(),
                Longitude::try_from(-121.374826).unwrap()
            ),
            MaidenheadLocator::from_str("CN97hk")
                .unwrap()
                .to_point()
                .unwrap()
        )
    }
}
