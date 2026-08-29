use core::fmt::Display;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

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

    #[strum(serialize = "SSB")]
    #[serde(rename = "SSB")]
    SingleSideBand,
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

    #[strum(serialize = "RTTY-R")]
    #[serde(rename = "RTTY-R")]
    RadioTeletypeReverse,

    #[strum(serialize = "FT8")]
    #[serde(rename = "FT8")]
    FT8,

    #[strum(serialize = "FT4")]
    #[serde(rename = "FT4")]
    FT4,

    Unspecified,
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
pub enum RepeaterDuplexOffset {
    #[strum(serialize = "DUP+")]
    #[serde(rename = "DUP+")]
    Plus,
    #[strum(serialize = "DUP-")]
    #[serde(rename = "DUP-")]
    Minus,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! is_child_variant {
    ($fn_name:ident : $self_variant:ident => $child_type:ident :: $child_variant:ident) => {
        #[inline(always)]
        pub fn $fn_name(&self) -> bool {
            matches!(self, Self::$self_variant($child_type::$child_variant))
        }
    };
}

macro_rules! new_child_variant {
    ($fn_name:ident : $self_variant:ident => $child:expr ) => {
        #[inline(always)]
        pub fn $fn_name() -> Self {
            Self::$self_variant($child)
        }
    };
}

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

impl OperatingMode {
    new_child_variant!(new_morse : Morse => MorseMode::ContinuousWave);
    is_child_variant!(is_continuous_wave : Morse => MorseMode::ContinuousWave);

    new_child_variant!(new_modulated_continuous_wave : Morse => MorseMode::ModulatedContinuousWave);
    is_child_variant!(is_modulated_continuous_wave : Morse => MorseMode::ModulatedContinuousWave);

    new_child_variant!(new_continuous_wave_reverse : Morse => MorseMode::ContinuousWaveReverse);
    is_child_variant!(is_continuous_wave_reverse : Morse => MorseMode::ContinuousWaveReverse);

    new_child_variant!(new_frequency_shift_keyed : Morse => MorseMode::FrequencyShiftKeyed);
    is_child_variant!(is_frequency_shift_keyed : Morse => MorseMode::FrequencyShiftKeyed);

    // --------------------------------------------------------------------------------------------

    new_child_variant!(new_amplitude_modulated : AnalogVoice => AnalogVoiceMode::AmplitudeModulated);
    is_child_variant!(is_amplitude_modulated : AnalogVoice => AnalogVoiceMode::AmplitudeModulated);

    new_child_variant!(new_frequency_modulated : AnalogVoice => AnalogVoiceMode::FrequencyModulated);
    is_child_variant!(is_frequency_modulated : AnalogVoice => AnalogVoiceMode::FrequencyModulated);

    new_child_variant!(new_phase_modulated : AnalogVoice => AnalogVoiceMode::PhaseModulated);
    is_child_variant!(is_phase_modulated : AnalogVoice => AnalogVoiceMode::PhaseModulated);

    new_child_variant!(new_wideband_frequency_modulated : AnalogVoice => AnalogVoiceMode::WidebandFrequencyModulated);
    is_child_variant!(is_wideband_frequency_modulated : AnalogVoice => AnalogVoiceMode::WidebandFrequencyModulated);

    new_child_variant!(new_lower_sideband : AnalogVoice => AnalogVoiceMode::LowerSideBand);
    is_child_variant!(is_lower_sideband : AnalogVoice => AnalogVoiceMode::LowerSideBand);

    new_child_variant!(new_upper_sideband : AnalogVoice => AnalogVoiceMode::UpperSideBand);
    is_child_variant!(is_upper_sideband : AnalogVoice => AnalogVoiceMode::UpperSideBand);

    new_child_variant!(new_single_sideband : AnalogVoice => AnalogVoiceMode::SingleSideBand);
    is_child_variant!(is_single_sideband : AnalogVoice => AnalogVoiceMode::SingleSideBand);

    // --------------------------------------------------------------------------------------------

    new_child_variant!(new_d_star : DigitalVoice => DigitalVoiceMode::DStar);
    is_child_variant!(is_d_star : DigitalVoice => DigitalVoiceMode::DStar);

    new_child_variant!(new_free_dv : DigitalVoice => DigitalVoiceMode::FreeDV);
    is_child_variant!(is_free_dv : DigitalVoice => DigitalVoiceMode::FreeDV);

    new_child_variant!(new_dmr : DigitalVoice => DigitalVoiceMode::DigitalMobileRadio);
    is_child_variant!(is_dmr : DigitalVoice => DigitalVoiceMode::DigitalMobileRadio);

    new_child_variant!(new_system_fusion : DigitalVoice => DigitalVoiceMode::SystemFusion);
    is_child_variant!(is_system_fusion : DigitalVoice => DigitalVoiceMode::SystemFusion);

    // --------------------------------------------------------------------------------------------

    new_child_variant!(new_fast_scan_television : Image => ImageMode::FastScanTelevision);
    is_child_variant!(is_fast_scan_television : Image => ImageMode::FastScanTelevision);

    new_child_variant!(new_slow_scan_television : Image => ImageMode::SlowScanTelevision);
    is_child_variant!(is_slow_scan_television : Image => ImageMode::SlowScanTelevision);

    new_child_variant!(new_facsimile : Image => ImageMode::Facsimile);
    is_child_variant!(is_facsimile : Image => ImageMode::Facsimile);

    // --------------------------------------------------------------------------------------------

    new_child_variant!(new_radio_teletype : Data => DataMode::RadioTeletype);
    is_child_variant!(is_radio_teletype : Data => DataMode::RadioTeletype);

    new_child_variant!(new_radio_teletype_reverse : Data => DataMode::RadioTeletypeReverse);
    is_child_variant!(is_radio_teletype_reverse : Data => DataMode::RadioTeletypeReverse);

    new_child_variant!(new_ft8 : Data => DataMode::FT8);
    is_child_variant!(is_ft8 : Data => DataMode::FT8);

    new_child_variant!(new_ft4 : Data => DataMode::FT4);
    is_child_variant!(is_ft4 : Data => DataMode::FT4);

    new_child_variant!(new_data_unspecified : Data => DataMode::Unspecified);
    is_child_variant!(is_data_unspecified : Data => DataMode::Unspecified);
}
