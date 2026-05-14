//! Validated string identifier types and the [`StringLike`] trait.
//!
//! Three newtype wrappers enforce invariants on string values used as identifiers:
//!
//! | Type | Max length | Allowed content |
//! |------|-----------|-----------------|
//! | [`Name`] | 32 | Starts with ASCII letter; then `[a-zA-Z0-9_-]`; normalised to lowercase on parse |
//! | [`Label`] | 48 | Any string under the limit (human-readable labels) |
//! | [`Tag`] | 24 | Non-empty; no whitespace |
//!
//! The `name_fn` macro generates zero-cost `fn` accessors that return a typed [`Name`]
//! literal, avoiding repeated `new_unchecked` calls at call sites.
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::{names::Name, StringLike};
//! use std::str::FromStr;
//!
//! let n: Name = Name::from_str("Yaesu").unwrap();
//! assert_eq!(n.as_str(), "yaesu"); // normalised to lowercase
//! assert!(Name::is_valid("elecraft"));
//! assert!(!Name::is_valid("9bad"));   // must start with a letter
//! assert!(!Name::is_valid(""));
//! ```
//!
//! Pre-defined brand-name accessors:
//!
//! ```rust
//! use rfham_core::{names::{brand_name_icom, brand_name_yaesu}, StringLike};
//!
//! assert_eq!(brand_name_icom().as_str(), "icom");
//! assert_eq!(brand_name_yaesu().as_str(), "yaesu");
//! ```

use crate::{StringLike, error::CoreError};
use core::{fmt::Display, hash::Hash, str::FromStr};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! name_fn {
    ($vis:vis $fn_name:ident => $name:literal) => {
        #[inline(always)]
        $vis fn $fn_name() -> $crate::names::Name {
            $crate::names::Name::new_unchecked($name)
        }
    };
}
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Name(String);

pub trait Named {
    fn name(&self) -> &Name;
    fn set_name(&mut self, name: Name);
}

pub trait MaybeNamed {
    fn with_name(self, name: Name) -> Self;

    fn name(&self) -> Option<&Name>;
    fn set_name(&mut self, name: Name);
    fn unset_name(&mut self);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Label(String);

pub trait Labeled {
    fn label(&self) -> &Label;
    fn set_set_label(&mut self, name: Label);
}

pub trait MaybeLabeled {
    fn with_label(self, label: Label) -> Self;

    fn label(&self) -> Option<&Label>;
    fn set_label(&mut self, label: Label);
    fn unset_label(&mut self);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Tag(String);

pub trait Tagged {
    fn with_tags(self, tags: impl IntoIterator<Item = Tag>) -> Self;
    fn tags(&self) -> impl Iterator<Item = &Tag>;
    fn extend_tags(&mut self, tags: impl IntoIterator<Item = Tag>);
    fn has_tag(&self, tag: &Tag) -> bool;
    fn add_tag(&mut self, tag: Tag);
    fn remove_tag(&mut self, tag: &Tag);
    fn clear_tags(&mut self);
}

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
// Implementations ❯ Name
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
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "Name"))
        }
    }
}

impl StringLike for Name {
    const MAX_LENGTH: usize = 32;
    fn new_unchecked<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn is_valid(s: &str) -> bool {
        let mut chars = s.chars();
        !s.is_empty()
            && s.len() < Self::MAX_LENGTH
            && chars.next().unwrap().is_ascii_alphabetic()
            && chars.all(|c| c.is_ascii_alphanumeric() || ['-', '_'].contains(&c))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ DisplayName
// ------------------------------------------------------------------------------------------------

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Label> for String {
    fn from(value: Label) -> Self {
        value.0
    }
}

impl AsRef<str> for Label {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for Label {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "DisplayName"))
        }
    }
}

impl StringLike for Label {
    const MAX_LENGTH: usize = 48;

    fn new_unchecked<S: Into<String>>(display_name: S) -> Self {
        Self(display_name.into())
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn is_valid(s: &str) -> bool {
        s.len() < Self::MAX_LENGTH
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Tag
// ------------------------------------------------------------------------------------------------

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Tag> for String {
    fn from(value: Tag) -> Self {
        value.0
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for Tag {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "Tag"))
        }
    }
}

impl StringLike for Tag {
    const MAX_LENGTH: usize = 24;

    fn new_unchecked<S: Into<String>>(tag: S) -> Self {
        Self(tag.into())
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty() && s.len() < Self::MAX_LENGTH && s.chars().all(|c| !c.is_whitespace())
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{Label, Name, Tag};
    use crate::names::StringLike;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn name_valid_inputs() {
        assert!(Name::is_valid("abc"));
        assert!(Name::is_valid("a-b_c123"));
        assert!(Name::is_valid("a")); // single char
        assert!(Name::is_valid("elecraft"));
    }

    #[test]
    fn name_invalid_inputs() {
        assert!(!Name::is_valid("")); // empty
        assert!(!Name::is_valid("1abc")); // starts with digit
        assert!(!Name::is_valid("-abc")); // starts with dash
        assert!(!Name::is_valid(&"a".repeat(32))); // at or beyond max length
    }

    #[test]
    fn name_from_str_lowercases() {
        let n = Name::from_str("Yaesu").unwrap();
        assert_eq!(n.as_str(), "yaesu");
        let n = Name::from_str("ICOM").unwrap();
        assert_eq!(n.as_str(), "icom");
    }

    #[test]
    fn name_from_str_invalid_returns_error() {
        assert!("9bad".parse::<Name>().is_err());
        assert!("".parse::<Name>().is_err());
    }

    #[test]
    fn display_name_valid() {
        assert!(Label::is_valid("Yaesu FT-991A"));
        assert!(Label::is_valid("")); // empty is valid for DisplayName
    }

    #[test]
    fn display_name_too_long_is_invalid() {
        assert!(!Label::is_valid(&"x".repeat(48)));
    }

    #[test]
    fn tag_valid_no_whitespace() {
        assert!(Tag::is_valid("contest"));
        assert!(Tag::is_valid("dx"));
        assert!(Tag::is_valid("sota-activation"));
    }

    #[test]
    fn tag_invalid_inputs() {
        assert!(!Tag::is_valid("")); // empty
        assert!(!Tag::is_valid("has space")); // contains whitespace
        assert!(!Tag::is_valid("tab\there")); // tab counts as whitespace
        assert!(!Tag::is_valid(&"x".repeat(24))); // at max length
    }
}
