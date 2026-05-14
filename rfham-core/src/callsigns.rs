//! Provides the [`CallSign`] type for parsing and validating amateur radio callsigns.
//!
//! In general an amateur radio callsign is of one of these forms where:
//!
//! * *P* – prefix character (letter or numeral, subject to exclusions below). Prefixes can be
//!   formed using one-letter, two-letters, a digit and a letter, a letter and a digit, or in
//!   rare cases a digit and two letters. There is no ITU allocation of digit-only prefixes.
//!   Letter-digit-letter prefixes are possible but there are no known cases of them being
//!   issued by national bodies.
//! * *N* – a single numeral which separates prefix from suffix (any digit from 0 to 9).
//!   Often a cross-hatched Ø is used for the numeral zero to distinguish it from the letter O.
//! * *S* – suffix character (letter or numeral, last character must be a letter). Digits are
//!   in practise used sparingly in suffixes and almost always for special events. This avoids
//!   confusion with separating numerals and digits in prefixes in regularly issued call signs.
//!
//!   From [Wikipedia](https://en.wikipedia.org/wiki/Amateur_radio_call_signs)
//!
//! ## Ancillary Prefixes and Suffixes
//!
//! Ancillary prefixes or suffixes further identify the location and/or operating condition
//! of an amateur operator.
//!
//! | Suffix | Meaning |
//! |--------|---------|
//! | `/P`   | Portable |
//! | `/M`   | Mobile |
//! | `/AM`  | Aeronautical mobile |
//! | `/MM`  | Maritime mobile |
//! | `/A`   | Alternate location |
//! | `/QRP` | Low-power (≤5 W) operation |
//! | `/AG`, `/AE` | FCC licence pending upgrade |
//!
//! # Unavailability Rules
//!
//!  1. KA2AA-KA9ZZ, KC4AAA-KC4AAF, KC4USA-KC4USZ, KG4AA-KG4ZZ, KC6AA-KC6ZZ, KL9KAA- KL9KHZ,
//!     KX6AA-KX6ZZ;
//!  2. Any call sign having the letters SOS or QRA-QUZ as the suffix;
//!  3. Any call sign having the letters AM-AZ as the prefix (these prefixes are assigned to
//!     other countries by the ITU);
//!  4. Any 2-by-3 format call sign having the letter X as the first letter of the suffix;
//!  5. Any 2-by-3 format call sign having the letters AF, KF, NF, or WF as the prefix and
//!     the letters EMA as the suffix (U.S Government FEMA stations);
//!  6. Any 2-by-3 format call sign having the letters AA-AL as the prefix;
//!  7. Any 2-by-3 format call sign having the letters NA-NZ as the prefix;
//!  8. Any 2-by-3 format call sign having the letters WC, WK, WM, WR, or WT as the prefix
//!     (Group X call signs);
//!  9. Any 2-by-3 format call sign having the letters KP, NP or WP as the prefix and the
//!     numeral 0, 6, 7, 8 or 9;
//! 10. Any 2-by-2 format call sign having the letters KP, NP or WP as the prefix and the
//!     numeral 0, 6, 7, 8 or 9;
//! 11. Any 2-by-1 format call sign having the letters KP, NP or WP as the prefix and the
//!     numeral 0, 6, 7, 8 or 9;
//! 12. Call signs having the single letter prefix (K, N or W), a single digit numeral  
//!     0, 1, 2, 3, 4, 5, 6, 7, 8, 9 and a single letter suffix are reserved for the
//!     special event call sign system.
//!
//! # Examples
//!
//! ```rust
//! use rfham_core::callsigns::CallSign;
//!
//! let cs: CallSign = "K7SKJ".parse().unwrap();
//! assert_eq!(cs.prefix(), "K");
//! assert_eq!(cs.separator_numeral(), 7);
//! assert_eq!(cs.suffix(), "SKJ");
//! assert!(!cs.is_mobile());
//! ```
//!
//! Ancillary qualifiers round-trip through `Display`:
//!
//! ```rust
//! use rfham_core::callsigns::CallSign;
//!
//! let cs: CallSign = "LM9L40Y/P".parse().unwrap();
//! assert!(cs.is_portable());
//! assert_eq!(cs.to_string(), "LM9L40Y/P");
//! ```
//!
//! Invalid callsigns return an error:
//!
//! ```rust
//! use rfham_core::callsigns::CallSign;
//!
//! assert!(CallSign::is_valid("K7SKJ"));
//! assert!(!CallSign::is_valid("NODIGIT"));
//! assert!("NODIGIT".parse::<CallSign>().is_err());
//! ```

use crate::error::CoreError;
use regex::Regex;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, str::FromStr, sync::LazyLock};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The type for ITU-format amateur radio callsigns, with parsing and validation from strings.
///
/// This type stores the components of a callsign separately to allow for easy access to the
/// refix, suffix, and ancillary qualifiers. The `Display` implementation formats the callsign
/// in the standard manner.
///
#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct CallSign {
    ancillary_prefix: Option<String>,
    prefix: String,
    separator: u8,
    suffix: String,
    ancillary_suffix: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Returns `true` if the string `s` is a valid ancillary prefix, i.e. consists of only letters
/// and digits.
///
pub fn ancillary_prefix_is_valid(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

///
/// Returns `true` if the string `s` is a valid prefix, i.e. consists of only letters
/// and digits in specific ordering.
///
pub fn prefix_is_valid(s: &str) -> bool {
    CALLSIGN_PREFIX_REGEX.is_match(s)
}

///
/// Returns `true` if the string `s` is a valid numeric separator, a single ASCII digit.
///
pub fn separator_numeral_is_valid(s: &str) -> bool {
    s.len() == 1 && s.chars().all(|c| c.is_ascii_digit())
}

///
/// Returns `true` if the string `s` is a valid suffix, i.e. consists of only letters
/// and digits.
///
pub fn suffix_is_valid(s: &str) -> bool {
    !s.is_empty() && s.len() <= 10 && s.chars().all(|c| c.is_ascii_alphanumeric())
}

///
/// Returns `true` if the string `s` is a valid suffix, using strict rules, i.e. a
/// shorter length and must end in a letter.
///
pub fn suffix_is_strictly_valid(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 4
        && s.chars().all(|c| c.is_ascii_alphanumeric())
        && s.chars().last().unwrap().is_ascii_alphabetic()
}

///
/// Returns `true` if the string `s` is a valid ancillary suffix, i.e. consists of only letters
/// and digits.
///
pub fn ancillary_suffix_is_valid(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

static CALLSIGN_PREFIX_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[A-Z][0-9][A-Z]?)|(?:[0-9][A-Z]{0,2})|(?:[A-Z]{1,3})$").unwrap()
});

static CALLSIGN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
    ^
        (?:(?<aprefix>[A-Z0-9]+)\/)?
        (?<prefix>(?:[A-Z][0-9][A-Z]?)|(?:[0-9][A-Z]{0,2})|(?:[A-Z]{1,3}))
        (?<sep>[0-9])
        (?<suffix>[A-Z0-9]{1,10})
        (?:\/(?<asuffix>[A-Z0-9]+))?
    $",
    )
    .unwrap()
});

const ODD_CALLSIGN_PREFIXES: &[&str; 16] = &[
    "1A", // is used by the Sovereign Military Order of Malta
    "1B", // is used by the Turkish Republic of Northern Cyprus
    "1C", "1X", // are occasionally used by separatists in the Chechnya
    "1S", // is sometimes used on the Spratly Islands in the South China Sea
    "1Z", // has been used in Kawthoolei, an unrecognized breakaway region of Myanmar
    "D0",
    "1C",  // were used in 2014, allegedly from the unrecognized Donetsk People's Republic
    "S0",  // is a prefix used in the Western Sahara
    "S1A", // is used by the Principality of Sealand
    "T1",  // has appeared as a callsign from Transnistria
    "T0", "0S", "1P",
    "T89", // have occasionally been used by operators in the Principality of Seborga
    "Z6",  // was chosen by the Telecommunications Regulatory Authority of the Republic of Kosovo
];

// ------------------------------------------------------------------------------------------------

impl Display for CallSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            if let Some(ancillary_prefix) = &self.ancillary_prefix {
                format!("{ancillary_prefix}/")
            } else {
                String::default()
            },
            self.prefix,
            self.separator,
            self.suffix,
            if let Some(ancillary_suffix) = &self.ancillary_suffix {
                format!("/{ancillary_suffix}")
            } else {
                String::default()
            },
        )
    }
}

impl FromStr for CallSign {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = CALLSIGN_REGEX.captures(s);
        if let Some(captures) = captures {
            let result = CallSign::new(
                captures.name("prefix").unwrap().as_str(),
                u8::from_str(captures.name("sep").unwrap().as_str())
                    .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "CallSign"))?,
                captures.name("suffix").unwrap().as_str(),
            );
            let result = if let Some(a_prefix) = captures.name("aprefix") {
                result.with_ancillary_prefix(a_prefix.as_str())
            } else {
                result
            };
            let result = if let Some(a_suffix) = captures.name("asuffix") {
                result.with_ancillary_suffix(a_suffix.as_str())
            } else {
                result
            };
            Ok(result)
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "CallSign"))
        }
    }
}

impl CallSign {
    ///
    /// Construct a new callsign from it's constituent parts/components. Validation of suffixes
    /// uses a lax definition of validity by default to allow for special-event and commemorative
    /// callsigns which often have longer suffixes and/or digits in the suffix. For strict
    /// validation of suffixes, use the `new_strict` constructor.
    /// The prefix and suffix are required, but ancillary prefixes and suffixes are optional.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the three components are invalid, e.g. if the
    /// separator numeral is not a single digit from 0 to 9.
    ///
    pub fn new<S1: Into<String>, N: Into<u8>, S2: Into<String>>(
        prefix: S1,
        separator: N,
        suffix: S2,
    ) -> Self {
        Self::new_inner(prefix.into(), separator.into(), suffix.into(), false)
    }

    ///
    /// Construct a new callsign from it's constituent parts/components. Validation of suffixes
    /// follow the ITU stricter rules for regular callsigns, which disallow suffixes longer
    /// than four characters or ending with a digit.
    /// The prefix and suffix are required, but ancillary prefixes and suffixes are optional.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the three components are invalid, e.g. if the
    /// separator numeral is not a single digit from 0 to 9.
    ///
    pub fn new_strict<S1: Into<String>, N: Into<u8>, S2: Into<String>>(
        prefix: S1,
        separator: N,
        suffix: S2,
    ) -> Self {
        Self::new_inner(prefix.into(), separator.into(), suffix.into(), true)
    }

    fn new_inner(prefix: String, separator: u8, suffix: String, strict: bool) -> Self {
        assert!(
            prefix_is_valid(&prefix),
            "Prefix must be 1-3 characters, with at most one digit in the middle"
        );
        assert!(
            separator <= 9,
            "Separator numeral must be a single digit from 0 to 9"
        );
        if strict {
            assert!(
                suffix_is_strictly_valid(&suffix),
                "Suffix must be 1-4 characters, with the last character a letter"
            );
        } else {
            assert!(suffix_is_valid(&suffix), "Suffix must be 1-10 characters");
        }
        Self {
            ancillary_prefix: None,
            prefix,
            separator,
            suffix,
            ancillary_suffix: None,
        }
    }

    ///
    /// Add an ancillary prefix to this callsign, returning a new `CallSign` instance
    /// with the ancillary prefix set. The ancillary prefix is typically used to
    /// indicate a special location.
    ///
    pub fn with_ancillary_prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.ancillary_prefix = Some(prefix.into());
        self
    }

    ///
    /// Add an ancillary suffix to this callsign, returning a new `CallSign` instance
    /// with the ancillary suffix set. The ancillary suffix is typically used to
    /// indicate a special location or operating condition, e.g. `/A` for operation
    /// from an alternate licensed location.
    ///
    pub fn with_ancillary_suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.ancillary_suffix = Some(suffix.into());
        self
    }

    ///
    /// Return a new `CallSign` instance with ancillary prefixes and suffixes removed,
    /// leaving only the core callsign components. This is useful for comparing the core
    /// callsigns.
    pub fn without_ancillaries(&self) -> Self {
        Self {
            ancillary_prefix: None,
            ancillary_suffix: None,
            ..self.clone()
        }
    }

    pub fn ancillary_prefix(&self) -> Option<&String> {
        self.ancillary_prefix.as_ref()
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    pub fn separator_numeral(&self) -> u8 {
        self.separator
    }

    pub fn suffix(&self) -> &String {
        &self.suffix
    }

    pub fn ancillary_suffix(&self) -> Option<&String> {
        self.ancillary_suffix.as_ref()
    }

    /// Returns `true` if `s` matches the ITU callsign pattern.
    pub fn is_valid(s: &str) -> bool {
        CALLSIGN_REGEX.is_match(s)
    }

    /// Returns `true` if this is a special-event or commemorative callsign — i.e. the suffix
    /// is longer than four characters or ends with a digit.
    pub fn is_special(&self) -> bool {
        self.suffix.len() > 4 || self.suffix.chars().last().unwrap().is_ascii_digit()
    }

    /// Returns `true` if the prefix appears in the list of non-standard or
    /// unrecognised-entity prefixes tracked by this library.
    pub fn is_prefix_non_standard(&self) -> bool {
        ODD_CALLSIGN_PREFIXES.contains(&self.prefix.as_str())
    }

    /// Returns `true` when the `/A` ancillary suffix indicates operation from an alternate
    /// licensed location.
    pub fn is_at_alternate_location(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("A"))
            .unwrap_or_default()
    }

    /// Returns `true` when the `/P` ancillary suffix indicates portable operation.
    pub fn is_portable(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("P"))
            .unwrap_or_default()
    }

    /// Returns `true` when the `/M` ancillary suffix indicates mobile operation.
    pub fn is_mobile(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("M"))
            .unwrap_or_default()
    }

    /// Returns `true` when the `/AM` ancillary suffix indicates aeronautical mobile operation.
    pub fn is_aeronautical_mobile(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("AM"))
            .unwrap_or_default()
    }

    /// Returns `true` when the `/MM` ancillary suffix indicates maritime mobile operation.
    pub fn is_maritime_mobile(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("MM"))
            .unwrap_or_default()
    }

    /// Returns `true` when the `/QRP` ancillary suffix indicates the station is operating
    /// at QRP power levels (typically ≤5 W).
    pub fn is_operating_qrp(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("QRP"))
            .unwrap_or_default()
    }

    /// Returns `true` when the `/AG` or `/AE` ancillary suffix indicates a pending FCC
    /// licence upgrade.
    pub fn is_fcc_license_pending(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("AG") || s.eq_ignore_ascii_case("AE"))
            .unwrap_or_default()
    }

    ///
    /// returns `true` only when the core components of the callsign are equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfham_core::callsigns::CallSign;
    /// use std::str::FromStr;
    ///
    /// let callsign1 = CallSign::from_str("K7SKJ/M").unwrap();
    /// let callsign2 = CallSign::from_str("K7SKJ/VE6").unwrap();
    ///
    /// assert!(callsign1.eq_without_ancillaries(&callsign2));
    /// ```
    pub fn eq_without_ancillaries(&self, other: &Self) -> bool {
        self.without_ancillaries() == other.without_ancillaries()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::callsigns::CallSign;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    const VALID: &[&str] = &[
        "3DA0RS",
        "4D71/N0NM",
        "4X130RISHON",
        "4X4AAA",
        "9N38",
        "A22A",
        "AX3GAMES",
        "B2AA",
        "BV100",
        "DA2MORSE",
        "DB50FIRAC",
        "DL50FRANCE",
        "FBC5AGB",
        "FBC5CWU",
        "FBC5LMJ",
        "FBC5NOD",
        "FBC5YJ",
        "FBC6HQP",
        "GB50RSARS",
        "HA80MRASZ",
        "HB9STEVE",
        "HG5FIRAC",
        "HG80MRASZ",
        "HL1AA",
        "I2OOOOX",
        "II050SCOUT",
        "IP1METEO",
        "J42004A",
        "J42004Q",
        "K4X",
        "LM1814",
        "LM2T70Y",
        "LM9L40Y",
        "LM9L40Y/P",
        "M0A",
        "N2ASD",
        "OEM2BZL",
        "OEM3SGU",
        "OEM3SGU/3",
        "OEM6CLD",
        "OEM8CIQ",
        "OM2011GOOOLY",
        "ON1000NOTGER",
        "ON70REDSTAR",
        "PA09SHAPE",
        "PA65VERON",
        "PA90CORUS",
        "PG50RNARS",
        "PG540BUFFALO",
        "S55CERKNO",
        "TM380",
        // How is this valid => "TX9",
        "TYA11",
        "U5ARTEK/A",
        "V6T1",
        "VB3Q70",
        "VI2AJ2010",
        "VI2FG30",
        "VI4WIP50",
        "VU3DJQF1",
        "VX31763",
        // How is this valid => "WD4",
        "XUF2B",
        "YI9B4E",
        "YO1000LEANY",
        "ZL4RUGBY",
        "ZS9MADIBA",
        "C6AFO",   // Bahamian
        "C6AGB",   // Bahamian
        "VE9COAL", // Canadian commemorative event
    ];

    #[test]
    fn test_callsign_components() {
        let callsign = CallSign::from_str("K7SKJ/M").unwrap();
        assert_eq!(None, callsign.ancillary_prefix());
        assert_eq!("K", callsign.prefix().as_str());
        assert_eq!(7, callsign.separator_numeral());
        assert_eq!("SKJ", callsign.suffix().as_str());
        assert_eq!(Some("M"), callsign.ancillary_suffix().map(|s| s.as_str()));
        assert!(!callsign.is_special());
    }

    #[test]
    fn test_callsign_mobile_qualifiers() {
        assert!("K7SKJ/M".parse::<CallSign>().unwrap().is_mobile());
        assert!("K7SKJ/P".parse::<CallSign>().unwrap().is_portable());
        assert!(
            "K7SKJ/AM"
                .parse::<CallSign>()
                .unwrap()
                .is_aeronautical_mobile()
        );
        assert!("K7SKJ/MM".parse::<CallSign>().unwrap().is_maritime_mobile());
        assert!(
            "K7SKJ/A"
                .parse::<CallSign>()
                .unwrap()
                .is_at_alternate_location()
        );
        assert!("K7SKJ/QRP".parse::<CallSign>().unwrap().is_operating_qrp());
    }

    #[test]
    fn test_callsign_fcc_pending() {
        assert!(
            "K7SKJ/AG"
                .parse::<CallSign>()
                .unwrap()
                .is_fcc_license_pending()
        );
        assert!(
            "K7SKJ/AE"
                .parse::<CallSign>()
                .unwrap()
                .is_fcc_license_pending()
        );
        assert!(
            !"K7SKJ/P"
                .parse::<CallSign>()
                .unwrap()
                .is_fcc_license_pending()
        );
    }

    #[test]
    fn test_callsign_special() {
        assert!("GB50RSARS".parse::<CallSign>().unwrap().is_special()); // long suffix
        assert!(!"K7SKJ".parse::<CallSign>().unwrap().is_special()); // normal suffix
    }

    #[test]
    fn test_callsign_no_qualifier_flags_false() {
        let cs: CallSign = "K7SKJ".parse().unwrap();
        assert!(!cs.is_mobile());
        assert!(!cs.is_portable());
        assert!(!cs.is_aeronautical_mobile());
        assert!(!cs.is_maritime_mobile());
        assert!(!cs.is_at_alternate_location());
        assert!(!cs.is_operating_qrp());
        assert!(!cs.is_fcc_license_pending());
    }

    #[test]
    fn test_invalid_callsigns() {
        assert!(!CallSign::is_valid("NODIGIT")); // no separator digit
        assert!(!CallSign::is_valid("")); // empty
        assert!(!CallSign::is_valid("K7SK!")); // invalid character
        assert!("NODIGIT".parse::<CallSign>().is_err());
    }

    #[test]
    fn test_callsign_display_roundtrip() {
        for s in VALID {
            assert_eq!(s.to_string(), CallSign::from_str(s).unwrap().to_string());
        }
    }
}
