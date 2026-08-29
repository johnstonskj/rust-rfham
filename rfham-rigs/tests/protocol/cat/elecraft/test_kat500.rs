//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::kat500`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command, Frequency,
        cat::elecraft::kat500::{
            AntennaSide, Bypass, ClearCurrentFault, EepromInit, GetAmplifierInterface, GetAntenna,
            GetAntennaSide, GetAttenuatorState, GetAtuFaultState, GetAtuKeepInPlaceState,
            GetAtuPreset, GetAutoBypassState, GetAutoEnableState, GetBand, GetBaudRate,
            GetCapacitorTopology, GetCapacitorValue, GetDemoModeState, GetErrorMessage,
            GetFanThreshold, GetFaultDelayTime, GetFaultStatus, GetFaultThresholdHigh,
            GetFaultThresholdLow, GetFirmwareVersion, GetFixedBypassState, GetFixedLcState,
            GetForwardPowerA, GetForwardPowerB, GetForwardVoltage, GetFrequency, GetInductance,
            GetInductorSwitch, GetInhibitFan, GetMeterType, GetOperatingMode, GetPowerSensorInput,
            GetPowerStatus, GetReflectedVoltage, GetSerialNumber, GetSwr, GetSwrBypassThreshold,
            GetSwrMeter, GetTunePower, GetTuneSatisfiedSwr, GetTuneState, GetTuningSpeedLimit,
            MeterType, OperatingMode, ResetDevice, SetAmplifierInterface, SetAntenna,
            SetAntennaSide, SetAttenuatorState, SetAtuKeepInPlaceState, SetAtuPreset,
            SetAutoBypassState, SetAutoEnableState, SetBand, SetBaudRate, SetCapacitorTopology,
            SetCapacitorValue, SetDemoModeState, SetFanThreshold, SetFaultDelayTime,
            SetFaultThresholdHigh, SetFaultThresholdLow, SetFixedBypassState, SetFixedLcState,
            SetFrequency, SetInductance, SetInductorSwitch, SetInhibitFan, SetMeterType,
            SetOperatingMode, SetSwrBypassThreshold, SetTunePower, SetTuneSatisfiedSwr,
            SetTuningSpeedLimit, StartTune,
        },
    },
    transport::BaudRate,
};

// ------------------------------------------------------------------------------------------------
// GET-only / action-only commands (bare command_id + ';')
// ------------------------------------------------------------------------------------------------

#[test]
fn get_auto_bypass_state_encodes() {
    assert_eq!(GetAutoBypassState.to_message().unwrap(), b"AB;".to_vec());
}

#[test]
fn get_auto_enable_state_encodes() {
    assert_eq!(GetAutoEnableState.to_message().unwrap(), b"AE;".to_vec());
}

#[test]
fn get_atu_fault_state_encodes() {
    assert_eq!(GetAtuFaultState.to_message().unwrap(), b"AFT;".to_vec());
}

#[test]
fn get_atu_keep_in_place_state_encodes() {
    assert_eq!(
        GetAtuKeepInPlaceState.to_message().unwrap(),
        b"AKIP;".to_vec()
    );
}

#[test]
fn get_amplifier_interface_encodes() {
    assert_eq!(
        GetAmplifierInterface.to_message().unwrap(),
        b"AMPI;".to_vec()
    );
}

#[test]
fn get_antenna_encodes() {
    assert_eq!(GetAntenna.to_message().unwrap(), b"AN;".to_vec());
}

#[test]
fn get_atu_preset_encodes() {
    assert_eq!(GetAtuPreset.to_message().unwrap(), b"AP;".to_vec());
}

#[test]
fn get_attenuator_state_encodes() {
    assert_eq!(GetAttenuatorState.to_message().unwrap(), b"ATTN;".to_vec());
}

#[test]
fn get_band_encodes() {
    assert_eq!(GetBand.to_message().unwrap(), b"BN;".to_vec());
}

#[test]
fn get_baud_rate_encodes() {
    // Notable: this is the one command family in this module using the `#BR` identifier rather
    // than a bare identifier.
    assert_eq!(GetBaudRate.to_message().unwrap(), b"#BR;".to_vec());
}

#[test]
fn bypass_encodes() {
    assert_eq!(Bypass.to_message().unwrap(), b"BYP;".to_vec());
}

#[test]
fn get_capacitor_value_encodes() {
    assert_eq!(GetCapacitorValue.to_message().unwrap(), b"C;".to_vec());
}

#[test]
fn get_capacitor_topology_encodes() {
    assert_eq!(GetCapacitorTopology.to_message().unwrap(), b"CT;".to_vec());
}

#[test]
fn get_demo_mode_state_encodes() {
    assert_eq!(GetDemoModeState.to_message().unwrap(), b"DM;".to_vec());
}

#[test]
fn eeprom_init_encodes() {
    assert_eq!(EepromInit.to_message().unwrap(), b"EEINIT;".to_vec());
}

#[test]
fn get_error_message_encodes() {
    assert_eq!(GetErrorMessage.to_message().unwrap(), b"EM;".to_vec());
}

#[test]
fn get_frequency_encodes() {
    assert_eq!(GetFrequency.to_message().unwrap(), b"F;".to_vec());
}

#[test]
fn get_forward_power_a_encodes() {
    assert_eq!(GetForwardPowerA.to_message().unwrap(), b"FA;".to_vec());
}

#[test]
fn get_forward_power_b_encodes() {
    assert_eq!(GetForwardPowerB.to_message().unwrap(), b"FB;".to_vec());
}

#[test]
fn get_fan_threshold_encodes() {
    assert_eq!(GetFanThreshold.to_message().unwrap(), b"FC;".to_vec());
}

#[test]
fn get_fault_delay_time_encodes() {
    assert_eq!(GetFaultDelayTime.to_message().unwrap(), b"FDT;".to_vec());
}

#[test]
fn get_fault_status_encodes() {
    assert_eq!(GetFaultStatus.to_message().unwrap(), b"FLT;".to_vec());
}

#[test]
fn clear_current_fault_encodes() {
    assert_eq!(ClearCurrentFault.to_message().unwrap(), b"FLTC;".to_vec());
}

#[test]
fn get_tune_satisfied_swr_encodes() {
    assert_eq!(GetTuneSatisfiedSwr.to_message().unwrap(), b"FTNS;".to_vec());
}

#[test]
fn get_fault_threshold_low_encodes() {
    assert_eq!(GetFaultThresholdLow.to_message().unwrap(), b"FT0;".to_vec());
}

#[test]
fn get_fault_threshold_high_encodes() {
    assert_eq!(
        GetFaultThresholdHigh.to_message().unwrap(),
        b"FT1;".to_vec()
    );
}

#[test]
fn get_fixed_lc_state_encodes() {
    assert_eq!(GetFixedLcState.to_message().unwrap(), b"FX;".to_vec());
}

#[test]
fn get_fixed_bypass_state_encodes() {
    assert_eq!(GetFixedBypassState.to_message().unwrap(), b"FY;".to_vec());
}

#[test]
fn get_inductance_encodes() {
    assert_eq!(GetInductance.to_message().unwrap(), b"I;".to_vec());
}

#[test]
fn get_inhibit_fan_encodes() {
    assert_eq!(GetInhibitFan.to_message().unwrap(), b"IF;".to_vec());
}

#[test]
fn get_inductor_switch_encodes() {
    assert_eq!(GetInductorSwitch.to_message().unwrap(), b"L;".to_vec());
}

#[test]
fn get_operating_mode_encodes() {
    assert_eq!(GetOperatingMode.to_message().unwrap(), b"MD;".to_vec());
}

#[test]
fn get_meter_type_encodes() {
    assert_eq!(GetMeterType.to_message().unwrap(), b"MT;".to_vec());
}

#[test]
fn get_power_status_encodes() {
    assert_eq!(GetPowerStatus.to_message().unwrap(), b"PS;".to_vec());
}

#[test]
fn get_power_sensor_input_encodes() {
    assert_eq!(GetPowerSensorInput.to_message().unwrap(), b"PSI;".to_vec());
}

#[test]
fn reset_device_encodes() {
    assert_eq!(ResetDevice.to_message().unwrap(), b"RSTX;".to_vec());
}

#[test]
fn get_firmware_version_encodes() {
    assert_eq!(GetFirmwareVersion.to_message().unwrap(), b"RV;".to_vec());
}

#[test]
fn get_antenna_side_encodes() {
    assert_eq!(GetAntennaSide.to_message().unwrap(), b"SIDE;".to_vec());
}

#[test]
fn get_tuning_speed_limit_encodes() {
    assert_eq!(GetTuningSpeedLimit.to_message().unwrap(), b"SL;".to_vec());
}

#[test]
fn get_swr_meter_encodes() {
    assert_eq!(GetSwrMeter.to_message().unwrap(), b"SM;".to_vec());
}

#[test]
fn get_serial_number_encodes() {
    assert_eq!(GetSerialNumber.to_message().unwrap(), b"SN;".to_vec());
}

#[test]
fn start_tune_encodes() {
    assert_eq!(StartTune.to_message().unwrap(), b"ST;".to_vec());
}

#[test]
fn get_tune_state_encodes() {
    assert_eq!(GetTuneState.to_message().unwrap(), b"T;".to_vec());
}

#[test]
fn get_tune_power_encodes() {
    assert_eq!(GetTunePower.to_message().unwrap(), b"TP;".to_vec());
}

#[test]
fn get_forward_voltage_encodes() {
    assert_eq!(GetForwardVoltage.to_message().unwrap(), b"VFWD;".to_vec());
}

#[test]
fn get_reflected_voltage_encodes() {
    assert_eq!(GetReflectedVoltage.to_message().unwrap(), b"VRFL;".to_vec());
}

#[test]
fn get_swr_encodes() {
    assert_eq!(GetSwr.to_message().unwrap(), b"VSWR;".to_vec());
}

#[test]
fn get_swr_bypass_threshold_encodes() {
    assert_eq!(
        GetSwrBypassThreshold.to_message().unwrap(),
        b"VSWRB;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SET commands: plain boolean state (`{ state }` shorthand => field `on`)
// ------------------------------------------------------------------------------------------------

#[test]
fn set_auto_bypass_state_encodes() {
    assert_eq!(
        SetAutoBypassState { on: true }.to_message().unwrap(),
        b"AB1;".to_vec()
    );
}

#[test]
fn set_auto_enable_state_encodes() {
    assert_eq!(
        SetAutoEnableState { on: true }.to_message().unwrap(),
        b"AE1;".to_vec()
    );
}

#[test]
fn set_atu_keep_in_place_state_encodes() {
    assert_eq!(
        SetAtuKeepInPlaceState { on: true }.to_message().unwrap(),
        b"AKIP1;".to_vec()
    );
}

#[test]
fn set_attenuator_state_encodes() {
    assert_eq!(
        SetAttenuatorState { on: true }.to_message().unwrap(),
        b"ATTN1;".to_vec()
    );
}

#[test]
fn set_demo_mode_state_encodes() {
    assert_eq!(
        SetDemoModeState { on: true }.to_message().unwrap(),
        b"DM1;".to_vec()
    );
}

#[test]
fn set_fixed_lc_state_encodes() {
    assert_eq!(
        SetFixedLcState { on: true }.to_message().unwrap(),
        b"FX1;".to_vec()
    );
}

#[test]
fn set_fixed_bypass_state_encodes() {
    assert_eq!(
        SetFixedBypassState { on: true }.to_message().unwrap(),
        b"FY1;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SET commands: boolean with a custom field name
// ------------------------------------------------------------------------------------------------

#[test]
fn set_amplifier_interface_encodes() {
    assert_eq!(
        SetAmplifierInterface { closed: true }.to_message().unwrap(),
        b"AMPI1;".to_vec()
    );
}

#[test]
fn set_capacitor_topology_encodes() {
    assert_eq!(
        SetCapacitorTopology { hi_z: true }.to_message().unwrap(),
        b"CT1;".to_vec()
    );
}

#[test]
fn set_inhibit_fan_encodes() {
    assert_eq!(
        SetInhibitFan { inhibit: true }.to_message().unwrap(),
        b"IF1;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SET commands: numeric, no `validate`
// ------------------------------------------------------------------------------------------------

#[test]
fn set_atu_preset_encodes() {
    assert_eq!(
        SetAtuPreset { preset: 42 }.to_message().unwrap(),
        b"AP042;".to_vec()
    );
}

#[test]
fn set_capacitor_value_encodes() {
    assert_eq!(
        SetCapacitorValue { value: 200 }.to_message().unwrap(),
        b"C200;".to_vec()
    );
}

#[test]
fn set_frequency_encodes() {
    let cmd = SetFrequency {
        freq_hz: Frequency::from(7_000_000u64),
    };
    assert_eq!(cmd.to_message().unwrap(), b"F07000000;".to_vec());
}

#[test]
fn set_fan_threshold_encodes() {
    assert_eq!(
        SetFanThreshold { threshold_w: 150 }.to_message().unwrap(),
        b"FC150;".to_vec()
    );
}

#[test]
fn set_fault_delay_time_encodes() {
    assert_eq!(
        SetFaultDelayTime { delay_ms: 250 }.to_message().unwrap(),
        b"FDT250;".to_vec()
    );
}

#[test]
fn set_tune_satisfied_swr_encodes() {
    assert_eq!(
        SetTuneSatisfiedSwr { swr_d: 15 }.to_message().unwrap(),
        b"FTNS015;".to_vec()
    );
}

#[test]
fn set_fault_threshold_low_encodes() {
    assert_eq!(
        SetFaultThresholdLow { swr_d: 20 }.to_message().unwrap(),
        b"FT0020;".to_vec()
    );
}

#[test]
fn set_fault_threshold_high_encodes() {
    assert_eq!(
        SetFaultThresholdHigh { swr_d: 30 }.to_message().unwrap(),
        b"FT1030;".to_vec()
    );
}

#[test]
fn set_inductor_switch_encodes() {
    assert_eq!(
        SetInductorSwitch { mask: 5 }.to_message().unwrap(),
        b"L005;".to_vec()
    );
}

#[test]
fn set_tune_power_encodes() {
    assert_eq!(
        SetTunePower { power_w: 100 }.to_message().unwrap(),
        b"TP100;".to_vec()
    );
}

#[test]
fn set_swr_bypass_threshold_encodes() {
    assert_eq!(
        SetSwrBypassThreshold { swr_d: 20 }.to_message().unwrap(),
        b"VSWRB020;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SET commands: numeric, with `validate`
// ------------------------------------------------------------------------------------------------

#[test]
fn set_antenna_encodes() {
    assert_eq!(
        SetAntenna { antenna: 3 }.to_message().unwrap(),
        b"AN3;".to_vec()
    );
}

#[test]
fn set_antenna_accepts_boundary_values() {
    assert!(SetAntenna { antenna: 1 }.validate().is_ok());
    assert!(SetAntenna { antenna: 6 }.validate().is_ok());
}

#[test]
fn set_antenna_rejects_out_of_range() {
    assert!(matches!(
        SetAntenna { antenna: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetAntenna { antenna: 7 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_band_encodes() {
    assert_eq!(SetBand { band: 7 }.to_message().unwrap(), b"BN07;".to_vec());
}

#[test]
fn set_band_accepts_boundary_values() {
    assert!(SetBand { band: 0 }.validate().is_ok());
    assert!(SetBand { band: 13 }.validate().is_ok());
}

#[test]
fn set_band_rejects_out_of_range() {
    assert!(matches!(
        SetBand { band: 14 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_inductance_encodes() {
    assert_eq!(
        SetInductance { tap: 63 }.to_message().unwrap(),
        b"I063;".to_vec()
    );
}

#[test]
fn set_inductance_accepts_boundary_values() {
    assert!(SetInductance { tap: 0 }.validate().is_ok());
    assert!(SetInductance { tap: 63 }.validate().is_ok());
}

#[test]
fn set_inductance_rejects_out_of_range() {
    assert!(matches!(
        SetInductance { tap: 64 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn set_tuning_speed_limit_encodes() {
    assert_eq!(
        SetTuningSpeedLimit { level: 5 }.to_message().unwrap(),
        b"SL5;".to_vec()
    );
}

#[test]
fn set_tuning_speed_limit_accepts_boundary_values() {
    assert!(SetTuningSpeedLimit { level: 0 }.validate().is_ok());
    assert!(SetTuningSpeedLimit { level: 9 }.validate().is_ok());
}

#[test]
fn set_tuning_speed_limit_rejects_out_of_range() {
    assert!(matches!(
        SetTuningSpeedLimit { level: 10 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// SET commands: `for as byte` enum fields -- small enums (<= 4 variants), all variants covered
// ------------------------------------------------------------------------------------------------

#[test]
fn set_operating_mode_encodes_auto() {
    assert_eq!(
        SetOperatingMode {
            mode: OperatingMode::Auto
        }
        .to_message()
        .unwrap(),
        b"MD0;".to_vec()
    );
}

#[test]
fn set_operating_mode_encodes_semi_auto() {
    assert_eq!(
        SetOperatingMode {
            mode: OperatingMode::SemiAuto
        }
        .to_message()
        .unwrap(),
        b"MD1;".to_vec()
    );
}

#[test]
fn set_operating_mode_encodes_manual() {
    assert_eq!(
        SetOperatingMode {
            mode: OperatingMode::Manual
        }
        .to_message()
        .unwrap(),
        b"MD2;".to_vec()
    );
}

#[test]
fn set_meter_type_encodes_swr() {
    assert_eq!(
        SetMeterType {
            meter: MeterType::Swr
        }
        .to_message()
        .unwrap(),
        b"MT0;".to_vec()
    );
}

#[test]
fn set_meter_type_encodes_power() {
    assert_eq!(
        SetMeterType {
            meter: MeterType::Power
        }
        .to_message()
        .unwrap(),
        b"MT1;".to_vec()
    );
}

#[test]
fn set_meter_type_encodes_reflected() {
    assert_eq!(
        SetMeterType {
            meter: MeterType::Reflected
        }
        .to_message()
        .unwrap(),
        b"MT2;".to_vec()
    );
}

#[test]
fn set_antenna_side_encodes_left() {
    assert_eq!(
        SetAntennaSide {
            side: AntennaSide::Left
        }
        .to_message()
        .unwrap(),
        b"SIDE0;".to_vec()
    );
}

#[test]
fn set_antenna_side_encodes_right() {
    assert_eq!(
        SetAntennaSide {
            side: AntennaSide::Right
        }
        .to_message()
        .unwrap(),
        b"SIDE1;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetBaudRate / SetBaudRate: unique `#BR` identifier, and only 4 of the `BaudRate` variants are
// accepted (unsupported variants are rejected inside `argument_bytes`, surfaced via `to_message`).
// ------------------------------------------------------------------------------------------------

#[test]
fn set_baud_rate_encodes_4800() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd4800,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR0;".to_vec());
}

#[test]
fn set_baud_rate_encodes_9600() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd9600,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR1;".to_vec());
}

#[test]
fn set_baud_rate_encodes_19200() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd19200,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR2;".to_vec());
}

#[test]
fn set_baud_rate_encodes_38400() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd38400,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR3;".to_vec());
}

#[test]
fn set_baud_rate_rejects_unsupported_rate() {
    // Unlike the other range-checked commands in this module, this rejection happens inside
    // `argument_bytes` (there is no separate `validate`/`if valid_fn` on `SetBaudRate`), so it
    // surfaces through `to_message` itself rather than through `Command::validate`.
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd300,
    };
    assert!(matches!(
        cmd.to_message(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}
