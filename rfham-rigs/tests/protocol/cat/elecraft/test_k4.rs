//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::k4`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.
//!
//! # `ManualNotch`, `NoiseReduction`, `RepeaterOffset`
//!
//! These are response-only data structs with no `Command` impl of their own (only the
//! corresponding `Get*` command types, tested below, implement `Command`), so there is nothing to
//! encode and they are skipped here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command,
        cat::elecraft::k4::{
            AtuMode, CaptureScreenshot, CenterPanadapterOnVfoA, CenterPanadapterOnVfoB,
            CopyVfoAtoVfoB, DigitalAudioRoutingMode, GetActiveSoftwareReleaseChannel, GetAtuMode,
            GetAudioLineInputLevel, GetAudioLineOutputLevel, GetAudioMixRatio,
            GetBandIndependenceState, GetCoarseTuningStep, GetCurrentBandPowerLimit,
            GetDigitalAudioRoutingMode, GetDigitalOutputPin1State, GetErrorReportingState,
            GetKeyerPaddleEmulationMode, GetMicInputSource, GetPowerStatus, GetRepeaterOffset,
            GetScreenCount, GetStreamingLatencyClass, GetTransceiverId, GetTransceiverSerialNumber,
            GetTransmitDataBandwidth, GetTransmitGain, GetTransmitGainConstant,
            GetTransmitTestModeState, GetUtcTimestamp, GetVfoAAgcMode, GetVfoAAutoNotchState,
            GetVfoACtssTone, GetVfoAFilterPresetSlot, GetVfoAIfCenterPitch,
            GetVfoAManualNotchSettings, GetVfoAModeAlternates, GetVfoANoiseReductionSettings,
            GetVfoATextDecodeMode, GetVfoATransverterActiveBandSlot, GetVfoATransverterOffset,
            GetVfoATuningStep, GetVfoBAgcMode, GetVfoBAutoNotchState, GetVfoBCtssTone,
            GetVfoBFilterPresetSlot, GetVfoBIfCenterPitch, GetVfoBManualNotchSettings,
            GetVfoBModeAlternates, GetVfoBNoiseReductionSettings, GetVfoBTextDecodeMode,
            GetVfoBTransverterActiveBandSlot, GetVfoBTuningStep, GetVoxGain, GetVoxInhibitState,
            GetWattmeterCalibrationConstant, KeyerPaddleEmulationMode, MicInputSource,
            PlayDvrMessage, PowerStatus, RepeaterOffsetDirection, SetActiveSoftwareReleaseChannel,
            SetAtuMode, SetAtuTuningState, SetAudioLineInputLevel, SetAudioLineOutputLevel,
            SetAudioMixRatio, SetBandIndependenceState, SetCoarseTuningStep, SetCommandEchoState,
            SetCwSidetonePitch, SetDigitalAudioRoutingMode, SetDigitalOutputPin1State,
            SetErrorReportingState, SetK4QskOrVoxDelay, SetKeyerPaddleEmulationMode, SetKeyerSpeed,
            SetMicInputSource, SetPowerStatus, SetRepeaterOffset, SetStreamingLatencyClass,
            SetSystemAutoInfoInterval, SetTransmitDataBandwidth, SetTransmitTestModeState,
            SetVfoAAgcMode, SetVfoAAutoNotchState, SetVfoACtssTone, SetVfoAFilterPresetSlot,
            SetVfoAManualNotchSettings, SetVfoANoiseReductionSettings, SetVfoATextDecodeMode,
            SetVfoATransverterActiveBandSlot, SetVfoATuningStep, SetVfoBAgcMode,
            SetVfoBAutoNotchState, SetVfoBCtssTone, SetVfoBFilterPresetSlot,
            SetVfoBManualNotchSettings, SetVfoBNoiseReductionSettings, SetVfoBTextDecodeMode,
            SetVfoBTransverterActiveBandSlot, SetVfoBTuningStep, SetVoxGain, SetVoxInhibitState,
            SetWattmeterCalibrationConstant, SoftwareReleaseChannel, SwapVfoAandVfoB,
            TextDecodeMode,
        },
    },
};

/// Mirrors the sign encoding produced by the private `format_i16_ascii_4` helper (shared with
/// `k3_kx`, used here via `manual_notch_argument_bytes`): a literal `+`/`-` character followed by
/// the zero-padded magnitude. (An earlier version of this helper printed the sign byte's
/// *decimal ASCII code point* — `43`/`45` — instead of the character itself; that bug has since
/// been fixed in `format_i16_ascii_4`.)
fn expected_signed_offset_4(n: i16) -> Vec<u8> {
    let sign = if n.is_negative() { "-" } else { "+" };
    format!("{sign}{:04}", n.unsigned_abs()).into_bytes()
}

// ------------------------------------------------------------------------------------------------
// AB: CopyVfoAToB, SwapVfoAB
// ------------------------------------------------------------------------------------------------

#[test]
fn copy_vfo_a_to_b_encodes() {
    assert_eq!(CopyVfoAtoVfoB.to_message().unwrap(), b"AB0;".to_vec());
}

#[test]
fn swap_vfo_ab_encodes() {
    assert_eq!(SwapVfoAandVfoB.to_message().unwrap(), b"AB1;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// AT: GetAtuMode, SetAtuMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_atu_mode_encodes() {
    assert_eq!(GetAtuMode.to_message().unwrap(), b"AT;".to_vec());
}

#[test]
fn set_atu_mode_encodes() {
    let cmd = SetAtuMode {
        mode: AtuMode::Inline,
    };
    assert_eq!(cmd.to_message().unwrap(), b"AT1;".to_vec());
}

#[test]
fn set_atu_mode_accepts_boundary_values() {
    assert!(
        SetAtuMode {
            mode: AtuMode::Bypassed
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetAtuMode {
            mode: AtuMode::Tuning
        }
        .validate()
        .is_ok()
    );
}

// ------------------------------------------------------------------------------------------------
// BI: GetBandIndependence, SetBandIndependence
// ------------------------------------------------------------------------------------------------

#[test]
fn get_band_independence_encodes() {
    assert_eq!(
        GetBandIndependenceState.to_message().unwrap(),
        b"BI;".to_vec()
    );
}

#[test]
fn set_band_independence_encodes_on() {
    let cmd = SetBandIndependenceState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"BI1;".to_vec());
}

#[test]
fn set_band_independence_encodes_off() {
    let cmd = SetBandIndependenceState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"BI0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// CW: SetCwSidetonePitch
// ------------------------------------------------------------------------------------------------

#[test]
fn set_cw_sidetone_pitch_encodes() {
    let cmd = SetCwSidetonePitch { pitch_hz: 500 };
    assert_eq!(cmd.to_message().unwrap(), b"CW500;".to_vec());
}

#[test]
fn set_cw_sidetone_pitch_accepts_boundary_values() {
    assert!(SetCwSidetonePitch { pitch_hz: 300 }.validate().is_ok());
    assert!(SetCwSidetonePitch { pitch_hz: 800 }.validate().is_ok());
}

#[test]
fn set_cw_sidetone_pitch_rejects_out_of_range() {
    assert!(matches!(
        SetCwSidetonePitch { pitch_hz: 299 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetCwSidetonePitch { pitch_hz: 801 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// DA: GetDigitalAudio, SetDigitalAudio
// ------------------------------------------------------------------------------------------------

#[test]
fn get_digital_audio_encodes() {
    assert_eq!(
        GetDigitalAudioRoutingMode.to_message().unwrap(),
        b"DA;".to_vec()
    );
}

#[test]
fn set_digital_audio_encodes() {
    let cmd = SetDigitalAudioRoutingMode {
        mode: DigitalAudioRoutingMode::DigitalOut,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DA2;".to_vec());
}

#[test]
fn set_digital_audio_accepts_boundary_values() {
    assert!(
        SetDigitalAudioRoutingMode {
            mode: DigitalAudioRoutingMode::Analog
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetDigitalAudioRoutingMode {
            mode: DigitalAudioRoutingMode::FullDigital
        }
        .validate()
        .is_ok()
    );
}

// ------------------------------------------------------------------------------------------------
// DO: GetDigOut1, SetDigOut1
// ------------------------------------------------------------------------------------------------

#[test]
fn get_dig_out_1_encodes() {
    assert_eq!(
        GetDigitalOutputPin1State.to_message().unwrap(),
        b"DO1;".to_vec()
    );
}

#[test]
fn set_dig_out_1_encodes_high() {
    let cmd = SetDigitalOutputPin1State { high: true };
    assert_eq!(cmd.to_message().unwrap(), b"DO1;".to_vec());
}

#[test]
fn set_dig_out_1_encodes_low() {
    let cmd = SetDigitalOutputPin1State { high: false };
    assert_eq!(cmd.to_message().unwrap(), b"DO1;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// DW: GetTxDataBandwidth, SetTxDataBandwidth
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_data_bandwidth_encodes() {
    assert_eq!(
        GetTransmitDataBandwidth.to_message().unwrap(),
        b"DW;".to_vec()
    );
}

#[test]
fn set_tx_data_bandwidth_encodes() {
    let cmd = SetTransmitDataBandwidth {
        bandwidth_10hz: 250,
    };
    assert_eq!(cmd.to_message().unwrap(), b"DW0250;".to_vec());
}

#[test]
fn set_tx_data_bandwidth_accepts_boundary_values() {
    assert!(
        SetTransmitDataBandwidth { bandwidth_10hz: 0 }
            .validate()
            .is_ok()
    );
    assert!(
        SetTransmitDataBandwidth {
            bandwidth_10hz: 9999
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_tx_data_bandwidth_rejects_out_of_range() {
    assert!(matches!(
        SetTransmitDataBandwidth {
            bandwidth_10hz: 10000
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// EC: SetCommandEcho
// ------------------------------------------------------------------------------------------------

#[test]
fn set_command_echo_encodes_on() {
    let cmd = SetCommandEchoState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"EC1;".to_vec());
}

#[test]
fn set_command_echo_encodes_off() {
    let cmd = SetCommandEchoState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"EC0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// ER: GetErrorReporting, SetErrorReporting
// ------------------------------------------------------------------------------------------------

#[test]
fn get_error_reporting_encodes() {
    assert_eq!(
        GetErrorReportingState.to_message().unwrap(),
        b"ER;".to_vec()
    );
}

#[test]
fn set_error_reporting_encodes_on() {
    let cmd = SetErrorReportingState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"ER1;".to_vec());
}

#[test]
fn set_error_reporting_encodes_off() {
    let cmd = SetErrorReportingState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"ER0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// FC: CenterPanadapterA, CenterPanadapterB
// ------------------------------------------------------------------------------------------------

#[test]
fn center_panadapter_a_encodes() {
    assert_eq!(
        CenterPanadapterOnVfoA.to_message().unwrap(),
        b"FC;".to_vec()
    );
}

#[test]
fn center_panadapter_b_encodes() {
    assert_eq!(
        CenterPanadapterOnVfoB.to_message().unwrap(),
        b"FC$;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// FP: GetFilterPresetA, GetFilterPresetB, SetFilterPresetA, SetFilterPresetB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_filter_preset_a_encodes() {
    assert_eq!(
        GetVfoAFilterPresetSlot.to_message().unwrap(),
        b"FP;".to_vec()
    );
}

#[test]
fn get_filter_preset_b_encodes() {
    assert_eq!(
        GetVfoBFilterPresetSlot.to_message().unwrap(),
        b"FP$;".to_vec()
    );
}

#[test]
fn set_filter_preset_a_encodes() {
    let cmd = SetVfoAFilterPresetSlot { preset: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"FP3;".to_vec());
}

#[test]
fn set_filter_preset_a_accepts_boundary_values() {
    assert!(SetVfoAFilterPresetSlot { preset: 1 }.validate().is_ok());
    assert!(SetVfoAFilterPresetSlot { preset: 8 }.validate().is_ok());
}

#[test]
fn set_filter_preset_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoAFilterPresetSlot { preset: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoAFilterPresetSlot { preset: 9 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_filter_preset_b_encodes() {
    let cmd = SetVfoBFilterPresetSlot { preset: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"FP$5;".to_vec());
}

#[test]
fn set_filter_preset_b_accepts_boundary_values() {
    assert!(SetVfoBFilterPresetSlot { preset: 1 }.validate().is_ok());
    assert!(SetVfoBFilterPresetSlot { preset: 8 }.validate().is_ok());
}

#[test]
fn set_filter_preset_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBFilterPresetSlot { preset: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetVfoBFilterPresetSlot { preset: 9 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GT: GetK4AgcModeA, GetK4AgcModeB, SetK4AgcModeA, SetK4AgcModeB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_k4_agc_mode_a_encodes() {
    assert_eq!(GetVfoAAgcMode.to_message().unwrap(), b"GT;".to_vec());
}

#[test]
fn get_k4_agc_mode_b_encodes() {
    assert_eq!(GetVfoBAgcMode.to_message().unwrap(), b"GT$;".to_vec());
}

#[test]
fn set_k4_agc_mode_a_encodes() {
    let cmd = SetVfoAAgcMode { mode: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"GT01;".to_vec());
}

#[test]
fn set_k4_agc_mode_a_accepts_boundary_values() {
    assert!(SetVfoAAgcMode { mode: 0 }.validate().is_ok());
    assert!(SetVfoAAgcMode { mode: 3 }.validate().is_ok());
}

#[test]
fn set_k4_agc_mode_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoAAgcMode { mode: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_k4_agc_mode_b_encodes() {
    let cmd = SetVfoBAgcMode { mode: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"GT$02;".to_vec());
}

#[test]
fn set_k4_agc_mode_b_accepts_boundary_values() {
    assert!(SetVfoBAgcMode { mode: 0 }.validate().is_ok());
    assert!(SetVfoBAgcMode { mode: 3 }.validate().is_ok());
}

#[test]
fn set_k4_agc_mode_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBAgcMode { mode: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// ID: GetRadioId
// ------------------------------------------------------------------------------------------------

#[test]
fn get_radio_id_encodes() {
    assert_eq!(GetTransceiverId.to_message().unwrap(), b"ID;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// IS: GetK4IfCenterPitchA, GetK4IfCenterPitchB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_k4_if_center_pitch_a_encodes() {
    assert_eq!(GetVfoAIfCenterPitch.to_message().unwrap(), b"IS;".to_vec());
}

#[test]
fn get_k4_if_center_pitch_b_encodes() {
    assert_eq!(GetVfoBIfCenterPitch.to_message().unwrap(), b"IS$;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// KP: GetKeyerPaddle, SetKeyerPaddle
// ------------------------------------------------------------------------------------------------

#[test]
fn get_keyer_paddle_encodes() {
    assert_eq!(
        GetKeyerPaddleEmulationMode.to_message().unwrap(),
        b"KP;".to_vec()
    );
}

#[test]
fn set_keyer_paddle_encodes_normal() {
    let cmd = SetKeyerPaddleEmulationMode {
        mode: KeyerPaddleEmulationMode::Normal,
    };
    assert_eq!(cmd.to_message().unwrap(), b"KP0;".to_vec());
}

#[test]
fn set_keyer_paddle_encodes_dit_only() {
    let cmd = SetKeyerPaddleEmulationMode {
        mode: KeyerPaddleEmulationMode::DitOnly,
    };
    assert_eq!(cmd.to_message().unwrap(), b"KP1;".to_vec());
}

#[test]
fn set_keyer_paddle_encodes_dah_only() {
    let cmd = SetKeyerPaddleEmulationMode {
        mode: KeyerPaddleEmulationMode::DahOnly,
    };
    assert_eq!(cmd.to_message().unwrap(), b"KP2;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// KS: SetK4KeyerSpeed
// ------------------------------------------------------------------------------------------------

#[test]
fn set_k4_keyer_speed_encodes() {
    let cmd = SetKeyerSpeed { wpm: 40 };
    assert_eq!(cmd.to_message().unwrap(), b"KS040;".to_vec());
}

#[test]
fn set_k4_keyer_speed_accepts_boundary_values() {
    assert!(SetKeyerSpeed { wpm: 8 }.validate().is_ok());
    assert!(SetKeyerSpeed { wpm: 100 }.validate().is_ok());
}

#[test]
fn set_k4_keyer_speed_rejects_out_of_range() {
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
// LI: GetLineInput, SetLineInput
// ------------------------------------------------------------------------------------------------

#[test]
fn get_line_input_encodes() {
    assert_eq!(
        GetAudioLineInputLevel.to_message().unwrap(),
        b"LI;".to_vec()
    );
}

#[test]
fn set_line_input_encodes() {
    let cmd = SetAudioLineInputLevel { level: 30 };
    assert_eq!(cmd.to_message().unwrap(), b"LI030;".to_vec());
}

#[test]
fn set_line_input_accepts_boundary_values() {
    assert!(SetAudioLineInputLevel { level: 0 }.validate().is_ok());
    assert!(SetAudioLineInputLevel { level: 60 }.validate().is_ok());
}

#[test]
fn set_line_input_rejects_out_of_range() {
    assert!(matches!(
        SetAudioLineInputLevel { level: 61 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// LO: GetLineOutput, SetLineOutput
// ------------------------------------------------------------------------------------------------

#[test]
fn get_line_output_encodes() {
    assert_eq!(
        GetAudioLineOutputLevel.to_message().unwrap(),
        b"LO;".to_vec()
    );
}

#[test]
fn set_line_output_encodes() {
    let cmd = SetAudioLineOutputLevel { level: 45 };
    assert_eq!(cmd.to_message().unwrap(), b"LO045;".to_vec());
}

#[test]
fn set_line_output_accepts_boundary_values() {
    assert!(SetAudioLineOutputLevel { level: 0 }.validate().is_ok());
    assert!(SetAudioLineOutputLevel { level: 60 }.validate().is_ok());
}

#[test]
fn set_line_output_rejects_out_of_range() {
    assert!(matches!(
        SetAudioLineOutputLevel { level: 61 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// MA: GetModeAlternatesA, GetModeAlternatesB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_mode_alternates_a_encodes() {
    assert_eq!(GetVfoAModeAlternates.to_message().unwrap(), b"MA;".to_vec());
}

#[test]
fn get_mode_alternates_b_encodes() {
    assert_eq!(
        GetVfoBModeAlternates.to_message().unwrap(),
        b"MA$;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// MI: GetMicInput, SetMicInput
// ------------------------------------------------------------------------------------------------

#[test]
fn get_mic_input_encodes() {
    assert_eq!(GetMicInputSource.to_message().unwrap(), b"MI;".to_vec());
}

#[test]
fn set_mic_input_encodes_front() {
    let cmd = SetMicInputSource {
        input: MicInputSource::Front,
    };
    assert_eq!(cmd.to_message().unwrap(), b"MI0;".to_vec());
}

#[test]
fn set_mic_input_encodes_rear() {
    let cmd = SetMicInputSource {
        input: MicInputSource::Rear,
    };
    assert_eq!(cmd.to_message().unwrap(), b"MI1;".to_vec());
}

#[test]
fn set_mic_input_encodes_usb() {
    let cmd = SetMicInputSource {
        input: MicInputSource::Usb,
    };
    assert_eq!(cmd.to_message().unwrap(), b"MI2;".to_vec());
}

#[test]
fn set_mic_input_encodes_bluetooth() {
    let cmd = SetMicInputSource {
        input: MicInputSource::Bluetooth,
    };
    assert_eq!(cmd.to_message().unwrap(), b"MI3;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// MX: GetAudioMix, SetAudioMix
// ------------------------------------------------------------------------------------------------

#[test]
fn get_audio_mix_encodes() {
    assert_eq!(GetAudioMixRatio.to_message().unwrap(), b"MX;".to_vec());
}

#[test]
fn set_audio_mix_encodes() {
    let cmd = SetAudioMixRatio { ratio: 50 };
    assert_eq!(cmd.to_message().unwrap(), b"MX50;".to_vec());
}

#[test]
fn set_audio_mix_accepts_boundary_values() {
    assert!(SetAudioMixRatio { ratio: 0 }.validate().is_ok());
    assert!(SetAudioMixRatio { ratio: 99 }.validate().is_ok());
}

#[test]
fn set_audio_mix_rejects_out_of_range() {
    assert!(matches!(
        SetAudioMixRatio { ratio: 100 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// NA: GetAutoNotchA, GetAutoNotchB, SetAutoNotchA, SetAutoNotchB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_auto_notch_a_encodes() {
    assert_eq!(GetVfoAAutoNotchState.to_message().unwrap(), b"NA;".to_vec());
}

#[test]
fn get_auto_notch_b_encodes() {
    assert_eq!(
        GetVfoBAutoNotchState.to_message().unwrap(),
        b"NA$;".to_vec()
    );
}

#[test]
fn set_auto_notch_a_encodes_on() {
    let cmd = SetVfoAAutoNotchState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"NA1;".to_vec());
}

#[test]
fn set_auto_notch_a_encodes_off() {
    let cmd = SetVfoAAutoNotchState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"NA0;".to_vec());
}

#[test]
fn set_auto_notch_b_encodes_on() {
    let cmd = SetVfoBAutoNotchState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"NA$1;".to_vec());
}

#[test]
fn set_auto_notch_b_encodes_off() {
    let cmd = SetVfoBAutoNotchState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"NA$0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// NM: GetManualNotchA, GetManualNotchB, SetManualNotchA, SetManualNotchB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_manual_notch_a_encodes() {
    assert_eq!(
        GetVfoAManualNotchSettings.to_message().unwrap(),
        b"NM;".to_vec()
    );
}

#[test]
fn get_manual_notch_b_encodes() {
    assert_eq!(
        GetVfoBManualNotchSettings.to_message().unwrap(),
        b"NM$;".to_vec()
    );
}

#[test]
fn set_manual_notch_a_encodes() {
    let cmd = SetVfoAManualNotchSettings {
        state: true,
        offset_hz: 300,
    };
    let mut expected = b"NM10".to_vec();
    expected.extend(expected_signed_offset_4(300));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_manual_notch_a_encodes_off_state() {
    let cmd = SetVfoAManualNotchSettings {
        state: false,
        offset_hz: -150,
    };
    let mut expected = b"NM00".to_vec();
    expected.extend(expected_signed_offset_4(-150));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_manual_notch_a_accepts_boundary_values() {
    assert!(
        SetVfoAManualNotchSettings {
            state: true,
            offset_hz: 9999
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetVfoAManualNotchSettings {
            state: true,
            offset_hz: -9999
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_manual_notch_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoAManualNotchSettings {
            state: true,
            offset_hz: 10000
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_manual_notch_b_encodes() {
    let cmd = SetVfoBManualNotchSettings {
        state: true,
        offset_hz: 300,
    };
    let mut expected = b"NM$10".to_vec();
    expected.extend(expected_signed_offset_4(300));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_manual_notch_b_accepts_boundary_values() {
    assert!(
        SetVfoBManualNotchSettings {
            state: true,
            offset_hz: 9999
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetVfoBManualNotchSettings {
            state: true,
            offset_hz: -9999
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_manual_notch_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBManualNotchSettings {
            state: true,
            offset_hz: -10000
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// NR: GetNoiseReductionA, GetNoiseReductionB, SetNoiseReductionA, SetNoiseReductionB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_noise_reduction_a_encodes() {
    assert_eq!(
        GetVfoANoiseReductionSettings.to_message().unwrap(),
        b"NR;".to_vec()
    );
}

#[test]
fn get_noise_reduction_b_encodes() {
    assert_eq!(
        GetVfoBNoiseReductionSettings.to_message().unwrap(),
        b"NR$;".to_vec()
    );
}

#[test]
fn set_noise_reduction_a_encodes() {
    let cmd = SetVfoANoiseReductionSettings {
        state: true,
        level: 5,
    };
    assert_eq!(cmd.to_message().unwrap(), b"NR15;".to_vec());
}

#[test]
fn set_noise_reduction_a_encodes_off() {
    let cmd = SetVfoANoiseReductionSettings {
        state: false,
        level: 0,
    };
    assert_eq!(cmd.to_message().unwrap(), b"NR00;".to_vec());
}

#[test]
fn set_noise_reduction_a_accepts_boundary_values() {
    assert!(
        SetVfoANoiseReductionSettings {
            state: true,
            level: 0
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetVfoANoiseReductionSettings {
            state: true,
            level: 9
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_noise_reduction_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoANoiseReductionSettings {
            state: true,
            level: 10
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_noise_reduction_b_encodes() {
    let cmd = SetVfoBNoiseReductionSettings {
        state: true,
        level: 7,
    };
    assert_eq!(cmd.to_message().unwrap(), b"NR$17;".to_vec());
}

#[test]
fn set_noise_reduction_b_accepts_boundary_values() {
    assert!(
        SetVfoBNoiseReductionSettings {
            state: true,
            level: 0
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetVfoBNoiseReductionSettings {
            state: true,
            level: 9
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_noise_reduction_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBNoiseReductionSettings {
            state: true,
            level: 10
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// PB: PlayDvrMessage
// ------------------------------------------------------------------------------------------------

#[test]
fn play_dvr_message_encodes() {
    let cmd = PlayDvrMessage { message: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"PB3;".to_vec());
}

#[test]
fn play_dvr_message_accepts_boundary_values() {
    assert!(PlayDvrMessage { message: 0 }.validate().is_ok());
    assert!(PlayDvrMessage { message: 8 }.validate().is_ok());
}

#[test]
fn play_dvr_message_rejects_out_of_range() {
    assert!(matches!(
        PlayDvrMessage { message: 9 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// PL: GetPlToneA, GetPlToneB, SetPlToneA, SetPlToneB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_pl_tone_a_encodes() {
    assert_eq!(GetVfoACtssTone.to_message().unwrap(), b"PL;".to_vec());
}

#[test]
fn get_pl_tone_b_encodes() {
    assert_eq!(GetVfoBCtssTone.to_message().unwrap(), b"PL$;".to_vec());
}

#[test]
fn set_pl_tone_a_encodes() {
    let cmd = SetVfoACtssTone { tone_code: 12 };
    assert_eq!(cmd.to_message().unwrap(), b"PL012;".to_vec());
}

#[test]
fn set_pl_tone_a_accepts_boundary_values() {
    assert!(SetVfoACtssTone { tone_code: 0 }.validate().is_ok());
    assert!(SetVfoACtssTone { tone_code: 38 }.validate().is_ok());
}

#[test]
fn set_pl_tone_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoACtssTone { tone_code: 39 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_pl_tone_b_encodes() {
    let cmd = SetVfoBCtssTone { tone_code: 8 };
    assert_eq!(cmd.to_message().unwrap(), b"PL$008;".to_vec());
}

#[test]
fn set_pl_tone_b_accepts_boundary_values() {
    assert!(SetVfoBCtssTone { tone_code: 0 }.validate().is_ok());
    assert!(SetVfoBCtssTone { tone_code: 38 }.validate().is_ok());
}

#[test]
fn set_pl_tone_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBCtssTone { tone_code: 39 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// PP: GetPerBandPower
// ------------------------------------------------------------------------------------------------

#[test]
fn get_per_band_power_encodes() {
    assert_eq!(
        GetCurrentBandPowerLimit.to_message().unwrap(),
        b"PP;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// PS: GetK4PowerStatus, SetK4PowerStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_k4_power_status_encodes() {
    assert_eq!(GetPowerStatus.to_message().unwrap(), b"PS;".to_vec());
}

#[test]
fn set_k4_power_status_encodes_off() {
    let cmd = SetPowerStatus {
        state: PowerStatus::PowerOff,
    };
    assert_eq!(cmd.to_message().unwrap(), b"PS0;".to_vec());
}

#[test]
fn set_k4_power_status_encodes_on() {
    let cmd = SetPowerStatus {
        state: PowerStatus::PowerOn,
    };
    assert_eq!(cmd.to_message().unwrap(), b"PS1;".to_vec());
}

#[test]
fn set_k4_power_status_encodes_firmware_restart() {
    let cmd = SetPowerStatus {
        state: PowerStatus::FirmwareRestart,
    };
    assert_eq!(cmd.to_message().unwrap(), b"PS2;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// RL: GetSoftwareRelease, SetSoftwareRelease
// ------------------------------------------------------------------------------------------------

#[test]
fn get_software_release_encodes() {
    assert_eq!(
        GetActiveSoftwareReleaseChannel.to_message().unwrap(),
        b"RL;".to_vec()
    );
}

#[test]
fn set_software_release_encodes_stable() {
    let cmd = SetActiveSoftwareReleaseChannel {
        channel: SoftwareReleaseChannel::Stable,
    };
    assert_eq!(cmd.to_message().unwrap(), b"RL0;".to_vec());
}

#[test]
fn set_software_release_encodes_beta() {
    let cmd = SetActiveSoftwareReleaseChannel {
        channel: SoftwareReleaseChannel::Beta,
    };
    assert_eq!(cmd.to_message().unwrap(), b"RL1;".to_vec());
}

#[test]
fn set_software_release_encodes_alpha() {
    let cmd = SetActiveSoftwareReleaseChannel {
        channel: SoftwareReleaseChannel::Alpha,
    };
    assert_eq!(cmd.to_message().unwrap(), b"RL2;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// RP: GetRepeaterOffset, SetRepeaterOffset
// ------------------------------------------------------------------------------------------------

#[test]
fn get_repeater_offset_encodes() {
    assert_eq!(GetRepeaterOffset.to_message().unwrap(), b"RP;".to_vec());
}

#[test]
fn set_repeater_offset_encodes() {
    let cmd = SetRepeaterOffset {
        direction: RepeaterOffsetDirection::Positive,
        offset_hz: 600_000,
    };
    assert_eq!(cmd.to_message().unwrap(), b"RP10600000;".to_vec());
}

#[test]
fn set_repeater_offset_accepts_boundary_values() {
    assert!(
        SetRepeaterOffset {
            direction: RepeaterOffsetDirection::Off,
            offset_hz: 0
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetRepeaterOffset {
            direction: RepeaterOffsetDirection::Negative,
            offset_hz: 999_999
        }
        .validate()
        .is_ok()
    );
}

// ------------------------------------------------------------------------------------------------
// SC: GetScreenCount
// ------------------------------------------------------------------------------------------------

#[test]
fn get_screen_count_encodes() {
    assert_eq!(GetScreenCount.to_message().unwrap(), b"SC;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SD: SetK4Delay
// ------------------------------------------------------------------------------------------------

#[test]
fn set_k4_delay_encodes() {
    let cmd = SetK4QskOrVoxDelay { delay_ms: 500 };
    assert_eq!(cmd.to_message().unwrap(), b"SD0500;".to_vec());
}

#[test]
fn set_k4_delay_accepts_boundary_values() {
    assert!(SetK4QskOrVoxDelay { delay_ms: 0 }.validate().is_ok());
    assert!(SetK4QskOrVoxDelay { delay_ms: 2000 }.validate().is_ok());
}

#[test]
fn set_k4_delay_rejects_out_of_range() {
    assert!(matches!(
        SetK4QskOrVoxDelay { delay_ms: 2001 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// SI: SetSystemAutoInfo
// ------------------------------------------------------------------------------------------------

#[test]
fn set_system_auto_info_encodes() {
    let cmd = SetSystemAutoInfoInterval { interval_ms: 1000 };
    assert_eq!(cmd.to_message().unwrap(), b"SI1000;".to_vec());
}

#[test]
fn set_system_auto_info_accepts_boundary_values() {
    assert!(
        SetSystemAutoInfoInterval { interval_ms: 0 }
            .validate()
            .is_ok()
    );
    assert!(
        SetSystemAutoInfoInterval { interval_ms: 9999 }
            .validate()
            .is_ok()
    );
}

#[test]
fn set_system_auto_info_rejects_out_of_range() {
    assert!(matches!(
        SetSystemAutoInfoInterval { interval_ms: 10000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// SL: GetStreamingLatency, SetStreamingLatency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_streaming_latency_encodes() {
    assert_eq!(
        GetStreamingLatencyClass.to_message().unwrap(),
        b"SL;".to_vec()
    );
}

#[test]
fn set_streaming_latency_encodes() {
    let cmd = SetStreamingLatencyClass { latency: 25 };
    assert_eq!(cmd.to_message().unwrap(), b"SL25;".to_vec());
}

#[test]
fn set_streaming_latency_accepts_boundary_values() {
    assert!(SetStreamingLatencyClass { latency: 0 }.validate().is_ok());
    assert!(SetStreamingLatencyClass { latency: 99 }.validate().is_ok());
}

#[test]
fn set_streaming_latency_rejects_out_of_range() {
    assert!(matches!(
        SetStreamingLatencyClass { latency: 100 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// SN: GetSerialNumber
// ------------------------------------------------------------------------------------------------

#[test]
fn get_serial_number_encodes() {
    assert_eq!(
        GetTransceiverSerialNumber.to_message().unwrap(),
        b"SN;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SS: CaptureScreenshot
// ------------------------------------------------------------------------------------------------

#[test]
fn capture_screenshot_encodes() {
    assert_eq!(CaptureScreenshot.to_message().unwrap(), b"SS;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// TA: GetTxGainConstant
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_gain_constant_encodes() {
    assert_eq!(
        GetTransmitGainConstant.to_message().unwrap(),
        b"TA;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// TD: GetTextDecodeModeA, GetTextDecodeModeB, SetTextDecodeModeA, SetTextDecodeModeB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_text_decode_mode_a_encodes() {
    assert_eq!(GetVfoATextDecodeMode.to_message().unwrap(), b"TD;".to_vec());
}

#[test]
fn get_text_decode_mode_b_encodes() {
    assert_eq!(
        GetVfoBTextDecodeMode.to_message().unwrap(),
        b"TD$;".to_vec()
    );
}

#[test]
fn set_text_decode_mode_a_encodes() {
    let cmd = SetVfoATextDecodeMode {
        mode: TextDecodeMode::Rtty,
    };
    assert_eq!(cmd.to_message().unwrap(), b"TD2;".to_vec());
}

#[test]
fn set_text_decode_mode_a_accepts_boundary_values() {
    assert!(
        SetVfoATextDecodeMode {
            mode: TextDecodeMode::Off
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetVfoATextDecodeMode {
            mode: TextDecodeMode::Psk
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_text_decode_mode_b_encodes() {
    let cmd = SetVfoBTextDecodeMode {
        mode: TextDecodeMode::Rtty,
    };
    assert_eq!(cmd.to_message().unwrap(), b"TD$1;".to_vec());
}

#[test]
fn set_text_decode_mode_b_accepts_boundary_values() {
    assert!(
        SetVfoBTextDecodeMode {
            mode: TextDecodeMode::Off
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetVfoBTextDecodeMode {
            mode: TextDecodeMode::Psk
        }
        .validate()
        .is_ok()
    );
}

// ------------------------------------------------------------------------------------------------
// TG: GetTxGain
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_gain_encodes() {
    assert_eq!(GetTransmitGain.to_message().unwrap(), b"TG;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// TS: GetTxTestMode, SetTxTestMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_test_mode_encodes() {
    assert_eq!(
        GetTransmitTestModeState.to_message().unwrap(),
        b"TS;".to_vec()
    );
}

#[test]
fn set_tx_test_mode_encodes_on() {
    let cmd = SetTransmitTestModeState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"TS1;".to_vec());
}

#[test]
fn set_tx_test_mode_encodes_off() {
    let cmd = SetTransmitTestModeState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"TS0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// TU: SetTune
// ------------------------------------------------------------------------------------------------

#[test]
fn set_tune_encodes_start() {
    let cmd = SetAtuTuningState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"TU1;".to_vec());
}

#[test]
fn set_tune_encodes_stop() {
    let cmd = SetAtuTuningState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"TU0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// UT: GetUtcTimestamp
// ------------------------------------------------------------------------------------------------

#[test]
fn get_utc_timestamp_encodes() {
    assert_eq!(GetUtcTimestamp.to_message().unwrap(), b"UT;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// VC: GetCoarseTuneStep, SetCoarseTuneStep
// ------------------------------------------------------------------------------------------------

#[test]
fn get_coarse_tune_step_encodes() {
    assert_eq!(GetCoarseTuningStep.to_message().unwrap(), b"VC;".to_vec());
}

#[test]
fn set_coarse_tune_step_encodes() {
    let cmd = SetCoarseTuningStep { step: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"VC05;".to_vec());
}

#[test]
fn set_coarse_tune_step_accepts_boundary_values() {
    assert!(SetCoarseTuningStep { step: 0 }.validate().is_ok());
    assert!(SetCoarseTuningStep { step: 99 }.validate().is_ok());
}

#[test]
fn set_coarse_tune_step_rejects_out_of_range() {
    assert!(matches!(
        SetCoarseTuningStep { step: 100 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// VG: GetVoxGain, SetVoxGain
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vox_gain_encodes() {
    assert_eq!(GetVoxGain.to_message().unwrap(), b"VG;".to_vec());
}

#[test]
fn set_vox_gain_encodes() {
    let cmd = SetVoxGain { gain: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"VG005;".to_vec());
}

#[test]
fn set_vox_gain_accepts_boundary_values() {
    assert!(SetVoxGain { gain: 0 }.validate().is_ok());
    assert!(SetVoxGain { gain: 9 }.validate().is_ok());
}

#[test]
fn set_vox_gain_rejects_out_of_range() {
    assert!(matches!(
        SetVoxGain { gain: 10 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// VI: GetVoxInhibit, SetVoxInhibit
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vox_inhibit_encodes() {
    assert_eq!(GetVoxInhibitState.to_message().unwrap(), b"VI;".to_vec());
}

#[test]
fn set_vox_inhibit_encodes_on() {
    let cmd = SetVoxInhibitState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"VI1;".to_vec());
}

#[test]
fn set_vox_inhibit_encodes_off() {
    let cmd = SetVoxInhibitState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"VI0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// VO: GetVfoOffsetA, GetVfoOffsetB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_offset_a_encodes() {
    assert_eq!(
        GetVfoATransverterOffset.to_message().unwrap(),
        b"VO;".to_vec()
    );
}

#[test]
fn get_vfo_offset_b_encodes() {
    assert_eq!(
        GetVfoATransverterOffset.to_message().unwrap(),
        b"VO$;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// VT: GetVfoTuningStepA, GetVfoTuningStepB, SetVfoTuningStepA, SetVfoTuningStepB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_tuning_step_a_encodes() {
    assert_eq!(GetVfoATuningStep.to_message().unwrap(), b"VT;".to_vec());
}

#[test]
fn get_vfo_tuning_step_b_encodes() {
    assert_eq!(GetVfoBTuningStep.to_message().unwrap(), b"VT$;".to_vec());
}

#[test]
fn set_vfo_tuning_step_a_encodes() {
    let cmd = SetVfoATuningStep { step_hz: 1000 };
    assert_eq!(cmd.to_message().unwrap(), b"VT001000;".to_vec());
}

#[test]
fn set_vfo_tuning_step_a_accepts_boundary_values() {
    assert!(SetVfoATuningStep { step_hz: 0 }.validate().is_ok());
    assert!(SetVfoATuningStep { step_hz: 999_999 }.validate().is_ok());
}

#[test]
fn set_vfo_tuning_step_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoATuningStep { step_hz: 1_000_000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_vfo_tuning_step_b_encodes() {
    let cmd = SetVfoBTuningStep { step_hz: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"VT$000001;".to_vec());
}

#[test]
fn set_vfo_tuning_step_b_accepts_boundary_values() {
    assert!(SetVfoBTuningStep { step_hz: 0 }.validate().is_ok());
    assert!(SetVfoBTuningStep { step_hz: 999_999 }.validate().is_ok());
}

#[test]
fn set_vfo_tuning_step_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBTuningStep { step_hz: 1_000_000 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// WM: GetWattmeterCalibration, SetWattmeterCalibration
// ------------------------------------------------------------------------------------------------

#[test]
fn get_wattmeter_calibration_encodes() {
    assert_eq!(
        GetWattmeterCalibrationConstant.to_message().unwrap(),
        b"WM;".to_vec()
    );
}

#[test]
fn set_wattmeter_calibration_encodes() {
    let cmd = SetWattmeterCalibrationConstant { value: 128 };
    assert_eq!(cmd.to_message().unwrap(), b"WM128;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// XV: GetTransverterBandA, GetTransverterBandB, SetTransverterBandA, SetTransverterBandB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transverter_band_a_encodes() {
    assert_eq!(
        GetVfoATransverterActiveBandSlot.to_message().unwrap(),
        b"XV;".to_vec()
    );
}

#[test]
fn get_transverter_band_b_encodes() {
    assert_eq!(
        GetVfoBTransverterActiveBandSlot.to_message().unwrap(),
        b"XV$;".to_vec()
    );
}

#[test]
fn set_transverter_band_a_encodes() {
    let cmd = SetVfoATransverterActiveBandSlot { band_slot: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"XV03;".to_vec());
}

#[test]
fn set_transverter_band_a_accepts_boundary_values() {
    assert!(
        SetVfoATransverterActiveBandSlot { band_slot: 0 }
            .validate()
            .is_ok()
    );
    assert!(
        SetVfoATransverterActiveBandSlot { band_slot: 8 }
            .validate()
            .is_ok()
    );
}

#[test]
fn set_transverter_band_a_rejects_out_of_range() {
    assert!(matches!(
        SetVfoATransverterActiveBandSlot { band_slot: 9 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_transverter_band_b_encodes() {
    let cmd = SetVfoBTransverterActiveBandSlot { band_slot: 7 };
    assert_eq!(cmd.to_message().unwrap(), b"XV$07;".to_vec());
}

#[test]
fn set_transverter_band_b_accepts_boundary_values() {
    assert!(
        SetVfoBTransverterActiveBandSlot { band_slot: 0 }
            .validate()
            .is_ok()
    );
    assert!(
        SetVfoBTransverterActiveBandSlot { band_slot: 8 }
            .validate()
            .is_ok()
    );
}

#[test]
fn set_transverter_band_b_rejects_out_of_range() {
    assert!(matches!(
        SetVfoBTransverterActiveBandSlot { band_slot: 9 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}
