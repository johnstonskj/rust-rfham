//!
//! CAT commands for the Elecraft KAT500 automatic antenna tuner.
//!
//! Unlike the transceiver, amplifier, and panadapter command sets in sibling modules, KAT500
//! command identifiers carry **no wire prefix** — they are sent exactly as shown (e.g. `AN`, `BN`,
//! `VSWR`), with the single exception of [`GetBaudRate`]/[`SetBaudRate`] which use the `#BR`
//! identifier (leading `#`).
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft KAT500 Automatic Antenna Tuner Command Reference](https://ftp.elecraft.com/KAT500/Manuals%20Downloads/KAT500%20Automatic%20Antenna%20Tuner%20Serial%20Command%20Reference.pdf), Sep 2023.
//!

use crate::{
    error::{RigError, enum_parse, invalid_argument_value},
    protocol::{
        Frequency,
        cat::{
            Command,
            common::{
                bytes_to_vec, string_from_ascii, u8_from_ascii, u16_from_ascii, u32_from_ascii,
                validate_response,
            },
        },
    },
    transport::BaudRate,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoBypassState, SetAutoBypassState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether automatic bypass is enabled.

When automatic bypass is on, the tuner monitors SWR and switches itself out of circuit whenever
the untuned SWR is already acceptable.

# Command format

> `AB;`

# Response format

> `AB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAutoBypassState
);

define_command!("Set whether automatic bypass is enabled.

# Command format

> `AB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoBypassState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoEnableState, SetAutoEnableState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the ATU is enabled, i.e. whether automatic tuning is allowed.

# Command format

> `AE;`

# Response format

> `AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAutoEnableState
);

define_command!("Set whether the ATU is enabled, i.e. whether automatic tuning is allowed.

# Command format

> `AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoEnableState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuFaultState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the ATU currently has a fault condition.

# Command format

> `AFT;`

# Response format

> `AFT{n};`

Where `n` is the boolean state `0` (no fault) or `1` (fault present)." =>
    GetAtuFaultState
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuKeepInPlaceState, SetAtuKeepInPlaceState
// ------------------------------------------------------------------------------------------------

define_command!("Get the ATU keep-in-place state.

When keep-in-place is on, the tuner retains its last L/C setting rather than re-tuning on band or
frequency changes.

# Command format

> `AKIP;`

# Response format

> `AKIP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAtuKeepInPlaceState
);

define_command!("Set the ATU keep-in-place state.

# Command format

> `AKIP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAtuKeepInPlaceState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAmplifierInterface, SetAmplifierInterface
// ------------------------------------------------------------------------------------------------

define_command!("Get the amplifier interface relay state.

# Command format

> `AMPI;`

# Response format

> `AMPI{n};`

Where `n` is the boolean state `0` (open) or `1` (closed)." =>
    GetAmplifierInterface
);

define_command!("Set the amplifier interface relay state.

# Command format

> `AMPI{n};`

Where `n` is the boolean state `0` (open) or `1` (closed)." =>
    SetAmplifierInterface {
        closed: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntenna, SetAntenna
// ------------------------------------------------------------------------------------------------

define_command!("Get the currently selected antenna port.

# Command format

> `AN;`

# Response format

> `AN{n};`

Where *n* is the antenna port number, between `1` and `6`." =>
    GetAntenna
);

define_command!("Set the currently selected antenna port.

# Command format

> `AN{n};`

Where *n* is the antenna port number, between `1` and `6`." =>
    SetAntenna {
        antenna: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuPreset, SetAtuPreset
// ------------------------------------------------------------------------------------------------

define_command!("Get the current ATU preset slot number.

# Command format

> `AP;`

# Response format

> `AP{nnn};`

Where *nnn* is the 3-digit preset slot number." =>
    GetAtuPreset
);

define_command!("Set the current ATU preset slot number.

# Command format

> `AP{nnn};`

Where *nnn* is the 3-digit preset slot number." =>
    SetAtuPreset {
        preset: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAttenuatorState, SetAttenuatorState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the built-in attenuator is enabled.

# Command format

> `ATTN;`

# Response format

> `ATTN{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAttenuatorState
);

define_command!("Set whether the built-in attenuator is enabled.

# Command format

> `ATTN{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAttenuatorState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBand, SetBand
// ------------------------------------------------------------------------------------------------

define_command!("Get the current band number.

# Command format

> `BN;`

# Response format

> `BN{nn};`

Where *nn* is the 2-digit band number, between `00` and `13`." =>
    GetBand
);

define_command!("Set the current band number.

# Command format

> `BN{nn};`

Where *nn* is the 2-digit band number, between `00` and `13`." =>
    SetBand {
        band: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBaudRate, SetBaudRate
// ------------------------------------------------------------------------------------------------

define_command!("Get the serial port baud rate.

Unlike every other command in this module, the baud-rate command identifier carries a leading
`#`, i.e. `#BR` rather than `BR`.

# Command format

> `#BR;`

# Response format

> `#BR{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    GetBaudRate
);

define_command!("Set the serial port baud rate.

Unlike every other command in this module, the baud-rate command identifier carries a leading
`#`, i.e. `#BR` rather than `BR`. Only 4800, 9600, 19200 and 38400 baud are supported by this
command; any other `BaudRate` value is rejected when the command is sent.

# Command format

> `#BR{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    SetBaudRate {
        baud_rate: BaudRate
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: Bypass
// ------------------------------------------------------------------------------------------------

define_command!("Force the ATU into bypass mode immediately.

This command takes effect immediately and has no query form.

# Command format

> `BYP;`" =>
    Bypass
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCapacitorValue, SetCapacitorValue
// ------------------------------------------------------------------------------------------------

define_command!("Get the tuning capacitor value.

# Command format

> `C;`

# Response format

> `C{nnn};`

Where *nnn* is the 3-digit capacitor value, between `000` and `255`." =>
    GetCapacitorValue
);

define_command!("Set the tuning capacitor value.

# Command format

> `C{nnn};`

Where *nnn* is the 3-digit capacitor value, between `000` and `255`." =>
    SetCapacitorValue {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCapacitorTopology, SetCapacitorTopology
// ------------------------------------------------------------------------------------------------

define_command!("Get the tuning capacitor topology (hi-Z or lo-Z).

# Command format

> `CT;`

# Response format

> `CT{n};`

Where `n` is `0` (lo-Z, capacitor on the output side) or `1` (hi-Z, capacitor on the input side)." =>
    GetCapacitorTopology
);

define_command!("Set the tuning capacitor topology (hi-Z or lo-Z).

# Command format

> `CT{n};`

Where `n` is `0` (lo-Z, capacitor on the output side) or `1` (hi-Z, capacitor on the input side)." =>
    SetCapacitorTopology {
        hi_z: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDemoModeState, SetDemoModeState
// ------------------------------------------------------------------------------------------------

define_command!("Get the demo-mode state.

# Command format

> `DM;`

# Response format

> `DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetDemoModeState
);

define_command!("Set the demo-mode state.

# Command format

> `DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetDemoModeState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: EepromInit
// ------------------------------------------------------------------------------------------------

define_command!("Re-initialize EEPROM storage to factory defaults.

This is destructive: it erases all stored antenna and band presets. There is no query form.

# Command format

> `EEINIT;`" =>
    EepromInit
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetErrorMessage
// ------------------------------------------------------------------------------------------------

define_command!("Get the last error message string.

# Command format

> `EM;`

# Response format

> `EM{text};`

The response is a variable-length ASCII text string, returned as raw bytes." =>
    GetErrorMessage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFrequency, SetFrequency
// ------------------------------------------------------------------------------------------------

define_command!("Get the operating frequency, in Hz.

# Command format

> `F;`

# Response format

> `F{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    GetFrequency
);

define_command!("Set the operating frequency, in Hz.

# Command format

> `F{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    SetFrequency {
        freq_hz: Frequency
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetForwardPowerA, GetForwardPowerB
// ------------------------------------------------------------------------------------------------

define_command!("Get the forward power reading on meter channel A.

# Command format

> `FA;`

# Response format

> `FA{nnn};`

Where *nnn* is the forward power, in deci-watts (tenths of a watt)." =>
    GetForwardPowerA
);

define_command!("Get the forward power reading on meter channel B.

# Command format

> `FB;`

# Response format

> `FB{nnn};`

Where *nnn* is the forward power, in deci-watts (tenths of a watt)." =>
    GetForwardPowerB
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFanThreshold, SetFanThreshold
// ------------------------------------------------------------------------------------------------

define_command!("Get the fan-on power threshold.

# Command format

> `FC;`

# Response format

> `FC{nnn};`

Where *nnn* is the 3-digit threshold, in watts, above which the cooling fan turns on." =>
    GetFanThreshold
);

define_command!("Set the fan-on power threshold.

# Command format

> `FC{nnn};`

Where *nnn* is the 3-digit threshold, in watts, above which the cooling fan turns on." =>
    SetFanThreshold {
        threshold_w: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultDelayTime, SetFaultDelayTime
// ------------------------------------------------------------------------------------------------

define_command!("Get the fault delay time.

# Command format

> `FDT;`

# Response format

> `FDT{nnn};`

Where *nnn* is the 3-digit fault delay time, in milliseconds." =>
    GetFaultDelayTime
);

define_command!("Set the fault delay time.

# Command format

> `FDT{nnn};`

Where *nnn* is the 3-digit fault delay time, in milliseconds." =>
    SetFaultDelayTime {
        delay_ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultStatus
// ------------------------------------------------------------------------------------------------

define_command!("Get the current fault status code.

# Command format

> `FLT;`

# Response format

> `FLT{nn};`

Where *nn* is the 2-digit fault status code; `00` indicates no fault." =>
    GetFaultStatus
);

// ------------------------------------------------------------------------------------------------
// Public Types: ClearCurrentFault
// ------------------------------------------------------------------------------------------------

define_command!("Clear the current fault condition.

There is no query form.

# Command format

> `FLTC;`" =>
    ClearCurrentFault
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTuneSatisfiedSwr, SetTuneSatisfiedSwr
// ------------------------------------------------------------------------------------------------

define_command!("Get the SWR threshold below which a tune cycle is considered successful.

# Command format

> `FTNS;`

# Response format

> `FTNS{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetTuneSatisfiedSwr
);

define_command!("Set the SWR threshold below which a tune cycle is considered successful.

# Command format

> `FTNS{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetTuneSatisfiedSwr {
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultThresholdLow, SetFaultThresholdLow
// ------------------------------------------------------------------------------------------------

define_command!("Get the lower fault SWR threshold.

# Command format

> `FT0;`

# Response format

> `FT0{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetFaultThresholdLow
);

define_command!("Set the lower fault SWR threshold.

# Command format

> `FT0{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetFaultThresholdLow {
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultThresholdHigh, SetFaultThresholdHigh
// ------------------------------------------------------------------------------------------------

define_command!("Get the upper fault SWR threshold.

# Command format

> `FT1;`

# Response format

> `FT1{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetFaultThresholdHigh
);

define_command!("Set the upper fault SWR threshold.

# Command format

> `FT1{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetFaultThresholdHigh {
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFixedLcState, SetFixedLcState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether fixed L/C mode is enabled.

When fixed L/C mode is on, the tuner holds a fixed inductor/capacitor setting rather than
re-tuning.

# Command format

> `FX;`

# Response format

> `FX{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetFixedLcState
);

define_command!("Set whether fixed L/C mode is enabled.

# Command format

> `FX{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetFixedLcState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFixedBypassState, SetFixedBypassState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether fixed bypass mode is active.

# Command format

> `FY;`

# Response format

> `FY{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetFixedBypassState
);

define_command!("Set whether fixed bypass mode is active.

# Command format

> `FY{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetFixedBypassState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInductance, SetInductance
// ------------------------------------------------------------------------------------------------

define_command!("Get the tuning inductance tap.

# Command format

> `I;`

# Response format

> `I{nnn};`

Where *nnn* is the 3-digit inductor tap value, between `000` and `063`." =>
    GetInductance
);

define_command!("Set the tuning inductance tap.

# Command format

> `I{nnn};`

Where *nnn* is the 3-digit inductor tap value, between `000` and `063`." =>
    SetInductance {
        tap: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInhibitFan, SetInhibitFan
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the cooling fan is inhibited.

# Command format

> `IF;`

# Response format

> `IF{n};`

Where `n` is `0` (fan enabled) or `1` (fan inhibited)." =>
    GetInhibitFan
);

define_command!("Set whether the cooling fan is inhibited.

# Command format

> `IF{n};`

Where `n` is `0` (fan enabled) or `1` (fan inhibited)." =>
    SetInhibitFan {
        inhibit: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInductorSwitch, SetInductorSwitch
// ------------------------------------------------------------------------------------------------

define_command!("Get the inductor switch bitmask.

# Command format

> `L;`

# Response format

> `L{nnn};`

Where *nnn* is the 3-digit inductor switch bitmask." =>
    GetInductorSwitch
);

define_command!("Set the inductor switch bitmask directly.

# Command format

> `L{nnn};`

Where *nnn* is the 3-digit inductor switch bitmask." =>
    SetInductorSwitch {
        mask: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOperatingMode, SetOperatingMode, OperatingMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the current ATU operating mode.

# Command format

> `MD;`

# Response format

> `MD{n};`

Where *n* is one of:

* `0`; automatic tuning.
* `1`; semi-automatic tuning.
* `2`; manual tuning." =>
    GetOperatingMode
);

define_command!("Set the current ATU operating mode.

# Command format

> `MD{n};`

Where *n* is one of:

* `0`; automatic tuning.
* `1`; semi-automatic tuning.
* `2`; manual tuning." =>
    SetOperatingMode {
        mode: OperatingMode
    }
);

define_command_enum!(
    "The ATU operating mode." => OperatingMode {
        "Automatic tuning; the ATU tunes without operator intervention." => Auto = b'0',
        "Semi-automatic tuning; the ATU tunes only when explicitly triggered." => SemiAuto = b'1',
        "Manual tuning; L/C values are set directly by the operator." => Manual = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMeterType, SetMeterType, MeterType
// ------------------------------------------------------------------------------------------------

define_command!("Get the front-panel meter display type.

# Command format

> `MT;`

# Response format

> `MT{n};`

Where *n* is one of:

* `0`; SWR.
* `1`; power.
* `2`; reflected power." =>
    GetMeterType
);

define_command!("Set the front-panel meter display type.

# Command format

> `MT{n};`

Where *n* is one of:

* `0`; SWR.
* `1`; power.
* `2`; reflected power." =>
    SetMeterType {
        meter: MeterType
    }
);

define_command_enum!(
    "The front-panel meter display type." => MeterType {
        "Display SWR." => Swr = b'0',
        "Display forward power." => Power = b'1',
        "Display reflected power." => Reflected = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatus
// ------------------------------------------------------------------------------------------------

define_command!("Get the power-on status.

# Command format

> `PS;`

# Response format

> `PS{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetPowerStatus
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerSensorInput
// ------------------------------------------------------------------------------------------------

define_command!("Get the forward power reading from the internal sensor.

# Command format

> `PSI;`

# Response format

> `PSI{nnn};`

Where *nnn* is the forward power, in deci-watts (tenths of a watt)." =>
    GetPowerSensorInput
);

// ------------------------------------------------------------------------------------------------
// Public Types: ResetDevice
// ------------------------------------------------------------------------------------------------

define_command!("Perform a soft reset of the KAT500, triggering a firmware restart.

There is no query form.

# Command format

> `RSTX;`" =>
    ResetDevice
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

define_command!("Get the firmware version string.

# Command format

> `RV;`

# Response format

> `RV{text};`

The response is a variable-length ASCII text string, returned as raw bytes, e.g. `RV02.12;`." =>
    GetFirmwareVersion
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntennaSide, SetAntennaSide, AntennaSide
// ------------------------------------------------------------------------------------------------

define_command!("Get the antenna side selection.

# Command format

> `SIDE;`

# Response format

> `SIDE{n};`

Where *n* is one of:

* `0`; left.
* `1`; right." =>
    GetAntennaSide
);

define_command!("Set the antenna side selection.

# Command format

> `SIDE{n};`

Where *n* is one of:

* `0`; left.
* `1`; right." =>
    SetAntennaSide {
        side: AntennaSide
    }
);

define_command_enum!(
    "The antenna side selection." => AntennaSide {
        "Left side." => Left = b'0',
        "Right side." => Right = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTuningSpeedLimit, SetTuningSpeedLimit
// ------------------------------------------------------------------------------------------------

define_command!("Get the tuning speed limit setting.

# Command format

> `SL;`

# Response format

> `SL{n};`

Where *n* is the tuning speed limit, between `0` (fastest) and `9` (slowest)." =>
    GetTuningSpeedLimit
);

define_command!("Set the tuning speed limit.

# Command format

> `SL{n};`

Where *n* is the tuning speed limit, between `0` (fastest) and `9` (slowest)." =>
    SetTuningSpeedLimit {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSwrMeter
// ------------------------------------------------------------------------------------------------

define_command!("Get the current SWR meter reading.

# Command format

> `SM;`

# Response format

> `SM{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetSwrMeter
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSerialNumber
// ------------------------------------------------------------------------------------------------

define_command!("Get the unit serial number.

# Command format

> `SN;`

# Response format

> `SN{text};`

The response is a variable-length decimal string, returned as raw bytes." =>
    GetSerialNumber
);

// ------------------------------------------------------------------------------------------------
// Public Types: StartTune
// ------------------------------------------------------------------------------------------------

define_command!("Initiate a tuning cycle.

There is no query form; use [`GetTuneState`] to poll for completion.

# Command format

> `ST;`" =>
    StartTune
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTuneState
// ------------------------------------------------------------------------------------------------

define_command!("Get whether a tuning cycle is currently in progress.

# Command format

> `T;`

# Response format

> `T{n};`

Where `n` is the boolean state `0` (idle) or `1` (tuning)." =>
    GetTuneState
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTunePower, SetTunePower
// ------------------------------------------------------------------------------------------------

define_command!("Get the RF power level used during a tune cycle.

# Command format

> `TP;`

# Response format

> `TP{nnn};`

Where *nnn* is the tune power, in watts." =>
    GetTunePower
);

define_command!("Set the RF power level used during a tune cycle.

# Command format

> `TP{nnn};`

Where *nnn* is the tune power, in watts." =>
    SetTunePower {
        power_w: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetForwardVoltage
// ------------------------------------------------------------------------------------------------

define_command!("Get the ADC forward voltage reading.

# Command format

> `VFWD;`

# Response format

> `VFWD{nnn};`

Where *nnn* is the raw ADC forward voltage reading, in counts." =>
    GetForwardVoltage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetReflectedVoltage
// ------------------------------------------------------------------------------------------------

define_command!("Get the ADC reflected voltage reading.

# Command format

> `VRFL;`

# Response format

> `VRFL{nnn};`

Where *nnn* is the raw ADC reflected voltage reading, in counts." =>
    GetReflectedVoltage
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSwr
// ------------------------------------------------------------------------------------------------

define_command!("Get the computed standing wave ratio.

# Command format

> `VSWR;`

# Response format

> `VSWR{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetSwr
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSwrBypassThreshold, SetSwrBypassThreshold
// ------------------------------------------------------------------------------------------------

define_command!("Get the SWR threshold above which bypass is engaged.

# Command format

> `VSWRB;`

# Response format

> `VSWRB{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetSwrBypassThreshold
);

define_command!("Set the SWR threshold above which bypass is engaged.

# Command format

> `VSWRB{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetSwrBypassThreshold {
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_command!(GetAutoBypassState => b"AB");
impl_command_with_response!(GetAutoBypassState => boolean);

impl_command!(SetAutoBypassState => b"AB" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAutoEnableState => b"AE");
impl_command_with_response!(GetAutoEnableState => boolean);

impl_command!(SetAutoEnableState => b"AE" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAtuFaultState => b"AFT");
impl_command_with_response!(GetAtuFaultState => boolean);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAtuKeepInPlaceState => b"AKIP");
impl_command_with_response!(GetAtuKeepInPlaceState => boolean);

impl_command!(SetAtuKeepInPlaceState => b"AKIP" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAmplifierInterface => b"AMPI");
impl_command_with_response!(GetAmplifierInterface => boolean);

impl_command!(SetAmplifierInterface => b"AMPI" for boolean closed);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAntenna => b"AN");
impl_command_with_response!(GetAntenna => 1, u8_from_ascii => u8);

impl_command!(
    SetAntenna => b"AN"
    format antenna uint 1,
    if |cmd: &SetAntenna| {
        if (1..=6).contains(&cmd.antenna) {
            Ok(())
        } else {
            Err(invalid_argument_value(
                "antenna",
                "u8",
                cmd.antenna
            ))
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAtuPreset => b"AP");
impl_command_with_response!(GetAtuPreset => 3, u16_from_ascii => u16);

impl_command!(SetAtuPreset => b"AP" format preset uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAttenuatorState => b"ATTN");
impl_command_with_response!(GetAttenuatorState => boolean);

impl_command!(SetAttenuatorState => b"ATTN" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetBand => b"BN");
impl_command_with_response!(GetBand => 2, u8_from_ascii => u8);

impl_command!(
    SetBand => b"BN"
    format band uint 2,
    if |cmd: &SetBand| {
        if cmd.band <= 13 {
            Ok(())
        } else {
            Err(invalid_argument_value(
                "band",
                "u8",
                cmd.band
            ))
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetBaudRate => b"#BR");
impl_command_with_response!(GetBaudRate => 1, |bytes: &[u8]| {
    match bytes[0] {
        b'0' => Ok(BaudRate::Bd4800),
        b'1' => Ok(BaudRate::Bd9600),
        b'2' => Ok(BaudRate::Bd19200),
        b'3' => Ok(BaudRate::Bd38400),
        _ => Err(enum_parse(string_from_ascii(bytes)?, "BaudRate"))
    }
} => BaudRate);

impl_command!(SetBaudRate => b"#BR" with |s: &SetBaudRate| {
    match s.baud_rate {
        BaudRate::Bd4800 => Ok(Some(vec![b'0'])),
        BaudRate::Bd9600 => Ok(Some(vec![b'1'])),
        BaudRate::Bd19200 => Ok(Some(vec![b'2'])),
        BaudRate::Bd38400 => Ok(Some(vec![b'3'])),
        _ => Err(invalid_argument_value(
            "baud_rate",
            "BaudRate",
            s.baud_rate
        ))
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(Bypass => b"BYP");

// ------------------------------------------------------------------------------------------------

impl_command!(GetCapacitorValue => b"C");
impl_command_with_response!(GetCapacitorValue => 3, u8_from_ascii => u8);

impl_command!(SetCapacitorValue => b"C" format value uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetCapacitorTopology => b"CT");
impl_command_with_response!(GetCapacitorTopology => boolean);

impl_command!(SetCapacitorTopology => b"CT" for boolean hi_z);

// ------------------------------------------------------------------------------------------------

impl_command!(GetDemoModeState => b"DM");
impl_command_with_response!(GetDemoModeState => boolean);

impl_command!(SetDemoModeState => b"DM" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(EepromInit => b"EEINIT");

// ------------------------------------------------------------------------------------------------

impl_command!(GetErrorMessage => b"EM");
impl_command_with_response!(GetErrorMessage => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFrequency => b"F");
impl_command_with_response!(GetFrequency => 8, |bytes| {
    Ok(Frequency::from(u64::from(u32_from_ascii(bytes)?)))
} => Frequency);

impl_command!(SetFrequency => b"F" with Some |cmd: &SetFrequency| {
    format!("{:08}", cmd.freq_hz.value()).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetForwardPowerA => b"FA");
impl_command_with_response!(GetForwardPowerA => 3, u16_from_ascii => u16);

impl_command!(GetForwardPowerB => b"FB");
impl_command_with_response!(GetForwardPowerB => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFanThreshold => b"FC");
impl_command_with_response!(GetFanThreshold => 3, u16_from_ascii => u16);

impl_command!(SetFanThreshold => b"FC" format threshold_w uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFaultDelayTime => b"FDT");
impl_command_with_response!(GetFaultDelayTime => 3, u16_from_ascii => u16);

impl_command!(SetFaultDelayTime => b"FDT" format delay_ms uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFaultStatus => b"FLT");
impl_command_with_response!(GetFaultStatus => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(ClearCurrentFault => b"FLTC");

// ------------------------------------------------------------------------------------------------

impl_command!(GetTuneSatisfiedSwr => b"FTNS");
impl_command_with_response!(GetTuneSatisfiedSwr => 3, u16_from_ascii => u16);

impl_command!(SetTuneSatisfiedSwr => b"FTNS" format swr_d uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFaultThresholdLow => b"FT0");
impl_command_with_response!(GetFaultThresholdLow => 3, u16_from_ascii => u16);

impl_command!(SetFaultThresholdLow => b"FT0" format swr_d uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFaultThresholdHigh => b"FT1");
impl_command_with_response!(GetFaultThresholdHigh => 3, u16_from_ascii => u16);

impl_command!(SetFaultThresholdHigh => b"FT1" format swr_d uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFixedLcState => b"FX");
impl_command_with_response!(GetFixedLcState => boolean);

impl_command!(SetFixedLcState => b"FX" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFixedBypassState => b"FY");
impl_command_with_response!(GetFixedBypassState => boolean);

impl_command!(SetFixedBypassState => b"FY" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetInductance => b"I");
impl_command_with_response!(GetInductance => 3, u8_from_ascii => u8);

impl_command!(
    SetInductance => b"I"
    format tap uint 3,
    if |cmd: &SetInductance| {
        if cmd.tap <= 63 {
            Ok(())
        } else {
            Err(invalid_argument_value(
                "tap",
                "u8",
                cmd.tap
            ))
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetInhibitFan => b"IF");
impl_command_with_response!(GetInhibitFan => boolean);

impl_command!(SetInhibitFan => b"IF" for boolean inhibit);

// ------------------------------------------------------------------------------------------------

impl_command!(GetInductorSwitch => b"L");
impl_command_with_response!(GetInductorSwitch => 3, u8_from_ascii => u8);

impl_command!(SetInductorSwitch => b"L" format mask uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetOperatingMode => b"MD");
impl_command_with_response!(GetOperatingMode => try_from enum OperatingMode);

impl_command!(SetOperatingMode => b"MD" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetMeterType => b"MT");
impl_command_with_response!(GetMeterType => try_from enum MeterType);

impl_command!(SetMeterType => b"MT" for as byte meter);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerStatus => b"PS");
impl_command_with_response!(GetPowerStatus => boolean);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerSensorInput => b"PSI");
impl_command_with_response!(GetPowerSensorInput => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(ResetDevice => b"RSTX");

// ------------------------------------------------------------------------------------------------

impl_command!(GetFirmwareVersion => b"RV");
impl_command_with_response!(GetFirmwareVersion => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAntennaSide => b"SIDE");
impl_command_with_response!(GetAntennaSide => try_from enum AntennaSide);

impl_command!(SetAntennaSide => b"SIDE" for as byte side);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTuningSpeedLimit => b"SL");
impl_command_with_response!(GetTuningSpeedLimit => 1, u8_from_ascii => u8);

impl_command!(
    SetTuningSpeedLimit => b"SL"
    format level uint 1,
    if |cmd: &SetTuningSpeedLimit| {
        if cmd.level <= 9 {
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

impl_command!(GetSwrMeter => b"SM");
impl_command_with_response!(GetSwrMeter => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSerialNumber => b"SN");
impl_command_with_response!(GetSerialNumber => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(StartTune => b"ST");

// ------------------------------------------------------------------------------------------------

impl_command!(GetTuneState => b"T");
impl_command_with_response!(GetTuneState => boolean);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTunePower => b"TP");
impl_command_with_response!(GetTunePower => 3, u16_from_ascii => u16);

impl_command!(SetTunePower => b"TP" format power_w uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetForwardVoltage => b"VFWD");
impl_command_with_response!(GetForwardVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetReflectedVoltage => b"VRFL");
impl_command_with_response!(GetReflectedVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSwr => b"VSWR");
impl_command_with_response!(GetSwr => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSwrBypassThreshold => b"VSWRB");
impl_command_with_response!(GetSwrBypassThreshold => 3, u16_from_ascii => u16);

impl_command!(SetSwrBypassThreshold => b"VSWRB" format swr_d uint 3);
