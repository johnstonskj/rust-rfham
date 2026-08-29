//!
//! CAT commands for the Elecraft P3 panadapter.
//!
//! All P3 commands and responses use a leading `#` prefix, for example `#AVG3;`. The GET form of
//! a command is the bare command letters with no data, e.g. `#AVG;`.
//!
//! Two commands break this convention and are hand-implemented rather than using the usual
//! command macros: [`GetProductId`] (`=`) uses no `#` prefix and no `;` terminator on either the
//! query or the response, and [`BitmapUpload`] (`#BMP`) omits the command-id echo and terminator
//! on its response only.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft P3 Programmer's Reference, rev. A7](https://ftp.elecraft.com/P3/Manuals%20Downloads/P3_Pgmrs_Ref_Rev_A7.pdf), Apr 2016.
//!

use crate::{
    error::{RigError, invalid_response_command_id, invalid_response_length},
    protocol::{
        Frequency,
        cat::{
            Command, CommandWithResponse,
            common::{
                ASCII_DIGIT_ZERO, ASCII_SIGN_NEGATIVE, ASCII_SIGN_POSITIVE, sign_from_ascii_loose,
                string_from_ascii, u8_from_ascii, u16_from_ascii, u32_from_ascii,
                validate_response,
            },
        },
    },
    transport::BaudRate,
};
use strum::EnumIs;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetProductId, ProductId
// ------------------------------------------------------------------------------------------------

define_command!("Query the P3 product identification.

# Command format

> `=`

Unlike every other P3 command, there is no `#` prefix and no `;` terminator.

# Response format

> `P3` or `p3`

`P3` if the main firmware is executing, `p3` if the boot loader is ready to receive new
firmware. As with the query, the response has no `#` prefix and no `;` terminator." =>
    GetProductId
);

/// Parsed P3 product-identification response, as returned by [`GetProductId`].
#[derive(Clone, Debug, PartialEq, Eq, EnumIs)]
pub enum ProductId {
    /// The main firmware is executing (`P3`).
    MainFirmwareExecuting,
    /// The boot loader is ready to receive new firmware (`p3`).
    BootLoaderReady,
}

// ------------------------------------------------------------------------------------------------
// Public Types: GetDisplayAveraging, SetDisplayAveraging
// ------------------------------------------------------------------------------------------------

define_command!("Query the spectrum display averaging time.

# Command format

> `#AVG;`

# Response format

> `#AVGnn;`

Where *nn* is `00` (averaging off) or the averaging time constant, between `02` and `20`
(averaging on)." =>
    GetDisplayAveraging
);

define_command!("Set the spectrum display averaging time.

# Command format

> `#AVGnn;`

Where *nn* is `00` (averaging off) or the averaging time constant, between `02` and `20`
(averaging on)." =>
    SetDisplayAveraging {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: BitmapUpload, BitmapData
// ------------------------------------------------------------------------------------------------

define_command!("Trigger a bitmap screenshot transfer from the P3 display.

# Command format

> `#BMP;`

# Response format

> `[bmp]cc`

Where `[bmp]` is 131,638 bytes of binary image data in standard .BMP file format and `cc` is a
two-byte checksum, the modulo-65,536 sum of all 131,638 image bytes, sent least-significant byte
first.

Unlike other P3 responses, this response does not echo the command id and has no terminating
`;`." =>
    BitmapUpload
);

define_command_struct!(
    "Parsed bitmap-screenshot response." =>
    BitmapData no_copy {
        "The raw .BMP image data, 131,638 bytes." =>
        image_data: Vec<u8>,
        "The modulo-65,536 checksum over `image_data`, as sent least-significant byte first." =>
        checksum: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBaudRate, SetBaudRate
// ------------------------------------------------------------------------------------------------

define_command!("Query the baud rate of the P3's PC-facing RS232 port.

# Command format

> `#BR;`

# Response format

> `#BRn;`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    GetBaudRate
);

define_command!("Set the baud rate of the P3's PC-facing RS232 port.

# Command format

> `#BRn;`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud.

The P3 Utility program automatically sets the P3 to 38400 baud for firmware downloads, then
restores the baud rate to the user's prior selection.

**Note**: The RS232 port that connects to the K3 always runs at 38400 baud regardless of this
setting; this command affects only the P3's PC-facing port, not the K3." =>
    SetBaudRate {
        baud_rate: BaudRate
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCenterFrequency, SetCenterFrequency, CenterFrequency
// ------------------------------------------------------------------------------------------------

define_command!("Query the panadapter center frequency.

# Command format

> `#CTF;`

# Response format

> `#CTFsxxxxxxxxxxx;`

Where *s* is `+`, `-`, or a space (meaning `+`), and *xxxxxxxxxxx* is the center frequency in
Hz." =>
    GetCenterFrequency
);

define_command!("Set the panadapter center frequency.

# Command format

> `#CTFsxxxxxxxxxxx;`

Where *s* is `+` or `-`, and *xxxxxxxxxxx* is the center frequency in Hz.

**Example**: `#CTF+00014060000;` sets the center frequency to 14060 kHz.

**Note**: If the specified frequency is in a different band than the K3 is tuned to, the action
is undefined. A value of zero sets the center frequency to the main VFO frequency of the
transceiver. For transceivers other than the K3, the center frequency is interpreted relative to
the frequency the transceiver is tuned to and may be positive or negative." =>
    SetCenterFrequency {
        center: CenterFrequency
    }
);

define_command_struct!(
    "A parsed signed frequency value, as used by `#CTF`, `#MFA` and `#MFB`." =>
    CenterFrequency {
        "`true` if the value is negative." =>
        is_negative: bool,
        "The magnitude of the frequency, in Hz." =>
        frequency: Frequency
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDisplayMode, SetDisplayMode, DisplayMode
// ------------------------------------------------------------------------------------------------

define_command!("Query the P3 display mode.

# Command format

> `#DSM;`

# Response format

> `#DSMn;`

Where *n* is one of:

* `0`; spectrum only.
* `1`; spectrum + waterfall.
* `2`; spectrum + power meters.
* `3`; spectrum + waterfall + power meters." =>
    GetDisplayMode
);

define_command!("Set the P3 display mode.

# Command format

> `#DSMn;`

Where *n* is one of:

* `0`; spectrum only.
* `1`; spectrum + waterfall.
* `2`; spectrum + power meters.
* `3`; spectrum + waterfall + power meters." =>
    SetDisplayMode {
        mode: DisplayMode
    }
);

define_command_enum!(
    "P3 display mode." => DisplayMode {
        "Spectrum display only." => SpectrumOnly = b'0',
        "Spectrum display plus waterfall." => SpectrumAndWaterfall = b'1',
        "Spectrum display plus power meters." => SpectrumAndPowerMeters = b'2',
        "Spectrum display, waterfall, and power meters." => SpectrumAndWaterfallAndPowerMeters = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFunctionKeyLabel, FunctionKeyLabel
// ------------------------------------------------------------------------------------------------

define_command!("Query a function key's label.

# Command format

> `#FNLn;`

Where *n* is the function key number, `1` to `8`.

# Response format

> `#FNLnccccccccc;`

Where *n* is the function key number and *ccccccccc* are the 9 ASCII characters of the label for
`FN`*n*." =>
    GetFunctionKeyLabel {
        function_key: u8
    }
);

define_command_struct!(
    "A parsed function-key label response." =>
    FunctionKeyLabel no_copy {
        "The function key number, `1` to `8`." =>
        function_key: u8,
        "The 9-character label text." =>
        label: String
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFontSize, SetFontSize, FontSize
// ------------------------------------------------------------------------------------------------

define_command!("Query the display font size.

# Command format

> `#FON;`

# Response format

> `#FONn;`

Where *n* is one of:

* `0`; 5 x 7 pixels.
* `1`; 7 x 11 pixels.
* `2`; 9 x 14 pixels." =>
    GetFontSize
);

define_command!("Set the display font size.

# Command format

> `#FONn;`

Where *n* is one of:

* `0`; 5 x 7 pixels.
* `1`; 7 x 11 pixels.
* `2`; 9 x 14 pixels." =>
    SetFontSize {
        size: FontSize
    }
);

define_command_enum!(
    "P3 main-display font size." => FontSize {
        "5 x 7 pixels." => Small = b'0',
        "7 x 11 pixels." => Medium = b'1',
        "9 x 14 pixels." => Large = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetFunctionKeyExecute
// ------------------------------------------------------------------------------------------------

define_command!("Execute a function key.

# Command format

> `#FNXn;`

Where *n* is the function key number, `1` to `8`, for keys `FN1`-`FN8`. Executes the function
assigned to the key, if any." =>
    SetFunctionKeyExecute {
        function_key: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFixedTuneAutoAdjustMode, SetFixedTuneAutoAdjustMode, FixedTuneAutoAdjustMode
// ------------------------------------------------------------------------------------------------

define_command!("Query the fixed-tune auto-adjust mode.

# Command format

> `#FXA;`

# Response format

> `#FXAn;`

Where *n* is one of:

* `0`; full screen.
* `1`; half screen.
* `2`; slide.
* `3`; static.

This specifies how far the P3 center frequency moves when the K3 VFO A is tuned off screen in
fixed-tune mode." =>
    GetFixedTuneAutoAdjustMode
);

define_command!("Set the fixed-tune auto-adjust mode.

# Command format

> `#FXAn;`

Where *n* is one of:

* `0`; full screen.
* `1`; half screen.
* `2`; slide.
* `3`; static.

This specifies how far the P3 center frequency moves when the K3 VFO A is tuned off screen in
fixed-tune mode." =>
    SetFixedTuneAutoAdjustMode {
        mode: FixedTuneAutoAdjustMode
    }
);

define_command_enum!(
    "Fixed-tune auto-adjust behavior." => FixedTuneAutoAdjustMode {
        "Full screen." => FullScreen = b'0',
        "Half screen." => HalfScreen = b'1',
        "Slide." => Slide = b'2',
        "Static." => Static = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFixedTuneOrTrackingMode, SetFixedTuneOrTrackingMode, FixedTuneOrTrackingMode
// ------------------------------------------------------------------------------------------------

define_command!("Query the fixed-tune or tracking select mode.

# Command format

> `#FXT;`

# Response format

> `#FXTn;`

Where *n* is one of:

* `0`; tracking mode.
* `1`; fixed-tune mode." =>
    GetFixedTuneOrTrackingMode
);

define_command!("Set the fixed-tune or tracking select mode.

# Command format

> `#FXTn;`

Where *n* is one of:

* `0`; tracking mode.
* `1`; fixed-tune mode." =>
    SetFixedTuneOrTrackingMode {
        mode: FixedTuneOrTrackingMode
    }
);

define_command_enum!(
    "Fixed-tune or tracking mode." => FixedTuneOrTrackingMode {
        "Tracking mode." => Tracking = b'0',
        "Fixed-tune mode." => FixedTune = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFnLabelDisplay, SetFnLabelDisplay
// ------------------------------------------------------------------------------------------------

define_command!("Query whether FN key labels are shown on the display.

# Command format

> `#LBL;`

# Response format

> `#LBLn;`

Where `n` is the boolean state `0` (FN key labels off) or `1` (FN key labels on)." =>
    GetFnLabelDisplay
);

define_command!("Set whether FN key labels are shown on the display.

# Command format

> `#LBLn;`

Where `n` is the boolean state `0` (FN key labels off) or `1` (FN key labels on)." =>
    SetFnLabelDisplay {
        labels_on: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMarkerAFrequency, SetMarkerAFrequency, MarkerFrequency
// ------------------------------------------------------------------------------------------------

define_command!("Query the marker A frequency.

# Command format

> `#MFA;`

# Response format

> `#MFAsxxxxxxxxxxx;`

Where *s* is `+`, `-`, or a space (meaning `+`), and *xxxxxxxxxxx* is the marker frequency in
Hz." =>
    GetMarkerAFrequency
);

define_command!("Set the marker A frequency.

# Command format

> `#MFAsxxxxxxxxxxx;`

Where *s* is `+` or `-`, and *xxxxxxxxxxx* is the marker frequency in Hz.

**Example**: `#MFA+00014060000;` sets the marker A frequency to 14060 kHz.

**Note**: If the specified frequency is in a different band than the K3 is tuned to, the action
is undefined. A value of zero sets the marker to the main VFO frequency of the transceiver. For
transceivers other than the K3, the marker frequency is interpreted relative to the frequency
the transceiver is tuned to and may be positive or negative." =>
    SetMarkerAFrequency {
        marker: MarkerFrequency
    }
);

define_command_struct!(
    "A parsed signed marker-frequency value, as used by `#MFA` and `#MFB`." =>
    MarkerFrequency {
        "`true` if the value is negative." =>
        is_negative: bool,
        "The magnitude of the frequency, in Hz." =>
        frequency: Frequency
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMarkerBFrequency, SetMarkerBFrequency
// ------------------------------------------------------------------------------------------------

define_command!("Query the marker B frequency.

# Command format

> `#MFB;`

# Response format

> `#MFBsxxxxxxxxxxx;`

Where *s* is `+`, `-`, or a space (meaning `+`), and *xxxxxxxxxxx* is the marker frequency in
Hz." =>
    GetMarkerBFrequency
);

define_command!("Set the marker B frequency.

# Command format

> `#MFBsxxxxxxxxxxx;`

Where *s* is `+` or `-`, and *xxxxxxxxxxx* is the marker frequency in Hz.

**Example**: `#MFB+00014060000;` sets the marker B frequency to 14060 kHz.

**Note**: If the specified frequency is in a different band than the K3 is tuned to, the action
is undefined. A value of zero sets the marker to the main VFO frequency of the transceiver. For
transceivers other than the K3, the marker frequency is interpreted relative to the frequency
the transceiver is tuned to and may be positive or negative." =>
    SetMarkerBFrequency {
        marker: MarkerFrequency
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMarkerAState, SetMarkerAState
// ------------------------------------------------------------------------------------------------

define_command!("Query whether marker A is on/off.

# Command format

> `#MKA;`

# Response format

> `#MKAn;`

Where `n` is the boolean state `0` (marker off) or `1` (marker on)." =>
    GetMarkerAState
);

define_command!("Set marker A on/off.

# Command format

> `#MKAn;`

Where `n` is the boolean state `0` (marker off) or `1` (marker on).

**Note**: The last marker to be turned on automatically becomes the active marker, meaning it
can be adjusted with the knob and is the one that responds to the QSY command. If the marker was
off-screen before executing a marker-on command, it will default to the center frequency." =>
    SetMarkerAState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMarkerBState, SetMarkerBState
// ------------------------------------------------------------------------------------------------

define_command!("Query whether marker B is on/off.

# Command format

> `#MKB;`

# Response format

> `#MKBn;`

Where `n` is the boolean state `0` (marker off) or `1` (marker on)." =>
    GetMarkerBState
);

define_command!("Set marker B on/off.

# Command format

> `#MKBn;`

Where `n` is the boolean state `0` (marker off) or `1` (marker on).

**Note**: The last marker to be turned on automatically becomes the active marker, meaning it
can be adjusted with the knob and is the one that responds to the QSY command. If the marker was
off-screen before executing a marker-on command, it will default to the center frequency." =>
    SetMarkerBState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetNoiseBlankerState, SetNoiseBlankerState
// ------------------------------------------------------------------------------------------------

define_command!("Query the noise-blanker on/off status.

# Command format

> `#NB;`

# Response format

> `#NBn;`

Where `n` is the boolean state `0` (noise blanker off) or `1` (noise blanker on)." =>
    GetNoiseBlankerState
);

define_command!("Set the noise-blanker on/off status.

# Command format

> `#NBn;`

Where `n` is the boolean state `0` (noise blanker off) or `1` (noise blanker on)." =>
    SetNoiseBlankerState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetNoiseBlankerLevel, SetNoiseBlankerLevel
// ------------------------------------------------------------------------------------------------

define_command!("Query the noise-blanker level.

# Command format

> `#NBL;`

# Response format

> `#NBLnn;`

Where *nn* is the aggressiveness of the noise-blanker algorithm, between `01` (least aggressive)
and `15` (most aggressive)." =>
    GetNoiseBlankerLevel
);

define_command!("Set the noise-blanker level.

# Command format

> `#NBLnn;`

Where *nn* is the aggressiveness of the noise-blanker algorithm, between `01` (least aggressive)
and `15` (most aggressive)." =>
    SetNoiseBlankerLevel {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPeakModeState, SetPeakModeState
// ------------------------------------------------------------------------------------------------

define_command!("Query the peak mode.

# Command format

> `#PKM;`

# Response format

> `#PKMn;`

Where `n` is the boolean state `0` (peak mode off) or `1` (peak mode on)." =>
    GetPeakModeState
);

define_command!("Set the peak mode.

# Command format

> `#PKMn;`

Where `n` is the boolean state `0` (peak mode off) or `1` (peak mode on)." =>
    SetPeakModeState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatus, SetPowerStatus
// ------------------------------------------------------------------------------------------------

define_command!("Query the power status.

# Command format

> `#PS;`

# Response format

> `#PSn;`

Where `n` = `1` indicates the P3 is on." =>
    GetPowerStatus
);

define_command!("Set the power status.

# Command format

> `#PSn;`

Where `n` = `1` indicates the P3 is on.

**Note**: `#PS0` turns the P3 off, but this removes power so `#PS1` cannot be used to turn it
back on. If the power-on jumper on the rear-panel I/O board is in the 'always on' position, then
the `#PS0` command has no effect." =>
    SetPowerStatus { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetPassThroughModeState
// ------------------------------------------------------------------------------------------------

define_command!("Enter serial pass-through mode.

# Command format

> `#PT;`

This command takes no argument. Once received, panadapter operation ceases and all data
received on either RS232 port is passed through immediately to the other RS232 port without
delay or modification.

**Note**: This command is used by P3 Utility when downloading new firmware to the K3
transceiver. Pass-through mode ends automatically 8 seconds after the last RS232 activity." =>
    SetPassThroughModeState
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetQsyToMarker, QsyAction
// ------------------------------------------------------------------------------------------------

define_command!("Command the transceiver to QSY to the active marker frequency.

# Command format

> `#QSYn;`

Where *n* is one of:

* `1`; QSY.
* `0`; undo QSY.

'QSY' means the currently-active marker frequency is transferred to the associated VFO on the
K3.

**Note**: MKR A controls VFO A and MKR B controls VFO B. 'Undo QSY' means to return the VFO to
the frequency it was on before the last QSY, a one-level undo command." =>
    SetQsyToMarker {
        action: QsyAction
    }
);

define_command_enum!(
    "QSY / undo-QSY action." => QsyAction {
        "Transfer the active marker frequency to its associated VFO." => Qsy = b'1',
        "Undo the last QSY, restoring the VFO's previous frequency." => UndoQsy = b'0'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRelativeCenterFrequency, SetRelativeCenterFrequency, RelativeCenterFrequencyOffset
// ------------------------------------------------------------------------------------------------

define_command!("Query the reference frequency calibration offset.

# Command format

> `#RCF;`

# Response format

> `#RCFsnnnnnn;`

Where *s* is `+` or `-` and *nnnnnn* is the difference, in Hz, between the current center
frequency and the VFO A frequency." =>
    GetRelativeCenterFrequency
);

define_command!("Set the reference frequency calibration offset.

# Command format

> `#RCFsnnnnnn;`

Where *s* is `+` or `-` and *nnnnnn* is the offset in Hz which, when added to the VFO A
frequency, becomes the new center frequency.

**Note**: This command is used to position the VFO A cursor on the screen. For example, if the
current span is set to 50 kHz, `#RCF+025000;` will move the VFO A cursor to the left edge of the
screen (the center frequency moves up 25 kHz, which shifts the VFO A cursor to the left)." =>
    SetRelativeCenterFrequency {
        offset: RelativeCenterFrequencyOffset
    }
);

define_command_struct!(
    "A parsed signed relative center-frequency offset, as used by `#RCF`." =>
    RelativeCenterFrequencyOffset {
        "`true` if the value is negative." =>
        is_negative: bool,
        "The magnitude of the offset, in Hz." =>
        offset_hz: u32
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetReferenceLevel, SetReferenceLevel
// ------------------------------------------------------------------------------------------------

define_command!("Query the spectrum display reference level.

# Command format

> `#REF;`

# Response format

> `#REFsnnn;`

Where *s* is `+`, `-`, or a space (meaning `+`), and *nnn* is the reference level in dBm,
between `-170` and `+010`." =>
    GetReferenceLevel
);

define_command!("Set the spectrum display reference level.

# Command format

> `#REFsnnn;`

Where *s* is `+` or `-`, and *nnn* is the reference level in dBm, between `-170` and `+010`.

**Example**: `#REF-120;` sets the reference level (at the bottom of the P3 spectrum screen) to
-120 dBm." =>
    SetReferenceLevel {
        dbm: i16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: Reset
// ------------------------------------------------------------------------------------------------

define_command!("Force a power-on reset.

# Command format

> `#RST;`

There is no response to this command." =>
    Reset
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFpgaImageFirmwareRevision, FpgaImageFirmwareRevision, FirmwareRevision
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA board's FPGA image revision.

# Command format

> `#RVF;`

# Response format

> `#RVFnnNN.NN;`

Where *nn* is the FPGA image number, `00` to `05`, and *NN.NN* is the image revision, e.g.
`01.23`.

**Note**: Returns `99.99` if no FPGA image is installed." =>
    GetFpgaImageFirmwareRevision
);

define_command_struct!(
    "A parsed FPGA image firmware-revision response." =>
    FpgaImageFirmwareRevision {
        "The FPGA image number, `0` to `5`." =>
        image_number: u8,
        "The image revision, or `None` if no FPGA image is installed." =>
        revision: Option<FirmwareRevision>
    }
);

define_command_struct!(
    "A parsed `major.minor` firmware revision, e.g. `01.23`." =>
    FirmwareRevision {
        major: u8,
        minor: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMainFirmware
// ------------------------------------------------------------------------------------------------

define_command!("Query the main firmware revision.

# Command format

> `#RVM;`

# Response format

> `#RVMNN.NN;`

Where *NN.NN* is the firmware revision, e.g. `01.23`." =>
    GetMainFirmware
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaFirmwareRevision, SvgaFirmwareRevision
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA board firmware revision.

# Command format

> `#RVS;`

# Response format

> `#RVSNN.NN;`

Where *NN.NN* is the firmware revision, e.g. `01.23`.

**Note**: Returns `99.99` if no SVGA firmware is installed, and `00.00` if only the SVGA boot
loader is installed." =>
    GetSvgaFirmwareRevision
);

/// SVGA daughter-board firmware revision state, as returned by [`GetSvgaFirmwareRevision`].
///
/// The `Installed` variant carries a [`FirmwareRevision`] payload, which `define_command_enum!`
/// cannot express (it only supports plain C-like enums with byte-literal discriminants), so this
/// type is hand-written instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIs)]
pub enum SvgaFirmwareRevision {
    /// No SVGA firmware is installed (`99.99`).
    NotInstalled,
    /// Only the SVGA boot loader is installed (`00.00`).
    BootLoaderOnly,
    /// SVGA firmware is installed, with this revision.
    Installed(FirmwareRevision),
}

// ------------------------------------------------------------------------------------------------
// Public Types: GetScale, SetScale
// ------------------------------------------------------------------------------------------------

define_command!("Query the current display scale.

# Command format

> `#SCL;`

# Response format

> `#SCLnnn;`

Where *nnn* is the scale, the difference in dB between the top and bottom of the spectrum
screen, between `010` and `080` dB." =>
    GetScale
);

define_command!("Set the current display scale.

# Command format

> `#SCLnnn;`

Where *nnn* is the scale, the difference in dB between the top and bottom of the spectrum
screen, between `010` and `080` dB.

**Example**: `#SCL080;` sets the scale to 80 dB." =>
    SetScale {
        db: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSpanMode, SetSpanMode, SpanMode
// ------------------------------------------------------------------------------------------------

define_command!("Query the span mode.

# Command format

> `#SPM;`

# Response format

> `#SPMn;`

Where *n* is one of:

* `0`; continuous span mode.
* `1`; stepped span mode.

In stepped span mode, the span steps between 2, 5, 10, 20, 50, 100 and 200 kHz." =>
    GetSpanMode
);

define_command!("Set the span mode.

# Command format

> `#SPMn;`

Where *n* is one of:

* `0`; continuous span mode.
* `1`; stepped span mode.

In stepped span mode, the span steps between 2, 5, 10, 20, 50, 100 and 200 kHz." =>
    SetSpanMode {
        mode: SpanMode
    }
);

define_command_enum!(
    "Panadapter span-stepping mode." => SpanMode {
        "Continuous span mode." => Continuous = b'0',
        "Stepped span mode (2, 5, 10, 20, 50, 100, 200 kHz)." => Stepped = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSpan, SetSpan
// ------------------------------------------------------------------------------------------------

define_command!("Query the panadapter span.

# Command format

> `#SPN;`

# Response format

> `#SPNxxxxxx;`

Where *xxxxxx* is the span, in 100 Hz units, between `000020` and `002000`.

**Example**: `#SPN000500;` is a span of 50 kHz." =>
    GetSpan
);

define_command!("Set the panadapter span.

# Command format

> `#SPNxxxxxx;`

Where *xxxxxx* is the span, in 100 Hz units, between `000020` and `002000`.

**Example**: `#SPN000500;` sets the span to 50 kHz." =>
    SetSpan {
        span_hundred_hz: u32
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaDecodedDataDisplayState, SetSvgaDecodedDataDisplayState
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA decoded-data display state.

# Command format

> `#SVDT;`

# Response format

> `#SVDTn;`

Where `n` is the boolean state `0` (data display off) or `1` (data display on)." =>
    GetSvgaDecodedDataDisplayState
);

define_command!("Set the SVGA decoded-data display state.

# Command format

> `#SVDTn;`

Where `n` is the boolean state `0` (data display off) or `1` (data display on)." =>
    SetSvgaDecodedDataDisplayState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaDisplayState, SetSvgaDisplayState
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA display state.

# Command format

> `#SVEN;`

# Response format

> `#SVENn;`

Where `n` is the boolean state `0` (SVGA display off) or `1` (SVGA display on)." =>
    GetSvgaDisplayState
);

define_command!("Set the SVGA display state.

# Command format

> `#SVENn;`

Where `n` is the boolean state `0` (SVGA display off) or `1` (SVGA display on)." =>
    SetSvgaDisplayState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaSpectrumFillState, SetSvgaSpectrumFillState
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA spectrum-fill state.

# Command format

> `#SVFL;`

# Response format

> `#SVFLn;`

Where `n` is the boolean state `0` (fill off) or `1` (fill on). When on, the area below the
spectrum trace on the external SVGA display is filled in for easier visibility." =>
    GetSvgaSpectrumFillState
);

define_command!("Set the SVGA spectrum-fill state.

# Command format

> `#SVFLn;`

Where `n` is the boolean state `0` (fill off) or `1` (fill on). When on, the area below the
spectrum trace on the external SVGA display is filled in for easier visibility." =>
    SetSvgaSpectrumFillState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaFontSize, SetSvgaFontSize, SvgaFontSize
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA font selection.

# Command format

> `#SVFN;`

# Response format

> `#SVFNn;`

Where *n* is the font number, `0` to `3`. The larger the number, the larger the font." =>
    GetSvgaFontSize
);

define_command!("Set the SVGA font selection.

# Command format

> `#SVFNn;`

Where *n* is the font number, `0` to `3`. The larger the number, the larger the font." =>
    SetSvgaFontSize {
        size: SvgaFontSize
    }
);

define_command_enum!(
    "SVGA display font size." => SvgaFontSize {
        Small = b'0',
        Medium = b'1',
        Large = b'2',
        Larger = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaDisplayResolution, SetSvgaDisplayResolution, SvgaDisplayResolution
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA display resolution.

# Command format

> `#SVRS;`

# Response format

> `#SVRSn;`

Where *n* is the external display resolution, `0` to `4`. See the manual for the SVGA option for
more details." =>
    GetSvgaDisplayResolution
);

define_command!("Set the SVGA display resolution.

# Command format

> `#SVRSn;`

Where *n* is the external display resolution, `0` to `4`. See the manual for the SVGA option for
more details." =>
    SetSvgaDisplayResolution {
        resolution: SvgaDisplayResolution
    }
);

define_command_enum!(
    "SVGA external display resolution." => SvgaDisplayResolution {
        "XGA: 1024x768." => Xga = b'0',
        "SXGA: 1280x1024." => Sxga = b'1',
        "WXGA+: 1440x900." => WxgaPlus = b'2',
        "(F)HD: 1920x1080." => FHd = b'3',
        "(F)HD: 1920x1080, alternate clock rate." => FHdAlt = b'4'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSvgaWaterfallBias, SetSvgaWaterfallBias
// ------------------------------------------------------------------------------------------------

define_command!("Query the SVGA waterfall bias.

# Command format

> `#SVWB;`

# Response format

> `#SVWBnn;`

Where *nn* is the bias, between `01` and `99`, corresponding to 0.1 to 9.9 in the P3's 'SVGA
bias' menu entry. The higher the number, the greater the color contrast in the external display
waterfall; a value of `10` (1.0) looks similar to the P3's own screen on a typical monitor." =>
    GetSvgaWaterfallBias
);

define_command!("Set the SVGA waterfall bias.

# Command format

> `#SVWBnn;`

Where *nn* is the bias, between `01` and `99`, corresponding to 0.1 to 9.9 in the P3's 'SVGA
bias' menu entry. The higher the number, the greater the color contrast in the external display
waterfall; a value of `10` (1.0) looks similar to the P3's own screen on a typical monitor." =>
    SetSvgaWaterfallBias {
        bias: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoBCursorState, SetVfoBCursorState
// ------------------------------------------------------------------------------------------------

define_command!("Query the VFO B cursor on/off state.

# Command format

> `#VFB;`

# Response format

> `#VFBn;`

Where `n` is the boolean state `0` (VFO B cursor off) or `1` (VFO B cursor on)." =>
    GetVfoBCursorState
);

define_command!("Set the VFO B cursor on/off state.

# Command format

> `#VFBn;`

Where `n` is the boolean state `0` (VFO B cursor off) or `1` (VFO B cursor on)." =>
    SetVfoBCursorState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetWaterfallAveragingState, SetWaterfallAveragingState
// ------------------------------------------------------------------------------------------------

define_command!("Query the waterfall averaging on/off state.

# Command format

> `#WFA;`

# Response format

> `#WFAn;`

Where `n` is the boolean state `0` (waterfall averaging off) or `1` (waterfall averaging on)." =>
    GetWaterfallAveragingState
);

define_command!("Set the waterfall averaging on/off state.

# Command format

> `#WFAn;`

Where `n` is the boolean state `0` (waterfall averaging off) or `1` (waterfall averaging on)." =>
    SetWaterfallAveragingState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetWaterfallColor, SetWaterfallColor, WaterfallColor
// ------------------------------------------------------------------------------------------------

define_command!("Query the waterfall color.

# Command format

> `#WFC;`

# Response format

> `#WFCn;`

Where *n* is one of:

* `0`; gray scale waterfall.
* `1`; colored waterfall." =>
    GetWaterfallColor
);

define_command!("Set the waterfall color.

# Command format

> `#WFCn;`

Where *n* is one of:

* `0`; gray scale waterfall.
* `1`; colored waterfall." =>
    SetWaterfallColor {
        color: WaterfallColor
    }
);

define_command_enum!(
    "Waterfall color mode." => WaterfallColor {
        "Display in gray-scale only." => GrayScale = b'0',
        "Display in color." => Colored = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetWaterfallMarkersState, SetWaterfallMarkersState
// ------------------------------------------------------------------------------------------------

define_command!("Query the waterfall markers on/off state.

# Command format

> `#WFM;`

# Response format

> `#WFMn;`

Where `n` is the boolean state `0` (waterfall markers off) or `1` (waterfall markers on)." =>
    GetWaterfallMarkersState
);

define_command!("Set the waterfall markers on/off state.

# Command format

> `#WFMn;`

Where `n` is the boolean state `0` (waterfall markers off) or `1` (waterfall markers on)." =>
    SetWaterfallMarkersState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverConnected, SetTransceiverConnected
// ------------------------------------------------------------------------------------------------

define_command!("Query the selected transceiver.

# Command format

> `#XCV;`

# Response format

> `#XCVnn;`

Where *nn* is `00` (K3), `01` (user-defined transceiver), `02` (455 kHz IF), etc. up to the last
'transceiver' in the 'Xcvr Sel' menu selection." =>
    GetTransceiverConnected
);

define_command!("Set the selected transceiver.

# Command format

> `#XCVnn;`

Where *nn* is `00` (K3), `01` (user-defined transceiver), `02` (455 kHz IF), etc. up to the last
'transceiver' in the 'Xcvr Sel' menu selection." =>
    SetTransceiverConnected {
        transceiver: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

/// Format a signed [`Frequency`] as a P3 `s` + 11-digit ASCII value, as used by `#CTF`, `#MFA`
/// and `#MFB`.
fn format_signed_frequency(is_negative: bool, frequency: Frequency) -> Vec<u8> {
    let mut bytes = vec![if is_negative {
        ASCII_SIGN_NEGATIVE
    } else {
        ASCII_SIGN_POSITIVE
    }];
    bytes.extend(Vec::<u8>::from(frequency));
    bytes
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// `GetProductId` uses non-standard framing (no `#` prefix, no `;` terminator on either the query
// or the response), so `Command` is hand-rolled instead of using `impl_command!`.
impl Command for GetProductId {
    const MESSAGE_TERMINATOR: u8 = b';';

    fn command_id(&self) -> &[u8] {
        b"="
    }

    fn message_preamble(&self) -> Option<&[u8]> {
        None
    }

    fn to_message(&self) -> Result<Vec<u8>, RigError> {
        Ok(self.command_id().to_vec())
    }
}

impl CommandWithResponse for GetProductId {
    type Response = ProductId;

    fn parse(&self, bytes: &[u8]) -> Result<ProductId, RigError> {
        if bytes == b"P3" {
            Ok(ProductId::MainFirmwareExecuting)
        } else if bytes == b"p3" {
            Ok(ProductId::BootLoaderReady)
        } else {
            Err(invalid_response_command_id(b"[Pp]3"))
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetDisplayAveraging => b"#AVG");
impl_command_with_response!(GetDisplayAveraging => 2, u8_from_ascii => u8);

impl_command!(SetDisplayAveraging => b"#AVG" format level uint 2);

// ------------------------------------------------------------------------------------------------

impl_command!(BitmapUpload => b"#BMP");

// The `#BMP` response omits the command-id echo and trailing `;` terminator that
// `validate_response` (and therefore `impl_command_with_response!`) assumes, so response parsing
// is hand-rolled instead.
impl CommandWithResponse for BitmapUpload {
    type Response = BitmapData;

    fn expected_response_length(&self) -> usize {
        131_640
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
        if bytes.len() != self.expected_response_length() {
            Err(invalid_response_length(
                self.expected_response_length(),
                bytes.len(),
            ))
        } else {
            Ok(BitmapData {
                image_data: bytes[..131_638].to_vec(),
                checksum: u16::from_le_bytes(bytes[131_638..].try_into().unwrap()),
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetBaudRate => b"#BR");
impl_command_with_response!(GetBaudRate => 1, |bytes: &[u8]| {
    match bytes[0] {
        b'0' => Ok(BaudRate::Bd4800),
        b'1' => Ok(BaudRate::Bd9600),
        b'2' => Ok(BaudRate::Bd19200),
        b'3' => Ok(BaudRate::Bd38400),
        _ => Err(invalid_response_command_id(b"[0-3]"))
    }
} => BaudRate);

impl_command!(SetBaudRate => b"#BR" with |s: &SetBaudRate| {
    match s.baud_rate {
        BaudRate::Bd4800 => Ok(Some(vec![b'0'])),
        BaudRate::Bd9600 => Ok(Some(vec![b'1'])),
        BaudRate::Bd19200 => Ok(Some(vec![b'2'])),
        BaudRate::Bd38400 => Ok(Some(vec![b'3'])),
        _ => Err(RigError::InvalidArgumentValue {
            argument_name: "baud_rate",
            type_name: "BaudRate",
            value: format!("{:?}", s.baud_rate)
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetCenterFrequency => b"#CTF");
impl_command_with_response!(GetCenterFrequency => 12, |bytes: &[u8]| {
    Ok(CenterFrequency {
        is_negative: sign_from_ascii_loose(bytes[0])? < 0,
        frequency: Frequency::try_from(&bytes[1..])?
    })
} => CenterFrequency);

impl_command!(SetCenterFrequency => b"#CTF" with Some |s: &SetCenterFrequency| {
    format_signed_frequency(s.center.is_negative, s.center.frequency)
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetDisplayMode => b"#DSM");
impl_command_with_response!(GetDisplayMode => try_from enum DisplayMode);

impl_command!(SetDisplayMode => b"#DSM" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFunctionKeyLabel => b"#FNL" with |s: &GetFunctionKeyLabel| {
    if (1..=8).contains(&s.function_key) {
        Ok(Some(vec![s.function_key + ASCII_DIGIT_ZERO]))
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "function_key",
            type_name: "u8",
            value: s.function_key.to_string()
        })
    }
});
impl_command_with_response!(GetFunctionKeyLabel => 10, |bytes: &[u8]| {
    Ok(FunctionKeyLabel {
        function_key: u8_from_ascii(&bytes[0..1])?,
        label: string_from_ascii(&bytes[1..])?,
    })
} => FunctionKeyLabel);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFontSize => b"#FON");
impl_command_with_response!(GetFontSize => try_from enum FontSize);

impl_command!(SetFontSize => b"#FON" for as byte size);

// ------------------------------------------------------------------------------------------------

impl_command!(SetFunctionKeyExecute => b"#FNX" with |s: &SetFunctionKeyExecute| {
    if (1..=8).contains(&s.function_key) {
        Ok(Some(vec![s.function_key + ASCII_DIGIT_ZERO]))
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "function_key",
            type_name: "u8",
            value: s.function_key.to_string()
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetFixedTuneAutoAdjustMode => b"#FXA");
impl_command_with_response!(GetFixedTuneAutoAdjustMode => try_from enum FixedTuneAutoAdjustMode);

impl_command!(SetFixedTuneAutoAdjustMode => b"#FXA" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFixedTuneOrTrackingMode => b"#FXT");
impl_command_with_response!(GetFixedTuneOrTrackingMode => try_from enum FixedTuneOrTrackingMode);

impl_command!(SetFixedTuneOrTrackingMode => b"#FXT" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFnLabelDisplay => b"#LBL");
impl_command_with_response!(GetFnLabelDisplay => boolean);

impl_command!(SetFnLabelDisplay => b"#LBL" for boolean labels_on);

// ------------------------------------------------------------------------------------------------

impl_command!(GetMarkerAFrequency => b"#MFA");
impl_command_with_response!(GetMarkerAFrequency => 12, |bytes: &[u8]| {
    Ok(MarkerFrequency {
        is_negative: sign_from_ascii_loose(bytes[0])? < 0,
        frequency: Frequency::try_from(&bytes[1..])?
    })
} => MarkerFrequency);

impl_command!(SetMarkerAFrequency => b"#MFA" with Some |s: &SetMarkerAFrequency| {
    format_signed_frequency(s.marker.is_negative, s.marker.frequency)
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetMarkerBFrequency => b"#MFB");
impl_command_with_response!(GetMarkerBFrequency => 12, |bytes: &[u8]| {
    Ok(MarkerFrequency {
        is_negative: sign_from_ascii_loose(bytes[0])? < 0,
        frequency: Frequency::try_from(&bytes[1..])?
    })
} => MarkerFrequency);

impl_command!(SetMarkerBFrequency => b"#MFB" with Some |s: &SetMarkerBFrequency| {
    format_signed_frequency(s.marker.is_negative, s.marker.frequency)
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetMarkerAState => b"#MKA");
impl_command_with_response!(GetMarkerAState => boolean);

impl_command!(SetMarkerAState => b"#MKA" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetMarkerBState => b"#MKB");
impl_command_with_response!(GetMarkerBState => boolean);

impl_command!(SetMarkerBState => b"#MKB" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetNoiseBlankerState => b"#NB");
impl_command_with_response!(GetNoiseBlankerState => boolean);

impl_command!(SetNoiseBlankerState => b"#NB" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetNoiseBlankerLevel => b"#NBL");
impl_command_with_response!(GetNoiseBlankerLevel => 2, u8_from_ascii => u8);

impl_command!(
    SetNoiseBlankerLevel => b"#NBL"
    format level uint 2,
    if |s: &SetNoiseBlankerLevel| {
        if (1..=15).contains(&s.level) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "level",
                type_name: "u8",
                value: s.level.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPeakModeState => b"#PKM");
impl_command_with_response!(GetPeakModeState => boolean);

impl_command!(SetPeakModeState => b"#PKM" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerStatus => b"#PS");
impl_command_with_response!(GetPowerStatus => boolean);

impl_command!(SetPowerStatus => b"#PS" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(SetPassThroughModeState => b"#PT");

// ------------------------------------------------------------------------------------------------

impl_command!(SetQsyToMarker => b"#QSY" for as byte action);

// ------------------------------------------------------------------------------------------------

impl_command!(GetRelativeCenterFrequency => b"#RCF");
impl_command_with_response!(GetRelativeCenterFrequency => 7, |bytes: &[u8]| {
    Ok(RelativeCenterFrequencyOffset {
        is_negative: sign_from_ascii_loose(bytes[0])? < 0,
        offset_hz: u32_from_ascii(&bytes[1..])?,
    })
} => RelativeCenterFrequencyOffset);

impl_command!(SetRelativeCenterFrequency => b"#RCF" with Some |s: &SetRelativeCenterFrequency| {
    let mut v = vec![if s.offset.is_negative { ASCII_SIGN_NEGATIVE } else { ASCII_SIGN_POSITIVE }];
    v.extend(format!("{:06}", s.offset.offset_hz).into_bytes());
    v
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetReferenceLevel => b"#REF");
impl_command_with_response!(GetReferenceLevel => 4, |bytes: &[u8]| {
    let sign = sign_from_ascii_loose(bytes[0])? as i16;
    let magnitude = u16_from_ascii(&bytes[1..4])? as i16;
    Ok(sign * magnitude)
} => i16);

impl_command!(SetReferenceLevel => b"#REF" with Some |s: &SetReferenceLevel| {
    let mut v = vec![if s.dbm.is_negative() { ASCII_SIGN_NEGATIVE } else { ASCII_SIGN_POSITIVE }];
    v.extend(format!("{:03}", s.dbm.unsigned_abs()).into_bytes());
    v
}, if |s: &SetReferenceLevel| {
    if (-170..=10).contains(&s.dbm) {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "dbm",
            type_name: "i16",
            value: s.dbm.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(Reset => b"#RST");

// ------------------------------------------------------------------------------------------------

impl_command!(GetFpgaImageFirmwareRevision => b"#RVF");
impl_command_with_response!(GetFpgaImageFirmwareRevision => 7, |bytes: &[u8]| {
    if bytes[4] == b'.' {
        Ok(FpgaImageFirmwareRevision {
            image_number: u8_from_ascii(&bytes[0..=1])?,
            revision: if &bytes[2..] == b"99.99" {
                None
            } else {
                Some(FirmwareRevision {
                    major: u8_from_ascii(&bytes[2..=3])?,
                    minor: u8_from_ascii(&bytes[5..=6])?,
                })
            }
        })
    } else {
        Err(RigError::InvalidResponseData {
            data: bytes.to_vec(),
        })
    }
} => FpgaImageFirmwareRevision);

// ------------------------------------------------------------------------------------------------

impl_command!(GetMainFirmware => b"#RVM");
impl_command_with_response!(GetMainFirmware => 5, |bytes: &[u8]| {
    if bytes[2] == b'.' {
        Ok(FirmwareRevision {
            major: u8_from_ascii(&bytes[0..=1])?,
            minor: u8_from_ascii(&bytes[3..=4])?,
        })
    } else {
        Err(RigError::InvalidResponseData {
            data: bytes.to_vec(),
        })
    }
} => FirmwareRevision);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaFirmwareRevision => b"#RVS");
impl_command_with_response!(GetSvgaFirmwareRevision => 5, |bytes: &[u8]| {
    if bytes[2] == b'.' {
        if &bytes[2..] == b"99.99" {
            Ok(SvgaFirmwareRevision::NotInstalled)
        } else if &bytes[2..] == b"00.00" {
            Ok(SvgaFirmwareRevision::BootLoaderOnly)
        } else {
            Ok(SvgaFirmwareRevision::Installed(
                FirmwareRevision {
                    major: u8_from_ascii(&bytes[0..=1])?,
                    minor: u8_from_ascii(&bytes[3..=4])?,
                }
            ))
        }
    } else {
        Err(RigError::InvalidResponseData {
            data: bytes.to_vec(),
        })
    }
} => SvgaFirmwareRevision);

// ------------------------------------------------------------------------------------------------

impl_command!(GetScale => b"#SCL");
impl_command_with_response!(GetScale => 3, u16_from_ascii => u16);

impl_command!(SetScale => b"#SCL" with Some |s: &SetScale| {
    format!("{:03}", s.db).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetSpanMode => b"#SPM");
impl_command_with_response!(GetSpanMode => try_from enum SpanMode);

impl_command!(SetSpanMode => b"#SPM" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSpan => b"#SPN");
impl_command_with_response!(GetSpan => 6, u32_from_ascii => u32);

impl_command!(SetSpan => b"#SPN" with Some |s: &SetSpan| {
    format!("{:06}", s.span_hundred_hz).into_bytes()
});
impl_command_with_response!(SetSpan => 6, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaDecodedDataDisplayState => b"#SVDT");
impl_command_with_response!(GetSvgaDecodedDataDisplayState => boolean);

impl_command!(SetSvgaDecodedDataDisplayState => b"#SVDT" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaDisplayState => b"#SVEN");
impl_command_with_response!(GetSvgaDisplayState => boolean);

impl_command!(SetSvgaDisplayState => b"#SVEN" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaSpectrumFillState => b"#SVFL");
impl_command_with_response!(GetSvgaSpectrumFillState => boolean);

impl_command!(SetSvgaSpectrumFillState => b"#SVFL" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaFontSize => b"#SVFN");
impl_command_with_response!(GetSvgaFontSize => try_from enum SvgaFontSize);

impl_command!(SetSvgaFontSize => b"#SVFN" for as byte size);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaDisplayResolution => b"#SVRS");
impl_command_with_response!(GetSvgaDisplayResolution => try_from enum SvgaDisplayResolution);

impl_command!(SetSvgaDisplayResolution => b"#SVRS" for as byte resolution);

// ------------------------------------------------------------------------------------------------

impl_command!(GetSvgaWaterfallBias => b"#SVWB");
impl_command_with_response!(GetSvgaWaterfallBias => 2, u8_from_ascii => u8);

impl_command!(SetSvgaWaterfallBias => b"#SVWB" format bias uint 2);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBCursorState => b"#VFB");
impl_command_with_response!(GetVfoBCursorState => boolean);

impl_command!(SetVfoBCursorState => b"#VFB" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetWaterfallAveragingState => b"#WFA");
impl_command_with_response!(GetWaterfallAveragingState => boolean);

impl_command!(SetWaterfallAveragingState => b"#WFA" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetWaterfallColor => b"#WFC");
impl_command_with_response!(GetWaterfallColor => try_from enum WaterfallColor);

impl_command!(SetWaterfallColor => b"#WFC" for as byte color);

// ------------------------------------------------------------------------------------------------

impl_command!(GetWaterfallMarkersState => b"#WFM");
impl_command_with_response!(GetWaterfallMarkersState => boolean);

impl_command!(SetWaterfallMarkersState => b"#WFM" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverConnected => b"#XCV");
impl_command_with_response!(GetTransceiverConnected => 2, u8_from_ascii => u8);

impl_command!(SetTransceiverConnected => b"#XCV" format transceiver uint 2);
