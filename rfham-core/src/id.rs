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

use crate::error::CoreError as Error;
use core::{fmt::Display, str::FromStr};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! name_fn {
    ($vis:vis $fn_name:ident => $name:literal) => {
        #[inline(always)]
        $vis fn $fn_name() -> $crate::id::Name {
            $crate::id::Name::new_unchecked($name)
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Name(String);

pub const RFHAM_URN_PREFIX: &str = "urn:rfham:";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

name_fn!(pub brand_name_baofeng =>  "baofeng");
name_fn!(pub brand_name_chameleon =>  "chameleon");
name_fn!(pub brand_name_elecraft =>  "elecraft");
name_fn!(pub brand_name_gabil =>  "gabil");
name_fn!(pub brand_name_icom =>  "icom");
name_fn!(pub brand_name_kenwood =>  "kenwood");
name_fn!(pub brand_name_yaesu => "yaesu");

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(Error::InvalidValueFromStr(s.to_string(), "Name"))
        }
    }
}

impl Name {
    pub fn new_unchecked<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }

    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_valid(s: &str) -> bool {
        let mut chars = s.chars();
        !s.is_empty()
            && chars.next().unwrap().is_ascii_alphabetic()
            && chars.all(|c| c.is_ascii_alphanumeric() || ['-', '_'].contains(&c))
    }
}
