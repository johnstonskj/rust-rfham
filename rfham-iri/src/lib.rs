//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use rfham_core::Name;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use strum::{AsRefStr, Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! rfham {
    ($kind:ident / $brand:ident / $model:ident) => {
        $crate::UniversalRigName::$kind(
            <::rfham_core::names::Name as ::rfham_core::StringLike>::new_unchecked(stringify!(
                $brand
            )),
            <::rfham_core::names::Name as ::rfham_core::StringLike>::new_unchecked(stringify!(
                $model
            )),
        )
    };
    ($kind:ident / $brand:literal / $model:ident) => {
        $crate::UniversalRigName::$kind(
            <::rfham_core::names::Name as ::std::str::FromStr>::from_str($brand).unwrap(),
            <::rfham_core::names::Name as ::rfham_core::StringLike>::new_unchecked(stringify!(
                $model
            )),
        )
    };
    ($kind:ident / $brand:ident / $model:literal) => {
        $crate::UniversalRigName::$kind(
            <::rfham_core::names::Name as ::rfham_core::StringLike>::new_unchecked(stringify!(
                $brand
            )),
            <::rfham_core::names::Name as ::std::str::FromStr>::from_str($model).unwrap(),
        )
    };
    ($kind:ident / $brand:literal / $model:literal) => {
        $crate::UniversalRigName::$kind(
            <::rfham_core::names::Name as ::std::str::FromStr>::from_str($brand).unwrap(),
            <::rfham_core::names::Name as ::std::str::FromStr>::from_str($model).unwrap(),
        )
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const RFHAM_URN_SCHEME: &str = "rfham";

///
///
/// ```bnf
/// Urn         ::= Scheme SubScheme Brand Model Version? More?
/// Scheme      ::= 'rfham'
/// SubScheme   ::= ':' ( 'amp' | 'ant' | 'rig' | 'tuner' )
/// Brand       ::= '/' Name
/// Model       ::= '/' Name
/// Version     ::= '/' UrlEncodedString
/// More        ::= '#' UrlEncodedString
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct UniversalRigName {
    kind: Kind,
    brand: Name,
    model: Name,
    version: Option<String>,
    more: Option<String>,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    EnumIs,
    EnumDisplay,
    AsRefStr,
    EnumString,
    EnumIter,
)]
pub enum Kind {
    ///
    /// An RF power amplifier. This may be internal to a transceiver, or external.
    ///
    #[serde(rename = "amp")]
    #[strum(serialize = "amp")]
    Amplifier,

    ///
    /// An antenna; in general this is a description of the antenna's characteristics.
    ///
    #[serde(rename = "ant")]
    #[strum(serialize = "ant")]
    Antenna,

    ///
    /// An S-meter, SWR meter, power meter, or other measurement device.
    /// This may be internal to a transceiver, or external.
    ///
    #[serde(rename = "meter")]
    #[strum(serialize = "meter")]
    Meter,

    ///
    /// A panadapter, which is a device that provides a visual representation of the RF spectrum.
    /// This may be internal to a transceiver, or external.
    ///
    #[serde(rename = "pan")]
    #[strum(serialize = "pan")]
    PanAdapter,

    ///
    /// A receiver, or transceiver, which is a device that can both transmit and receive radio signals.
    ///
    #[serde(rename = "rig")]
    #[strum(serialize = "rig")]
    Rig,

    ///
    /// An antenna rotator, which is a device that can rotate an antenna to point in different directions.
    ///
    #[serde(rename = "rotator")]
    #[strum(serialize = "rotator")]
    Rotator,

    ///
    /// An antenna tuner, which is a device that matches the impedance of the antenna to the transmitter.
    /// This may be internal to a transceiver, or external.
    ///
    #[serde(rename = "tuner")]
    #[strum(serialize = "tuner")]
    Tuner,
}

#[derive(Debug, Error)]
pub enum UrnError {
    #[error("Invalid scheme value `{0}` expecting `{RFHAM_URN_SCHEME}`")]
    InvalidScheme(String),

    #[error(
        "Invalid sub-scheme value `{0}` expecting one of: `amp`, `ant`, `meter`, `pan`, `rig`, `rotator`, or `tuner`"
    )]
    InvalidSubScheme(String),

    #[error("Urn expecdts only scheme and sub-scheme")]
    TooManySchemes,

    #[error("Could not parse `{0}` as a Name value for brand")]
    InvalidBrandName(String),

    #[error("Could not parse `{0}` as a Name value for model")]
    InvalidModelName(String),

    #[error("Scheme expects 3..=4 components, given {0}")]
    InvalidComponentCount(usize),

    #[error("Scheme expects only 0..=1 fragment values")]
    TooManyFragments,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const URN_SUPER_SCHEME: &str = "urn";

impl Display for UniversalRigName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{RFHAM_URN_SCHEME}:{}/{}/{}{}{}",
            self.kind.as_ref().to_ascii_lowercase(),
            self.brand.as_ref().to_lowercase(),
            self.model.as_ref().to_lowercase(),
            if let Some(version) = &self.version {
                format!("/{}", version.to_lowercase())
            } else {
                String::new()
            },
            if let Some(more) = &self.more {
                format!("#{more}")
            } else {
                String::new()
            }
        )
        .fmt(f)
    }
}

impl FromStr for UniversalRigName {
    type Err = UrnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (urn, more) = if let Some((urn, more)) = s.rsplit_once('#') {
            (urn, Some(more.to_string()))
        } else {
            (s, None)
        };
        if urn.contains('#') {
            return Err(UrnError::TooManyFragments);
        }

        let parts = urn.split(':').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(UrnError::TooManySchemes);
        }

        if parts[0] != RFHAM_URN_SCHEME {
            Err(UrnError::InvalidScheme(parts[0].to_string()))
        } else {
            let parts = parts[1].split('/').collect::<Vec<_>>();
            if !(3..=4).contains(&parts.len()) {
                return Err(UrnError::InvalidComponentCount(parts.len()));
            }

            Ok(Self {
                kind: Kind::from_str(parts[0])
                    .map_err(|_| UrnError::InvalidSubScheme(parts[0].to_string()))?,
                model: Name::from_str(parts[1])
                    .map_err(|_| UrnError::InvalidBrandName(parts[1].to_string()))?,
                brand: Name::from_str(parts[2])
                    .map_err(|_| UrnError::InvalidModelName(parts[2].to_string()))?,
                version: if parts.len() == 4 {
                    Some(parts[3].to_string())
                } else {
                    None
                },
                more: more,
            })
        }
    }
}

impl UniversalRigName {
    pub fn new(kind: Kind, brand: Name, model: Name) -> Self {
        Self {
            kind,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn amplifier(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::Amplifier,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn antenna(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::Antenna,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn meter(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::Meter,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn pan_adapter(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::PanAdapter,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn rig(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::Rig,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn rotator(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::Rotator,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn tuner(brand: Name, model: Name) -> Self {
        Self {
            kind: Kind::Tuner,
            brand,
            model,
            version: None,
            more: None,
        }
    }

    pub fn with_version<S>(mut self, version: S) -> Self
    where
        S: Into<String>,
    {
        self.version = Some(version.into());
        self
    }

    pub fn with_more<S>(mut self, more: S) -> Self
    where
        S: Into<String>,
    {
        self.more = Some(more.into());
        self
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn brand(&self) -> &Name {
        &self.brand
    }

    pub fn model(&self) -> &Name {
        &self.model
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn more(&self) -> Option<&String> {
        self.more.as_ref()
    }

    pub fn is_amplifier(&self) -> bool {
        matches!(self.kind, Kind::Amplifier)
    }

    pub fn is_antenna(&self) -> bool {
        matches!(self.kind, Kind::Antenna)
    }

    pub fn is_rig(&self) -> bool {
        matches!(self.kind, Kind::Rig)
    }

    pub fn is_tuner(&self) -> bool {
        matches!(self.kind, Kind::Tuner)
    }

    pub fn to_urn_string(&self) -> String {
        format!("{URN_SUPER_SCHEME}:{self}")
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    #[test]
    fn test_macro() {
        assert_eq!(
            "rfham:rig/icom/ic_705",
            &rfham!(rig / Icom / ic_705).to_string()
        );

        assert_eq!(
            "rfham:rig/icom/ic_7300",
            &rfham!(rig / "Icom" / ic_7300).to_string()
        );

        assert_eq!(
            "rfham:rig/icom/ic-705",
            &rfham!(rig / Icom / "ic-705").to_string()
        );

        assert_eq!(
            "rfham:rig/icom/ic-7300",
            &rfham!(rig / "Icom" / "ic-7300").to_string()
        );
    }
}
