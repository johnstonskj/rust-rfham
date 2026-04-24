use num_rational::Rational32;
use num_traits::{
    ConstZero,
    cast::{FromPrimitive, ToPrimitive},
};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LengthInFeet {
    feet: u32,
    inches: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Units {
    #[default]
    Metric,
    Imperial,
}

impl Display for LengthInFeet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                if let Some((whole, fractional)) = self.inches_only_fractional() {
                    format!(
                        "{}′{}",
                        self.feet,
                        match (whole, fractional) {
                            (0, z) if z == Rational32::ZERO => String::default(),
                            (whole, z) if z == Rational32::ZERO => format!(" {whole}″"),
                            (0, fractional) => format!(" {fractional}″"),
                            (whole, fractional) => format!(" {whole} {fractional}″"),
                        }
                    )
                } else {
                    format!("{}′ {}″", self.feet, self.inches)
                }
            } else {
                format!(
                    "{}′{}",
                    self.feet,
                    if self.inches != f32::ZERO {
                        format!(" {}″", self.inches)
                    } else {
                        String::default()
                    }
                )
            }
        )
    }
}

impl PartialOrd for LengthInFeet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.feet.partial_cmp(&other.feet) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.inches.partial_cmp(&other.inches)
    }
}

impl LengthInFeet {
    pub fn new(feet: f64) -> Self {
        let uint_feet = feet.floor() as u32;
        let float_inches = feet.fract() as f32;
        println!(
            "u32: {uint_feet}, f32: {float_inches}/{}",
            float_inches * 12.0
        );
        Self::feet_and_inches(uint_feet, float_inches * 12.0)
    }

    pub fn inches(inches: f64) -> Self {
        let feet = (inches / 12.0) as u32;
        let float_inches = inches.rem_euclid(12.0) as f32;
        Self::feet_and_inches(feet, float_inches)
    }

    pub fn feet_and_inches(feet: u32, inches: f32) -> Self {
        Self { feet, inches }
    }
    pub fn feet_and_inches_fractional(
        feet: u32,
        inches: u32,
        fractional: Rational32,
    ) -> Option<Self> {
        (fractional + Rational32::from_u32(inches).unwrap())
            .to_f32()
            .map(|inches| Self::feet_and_inches(feet, inches))
    }

    pub fn feet_only(&self) -> u32 {
        self.feet
    }

    pub fn inches_only_decimal(&self) -> f32 {
        self.inches
    }

    pub fn inches_only_fractional(&self) -> Option<(u32, Rational32)> {
        if let Some(fractional) = Rational32::from_f32(self.inches.fract()) {
            let uint_inches = self.inches.floor() as u32;
            Some((uint_inches, fractional))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::conversions::LengthInFeet;

    #[test]
    fn test_construct_feet() {
        assert_eq!(
            LengthInFeet::new(0.0),
            LengthInFeet {
                feet: 0,
                inches: 0.0
            }
        );
        assert_eq!(LengthInFeet::new(0.0).to_string(), "0′".to_string());
        assert_eq!(format!("{:#}", LengthInFeet::new(0.0)), "0′".to_string());

        assert_eq!(
            LengthInFeet::new(1.5),
            LengthInFeet {
                feet: 1,
                inches: 6.0
            }
        );
        assert_eq!(LengthInFeet::new(1.5).to_string(), "1′ 6″".to_string());
        assert_eq!(format!("{:#}", LengthInFeet::new(1.5)), "1′ 6″".to_string());

        assert_eq!(
            LengthInFeet::new(2.55),
            LengthInFeet {
                feet: 2,
                inches: 6.6000004
            }
        );
        assert_eq!(
            LengthInFeet::new(2.6875).to_string(),
            "2′ 8.25″".to_string()
        );
        assert_eq!(
            format!("{:#}", LengthInFeet::new(2.6875)),
            "2′ 8 1/4″".to_string()
        );
    }
}
