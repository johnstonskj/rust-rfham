//! Unit conversion helpers for ham-radio measurements.
//!
//! [`ImperialFoot`] provides a measure of length in feet, can display using prime /
//! double-prime notation (`′` / `″`). The alternate formatter (`{:#}`) renders fractional
//! inches as a rational number where possible (e.g. `8 1/4″` instead of `8.25″`).
//!
//! # Representation
//!
//! The following BNF describes the representation supported by this module, and allows decimal
//! and rational forms as is typically used.
//!
//! ```bnf
//! FootString ::= Decimal | Rational
//!
//! Decimal ::= Sign? Integer ( "." Integer )?
//!
//! Rational ::= Sign? ( RationalFeet RationalInches | RationalFeet | RationalInches )
//!
//! RationalFeet ::= Integer ( "ft" | "′" )?
//!
//! RationalInches ::= Integer ( Integer "/" Integer )? ( "in" | "″" )?
//!
//! Integer ::= [0-9]+
//!
//! Sign ::= "+" | "-"
//! ```
//! | Decimal        | Rational | Labeled Rational              |
//! | -------------- | -------- | ----------------------------- |
//! | `1.0`          | `1`      | `1ft` or `1′`                 |
//! | `1.5`          | `1 6`    | `1ft 6in` or `1′ 6″`          |
//! | `1.5416666667` | `1 6.5`  | `1ft 6 1/2in` or `1′ 6 1/2″`  |
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::non_si::ImperialFoot;
//!
//! let l = ImperialFoot::new(2.6875); // 2 feet 8.25 inches
//! assert_eq!(l.to_string(),    "2′ 8.25″");
//! assert_eq!(format!("{l:#}"), "2′ 8 1/4″");
//! ```

use crate::{Measure, error::CoreError, frequencies::Wavelength};
use num_rational::Rational32;
use num_traits::{
    ConstZero,
    cast::{FromPrimitive, ToPrimitive},
};
use regex::Regex;
use std::{fmt::Display, str::FromStr, sync::LazyLock};
use uom::si::{f64::Length, length::meter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImperialFoot(f64);

pub const THOU_PER_INCH: f64 = 1_000.0;
pub const THOU_PER_FOOT: f64 = INCHES_PER_FOOT * THOU_PER_INCH;
pub const BARLEYCORNS_PER_INCH: f64 = 3.0;
pub const BARLEYCORNS_PER_FOOT: f64 = INCHES_PER_FOOT * BARLEYCORNS_PER_INCH;
pub const HANDS_PER_FOOT: f64 = 3.0;
pub const INCHES_PER_FOOT: f64 = 12.0;

pub const FEET_PER_YARD: f64 = 3.0;
pub const FEET_PER_CHAIN: f64 = FEET_PER_YARD * 22.0;
pub const FEET_PER_FURLONG: f64 = FEET_PER_CHAIN * 10.0;
pub const FEET_PER_MILE: f64 = FEET_PER_FURLONG * 8.0;
pub const FEET_PER_LEAGUE: f64 = FEET_PER_MILE * 3.0;

pub const FEET_PER_FATHOM: f64 = 6.0;
pub const FEET_PER_ADMIRALTY_FATHOM: f64 = 6.08;
pub const FEET_PER_CABLE: f64 = FEET_PER_FATHOM * 100.0;
pub const FEET_PER_NAUTICAL_MILE: f64 = FEET_PER_CABLE * 10.0;

pub const FEET_PER_METER: f64 = 3.280839895;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

static FEET_INCHES_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
    ^(?:
        (?<rational>
            [+-]?
            (?:
                (?<feet>[0-9]+)
                (?:\s*(?:ft|′))?
            )
            (?:
                \s*
                (?<inches>[0-9]+)
                (?<fractional>\s+[0-9]+\/[0-9]+)?
                (?:\s*(?:in|″))?
            )?
        )
        |
        (?<rational_inches>
            [+-]?
            (?:
                \s*
                (?<r_inches>[0-9]+)
                (?<r_fractional>\s+[0-9]+\/[0-9]+)?
                (?:\s*(?:in|″))?
            )?
        )
        |
        (?<decimal>
            [+-]?
            (?<d_feet>[0-9]+(?:\.[0-9]+)?)
            (?:\s*(?:in|)′)?
        )
    )$",
    )
    .unwrap()
});

impl Display for ImperialFoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let feet = self.feet();
        write!(
            f,
            "{}",
            if f.alternate() {
                if let Some((whole, rational)) = self.rational_inches() {
                    let (feet, inches, rational) = if whole >= 12 {
                        (feet.floor() as i64, whole % 12, rational)
                    } else {
                        (feet.floor() as i64, whole, rational)
                    };
                    format!(
                        "{}′{}",
                        feet,
                        match (inches, rational) {
                            (0, z) if z == Rational32::ZERO => String::default(),
                            (inches, z) if z == Rational32::ZERO => format!(" {inches}″"),
                            (0, rational) => format!(" {rational}″"),
                            (inches, rational) => format!(" {inches} {rational}″"),
                        }
                    )
                } else {
                    let inches = feet.fract() * 12.0;
                    let feet = feet.floor() as i64;
                    format!(
                        "{}′{}",
                        feet,
                        if inches != f64::ZERO {
                            format!(" {}″", inches)
                        } else {
                            String::default()
                        }
                    )
                }
            } else {
                let inches = feet.fract() * 12.0;
                let feet = feet.floor() as i64;
                format!(
                    "{}′{}",
                    feet,
                    if inches != f64::ZERO {
                        format!(" {}″", inches)
                    } else {
                        String::default()
                    }
                )
            }
        )
    }
}

impl FromStr for ImperialFoot {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = FEET_INCHES_REGEX.captures(s) {
            if let Some(_) = captures.name("rational") {
                let feet: f64 = captures.name("feet").unwrap().as_str().parse()?;
                let inches: f64 = if let Some(inches) = captures.name("inches") {
                    inches.as_str().parse()?
                } else {
                    f64::ZERO
                };
                let fractional: f64 = if let Some(_) = captures.name("fractional") {
                    let numerator: f64 = captures.name("numerator").unwrap().as_str().parse()?;
                    let denominator: f64 =
                        captures.name("denominator").unwrap().as_str().parse()?;
                    numerator / denominator
                } else {
                    f64::ZERO
                };
                Ok(Self(feet + ((inches + fractional) * INCHES_PER_FOOT)))
            } else if let Some(_) = captures.name("rational_inches") {
                let inches: f64 = if let Some(inches) = captures.name("r_inches") {
                    inches.as_str().parse()?
                } else {
                    f64::ZERO
                };
                let fractional: f64 = if let Some(_) = captures.name("r_fractional") {
                    let numerator: f64 = captures.name("r_numerator").unwrap().as_str().parse()?;
                    let denominator: f64 =
                        captures.name("r_denominator").unwrap().as_str().parse()?;
                    numerator / denominator
                } else {
                    f64::ZERO
                };
                Ok(Self((inches + fractional) * INCHES_PER_FOOT))
            } else if let Some(_) = captures.name("decimal") {
                Ok(Self(captures.name("d_feet").unwrap().as_str().parse()?))
            } else {
                Err(CoreError::InvalidValueFromStr(
                    s.to_string(),
                    "ImperialFoot",
                ))
            }
        } else {
            Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "ImperialFoot",
            ))
        }
    }
}

impl From<Wavelength> for ImperialFoot {
    fn from(value: Wavelength) -> Self {
        Self::from_meters(value.value())
    }
}

impl From<Length> for ImperialFoot {
    fn from(value: Length) -> Self {
        Self::from_meters(value.value)
    }
}

impl From<ImperialFoot> for f64 {
    fn from(value: ImperialFoot) -> Self {
        value.0
    }
}

impl TryFrom<f64> for ImperialFoot {
    type Error = CoreError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(CoreError::InvalidValue(value.to_string(), "ImperialFoot"))
        }
    }
}

impl Measure for ImperialFoot {
    fn value(&self) -> f64 {
        self.0
    }

    fn is_valid(value: f64) -> bool {
        value.is_finite() && !value.is_nan()
    }

    fn is_magnitude() -> bool {
        false
    }
}

impl ImperialFoot {
    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
    // Constructors
    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
    pub const fn new(feet: f64) -> Self {
        Self(feet)
    }

    pub const fn new_feet_and_inches(feet: i64, inches: f64) -> Self {
        Self(feet as f64 + (inches / 12.0))
    }

    pub fn new_rational_feet_and_inches(
        feet: i64,
        inches: u32,
        fractional: Rational32,
    ) -> Option<Self> {
        (fractional + Rational32::from_u32(inches).unwrap())
            .to_f64()
            .map(|inches| Self::new_feet_and_inches(feet, inches))
    }

    pub const fn from_meters(meters: f64) -> Self {
        Self::new(meters * FEET_PER_METER)
    }

    pub fn to_meters(&self) -> Length {
        Length::new::<meter>(self.0 / FEET_PER_METER)
    }

    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
    // Accessors
    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄

    #[inline(always)]
    pub const fn feet(&self) -> f64 {
        self.0
    }

    #[inline(always)]
    pub const fn integer_feet(&self) -> i64 {
        self.0 as i64
    }

    /// Number of inches.
    ///
    /// 1 Inch (in, ″) = 1/12 Foot.
    #[inline(always)]
    pub const fn integer_inches(&self) -> i64 {
        self.inches() as i64
    }

    pub fn rational_inches(&self) -> Option<(u32, Rational32)> {
        let inches = self.inches();
        if let Some(fractional) = Rational32::from_f32(inches.fract() as f32) {
            let uint_inches = inches.floor() as u32;
            Some((uint_inches, fractional))
        } else {
            None
        }
    }

    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
    // Derived Units
    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄

    /// Number of Thou.
    ///
    /// 1 Thou (th) = 1/1000 Inche.
    #[inline(always)]
    pub const fn thou(&self) -> f64 {
        self.inches() * THOU_PER_INCH
    }

    /// Number of Thou.
    ///
    /// 1 Barleycorn = 1/3 Inche.
    #[inline(always)]
    pub const fn barleycorns(&self) -> f64 {
        self.inches() * BARLEYCORNS_PER_INCH
    }

    /// Number of Hands.
    ///
    /// 1 Hand (hh) = 1/3 Foot, or 4 Inches.
    #[inline(always)]
    pub const fn hands(&self) -> f64 {
        self.feet() * HANDS_PER_FOOT
    }

    /// Number of ISO 2848 Basic Modules.
    ///
    /// 1 Basic Module (M) = 100mm (see note).
    ///
    /// It is primarily defined as 100 mm (3.937 inches), with the proviso that in countries
    /// using imperial units it is defined as 4 inches (101.6 mm).
    #[inline(always)]
    pub const fn iso_2848_basic_modules(&self, metric: bool) -> f64 {
        self.inches() / if metric { 3.937007874 } else { 4.0 }
    }

    /// Number of Inches.
    ///
    /// 1 Inch = 1/12 Foot.
    pub const fn inches(&self) -> f64 {
        self.feet() * INCHES_PER_FOOT
    }

    /// Number of Yards.
    ///
    /// 1 Yard (yd) = 3 Feet.
    #[inline(always)]
    pub const fn yards(&self) -> f64 {
        self.feet() / FEET_PER_YARD
    }

    /// Number of Chains.
    ///
    /// 1 Chain (ch) = 22 Yards, or 66 Feet.
    #[inline(always)]
    pub const fn chains(&self) -> f64 {
        self.feet() / FEET_PER_CHAIN
    }

    /// Number of Furlong.
    ///
    /// 1 Furlong (fur) = 10 Chains, or 660 Feet.
    #[inline(always)]
    pub const fn furlongs(&self) -> f64 {
        self.feet() / FEET_PER_FURLONG
    }

    /// Number of Miles.
    ///
    /// 1 Mile (mi) = 8 Furlongs, or 5280 Feet.
    #[inline(always)]
    pub const fn miles(&self) -> f64 {
        self.feet() / FEET_PER_MILE
    }

    /// Number of Leagues.
    ///
    /// 1 League (lea) = 3 Miles, or 15,840 Feet.
    #[inline(always)]
    pub const fn leagues(&self) -> f64 {
        self.feet() / FEET_PER_LEAGUE
    }

    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
    // Derived Maritime Units
    // ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄

    /// Number of Fathoms.
    ///
    /// 1 Fathom (ftm) = 2 Yards, or 6 Feet.
    #[inline(always)]
    pub const fn fathoms(&self) -> f64 {
        self.feet() / FEET_PER_FATHOM
    }

    #[inline(always)]
    pub const fn admiralty_fathoms_only(&self) -> f64 {
        self.feet() / FEET_PER_ADMIRALTY_FATHOM
    }

    /// Number of Cables.
    ///
    /// 1 Cable = 100 Fathoms.
    #[inline(always)]
    pub const fn cables(&self) -> f64 {
        self.feet() / FEET_PER_CABLE
    }

    /// Number of Nautical miles.
    ///
    /// 1 Nautical Mile (nmi) = 10 Cables.
    #[inline(always)]
    pub const fn nautical_miles_only(&self) -> f64 {
        self.feet() / FEET_PER_NAUTICAL_MILE
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::non_si::ImperialFoot;

    #[test]
    fn test_construct_feet() {
        assert_eq!(
            ImperialFoot::new(0.0),
            ImperialFoot::new_feet_and_inches(0, 0.0)
        );
        assert_eq!(ImperialFoot::new(0.0).to_string(), "0′".to_string());
        assert_eq!(format!("{:#}", ImperialFoot::new(0.0)), "0′".to_string());

        assert_eq!(
            ImperialFoot::new(1.5),
            ImperialFoot::new_feet_and_inches(1, 6.0)
        );
        assert_eq!(ImperialFoot::new(1.5).to_string(), "1′ 6″".to_string());
        assert_eq!(format!("{:#}", ImperialFoot::new(1.5)), "1′ 6″".to_string());

        assert_eq!(
            ImperialFoot::new(2.55),
            ImperialFoot::new_feet_and_inches(2, 6.6)
        );
        assert_eq!(
            ImperialFoot::new(2.6875).to_string(),
            "2′ 8.25″".to_string()
        );
        assert_eq!(
            format!("{:#}", ImperialFoot::new(2.6875)),
            "2′ 8 1/4″".to_string()
        );
    }
}
