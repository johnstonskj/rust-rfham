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

/// In general an amateur radio callsign is of one of these forms where:
///
/// * *P* – prefix character (letter or numeral, subject to exclusions below). Prefixes can be
///   formed using one-letter, two-letters, a digit and a letter, a letter and a digit, or in
///   rare cases a digit and two letters. There is no ITU allocation of digit-only prefixes.
///   Letter-digit-letter prefixes are possible but there are no known cases of them being
///   issued by national bodies.
/// * *N* – a single numeral which separates prefix from suffix (any digit from 0 to 9).
///   Often a cross-hatched Ø is used for the numeral zero to distinguish it from the letter O.
/// * *S* – suffix character (letter or numeral, last character must be a letter). Digits are
///   in practise used sparingly in suffixes and almost always for special events. This avoids
///   confusion with separating numerals and digits in prefixes in regularly issued call signs.
///
///   From [Wikipedia](https://en.wikipedia.org/wiki/Amateur_radio_call_signs)
#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct CallSign {
    ancillary_prefix: Option<String>,
    prefix: String,
    separator: u8,
    suffix: String,
    ancillary_suffix: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
    pub fn new<S1: Into<String>, N: Into<u8>, S2: Into<String>>(
        prefix: S1,
        separator: N,
        suffix: S2,
    ) -> Self {
        Self {
            ancillary_prefix: None,
            prefix: prefix.into(),
            separator: separator.into(),
            suffix: suffix.into(),
            ancillary_suffix: None,
        }
    }

    pub fn with_ancillary_prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.ancillary_prefix = Some(prefix.into());
        self
    }

    pub fn with_ancillary_suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.ancillary_suffix = Some(suffix.into());
        self
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

    pub fn is_valid(s: &str) -> bool {
        CALLSIGN_REGEX.is_match(s)
    }

    pub fn is_special(&self) -> bool {
        self.suffix.len() > 4 || self.suffix.chars().last().unwrap().is_ascii_digit()
    }

    pub fn is_prefix_non_standard(&self) -> bool {
        ODD_CALLSIGN_PREFIXES.contains(&self.prefix.as_str())
    }

    pub fn is_at_alternate_location(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("A"))
            .unwrap_or_default()
    }

    pub fn is_portable(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("P"))
            .unwrap_or_default()
    }

    pub fn is_mobile(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("M"))
            .unwrap_or_default()
    }

    pub fn is_aeronautical_mobile(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("AM"))
            .unwrap_or_default()
    }

    pub fn is_maritime_mobile(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("MM"))
            .unwrap_or_default()
    }

    pub fn is_operating_qrp(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("QRP"))
            .unwrap_or_default()
    }

    pub fn is_fcc_license_pending(&self) -> bool {
        self.ancillary_suffix()
            .map(|s| s.eq_ignore_ascii_case("AG") || s.eq_ignore_ascii_case("AE"))
            .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::callsign::CallSign;
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
    fn test_callsign_validity() {
        for s in VALID {
            assert_eq!(s.to_string(), CallSign::from_str(s).unwrap().to_string());
        }
    }

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
}
