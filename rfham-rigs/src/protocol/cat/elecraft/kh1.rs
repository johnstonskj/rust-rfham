//!
//! CAT commands for the Elecraft KH1 portable transceiver.
//!
//! The KH1 is a minimalist QRP CW/DATA transceiver. Its command set is much smaller than the
//! K3/K4, and most commands are SET-only; the radio instead pushes state changes via AI mode.
//! Notable differences from the K3/K4 command set:
//!
//! * The `FA` command uses 10 Hz resolution in an 8-digit field, not the 11-digit 1 Hz resolution
//!   used by the K3/K4 (e.g. `FA00014074;` = 14.074 MHz, where the last digit is tens of Hz).
//! * Some commands mirror K3/KX equivalents in format, but with restricted ranges.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft KH1 Programmer's Reference, rev.
//!    B2](https://ftp.elecraft.com/KH1/Manuals%20Downloads/Elecraft%20KH1%20Programmer's%20Ref,%20rev%20B2.pdf),
//!    Jan 2026.
//!

use crate::{
    error::RigError,
    protocol::{
        Frequency,
        cat::{
            Command,
            common::{
                bool_from_ascii_1_0, bytes_to_vec, u8_from_ascii, u32_from_ascii,
                validate_integer_in_range, validate_response,
            },
        },
    },
};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: SetAfGain
// ------------------------------------------------------------------------------------------------

define_command!("Set AF gain (`AG`).

AF gain can also be incremented/decremented using the ENAU/ENAD commands.

# Command format

> `AG{nn};`

Where *nn* is the AF gain level, between `00` and `30`." =>
    SetAfGain {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDisplayText, SetDisplayText
// ------------------------------------------------------------------------------------------------

define_command!("Get the contents of a display line (`DS`).

The LCD supports 8 special characters, some of which change depending on the radio's
operational context. For a GET, the host app must translate these to suitable characters within the
host's display environment. For a SET, the host app must embed low-hex ASCII values for special
characters that make sense in the KH1's context. List of characters and contexts TBD.

# Command format

> `DS{l};`

Where *l* is the display line number, `1` for the top line, or `2` for the bottom line.

# Response format

> `DS{l}{ssssssssssssssss};`

Where:

* *l* is the display line number, `1` for the top line, or `2` for the bottom line
* *ssssssssssssssss* is the 16-character line content, space-padded." =>
    GetDisplayText {
        line: u8
    }
);

define_command!("Set the contents of a display line (`DS`).

The LCD supports 8 special characters, some of which change depending on the radio's
operational context. For a GET, the host app must translate these to suitable characters within the
host's display environment. For a SET, the host app must embed low-hex ASCII values for special
characters that make sense in the KH1's context. List of characters and contexts TBD.

# Command format

> `DS{l}{ssssssssssssssss};`

Where:

* *l* is the display line number, `1` for the top line, or `2` for the bottom line
* *ssssssssssssssss* is the text content, truncated or space-padded to exactly 16 characters

**Note**: The DS SET string is flashed for about 1.5 seconds. Use subsequent DS SETs to keep the
flashed string on the display longer." =>
    SetDisplayText no_copy {
        "Display line, `1` or `2`."
        line: u8,
        "Text content (truncated and padded to 16 characters)."
        text: Vec<u8>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: EmulateEncoderRotation, Encoder, Direction
// ------------------------------------------------------------------------------------------------

define_command!("Emulate encoder rotation (`EN`).

# Command format

> `EN{e}{d};`

Where:

* *e* is the encoder type, 'A' (AF gain) or 'V' (VFO)
* *d* is the rotational direction, 'U' (up/clockwise) or 'D' (down/counter-clockwise)." =>
    EmulateEncoderRotation {
        encoder: Encoder,
        direction: EncoderDirection
    }
);

define_command_enum!(
    "Constants for the encoders present on the KH1" => Encoder {
        "AF Gain" => AfGain = b'A',
        "VFO" => Vfo = b'V'
    }
);

define_command_enum!(
    "Constants for the rotational direction of an encoder." =>
    EncoderDirection {
        "Turned Up/Clockwise" => Up = b'U',
        "Turned Down/Counter-Clockwise" => Down = b'D'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetOperatingFrequency
// ------------------------------------------------------------------------------------------------

define_command!("Set the VFO A operating frequency (`FA`).

Unlike the K3/K4 `FA` command, which uses 11-digit 1 Hz resolution, the KH1 `FA` command uses
8-digit **10 Hz** resolution.

# Command format

> `FA{ffffffff};`

Where *ffffffff* is the frequency,  is in 10 Hz units e.g. 1400000 = 14000.00 kHz." =>
    SetOperatingFrequency {
        freq_10hz: u32
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetVfoOffset
// ------------------------------------------------------------------------------------------------

define_command!("Set the VFO offset (`FO`).

# Command format

> `FO{nn};`

Where *nn* is a value between `00` and `99` applied as positive offsets, in Hz, from
the original VFO frequency; this is intended for use with FT8 transmit.

**Note**: Sending `00`-`98` also puts the VFO display into 1-Hz format as a reminder
that this is in effect. If *nn* = `99`, the offset is removed and the display returns
to the original format." =>
    SetVfoOffset {
        offset_hz: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetHelpInformation
// ------------------------------------------------------------------------------------------------

define_command!("Get Help Information (`H`).

Responds with terse help information.

# Command format

> `H;`

# Response format

> `H{s..};`

Where *s..* is an undefined number of characters." =>
    GetHelpInformation
);

// ------------------------------------------------------------------------------------------------
// Public Types: EmulateHandKeyPress, HandKeyState
// ------------------------------------------------------------------------------------------------

define_command!("Emulate a hand-key press. (`HK`)

This command is especially useful for starting and stopping transmit when sending FT8 messages.

# Command format

> `HK{m};`

Where *m* is `1` for key-down, and `0` for key-up.

**Note**: If you use `SW` to emulate TUNE (transmit key-down), exit TUNE using `SW4T` ('x' switch 
tap) rather than `HK0`." =>
    EmulateHandKeyPress {
        state: HandKeyState
    }
);

define_command_enum!(
    "Represents the state of the HandKey for the [`EmulateHandKeyPress`] command." =>
    HandKeyState {
        KeyDown = b'1',
        KeyUp = b'0'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverId
// ------------------------------------------------------------------------------------------------

define_command!("Get transceiver identification (`I`).

# Command format

> `I;`

# Response format

> `I{sss};`

Where: *sss* will be `KH1` under normal conditions, if the radio is in the boot loader, it responds
to with `kh1`." =>
    GetTransceiverId
);

pub const KH1_ID_NORMAL: &str = "KH1";
pub const KH1_ID_IN_BOOT_LOADER: &str = "kh1";

// ------------------------------------------------------------------------------------------------
// Public Types: LoadFirmware
// ------------------------------------------------------------------------------------------------

define_command!("Load Firmware (`LD`).

Jump to the boot loader.

# Command format

> `LD;`
" =>
    LoadFirmware
);

// ------------------------------------------------------------------------------------------------
// Public Types: DumpLog, LogAction
// ------------------------------------------------------------------------------------------------

define_command!("Dump Logs (`LG`).

# Command format

> `LG{n};`

Where *n* is the action; `0` dump, `1` stop, `2` (ontinue, `3` erase.

## Notes

1. Time stamps are sent once per minute while sending.
2. During dumps, no GET commands should be sent to the KH1, as the results will end up embedded
   in the log text stream. SETs are OK (e.g., LG1/LG2 to stop/continue a dump).
3. Uppercase log text was transmitted. Lowercase log text was entered as a NOTE (i.e. after tapping
   MSG) or during TX TEST mode.
4. Dump and Erase can also be accomplished using the KH1’s LOGGING menu entry." =>
    DumpLog {
        action: LogAction
    }
);

define_command_enum!(
    "Log action for DumpLog" => LogAction {
        Dump = b'0',
        Stop = b'1',
        Continue = b'2',
        Erase = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetOperatingMode
// ------------------------------------------------------------------------------------------------

define_command!("Set the operating mode (`MD`).

# Command format

> `MD{n};`

Where *n* is one of:

* `0`; LSB.
* `1`; USB.
* `2`; CW.
* `4`; DATA.

**Note:** the KH1 supports only modes `0`, `1`, `2`, and `4`.

**Note**: In SSB modes, the KH1 operates cross-mode (CW transmit, SSB receive). SSB receiving
operarators hear the KH1's CW at a 700 Hz pitch." =>
    SetOperatingMode {
        mode: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SelectMenuItem
// ------------------------------------------------------------------------------------------------

define_command!("Select a menu item by its 3-character ID (`MN`).

# Command format

> `MN{sss};`

Where *sss* is the 3-character ASCII menu item identifier, space-padded if shorter (e.g. `MNK␣␣S;`
for keyer speed). See the B2 reference for the full menu ID list." =>
    SelectMenuItem {
        "3-character menu item ID (ASCII, space-padded if shorter)."
        item_id: [u8; 3]
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMenuParameter, SetMenuParameter
// ------------------------------------------------------------------------------------------------

define_command!("Get the current menu parameter value (`MP`).

# Command format

> `MP;`

# Response format

> `MP{nnn};`

Where *nnn* is the value for the currently selected menu item, between `000` and `255`." =>
    GetMenuParameter
);

define_command!("Set the current menu parameter value (`MP`).

# Command format

> `MP{nnn};`

Where *nnn* is the value for the currently selected menu item, between `000` and `255`." =>
    SetMenuParameter {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareRevision
// ------------------------------------------------------------------------------------------------

define_command!("Get the firmware revision string (`RV`).

# Command format

> `RV;`

# Response format

> `RV{xx.xx};`

Where *xx.xx* is the firmware version string, e.g. `01.08`." =>
    GetFirmwareRevision
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverSerialNumber
// ------------------------------------------------------------------------------------------------

define_command!("Get the transceiver serial number (`SN`).

# Command format

> `SN;`

# Response format

> `SN{nnnnn};`

Where *nnnnn* is the 5-digit serial number." =>
    GetTransceiverSerialNumber
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverStatus, TransceiverStatus
// ------------------------------------------------------------------------------------------------

define_command!("Get combined transceiver status (`ST`).

# Command format

> `ST;`

# Response format

> `ST{n}{s}{a};`

Where:

* *n*; number of self-test errors since power-up
* *s*; `S` if this unit has an assigned serial number, otherwise `s` (lower-case)
* *a*; `A` if an ATU module is found, otherwise `a` (lower-case)

**Note**: The ATU presence test is independent of the MENU:ATU MODE setting." =>
    GetTransceiverStatus
);

define_command_struct!(
    "KH1 TransceiverStatus returned by [`GetTransceiverStatus`]." =>
    TransceiverStatus {
        "Number of self-test errors since power-up." =>
        self_test_errors: u8,
        "Set to `true` if this unit has an assigned serial number, otherwise `false`." =>
        serial_numer_assigned: bool,
        "Set to `true` if an ATU module is found, otherwise `false`." =>
        atu_module_found: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: EmulateButtonTap, EmulateButtonHold
// ------------------------------------------------------------------------------------------------

define_command!("Emulate a button tap (`SW`).

# Command format

> `SW{n}{t};`

Where *n* is the button number, `1`-`4` (regular pushbutton switches) or `5`-`6` (encoder switches).

The value of *t* **must** be `T` which distinguishes this action from [`EmulateButtonHold`], which
shares the same `SW` command identifier." =>
    EmulateButtonTap {
        button: u8
    }
);

define_command!("Emulate a button hold (`SW`).

# Command format

> `SW{n}{t};`


Where *n* is the button number, `1`-`4` (regular pushbutton switches) or `5`-`6` (encoder switches).

The value of *t* **must** be `H` which distinguishes this action from [`EmulateButtonTap`], which
shares the same `SW` command identifier." =>
    EmulateButtonHold {
        button: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransmitLowLimit, GetTransmitHighLimit
// ------------------------------------------------------------------------------------------------

define_command!("Get the lower transmit frequency limit for the current band (`TXL`).

# Command format

> `TXL{n};`

Where *n* is `0` to `4` for `40` to `15` meters

# Response format

> `TXL{fffff};`

Where *fffff* is the lower limit, in kHz.

**Note**: transmit limits are set for the user's country of operation at the factory." =>
    GetTransmitLowerLimit {
        band: TransmitBand
    }
);

define_command!("Get the upper transmit frequency limit for the current band (`TXH`).

# Command format

> `TXH{n};`

Where *n* is `0` to `4` for `40` to `15` meters

# Response format

> `TXH{fffff};`

Where *fffff* is the upper limit, in kHz.

**Note**: transmit limits are set for the user's country of operation at the factory." =>
    GetTransmitUpperLimit {
        band: TransmitBand
    }
);

define_command_enum!(
    "Transmit band for [`GetTransmitLowerLimit`] and [`GetTransmitUpperLimit`]." =>
    TransmitBand {
        Band40m = b'0',
        Band30m = b'1',
        Band20m = b'2',
        Band17m = b'3',
        Band15m = b'4'
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_command!(
    SetAfGain => b"AG"
    format level uint 2,
    if |cmd: &SetAfGain| validate_integer_in_range("level", "u8", cmd.level, 0, 30)
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetDisplayText => b"DS" with Some |cmd: &GetDisplayText| {
    vec![b'0' + cmd.line]
}, if |cmd: &GetDisplayText| validate_integer_in_range("line", "u8", cmd.line, 1, 2));

impl_command_with_response!(GetDisplayText => 18, |bytes: &[u8]| {
    Ok(bytes[2..].to_vec())
} => Vec<u8>);

impl_command!(SetDisplayText => b"DS" with Some |cmd: &SetDisplayText| {
    let mut bytes = vec![b'0' + cmd.line, b' '];
    let len = cmd.text.len().min(16);
    bytes.extend_from_slice(&cmd.text[..len]);
    while bytes.len() < 18 {
        bytes.push(b' ');
    }
    bytes
}, if |cmd: &SetDisplayText| validate_integer_in_range("line", "u8", cmd.line, 1, 2));

// ------------------------------------------------------------------------------------------------

impl_command!(EmulateEncoderRotation => b"EN" with Some |cmd: &EmulateEncoderRotation| {
    vec![cmd.encoder as u8, cmd.direction as u8]
});

// ------------------------------------------------------------------------------------------------

impl_command!(SetOperatingFrequency => b"FA" with Some |cmd: &SetOperatingFrequency| {
    format!("{:08}", cmd.freq_10hz).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoOffset => b"FO"
    format offset_hz uint 2,
    if |cmd: &SetVfoOffset| validate_integer_in_range("offset_hz", "u8", cmd.offset_hz, 0, 99)
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetHelpInformation => b"H");

impl_command_with_response!(GetHelpInformation => string);

// ------------------------------------------------------------------------------------------------

impl_command!(EmulateHandKeyPress => b"HK" for as byte state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverId => b"I");
impl_command_with_response!(GetTransceiverId => string);

// ------------------------------------------------------------------------------------------------

impl_command!(LoadFirmware => b"LD");

// ------------------------------------------------------------------------------------------------

impl_command!(DumpLog => b"LG" for as byte action);

// ------------------------------------------------------------------------------------------------

impl_command!(SetOperatingMode => b"MD" with Some |cmd: &SetOperatingMode| {
    vec![b'0' + cmd.mode]
}, if |cmd: &SetOperatingMode| validate_kh1_operating_mode_digit(cmd.mode));

// ------------------------------------------------------------------------------------------------

impl_command!(SelectMenuItem => b"MN" with Some |cmd: &SelectMenuItem| {
    cmd.item_id.to_vec()
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetMenuParameter => b"MP");
impl_command_with_response!(GetMenuParameter => 3, u8_from_ascii => u8);

impl_command!(SetMenuParameter => b"MP" format value uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetFirmwareRevision => b"RV");
impl_command_with_response!(GetFirmwareRevision => 4, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverSerialNumber => b"SN");
impl_command_with_response!(GetTransceiverSerialNumber => 5, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverStatus => b"ST");
impl_command_with_response!(GetTransceiverStatus => 3, |bytes: &[u8]| {
    Ok(TransceiverStatus {
        self_test_errors: bytes[0],
        serial_numer_assigned: bool_from_ascii_1_0(bytes[1])?,
        atu_module_found: bool_from_ascii_1_0(bytes[2])?,
    })
} => TransceiverStatus);

// ------------------------------------------------------------------------------------------------

impl_command!(EmulateButtonTap => b"SW" with Some |cmd: &EmulateButtonTap| {
    vec![b'0' + cmd.button, b'T']
}, if |cmd: &EmulateButtonTap| validate_integer_in_range("button", "u8", cmd.button, 1, 6));

impl_command!(EmulateButtonHold => b"SW" with Some |cmd: &EmulateButtonHold| {
    vec![b'0' + cmd.button, b'H']
}, if |cmd: &EmulateButtonHold| validate_integer_in_range("button", "u8", cmd.button, 1, 6));

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransmitLowerLimit => b"TXL" for as byte band);
impl_command_with_response!(GetTransmitLowerLimit => try_from 5 Frequency);

impl_command!(GetTransmitUpperLimit => b"TXH" for as byte band);
impl_command_with_response!(GetTransmitUpperLimit => try_from 5 Frequency);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

/// Validate that a mode digit is one of the documented `MD` values (`0`, `1`, `2`, or `4`); the
/// KH1 does not support the full K3/K4 mode set.
pub(crate) fn validate_kh1_operating_mode_digit(mode: u8) -> Result<(), RigError> {
    if matches!(mode, 0 | 1 | 2 | 4) {
        Ok(())
    } else {
        error!("mode value {mode} is not a documented KH1 MD mode digit");
        Err(RigError::InvalidArgumentValue {
            argument_name: "mode",
            type_name: "u8",
            value: mode.to_string(),
        })
    }
}
