//!
//! CAT commands for the Elecraft KXPA100 100 W solid-state HF amplifier.
//!
//! All KXPA100 command identifiers carry a leading caret (`^`), for example `^BN05;`. The GET form
//! of a command is the bare command letters with no data, e.g. `^BN;`.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft KXPA100 Programmer's Reference](https://ftp.elecraft.com/KXPA/Manuals%20Downloads/KXPA100%20Amplifier%20Command%20Reference.pdf), Feb 2014.
//!

use crate::{
    error::{RigError, enum_parse},
    protocol::cat::common::{
        bytes_to_vec, format_uint_ascii, u8_from_ascii, u16_from_ascii, u32_from_ascii,
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetAdcReadings, AdcReadings
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the amplifier's ADC readings (PA drain voltage, drain current, and supply
voltage).

# Command format

> `^AD;`

# Response format

> `^AD{vvv} {iii} {sss};`

Where:

* *vvv* is the PA drain voltage, in tenths of a volt (e.g. `135` represents 13.5 V).
* *iii* is the PA drain current, in tenths of an ampere (e.g. `087` represents 8.7 A).
* *sss* is the supply voltage, in tenths of a volt." =>
    GetAdcReadings
);

define_command_struct!(
    "Represents the parsed ADC readings response, as returned by [`GetAdcReadings`]." =>
    AdcReadings {
        "The PA drain voltage, in tenths of a volt." =>
        drain_voltage_dv: u16,
        "The PA drain current, in tenths of an ampere." =>
        drain_current_da: u16,
        "The supply voltage, in tenths of a volt." =>
        supply_voltage_dv: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoBiasEnable, SetAutoBiasEnable
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether auto-bias is enabled.

# Command format

> `^AE;`

# Response format

> `^AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAutoBiasEnable
);

define_cat_command!("Set whether auto-bias is enabled.

# Command format

> `^AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoBiasEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntennaSelection, SetAntennaSelection
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the selected antenna port.

# Command format

> `^AN;`

# Response format

> `^AN{n};`

Where *n* is the antenna port, `1` or `2`." =>
    GetAntennaSelection
);

define_cat_command!("Set the antenna port.

# Command format

> `^AN{n};`

Where *n* is the antenna port, `1` or `2`." =>
    SetAntennaSelection {
        antenna: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuEnable, SetAtuEnable
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the internal ATU is switched into circuit.

# Command format

> `^AT;`

# Response format

> `^AT{n};`

Where `n` is `0` (bypass) or `1` (in-line)." =>
    GetAtuEnable
);

define_cat_command!("Set whether the internal ATU is switched into circuit.

# Command format

> `^AT{n};`

Where `n` is `0` (bypass) or `1` (in-line)." =>
    SetAtuEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandSelection, SetBandSelection
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the currently selected band.

# Command format

> `^BN;`

# Response format

> `^BN{nn};`

Where *nn* is one of:

* `00`; 160 m.
* `01`; 80 m.
* `02`; 60 m.
* `03`; 40 m.
* `04`; 30 m.
* `05`; 20 m.
* `06`; 17 m.
* `07`; 15 m.
* `08`; 12 m.
* `09`; 10 m.
* `10`; 6 m." =>
    GetBandSelection
);

define_cat_command!("Set the active band.

# Command format

> `^BN{nn};`

Where *nn* is one of:

* `00`; 160 m.
* `01`; 80 m.
* `02`; 60 m.
* `03`; 40 m.
* `04`; 30 m.
* `05`; 20 m.
* `06`; 17 m.
* `07`; 15 m.
* `08`; 12 m.
* `09`; 10 m.
* `10`; 6 m." =>
    SetBandSelection {
        band: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPcBaudRate, SetPcBaudRate
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PC-side RS-232 port data rate.

# Command format

> `^BRP;`

# Response format

> `^BRP{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    GetPcBaudRate
);

define_cat_command!("Set the PC-side RS-232 port data rate.

# Command format

> `^BRP{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    SetPcBaudRate {
        rate: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetXcvrBaudRate, SetXcvrBaudRate
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the transceiver-side RS-232 port data rate.

# Command format

> `^BRX;`

# Response format

> `^BRX{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    GetXcvrBaudRate
);

define_cat_command!("Set the transceiver-side RS-232 port data rate.

# Command format

> `^BRX{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    SetXcvrBaudRate {
        rate: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBusyStatus, BusyStatus
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the busy and PTT status.

# Command format

> `^BY;`

# Response format

> `^BY{b}{p};`

Where:

* *b* is the boolean state `0` (not busy) or `1` (busy).
* *p* is the boolean state `0` (PTT not asserted) or `1` (PTT asserted)." =>
    GetBusyStatus
);

define_command_struct!(
    "Represents the parsed busy/PTT status response, as returned by [`GetBusyStatus`]." =>
    BusyStatus {
        "`true` if the amplifier is busy." =>
        busy: bool,
        "`true` if PTT is currently asserted." =>
        ptt_asserted: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: ResetConfiguration
// ------------------------------------------------------------------------------------------------

define_cat_command!("Reset the amplifier configuration to factory defaults.

There is no response to this command.

# Command format

> `^CR;`" =>
    ResetConfiguration
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDemoMode, SetDemoMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether demo mode is enabled.

# Command format

> `^DM;`

# Response format

> `^DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetDemoMode
);

define_cat_command!("Set whether demo mode is enabled.

# Command format

> `^DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetDemoMode {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetErrorCount
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the accumulated error count since the last reset.

# Command format

> `^EC;`

# Response format

> `^EC{nnnn};`

Where *nnnn* is the 4-digit error count." =>
    GetErrorCount
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetErrorMessage
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the most recent error message.

# Command format

> `^EM;`

# Response format

> `^EM{text};`

The response is a variable-length ASCII text string, returned as raw bytes." =>
    GetErrorMessage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFrequency, SetFrequency
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the operating frequency currently reported to the amplifier.

# Command format

> `^F;`

# Response format

> `^F{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    GetFrequency
);

define_cat_command!("Inform the amplifier of the operating frequency, in Hz.

# Command format

> `^F{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    SetFrequency {
        hz: u32
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFrequencyEntryMode, SetFrequencyEntryMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the frequency entry source.

# Command format

> `^FE;`

# Response format

> `^FE{n};`

Where `n` is `0` (frequency is read automatically from the transceiver) or `1` (frequency is set
manually via [`SetFrequency`])." =>
    GetFrequencyEntryMode
);

define_cat_command!("Set the frequency entry source.

# Command format

> `^FE{n};`

Where `n` is `0` (automatic, from the transceiver) or `1` (manual, via [`SetFrequency`])." =>
    SetFrequencyEntryMode {
        manual: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultCode, ClearFault
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current fault code.

# Command format

> `^FL;`

# Response format

> `^FL{nn};`

Where *nn* is the 2-digit fault code; `00` indicates no fault." =>
    GetFaultCode
);

define_cat_command!("Clear the current fault condition.

There is no response to this command.

# Command format

> `^FLC;`" =>
    ClearFault
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFanThreshold, SetFanThreshold
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the fan-on threshold temperature.

# Command format

> `^FT;`

# Response format

> `^FT{nnn};`

Where *nnn* is the 3-digit threshold, in degrees Celsius, above which the cooling fan turns on." =>
    GetFanThreshold
);

define_cat_command!("Set the fan-on threshold temperature.

# Command format

> `^FT{nnn};`

Where *nnn* is the 3-digit threshold, in degrees Celsius, above which the cooling fan turns on." =>
    SetFanThreshold {
        celsius: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDrainCurrent
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PA drain current.

# Command format

> `^I;`

# Response format

> `^I{nnn};`

Where *nnn* is the drain current, in tenths of an ampere (e.g. `087` represents 8.7 A)." =>
    GetDrainCurrent
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetLowPassRelay, SetLowPassRelay
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the low-pass filter relay state.

# Command format

> `^LR;`

# Response format

> `^LR{n};`

Where `n` is `0` (bypass) or `1` (in-line)." =>
    GetLowPassRelay
);

define_cat_command!("Set the low-pass filter relay state.

# Command format

> `^LR{n};`

Where `n` is `0` (bypass) or `1` (in-line)." =>
    SetLowPassRelay {
        in_line: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOperatingMode, SetOperatingMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current operating mode.

# Command format

> `^MD;`

# Response format

> `^MD{n};`

Where `n` is `0` (standby) or `1` (operate)." =>
    GetOperatingMode
);

define_cat_command!("Set the operating mode.

# Command format

> `^MD{n};`

Where `n` is `0` (standby) or `1` (operate)." =>
    SetOperatingMode {
        operate: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMeterDisplay, SetMeterDisplay
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the front-panel meter display selection.

# Command format

> `^MT;`

# Response format

> `^MT{n};`

Where *n* is one of:

* `0`; output power.
* `1`; SWR.
* `2`; PA current.
* `3`; supply voltage." =>
    GetMeterDisplay
);

define_cat_command!("Set the front-panel meter display selection.

# Command format

> `^MT{n};`

Where *n* is one of:

* `0`; output power.
* `1`; SWR.
* `2`; PA current.
* `3`; supply voltage." =>
    SetMeterDisplay {
        selection: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOutputPower
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current RF output power.

# Command format

> `^OP;`

# Response format

> `^OP{nnn};`

Where *nnn* is the output power, in watts, between `000` and `100`." =>
    GetOutputPower
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPeakPowerControl, SetPeakPowerControl
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the peak-power control setting.

# Command format

> `^PC;`

# Response format

> `^PC{nnn};`

Where *nnn* is the peak power limit, as a percentage of rated output, between `000` and `100`." =>
    GetPeakPowerControl
);

define_cat_command!("Set the peak-power control setting.

# Command format

> `^PC{nnn};`

Where *nnn* is the peak power limit, as a percentage of rated output, between `000` and `100`." =>
    SetPeakPowerControl {
        percent: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPttDelay, SetPttDelay
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PTT delay.

# Command format

> `^PD;`

# Response format

> `^PD{nnn};`

Where *nnn* is the PTT delay, in milliseconds, between `000` and `500`." =>
    GetPttDelay
);

define_cat_command!("Set the PTT delay.

# Command format

> `^PD{nnn};`

Where *nnn* is the PTT delay, in milliseconds, between `000` and `500`." =>
    SetPttDelay {
        ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetProtectionFaultEnable, SetProtectionFaultEnable
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether protection-fault detection is enabled.

# Command format

> `^PF;`

# Response format

> `^PF{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetProtectionFaultEnable
);

define_cat_command!("Set whether protection-fault detection is enabled.

# Command format

> `^PF{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetProtectionFaultEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerInput
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the DC supply input voltage.

# Command format

> `^PI;`

# Response format

> `^PI{nnn};`

Where *nnn* is the supply voltage, in tenths of a volt (e.g. `135` represents 13.5 V)." =>
    GetPowerInput
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPaVoltage
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PA drain voltage.

# Command format

> `^PV;`

# Response format

> `^PV{nnn};`

Where *nnn* is the drain voltage, in tenths of a volt." =>
    GetPaVoltage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRfSense
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the RF sense level detected at the amplifier input.

# Command format

> `^RS;`

# Response format

> `^RS{nnn};`

Where *nnn* is the relative RF input level, between `000` and `100`." =>
    GetRfSense
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the firmware version string.

# Command format

> `^RV;`

# Response format

> `^RV{text};`

The response is a fixed 5-character ASCII version string, returned as raw bytes, e.g.
`^RV01.18;`." =>
    GetFirmwareVersion
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSwrInhibitThreshold, SetSwrInhibitThreshold
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the SWR protection-inhibit threshold.

# Command format

> `^SI;`

# Response format

> `^SI{nn};`

Where *nn* is SWR × 10, e.g. `30` represents an SWR of 3.0:1." =>
    GetSwrInhibitThreshold
);

define_cat_command!("Set the SWR protection-inhibit threshold.

# Command format

> `^SI{nn};`

Where *nn* is SWR × 10, e.g. `30` represents an SWR of 3.0:1." =>
    SetSwrInhibitThreshold {
        swr_d: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSwrMeter
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current SWR reading.

The value is `0` when the amplifier is not transmitting.

# Command format

> `^SM;`

# Response format

> `^SM{nn};`

Where *nn* is SWR × 10, e.g. `30` represents an SWR of 3.0:1." =>
    GetSwrMeter
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSerialNumber
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the amplifier serial number.

# Command format

> `^SN;`

# Response format

> `^SN{nnnnn};`

Where *nnnnn* is the 5-digit serial number." =>
    GetSerialNumber
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSupplyVoltage
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the DC supply voltage.

# Command format

> `^SV;`

# Response format

> `^SV{nnn};`

Where *nnn* is the supply voltage, in tenths of a volt." =>
    GetSupplyVoltage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSwrFaultEnable, SetSwrFaultEnable
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether SWR fault protection is enabled.

# Command format

> `^SW;`

# Response format

> `^SW{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetSwrFaultEnable
);

define_cat_command!("Set whether SWR fault protection is enabled.

# Command format

> `^SW{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetSwrFaultEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPaTemperature
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PA heatsink temperature.

# Command format

> `^TM;`

# Response format

> `^TM{nnn};`

Where *nnn* is the temperature, in degrees Celsius, between `000` and `150`." =>
    GetPaTemperature
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTunePower, SetTunePower
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the RF power level used during an ATU tune cycle.

# Command format

> `^TP;`

# Response format

> `^TP{nnn};`

Where *nnn* is the tune power, in watts, typically between `005` and `010`." =>
    GetTunePower
);

define_cat_command!("Set the RF power level used during an ATU tune cycle.

# Command format

> `^TP{nnn};`

Where *nnn* is the tune power, in watts, typically between `005` and `010`." =>
    SetTunePower {
        watts: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: InitiateTune
// ------------------------------------------------------------------------------------------------

define_cat_command!("Initiate an ATU tune cycle.

This command has no response. The tune cycle only proceeds if the internal ATU is enabled; see
[`SetAtuEnable`].

# Command format

> `^TU;`" =>
    InitiateTune
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRadioInterface, SetRadioInterface, RadioInterface
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the radio interface type and option bit.

# Command format

> `^XI;`

# Response format

> `^XI{nn}{o};`

Where:

* *nn* is the interface type: `00` (K3), `01` (BCD), `02` (Analog), `03` (Serial).
* *o* is the option bit, `0` or `1`; its meaning depends on *nn*." =>
    GetRadioInterface
);

define_cat_command!("Set the radio interface type and option bit.

# Command format

> `^XI{nn}{o};`

Where:

* *nn* is the interface type: `00` (K3), `01` (BCD), `02` (Analog), `03` (Serial).
* *o* is the option bit, `0` or `1`; its meaning depends on *nn*." =>
    SetRadioInterface {
        interface_type: u8,
        option: u8
    }
);

define_command_struct!(
    "Represents the parsed radio interface type and option bit, as returned by
    [`GetRadioInterface`]." =>
    RadioInterface {
        "The interface type: `0` (K3), `1` (BCD), `2` (Analog), `3` (Serial)." =>
        interface_type: RadioInterfaceType,
        "The option bit, `0` or `1`; its meaning depends on `interface_type`." =>
        option: u8
    }
);

define_command_enum!(
    "Represents the radio interface type, as returned by [`GetRadioInterface`] or set by [`SetRadioInterface`]." =>
    RadioInterfaceType {
        K3 = 0,
        Bcd = 1,
        Analog = 2,
        Serial = 3
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverPowerLevel
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the transceiver drive power level detected by the amplifier.

# Command format

> `^XP;`

# Response format

> `^XP{nnn};`

Where *nnn* is the relative drive power level, between `000` and `100`." =>
    GetTransceiverPowerLevel
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAdcReadings => b"^AD");
impl_cat_command_with_response!(GetAdcReadings => 11, |bytes: &[u8]| {
    Ok(AdcReadings {
        drain_voltage_dv: u16_from_ascii(&bytes[0..3])?,
        drain_current_da: u16_from_ascii(&bytes[4..7])?,
        supply_voltage_dv: u16_from_ascii(&bytes[8..11])?,
    })
} => AdcReadings);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAutoBiasEnable => b"^AE");
impl_cat_command_with_response!(GetAutoBiasEnable => boolean);

impl_cat_command!(SetAutoBiasEnable => b"^AE" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAntennaSelection => b"^AN");
impl_cat_command_with_response!(GetAntennaSelection => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetAntennaSelection => b"^AN"
    format antenna uint 1,
    if |cmd: &SetAntennaSelection| {
        if (1..=2).contains(&cmd.antenna) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "antenna",
                type_name: "u8",
                value: cmd.antenna.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAtuEnable => b"^AT");
impl_cat_command_with_response!(GetAtuEnable => boolean);

impl_cat_command!(SetAtuEnable => b"^AT" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBandSelection => b"^BN");
impl_cat_command_with_response!(GetBandSelection => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetBandSelection => b"^BN"
    format band uint 2,
    if |cmd: &SetBandSelection| {
        if cmd.band <= 10 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "band",
                type_name: "u8",
                value: cmd.band.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPcBaudRate => b"^BRP");
impl_cat_command_with_response!(GetPcBaudRate => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetPcBaudRate => b"^BRP"
    format rate uint 1,
    if |cmd: &SetPcBaudRate| {
        if cmd.rate <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "rate",
                type_name: "u8",
                value: cmd.rate.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetXcvrBaudRate => b"^BRX");
impl_cat_command_with_response!(GetXcvrBaudRate => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetXcvrBaudRate => b"^BRX"
    format rate uint 1,
    if |cmd: &SetXcvrBaudRate| {
        if cmd.rate <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "rate",
                type_name: "u8",
                value: cmd.rate.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBusyStatus => b"^BY");
impl_cat_command_with_response!(GetBusyStatus => 2, |bytes: &[u8]| {
    Ok(BusyStatus {
        busy: bytes[0] == b'1',
        ptt_asserted: bytes[1] == b'1',
    })
} => BusyStatus);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(ResetConfiguration => b"^CR");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDemoMode => b"^DM");
impl_cat_command_with_response!(GetDemoMode => boolean);

impl_cat_command!(SetDemoMode => b"^DM" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetErrorCount => b"^EC");
impl_cat_command_with_response!(GetErrorCount => 4, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetErrorMessage => b"^EM");
impl_cat_command_with_response!(GetErrorMessage => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFrequency => b"^F");
impl_cat_command_with_response!(GetFrequency => 8, u32_from_ascii => u32);

impl_cat_command!(SetFrequency => b"^F" with Some |cmd: &SetFrequency| {
    format!("{:08}", cmd.hz).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFrequencyEntryMode => b"^FE");
impl_cat_command_with_response!(GetFrequencyEntryMode => boolean);

impl_cat_command!(SetFrequencyEntryMode => b"^FE" for boolean manual);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultCode => b"^FL");
impl_cat_command_with_response!(GetFaultCode => 2, u8_from_ascii => u8);

impl_cat_command!(ClearFault => b"^FL" with Some |_: &ClearFault| {
    vec![b'C']
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFanThreshold => b"^FT");
impl_cat_command_with_response!(GetFanThreshold => 3, u8_from_ascii => u8);

impl_cat_command!(SetFanThreshold => b"^FT" format celsius uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDrainCurrent => b"^I");
impl_cat_command_with_response!(GetDrainCurrent => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetLowPassRelay => b"^LR");
impl_cat_command_with_response!(GetLowPassRelay => boolean);

impl_cat_command!(SetLowPassRelay => b"^LR" for boolean in_line);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetOperatingMode => b"^MD");
impl_cat_command_with_response!(GetOperatingMode => boolean);

impl_cat_command!(SetOperatingMode => b"^MD" for boolean operate);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMeterDisplay => b"^MT");
impl_cat_command_with_response!(GetMeterDisplay => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetMeterDisplay => b"^MT"
    format selection uint 1,
    if |cmd: &SetMeterDisplay| {
        if cmd.selection <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "selection",
                type_name: "u8",
                value: cmd.selection.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetOutputPower => b"^OP");
impl_cat_command_with_response!(GetOutputPower => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPeakPowerControl => b"^PC");
impl_cat_command_with_response!(GetPeakPowerControl => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetPeakPowerControl => b"^PC"
    format percent uint 3,
    if |cmd: &SetPeakPowerControl| {
        if cmd.percent <= 100 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "percent",
                type_name: "u8",
                value: cmd.percent.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPttDelay => b"^PD");
impl_cat_command_with_response!(GetPttDelay => 3, u16_from_ascii => u16);

impl_cat_command!(
    SetPttDelay => b"^PD"
    format ms uint 3,
    if |cmd: &SetPttDelay| {
        if cmd.ms <= 500 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "ms",
                type_name: "u16",
                value: cmd.ms.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetProtectionFaultEnable => b"^PF");
impl_cat_command_with_response!(GetProtectionFaultEnable => boolean);

impl_cat_command!(SetProtectionFaultEnable => b"^PF" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerInput => b"^PI");
impl_cat_command_with_response!(GetPowerInput => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPaVoltage => b"^PV");
impl_cat_command_with_response!(GetPaVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetRfSense => b"^RS");
impl_cat_command_with_response!(GetRfSense => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFirmwareVersion => b"^RV");
impl_cat_command_with_response!(GetFirmwareVersion => 5, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSwrInhibitThreshold => b"^SI");
impl_cat_command_with_response!(GetSwrInhibitThreshold => 2, u8_from_ascii => u8);

impl_cat_command!(SetSwrInhibitThreshold => b"^SI" format swr_d uint 2);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSwrMeter => b"^SM");
impl_cat_command_with_response!(GetSwrMeter => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSerialNumber => b"^SN");
impl_cat_command_with_response!(GetSerialNumber => 5, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSupplyVoltage => b"^SV");
impl_cat_command_with_response!(GetSupplyVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSwrFaultEnable => b"^SW");
impl_cat_command_with_response!(GetSwrFaultEnable => boolean);

impl_cat_command!(SetSwrFaultEnable => b"^SW" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPaTemperature => b"^TM");
impl_cat_command_with_response!(GetPaTemperature => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTunePower => b"^TP");
impl_cat_command_with_response!(GetTunePower => 3, u8_from_ascii => u8);

impl_cat_command!(SetTunePower => b"^TP" format watts uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(InitiateTune => b"^TU");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetRadioInterface => b"^XI");
impl_cat_command_with_response!(GetRadioInterface => 3, |bytes: &[u8]| {
    Ok(RadioInterface {
        interface_type: RadioInterfaceType::from_repr(u8_from_ascii(&bytes[0..2])?).ok_or_else(|| {
            enum_parse(format!("{bytes:02X?}"), "RadioInterfaceType")
        })?,
        option: u8_from_ascii(&bytes[2..3])?,
    })
} => RadioInterface);

impl_cat_command!(SetRadioInterface => b"^XI" with Some |cmd: &SetRadioInterface| {
    let mut v = format_uint_ascii(cmd.interface_type, 2);
    v.push(b'0' + cmd.option);
    v
}, if |cmd: &SetRadioInterface| {
    if cmd.interface_type > 3 {
        Err(RigError::InvalidArgumentValue {
            argument_name: "interface_type",
            type_name: "u8",
            value: cmd.interface_type.to_string(),
        })
    } else if cmd.option > 1 {
        Err(RigError::InvalidArgumentValue {
            argument_name: "option",
            type_name: "u8",
            value: cmd.option.to_string(),
        })
    } else {
        Ok(())
    }
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransceiverPowerLevel => b"^XP");
impl_cat_command_with_response!(GetTransceiverPowerLevel => 3, u8_from_ascii => u8);
