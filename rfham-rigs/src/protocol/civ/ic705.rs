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
    AnalogVoiceMode, DataMode, DigitalVoiceMode, MorseMode, OperatingMode,
    protocol::{Filter, Frequency, civ::BcdString},
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

define_command!(
    "Do..." => SendFrequencyData {
        frequency: Frequency
    }
);

define_command!(
    "Do..." => SendModeData {
        mode: OperatingMode
    }
);

define_command!("Do..." => ReadBandEdgeFrequencies);

define_command!("Do..." => ReadOperatingFrequency);

define_command!("Do..." => ReadOperatingMode);

define_command!(
    "Do...",
    SendOperatingFrequency {
        frequency: Frequency
    }
);

define_command!(
    "Do...",
    SendOperatingMode {
        mode: OperatingModeData
    }
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperatingModeData {
    mode: OperatingMode,
    filter: Option<Filter>,
}

// ------------------------------------------------------------------------------------------------
// Public Function
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

impl_command!(
    SendFrequencyData => 0x00
);

impl_command!(
    SendModeData => 0x01
);

impl_command!(
    ReadBandEdgeFrequencies => 0x02
);

impl_command!(
    ReadOperatingFrequency => 0x03
);

impl_command!(
    ReadOperatingMode => 0x04
);

impl_command!(
    SendOperatingFrequency => 0x05 with Some |cmd: &SendOperatingFrequency|{
    BcdString::from(cmd.frequency).into_bytes()
});

impl_command!(
    SendOperatingMode => 0x06
    with Some |cmd: &SendOperatingMode| {
    vec![
        match cmd.mode.mode {
            OperatingMode::AnalogVoice(AnalogVoiceMode::LowerSideBand) => 0x00,
            OperatingMode::AnalogVoice(AnalogVoiceMode::UpperSideBand) => 0x01,
            OperatingMode::AnalogVoice(AnalogVoiceMode::AmplitudeModulated) => 0x02,
            OperatingMode::Morse(MorseMode::ContinuousWave) => 0x03,
            OperatingMode::Data(DataMode::RadioTeletype) => 0x04,
            OperatingMode::AnalogVoice(AnalogVoiceMode::FrequencyModulated) => 0x05,
            OperatingMode::AnalogVoice(AnalogVoiceMode::WidebandFrequencyModulated) => 0x06,
            OperatingMode::Morse(MorseMode::ContinuousWaveReverse) => 0x07,
            OperatingMode::Data(DataMode::RadioTeletypeReverse) => 0x08,
            OperatingMode::DigitalVoice(DigitalVoiceMode::DStar) => 0x11,
            _ => {
                panic!("Unsupported operating mode: {:?}", cmd.mode.mode)
            }
        },
        match cmd.mode.filter {
            Some(Filter::One) => 0x01,
            Some(Filter::Two) => 0x02,
            Some(Filter::Three) => 0x03,
            None => 0x00,
        },
        ]
});

impl Default for OperatingModeData {
    fn default() -> Self {
        Self {
            mode: OperatingMode::AnalogVoice(AnalogVoiceMode::FrequencyModulated),
            filter: None,
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
mod tests {

    use crate::{
        OperatingMode,
        protocol::{
            Filter, Frequency,
            civ::{
                CivCommand, Command, IC_705_DEFAULT_ADDRESS,
                ic705::{
                    OperatingModeData, ReadOperatingFrequency, ReadOperatingMode,
                    SendOperatingFrequency, SendOperatingMode,
                },
            },
        },
    };

    #[test]
    fn test_read_operating_frequency() {
        assert_eq!(
            "[FE, FE, A4, E0, 03, FD]".to_string(),
            format!(
                "{:02X?}",
                ReadOperatingFrequency::send_to(IC_705_DEFAULT_ADDRESS)
                    .to_message()
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_send_operating_frequency() {
        assert_eq!(
            "[FE, FE, A4, E0, 05, 00, 00, 00, 50, 14, FD]".to_string(),
            format!(
                "{:02X?}",
                SendOperatingFrequency {
                    to_address: IC_705_DEFAULT_ADDRESS,
                    frequency: Frequency::from(145_000_000),
                }
                .to_message()
                .unwrap()
            )
        );
    }

    #[test]
    fn test_read_operating_mode() {
        assert_eq!(
            "[FE, FE, A4, E0, 04, FD]".to_string(),
            format!(
                "{:02X?}",
                ReadOperatingMode::send_to(IC_705_DEFAULT_ADDRESS,)
                    .to_message()
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_send_operating_mode() {
        assert_eq!(
            "[FE, FE, A4, E0, 06, 06, 01, FD]".to_string(),
            format!(
                "{:02X?}",
                SendOperatingMode {
                    to_address: IC_705_DEFAULT_ADDRESS,
                    mode: OperatingModeData {
                        mode: OperatingMode::new_wideband_frequency_modulated(),
                        filter: Some(Filter::One),
                    },
                }
                .to_message()
                .unwrap()
            )
        );
    }
}
