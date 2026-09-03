//!
//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::px3`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`).
//! Response *parsing* is intentionally out of scope here. None of the PX3 commands attach
//! argument validation, so there are no boundary/invalid-value tests in this file.
//!

use pretty_assertions::assert_eq;
use rfham_rigs::protocol::{
    Command,
    cat::elecraft::{
        k3_kx::VfoFrequencyChangeStep,
        px3::{
            GetBeaconModeState, GetBeaconTextMemoryLocation, GetBeaconTransmissionInterval,
            GetCalibrationSignalState, GetOppositeSideBandNullAmplitude,
            GetOppositeSideBandNullPhase, GetTextHangTime, GetTextTransmitMode,
            GetUsbKeyboardDetectedState, MoveMarkerAFrequency, MoveMarkerBFrequency,
            SetBeaconModeState, SetBeaconTextMemoryLocation, SetBeaconTransmissionInterval,
            SetCalibrationSignalState, SetOppositeSideBandNullAmplitude,
            SetOppositeSideBandNullPhase, SetTextHangTime, SetTextTransmitMode, TextTransmitMode,
        },
    },
};

// ------------------------------------------------------------------------------------------------

#[test]
fn get_beacon_transmission_interval_encodes() {
    assert_eq!(
        GetBeaconTransmissionInterval.to_message().unwrap(),
        b"#BCI;".to_vec()
    );
}

#[test]
fn set_beacon_transmission_interval_encodes_pos() {
    let cmd = SetBeaconTransmissionInterval { interval_secs: 10 };
    assert_eq!(cmd.to_message().unwrap(), b"#BCI0010;".to_vec());
}

#[test]
fn set_beacon_transmission_interval_encodes_neg() {
    let cmd = SetBeaconTransmissionInterval { interval_secs: 0 };
    assert_eq!(cmd.to_message().unwrap(), b"#BCI0000;".to_vec());
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_beacon_text_memory_location_encodes() {
    assert_eq!(
        GetBeaconTextMemoryLocation.to_message().unwrap(),
        b"#BCL;".to_vec()
    );
}

#[test]
fn set_beacon_text_memory_location_encodes_on() {
    let cmd = SetBeaconTextMemoryLocation { location: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"#BCL1;".to_vec());
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_beacon_mode_state_encodes() {
    assert_eq!(GetBeaconModeState.to_message().unwrap(), b"#BCN;".to_vec());
}

#[test]
fn set_beacon_mode_state_encodes_on() {
    let cmd = SetBeaconModeState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"#BCN1;".to_vec());
}

#[test]
fn set_beacon_mode_state_encodes_off() {
    let cmd = SetBeaconModeState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"#BCN0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetCalibSignal, SetCalibSignal
// ------------------------------------------------------------------------------------------------

#[test]
fn get_calibration_signal_state_encodes() {
    assert_eq!(
        GetCalibrationSignalState.to_message().unwrap(),
        b"#CAL;".to_vec()
    );
}

#[test]
fn set_calibration_signal_state_encodes_on() {
    let cmd = SetCalibrationSignalState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"#CAL1;".to_vec());
}

#[test]
fn set_calibration_signal_state_encodes_off() {
    let cmd = SetCalibrationSignalState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"#CAL0;".to_vec());
}

// ------------------------------------------------------------------------------------------------

// TODO: GetDisplayMode/SetDisplayMode

// ------------------------------------------------------------------------------------------------

// TODO: GetFunctionKeyLabelDisplayState/SetFunctionKeyLabelDisplayState

// ------------------------------------------------------------------------------------------------

#[test]
fn get_move_marker_a_frequency_encodes_none() {
    assert_eq!(
        MoveMarkerAFrequency { step: None }.to_message().unwrap(),
        b"#MAA;".to_vec()
    );
}

#[test]
fn get_move_marker_a_frequency_encodes() {
    assert_eq!(
        MoveMarkerAFrequency {
            step: Some(VfoFrequencyChangeStep::Step10Hz)
        }
        .to_message()
        .unwrap(),
        b"#MAA+1;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_move_marker_b_frequency_encodes() {
    assert_eq!(
        MoveMarkerBFrequency { step: None }.to_message().unwrap(),
        b"#MBA;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------

// TODO: SaveScreenshotToFlashDrive

// ------------------------------------------------------------------------------------------------

#[test]
fn get_opposite_sideband_null_amplitude_encodes() {
    assert_eq!(
        GetOppositeSideBandNullAmplitude.to_message().unwrap(),
        b"#OSBA;".to_vec()
    );
}

#[test]
fn set_opposite_sideband_null_amplitude_encodes_pos() {
    let cmd = SetOppositeSideBandNullAmplitude { amplitude: 10 };
    assert_eq!(cmd.to_message().unwrap(), b"#OSBA+0010;".to_vec());
}

#[test]
fn set_opposite_sideband_null_amplitude_encodes_neg() {
    let cmd = SetOppositeSideBandNullAmplitude { amplitude: -20 };
    assert_eq!(cmd.to_message().unwrap(), b"#OSBA-0020;".to_vec());
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_opposite_sideband_null_phase_encodes() {
    assert_eq!(
        GetOppositeSideBandNullPhase.to_message().unwrap(),
        b"#OSBP;".to_vec()
    );
}

#[test]
fn set_opposite_sideband_null_phase_encodes() {
    let cmd = SetOppositeSideBandNullPhase { phase: 45 };
    assert_eq!(cmd.to_message().unwrap(), b"#OSBP+0045;".to_vec());
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_text_hang_time_encodes() {
    assert_eq!(GetTextHangTime.to_message().unwrap(), b"#TXH;".to_vec());
}

#[test]
fn set_text_hang_time_encodes_on() {
    let cmd = SetTextHangTime { time_ms: 1000 };
    assert_eq!(cmd.to_message().unwrap(), b"#TXH01000;".to_vec());
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_text_transmit_mode_encodes() {
    assert_eq!(GetTextTransmitMode.to_message().unwrap(), b"#TXM;".to_vec());
}

#[test]
fn set_text_transmit_mode_encodes_off() {
    let cmd = SetTextTransmitMode {
        mode: TextTransmitMode::EnterKey,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#TXM0;".to_vec());
}

// ------------------------------------------------------------------------------------------------

#[test]
fn get_usb_keyboard_detected_state_encodes() {
    assert_eq!(
        GetUsbKeyboardDetectedState.to_message().unwrap(),
        b"#USB;".to_vec()
    );
}
