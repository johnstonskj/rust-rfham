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
    error::{RigError, invalid_argument_value},
    protocol::{
        Frequency,
        cat::common::{bytes_to_vec, u8_from_ascii, u16_from_ascii, u32_from_ascii},
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoBypassState, SetAutoBypassState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether automatic bypass is enabled (`AB`).

When automatic bypass is on, the tuner monitors SWR and switches itself out of circuit whenever
the untuned SWR is already acceptable.

# Command format

> `AB;`

# Response format

> `AB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAutoBypassState
);

define_cat_command!("Set whether automatic bypass is enabled (`AB`).

# Command format

> `AB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoBypassState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoEnableState, SetAutoEnableState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the ATU is enabled, i.e. whether automatic tuning is allowed (`AE`).

# Command format

> `AE;`

# Response format

> `AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAutoEnableState
);

define_cat_command!("Set whether the ATU is enabled, i.e. whether automatic tuning is allowed (`AE`).

# Command format

> `AE{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAutoEnableState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultConditionState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the ATU currently has a fault condition (`AFT`).

# Command format

> `AFT;`

# Response format

> `AFT{n};`

Where `n` is the boolean state `0` (no fault) or `1` (fault present)." =>
    GetFaultConditionState
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuKeepInPlaceState, SetAtuKeepInPlaceState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the ATU keep-in-place state (`AKIP`).

When keep-in-place is on, the tuner retains its last L/C setting rather than re-tuning on band or
frequency changes.

# Command format

> `AKIP;`

# Response format

> `AKIP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetKeepInPlaceState
);

define_cat_command!("Set the ATU keep-in-place state (`AKIP`).

# Command format

> `AKIP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetKeepInPlaceState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAmplifierInterface, SetAmplifierInterface
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the amplifier interface relay state (`AMPI`).

# Command format

> `AMPI;`

# Response format

> `AMPI{n};`

Where `n` is the boolean state `0` (open) or `1` (closed)." =>
    GetAmplifierInterfaceRelayClosedState
);

define_cat_command!("Set the amplifier interface relay state (`AMPI`).

# Command format

> `AMPI{n};`

Where `n` is the boolean state `0` (open) or `1` (closed)." =>
    SetAmplifierInterfaceRelayClosedState {
        state: AmplifierInterfaceRelayState
    }
);

define_command_enum!(
    "Amplifier interface relay state." =>
    AmplifierInterfaceRelayState {
        "Open" => Open = b'0',
        "Closed" => Closed = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntenna, SetAntenna
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the currently selected antenna port (`AN`).

# Command format

> `AN;`

# Response format

> `AN{n};`

Where *n* is the antenna port number, between `1` and `6`." =>
    GetAntennaSelection
);

define_cat_command!("Set the currently selected antenna port (`AN`).

# Command format

> `AN{n};`

Where *n* is the antenna port number, between `1` and `6`." =>
    SetAntennaSelection {
        antenna: SelectedAntenna
    }
);

define_command_enum!(
    "Antenna selection." =>
    SelectedAntenna {
        "Antenna Port 1" => Antenna1 = b'1',
        "Antenna Port 2" => Antenna2 = b'2',
        "Antenna Port 3" => Antenna3 = b'3',
        "Antenna Port 4" => Antenna4 = b'4',
        "Antenna Port 5" => Antenna5 = b'5',
        "Antenna Port 6" => Antenna6 = b'6'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuPreset, SetAtuPreset
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current ATU preset slot number (`AP`).

# Command format

> `AP;`

# Response format

> `AP{nnn};`

Where *nnn* is the 3-digit preset slot number." =>
    GetPresetSlotNumber
);

define_cat_command!("Set the current ATU preset slot number (`AP`).

# Command format

> `AP{nnn};`

Where *nnn* is the 3-digit preset slot number." =>
    SetPresetSlotNumber {
        slot_number: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAttenuatorState, SetAttenuatorState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the built-in attenuator is enabled (`ATTN`).

# Command format

> `ATTN;`

# Response format

> `ATTN{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetAttenuatorState
);

define_cat_command!("Set whether the built-in attenuator is enabled (`ATTN`).

# Command format

> `ATTN{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetAttenuatorState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBand, SetBand
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current band number (`BN`).

# Command format

> `BN;`

# Response format

> `BN{nn};`

Where *nn* is the 2-digit band number, between `00` and `13`." =>
    GetBand
);

define_cat_command!("Set the current band number (`BN`).

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

define_cat_command!("Get the serial port baud rate (`#BR`).

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

define_cat_command!("Set the serial port baud rate (`#BR`).

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

define_command_enum!(
    "RS-232 baud rate (K3/K3S only)." =>
    BaudRate {
        "4,800 baud" => Rate4800 = b'0',
        "9,600 baud" => Rate9600 = b'1',
        "19,200 baud" => Rate19200 = b'2',
        "38,400 baud" => Rate38400 = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: ForceBypassMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Force the ATU into bypass mode immediately (`BYP`).

This command takes effect immediately and has no query form.

# Command format

> `BYP;`" =>
    ForceBypassMode
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCapacitorValue, SetCapacitorValue
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the tuning capacitor value (`C`).

# Command format

> `C;`

# Response format

> `C{nnn};`

Where *nnn* is the 3-digit capacitor value, between `000` and `255`." =>
    GetCapacitorValue
);

define_cat_command!("Set the tuning capacitor value (`C`).

# Command format

> `C{nnn};`

Where *nnn* is the 3-digit capacitor value, between `000` and `255`." =>
    SetCapacitorValue {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCapacitorTopology, SetCapacitorTopology, CapacitorTopology
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the tuning capacitor topology (hi-Z or lo-Z) (`CT`).

# Command format

> `CT;`

# Response format

> `CT{n};`

Where `n` is `0` (lo-Z, capacitor on the output side) or `1` (hi-Z, capacitor on the input side).
See [`CapacitorTopology`] for details." =>
    GetCapacitorTopology
);

define_cat_command!("Set the tuning capacitor topology (hi-Z or lo-Z). (`CT`).

# Command format

> `CT{n};`

Where `n` is `0` (lo-Z, capacitor on the output side) or `1` (hi-Z, capacitor on the input side).
See [`CapacitorTopology`] for details." =>
    SetCapacitorTopology {
        topology: CapacitorTopology
    }
);

define_command_enum!(
    "Tuning capacitor topology." =>
    CapacitorTopology {
        "Lo-Z; capacitor on the output side." => LowZ = b'0',
        "Hi-Z; capacitor on the input side." => HighZ = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDemoModeState, SetDemoModeState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the demo-mode state (`DM`).

# Command format

> `DM;`

# Response format

> `DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetDemoModeState
);

define_cat_command!("Set the demo-mode state (`DM`).

# Command format

> `DM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetDemoModeState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: ResetToFactoryDefaults
// ------------------------------------------------------------------------------------------------

define_cat_command!("Re-initialize EEPROM storage to factory defaults (`EEINIT`).

This is destructive: it erases all stored antenna and band presets. There is no query form.

# Command format

> `EEINIT;`" =>
    ResetToFactoryDefaults
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetErrorMessage
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the last error message string (`EM`).

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

define_cat_command!("Get the operating frequency, in Hz. (`F`).

# Command format

> `F;`

# Response format

> `F{nnnnnnnn};`

Where *nnnnnnnn* is the frequency, in Hz, as an 8-digit zero-padded decimal value." =>
    GetFrequency
);

define_cat_command!("Set the operating frequency, in Hz. (`F`).

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

define_cat_command!("Get the forward power reading on meter channel A (`FA`).

# Command format

> `FA;`

# Response format

> `FA{nnn};`

Where *nnn* is the forward power, in deci-watts (tenths of a watt)." =>
    GetMeterChannelAForwardPower
);

define_cat_command!("Get the forward power reading on meter channel B (`FB`).

# Command format

> `FB;`

# Response format

> `FB{nnn};`

Where *nnn* is the forward power, in deci-watts (tenths of a watt)." =>
    GetMeterChannelBForwardPower
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFanThreshold, SetFanThreshold
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the fan-on power threshold (`FC`).

# Command format

> `FC;`

# Response format

> `FC{nnn};`

Where *nnn* is the 3-digit threshold, in watts, above which the cooling fan turns on." =>
    GetFanThreshold
);

define_cat_command!("Set the fan-on power threshold (`FC`).

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

define_cat_command!("Get the fault delay time (`FDT`).

# Command format

> `FDT;`

# Response format

> `FDT{nnn};`

Where *nnn* is the 3-digit fault delay time, in milliseconds." =>
    GetFaultDelayTime
);

define_cat_command!("Set the fault delay time (`FDT`).

# Command format

> `FDT{nnn};`

Where *nnn* is the 3-digit fault delay time, in milliseconds." =>
    SetFaultDelayTime {
        delay_ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultConditionCode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current fault condition/status code (`FLT`).

# Command format

> `FLT;`

# Response format

> `FLT{nn};`

Where *nn* is the 2-digit fault status code; `00` indicates no fault." =>
    GetFaultConditionCode
);

// ------------------------------------------------------------------------------------------------
// Public Types: ClearFaultCondition
// ------------------------------------------------------------------------------------------------

define_cat_command!("Clear the current fault condition (`FLTC`).

There is no query form.

# Command format

> `FLTC;`" =>
    ClearFaultCondition
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTuneSatisfiedSwr, SetTuneSatisfiedSwr
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the SWR threshold below which a tune cycle is considered successful (`FTNS`).

# Command format

> `FTNS;`

# Response format

> `FTNS{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetTuneSatisfiedSwrThreshold
);

define_cat_command!("Set the SWR threshold below which a tune cycle is considered successful (`FTNS`).

# Command format

> `FTNS{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetTuneSatisfiedSwrThreshold {
        swr: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultSwrThresholdLow, SetFaultSwrThresholdLow
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the lower fault SWR threshold (`FT0`).

# Command format

> `FT0;`

# Response format

> `FT0{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetFaultSwrThresholdLow
);

define_cat_command!("Set the lower fault SWR threshold (`FT0`).

# Command format

> `FT0{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetFaultSwrThresholdLow {
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultSwrThresholdHigh, SetFaultSwrThresholdHigh
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the upper fault SWR threshold (`FT1`).

# Command format

> `FT1;`

# Response format

> `FT1{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetFaultSwrThresholdHigh
);

define_cat_command!("Set the upper fault SWR threshold (`FT1`).

# Command format

> `FT1{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    SetFaultSwrThresholdHigh {
        swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFixedLcState, SetFixedLcState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether fixed L/C mode is enabled (`FX`).

When fixed L/C mode is on, the tuner holds a fixed inductor/capacitor setting rather than
re-tuning.

# Command format

> `FX;`

# Response format

> `FX{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetFixedLcState
);

define_cat_command!("Set whether fixed L/C mode is enabled (`FX`).

# Command format

> `FX{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetFixedLcState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFixedBypassState, SetFixedBypassState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether fixed bypass mode is active (`FY`).

# Command format

> `FY;`

# Response format

> `FY{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetFixedBypassState
);

define_cat_command!("Set whether fixed bypass mode is active (`FY`).

# Command format

> `FY{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetFixedBypassState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInductance, SetInductance
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the tuning inductance tap (`I`).

# Command format

> `I;`

# Response format

> `I{nnn};`

Where *nnn* is the 3-digit inductor tap value, between `000` and `063`." =>
    GetInductanceTap
);

define_cat_command!("Set the tuning inductance tap (`I`).

# Command format

> `I{nnn};`

Where *nnn* is the 3-digit inductor tap value, between `000` and `063`." =>
    SetInductanceTap {
        tap: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInhibitFan, SetInhibitFan
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the cooling fan is inhibited (`IF`).

# Command format

> `IF;`

# Response format

> `IF{n};`

Where `n` is `0` (fan enabled) or `1` (fan inhibited)." =>
    GetInhibitFan
);

define_cat_command!("Set whether the cooling fan is inhibited (`IF`).

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

define_cat_command!("Get the inductor switch bitmask (`L`).

# Command format

> `L;`

# Response format

> `L{nnn};`

Where *nnn* is the 3-digit inductor switch bitmask." =>
    GetInductorSwitch
);

define_cat_command!("Set the inductor switch bitmask directly (`L`).

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

define_cat_command!("Get the current ATU operating mode (`MD`).

# Command format

> `MD;`

# Response format

> `MD{n};`

Where *n* is one of:

* `0`; automatic tuning.
* `1`; semi-automatic tuning.
* `2`; manual tuning. See [`OperatingMode`] for details." =>
    GetOperatingMode
);

define_cat_command!("Set the current ATU operating mode (`MD`).

# Command format

> `MD{n};`

Where *n* is one of:

* `0`; automatic tuning.
* `1`; semi-automatic tuning.
* `2`; manual tuning. See [`OperatingMode`] for details." =>
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

define_cat_command!("Get the front-panel meter display type (`MT`).

# Command format

> `MT;`

# Response format

> `MT{n};`

Where *n* is one of:

* `0`; SWR.
* `1`; power.
* `2`; reflected power. See [`MeterType`] for details." =>
    GetMeterType
);

define_cat_command!("Set the front-panel meter display type (`MT`).

# Command format

> `MT{n};`

Where *n* is one of:

* `0`; SWR.
* `1`; power.
* `2`; reflected power. See [`MeterType`] for details." =>
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

define_cat_command!("Get the power-on status (`PS`).

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

define_cat_command!("Get the forward power reading from the internal sensor (`PSI`).

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

define_cat_command!("Perform a soft reset of the KAT500, triggering a firmware restart (`RSTX`).

There is no query form.

# Command format

> `RSTX;`" =>
    ResetDevice
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the firmware version string (`RV`).

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

define_cat_command!("Get the antenna side selection (`SIDE`).

# Command format

> `SIDE;`

# Response format

> `SIDE{n};`

Where *n* is one of:

* `0`; left.
* `1`; right." =>
    GetAntennaSideSelection
);

define_cat_command!("Set the antenna side selection (`SIDE`).

# Command format

> `SIDE{n};`

Where *n* is one of:

* `0`; left.
* `1`; right." =>
    SetAntennaSideSelection {
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

define_cat_command!("Get the tuning speed limit setting (`SL`).

# Command format

> `SL;`

# Response format

> `SL{n};`

Where *n* is the tuning speed limit, between `0` (fastest) and `9` (slowest)." =>
    GetTuningSpeedLimit
);

define_cat_command!("Set the tuning speed limit (`SL`).

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

define_cat_command!("Get the current SWR meter reading (`SM`).

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

define_cat_command!("Get the unit serial number (`SN`).

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

define_cat_command!("Initiate a tuning cycle (`ST`).

There is no query form; use [`GetTuningState`] to poll for completion.

# Command format

> `ST;`" =>
    StartTuningCycle
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTuneState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether a tuning cycle is currently in progress (`T`).

# Command format

> `T;`

# Response format

> `T{n};`

Where `n` is the boolean state `0` (idle) or `1` (tuning)." =>
    GetTuningState
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTunePower, SetTunePower
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the RF power level used during a tune cycle (`TP`).

# Command format

> `TP;`

# Response format

> `TP{nnn};`

Where *nnn* is the tune power, in watts." =>
    GetTuningPower
);

define_cat_command!("Set the RF power level used during a tune cycle (`TP`).

# Command format

> `TP{nnn};`

Where *nnn* is the tune power, in watts." =>
    SetTuningPower {
        power_w: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetForwardVoltage
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the ADC forward voltage reading (`VFWD`).

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

define_cat_command!("Get the ADC reflected voltage reading (`VRFL`).

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

define_cat_command!("Get the computed standing wave ratio (`VSWR`).

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

define_cat_command!("Get the SWR threshold above which bypass is engaged (`VSWRB`).

# Command format

> `VSWRB;`

# Response format

> `VSWRB{nnn};`

Where *nnn* is SWR × 10, e.g. `150` represents an SWR of 1.5:1." =>
    GetSwrBypassThreshold
);

define_cat_command!("Set the SWR threshold above which bypass is engaged (`VSWRB`).

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

impl_cat_command!(GetAutoBypassState => b"AB");
impl_cat_command_with_response!(GetAutoBypassState => boolean);

impl_cat_command!(SetAutoBypassState => b"AB" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAutoEnableState => b"AE");
impl_cat_command_with_response!(GetAutoEnableState => boolean);

impl_cat_command!(SetAutoEnableState => b"AE" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetKeepInPlaceState => b"AKIP");
impl_cat_command_with_response!(GetKeepInPlaceState => boolean);

impl_cat_command!(SetKeepInPlaceState => b"AKIP" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAmplifierInterfaceRelayClosedState => b"AMPI");
impl_cat_command_with_response!(
    GetAmplifierInterfaceRelayClosedState => try_from enum AmplifierInterfaceRelayState
);

impl_cat_command!(SetAmplifierInterfaceRelayClosedState => b"AMPI" for as byte state);
impl_set_cat_command_from_enum!(
    SetAmplifierInterfaceRelayClosedState, AmplifierInterfaceRelayState => state {
        Open => open,
        Closed => close
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAntennaSelection => b"AN");
impl_cat_command_with_response!(GetAntennaSelection => try_from enum SelectedAntenna);

impl_cat_command!(SetAntennaSelection => b"AN" for as byte antenna);
impl_set_cat_command_from_enum!(
    SetAntennaSelection, SelectedAntenna => antenna {
        Antenna1 => select_antenna_1,
        Antenna2 => select_antenna_2,
        Antenna3 => select_antenna_3,
        Antenna4 => select_antenna_4,
        Antenna5 => select_antenna_5,
        Antenna6 => select_antenna_6
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPresetSlotNumber => b"AP");
impl_cat_command_with_response!(GetPresetSlotNumber => 3, u16_from_ascii => u16);

impl_cat_command!(SetPresetSlotNumber => b"AP" format slot_number uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAttenuatorState => b"ATTN");
impl_cat_command_with_response!(GetAttenuatorState => boolean);

impl_cat_command!(SetAttenuatorState => b"ATTN" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBand => b"BN");
impl_cat_command_with_response!(GetBand => 2, u8_from_ascii => u8);

impl_cat_command!(
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

impl_cat_command!(GetBaudRate => b"#BR");
impl_cat_command_with_response!(GetBaudRate => try_from enum BaudRate);

impl_cat_command!(SetBaudRate => b"#BR" for as byte baud_rate);
impl_set_cat_command_from_enum!(SetBaudRate, BaudRate => baud_rate {
    Rate4800 => to_4800,
    Rate9600 => to_9600,
    Rate19200 => to_19200,
    Rate38400 => to_38400
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(ForceBypassMode => b"BYP");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetCapacitorValue => b"C");
impl_cat_command_with_response!(GetCapacitorValue => 3, u8_from_ascii => u8);

impl_cat_command!(SetCapacitorValue => b"C" format value uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetCapacitorTopology => b"CT");
impl_cat_command_with_response!(GetCapacitorTopology => try_from enum CapacitorTopology);

impl_cat_command!(SetCapacitorTopology => b"CT" for as byte topology);
impl_set_cat_command_from_enum!(SetCapacitorTopology, CapacitorTopology => topology {
    LowZ => to_low_z,
    HighZ => to_high_z
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDemoModeState => b"DM");
impl_cat_command_with_response!(GetDemoModeState => boolean);

impl_cat_command!(SetDemoModeState => b"DM" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(ResetToFactoryDefaults => b"EEINIT");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetErrorMessage => b"EM");
impl_cat_command_with_response!(GetErrorMessage => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFrequency => b"F");
impl_cat_command_with_response!(GetFrequency => 8, |bytes| {
    Ok(Frequency::from(u64::from(u32_from_ascii(bytes)?)))
} => Frequency);

impl_cat_command!(SetFrequency => b"F" with Some |cmd: &SetFrequency| {
    format!("{:08}", cmd.freq_hz.value()).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMeterChannelAForwardPower => b"FA");
impl_cat_command_with_response!(GetMeterChannelAForwardPower => 3, u16_from_ascii => u16);

impl_cat_command!(GetMeterChannelBForwardPower => b"FB");
impl_cat_command_with_response!(GetMeterChannelBForwardPower => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFanThreshold => b"FC");
impl_cat_command_with_response!(GetFanThreshold => 3, u16_from_ascii => u16);

impl_cat_command!(SetFanThreshold => b"FC" format threshold_w uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultConditionState => b"AFT");
impl_cat_command_with_response!(GetFaultConditionState => boolean);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultDelayTime => b"FDT");
impl_cat_command_with_response!(GetFaultDelayTime => 3, u16_from_ascii => u16);

impl_cat_command!(SetFaultDelayTime => b"FDT" format delay_ms uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultConditionCode => b"FLT");
impl_cat_command_with_response!(GetFaultConditionCode => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(ClearFaultCondition => b"FLTC");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTuneSatisfiedSwrThreshold => b"FTNS");
impl_cat_command_with_response!(GetTuneSatisfiedSwrThreshold => 3, u16_from_ascii => u16);

impl_cat_command!(SetTuneSatisfiedSwrThreshold => b"FTNS" format swr uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultSwrThresholdLow => b"FT0");
impl_cat_command_with_response!(GetFaultSwrThresholdLow => 3, u16_from_ascii => u16);

impl_cat_command!(SetFaultSwrThresholdLow => b"FT0" format swr_d uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultSwrThresholdHigh => b"FT1");
impl_cat_command_with_response!(GetFaultSwrThresholdHigh => 3, u16_from_ascii => u16);

impl_cat_command!(SetFaultSwrThresholdHigh => b"FT1" format swr_d uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFixedLcState => b"FX");
impl_cat_command_with_response!(GetFixedLcState => boolean);

impl_cat_command!(SetFixedLcState => b"FX" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFixedBypassState => b"FY");
impl_cat_command_with_response!(GetFixedBypassState => boolean);

impl_cat_command!(SetFixedBypassState => b"FY" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetInductanceTap => b"I");
impl_cat_command_with_response!(GetInductanceTap => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetInductanceTap => b"I"
    format tap uint 3,
    if |cmd: &SetInductanceTap| {
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

impl_cat_command!(GetInhibitFan => b"IF");
impl_cat_command_with_response!(GetInhibitFan => boolean);

impl_cat_command!(SetInhibitFan => b"IF" for boolean inhibit);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetInductorSwitch => b"L");
impl_cat_command_with_response!(GetInductorSwitch => 3, u8_from_ascii => u8);

impl_cat_command!(SetInductorSwitch => b"L" format mask uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetOperatingMode => b"MD");
impl_cat_command_with_response!(GetOperatingMode => try_from enum OperatingMode);

impl_cat_command!(SetOperatingMode => b"MD" for as byte mode);
impl_set_cat_command_from_enum!(SetOperatingMode, OperatingMode => mode {
    Auto => to_auto,
    SemiAuto => to_semi_auto,
    Manual => to_manual
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMeterType => b"MT");
impl_cat_command_with_response!(GetMeterType => try_from enum MeterType);

impl_cat_command!(SetMeterType => b"MT" for as byte meter);
impl_set_cat_command_from_enum!(SetMeterType, MeterType => meter {
    Swr => to_swr,
    Power => to_power,
    Reflected => to_reflected
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerStatus => b"PS");
impl_cat_command_with_response!(GetPowerStatus => boolean);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerSensorInput => b"PSI");
impl_cat_command_with_response!(GetPowerSensorInput => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(ResetDevice => b"RSTX");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFirmwareVersion => b"RV");
impl_cat_command_with_response!(GetFirmwareVersion => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAntennaSideSelection => b"SIDE");
impl_cat_command_with_response!(GetAntennaSideSelection => try_from enum AntennaSide);

impl_cat_command!(SetAntennaSideSelection => b"SIDE" for as byte side);
impl_set_cat_command_from_enum!(SetAntennaSideSelection, AntennaSide => side {
    Left => to_left,
    Right => to_right
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTuningSpeedLimit => b"SL");
impl_cat_command_with_response!(GetTuningSpeedLimit => 1, u8_from_ascii => u8);

impl_cat_command!(
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

impl_cat_command!(GetSwrMeter => b"SM");
impl_cat_command_with_response!(GetSwrMeter => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSerialNumber => b"SN");
impl_cat_command_with_response!(GetSerialNumber => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(StartTuningCycle => b"ST");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTuningState => b"T");
impl_cat_command_with_response!(GetTuningState => boolean);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTuningPower => b"TP");
impl_cat_command_with_response!(GetTuningPower => 3, u16_from_ascii => u16);

impl_cat_command!(SetTuningPower => b"TP" format power_w uint 3);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetForwardVoltage => b"VFWD");
impl_cat_command_with_response!(GetForwardVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetReflectedVoltage => b"VRFL");
impl_cat_command_with_response!(GetReflectedVoltage => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSwr => b"VSWR");
impl_cat_command_with_response!(GetSwr => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSwrBypassThreshold => b"VSWRB");
impl_cat_command_with_response!(GetSwrBypassThreshold => 3, u16_from_ascii => u16);

impl_cat_command!(SetSwrBypassThreshold => b"VSWRB" format swr_d uint 3);
