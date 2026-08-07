use std::fmt::Display;

use crate::{AfGainControl, AntennaSwitching, Frequency, VfoAandB, error::RigError};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Base Traits
// ------------------------------------------------------------------------------------------------

pub trait Entity {}

pub trait Configurable: Entity {
    type Config;

    fn config(&self) -> &Self::Config;
    fn config_mut(&mut self) -> &mut Self::Config;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Rig Traits
// ------------------------------------------------------------------------------------------------

pub trait Rig: Configurable<Config = Self::RigConfig> {
    type RigConfig: RigConfig;

    fn has_af_gain_control(&self) -> bool;
    fn af_gain_control(&self) -> impl AfGainControl;

    fn has_vfo_a_and_b(&self) -> bool;
    fn vfo_a_and_b(&self) -> impl VfoAandB;
    fn current_vfo(&self) -> Result<impl Vfo, RigError>;

    fn has_antenna_switching(&self) -> bool;
    fn antenna_switching(&self) -> impl AntennaSwitching;
}

pub trait RigConfig {
    fn brand_name(&self) -> &str;
    fn model_name(&self) -> &str;
    fn model_version(&self) -> Option<&str>;
    fn serial_number(&self) -> Option<&str>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ VFO Traits
// ------------------------------------------------------------------------------------------------

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
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum VfoLabel {
    VfoA,
    VfoB,
    VfoSub,
}

pub trait Vfo: Configurable<Config = Self::VfoConfig> {
    type VfoConfig: VfoConfig;

    fn label(&self) -> VfoLabel;

    fn frequency(&self) -> Result<(), RigError>;
    fn set_frequency(&mut self, frequency: Frequency) -> Result<(), RigError>;

    fn frequency_up(&mut self) -> Result<(), RigError>;
    fn frequency_down(&mut self) -> Result<(), RigError>;
}

pub trait VfoConfig {
    fn frequency_increment(&self) -> Result<(), RigError>;
    fn set_frequency_increment(&self, increment: Frequency) -> Result<(), RigError>;
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, EnumIs, EnumTryAs,
)]
pub enum OperatingMode {
    Morse(MorseMode),
    AnalogVoice(AnalogVoiceMode),
    DigitalVoice(DigitalVoiceMode),
    Data(DataMode),
    Image(ImageMode),
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
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum MorseMode {
    #[strum(serialize = "CW")]
    #[serde(rename = "CW")]
    ContinuousWave,

    #[strum(serialize = "MCW")]
    #[serde(rename = "MCW")]
    ModulatedContinuousWave,

    #[strum(serialize = "CW-R")]
    #[serde(rename = "CW-R")]
    ContinuousWaveReverse,

    #[strum(serialize = "FSK")]
    #[serde(rename = "FSK")]
    FrequencyShiftKeyed,
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
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum AnalogVoiceMode {
    #[strum(serialize = "AM")]
    #[serde(rename = "AM")]
    AmplitudeModulated,

    #[strum(serialize = "FM")]
    #[serde(rename = "FM")]
    FrequencyModulated,

    #[strum(serialize = "PM")]
    #[serde(rename = "PM")]
    PhaseModulated,

    #[strum(serialize = "WFM")]
    #[serde(rename = "WFM")]
    WidebandFrequencyModulated,

    #[strum(serialize = "LSB")]
    #[serde(rename = "LSB")]
    LowerSideBand,

    #[strum(serialize = "USB")]
    #[serde(rename = "USB")]
    UpperSideBand,
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
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum DigitalVoiceMode {
    #[strum(serialize = "D-STAR")]
    #[serde(rename = "D-STAR")]
    DStar,

    #[strum(serialize = "FreeDV")]
    #[serde(rename = "FreeDV")]
    FreeDV,

    #[strum(serialize = "DMR")]
    #[serde(rename = "DMR")]
    DigitalMobileRadio,

    #[strum(serialize = "Fusion")]
    #[serde(rename = "Fusion")]
    SystemFusion,
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
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum ImageMode {
    #[strum(serialize = "FSTV")]
    #[serde(rename = "FSTV")]
    FastScanTelevision,

    #[strum(serialize = "SSTV")]
    #[serde(rename = "SSTV")]
    SlowScanTelevision,

    #[strum(serialize = "Fax")]
    #[serde(rename = "Fax")]
    Facsimile,
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
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum DataMode {
    #[strum(serialize = "RTTY")]
    #[serde(rename = "RTTY")]
    RadioTeletype,

    #[strum(serialize = "FT8")]
    #[serde(rename = "FT8")]
    FT8,

    #[strum(serialize = "FT4")]
    #[serde(rename = "FT4")]
    FT4,

    Unspecified,
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

impl Display for OperatingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnalogVoice(m) => m.fmt(f),
            Self::Data(m) => m.fmt(f),
            Self::DigitalVoice(m) => m.fmt(f),
            Self::Image(m) => m.fmt(f),
            Self::Morse(m) => m.fmt(f),
        }
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
mod tests {}
