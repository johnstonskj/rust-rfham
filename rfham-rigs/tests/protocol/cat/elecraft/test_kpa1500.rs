//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::kpa1500`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command,
        cat::elecraft::kpa1500::{
            ClearFault, GetAdcReadings, GetAlcEnable, GetAlcThreshold, GetAmModeEnable,
            GetAntennaBandMap, GetAntennaSelection, GetAttenuatorReleaseTime, GetAtuPreset,
            GetAtuStatus, GetAutoAntennaSelection, GetAutoInfoMode, GetBandSelection,
            GetBypassRelay, GetDemoMode, GetDisplaySelect, GetFanMinimumSpeed, GetFaultCode,
            GetFaultSpeaker, GetFirmwareVersion, GetFrequency, GetInhibitInput, GetOperateMode,
            GetOutputPower, GetPaTemperature, GetPaVoltageCurrent, GetPcBaudRate,
            GetPeakPowerControl, GetPowerAdjustment, GetPowerAndSwr, GetPowerStatus,
            GetPowerStatusSummary, GetProtectionFaultEnable, GetPttDelay, GetRadioInterface,
            GetSerialNumber, GetStandbyOnBandChange, GetTrDelay, GetTransceiverVoltage,
            GetTunePower, GetXcvrBaudRate, RecallAtuPreset, SetAlcEnable, SetAlcThreshold,
            SetAmModeEnable, SetAntennaBandMap, SetAntennaSelection, SetAttenuatorReleaseTime,
            SetAutoAntennaSelection, SetAutoInfoMode, SetBandSelection, SetBypassRelay,
            SetDemoMode, SetDisplaySelect, SetFanMinimumSpeed, SetFaultSpeaker, SetFrequency,
            SetInhibitInput, SetOperateMode, SetPcBaudRate, SetPeakPowerControl,
            SetPowerAdjustment, SetProtectionFaultEnable, SetPttDelay, SetRadioInterface,
            SetStandbyOnBandChange, SetTrDelay, SetTunePower, SetXcvrBaudRate, TurnPowerOff,
        },
    },
};

// ------------------------------------------------------------------------------------------------
// GetAutoAntennaSelection, SetAutoAntennaSelection
// ------------------------------------------------------------------------------------------------

#[test]
fn get_auto_antenna_selection_encodes() {
    assert_eq!(
        GetAutoAntennaSelection.to_message().unwrap(),
        b"^AA;".to_vec()
    );
}

#[test]
fn set_auto_antenna_selection_encodes() {
    assert_eq!(
        SetAutoAntennaSelection { enabled: true }
            .to_message()
            .unwrap(),
        b"^AA1;".to_vec()
    );
    assert_eq!(
        SetAutoAntennaSelection { enabled: false }
            .to_message()
            .unwrap(),
        b"^AA0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetAntennaBandMap, SetAntennaBandMap
// ------------------------------------------------------------------------------------------------

#[test]
fn get_antenna_band_map_encodes() {
    assert_eq!(GetAntennaBandMap.to_message().unwrap(), b"^AB;".to_vec());
}

#[test]
fn set_antenna_band_map_encodes() {
    let cmd = SetAntennaBandMap { antenna: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"^AB1;".to_vec());
}

#[test]
fn set_antenna_band_map_accepts_boundary_values() {
    assert!(SetAntennaBandMap { antenna: 1 }.validate().is_ok());
    assert!(SetAntennaBandMap { antenna: 2 }.validate().is_ok());
}

#[test]
fn set_antenna_band_map_rejects_out_of_range() {
    assert!(matches!(
        SetAntennaBandMap { antenna: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetAntennaBandMap { antenna: 3 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetAdcReadings
// ------------------------------------------------------------------------------------------------

#[test]
fn get_adc_readings_encodes() {
    assert_eq!(GetAdcReadings.to_message().unwrap(), b"^AD;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetAlcEnable, SetAlcEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_alc_enable_encodes() {
    assert_eq!(GetAlcEnable.to_message().unwrap(), b"^AE;".to_vec());
}

#[test]
fn set_alc_enable_encodes() {
    assert_eq!(
        SetAlcEnable { enabled: true }.to_message().unwrap(),
        b"^AE1;".to_vec()
    );
    assert_eq!(
        SetAlcEnable { enabled: false }.to_message().unwrap(),
        b"^AE0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetAutoInfoMode, SetAutoInfoMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_auto_info_mode_encodes() {
    assert_eq!(GetAutoInfoMode.to_message().unwrap(), b"^AI;".to_vec());
}

#[test]
fn set_auto_info_mode_encodes() {
    assert_eq!(
        SetAutoInfoMode { enabled: true }.to_message().unwrap(),
        b"^AI1;".to_vec()
    );
    assert_eq!(
        SetAutoInfoMode { enabled: false }.to_message().unwrap(),
        b"^AI0;".to_vec()
    );
}

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
// GetAmModeEnable, SetAmModeEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_am_mode_enable_encodes() {
    assert_eq!(GetAmModeEnable.to_message().unwrap(), b"^AM;".to_vec());
}

#[test]
fn set_am_mode_enable_encodes() {
    assert_eq!(
        SetAmModeEnable { enabled: true }.to_message().unwrap(),
        b"^AM1;".to_vec()
    );
    assert_eq!(
        SetAmModeEnable { enabled: false }.to_message().unwrap(),
        b"^AM0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetAntennaSelection, SetAntennaSelection
// ------------------------------------------------------------------------------------------------

#[test]
fn get_antenna_selection_encodes() {
    assert_eq!(GetAntennaSelection.to_message().unwrap(), b"^AN;".to_vec());
}

#[test]
fn set_antenna_selection_encodes() {
    let cmd = SetAntennaSelection { antenna: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"^AN2;".to_vec());
}

#[test]
fn set_antenna_selection_accepts_boundary_values() {
    assert!(SetAntennaSelection { antenna: 1 }.validate().is_ok());
    assert!(SetAntennaSelection { antenna: 2 }.validate().is_ok());
}

#[test]
fn set_antenna_selection_rejects_out_of_range() {
    assert!(matches!(
        SetAntennaSelection { antenna: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetAntennaSelection { antenna: 3 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetAtuPreset, RecallAtuPreset
// ------------------------------------------------------------------------------------------------

#[test]
fn get_atu_preset_encodes() {
    assert_eq!(GetAtuPreset.to_message().unwrap(), b"^AP;".to_vec());
}

#[test]
fn recall_atu_preset_encodes() {
    // Same bare command string as GetAtuPreset's query form; it is the write-only trigger.
    assert_eq!(RecallAtuPreset.to_message().unwrap(), b"^AP;".to_vec());
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
// GetAtuStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_atu_status_encodes() {
    assert_eq!(GetAtuStatus.to_message().unwrap(), b"^AS;".to_vec());
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
// GetBypassRelay, SetBypassRelay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_bypass_relay_encodes() {
    assert_eq!(GetBypassRelay.to_message().unwrap(), b"^BP;".to_vec());
}

#[test]
fn set_bypass_relay_encodes() {
    assert_eq!(
        SetBypassRelay { bypassed: true }.to_message().unwrap(),
        b"^BP1;".to_vec()
    );
    assert_eq!(
        SetBypassRelay { bypassed: false }.to_message().unwrap(),
        b"^BP0;".to_vec()
    );
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
    assert_eq!(GetDemoMode.to_message().unwrap(), b"^DM;".to_vec());
}

#[test]
fn set_demo_mode_encodes() {
    assert_eq!(
        SetDemoMode { enabled: true }.to_message().unwrap(),
        b"^DM1;".to_vec()
    );
    assert_eq!(
        SetDemoMode { enabled: false }.to_message().unwrap(),
        b"^DM0;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetDisplaySelect, SetDisplaySelect
// ------------------------------------------------------------------------------------------------

#[test]
fn get_display_select_encodes() {
    assert_eq!(GetDisplaySelect.to_message().unwrap(), b"^DS;".to_vec());
}

#[test]
fn set_display_select_encodes() {
    let cmd = SetDisplaySelect { screen: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"^DS2;".to_vec());
}

#[test]
fn set_display_select_accepts_boundary_values() {
    assert!(SetDisplaySelect { screen: 0 }.validate().is_ok());
    assert!(SetDisplaySelect { screen: 3 }.validate().is_ok());
}

#[test]
fn set_display_select_rejects_out_of_range() {
    assert!(matches!(
        SetDisplaySelect { screen: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
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
// GetFrequency, SetFrequency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_frequency_encodes() {
    assert_eq!(GetFrequency.to_message().unwrap(), b"^FQ;".to_vec());
}

#[test]
fn set_frequency_encodes() {
    // SetFrequency has no `validate` — argument_bytes zero-pads to 8 digits unconditionally.
    let cmd = SetFrequency { hz: 7_074_000 };
    assert_eq!(cmd.to_message().unwrap(), b"^FQ07074000;".to_vec());
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
// GetOutputPower
// ------------------------------------------------------------------------------------------------

#[test]
fn get_output_power_encodes() {
    assert_eq!(GetOutputPower.to_message().unwrap(), b"^OP;".to_vec());
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
// GetPeakPowerControl, SetPeakPowerControl
// ------------------------------------------------------------------------------------------------

#[test]
fn get_peak_power_control_encodes() {
    assert_eq!(GetPeakPowerControl.to_message().unwrap(), b"^PC;".to_vec());
}

#[test]
fn set_peak_power_control_encodes() {
    let cmd = SetPeakPowerControl { watts: 1000 };
    assert_eq!(cmd.to_message().unwrap(), b"^PC1000;".to_vec());
}

#[test]
fn set_peak_power_control_accepts_boundary_values() {
    assert!(SetPeakPowerControl { watts: 0 }.validate().is_ok());
    assert!(SetPeakPowerControl { watts: 1500 }.validate().is_ok());
}

#[test]
fn set_peak_power_control_rejects_out_of_range() {
    assert!(matches!(
        SetPeakPowerControl { watts: 1501 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetPttDelay, SetPttDelay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_ptt_delay_encodes() {
    assert_eq!(GetPttDelay.to_message().unwrap(), b"^PD;".to_vec());
}

#[test]
fn set_ptt_delay_encodes() {
    let cmd = SetPttDelay { ms: 250 };
    assert_eq!(cmd.to_message().unwrap(), b"^PD250;".to_vec());
}

#[test]
fn set_ptt_delay_accepts_boundary_values() {
    assert!(SetPttDelay { ms: 0 }.validate().is_ok());
    assert!(SetPttDelay { ms: 500 }.validate().is_ok());
}

#[test]
fn set_ptt_delay_rejects_out_of_range() {
    assert!(matches!(
        SetPttDelay { ms: 501 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetProtectionFaultEnable, SetProtectionFaultEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_protection_fault_enable_encodes() {
    assert_eq!(
        GetProtectionFaultEnable.to_message().unwrap(),
        b"^PF;".to_vec()
    );
}

#[test]
fn set_protection_fault_enable_encodes() {
    assert_eq!(
        SetProtectionFaultEnable { enabled: true }
            .to_message()
            .unwrap(),
        b"^PF1;".to_vec()
    );
    assert_eq!(
        SetProtectionFaultEnable { enabled: false }
            .to_message()
            .unwrap(),
        b"^PF0;".to_vec()
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
// GetPowerStatusSummary
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_status_summary_encodes() {
    assert_eq!(
        GetPowerStatusSummary.to_message().unwrap(),
        b"^PWR;".to_vec()
    );
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
// GetTunePower, SetTunePower
// ------------------------------------------------------------------------------------------------

#[test]
fn get_tune_power_encodes() {
    assert_eq!(GetTunePower.to_message().unwrap(), b"^TP;".to_vec());
}

#[test]
fn set_tune_power_encodes() {
    // SetTunePower has no `validate` — argument_bytes zero-pads to 4 digits unconditionally.
    let cmd = SetTunePower { watts: 500 };
    assert_eq!(cmd.to_message().unwrap(), b"^TP0500;".to_vec());
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
// GetTransceiverVoltage
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transceiver_voltage_encodes() {
    assert_eq!(
        GetTransceiverVoltage.to_message().unwrap(),
        b"^TV;".to_vec()
    );
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
