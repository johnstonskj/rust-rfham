//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::kpa500`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command,
        cat::elecraft::kpa500::{
            ClearFault, GetAlcThreshold, GetAttenuatorReleaseTime, GetBandSelection, GetDemoMode,
            GetFanMinimumSpeed, GetFaultCode, GetFaultSpeaker, GetFirmwareVersion, GetInhibitInput,
            GetOperateMode, GetPaTemperature, GetPaVoltageCurrent, GetPcBaudRate,
            GetPowerAdjustment, GetPowerAndSwr, GetPowerStatus, GetRadioInterface, GetSerialNumber,
            GetStandbyOnBandChange, GetTrDelay, GetXcvrBaudRate, SetAlcThreshold,
            SetAttenuatorReleaseTime, SetBandSelection, SetDemoMode, SetFanMinimumSpeed,
            SetFaultSpeaker, SetInhibitInput, SetOperateMode, SetPcBaudRate, SetPowerAdjustment,
            SetRadioInterface, SetStandbyOnBandChange, SetTrDelay, SetXcvrBaudRate, TurnPowerOff,
        },
    },
};

// ------------------------------------------------------------------------------------------------
// GetAlcThreshold, SetAlcThreshold
// ------------------------------------------------------------------------------------------------

#[test]
fn get_alc_threshold_encodes() {
    assert_eq!(GetAlcThreshold.to_message().unwrap(), b"^AL;".to_vec());
}

#[test]
fn set_alc_threshold_encodes() {
    let cmd = SetAlcThreshold { value: 100 };
    assert_eq!(cmd.to_message().unwrap(), b"^AL100;".to_vec());
}

#[test]
fn set_alc_threshold_accepts_boundary_values() {
    assert!(SetAlcThreshold { value: 0 }.validate().is_ok());
    assert!(SetAlcThreshold { value: 210 }.validate().is_ok());
}

#[test]
fn set_alc_threshold_rejects_out_of_range() {
    assert!(matches!(
        SetAlcThreshold { value: 211 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetAttenuatorReleaseTime, SetAttenuatorReleaseTime
// ------------------------------------------------------------------------------------------------

#[test]
fn get_attenuator_release_time_encodes() {
    assert_eq!(
        GetAttenuatorReleaseTime.to_message().unwrap(),
        b"^AR;".to_vec()
    );
}

#[test]
fn set_attenuator_release_time_encodes() {
    let cmd = SetAttenuatorReleaseTime { ms: 2000 };
    assert_eq!(cmd.to_message().unwrap(), b"^AR2000;".to_vec());
}

#[test]
fn set_attenuator_release_time_accepts_boundary_values() {
    assert!(SetAttenuatorReleaseTime { ms: 1400 }.validate().is_ok());
    assert!(SetAttenuatorReleaseTime { ms: 5000 }.validate().is_ok());
}

#[test]
fn set_attenuator_release_time_rejects_out_of_range() {
    assert!(matches!(
        SetAttenuatorReleaseTime { ms: 1399 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetAttenuatorReleaseTime { ms: 5001 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetStandbyOnBandChange, SetStandbyOnBandChange
// ------------------------------------------------------------------------------------------------

#[test]
fn get_standby_on_band_change_encodes() {
    assert_eq!(
        GetStandbyOnBandChange.to_message().unwrap(),
        b"^BC;".to_vec()
    );
}

#[test]
fn set_standby_on_band_change_encodes() {
    assert_eq!(
        SetStandbyOnBandChange { enabled: true }
            .to_message()
            .unwrap(),
        b"^BC1;".to_vec()
    );
    assert_eq!(
        SetStandbyOnBandChange { enabled: false }
            .to_message()
            .unwrap(),
        b"^BC0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetBandSelection, SetBandSelection
// ------------------------------------------------------------------------------------------------

#[test]
fn get_band_selection_encodes() {
    assert_eq!(GetBandSelection.to_message().unwrap(), b"^BN;".to_vec());
}

#[test]
fn set_band_selection_encodes() {
    let cmd = SetBandSelection { band: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"^BN05;".to_vec());
}

#[test]
fn set_band_selection_accepts_boundary_values() {
    assert!(SetBandSelection { band: 0 }.validate().is_ok());
    assert!(SetBandSelection { band: 10 }.validate().is_ok());
}

#[test]
fn set_band_selection_rejects_out_of_range() {
    assert!(matches!(
        SetBandSelection { band: 11 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetPcBaudRate, SetPcBaudRate
// ------------------------------------------------------------------------------------------------

#[test]
fn get_pc_baud_rate_encodes() {
    assert_eq!(GetPcBaudRate.to_message().unwrap(), b"^BRP;".to_vec());
}

#[test]
fn set_pc_baud_rate_encodes() {
    let cmd = SetPcBaudRate { rate: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"^BRP2;".to_vec());
}

#[test]
fn set_pc_baud_rate_accepts_boundary_values() {
    assert!(SetPcBaudRate { rate: 0 }.validate().is_ok());
    assert!(SetPcBaudRate { rate: 3 }.validate().is_ok());
}

#[test]
fn set_pc_baud_rate_rejects_out_of_range() {
    assert!(matches!(
        SetPcBaudRate { rate: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetXcvrBaudRate, SetXcvrBaudRate
// ------------------------------------------------------------------------------------------------

#[test]
fn get_xcvr_baud_rate_encodes() {
    assert_eq!(GetXcvrBaudRate.to_message().unwrap(), b"^BRX;".to_vec());
}

#[test]
fn set_xcvr_baud_rate_encodes() {
    let cmd = SetXcvrBaudRate { rate: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"^BRX1;".to_vec());
}

#[test]
fn set_xcvr_baud_rate_accepts_boundary_values() {
    assert!(SetXcvrBaudRate { rate: 0 }.validate().is_ok());
    assert!(SetXcvrBaudRate { rate: 3 }.validate().is_ok());
}

#[test]
fn set_xcvr_baud_rate_rejects_out_of_range() {
    assert!(matches!(
        SetXcvrBaudRate { rate: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetDemoMode, SetDemoMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_demo_mode_encodes() {
    assert_eq!(GetDemoMode.to_message().unwrap(), b"^DMO;".to_vec());
}

#[test]
fn set_demo_mode_encodes() {
    assert_eq!(
        SetDemoMode { enabled: true }.to_message().unwrap(),
        b"^DMO1;".to_vec()
    );
    assert_eq!(
        SetDemoMode { enabled: false }.to_message().unwrap(),
        b"^DMO0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetFanMinimumSpeed, SetFanMinimumSpeed
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fan_minimum_speed_encodes() {
    assert_eq!(GetFanMinimumSpeed.to_message().unwrap(), b"^FC;".to_vec());
}

#[test]
fn set_fan_minimum_speed_encodes() {
    let cmd = SetFanMinimumSpeed { level: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"^FC3;".to_vec());
}

#[test]
fn set_fan_minimum_speed_accepts_boundary_values() {
    assert!(SetFanMinimumSpeed { level: 0 }.validate().is_ok());
    assert!(SetFanMinimumSpeed { level: 6 }.validate().is_ok());
}

#[test]
fn set_fan_minimum_speed_rejects_out_of_range() {
    assert!(matches!(
        SetFanMinimumSpeed { level: 7 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetFaultCode, ClearFault
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fault_code_encodes() {
    assert_eq!(GetFaultCode.to_message().unwrap(), b"^FL;".to_vec());
}

#[test]
fn clear_fault_encodes() {
    assert_eq!(ClearFault.to_message().unwrap(), b"^FLC;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetInhibitInput, SetInhibitInput
// ------------------------------------------------------------------------------------------------

#[test]
fn get_inhibit_input_encodes() {
    assert_eq!(GetInhibitInput.to_message().unwrap(), b"^NH;".to_vec());
}

#[test]
fn set_inhibit_input_encodes() {
    assert_eq!(
        SetInhibitInput { enabled: true }.to_message().unwrap(),
        b"^NH1;".to_vec()
    );
    assert_eq!(
        SetInhibitInput { enabled: false }.to_message().unwrap(),
        b"^NH0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetPowerStatus, TurnPowerOff
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_status_encodes() {
    assert_eq!(GetPowerStatus.to_message().unwrap(), b"^ON;".to_vec());
}

#[test]
fn turn_power_off_encodes() {
    assert_eq!(TurnPowerOff.to_message().unwrap(), b"^ON0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetOperateMode, SetOperateMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_operate_mode_encodes() {
    assert_eq!(GetOperateMode.to_message().unwrap(), b"^OS;".to_vec());
}

#[test]
fn set_operate_mode_encodes() {
    assert_eq!(
        SetOperateMode { operate: true }.to_message().unwrap(),
        b"^OS1;".to_vec()
    );
    assert_eq!(
        SetOperateMode { operate: false }.to_message().unwrap(),
        b"^OS0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetPowerAdjustment, SetPowerAdjustment
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_adjustment_encodes() {
    assert_eq!(GetPowerAdjustment.to_message().unwrap(), b"^PJ;".to_vec());
}

#[test]
fn set_power_adjustment_encodes() {
    let cmd = SetPowerAdjustment { value: 100 };
    assert_eq!(cmd.to_message().unwrap(), b"^PJ100;".to_vec());
}

#[test]
fn set_power_adjustment_accepts_boundary_values() {
    assert!(SetPowerAdjustment { value: 80 }.validate().is_ok());
    assert!(SetPowerAdjustment { value: 120 }.validate().is_ok());
}

#[test]
fn set_power_adjustment_rejects_out_of_range() {
    assert!(matches!(
        SetPowerAdjustment { value: 79 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetPowerAdjustment { value: 121 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

#[test]
fn get_firmware_version_encodes() {
    assert_eq!(GetFirmwareVersion.to_message().unwrap(), b"^RVM;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSerialNumber
// ------------------------------------------------------------------------------------------------

#[test]
fn get_serial_number_encodes() {
    assert_eq!(GetSerialNumber.to_message().unwrap(), b"^SN;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFaultSpeaker, SetFaultSpeaker
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fault_speaker_encodes() {
    assert_eq!(GetFaultSpeaker.to_message().unwrap(), b"^SP;".to_vec());
}

#[test]
fn set_fault_speaker_encodes() {
    // NOTE: SetFaultSpeaker is defined via the `{ state }` shorthand, which names the boolean
    // field `on` (not `state`).
    assert_eq!(
        SetFaultSpeaker { on: true }.to_message().unwrap(),
        b"^SP1;".to_vec()
    );
    assert_eq!(
        SetFaultSpeaker { on: false }.to_message().unwrap(),
        b"^SP0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetPaTemperature
// ------------------------------------------------------------------------------------------------

#[test]
fn get_pa_temperature_encodes() {
    assert_eq!(GetPaTemperature.to_message().unwrap(), b"^TM;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetTrDelay, SetTrDelay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tr_delay_encodes() {
    assert_eq!(GetTrDelay.to_message().unwrap(), b"^TR;".to_vec());
}

#[test]
fn set_tr_delay_encodes() {
    let cmd = SetTrDelay { ms: 25 };
    assert_eq!(cmd.to_message().unwrap(), b"^TR25;".to_vec());
}

#[test]
fn set_tr_delay_accepts_boundary_values() {
    assert!(SetTrDelay { ms: 0 }.validate().is_ok());
    assert!(SetTrDelay { ms: 50 }.validate().is_ok());
}

#[test]
fn set_tr_delay_rejects_out_of_range() {
    assert!(matches!(
        SetTrDelay { ms: 51 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetPaVoltageCurrent
// ------------------------------------------------------------------------------------------------

#[test]
fn get_pa_voltage_current_encodes() {
    assert_eq!(GetPaVoltageCurrent.to_message().unwrap(), b"^VI;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetPowerAndSwr
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_and_swr_encodes() {
    assert_eq!(GetPowerAndSwr.to_message().unwrap(), b"^WS;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetRadioInterface, SetRadioInterface
// ------------------------------------------------------------------------------------------------

#[test]
fn get_radio_interface_encodes() {
    assert_eq!(GetRadioInterface.to_message().unwrap(), b"^XI;".to_vec());
}

#[test]
fn set_radio_interface_encodes() {
    let cmd = SetRadioInterface {
        interface_type: 3,
        option: 1,
    };
    assert_eq!(cmd.to_message().unwrap(), b"^XI031;".to_vec());
}

#[test]
fn set_radio_interface_accepts_boundary_values() {
    assert!(
        SetRadioInterface {
            interface_type: 0,
            option: 0,
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetRadioInterface {
            interface_type: 3,
            option: 1,
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn set_radio_interface_rejects_out_of_range() {
    assert!(matches!(
        SetRadioInterface {
            interface_type: 4,
            option: 0,
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetRadioInterface {
            interface_type: 0,
            option: 2,
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}
