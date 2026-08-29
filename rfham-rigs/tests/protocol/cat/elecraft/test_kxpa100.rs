//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::kxpa100`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command,
        cat::elecraft::kxpa100::{
            ClearFault, GetAdcReadings, GetAntennaSelection, GetAtuEnable, GetAutoBiasEnable,
            GetBandSelection, GetBusyStatus, GetDemoMode, GetDrainCurrent, GetErrorCount,
            GetErrorMessage, GetFanThreshold, GetFaultCode, GetFirmwareVersion, GetFrequency,
            GetFrequencyEntryMode, GetLowPassRelay, GetMeterDisplay, GetOperatingMode,
            GetOutputPower, GetPaTemperature, GetPaVoltage, GetPcBaudRate, GetPeakPowerControl,
            GetPowerInput, GetProtectionFaultEnable, GetPttDelay, GetRadioInterface, GetRfSense,
            GetSerialNumber, GetSupplyVoltage, GetSwrFaultEnable, GetSwrInhibitThreshold,
            GetSwrMeter, GetTransceiverPowerLevel, GetTunePower, GetXcvrBaudRate, InitiateTune,
            ResetConfiguration, SetAntennaSelection, SetAtuEnable, SetAutoBiasEnable,
            SetBandSelection, SetDemoMode, SetFanThreshold, SetFrequency, SetFrequencyEntryMode,
            SetLowPassRelay, SetMeterDisplay, SetOperatingMode, SetPcBaudRate, SetPeakPowerControl,
            SetProtectionFaultEnable, SetPttDelay, SetRadioInterface, SetSwrFaultEnable,
            SetSwrInhibitThreshold, SetTunePower, SetXcvrBaudRate,
        },
    },
};

// ------------------------------------------------------------------------------------------------
// GetAdcReadings
// ------------------------------------------------------------------------------------------------

#[test]
fn get_adc_readings_encodes() {
    assert_eq!(GetAdcReadings.to_message().unwrap(), b"^AD;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetAutoBiasEnable, SetAutoBiasEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_auto_bias_enable_encodes() {
    assert_eq!(GetAutoBiasEnable.to_message().unwrap(), b"^AE;".to_vec());
}

#[test]
fn set_auto_bias_enable_encodes_on() {
    let cmd = SetAutoBiasEnable { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"^AE1;".to_vec());
}

#[test]
fn set_auto_bias_enable_encodes_off() {
    let cmd = SetAutoBiasEnable { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"^AE0;".to_vec());
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
// GetAtuEnable, SetAtuEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_atu_enable_encodes() {
    assert_eq!(GetAtuEnable.to_message().unwrap(), b"^AT;".to_vec());
}

#[test]
fn set_atu_enable_encodes_on() {
    let cmd = SetAtuEnable { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"^AT1;".to_vec());
}

#[test]
fn set_atu_enable_encodes_off() {
    let cmd = SetAtuEnable { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"^AT0;".to_vec());
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
    let cmd = SetPcBaudRate { rate: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"^BRP1;".to_vec());
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
    let cmd = SetXcvrBaudRate { rate: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"^BRX2;".to_vec());
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
// GetBusyStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_busy_status_encodes() {
    assert_eq!(GetBusyStatus.to_message().unwrap(), b"^BY;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// ResetConfiguration
// ------------------------------------------------------------------------------------------------

#[test]
fn reset_configuration_encodes() {
    assert_eq!(ResetConfiguration.to_message().unwrap(), b"^CR;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetDemoMode, SetDemoMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_demo_mode_encodes() {
    assert_eq!(GetDemoMode.to_message().unwrap(), b"^DM;".to_vec());
}

#[test]
fn set_demo_mode_encodes_on() {
    let cmd = SetDemoMode { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"^DM1;".to_vec());
}

#[test]
fn set_demo_mode_encodes_off() {
    let cmd = SetDemoMode { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"^DM0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetErrorCount
// ------------------------------------------------------------------------------------------------

#[test]
fn get_error_count_encodes() {
    assert_eq!(GetErrorCount.to_message().unwrap(), b"^EC;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetErrorMessage
// ------------------------------------------------------------------------------------------------

#[test]
fn get_error_message_encodes() {
    assert_eq!(GetErrorMessage.to_message().unwrap(), b"^EM;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFrequency, SetFrequency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_frequency_encodes() {
    assert_eq!(GetFrequency.to_message().unwrap(), b"^F;".to_vec());
}

#[test]
fn set_frequency_encodes() {
    let cmd = SetFrequency { hz: 7_100_000 };
    assert_eq!(cmd.to_message().unwrap(), b"^F07100000;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFrequencyEntryMode, SetFrequencyEntryMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_frequency_entry_mode_encodes() {
    assert_eq!(
        GetFrequencyEntryMode.to_message().unwrap(),
        b"^FE;".to_vec()
    );
}

#[test]
fn set_frequency_entry_mode_encodes_manual() {
    let cmd = SetFrequencyEntryMode { manual: true };
    assert_eq!(cmd.to_message().unwrap(), b"^FE1;".to_vec());
}

#[test]
fn set_frequency_entry_mode_encodes_automatic() {
    let cmd = SetFrequencyEntryMode { manual: false };
    assert_eq!(cmd.to_message().unwrap(), b"^FE0;".to_vec());
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
// GetFanThreshold, SetFanThreshold
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fan_threshold_encodes() {
    assert_eq!(GetFanThreshold.to_message().unwrap(), b"^FT;".to_vec());
}

#[test]
fn set_fan_threshold_encodes() {
    let cmd = SetFanThreshold { celsius: 60 };
    assert_eq!(cmd.to_message().unwrap(), b"^FT060;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetDrainCurrent
// ------------------------------------------------------------------------------------------------

#[test]
fn get_drain_current_encodes() {
    assert_eq!(GetDrainCurrent.to_message().unwrap(), b"^I;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetLowPassRelay, SetLowPassRelay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_low_pass_relay_encodes() {
    assert_eq!(GetLowPassRelay.to_message().unwrap(), b"^LR;".to_vec());
}

#[test]
fn set_low_pass_relay_encodes_in_line() {
    let cmd = SetLowPassRelay { in_line: true };
    assert_eq!(cmd.to_message().unwrap(), b"^LR1;".to_vec());
}

#[test]
fn set_low_pass_relay_encodes_bypass() {
    let cmd = SetLowPassRelay { in_line: false };
    assert_eq!(cmd.to_message().unwrap(), b"^LR0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetOperatingMode, SetOperatingMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_operating_mode_encodes() {
    assert_eq!(GetOperatingMode.to_message().unwrap(), b"^MD;".to_vec());
}

#[test]
fn set_operating_mode_encodes_operate() {
    let cmd = SetOperatingMode { operate: true };
    assert_eq!(cmd.to_message().unwrap(), b"^MD1;".to_vec());
}

#[test]
fn set_operating_mode_encodes_standby() {
    let cmd = SetOperatingMode { operate: false };
    assert_eq!(cmd.to_message().unwrap(), b"^MD0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMeterDisplay, SetMeterDisplay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_meter_display_encodes() {
    assert_eq!(GetMeterDisplay.to_message().unwrap(), b"^MT;".to_vec());
}

#[test]
fn set_meter_display_encodes() {
    let cmd = SetMeterDisplay { selection: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"^MT2;".to_vec());
}

#[test]
fn set_meter_display_accepts_boundary_values() {
    assert!(SetMeterDisplay { selection: 0 }.validate().is_ok());
    assert!(SetMeterDisplay { selection: 3 }.validate().is_ok());
}

#[test]
fn set_meter_display_rejects_out_of_range() {
    assert!(matches!(
        SetMeterDisplay { selection: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetOutputPower
// ------------------------------------------------------------------------------------------------

#[test]
fn get_output_power_encodes() {
    assert_eq!(GetOutputPower.to_message().unwrap(), b"^OP;".to_vec());
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
    let cmd = SetPeakPowerControl { percent: 75 };
    assert_eq!(cmd.to_message().unwrap(), b"^PC075;".to_vec());
}

#[test]
fn set_peak_power_control_accepts_boundary_values() {
    assert!(SetPeakPowerControl { percent: 0 }.validate().is_ok());
    assert!(SetPeakPowerControl { percent: 100 }.validate().is_ok());
}

#[test]
fn set_peak_power_control_rejects_out_of_range() {
    assert!(matches!(
        SetPeakPowerControl { percent: 101 }.validate(),
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
fn set_protection_fault_enable_encodes_on() {
    let cmd = SetProtectionFaultEnable { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"^PF1;".to_vec());
}

#[test]
fn set_protection_fault_enable_encodes_off() {
    let cmd = SetProtectionFaultEnable { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"^PF0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetPowerInput
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_input_encodes() {
    assert_eq!(GetPowerInput.to_message().unwrap(), b"^PI;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetPaVoltage
// ------------------------------------------------------------------------------------------------

#[test]
fn get_pa_voltage_encodes() {
    assert_eq!(GetPaVoltage.to_message().unwrap(), b"^PV;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetRfSense
// ------------------------------------------------------------------------------------------------

#[test]
fn get_rf_sense_encodes() {
    assert_eq!(GetRfSense.to_message().unwrap(), b"^RS;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

#[test]
fn get_firmware_version_encodes() {
    assert_eq!(GetFirmwareVersion.to_message().unwrap(), b"^RV;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSwrInhibitThreshold, SetSwrInhibitThreshold
// ------------------------------------------------------------------------------------------------

#[test]
fn get_swr_inhibit_threshold_encodes() {
    assert_eq!(
        GetSwrInhibitThreshold.to_message().unwrap(),
        b"^SI;".to_vec()
    );
}

#[test]
fn set_swr_inhibit_threshold_encodes() {
    let cmd = SetSwrInhibitThreshold { swr_d: 30 };
    assert_eq!(cmd.to_message().unwrap(), b"^SI30;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSwrMeter
// ------------------------------------------------------------------------------------------------

#[test]
fn get_swr_meter_encodes() {
    assert_eq!(GetSwrMeter.to_message().unwrap(), b"^SM;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSerialNumber
// ------------------------------------------------------------------------------------------------

#[test]
fn get_serial_number_encodes() {
    assert_eq!(GetSerialNumber.to_message().unwrap(), b"^SN;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSupplyVoltage
// ------------------------------------------------------------------------------------------------

#[test]
fn get_supply_voltage_encodes() {
    assert_eq!(GetSupplyVoltage.to_message().unwrap(), b"^SV;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSwrFaultEnable, SetSwrFaultEnable
// ------------------------------------------------------------------------------------------------

#[test]
fn get_swr_fault_enable_encodes() {
    assert_eq!(GetSwrFaultEnable.to_message().unwrap(), b"^SW;".to_vec());
}

#[test]
fn set_swr_fault_enable_encodes_on() {
    let cmd = SetSwrFaultEnable { enabled: true };
    assert_eq!(cmd.to_message().unwrap(), b"^SW1;".to_vec());
}

#[test]
fn set_swr_fault_enable_encodes_off() {
    let cmd = SetSwrFaultEnable { enabled: false };
    assert_eq!(cmd.to_message().unwrap(), b"^SW0;".to_vec());
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
    let cmd = SetTunePower { watts: 8 };
    assert_eq!(cmd.to_message().unwrap(), b"^TP008;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// InitiateTune
// ------------------------------------------------------------------------------------------------

#[test]
fn initiate_tune_encodes() {
    assert_eq!(InitiateTune.to_message().unwrap(), b"^TU;".to_vec());
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
        interface_type: 2,
        option: 1,
    };
    assert_eq!(cmd.to_message().unwrap(), b"^XI021;".to_vec());
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
fn set_radio_interface_rejects_out_of_range_interface_type() {
    assert!(matches!(
        SetRadioInterface {
            interface_type: 4,
            option: 0,
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_radio_interface_rejects_out_of_range_option() {
    assert!(matches!(
        SetRadioInterface {
            interface_type: 0,
            option: 2,
        }
        .validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetTransceiverPowerLevel
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transceiver_power_level_encodes() {
    assert_eq!(
        GetTransceiverPowerLevel.to_message().unwrap(),
        b"^XP;".to_vec()
    );
}
