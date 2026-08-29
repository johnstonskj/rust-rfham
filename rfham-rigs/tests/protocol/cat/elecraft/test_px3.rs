//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::px3`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`).
//! Response *parsing* is intentionally out of scope here. None of the PX3 commands attach
//! argument validation, so there are no boundary/invalid-value tests in this file.

use pretty_assertions::assert_eq;
use rfham_rigs::protocol::{
    Command,
    cat::elecraft::px3::{
        GetBandscopeChannelIndicatorState, GetBandscopeChannelList, GetBandscopeChannelName,
        GetCalibSignal, GetMemoryAntennaA, GetMemoryAntennaB, GetOffscreenBandscopeActive,
        GetOffscreenBandscopePosition, GetTxHold, GetTxMarker, GetUsbAudioEnable,
        SetBandscopeChannelIndicatorState, SetBandscopeChannelList, SetBandscopeChannelName,
        SetCalibSignal, SetOffscreenBandscopeActive, SetOffscreenBandscopePosition, SetTxHold,
        SetTxMarker, SetUsbAudioEnable,
    },
};

// ------------------------------------------------------------------------------------------------
// GetBandscopeChannelIndicatorState, SetBandscopeChannelIndicatorState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_bandscope_channel_indicator_state_encodes() {
    assert_eq!(
        GetBandscopeChannelIndicatorState.to_message().unwrap(),
        b"#BCI;".to_vec()
    );
}

#[test]
fn set_bandscope_channel_indicator_state_encodes_on() {
    let cmd = SetBandscopeChannelIndicatorState { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#BCI1;".to_vec());
}

#[test]
fn set_bandscope_channel_indicator_state_encodes_off() {
    let cmd = SetBandscopeChannelIndicatorState { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#BCI0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetBandscopeChannelList, SetBandscopeChannelList
// ------------------------------------------------------------------------------------------------

#[test]
fn get_bandscope_channel_list_encodes() {
    assert_eq!(
        GetBandscopeChannelList.to_message().unwrap(),
        b"#BCL;".to_vec()
    );
}

#[test]
fn set_bandscope_channel_list_encodes_on() {
    let cmd = SetBandscopeChannelList { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#BCL1;".to_vec());
}

#[test]
fn set_bandscope_channel_list_encodes_off() {
    let cmd = SetBandscopeChannelList { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#BCL0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetBandscopeChannelName, SetBandscopeChannelName
// ------------------------------------------------------------------------------------------------

#[test]
fn get_bandscope_channel_name_encodes() {
    assert_eq!(
        GetBandscopeChannelName.to_message().unwrap(),
        b"#BCN;".to_vec()
    );
}

#[test]
fn set_bandscope_channel_name_encodes_on() {
    let cmd = SetBandscopeChannelName { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#BCN1;".to_vec());
}

#[test]
fn set_bandscope_channel_name_encodes_off() {
    let cmd = SetBandscopeChannelName { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#BCN0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetCalibSignal, SetCalibSignal
// ------------------------------------------------------------------------------------------------

#[test]
fn get_calib_signal_encodes() {
    assert_eq!(GetCalibSignal.to_message().unwrap(), b"#CAL;".to_vec());
}

#[test]
fn set_calib_signal_encodes_on() {
    let cmd = SetCalibSignal { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#CAL1;".to_vec());
}

#[test]
fn set_calib_signal_encodes_off() {
    let cmd = SetCalibSignal { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#CAL0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMemoryAntennaA
// ------------------------------------------------------------------------------------------------

#[test]
fn get_memory_antenna_a_encodes() {
    assert_eq!(GetMemoryAntennaA.to_message().unwrap(), b"#MAA;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMemoryAntennaB
// ------------------------------------------------------------------------------------------------

#[test]
fn get_memory_antenna_b_encodes() {
    assert_eq!(GetMemoryAntennaB.to_message().unwrap(), b"#MBA;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetOffscreenBandscopePosition, SetOffscreenBandscopePosition
// ------------------------------------------------------------------------------------------------

#[test]
fn get_offscreen_bandscope_position_encodes() {
    assert_eq!(
        GetOffscreenBandscopePosition.to_message().unwrap(),
        b"#OSBP;".to_vec()
    );
}

#[test]
fn set_offscreen_bandscope_position_encodes() {
    let cmd = SetOffscreenBandscopePosition { position: 42 };
    assert_eq!(cmd.to_message().unwrap(), b"#OSBP42;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetOffscreenBandscopeActive, SetOffscreenBandscopeActive
// ------------------------------------------------------------------------------------------------

#[test]
fn get_offscreen_bandscope_active_encodes() {
    assert_eq!(
        GetOffscreenBandscopeActive.to_message().unwrap(),
        b"#OSBA;".to_vec()
    );
}

#[test]
fn set_offscreen_bandscope_active_encodes_on() {
    let cmd = SetOffscreenBandscopeActive { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#OSBA1;".to_vec());
}

#[test]
fn set_offscreen_bandscope_active_encodes_off() {
    let cmd = SetOffscreenBandscopeActive { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#OSBA0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetTxHold, SetTxHold
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_hold_encodes() {
    assert_eq!(GetTxHold.to_message().unwrap(), b"#TXH;".to_vec());
}

#[test]
fn set_tx_hold_encodes_on() {
    let cmd = SetTxHold { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#TXH1;".to_vec());
}

#[test]
fn set_tx_hold_encodes_off() {
    let cmd = SetTxHold { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#TXH0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetTxMarker, SetTxMarker
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tx_marker_encodes() {
    assert_eq!(GetTxMarker.to_message().unwrap(), b"#TXM;".to_vec());
}

#[test]
fn set_tx_marker_encodes_on() {
    let cmd = SetTxMarker { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#TXM1;".to_vec());
}

#[test]
fn set_tx_marker_encodes_off() {
    let cmd = SetTxMarker { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#TXM0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetUsbAudioEnable, SetUsbAudioEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_usb_audio_enable_encodes() {
    assert_eq!(GetUsbAudioEnable.to_message().unwrap(), b"#USB;".to_vec());
}

#[test]
fn set_usb_audio_enable_encodes_on() {
    let cmd = SetUsbAudioEnable { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"#USB1;".to_vec());
}

#[test]
fn set_usb_audio_enable_encodes_off() {
    let cmd = SetUsbAudioEnable { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"#USB0;".to_vec());
}
