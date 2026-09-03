//!
//! CAT commands for the Elecraft PX3 panadapter.
//!
//! Many of the commands supported by the PX3 are identical to those supported by the P3. In this
//! case only when the PX3 version is significantly different are they documented here.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # Supported P3 Commands
//!
//! * `GetProductId`, `ProductId`
//! * `GetDisplayAveragingTimeConstant`, `SetDisplayAveragingTimeConstant`
//! * `UploadScreenshotBitmap`, `BitmapData`
//! * `SetBaudRate` (sends `#BR`)
//! * `GetCenterFrequency`, `SetCenterFrequency`
//! * `GetFunctionKeyLabel`, `FunctionKeyLabel`
//! * `ExecuteFunctionKey`
//! * `GetFixedTuneAutoAdjustMode`, `SetFixedTuneAutoAdjustMode`, `FixedTuneAutoAdjustMode`
//! * `GetNoiseBlankerState`, `SetNoiseBlankerState`
//! * `GetNoiseBlankerLevel`, `SetNoiseBlankerLevel`
//! * `GetPeakModeState`, `SetPeakModeState`
//! * `GetPowerStatus`, `SetPowerStatus`
//! * `SetPassThroughModeState`
//! * `SetQsyToMarker`, `QsyAction`
//! * `GetRelativeCenterFrequency`, `SetRelativeCenterFrequency`
//! * `GetReferenceLevel`, `SetReferenceLevel`
//! * `Reset`
//! * `GetScale`, `SetScale`
//! * `GetSpan`, `SetSpan`
//! * `GetVfoBCursorState`, `SetVfoBCursorState`
//!
//! # References
//!
//! 1. [Elecraft PX3 Programmer's Reference, rev. A6](https://ftp.elecraft.com/PX3/Manuals%20Downloads/PX3_Pgmrs_Ref_A6.pdf), Feb 2017.
//!

use crate::{
    error::{RigError, invalid_argument_value},
    protocol::cat::{
        common::{i16_from_ascii, u8_from_ascii, u16_from_ascii, u32_from_ascii},
        elecraft::k3_kx::VfoFrequencyChangeStep,
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandscopeChannelIndicatorState, SetBandscopeChannelIndicatorState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the beacon transmission interval (`#BCI`).

# Command format

> `#BCI;`

# Response format

> `#BCI{nnnn};`

where *nnnn* is the beacon interval, or time to wait between beacon transmissions, in seconds when
beacon mode is activated. The value can be between `1` and `3600`." =>
    GetBeaconTransmissionInterval
);

define_cat_command!("Set the beacon transmission interval (`#BCI`).

# Command format

> `#BCI{nnnn};`

where *nnnn* is the beacon interval, or time to wait between beacon transmissions, in seconds when
beacon mode is activated. The value can be between `1` and `3600`." =>
    SetBeaconTransmissionInterval {interval_secs: u16 }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandscopeChannelList, SetBandscopeChannelList
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the beacon text memory location (`#BCL`).

# Command format

> `#BCL;`

# Response format

> `#BCL{nn};`

where *nn* is the beacon text memory location, between `0` and `50`." =>
    GetBeaconTextMemoryLocation
);

define_cat_command!("Set the beacon text memory location (`#BCL`).

# Command format

> `#BCL{nn};`

where *nn* is the beacon text memory location, between `0` and `50`." =>
    SetBeaconTextMemoryLocation { location: u8 }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandscopeChannelName, SetBandscopeChannelName
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the beacon mode on/off state (`#BCN`).

# Command format

> `#BCN;`

# Response format

> `#BCN{n};`

Where *n* is the boolean state `0` (off) or `1` (on)." =>
    GetBeaconModeState
);

define_cat_command!("Set the beacon mode on/off state (`#BCN`).

# Command format

> `#BCN{n};`

Where *n* is the boolean state `0` (off) or `1` (on)." =>
    SetBeaconModeState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCalibSignal, SetCalibSignal
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the calibration signal on/off state (`#CAL`).

# Command format

> `#CAL;`

# Response format

> `#CAL{n};`

Where *n* is the boolean state `0` (off) or `1` (on).

**Note**: the calibration signal generated is based on the current KX3 band." =>
    GetCalibrationSignalState
);

define_cat_command!("Set the calibration signal on/off state (`#CAL`).

# Command format

> `#CAL{n};`

Where *n* is the boolean state `0` (off) or `1` (on).

**Note**: the calibration signal generated is based on the current KX3 band." =>
    SetCalibrationSignalState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDisplayMode, SetDisplayMode, DisplayMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PX3 display mode (`#DSM`).

# Command format

> `#DSM;`

# Response format

> `#DSM{n};`

Where *n* is one of:

* `0`; spectrum only.
* `1`; spectrum + waterfall." =>
    GetDisplayMode
);

define_cat_command!("Set the PX3 display mode (`#DSM`).

# Command format

> `#DSM{n};`

Where *n* is one of:

* `0`; spectrum only.
* `1`; spectrum + waterfall." =>
    SetDisplayMode {
        mode: DisplayMode
    }
);

define_command_enum!(
    "P3 display mode." => DisplayMode {
        "Spectrum display only." => SpectrumOnly = b'0',
        "Spectrum display plus waterfall." => SpectrumAndWaterfall = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFunctionKeyLabelDisplayState, SetFunctionKeyLabelDisplayState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether FN key labels are shown on the display (`#LBL`).

# Command format

> `#LBL;`

# Response format

> `#LBL{n};`

Where *n* is one of: `0` (FN key labels off), `1` (FN key labels on) or `2` (text decode on)." =>
    GetFunctionKeyLabelDisplayState
);

define_cat_command!("Set whether FN key labels are shown on the display (`#LBL`).

# Command format

> `#LBL{n};`

Where *n* is one of: `0` (FN key labels off), `1` (FN key labels on) or `2` (text decode on)." =>
    SetFunctionKeyLabelDisplayState { state: FunctionKeyLabelDisplayState }
);

define_command_enum!(
    "FN key label display state." => FunctionKeyLabelDisplayState {
        "FN key labels are not shown on the display." => Off = b'0',
        "FN key labels are shown on the display." => On = b'1',
        "Text decode on the display." => TextDecodeOn = b'2'
    }
);
// ------------------------------------------------------------------------------------------------
// Public Types: MoveMarkerAFrequency
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move marker A frequency up/down (`#MAA`).

There are two ways to use this command, either specify an adjustment selection or use an internal
value which is based on the current span and mode. Specifying the adjustment uses the same values
as used by the K3/KX3 UP and DN commands ([`VfoFrequencyChangeStep`]).

**Note**: If the marker is turned off, the marker frequency will still be updated.

# Command format

> `#MAA{s}{n};`

Where *s* = `+` to increment, `-` to decrement.

Where *n* is the increment/decrement value, one of:

* `0` adjust by 1 Hz
* `1` adjust by 10 Hz
* `2` adjust by 20 Hz
* `3` adjust by 50 Hz
* `4` adjust by 1 kHz
* `5` adjust by 2 kHz
* `6` adjust by 3 kHz
* `7` adjust by 5 kHz
* `8` adjust by 100 Hz
* `9` adjust by 200 Hz

# Alternate command format

> `#MAA{s};`

Where *s* = `+` to increment, `-` to decrement.

The step size is automatically determined by the current span and mode:

* USB, LSB, AM & FM
  * Span < 5 kHz, step = 10 Hz
  * Span 2-9.99 kHz, step = 20 Hz
  * Span 10-49.9 kHz, step = 50 Hz
  * Span 50-99.1 kHz, step = 100 Hz
  * Span 100-200 kHz, step = 200 Hz
* CW & Data
  * Span < 5 kHz, step = 2 Hz
  * Span 2-9.99 kHz, step = 10 Hz
  * Span 10-49.9 kHz, step = 20 Hz
  * Span 50-99.1 kHz, step = 50 Hz
  * Span 100-200 kHz, step = 100 Hz" =>
    MoveMarkerAFrequency {
        step: Option<VfoFrequencyChangeStep>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: MoveMarkerBFrequency
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move marker B frequency up/down (`#MBA`).

There are two ways to use this command, either specify an adjustment selection or use an internal
value which is based on the current span and mode. Specifying the adjustment uses the same values
as used by the K3/KX3 UP and DN commands ([`VfoFrequencyChangeStep`]).

**Note**: If the marker is turned off, the marker frequency will still be updated.

# Command format

> `#MBA{s}{n};`

Where *s* = `+` to increment, `-` to decrement.

Where *n* is the increment/decrement value, one of:

* `0` adjust by 1 Hz
* `1` adjust by 10 Hz
* `2` adjust by 20 Hz
* `3` adjust by 50 Hz
* `4` adjust by 1 kHz
* `5` adjust by 2 kHz
* `6` adjust by 3 kHz
* `7` adjust by 5 kHz
* `8` adjust by 100 Hz
* `9` adjust by 200 Hz.

# Alternate command format

> `#MAA{s};`

Where *s* = `+` to increment, `-` to decrement.

The step size is automatically determined by the current span and mode:

* USB, LSB, AM & FM
  * Span < 5 kHz, step = 10 Hz
  * Span 2-9.99 kHz, step = 20 Hz
  * Span 10-49.9 kHz, step = 50 Hz
  * Span 50-99.1 kHz, step = 100 Hz
  * Span 100-200 kHz, step = 200 Hz
* CW & Data
  * Span < 5 kHz, step = 2 Hz
  * Span 2-9.99 kHz, step = 10 Hz
  * Span 10-49.9 kHz, step = 20 Hz
  * Span 50-99.1 kHz, step = 50 Hz
  * Span 100-200 kHz, step = 100 Hz" =>
    MoveMarkerBFrequency {
        step: Option<VfoFrequencyChangeStep>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOffscreenBandscopePosition, SetOffscreenBandscopePositionGetNoiseBlankerState, SetNoiseBlankerState*
// ------------------------------------------------------------------------------------------------

define_cat_command!("Save screenshot to flash drive (`#MSS`).

Creates a bitmap copy of the LCD screen (screen shot) and saves it to the MSD flash drive (thumb 
drive, flash memory stick). Each time the screen shot is performed, a new file is created. Filenames
use a numeric format in which the first 3 characters are `PX3` followed by a 5 digit number, i.e.
`PX300009.BMP`.

**Note**: while the PX3 is busy saving a screen shot, other commands will be received but not
processed.

# Command format

> `#MSS;`
" =>
    SaveScreenshotToFlashDrive
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOppositeSideBandNullAmplitude, SetOppositeSideBandNullAmplitude
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the opposite side band null amplitude value (`#OSBA`).

# Command format

> `#OSBA;`

# Response format

> `#OSBA{s}{nnnn};`

Where *s* is the sign (`+`, `-`, or `␣`) and *nnnn* is the value in the range `-9999` to `+9999`.

**Note**: this is a per-band setting." =>
    GetOppositeSideBandNullAmplitude
);

define_cat_command!("Set the opposite side band null amplitude value (`#OSBA`).

# Command format

> `#OSBA{s}{nnnn};`

Where *s* is the sign (`+`, `-`, or `␣`) and *nnnn* is the value in the range `-9999` to `+9999`.

**Note**: this is a per-band setting." =>
    SetOppositeSideBandNullAmplitude {
        amplitude: i16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOppositeSideBandNullPhase, SetOppositeSideBandNullPhase
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the opposite side band null phase value (`#OSBP`).

# Command format

> `#OSBP;`

# Response format

> `#OSBP{s}{nnnn};`

Where *s* is the sign (`+`, `-`, or `␣`) and *nnnn* is the value in the range `-9999` to `+9999`.

**Note**: this is a per-band setting." =>
    GetOppositeSideBandNullPhase
);

define_cat_command!("Set the opposite side band null phase value (`#OSBP`).

# Command format

> `#OSBP{s}{nnn};`

Where *s* is the sign (`+`, `-`, or `␣`) and *nnn* is the value (multiplied by 10) in the range
`-450` (-45.0) to `450` (45.0).

**Note**: this is a per-band setting." =>
    SetOppositeSideBandNullPhase {
        phase: i16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTextHangTime, SetTextHangTime
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the text hang time in milliseconds (`#TXH`).

# Command format

> `#TXH;`

# Response format

> `#TXH{nnnnn};`

Where *nnnnn* is the time in milliseconds to keep the KX3 transmitting after the last PX3 keyboard
text character is sent; values `00000` to `90000`." =>
    GetTextHangTime
);

define_cat_command!("Set the text hang time in milliseconds (`#TXH`).

# Command format

> `#TXH{nnnnn};`

Where *nnnnn* is the time in milliseconds to keep the KX3 transmitting after the last PX3 keyboard
text character is sent; values `00000` to `90000`." =>
    SetTextHangTime {time_ms: u32 }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTextTransmitMode, SetTextTransmitMode, TextTransmitMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the text transmit mode (`#TXM`).

# Command format

> `#TXM;`

# Response format

> `#TXM{nn};`

Where *nn* is one of:  `00` (Enter key), `01` (^R/^T toggle), `02` (Any key), or `03` (Space key)." =>
    GetTextTransmitMode
);

define_cat_command!("Set the text transmit mode (`#TXM`).

# Command format

> `#TXM{nn};`

Where *nn* is one of:  `00` (Enter key), `01` (^R/^T toggle), `02` (Any key), or `03` (Space key).." =>
    SetTextTransmitMode {mode: TextTransmitMode }
);

define_command_enum!(
    "Text transmit mode." => TextTransmitMode {
        "Enter key." => EnterKey = b'0',
        "^R/^T toggle." => CtrRCtrlTToggle = b'1',
        "Any key." => AnyKey = b'2',
        "Space key." => SpaceKey = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetUsbKeyboardDetectedState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get USB keyboard detected on/off state (`#USB`).

# Command format

> `#USB;`

# Response format

> `#USB{n};`

Where *n* is the boolean state `0` (off) or `1` (on)." =>
    GetUsbKeyboardDetectedState
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBeaconTransmissionInterval => b"#BCI");
impl_cat_command_with_response!(GetBeaconTransmissionInterval => 4, u16_from_ascii => u16);

impl_cat_command!(
    SetBeaconTransmissionInterval => b"#BCI"
    format interval_secs uint 4,
    if |cmd: &SetBeaconTransmissionInterval| {
        if cmd.interval_secs >= 1 && cmd.interval_secs <= 3600 {
            Ok(())
        } else {
            Err(invalid_argument_value("interval_secs", "u32", cmd.interval_secs))
        }
    }
);

impl SetBeaconTransmissionInterval {
    pub const fn in_seconds(interval_secs: u16) -> Self {
        assert!(
            interval_secs >= 1 && interval_secs <= 3600,
            "interval_secs must be between 1 and 3600"
        );
        Self { interval_secs }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBeaconTextMemoryLocation => b"#BCL");
impl_cat_command_with_response!(GetBeaconTextMemoryLocation => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetBeaconTextMemoryLocation => b"#BCL"
    format location uint 2,
    if |cmd: &SetBeaconTextMemoryLocation| {
        if cmd.location <= 50 {
            Ok(())
        } else {
            Err(invalid_argument_value("location", "u8", cmd.location))
        }
    }
);

impl SetBeaconTextMemoryLocation {
    pub const fn to_location(location: u8) -> Self {
        assert!(location <= 50, "location must be between 0 and 50");
        Self { location }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBeaconModeState => b"#BCN");
impl_cat_command_with_response!(GetBeaconModeState => boolean);

impl_cat_command!(SetBeaconModeState => b"#BCN" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetCalibrationSignalState => b"#CAL");
impl_cat_command_with_response!(GetCalibrationSignalState => boolean);

impl_cat_command!(SetCalibrationSignalState => b"#CAL" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDisplayMode => b"#DSM");
impl_cat_command_with_response!(GetDisplayMode => try_from enum DisplayMode);

impl_cat_command!(SetDisplayMode => b"#DSM" for as byte mode);
impl_set_cat_command_from_enum!(
SetDisplayMode, DisplayMode => mode {
    SpectrumOnly => spectrum_only,
    SpectrumAndWaterfall => spectrum_and_waterfall
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFunctionKeyLabelDisplayState => b"#LBL");
impl_cat_command_with_response!(GetFunctionKeyLabelDisplayState => try_from enum FunctionKeyLabelDisplayState);

impl_cat_command!(SetFunctionKeyLabelDisplayState => b"#LBL" for as byte state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(MoveMarkerAFrequency => b"#MAA"  with Some |cmd: &MoveMarkerAFrequency| {
    cmd.step.map(|step| vec![step as u8]).unwrap_or_default()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(MoveMarkerBFrequency => b"#MAB"  with Some |cmd: &MoveMarkerBFrequency| {
    cmd.step.map(|step| vec![step as u8]).unwrap_or_default()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SaveScreenshotToFlashDrive => b"#MSS" );

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetOppositeSideBandNullAmplitude => b"#OSBA");
impl_cat_command_with_response!(GetOppositeSideBandNullAmplitude => 4, i16_from_ascii => i16);

impl_cat_command!(SetOppositeSideBandNullAmplitude => b"#OSBA" format amplitude int 4);

impl SetOppositeSideBandNullAmplitude {
    pub const fn with_amplitude(amplitude: i16) -> Self {
        assert!(
            amplitude >= -9999 && amplitude <= 9999,
            "amplitude must be between -9999 and 9999"
        );
        Self { amplitude }
    }
}
// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetOppositeSideBandNullPhase => b"#OSBP");
impl_cat_command_with_response!(GetOppositeSideBandNullPhase => 4, i16_from_ascii => i16);

impl_cat_command!(SetOppositeSideBandNullPhase => b"#OSBP" format phase int 4);

impl SetOppositeSideBandNullPhase {
    pub const fn with_phase(phase: i16) -> Self {
        assert!(
            phase >= -9999 && phase <= 9999,
            "phase must be between -9999 and 9999"
        );
        Self { phase }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTextHangTime => b"#TXH");
impl_cat_command_with_response!(GetTextHangTime => 4, u32_from_ascii => u32);

impl_cat_command!(SetTextHangTime => b"#TXH" format time_ms uint 5);

impl SetTextHangTime {
    pub const fn in_milliseconds(time_ms: u32) -> Self {
        assert!(time_ms <= 99999, "time_ms must be between 0 and 99999");
        Self { time_ms }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTextTransmitMode
 => b"#TXM");
impl_cat_command_with_response!(GetTextTransmitMode => try_from enum TextTransmitMode);

impl_cat_command!(SetTextTransmitMode => b"#TXM" for as byte mode);

impl_set_cat_command_from_enum!(
    SetTextTransmitMode, TextTransmitMode => mode {
        EnterKey => enter_key,
        CtrRCtrlTToggle => ctr_r_ctrl_t_toggle,
        AnyKey => any_key,
        SpaceKey => space_key
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetUsbKeyboardDetectedState => b"#USB");
impl_cat_command_with_response!(GetUsbKeyboardDetectedState => boolean);
