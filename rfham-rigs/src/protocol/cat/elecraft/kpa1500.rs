//!
//! CAT commands for the Elecraft KPA1500 1500-watt solid-state HF/6m amplifier.
//!
//! All KPA1500 commands and responses use a leading caret (`^`), for example `^BN05;`. The GET
//! form of a command is the bare command letters with no data, e.g. `^BN;`.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft KPA1500 Programmer's Reference, rev 3.03](https://ftp.elecraft.com/KPA1500/Manuals%20Downloads/KPA1500ProgrammingReferenceV3.pdf), Jun 2026.
//!

use crate::{
    error::RigError,
    protocol::cat::{
        Command,
        common::{
            format_uint_ascii, u8_from_ascii, u16_from_ascii, u32_from_ascii, validate_response,
        },
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoAntennaSelection, SetAutoAntennaSelection
// ------------------------------------------------------------------------------------------------

define_command!("Get whether automatic antenna selection is enabled.

When enabled, the amplifier automatically selects the antenna port assigned to the current band
via [`SetAntennaBandMap`].

# Command format

> `^AA;`

# Response format

> `^AA{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAutoAntennaSelection
);

define_command!("Set whether automatic antenna selection is enabled.

# Command format

> `^AA{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoAntennaSelection {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntennaBandMap, SetAntennaBandMap
// ------------------------------------------------------------------------------------------------

define_command!("Get the antenna port assigned to the current band.

# Command format

> `^AB;`

# Response format

> `^AB{n};`

Where *n* is the antenna port, `1` or `2`." =>
    GetAntennaBandMap
);

define_command!("Set the antenna port assigned to the current band.

# Command format

> `^AB{n};`

Where *n* is the antenna port, `1` or `2`." =>
    SetAntennaBandMap {
        antenna: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAdcReadings, AdcReadings
// ------------------------------------------------------------------------------------------------

define_command!("Get the raw ADC readings for drain voltage, drain current, and supply voltage.

# Command format

> `^AD;`

# Response format

> `^AD{vvv} {iii} {sss};`

Where:

* *vvv* is the PA drain voltage, in tenths of a volt.
* *iii* is the PA drain current, in tenths of an ampere.
* *sss* is the supply voltage, in tenths of a volt." =>
    GetAdcReadings
);

define_command_struct!(
    "ADC readings returned by [`GetAdcReadings`]." =>
    AdcReadings {
        "PA drain voltage, in tenths of a volt." =>
        drain_voltage_dv: u16,
        "PA drain current, in tenths of an ampere." =>
        drain_current_da: u16,
        "Supply voltage, in tenths of a volt." =>
        supply_voltage_dv: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAlcEnable, SetAlcEnable
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the ALC (Automatic Level Control) output is enabled.

# Command format

> `^AE;`

# Response format

> `^AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAlcEnable
);

define_command!("Set whether the ALC (Automatic Level Control) output is enabled.

# Command format

> `^AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAlcEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoInfoMode, SetAutoInfoMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the auto-information (AI) mode.

# Command format

> `^AI;`

# Response format

> `^AI{n};`

Where `n` is the boolean state `0` (off) or `1` (on) — when on, the amplifier reports status
changes automatically, without being polled." =>
    GetAutoInfoMode
);

define_command!("Set the auto-information (AI) mode.

# Command format

> `^AI{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoInfoMode {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAlcThreshold, SetAlcThreshold
// ------------------------------------------------------------------------------------------------

define_command!("Get the ALC (Automatic Level Control) threshold for the current band.

# Command format

> `^AL;`

# Response format

> `^AL{nnn};`

Where *nnn* is the ALC threshold, between `000` and `210`." =>
    GetAlcThreshold
);

define_command!("Set the ALC (Automatic Level Control) threshold for the current band.

# Command format

> `^AL{nnn};`

Where *nnn* is the ALC threshold, between `000` and `210`." =>
    SetAlcThreshold {
        value: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAmModeEnable, SetAmModeEnable
// ------------------------------------------------------------------------------------------------

define_command!("Get whether AM mode is enabled.

# Command format

> `^AM;`

# Response format

> `^AM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAmModeEnable
);

define_command!("Set whether AM mode is enabled.

# Command format

> `^AM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAmModeEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntennaSelection, SetAntennaSelection
// ------------------------------------------------------------------------------------------------

define_command!("Get the currently selected antenna port.

# Command format

> `^AN;`

# Response format

> `^AN{n};`

Where *n* is the antenna port, `1` or `2`." =>
    GetAntennaSelection
);

define_command!("Set the currently selected antenna port.

# Command format

> `^AN{n};`

Where *n* is the antenna port, `1` or `2`." =>
    SetAntennaSelection {
        antenna: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuPreset, RecallAtuPreset
// ------------------------------------------------------------------------------------------------

define_command!("Get whether an ATU preset is currently loaded for the operating frequency.

# Command format

> `^AP;`

# Response format

> `^AP{n};`

Where `n` is the boolean state `0` (no preset loaded) or `1` (preset loaded).

Use [`RecallAtuPreset`] to trigger recall of the stored preset." =>
    GetAtuPreset
);

define_command!("Recall (load) the ATU preset stored for the frequency the amplifier is currently
using.

This is a write-only trigger: it re-applies the L/C network values recorded for the current
frequency without running a full ATU tuning cycle. It is sent as the same bare command string used
by [`GetAtuPreset`]'s query form, and produces no response of its own beyond the amplifier acting
on the recalled preset.

# Command format

> `^AP;`" =>
    RecallAtuPreset
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAttenuatorReleaseTime, SetAttenuatorReleaseTime
// ------------------------------------------------------------------------------------------------

define_command!("Get the attenuator fault release time.

# Command format

> `^AR;`

# Response format

> `^AR{nnnn};`

Where *nnnn* is the release time, in milliseconds, between `1400` and `5000`." =>
    GetAttenuatorReleaseTime
);

define_command!("Set the attenuator fault release time.

# Command format

> `^AR{nnnn};`

Where *nnnn* is the release time, in milliseconds, between `1400` and `5000`." =>
    SetAttenuatorReleaseTime {
        ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuStatus, AtuStatus
// ------------------------------------------------------------------------------------------------

define_command!("Get the ATU status flags.

# Command format

> `^AS;`

# Response format

> `^AS{t}{i}{p};`

Where:

* `t` is the boolean state `0` (idle) or `1` (tuning) — whether a tune cycle is in progress.
* `i` is the boolean state `0` (out of line) or `1` (in line) — whether the ATU is currently
  switched into the RF path.
* `p` is the boolean state `0` (no preset) or `1` (preset loaded) — whether a preset is loaded for
  the current frequency." =>
    GetAtuStatus
);

define_command_struct!(
    "ATU status flags returned by [`GetAtuStatus`]." =>
    AtuStatus {
        "`true` while a tune cycle is in progress." =>
        tuning: bool,
        "`true` when the ATU is switched into the RF path." =>
        in_line: bool,
        "`true` when a preset is loaded for the current frequency." =>
        preset_loaded: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetStandbyOnBandChange, SetStandbyOnBandChange
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the amplifier returns to standby on band change.

# Command format

> `^BC;`

# Response format

> `^BC{n};`

Where `n` is the boolean state `0` (return to the prior operate/standby state) or `1` (stay in
standby)." =>
    GetStandbyOnBandChange
);

define_command!("Set whether the amplifier returns to standby on band change.

# Command format

> `^BC{n};`

Where `n` is the boolean state `0` (return to the prior operate/standby state) or `1` (stay in
standby)." =>
    SetStandbyOnBandChange {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandSelection, SetBandSelection
// ------------------------------------------------------------------------------------------------

define_command!("Get the currently selected band.

# Command format

> `^BN;`

# Response format

> `^BN{nn};`

Where *nn* is one of:

* `00`; 160m.
* `01`; 80m.
* `02`; 60m.
* `03`; 40m.
* `04`; 30m.
* `05`; 20m.
* `06`; 17m.
* `07`; 15m.
* `08`; 12m.
* `09`; 10m.
* `10`; 6m." =>
    GetBandSelection
);

define_command!("Set the active band.

# Command format

> `^BN{nn};`

Where *nn* is one of:

* `00`; 160m.
* `01`; 80m.
* `02`; 60m.
* `03`; 40m.
* `04`; 30m.
* `05`; 20m.
* `06`; 17m.
* `07`; 15m.
* `08`; 12m.
* `09`; 10m.
* `10`; 6m." =>
    SetBandSelection {
        band: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBypassRelay, SetBypassRelay
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the RF bypass relay is engaged.

# Command format

> `^BP;`

# Response format

> `^BP{n};`

Where `n` is the boolean state `0` (amplifier in the RF path) or `1` (bypassed)." =>
    GetBypassRelay
);

define_command!("Set the RF bypass relay.

# Command format

> `^BP{n};`

Where `n` is the boolean state `0` (amplifier in the RF path) or `1` (bypassed)." =>
    SetBypassRelay {
        bypassed: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPcBaudRate, SetPcBaudRate
// ------------------------------------------------------------------------------------------------

define_command!("Get the PC RS-232 port data rate.

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

define_command!("Set the PC RS-232 port data rate.

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

define_command!("Get the transceiver RS-232 port data rate.

# Command format

> `^BRX;`

# Response format

> `^BRX{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud.

Uses the same rate codes as [`GetPcBaudRate`]." =>
    GetXcvrBaudRate
);

define_command!("Set the transceiver RS-232 port data rate.

# Command format

> `^BRX{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud.

Uses the same rate codes as [`SetPcBaudRate`]." =>
    SetXcvrBaudRate {
        rate: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDemoMode, SetDemoMode
// ------------------------------------------------------------------------------------------------

define_command!("Get whether demo mode is enabled.

# Command format

> `^DM;`

# Response format

> `^DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetDemoMode
);

define_command!("Set whether demo mode is enabled.

# Command format

> `^DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetDemoMode {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDisplaySelect, SetDisplaySelect
// ------------------------------------------------------------------------------------------------

define_command!("Get the active display screen.

# Command format

> `^DS;`

# Response format

> `^DS{n};`

Where *n* is one of:

* `0`; main.
* `1`; ATU.
* `2`; meters.
* `3`; fault log." =>
    GetDisplaySelect
);

define_command!("Set the active display screen.

# Command format

> `^DS{n};`

Where *n* is one of:

* `0`; main.
* `1`; ATU.
* `2`; meters.
* `3`; fault log." =>
    SetDisplaySelect {
        screen: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFanMinimumSpeed, SetFanMinimumSpeed
// ------------------------------------------------------------------------------------------------

define_command!("Get the fan minimum control level.

# Command format

> `^FC;`

# Response format

> `^FC{n};`

Where *n* is the fan minimum level, between `0` (off) and `6` (high)." =>
    GetFanMinimumSpeed
);

define_command!("Set the fan minimum control level.

# Command format

> `^FC{n};`

Where *n* is the fan minimum level, between `0` (off) and `6` (high)." =>
    SetFanMinimumSpeed {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultCode, ClearFault
// ------------------------------------------------------------------------------------------------

define_command!("Get the current fault code.

# Command format

> `^FL;`

# Response format

> `^FL{nn};`

Where *nn* is the fault code; `00` indicates no fault.

Use [`ClearFault`] to clear an active fault." =>
    GetFaultCode
);

define_command!("Clear the amplifier's current fault condition, allowing it to resume normal
operation.

This is a write-only trigger, sent as the command identifier followed by the literal character
`C` rather than a numeric argument; it produces no response of its own. Use [`GetFaultCode`]
afterwards to confirm the fault has cleared (`00`).

# Command format

> `^FLC;`" =>
    ClearFault
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFrequency, SetFrequency
// ------------------------------------------------------------------------------------------------

define_command!("Get the frequency the amplifier is currently using for band determination.

# Command format

> `^FQ;`

# Response format

> `^FQ{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    GetFrequency
);

define_command!("Set the operating frequency used for band determination.

# Command format

> `^FQ{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    SetFrequency {
        hz: u32
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInhibitInput, SetInhibitInput
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the INHIBIT# input pin is enabled.

# Command format

> `^NH;`

# Response format

> `^NH{n};`

Where `n` is the boolean state `0` (disabled) or `1` (enabled)." =>
    GetInhibitInput
);

define_command!("Set whether the INHIBIT# input pin is enabled.

# Command format

> `^NH{n};`

Where `n` is the boolean state `0` (disabled) or `1` (enabled)." =>
    SetInhibitInput {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatus, TurnPowerOff
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the KPA1500 is powered on.

# Command format

> `^ON;`

# Response format

> `^ON{n};`

Where `n` is the boolean state `0` (off) or `1` (on). No response is sent if the amplifier is
already off.

Use [`TurnPowerOff`] to power the amplifier down." =>
    GetPowerStatus
);

define_command!("Turn the KPA1500 off.

This is a write-only trigger, sent as the command identifier followed by the literal argument
`0`; it powers the amplifier down and produces no response. There is no corresponding remote
command to power the amplifier back on — front-panel control is required.

# Command format

> `^ON0;`" =>
    TurnPowerOff
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOutputPower
// ------------------------------------------------------------------------------------------------

define_command!("Get the current output power.

# Command format

> `^OP;`

# Response format

> `^OP{nnnn};`

Where *nnnn* is the output power, in watts, between `0000` and `1500`." =>
    GetOutputPower
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOperateMode, SetOperateMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the current operate/standby mode.

# Command format

> `^OS;`

# Response format

> `^OS{n};`

Where `n` is the boolean state `0` (standby) or `1` (operate)." =>
    GetOperateMode
);

define_command!("Set operate or standby mode.

# Command format

> `^OS{n};`

Where `n` is the boolean state `0` (standby) or `1` (operate)." =>
    SetOperateMode {
        operate: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPeakPowerControl, SetPeakPowerControl
// ------------------------------------------------------------------------------------------------

define_command!("Get the peak-power control limit.

# Command format

> `^PC;`

# Response format

> `^PC{nnnn};`

Where *nnnn* is the output power limit, in watts, between `0000` and `1500`." =>
    GetPeakPowerControl
);

define_command!("Set the peak-power control limit.

# Command format

> `^PC{nnnn};`

Where *nnnn* is the output power limit, in watts, between `0000` and `1500`." =>
    SetPeakPowerControl {
        watts: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPttDelay, SetPttDelay
// ------------------------------------------------------------------------------------------------

define_command!("Get the PTT delay.

# Command format

> `^PD;`

# Response format

> `^PD{nnn};`

Where *nnn* is the PTT delay, in milliseconds, between `000` and `500`." =>
    GetPttDelay
);

define_command!("Set the PTT delay.

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

define_command!("Get whether protection faults are enabled.

# Command format

> `^PF;`

# Response format

> `^PF{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetProtectionFaultEnable
);

define_command!("Set whether protection faults are enabled.

# Command format

> `^PF{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetProtectionFaultEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerAdjustment, SetPowerAdjustment
// ------------------------------------------------------------------------------------------------

define_command!("Get the power adjustment for the current band.

# Command format

> `^PJ;`

# Response format

> `^PJ{nnn};`

Where *nnn* is the power adjustment, as a percentage of rated output, between `080` and `120`." =>
    GetPowerAdjustment
);

define_command!("Set the power adjustment for the current band.

# Command format

> `^PJ{nnn};`

Where *nnn* is the power adjustment, as a percentage of rated output, between `080` and `120`." =>
    SetPowerAdjustment {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatusSummary, PowerStatusSummary
// ------------------------------------------------------------------------------------------------

define_command!("Get the complete power status summary.

# Command format

> `^PWR;`

# Response format

> `^PWR{pppp} {ssss} {rrrr} {iiii};`

Where:

* *pppp* is the output power, in watts.
* *ssss* is SWR × 10, e.g. `0015` represents an SWR of 1.5:1.
* *rrrr* is the reflected power, in watts.
* *iiii* is the input power, in watts." =>
    GetPowerStatusSummary
);

define_command_struct!(
    "Power status summary returned by [`GetPowerStatusSummary`]." =>
    PowerStatusSummary {
        "Output power, in watts." =>
        power_w: u16,
        "SWR × 10, e.g. `15` represents an SWR of 1.5:1." =>
        swr_d: u16,
        "Reflected power, in watts." =>
        reflected_w: u16,
        "Input power, in watts." =>
        input_w: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

define_command!("Get the firmware version string.

# Command format

> `^RVM;`

# Response format

> `^RVM{text};`

Where *text* is the firmware version, e.g. `03.03`.

The returned `String` is the complete raw ASCII response line, including the `^RVM` prefix and
the terminating `;`." =>
    GetFirmwareVersion
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSerialNumber
// ------------------------------------------------------------------------------------------------

define_command!("Get the KPA1500 serial number.

# Command format

> `^SN;`

# Response format

> `^SN{nnnnn};`

Where *nnnnn* is the 5-digit serial number." =>
    GetSerialNumber
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultSpeaker, SetFaultSpeaker
// ------------------------------------------------------------------------------------------------

define_command!("Get the fault speaker on/off state.

# Command format

> `^SP;`

# Response format

> `^SP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetFaultSpeaker
);

define_command!("Set the fault speaker on/off state.

# Command format

> `^SP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetFaultSpeaker { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPaTemperature
// ------------------------------------------------------------------------------------------------

define_command!("Get the PA temperature.

# Command format

> `^TM;`

# Response format

> `^TM{nnn};`

Where *nnn* is the PA temperature, in degrees Celsius, between `000` and `200`." =>
    GetPaTemperature
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTunePower, SetTunePower
// ------------------------------------------------------------------------------------------------

define_command!("Get the tune power level.

# Command format

> `^TP;`

# Response format

> `^TP{nnnn};`

Where *nnnn* is the tune power, in watts." =>
    GetTunePower
);

define_command!("Set the tune power level.

# Command format

> `^TP{nnnn};`

Where *nnnn* is the tune power, in watts." =>
    SetTunePower {
        watts: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTrDelay, SetTrDelay
// ------------------------------------------------------------------------------------------------

define_command!("Get the T/R (transmit-to-receive) delay time.

# Command format

> `^TR;`

# Response format

> `^TR{nn};`

Where *nn* is the T/R delay, in milliseconds, between `00` and `50`." =>
    GetTrDelay
);

define_command!("Set the T/R (transmit-to-receive) delay time.

# Command format

> `^TR{nn};`

Where *nn* is the T/R delay, in milliseconds, between `00` and `50`." =>
    SetTrDelay {
        ms: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverVoltage
// ------------------------------------------------------------------------------------------------

define_command!("Get the transceiver supply voltage.

# Command format

> `^TV;`

# Response format

> `^TV{nnn};`

Where *nnn* is the transceiver supply voltage, in tenths of a volt." =>
    GetTransceiverVoltage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPaVoltageCurrent, PaVoltageCurrent
// ------------------------------------------------------------------------------------------------

define_command!("Get the PA supply voltage and drain current.

# Command format

> `^VI;`

# Response format

> `^VI{vvvv} {iiii};`

Where:

* *vvvv* is the PA supply voltage, in tenths of a volt.
* *iiii* is the PA drain current, in tenths of an ampere." =>
    GetPaVoltageCurrent
);

define_command_struct!(
    "PA voltage and current reading returned by [`GetPaVoltageCurrent`]." =>
    PaVoltageCurrent {
        "PA supply voltage, in tenths of a volt." =>
        voltage_dv: u16,
        "PA drain current, in tenths of an ampere." =>
        current_da: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerAndSwr, PowerAndSwr
// ------------------------------------------------------------------------------------------------

define_command!("Get the output power and SWR.

# Command format

> `^WS;`

# Response format

> `^WS{ppppp} {sssss};`

Where:

* *ppppp* is the output power, in watts.
* *sssss* is SWR × 10, e.g. `00015` represents an SWR of 1.5:1; zero when not transmitting." =>
    GetPowerAndSwr
);

define_command_struct!(
    "Power and SWR reading returned by [`GetPowerAndSwr`]." =>
    PowerAndSwr {
        "Output power, in watts." =>
        power_w: u16,
        "SWR × 10. Zero when not transmitting." =>
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRadioInterface, SetRadioInterface, RadioInterface
// ------------------------------------------------------------------------------------------------

define_command!("Get the radio interface type and option setting.

# Command format

> `^XI;`

# Response format

> `^XI{nn}{o};`

Where:

* *nn* is the interface type, one of `00` (K3), `01` (BCD), `02` (Analog), or `03` (Serial).
* *o* is an interface-specific option flag, `0` or `1`; its meaning depends on the selected
  interface type." =>
    GetRadioInterface
);

define_command!("Set the radio interface type and option setting.

# Command format

> `^XI{nn}{o};`

Where:

* *nn* is the interface type, one of `00` (K3), `01` (BCD), `02` (Analog), or `03` (Serial).
* *o* is an interface-specific option flag, `0` or `1`; its meaning depends on the selected
  interface type." =>
    SetRadioInterface {
        interface_type: u8,
        option: u8
    }
);

define_command_struct!(
    "The radio interface type and option, returned by [`GetRadioInterface`]." =>
    RadioInterface {
        "The interface type: `0` (K3), `1` (BCD), `2` (Analog), or `3` (Serial)." =>
        interface_type: u8,
        "An interface-specific option flag, `0` or `1`." =>
        option: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_command!(GetAutoAntennaSelection => b"^AA");
impl_command_with_response!(GetAutoAntennaSelection => boolean);

impl_command!(SetAutoAntennaSelection => b"^AA" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAntennaBandMap => b"^AB");
impl_command_with_response!(GetAntennaBandMap => 1, u8_from_ascii => u8);

impl_command!(
    SetAntennaBandMap => b"^AB"
    format antenna uint 1,
    if |cmd: &SetAntennaBandMap| {
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

impl_command!(GetAdcReadings => b"^AD");
impl_command_with_response!(GetAdcReadings => 11, |bytes: &[u8]| {
    Ok(AdcReadings {
        drain_voltage_dv: u16_from_ascii(&bytes[0..3])?,
        drain_current_da: u16_from_ascii(&bytes[4..7])?,
        supply_voltage_dv: u16_from_ascii(&bytes[8..11])?,
    })
} => AdcReadings);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAlcEnable => b"^AE");
impl_command_with_response!(GetAlcEnable => boolean);

impl_command!(SetAlcEnable => b"^AE" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAutoInfoMode => b"^AI");
impl_command_with_response!(GetAutoInfoMode => boolean);

impl_command!(SetAutoInfoMode => b"^AI" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAlcThreshold => b"^AL");
impl_command_with_response!(GetAlcThreshold => 3, u16_from_ascii => u16);

impl_command!(
    SetAlcThreshold => b"^AL"
    format value uint 3,
    if |cmd: &SetAlcThreshold| {
        if cmd.value <= 210 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "value",
                type_name: "u16",
                value: cmd.value.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAmModeEnable => b"^AM");
impl_command_with_response!(GetAmModeEnable => boolean);

impl_command!(SetAmModeEnable => b"^AM" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAntennaSelection => b"^AN");
impl_command_with_response!(GetAntennaSelection => 1, u8_from_ascii => u8);

impl_command!(
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

impl_command!(GetAtuPreset => b"^AP");
impl_command_with_response!(GetAtuPreset => boolean);

impl_command!(RecallAtuPreset => b"^AP");

// ------------------------------------------------------------------------------------------------

impl_command!(GetAttenuatorReleaseTime => b"^AR");
impl_command_with_response!(GetAttenuatorReleaseTime => 4, u16_from_ascii => u16);

impl_command!(SetAttenuatorReleaseTime => b"^AR" with Some |cmd: &SetAttenuatorReleaseTime| {
    format!("{:04}", cmd.ms).into_bytes()
}, if |cmd: &SetAttenuatorReleaseTime| {
    if (1400..=5000).contains(&cmd.ms) {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "ms",
            type_name: "u16",
            value: cmd.ms.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetAtuStatus => b"^AS");
impl_command_with_response!(GetAtuStatus => 3, |bytes: &[u8]| {
    Ok(AtuStatus {
        tuning: bytes[0] == b'1',
        in_line: bytes[1] == b'1',
        preset_loaded: bytes[2] == b'1',
    })
} => AtuStatus);

// ------------------------------------------------------------------------------------------------

impl_command!(GetStandbyOnBandChange => b"^BC");
impl_command_with_response!(GetStandbyOnBandChange => boolean);

impl_command!(SetStandbyOnBandChange => b"^BC" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetBandSelection => b"^BN");
impl_command_with_response!(GetBandSelection => 2, u8_from_ascii => u8);

impl_command!(
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

impl_command!(GetBypassRelay => b"^BP");
impl_command_with_response!(GetBypassRelay => boolean);

impl_command!(SetBypassRelay => b"^BP" for boolean bypassed);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPcBaudRate => b"^BRP");
impl_command_with_response!(GetPcBaudRate => 1, u8_from_ascii => u8);

impl_command!(
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

impl_command!(GetXcvrBaudRate => b"^BRX");
impl_command_with_response!(GetXcvrBaudRate => 1, u8_from_ascii => u8);

impl_command!(
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

impl_command!(GetDemoMode => b"^DM");
impl_command_with_response!(GetDemoMode => boolean);

impl_command!(SetDemoMode => b"^DM" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetDisplaySelect => b"^DS");
impl_command_with_response!(GetDisplaySelect => 1, u8_from_ascii => u8);

impl_command!(
    SetDisplaySelect => b"^DS"
    format screen uint 1,
    if |cmd: &SetDisplaySelect| {
        if cmd.screen <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "screen",
                type_name: "u8",
                value: cmd.screen.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFanMinimumSpeed => b"^FC");
impl_command_with_response!(GetFanMinimumSpeed => 1, u8_from_ascii => u8);

impl_command!(
    SetFanMinimumSpeed => b"^FC"
    format level uint 1,
    if |cmd: &SetFanMinimumSpeed| {
        if cmd.level <= 6 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "level",
                type_name: "u8",
                value: cmd.level.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFaultCode => b"^FL");
impl_command_with_response!(GetFaultCode => 2, u8_from_ascii => u8);

impl_command!(ClearFault => b"^FL" with Some |_: &ClearFault| {
    vec![b'C']
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetFrequency => b"^FQ");
impl_command_with_response!(GetFrequency => 8, u32_from_ascii => u32);

impl_command!(SetFrequency => b"^FQ" with Some |cmd: &SetFrequency| {
    format!("{:08}", cmd.hz).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetInhibitInput => b"^NH");
impl_command_with_response!(GetInhibitInput => boolean);

impl_command!(SetInhibitInput => b"^NH" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerStatus => b"^ON");
impl_command_with_response!(GetPowerStatus => boolean);

impl_command!(TurnPowerOff => b"^ON" with Some |_: &TurnPowerOff| {
    vec![b'0']
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetOutputPower => b"^OP");
impl_command_with_response!(GetOutputPower => 4, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetOperateMode => b"^OS");
impl_command_with_response!(GetOperateMode => boolean);

impl_command!(SetOperateMode => b"^OS" for boolean operate);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPeakPowerControl => b"^PC");
impl_command_with_response!(GetPeakPowerControl => 4, u16_from_ascii => u16);

impl_command!(SetPeakPowerControl => b"^PC" with Some |cmd: &SetPeakPowerControl| {
    format!("{:04}", cmd.watts).into_bytes()
}, if |cmd: &SetPeakPowerControl| {
    if cmd.watts <= 1500 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "watts",
            type_name: "u16",
            value: cmd.watts.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetPttDelay => b"^PD");
impl_command_with_response!(GetPttDelay => 3, u16_from_ascii => u16);

impl_command!(
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

impl_command!(GetProtectionFaultEnable => b"^PF");
impl_command_with_response!(GetProtectionFaultEnable => boolean);

impl_command!(SetProtectionFaultEnable => b"^PF" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerAdjustment => b"^PJ");
impl_command_with_response!(GetPowerAdjustment => 3, u8_from_ascii => u8);

impl_command!(
    SetPowerAdjustment => b"^PJ"
    format value uint 3,
    if |cmd: &SetPowerAdjustment| {
        if (80..=120).contains(&cmd.value) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "value",
                type_name: "u8",
                value: cmd.value.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerStatusSummary => b"^PWR");
impl_command_with_response!(GetPowerStatusSummary => 19, |bytes: &[u8]| {
    Ok(PowerStatusSummary {
        power_w: u16_from_ascii(&bytes[0..4])?,
        swr_d: u16_from_ascii(&bytes[5..9])?,
        reflected_w: u16_from_ascii(&bytes[10..14])?,
        input_w: u16_from_ascii(&bytes[15..19])?,
    })
} => PowerStatusSummary);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFirmwareVersion => b"^RVM");
impl_command_with_response!(GetFirmwareVersion => string);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSerialNumber => b"^SN");
impl_command_with_response!(GetSerialNumber => 5, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFaultSpeaker => b"^SP");
impl_command_with_response!(GetFaultSpeaker => boolean);

impl_command!(SetFaultSpeaker => b"^SP" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPaTemperature => b"^TM");
impl_command_with_response!(GetPaTemperature => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTunePower => b"^TP");
impl_command_with_response!(GetTunePower => 4, u16_from_ascii => u16);

impl_command!(SetTunePower => b"^TP" with Some |cmd: &SetTunePower| {
    format!("{:04}", cmd.watts).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetTrDelay => b"^TR");
impl_command_with_response!(GetTrDelay => 2, u8_from_ascii => u8);

impl_command!(
    SetTrDelay => b"^TR"
    format ms uint 2,
    if |cmd: &SetTrDelay| {
        if cmd.ms <= 50 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "ms",
                type_name: "u8",
                value: cmd.ms.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverVoltage => b"^TV");
impl_command_with_response!(GetTransceiverVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPaVoltageCurrent => b"^VI");
impl_command_with_response!(GetPaVoltageCurrent => 9, |bytes: &[u8]| {
    Ok(PaVoltageCurrent {
        voltage_dv: u16_from_ascii(&bytes[0..4])?,
        current_da: u16_from_ascii(&bytes[5..9])?,
    })
} => PaVoltageCurrent);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerAndSwr => b"^WS");
impl_command_with_response!(GetPowerAndSwr => 11, |bytes: &[u8]| {
    Ok(PowerAndSwr {
        power_w: u16_from_ascii(&bytes[0..5])?,
        swr_d: u16_from_ascii(&bytes[6..11])?,
    })
} => PowerAndSwr);

// ------------------------------------------------------------------------------------------------

impl_command!(GetRadioInterface => b"^XI");
impl_command_with_response!(GetRadioInterface => 3, |bytes: &[u8]| {
    Ok(RadioInterface {
        interface_type: u8_from_ascii(&bytes[0..2])?,
        option: bytes[2] - b'0',
    })
} => RadioInterface);

impl_command!(SetRadioInterface => b"^XI" with Some |cmd: &SetRadioInterface| {
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
