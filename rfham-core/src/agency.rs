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

use crate::{
    CountryCode,
    country::{country_code_uk, country_code_us},
    error::CoreError,
};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Agency {
    name: String,
    abbreviation: Option<String>,
    kind: AgencyKind,
    jurisdiction: Option<Jurisdiction>,
    url: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum AgencyKind {
    StandardsSetting,
    Regulatory,
    Maintaining,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Jurisdiction {
    International,
    Just(CountryCode),
    All(Vec<CountryCode>),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn agency_itu() -> Agency {
    Agency::new(
        "The International Telecommunication Union",
        AgencyKind::StandardsSetting,
    )
    .with_abbreviation("ITU")
    .with_jurisdiction(Jurisdiction::International)
    .with_url("https://www.itu.int")
}

pub fn agency_iaru() -> Agency {
    Agency::new(
        "The International Amateur Radio Union",
        AgencyKind::Maintaining,
    )
    .with_abbreviation("IARU")
    .with_jurisdiction(Jurisdiction::International)
    .with_url("https://www.iaru.org")
}

pub fn agency_arrl() -> Agency {
    Agency::new("The American Radio Relay League", AgencyKind::Maintaining)
        .with_abbreviation("ARRL")
        .with_jurisdiction(Jurisdiction::International)
        .with_url("http://www.arrl.org")
}

pub fn agency_fcc() -> Agency {
    Agency::new("Federal Communications Commission", AgencyKind::Regulatory)
        .with_abbreviation("FCC")
        .with_jurisdiction(Jurisdiction::Just(country_code_us()))
        .with_url("https://www.fcc.gov")
}

pub fn agency_ofcom() -> Agency {
    Agency::new("Ofcom", AgencyKind::Regulatory)
        .with_jurisdiction(Jurisdiction::Just(country_code_uk()))
        .with_url("https://www.ofcom.org.uk")
}

pub fn agency_rsgb() -> Agency {
    Agency::new("Radio Society of Great Britain", AgencyKind::Maintaining)
        .with_abbreviation("RSGB")
        .with_jurisdiction(Jurisdiction::Just(country_code_uk()))
        .with_url("https://www.rsgb.org")
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Agency
// ------------------------------------------------------------------------------------------------

impl Display for Agency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}{}, {:#}{}",
                self.name,
                if let Some(abbreviation) = self.abbreviation.as_ref() {
                    format!(" ({})", abbreviation)
                } else {
                    String::default()
                },
                self.kind,
                match &self.jurisdiction {
                    Some(Jurisdiction::International) => "international".to_string(),
                    Some(Jurisdiction::Just(cc)) => cc.to_string(),
                    Some(Jurisdiction::All(ccs)) => ccs
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    None => String::default(),
                }
            )
        } else {
            write!(
                f,
                "{}{}",
                self.name,
                if let Some(abbreviation) = self.abbreviation.as_ref() {
                    format!(" ({})", abbreviation)
                } else {
                    String::default()
                }
            )
        }
    }
}

impl Agency {
    pub fn new(name: &str, kind: AgencyKind) -> Self {
        Self {
            name: name.to_string(),
            abbreviation: None,
            kind,
            jurisdiction: None,
            url: None,
        }
    }

    pub fn with_abbreviation(mut self, abbreviation: &str) -> Self {
        self.abbreviation = Some(abbreviation.to_string());
        self
    }

    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn within_jurisdiction(&self, country: &CountryCode) -> Option<bool> {
        self.jurisdiction.as_ref().map(|v| v.contains(country))
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn abbreviation(&self) -> Option<&String> {
        self.abbreviation.as_ref()
    }

    pub fn kind(&self) -> AgencyKind {
        self.kind
    }

    pub fn jurisdiction(&self) -> Option<&Jurisdiction> {
        self.jurisdiction.as_ref()
    }

    pub fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ AgencyKind
// ------------------------------------------------------------------------------------------------

impl Display for AgencyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::StandardsSetting => "Standards-Setting Agency",
                    Self::Regulatory => "Regulatory Agency",
                    Self::Maintaining => "Maintaining Agency",
                }
            } else {
                match self {
                    Self::StandardsSetting => "standards",
                    Self::Regulatory => "regulatory",
                    Self::Maintaining => "maintaining",
                }
            }
        )
    }
}

impl FromStr for AgencyKind {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standards" => Ok(Self::StandardsSetting),
            "regulatory" => Ok(Self::Regulatory),
            "maintaining" => Ok(Self::Maintaining),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "AgencyKind")),
        }
    }
}

impl AgencyKind {
    pub fn is_standards_setting(&self) -> bool {
        matches!(self, Self::StandardsSetting)
    }

    pub fn is_regulatory(&self) -> bool {
        matches!(self, Self::Regulatory)
    }

    pub fn is_maintaining(&self) -> bool {
        matches!(self, Self::Maintaining)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Jurisdiction
// ------------------------------------------------------------------------------------------------

impl Display for Jurisdiction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::International => "international".to_string(),
                Self::Just(cc) => cc.to_string(),
                Self::All(ccs) => ccs
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            }
        )
    }
}

impl FromStr for Jurisdiction {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "international" {
            Ok(Self::International)
        } else if s.contains(',') {
            let list: Result<Vec<CountryCode>, CoreError> =
                s.split(',').map(CountryCode::from_str).collect();
            Ok(Self::All(list?))
        } else {
            Ok(Self::Just(CountryCode::from_str(s)?))
        }
    }
}

impl Jurisdiction {
    pub fn contains(&self, country: &CountryCode) -> bool {
        match self {
            Jurisdiction::International => true,
            Jurisdiction::Just(country_code) => country_code == country,
            Jurisdiction::All(country_codes) => country_codes.contains(country),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------
