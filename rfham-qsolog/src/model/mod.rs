//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!

use crate::error::LogError;
use rfham_core::{callsigns::CallSign, frequencies::Frequency};
use rfham_maidenhead::MaidenheadLocator;
use chrono::NaiveDateTime;
use strum::{EnumIs,EnumIter,FromRepr};
use core::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Log {
    type EntryId;
    type Entry: LogEntry<Id = Self::EntryId>;


    fn owner(&self) -> &<Self::Entry as LogEntry>::Contact;
    fn label(&self) -> Option<&String>;
    fn created(&self) -> &NaiveDateTime;

    fn get(&self, entry_id: Self::EntryId) -> Option<&Self::Entry>;
    fn entries(&self) -> impl Iterator<Item = &Self::Entry>;

    fn create(&mut self, entry: Self::Entry) -> Result<Self::EntryId, LogError>;
    fn update(&mut self, entry: Self::Entry) -> Result<Self::EntryId, LogError>;
    fn remove(&mut self, entry: Self::EntryId) -> Result<Self::EntryId, LogError>;
}

pub trait LogEntry {
    type Id;
    type Contact: Contact;

    fn id(&self) -> Option<&Self::Id>;
    fn start_time(&self) -> &NaiveDateTime;
    fn end_time(&self) -> &NaiveDateTime; 

    fn contact(&self) -> &Self::Contact;
    fn contact_rst(&self) -> SignalReport;

    fn frequency(&self) -> &Frequency;
    fn mode(&self) -> &String;
    
    fn reported_rst(&self) -> SignalReport;
}

pub trait Contact {
    type Address: Address;

    fn callsign(&self) -> &CallSign;
    fn club_callsign(&self) -> Option<&CallSign>;
    fn name(&self) -> Option<&str>;
    fn grid(&self) -> Option<&MaidenheadLocator>;
    fn address(&self) -> Option<&Self::Address>;
}

pub trait Address {
    fn house_number(&self) -> Option<&str>;
    fn street(&self) -> Option<&str>;
    fn street_line_2(&self) -> Option<&str>;
    fn apartment_number(&self) -> Option<&str>;
    fn city(&self) -> Option<&str>;
    fn state_or_province(&self) -> Option<&str>;
    fn postal_code(&self) -> Option<&str>;
    fn country(&self) -> &str;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SignalReport {
    readability: Readability,
    signal_strength: SignalStrength,
    db_over_s9: Option<u8>,
    tone: Option<Tone>,
    signal_distorted_by_auroral_propagation: bool,
    frequency_shift_at_key_on_off: bool,
    frequency_drift: bool,
    key_clicks: bool,
    signal_distorted_by_multipath_propagation: bool,
    signal_distorted_by_scatter_propagation: bool,
    exceptionally_stable_frequency: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIs, EnumIter, FromRepr)]
#[repr(u8)]
pub enum Readability {
    /// Completely unreadable.
    Unreadable = 1,
    /// Barely readable, occasional words distinguishable.
    BarelyReadable = 2,
    /// Readable with considerable difficulty.
    ReadableWithDifficulty = 3,
    /// Readable with practically no difficulty.
    PracticallyReadable = 4,
    /// Perfectly readable
    PerfectlyReadable = 5
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIs, EnumIter, FromRepr)]
#[repr(u8)]
pub enum SignalStrength {
    /// Faint, signal is barely perceptible
    Faint = 1,
    VeryWeak = 2,
    Weak = 3,
    Fair = 4,
    FairlyGood = 5,
    Good = 6,
    ModeratelyStrong = 7,
    Strong = 8,
    ExtremelyStrong = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIs, EnumIter, FromRepr)]
#[repr(u8)]
pub enum Tone {
    /// 60 Hz AC or less, very rough and broad
    ExtremelyRough = 1,
    /// 60 Hz AC or less, very rough and broad
    VeryRough = 2,
    /// Rough AC tone, rectified but not filtered
    Rough = 3,
    /// Rough note, some trace of filtering
    RatherRough = 4,
    /// Filtered rectified AC but strongly ripple-modulated
    MusicallyModulated = 5,
    /// Filtered tone, definite trace of ripple modulation
    Modulated = 6,
    /// Near pure tone, trace of ripple modulation
    NearPure = 7,
    /// Near perfect tone, slight trace of modulation
    NearPerfect = 8,
    /// Perfect tone, no trace of ripple or modulation of any kind
    Perfect = 9,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SignalReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.readability as u8, self.signal_strength as u8)?;
        if let Some(db_over_s9) = self.db_over_s9 {
            write!(f, "(+{db_over_s9})")?;
        }
        if let Some(tone) = self.tone {
            write!(f, "{}", tone as u8)?;
        }
        if self.signal_distorted_by_auroral_propagation { 
            write!(f, "A")?; 
        }
        if self.frequency_shift_at_key_on_off { 
            write!(f, "C")?;
        }
        if self.frequency_drift { 
            write!(f, "D")?;
        }
        if self.key_clicks { 
            write!(f, "K")?;
        }
        if self.signal_distorted_by_multipath_propagation { 
            write!(f, "M")?;
        }
        if self.signal_distorted_by_scatter_propagation { 
            write!(f, "S")?;
        }
        if self.exceptionally_stable_frequency { 
            write!(f, "X")?;
        }
        Ok(())
    }
}

impl SignalReport {
    pub fn new(
        readability: Readability,
        signal_strength: SignalStrength,
    ) -> Self {
        Self {
            readability,
            signal_strength,
            db_over_s9: None,
            tone: None,
            signal_distorted_by_auroral_propagation: false,
            frequency_shift_at_key_on_off: false,
            frequency_drift: false,
            key_clicks: false,
            signal_distorted_by_multipath_propagation: false,
            signal_distorted_by_scatter_propagation: false,
            exceptionally_stable_frequency: false,
        }
    }    
    
    pub fn new_cw(
        readability: Readability,
        signal_strength: SignalStrength,
        tone: Tone
    ) -> Self {
        Self {
            readability,
            signal_strength,
            db_over_s9: None,
            tone: Some(tone),
            signal_distorted_by_auroral_propagation: false,
            frequency_shift_at_key_on_off: false,
            frequency_drift: false,
            key_clicks: false,
            signal_distorted_by_multipath_propagation: false,
            signal_distorted_by_scatter_propagation: false,
            exceptionally_stable_frequency: false,
        }
    }

    pub fn with_db_over_s9(mut self, db_over_s9: u8) -> Self {
        self.db_over_s9 = Some(db_over_s9);
        self
    }

    pub fn with_signal_distorted_by_auroral_propagation(mut self) -> Self {
        self.signal_distorted_by_auroral_propagation = true;
        self
    }

    pub fn with_frequency_shift_at_key_on_off(mut self) -> Self {
        self.frequency_shift_at_key_on_off = true;
        self
    }

    pub fn with_frequency_drift(mut self) -> Self {
        self.frequency_drift = true;
        self
    }

    pub fn with_key_clicks(mut self) -> Self {
        self.key_clicks = true;
        self
    }

    pub fn with_signal_distorted_by_multipath_propagation(mut self) -> Self {
        self.signal_distorted_by_multipath_propagation = true;
        self
    }

    pub fn with_signal_distorted_by_scatter_propagation(mut self) -> Self {
        self.signal_distorted_by_scatter_propagation = true;
        self
    }

    pub fn with_exceptionally_stable_frequency(mut self) -> Self {
        self.exceptionally_stable_frequency = true;
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_signal_report_display_simple() {
        assert_eq!(
            "59",
            SignalReport::new(
                Readability::PerfectlyReadable, 
                SignalStrength::ExtremelyStrong,
            ).to_string().as_str()
        )
    }

    #[test]
    fn test_signal_report_display_simple_with_db() {
        assert_eq!(
            "59(+10)",
            SignalReport::new(
                Readability::PerfectlyReadable, 
                SignalStrength::ExtremelyStrong,
            )
            .with_db_over_s9(10)
            .to_string()
            .as_str()
        )
    }

    #[test]
    fn test_signal_report_display_cw() {
        assert_eq!(
            "599",
            SignalReport::new_cw(
                Readability::PerfectlyReadable, 
                SignalStrength::ExtremelyStrong,
                Tone::Perfect
            ).to_string().as_str()
        )
    }

    #[test]
    fn test_signal_report_display_cw_with_clicks() {
        assert_eq!(
            "599K",
            SignalReport::new_cw(
                Readability::PerfectlyReadable, 
                SignalStrength::ExtremelyStrong,
                Tone::Perfect
            )
            .with_key_clicks()
            .to_string()
            .as_str()
        )
    }
}
