//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::k3_kx`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.
//!
//! # `AtuNetworkValues`, `InstalledOptions` (+ `K3InstalledOptions`/`K4InstalledOptions`/
//! `KXInstalledOptions`), `K3IconsAndStatus`, `TransceiverInformation`
//!
//! These are response-only data structs with no `Command` impl of their own (only the
//! corresponding `Get*` command types, tested below, implement `Command`), so there is nothing to
//! encode and they are skipped here.

use pretty_assertions::assert_eq;
use rfham_itu::allocations::AllocationBand;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command, Vfo,
        cat::elecraft::k3_kx::{
            AutoInfoMode, BaudRate, ClearRitOffset, DataSubMode, DiversityModeState,
            EmulateButtonHold, EmulateButtonTap, GetActualPowerOutput, GetAgcTimeConstant,
            GetAtuNetworkValues, GetAudioPeakingFilterState, GetAutoInfoMode, GetBargraphValue,
            GetBufferedText, GetCwSidetonePitch, GetDataSubMode, GetDiversityMode, GetEssbMode,
            GetFirmwareRevision, GetHighResolutionSMeter, GetIfCenterFrequency,
            GetInstalledOptions, GetK3IconsAndStatus, GetKeyerSpeed, GetMemoryChannel,
            GetMenuParameter, GetMenuParameter16, GetMicGain, GetMonitorLevel, GetPowerStatus,
            GetQskDelay, GetReceiveAntenna, GetReceiveVfo, GetRitControl, GetRitXitOffset,
            GetSpeechCompression, GetSubReceiver, GetTransceiverInformation,
            GetTransmitBufferedText, GetTransmitMeterMode, GetTransmitPowerControl,
            GetTransmitState, GetTransmitVfoSplitModeState, GetVfoABandNumber,
            GetVfoADisplayAndIcons, GetVfoAFilterBandwidth, GetVfoAIfShift,
            GetVfoALegacyFilterBandwidth, GetVfoALockState, GetVfoANoiseBlanker,
            GetVfoANoiseBlankerLevel, GetVfoAOperatingMode, GetVfoAPreamp,
            GetVfoAReceiveAttenuator, GetVfoARfGain, GetVfoASMeter, GetVfoASquelch,
            GetVfoAXfilNumber, GetVfoBBandNumber, GetVfoBDisplayText, GetVfoBFilterBandwidth,
            GetVfoBIfShift, GetVfoBLegacyFilterBandwidth, GetVfoBLockState, GetVfoBNoiseBlanker,
            GetVfoBNoiseBlankerLevel, GetVfoBOperatingMode, GetVfoBPreamp,
            GetVfoBReceiveAttenuator, GetVfoBRfGain, GetVfoBSMeter, GetVfoBSquelch,
            GetVfoBXfilNumber, GetVfoLinkedState, GetVox, GetXitControl, GoToReceive, GoToTransmit,
            MoveRitOffsetDown, MoveRitOffsetUp, MoveVfoAFrequencyDown, MoveVfoAFrequencyUp,
            MoveVfoBFrequencyDown, MoveVfoBFrequencyUp, SelectMenuItem, SendCwText,
            SetAgcTimeConstant, SetAudioPeakingFilterState, SetAutoInfoMode, SetBaudRate,
            SetCommandProcessingDelay, SetDataSubMode, SetDiversityMode, SetErrorLogging,
            SetEssbMode, SetKeyerSpeed, SetMemoryChannel, SetMenuParameter, SetMenuParameter16,
            SetMicGain, SetMonitorLevel, SetPowerStatus, SetReceiveAntenna, SetReceiveVfo,
            SetRitControl, SetRitXitOffset, SetSpeechCompression, SetSubReceiver,
            SetTextToTerminal, SetTransmitEqualizer, SetTransmitMeterMode, SetTransmitPowerControl,
            SetTransmitVfoSplitModeState, SetVfoABandNumber, SetVfoAFilterBandwidth,
            SetVfoAIfShift, SetVfoALegacyFilterBandwidth, SetVfoALockState, SetVfoANoiseBlanker,
            SetVfoANoiseBlankerLevel, SetVfoAOperatingMode, SetVfoAPreamp,
            SetVfoAReceiveAttenuator, SetVfoARfGain, SetVfoASquelch, SetVfoBBandNumber,
            SetVfoBDisplayText, SetVfoBFilterBandwidth, SetVfoBIfShift,
            SetVfoBLegacyFilterBandwidth, SetVfoBLockState, SetVfoBNoiseBlanker,
            SetVfoBNoiseBlankerLevel, SetVfoBOperatingMode, SetVfoBPreamp,
            SetVfoBReceiveAttenuator, SetVfoBRfGain, SetVfoBSquelch, SetVfoLinkedState, SetVox,
            SetXitControl, VfoFrequencyChangeStep,
        },
    },
};

/// Mirrors the sign encoding produced by the private `format_i16_ascii_4` helper: a literal
/// `+`/`-` character followed by the zero-padded magnitude. (An earlier version of this helper
/// printed the sign byte's *decimal ASCII code point* — `43`/`45` — instead of the character
/// itself; that bug has since been fixed in `format_i16_ascii_4`.)
fn expected_signed_offset_4(n: i16) -> Vec<u8> {
    let sign = if n.is_negative() { "-" } else { "+" };
    format!("{sign}{:04}", n.unsigned_abs()).into_bytes()
}

// ------------------------------------------------------------------------------------------------
// AI: GetAutoInfoMode, SetAutoInfoMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_auto_info_mode_encodes() {
    assert_eq!(GetAutoInfoMode.to_message().unwrap(), b"AI;".to_vec());
}

#[test]
fn set_auto_info_mode_encodes_off() {
    let cmd = SetAutoInfoMode {
        mode: AutoInfoMode::Off,
    };
    assert_eq!(cmd.to_message().unwrap(), b"AI0;".to_vec());
}

#[test]
fn set_auto_info_mode_encodes_k2() {
    let cmd = SetAutoInfoMode {
        mode: AutoInfoMode::K2,
    };
    assert_eq!(cmd.to_message().unwrap(), b"AI1;".to_vec());
}

#[test]
fn set_auto_info_mode_encodes_k3() {
    let cmd = SetAutoInfoMode {
        mode: AutoInfoMode::K3,
    };
    assert_eq!(cmd.to_message().unwrap(), b"AI2;".to_vec());
}

#[test]
fn set_auto_info_mode_encodes_k3_extended() {
    let cmd = SetAutoInfoMode {
        mode: AutoInfoMode::K3Extended,
    };
    assert_eq!(cmd.to_message().unwrap(), b"AI3;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// AK: GetAtuNetworkValues
// ------------------------------------------------------------------------------------------------

#[test]
fn get_atu_network_values_encodes() {
    assert_eq!(GetAtuNetworkValues.to_message().unwrap(), b"AK;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// AP: GetAudioPeakingFilterState, SetAudioPeakingFilterState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_audio_peaking_filter_state_encodes() {
    assert_eq!(
        GetAudioPeakingFilterState.to_message().unwrap(),
        b"AP;".to_vec()
    );
}

#[test]
fn set_audio_peaking_filter_state_encodes_on() {
    let cmd = SetAudioPeakingFilterState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"AP1;".to_vec());
}

#[test]
fn set_audio_peaking_filter_state_encodes_off() {
    let cmd = SetAudioPeakingFilterState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"AP0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// AR: GetRxAntenna, SetRxAntenna
// ------------------------------------------------------------------------------------------------

#[test]
fn get_rx_antenna_encodes() {
    assert_eq!(GetReceiveAntenna.to_message().unwrap(), b"AR;".to_vec());
}

#[test]
fn set_rx_antenna_encodes_true() {
    let cmd = SetReceiveAntenna { rx_only: true };
    assert_eq!(cmd.to_message().unwrap(), b"AR1;".to_vec());
}

#[test]
fn set_rx_antenna_encodes_false() {
    let cmd = SetReceiveAntenna { rx_only: false };
    assert_eq!(cmd.to_message().unwrap(), b"AR0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// BG: GetBargraph
// ------------------------------------------------------------------------------------------------

#[test]
fn get_bargraph_encodes() {
    assert_eq!(GetBargraphValue.to_message().unwrap(), b"BG;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// BN: GetBandNumberA, GetBandNumberB, SetBandNumberA, SetBandNumberB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_band_number_a_encodes() {
    assert_eq!(GetVfoABandNumber.to_message().unwrap(), b"BN;".to_vec());
}

#[test]
fn get_band_number_b_encodes() {
    assert_eq!(GetVfoBBandNumber.to_message().unwrap(), b"BN$;".to_vec());
}

#[test]
fn set_band_number_a_encodes() {
    let cmd = SetVfoABandNumber {
        band: AllocationBand::Band40M,
    };
    assert_eq!(cmd.to_message().unwrap(), b"BN03;".to_vec());
}

#[test]
fn set_band_number_b_encodes() {
    let cmd = SetVfoBBandNumber {
        band: AllocationBand::Band20M,
    };
    assert_eq!(cmd.to_message().unwrap(), b"BN$05;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// BR: SetBaudRate
// ------------------------------------------------------------------------------------------------

#[test]
fn set_baud_rate_encodes_4800() {
    let cmd = SetBaudRate {
        rate: BaudRate::Rate4800,
    };
    assert_eq!(cmd.to_message().unwrap(), b"BR0;".to_vec());
}

#[test]
fn set_baud_rate_encodes_115200() {
    let cmd = SetBaudRate {
        rate: BaudRate::Rate115200,
    };
    assert_eq!(cmd.to_message().unwrap(), b"BR5;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// BW: GetFilterBandwidthA, GetFilterBandwidthB, SetFilterBandwidthA, SetFilterBandwidthB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_filter_bandwidth_a_encodes() {
    assert_eq!(
        GetVfoAFilterBandwidth.to_message().unwrap(),
        b"BW;".to_vec()
    );
}

#[test]
fn get_filter_bandwidth_b_encodes() {
    assert_eq!(
        GetVfoBFilterBandwidth.to_message().unwrap(),
        b"BW$;".to_vec()
    );
}

#[test]
fn set_filter_bandwidth_a_encodes() {
    let cmd = SetVfoAFilterBandwidth {
        bandwidth_10hz: 500,
    };
    assert_eq!(cmd.to_message().unwrap(), b"BW0500;".to_vec());
}

#[test]
fn set_filter_bandwidth_b_encodes() {
    let cmd = SetVfoBFilterBandwidth {
        bandwidth_10hz: 1234,
    };
    assert_eq!(cmd.to_message().unwrap(), b"BW$1234;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// CP: GetSpeechCompression, SetSpeechCompression
// ------------------------------------------------------------------------------------------------

#[test]
fn get_speech_compression_encodes() {
    assert_eq!(GetSpeechCompression.to_message().unwrap(), b"CP;".to_vec());
}

#[test]
fn set_speech_compression_encodes() {
    let cmd = SetSpeechCompression { level: 25 };
    assert_eq!(cmd.to_message().unwrap(), b"CP25;".to_vec());
}

#[test]
fn set_speech_compression_accepts_boundary_values() {
    assert!(SetSpeechCompression { level: 0 }.validate().is_ok());
    assert!(SetSpeechCompression { level: 40 }.validate().is_ok());
}

#[test]
fn set_speech_compression_rejects_out_of_range() {
    assert!(matches!(
        SetSpeechCompression { level: 41 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// CW: GetCwSidetonePitch
// ------------------------------------------------------------------------------------------------

#[test]
fn get_cw_sidetone_pitch_encodes() {
    assert_eq!(GetCwSidetonePitch.to_message().unwrap(), b"CW;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// DB: GetVfoBDisplayText, SetVfoBDisplayText
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_b_display_text_encodes() {
    assert_eq!(GetVfoBDisplayText.to_message().unwrap(), b"DB;".to_vec());
}

#[test]
fn set_vfo_b_display_text_encodes() {
    let cmd = SetVfoBDisplayText { text: *b"HELLO   " };
    assert_eq!(cmd.to_message().unwrap(), b"DBHELLO   ;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// DE: SetCommandDelay
// ------------------------------------------------------------------------------------------------

#[test]
fn set_command_delay_encodes() {
    let cmd = SetCommandProcessingDelay { delay_5ms: 20 };
    assert_eq!(cmd.to_message().unwrap(), b"DE20;".to_vec());
}

#[test]
fn set_command_delay_accepts_boundary_values() {
    assert!(
        SetCommandProcessingDelay { delay_5ms: 0 }
            .validate()
            .is_ok()
    );
    assert!(
        SetCommandProcessingDelay { delay_5ms: 99 }
            .validate()
            .is_ok()
    );
}

#[test]
fn set_command_delay_rejects_out_of_range() {
    assert!(matches!(
        SetCommandProcessingDelay { delay_5ms: 100 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// DN: VfoADown, VfoBDown
// ------------------------------------------------------------------------------------------------

#[test]
fn vfo_a_down_encodes() {
    assert_eq!(
        MoveVfoAFrequencyDown {
            step: Some(VfoFrequencyChangeStep::Step10Hz)
        }
        .to_message()
        .unwrap(),
        b"DN1;".to_vec()
    );
}

#[test]
fn vfo_b_down_encodes() {
    assert_eq!(
        MoveVfoBFrequencyDown {
            step: Some(VfoFrequencyChangeStep::Step10Hz)
        }
        .to_message()
        .unwrap(),
        b"DNB1;".to_vec()
    );
}

#[test]
fn vfo_a_down_encodes_none() {
    assert_eq!(
        MoveVfoAFrequencyDown { step: None }.to_message().unwrap(),
        b"DN;".to_vec()
    );
}

#[test]
fn vfo_b_down_encodes_none() {
    assert_eq!(
        MoveVfoBFrequencyDown { step: None }.to_message().unwrap(),
        b"DNB;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// DS: GetVfoADisplayText
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_a_display_text_encodes() {
    let cmd = GetVfoADisplayAndIcons;
    assert_eq!(cmd.to_message().unwrap(), b"DS;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// DT: GetDataSubMode, SetDataSubMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_data_sub_mode_encodes() {
    assert_eq!(GetDataSubMode.to_message().unwrap(), b"DT;".to_vec());
}

#[test]
fn set_data_sub_mode_encodes_data_afsk() {
    let cmd = SetDataSubMode {
        sub_mode: DataSubMode::DataAfsk,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DT0;".to_vec());
}

#[test]
fn set_data_sub_mode_encodes_afsk_a() {
    let cmd = SetDataSubMode {
        sub_mode: DataSubMode::AfskA,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DT1;".to_vec());
}

#[test]
fn set_data_sub_mode_encodes_fsk_d() {
    let cmd = SetDataSubMode {
        sub_mode: DataSubMode::FskD,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DT2;".to_vec());
}

#[test]
fn set_data_sub_mode_encodes_psk_d() {
    let cmd = SetDataSubMode {
        sub_mode: DataSubMode::PskD,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DT3;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// DV: GetDiversityMode, SetDiversityMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_diversity_mode_encodes() {
    assert_eq!(GetDiversityMode.to_message().unwrap(), b"DV;".to_vec());
}

#[test]
fn set_diversity_mode_encodes_on() {
    let cmd = SetDiversityMode {
        state: DiversityModeState::On,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DV1;".to_vec());
}

#[test]
fn set_diversity_mode_encodes_off() {
    let cmd = SetDiversityMode {
        state: DiversityModeState::Off,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DV0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// EL: SetErrorLogging
// ------------------------------------------------------------------------------------------------

#[test]
fn set_error_logging_encodes_on() {
    let cmd = SetErrorLogging { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"EL1;".to_vec());
}

#[test]
fn set_error_logging_encodes_off() {
    let cmd = SetErrorLogging { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"EL0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// ES: GetEssbMode, SetEssbMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_essb_mode_encodes() {
    assert_eq!(GetEssbMode.to_message().unwrap(), b"ES;".to_vec());
}

#[test]
fn set_essb_mode_encodes_on() {
    let cmd = SetEssbMode { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"ES1;".to_vec());
}

#[test]
fn set_essb_mode_encodes_off() {
    let cmd = SetEssbMode { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"ES0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// FI: GetIfCenterFrequency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_if_center_frequency_encodes() {
    assert_eq!(GetIfCenterFrequency.to_message().unwrap(), b"FI;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// FR: GetReceiveVfo, SetReceiveVfo
// ------------------------------------------------------------------------------------------------

#[test]
fn get_receive_vfo_encodes() {
    assert_eq!(GetReceiveVfo.to_message().unwrap(), b"FR;".to_vec());
}

#[test]
fn set_receive_vfo_encodes_a() {
    let cmd = SetReceiveVfo { vfo: Vfo::A };
    assert_eq!(cmd.to_message().unwrap(), b"FR0;".to_vec());
}

#[test]
fn set_receive_vfo_encodes_b() {
    let cmd = SetReceiveVfo { vfo: Vfo::B };
    assert_eq!(cmd.to_message().unwrap(), b"FR1;".to_vec());
}

#[test]
fn set_receive_vfo_accepts_a_and_b() {
    assert!(SetReceiveVfo { vfo: Vfo::A }.validate().is_ok());
    assert!(SetReceiveVfo { vfo: Vfo::B }.validate().is_ok());
}

#[test]
fn set_receive_vfo_rejects_other_vfos() {
    assert!(matches!(
        SetReceiveVfo { vfo: Vfo::C }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// FT: GetTransmitVfoSplitModeState, SetTransmitVfoSplitModeState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transmit_vfo_split_mode_state_encodes() {
    assert_eq!(
        GetTransmitVfoSplitModeState.to_message().unwrap(),
        b"FT;".to_vec()
    );
}

#[test]
fn set_transmit_vfo_split_mode_state_encodes_on() {
    let cmd = SetTransmitVfoSplitModeState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"FT1;".to_vec());
}

#[test]
fn set_transmit_vfo_split_mode_state_encodes_off() {
    let cmd = SetTransmitVfoSplitModeState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"FT0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// FW: GetLegacyFilterBandwidthA, GetLegacyFilterBandwidthB, SetLegacyFilterBandwidthA,
// SetLegacyFilterBandwidthB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_legacy_filter_bandwidth_a_encodes() {
    assert_eq!(
        GetVfoALegacyFilterBandwidth.to_message().unwrap(),
        b"FW;".to_vec()
    );
}

#[test]
fn get_legacy_filter_bandwidth_b_encodes() {
    assert_eq!(
        GetVfoBLegacyFilterBandwidth.to_message().unwrap(),
        b"FW$;".to_vec()
    );
}

#[test]
fn set_legacy_filter_bandwidth_a_encodes() {
    let cmd = SetVfoALegacyFilterBandwidth { bandwidth_hz: 2500 };
    assert_eq!(cmd.to_message().unwrap(), b"FW2500;".to_vec());
}

#[test]
fn set_legacy_filter_bandwidth_b_encodes() {
    let cmd = SetVfoBLegacyFilterBandwidth { bandwidth_hz: 300 };
    assert_eq!(cmd.to_message().unwrap(), b"FW$0300;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GT: GetAgcTimeConstant, SetAgcTimeConstant
// ------------------------------------------------------------------------------------------------

#[test]
fn get_agc_time_constant_encodes() {
    assert_eq!(GetAgcTimeConstant.to_message().unwrap(), b"GT;".to_vec());
}

#[test]
fn set_agc_time_constant_encodes() {
    let cmd = SetAgcTimeConstant { mode: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"GT02;".to_vec());
}

#[test]
fn set_agc_time_constant_accepts_boundary_values() {
    assert!(SetAgcTimeConstant { mode: 0 }.validate().is_ok());
    assert!(SetAgcTimeConstant { mode: 3 }.validate().is_ok());
}

#[test]
fn set_agc_time_constant_rejects_out_of_range() {
    assert!(matches!(
        SetAgcTimeConstant { mode: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// OM: GetInstalledOptions
// ------------------------------------------------------------------------------------------------

#[test]
fn get_installed_options_encodes() {
    assert_eq!(GetInstalledOptions.to_message().unwrap(), b"OM;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// IC: GetK3IconsAndStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_k3_icons_and_status_encodes() {
    assert_eq!(GetK3IconsAndStatus.to_message().unwrap(), b"IC;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// IF: GetTransceiverInformation
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transceiver_information_encodes() {
    assert_eq!(
        GetTransceiverInformation.to_message().unwrap(),
        b"IF;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// MD: GetOperatingModeA, GetOperatingModeB, SetOperatingModeA, SetOperatingModeB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_operating_mode_a_encodes() {
    assert_eq!(GetVfoAOperatingMode.to_message().unwrap(), b"MD;".to_vec());
}

#[test]
fn get_operating_mode_b_encodes() {
    assert_eq!(GetVfoBOperatingMode.to_message().unwrap(), b"MD$;".to_vec());
}

#[test]
fn set_operating_mode_a_encodes() {
    let cmd = SetVfoAOperatingMode { mode: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"MD3;".to_vec());
}

#[test]
fn set_operating_mode_a_accepts_boundary_values() {
    assert!(SetVfoAOperatingMode { mode: 1 }.validate().is_ok());
    assert!(SetVfoAOperatingMode { mode: 9 }.validate().is_ok());
}

#[test]
fn set_operating_mode_a_rejects_undefined_digits() {
    assert!(matches!(
        SetVfoBOperatingMode { mode: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoBOperatingMode { mode: 8 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_operating_mode_b_encodes() {
    let cmd = SetVfoBOperatingMode { mode: 7 };
    assert_eq!(cmd.to_message().unwrap(), b"MD$7;".to_vec());
}

#[test]
fn set_operating_mode_b_accepts_boundary_values() {
    assert!(SetVfoBOperatingMode { mode: 1 }.validate().is_ok());
    assert!(SetVfoBOperatingMode { mode: 9 }.validate().is_ok());
}

#[test]
fn set_operating_mode_b_rejects_undefined_digits() {
    assert!(matches!(
        SetVfoBOperatingMode { mode: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoBOperatingMode { mode: 8 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// IS: GetIfShiftA, GetIfShiftB, SetIfShiftA, SetIfShiftB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_if_shift_a_encodes() {
    assert_eq!(GetVfoAIfShift.to_message().unwrap(), b"IS;".to_vec());
}

#[test]
fn get_if_shift_b_encodes() {
    assert_eq!(GetVfoBIfShift.to_message().unwrap(), b"IS$;".to_vec());
}

#[test]
fn set_if_shift_a_encodes() {
    let cmd = SetVfoAIfShift { offset_hz: 1500 };
    let mut expected = b"IS".to_vec();
    expected.extend(expected_signed_offset_4(1500));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_if_shift_a_accepts_boundary_values() {
    assert!(SetVfoAIfShift { offset_hz: 2999 }.validate().is_ok());
    assert!(SetVfoAIfShift { offset_hz: -2999 }.validate().is_ok());
}

#[test]
fn set_if_shift_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoAIfShift { offset_hz: 3000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoAIfShift { offset_hz: -3000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_if_shift_b_encodes() {
    let cmd = SetVfoBIfShift { offset_hz: -500 };
    let mut expected = b"IS$".to_vec();
    expected.extend(expected_signed_offset_4(-500));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_if_shift_b_accepts_boundary_values() {
    assert!(SetVfoBIfShift { offset_hz: 2999 }.validate().is_ok());
    assert!(SetVfoBIfShift { offset_hz: -2999 }.validate().is_ok());
}

#[test]
fn set_if_shift_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBIfShift { offset_hz: 3000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoBIfShift { offset_hz: -3000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// KS: GetKeyerSpeed, SetKeyerSpeed
// ------------------------------------------------------------------------------------------------

#[test]
fn get_keyer_speed_encodes() {
    assert_eq!(GetKeyerSpeed.to_message().unwrap(), b"KS;".to_vec());
}

#[test]
fn set_keyer_speed_encodes() {
    let cmd = SetKeyerSpeed { wpm: 25 };
    assert_eq!(cmd.to_message().unwrap(), b"KS025;".to_vec());
}

#[test]
fn set_keyer_speed_accepts_boundary_values() {
    assert!(SetKeyerSpeed { wpm: 8 }.validate().is_ok());
    assert!(SetKeyerSpeed { wpm: 100 }.validate().is_ok());
}

#[test]
fn set_keyer_speed_rejects_out_of_range() {
    assert!(matches!(
        SetKeyerSpeed { wpm: 7 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetKeyerSpeed { wpm: 101 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// KY: SendCwText
// ------------------------------------------------------------------------------------------------

#[test]
fn send_cw_text_encodes_exact_length_send_now() {
    // 24 bytes of text needs no padding, keeping the expected value simple to construct.
    let text: Vec<u8> = b"123456789012345678901234".to_vec();
    assert_eq!(text.len(), 24);
    let cmd = SendCwText {
        buffer_only: false,
        text: text.clone(),
    };
    let mut expected = b"KY0 ".to_vec();
    expected.extend_from_slice(&text);
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn send_cw_text_encodes_buffer_only_flag() {
    let text: Vec<u8> = b"123456789012345678901234".to_vec();
    let cmd = SendCwText {
        buffer_only: true,
        text: text.clone(),
    };
    let mut expected = b"KY1 ".to_vec();
    expected.extend_from_slice(&text);
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn send_cw_text_pads_short_text_with_spaces() {
    let cmd = SendCwText {
        buffer_only: false,
        text: vec![],
    };
    let mut expected = b"KY0 ".to_vec();
    expected.extend(std::iter::repeat_n(b' ', 24));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn send_cw_text_truncates_long_text_to_24_bytes() {
    let long_text: Vec<u8> = std::iter::repeat_n(b'A', 30).collect();
    let cmd = SendCwText {
        buffer_only: false,
        text: long_text,
    };
    let mut expected = b"KY0 ".to_vec();
    expected.extend(std::iter::repeat_n(b'A', 24));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

// ------------------------------------------------------------------------------------------------
// LK: GetVfoLockA, GetVfoLockB, SetVfoLockA, SetVfoLockB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_lock_a_encodes() {
    assert_eq!(GetVfoALockState.to_message().unwrap(), b"LK;".to_vec());
}

#[test]
fn get_vfo_lock_b_encodes() {
    assert_eq!(GetVfoBLockState.to_message().unwrap(), b"LK$;".to_vec());
}

#[test]
fn set_vfo_lock_a_encodes_locked() {
    let cmd = SetVfoALockState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"LK1;".to_vec());
}

#[test]
fn set_vfo_lock_a_encodes_unlocked() {
    let cmd = SetVfoALockState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"LK0;".to_vec());
}

#[test]
fn set_vfo_lock_b_encodes_locked() {
    let cmd = SetVfoBLockState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"LK$1;".to_vec());
}

#[test]
fn set_vfo_lock_b_encodes_unlocked() {
    let cmd = SetVfoBLockState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"LK$0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// LN: GetVfoLink, SetVfoLink
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_link_encodes() {
    assert_eq!(GetVfoLinkedState.to_message().unwrap(), b"LN;".to_vec());
}

#[test]
fn set_vfo_link_encodes_linked() {
    let cmd = SetVfoLinkedState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"LN1;".to_vec());
}

#[test]
fn set_vfo_link_encodes_unlinked() {
    let cmd = SetVfoLinkedState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"LN0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// MC: GetMemoryChannel, SetMemoryChannel
// ------------------------------------------------------------------------------------------------

#[test]
fn get_memory_channel_encodes() {
    assert_eq!(GetMemoryChannel.to_message().unwrap(), b"MC;".to_vec());
}

#[test]
fn set_memory_channel_encodes() {
    let cmd = SetMemoryChannel { channel: 42 };
    assert_eq!(cmd.to_message().unwrap(), b"MC042;".to_vec());
}

#[test]
fn set_memory_channel_accepts_boundary_values() {
    assert!(SetMemoryChannel { channel: 1 }.validate().is_ok());
    assert!(SetMemoryChannel { channel: 100 }.validate().is_ok());
}

#[test]
fn set_memory_channel_rejects_out_of_range() {
    assert!(matches!(
        SetMemoryChannel { channel: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetMemoryChannel { channel: 101 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// MG: GetMicGain, SetMicGain
// ------------------------------------------------------------------------------------------------

#[test]
fn get_mic_gain_encodes() {
    assert_eq!(GetMicGain.to_message().unwrap(), b"MG;".to_vec());
}

#[test]
fn set_mic_gain_encodes() {
    let cmd = SetMicGain { gain: 30 };
    assert_eq!(cmd.to_message().unwrap(), b"MG030;".to_vec());
}

#[test]
fn set_mic_gain_accepts_boundary_values() {
    assert!(SetMicGain { gain: 0 }.validate().is_ok());
    assert!(SetMicGain { gain: 60 }.validate().is_ok());
}

#[test]
fn set_mic_gain_rejects_out_of_range() {
    assert!(matches!(
        SetMicGain { gain: 61 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// ML: GetMonitorLevel, SetMonitorLevel
// ------------------------------------------------------------------------------------------------

#[test]
fn get_monitor_level_encodes() {
    assert_eq!(GetMonitorLevel.to_message().unwrap(), b"ML;".to_vec());
}

#[test]
fn set_monitor_level_encodes() {
    let cmd = SetMonitorLevel { level: 15 };
    assert_eq!(cmd.to_message().unwrap(), b"ML015;".to_vec());
}

#[test]
fn set_monitor_level_accepts_boundary_values() {
    assert!(SetMonitorLevel { level: 0 }.validate().is_ok());
    assert!(SetMonitorLevel { level: 60 }.validate().is_ok());
}

#[test]
fn set_monitor_level_rejects_out_of_range() {
    assert!(matches!(
        SetMonitorLevel { level: 61 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// MN: SelectMenu
// ------------------------------------------------------------------------------------------------

#[test]
fn select_menu_encodes() {
    let cmd = SelectMenuItem { item: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"MN005;".to_vec());
}

#[test]
fn select_menu_accepts_boundary_values() {
    assert!(SelectMenuItem { item: 0 }.validate().is_ok());
    assert!(SelectMenuItem { item: 999 }.validate().is_ok());
}

#[test]
fn select_menu_rejects_out_of_range() {
    assert!(matches!(
        SelectMenuItem { item: 1000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// MP: GetMenuParameter, SetMenuParameter
// ------------------------------------------------------------------------------------------------

#[test]
fn get_menu_parameter_encodes() {
    assert_eq!(GetMenuParameter.to_message().unwrap(), b"MP;".to_vec());
}

#[test]
fn set_menu_parameter_encodes() {
    let cmd = SetMenuParameter { value: 7 };
    assert_eq!(cmd.to_message().unwrap(), b"MP07;".to_vec());
}

#[test]
fn set_menu_parameter_accepts_boundary_values() {
    assert!(SetMenuParameter { value: 0 }.validate().is_ok());
    assert!(SetMenuParameter { value: 99 }.validate().is_ok());
}

#[test]
fn set_menu_parameter_rejects_out_of_range() {
    assert!(matches!(
        SetMenuParameter { value: 100 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// MQ: GetMenuParameter16, SetMenuParameter16
// ------------------------------------------------------------------------------------------------

#[test]
fn get_menu_parameter_16_encodes() {
    assert_eq!(GetMenuParameter16.to_message().unwrap(), b"MQ;".to_vec());
}

#[test]
fn set_menu_parameter_16_encodes() {
    let cmd = SetMenuParameter16 { value: 1234 };
    assert_eq!(cmd.to_message().unwrap(), b"MQ1234;".to_vec());
}

#[test]
fn set_menu_parameter_16_accepts_boundary_values() {
    assert!(SetMenuParameter16 { value: 0 }.validate().is_ok());
    assert!(SetMenuParameter16 { value: 9999 }.validate().is_ok());
}

#[test]
fn set_menu_parameter_16_rejects_out_of_range() {
    assert!(matches!(
        SetMenuParameter16 { value: 10000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// NB: GetNoiseBlankerA, GetNoiseBlankerB, SetNoiseBlankerA, SetNoiseBlankerB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_noise_blanker_a_encodes() {
    assert_eq!(GetVfoANoiseBlanker.to_message().unwrap(), b"NB;".to_vec());
}

#[test]
fn get_noise_blanker_b_encodes() {
    assert_eq!(GetVfoBNoiseBlanker.to_message().unwrap(), b"NB$;".to_vec());
}

#[test]
fn set_noise_blanker_a_encodes_on() {
    let cmd = SetVfoANoiseBlanker { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"NB1;".to_vec());
}

#[test]
fn set_noise_blanker_a_encodes_off() {
    let cmd = SetVfoANoiseBlanker { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"NB0;".to_vec());
}

#[test]
fn set_noise_blanker_b_encodes_on() {
    let cmd = SetVfoBNoiseBlanker { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"NB$1;".to_vec());
}

#[test]
fn set_noise_blanker_b_encodes_off() {
    let cmd = SetVfoBNoiseBlanker { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"NB$0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// NL: GetNoiseBlankerLevelA, GetNoiseBlankerLevelB, SetNoiseBlankerLevelA, SetNoiseBlankerLevelB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_noise_blanker_level_a_encodes() {
    assert_eq!(
        GetVfoANoiseBlankerLevel.to_message().unwrap(),
        b"NL;".to_vec()
    );
}

#[test]
fn get_noise_blanker_level_b_encodes() {
    assert_eq!(
        GetVfoBNoiseBlankerLevel.to_message().unwrap(),
        b"NL$;".to_vec()
    );
}

#[test]
fn set_noise_blanker_level_a_encodes() {
    let cmd = SetVfoANoiseBlankerLevel { level: 10 };
    assert_eq!(cmd.to_message().unwrap(), b"NL10;".to_vec());
}

#[test]
fn set_noise_blanker_level_a_accepts_boundary_values() {
    assert!(SetVfoANoiseBlankerLevel { level: 0 }.validate().is_ok());
    assert!(SetVfoANoiseBlankerLevel { level: 21 }.validate().is_ok());
}

#[test]
fn set_noise_blanker_level_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoANoiseBlankerLevel { level: 22 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_noise_blanker_level_b_encodes() {
    let cmd = SetVfoBNoiseBlankerLevel { level: 9 };
    assert_eq!(cmd.to_message().unwrap(), b"NL$09;".to_vec());
}

#[test]
fn set_noise_blanker_level_b_accepts_boundary_values() {
    assert!(SetVfoBNoiseBlankerLevel { level: 0 }.validate().is_ok());
    assert!(SetVfoBNoiseBlankerLevel { level: 21 }.validate().is_ok());
}

#[test]
fn set_noise_blanker_level_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBNoiseBlankerLevel { level: 22 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// PA: GetPreampA, GetPreampB, SetPreampA, SetPreampB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_preamp_a_encodes() {
    assert_eq!(GetVfoAPreamp.to_message().unwrap(), b"PA;".to_vec());
}

#[test]
fn get_preamp_b_encodes() {
    assert_eq!(GetVfoBPreamp.to_message().unwrap(), b"PA$;".to_vec());
}

#[test]
fn set_preamp_a_encodes() {
    let cmd = SetVfoAPreamp { preamp: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"PA1;".to_vec());
}

#[test]
fn set_preamp_a_accepts_boundary_values() {
    assert!(SetVfoAPreamp { preamp: 0 }.validate().is_ok());
    assert!(SetVfoAPreamp { preamp: 2 }.validate().is_ok());
}

#[test]
fn set_preamp_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoAPreamp { preamp: 3 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_preamp_b_encodes() {
    let cmd = SetVfoBPreamp { preamp: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"PA$2;".to_vec());
}

#[test]
fn set_preamp_b_accepts_boundary_values() {
    assert!(SetVfoBPreamp { preamp: 0 }.validate().is_ok());
    assert!(SetVfoBPreamp { preamp: 2 }.validate().is_ok());
}

#[test]
fn set_preamp_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBPreamp { preamp: 3 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// PC: GetPowerControl, SetPowerControl
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_control_encodes() {
    assert_eq!(
        GetTransmitPowerControl.to_message().unwrap(),
        b"PC;".to_vec()
    );
}

#[test]
fn set_power_control_encodes() {
    let cmd = SetTransmitPowerControl { watts: 100 };
    assert_eq!(cmd.to_message().unwrap(), b"PC100;".to_vec());
}

#[test]
fn set_power_control_accepts_boundary_values() {
    assert!(SetTransmitPowerControl { watts: 0 }.validate().is_ok());
    assert!(SetTransmitPowerControl { watts: 110 }.validate().is_ok());
}

#[test]
fn set_power_control_rejects_out_of_range() {
    assert!(matches!(
        SetTransmitPowerControl { watts: 111 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// PO: GetActualPowerOutput
// ------------------------------------------------------------------------------------------------

#[test]
fn get_actual_power_output_encodes() {
    assert_eq!(GetActualPowerOutput.to_message().unwrap(), b"PO;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// PS: GetPowerStatus, SetPowerStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_status_encodes() {
    assert_eq!(GetPowerStatus.to_message().unwrap(), b"PS;".to_vec());
}

#[test]
fn set_power_status_encodes_on() {
    let cmd = SetPowerStatus { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"PS1;".to_vec());
}

#[test]
fn set_power_status_encodes_off() {
    let cmd = SetPowerStatus { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"PS0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// RA: GetReceiveAttenuatorA, GetReceiveAttenuatorB, SetReceiveAttenuatorA, SetReceiveAttenuatorB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_receive_attenuator_a_encodes() {
    assert_eq!(
        GetVfoAReceiveAttenuator.to_message().unwrap(),
        b"RA;".to_vec()
    );
}

#[test]
fn get_receive_attenuator_b_encodes() {
    assert_eq!(
        GetVfoBReceiveAttenuator.to_message().unwrap(),
        b"RA$;".to_vec()
    );
}

#[test]
fn set_receive_attenuator_a_encodes() {
    let cmd = SetVfoAReceiveAttenuator { level: 6 };
    assert_eq!(cmd.to_message().unwrap(), b"RA06;".to_vec());
}

#[test]
fn set_receive_attenuator_a_accepts_boundary_values() {
    assert!(SetVfoAReceiveAttenuator { level: 0 }.validate().is_ok());
    assert!(SetVfoAReceiveAttenuator { level: 15 }.validate().is_ok());
}

#[test]
fn set_receive_attenuator_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoAReceiveAttenuator { level: 16 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_receive_attenuator_b_encodes() {
    let cmd = SetVfoBReceiveAttenuator { level: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"RA$01;".to_vec());
}

#[test]
fn set_receive_attenuator_b_accepts_boundary_values() {
    assert!(SetVfoBReceiveAttenuator { level: 0 }.validate().is_ok());
    assert!(SetVfoBReceiveAttenuator { level: 15 }.validate().is_ok());
}

#[test]
fn set_receive_attenuator_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBReceiveAttenuator { level: 16 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// RC: ClearRit
// ------------------------------------------------------------------------------------------------

#[test]
fn clear_rit_encodes() {
    assert_eq!(ClearRitOffset.to_message().unwrap(), b"RC;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// RD: RitOffsetDown
// ------------------------------------------------------------------------------------------------

#[test]
fn rit_offset_down_encodes() {
    let cmd = MoveRitOffsetDown { hz: 50 };
    assert_eq!(cmd.to_message().unwrap(), b"RD0050;".to_vec());
}

#[test]
fn rit_offset_down_accepts_boundary_values() {
    assert!(MoveRitOffsetDown { hz: 0 }.validate().is_ok());
    assert!(MoveRitOffsetDown { hz: 9999 }.validate().is_ok());
}

#[test]
fn rit_offset_down_rejects_out_of_range() {
    assert!(matches!(
        MoveRitOffsetDown { hz: 10000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// RG: GetRfGainA, GetRfGainB, SetRfGainA, SetRfGainB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_rf_gain_a_encodes() {
    assert_eq!(GetVfoARfGain.to_message().unwrap(), b"RG;".to_vec());
}

#[test]
fn get_rf_gain_b_encodes() {
    assert_eq!(GetVfoBRfGain.to_message().unwrap(), b"RG$;".to_vec());
}

#[test]
fn set_rf_gain_a_encodes() {
    let cmd = SetVfoARfGain { gain: 220 };
    assert_eq!(cmd.to_message().unwrap(), b"RG220;".to_vec());
}

#[test]
fn set_rf_gain_a_accepts_boundary_values() {
    assert!(SetVfoARfGain { gain: 190 }.validate().is_ok());
    assert!(SetVfoARfGain { gain: 250 }.validate().is_ok());
}

#[test]
fn set_rf_gain_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoARfGain { gain: 189 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoARfGain { gain: 251 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_rf_gain_b_encodes() {
    let cmd = SetVfoBRfGain { gain: 200 };
    assert_eq!(cmd.to_message().unwrap(), b"RG$200;".to_vec());
}

#[test]
fn set_rf_gain_b_accepts_boundary_values() {
    assert!(SetVfoBRfGain { gain: 190 }.validate().is_ok());
    assert!(SetVfoBRfGain { gain: 250 }.validate().is_ok());
}

#[test]
fn set_rf_gain_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBRfGain { gain: 189 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoBRfGain { gain: 251 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// RO: GetRitXitOffset, SetRitXitOffset
// ------------------------------------------------------------------------------------------------

#[test]
fn get_rit_xit_offset_encodes() {
    assert_eq!(GetRitXitOffset.to_message().unwrap(), b"RO;".to_vec());
}

#[test]
fn set_rit_xit_offset_encodes() {
    let cmd = SetRitXitOffset { offset_hz: 500 };
    let mut expected = b"RO".to_vec();
    expected.extend(expected_signed_offset_4(500));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_rit_xit_offset_accepts_boundary_values() {
    assert!(SetRitXitOffset { offset_hz: 9999 }.validate().is_ok());
    assert!(SetRitXitOffset { offset_hz: -9999 }.validate().is_ok());
}

#[test]
fn set_rit_xit_offset_rejects_out_of_range() {
    assert!(matches!(
        SetRitXitOffset { offset_hz: 10000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetRitXitOffset { offset_hz: -10000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// RT: GetRitControl, SetRitControl
// ------------------------------------------------------------------------------------------------

#[test]
fn get_rit_control_encodes() {
    assert_eq!(GetRitControl.to_message().unwrap(), b"RT;".to_vec());
}

#[test]
fn set_rit_control_encodes_on() {
    let cmd = SetRitControl { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"RT1;".to_vec());
}

#[test]
fn set_rit_control_encodes_off() {
    let cmd = SetRitControl { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"RT0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// RU: RitOffsetUp
// ------------------------------------------------------------------------------------------------

#[test]
fn rit_offset_up_encodes() {
    let cmd = MoveRitOffsetUp { hz: 75 };
    assert_eq!(cmd.to_message().unwrap(), b"RU0075;".to_vec());
}

#[test]
fn rit_offset_up_accepts_boundary_values() {
    assert!(MoveRitOffsetUp { hz: 0 }.validate().is_ok());
    assert!(MoveRitOffsetUp { hz: 9999 }.validate().is_ok());
}

#[test]
fn rit_offset_up_rejects_out_of_range() {
    assert!(matches!(
        MoveRitOffsetUp { hz: 10000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// RV: GetFirmwareRevisions
// ------------------------------------------------------------------------------------------------

#[test]
fn get_firmware_revisions_encodes() {
    assert_eq!(GetFirmwareRevision.to_message().unwrap(), b"RV;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// RX: GoToReceive
// ------------------------------------------------------------------------------------------------

#[test]
fn go_to_receive_encodes() {
    assert_eq!(GoToReceive.to_message().unwrap(), b"RX;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SB: GetSubReceiver, SetSubReceiver
// ------------------------------------------------------------------------------------------------

#[test]
fn get_sub_receiver_encodes() {
    assert_eq!(GetSubReceiver.to_message().unwrap(), b"SB;".to_vec());
}

#[test]
fn set_sub_receiver_encodes_on() {
    let cmd = SetSubReceiver { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"SB1;".to_vec());
}

#[test]
fn set_sub_receiver_encodes_off() {
    let cmd = SetSubReceiver { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"SB0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SD: GetQskDelay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_qsk_delay_encodes() {
    assert_eq!(GetQskDelay.to_message().unwrap(), b"SD;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SM: GetSMeterA, GetSMeterB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_s_meter_a_encodes() {
    assert_eq!(GetVfoASMeter.to_message().unwrap(), b"SM;".to_vec());
}

#[test]
fn get_s_meter_b_encodes() {
    assert_eq!(GetVfoBSMeter.to_message().unwrap(), b"SM$;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SMH: GetHighResolutionSMeter
// ------------------------------------------------------------------------------------------------

#[test]
fn get_high_resolution_s_meter_encodes() {
    assert_eq!(
        GetHighResolutionSMeter.to_message().unwrap(),
        b"SMH;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SQ: GetSquelchA, GetSquelchB, SetSquelchA, SetSquelchB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_squelch_a_encodes() {
    assert_eq!(GetVfoASquelch.to_message().unwrap(), b"SQ;".to_vec());
}

#[test]
fn get_squelch_b_encodes() {
    assert_eq!(GetVfoBSquelch.to_message().unwrap(), b"SQ$;".to_vec());
}

#[test]
fn set_squelch_a_encodes() {
    let cmd = SetVfoASquelch { level: 10 };
    assert_eq!(cmd.to_message().unwrap(), b"SQ10;".to_vec());
}

#[test]
fn set_squelch_a_accepts_boundary_values() {
    assert!(SetVfoASquelch { level: 0 }.validate().is_ok());
    assert!(SetVfoASquelch { level: 29 }.validate().is_ok());
}

#[test]
fn set_squelch_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoASquelch { level: 30 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_squelch_b_encodes() {
    let cmd = SetVfoBSquelch { level: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"SQ$05;".to_vec());
}

#[test]
fn set_squelch_b_accepts_boundary_values() {
    assert!(SetVfoBSquelch { level: 0 }.validate().is_ok());
    assert!(SetVfoBSquelch { level: 29 }.validate().is_ok());
}

#[test]
fn set_squelch_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBSquelch { level: 30 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// SWT / SWH: EmulateButtonTap, EmulateButtonHold
// ------------------------------------------------------------------------------------------------

#[test]
fn emulate_button_tap_encodes() {
    let cmd = EmulateButtonTap { button: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"SWT05;".to_vec());
}

#[test]
fn emulate_button_hold_encodes() {
    let cmd = EmulateButtonHold { button: 12 };
    assert_eq!(cmd.to_message().unwrap(), b"SWH12;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// TB / TBX: GetBufferedText, GetTxBufferedText
// ------------------------------------------------------------------------------------------------

#[test]
fn get_buffered_text_encodes() {
    assert_eq!(GetBufferedText.to_message().unwrap(), b"TB;".to_vec());
}

#[test]
fn get_tx_buffered_text_encodes() {
    assert_eq!(
        GetTransmitBufferedText.to_message().unwrap(),
        b"TBX;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// TE: SetTxEqualizer
// ------------------------------------------------------------------------------------------------

#[test]
fn set_tx_equalizer_encodes() {
    let cmd = SetTransmitEqualizer {
        params: vec![1, 2, 3],
    };
    assert_eq!(cmd.to_message().unwrap(), vec![b'T', b'E', 1, 2, 3, b';']);
}

// ------------------------------------------------------------------------------------------------
// TM: GetTxMeterMode, SetTxMeterMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_meter_mode_encodes() {
    assert_eq!(GetTransmitMeterMode.to_message().unwrap(), b"TM;".to_vec());
}

#[test]
fn set_tx_meter_mode_encodes() {
    let cmd = SetTransmitMeterMode { mode: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"TM3;".to_vec());
}

#[test]
fn set_tx_meter_mode_accepts_boundary_values() {
    assert!(SetTransmitMeterMode { mode: 0 }.validate().is_ok());
    assert!(SetTransmitMeterMode { mode: 5 }.validate().is_ok());
}

#[test]
fn set_tx_meter_mode_rejects_out_of_range() {
    assert!(matches!(
        SetTransmitMeterMode { mode: 6 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// TQ: GetTransmitStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transmit_status_encodes() {
    assert_eq!(GetTransmitState.to_message().unwrap(), b"TQ;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// TT: SetTextToTerminal
// ------------------------------------------------------------------------------------------------

#[test]
fn set_text_to_terminal_encodes_on() {
    let cmd = SetTextToTerminal { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"TT1;".to_vec());
}

#[test]
fn set_text_to_terminal_encodes_off() {
    let cmd = SetTextToTerminal { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"TT0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// TX: GoToTransmit
// ------------------------------------------------------------------------------------------------

#[test]
fn go_to_transmit_encodes() {
    assert_eq!(GoToTransmit.to_message().unwrap(), b"TX;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// UP: VfoAUp, VfoBUp
// ------------------------------------------------------------------------------------------------

#[test]
fn vfo_a_up_encodes() {
    assert_eq!(
        MoveVfoAFrequencyUp {
            step: Some(VfoFrequencyChangeStep::Step10Hz)
        }
        .to_message()
        .unwrap(),
        b"UP1;".to_vec()
    );
}

#[test]
fn vfo_b_up_encodes() {
    assert_eq!(
        MoveVfoBFrequencyUp {
            step: Some(VfoFrequencyChangeStep::Step10Hz)
        }
        .to_message()
        .unwrap(),
        b"UPB1;".to_vec()
    );
}

#[test]
fn vfo_a_up_encodes_none() {
    assert_eq!(
        MoveVfoAFrequencyUp { step: None }.to_message().unwrap(),
        b"UP;".to_vec()
    );
}

#[test]
fn vfo_b_up_encodes_none() {
    assert_eq!(
        MoveVfoBFrequencyUp { step: None }.to_message().unwrap(),
        b"UPB;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// VX: GetVox, SetVox
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vox_encodes() {
    assert_eq!(GetVox.to_message().unwrap(), b"VX;".to_vec());
}

#[test]
fn set_vox_encodes_on() {
    let cmd = SetVox { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"VX1;".to_vec());
}

#[test]
fn set_vox_encodes_off() {
    let cmd = SetVox { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"VX0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// XF: GetXfilNumberA, GetXfilNumberB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_xfil_number_a_encodes() {
    assert_eq!(GetVfoAXfilNumber.to_message().unwrap(), b"XF;".to_vec());
}

#[test]
fn get_xfil_number_b_encodes() {
    assert_eq!(GetVfoBXfilNumber.to_message().unwrap(), b"XF$;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// XT: GetXitControl, SetXitControl
// ------------------------------------------------------------------------------------------------

#[test]
fn get_xit_control_encodes() {
    assert_eq!(GetXitControl.to_message().unwrap(), b"XT;".to_vec());
}

#[test]
fn set_xit_control_encodes_on() {
    let cmd = SetXitControl { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"XT1;".to_vec());
}

#[test]
fn set_xit_control_encodes_off() {
    let cmd = SetXitControl { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"XT0;".to_vec());
}
