//!
//! CAT commands specific to or extended on the Elecraft K2, via KIO2, transceiver.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # Implementation Notes
//!
//! The following notes are from Reference #1.
//!
//! **Extended Commands**; Some commands have an alternate data format which provides enhanced
//! functionality. These alternate formats are referred to as *extended* commands. For details, see
//! `K2` in the Meta-commands section.
//!
//! **Response Time**; The K2 will respond to most commands in less than 20 milliseconds. To cover
//! exceptions, we recommend using a timeout of 100 ms. Some commands have additional timing
//! requirements as explained later.
//!
//! **Polling**; Since the KIO2 provides a full-duplex interface, the computer can poll the K2 for
//! data at any time. However, we recommend that TX-mode polling not be used unless necessary. This
//! will prevent any problems with serial I/O that might be caused if high RF voltages are present
//! on the K2 chassis, such as might occur if grounding is inadequate.
//!
//! **Busy Indication**; Most SET commands cannot be safely handled when the K2 is in a busy state,
//! including transmit, direct frequency entry prompting, and scanning. The K2 will respond with
//! `?;` to disallowed commands at such times. The only SET commands that are allowed
//! unconditionally during busy states are: `AI`, `K2`, `KS`, `KY`, `PC`, `RX`, and `SW`. In
//! addition, `RC` (RIT clear) commands that occur during transmit will return `?;` but will still
//! take effect, clearing the RIT/XIT offset when the K2 next returns to receive mode, however
//! briefly. Finally, during CW message repeat intervals, `RC`, `RD`, and `RU` are all allowed.
//!
//! # References
//!
//! 1. [KIO2 Programmers Reference, rev. E](https://ftp.elecraft.com/K2/Manuals%20Downloads/KIO2%20Pgmrs%20Ref%20rev%20E.pdf), Feb 2004

use crate::{
    Level,
    error::{RigError, invalid_response_data, invalid_response_length},
    protocol::{
        Frequency,
        cat::{
            common::{
                assert_all_bytes_eq, assert_byte_eq, bool_from_ascii_1_0, string_from_ascii,
                u8_from_ascii, u16_from_ascii,
            },
            elecraft::Vfo,
        },
    },
};
use core::fmt::Display;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetK2CommandMode, SetK2CommandMode, K2CommandMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the K2 command mode.

K2 (K2 command mode): The K2 meta-command modifies the data format of selected commands, as follows:

* `K20`, Normal mode: This is the default. In this mode, command *extensions* are disabled, such as
  control of the audio filter by the `FW` command. This may simplify program development, and also
  provides greater compatibility with existing software.
* `K21`, Normal/rtty_off: Same as `K20`, except that the `MD` and `IF` commands report RTTY and 
  RTTY-reverse modes as LSB and USB, respectively. This may be useful if your program doesn't
  support the K2's RTTY mode.
* `K22`, Extended mode: Enables all command extensions. This is the mode we recommend for use with
  new application programs or programs that will be modified to function better with the K2.
* `K23`, Extended/rtty_off: Enables all extensions, but like mode `K21`, alters the nature of the
  `MD` and `IF` commands.

The K3/K3S also accepts this command and behaves identically.

# Command format

> `K2;`

# Response format

> `K2{n};`

Where *n* is:

* `0`; Normal RSP, RTTY decode on
* `1`; Normal RSP, RTTY decode off
* `2`; Extended RSP, RTTY decode on
* `3`; Extended RSP, RTTY decode off" =>
    GetK2CommandMode
);

define_cat_command!("Set the K2 command mode.

K2 (K2 command mode): The K2 meta-command modifies the data format of selected commands, as follows:

* `K20`, Normal mode: This is the default. In this mode, command *extensions* are disabled, such as
  control of the audio filter by the `FW` command. This may simplify program development, and also
  provides greater compatibility with existing software.
* `K21`, Normal/rtty_off: Same as `K20`, except that the `MD` and `IF` commands report RTTY and 
  RTTY-reverse modes as LSB and USB, respectively. This may be useful if your program doesn't
  support the K2's RTTY mode.
* `K22`, Extended mode: Enables all command extensions. This is the mode we recommend for use with
  new application programs or programs that will be modified to function better with the K2.
* `K23`, Extended/rtty_off: Enables all extensions, but like mode `K21`, alters the nature of the
  `MD` and `IF` commands.

The K3/K3S also accepts this command and behaves identically.

# Command format

> `K2{n};`

Where *n* is:

* `0`; Normal RSP, RTTY decode on
* `1`; Normal RSP, RTTY decode off
* `2`; Extended RSP, RTTY decode on
* `3`; Extended RSP, RTTY decode off" =>
    SetK2CommandMode {
        mode: K2CommandMode
    }
);

define_command_struct!(
    "Represents the parsed K2 command-mode response." =>
    K2CommandMode {
        extended: bool,
        rtty_off: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoInfoMode, SetAutoInfoMode, AutoInfoMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Auto-Information (AI) mode (`AI`).

The AI meta-command controls how and when the K2 responds to commands:

# Command format

> `AI;`

# Response format

> `AI{n};`

Where *n* is a value between `0` and `3`:

* `0`; No Auto-info: This is the default. No radio information will be automatically reported. (You
  can still request specific information using GET commands.)
* `1`; Auto-Info Mode 1: The K2 sends an `IF` (info) response within 1 second when any frequency or 
  mode-related event occurs, either manually (at the radio itself) or via computer commands.
  These events include: band change, mode change, VFO movement, RIT/XIT offset change or clear,
  and several additional switches (A/B, REV, A=B, SPLIT, CW REV, RIT, XIT). The IF responses are
  suppressed during VFO movement. Note: putting the K2 into auto-info mode `1` (by sending `AI1;`)
  causes an initial `IF` response.
* `2`; Auto-Info Mode 2: The K2 sends an appropriate response (`FA`, `FB`, `IF`, `GT`, etc.) whenever
  any front-panel event occurs. This applies to all of the events mentioned for mode `AI1`, as well
  as all potentiometer changes except AF GAIN and RF GAIN, and all switch presses. In some cases
  responses are grouped; e.g., pressing switches will report the present state of several
  parameters, including the one related to the new event.
* `3`; Combination: This is similar to mode `AI2` and is provided only for compatibility with existing
programs." =>
    GetAutoInfoMode
);

define_cat_command!("Set Auto-Information (AI) mode (`AI`).

The AI meta-command controls how and when the K2 responds to commands:

# Command format

> `AI{n};`


Where *n* is a value between `0` and `3`:

* `0`; No Auto-info: This is the default. No radio information will be automatically reported. (You
  can still request specific information using GET commands.)
* `1`; Auto-Info Mode 1: The K2 sends an `IF` (info) response within 1 second when any frequency or 
  mode-related event occurs, either manually (at the radio itself) or via computer commands.
  These events include: band change, mode change, VFO movement, RIT/XIT offset change or clear,
  and several additional switches (A/B, REV, A=B, SPLIT, CW REV, RIT, XIT). The IF responses are
  suppressed during VFO movement. Note: putting the K2 into auto-info mode `1` (by sending `AI1;`)
  causes an initial `IF` response.
* `2`; Auto-Info Mode 2: The K2 sends an appropriate response (`FA`, `FB`, `IF`, `GT`, etc.) whenever
  any front-panel event occurs. This applies to all of the events mentioned for mode `AI1`, as well
  as all potentiometer changes except AF GAIN and RF GAIN, and all switch presses. In some cases
  responses are grouped; e.g., pressing switches will report the present state of several
  parameters, including the one related to the new event.
* `3`; Combination: This is similar to mode `AI2` and is provided only for compatibility with existing
  programs." =>
    SetAutoInfoMode {
        mode: AutoInfoMode
    }
);

define_command_enum!(
    "Possible options for Auto-Information (AI) mode." =>
    AutoInfoMode {
        "AI off, No radio information will be automatically reported." => Off = b'0',
        "AI on, for any frequency or mode-related event." => OnlyFrequencyAndModeRelated = b'1',
        "AI on for any front-panel event." => AnyFrontPanel = b'2',
        "AI on for compatibility, similar to `AI2`." => CompatibilityCombination = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAntennaSelection, SetAntennaSelection, SelectedAntenna
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the currently selected antenna (`AN`).

# Command format

> `AN;`

# Response format

> `AN{n};`

Where *n* is the antenna number, either `1` or`2`. See [`SelectedAntenna`] for details." =>
    GetAntennaSelection
);

define_cat_command!("Set the currently selected antenna (`AN`).

# Command format

> `AN{n};`

Where *n* is the antenna number, either `1` or`2`. See [`SelectedAntenna`] for details." =>
    SetAntennaSelection {
        antenna: SelectedAntenna
    }
);

define_command_enum!(
    "Antenna selection." =>
    SelectedAntenna {
        "K2 Antenna 1" => Antenna1 = b'1',
        "K2 Antenna 2" => Antenna2 = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBargraphValue
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the bargraph meter value (`BG`).

# Command format

> `BG;`

# Response format

> `BG{nn};`

Where: 

* *nn* describes which bars are turned on. See [`BargraphMode`] for details.

**Notes**:

1. *nn* is `00` (no bars) through `10` (bar 10) if the bargraph is in **DOT** mode, and `12` (no
   bars) through `22` (all 10 bars) if the bargraph is in **BAR** mode.
2. Reads the S-meter level on receive. 
3. Reads the power output level or ALC level on transmit, depending on the **RF/ALC** selection.

Also see the `SM` command." =>
    GetBargraphValue
);

define_command_struct!(
    "Represents the parsed bargraph meter value response." =>
    BargraphValue {
        value: u8,
        mode: BargraphMode
    }
);

define_command_enum!(
    "Bargraph meter mode." =>
    BargraphMode {
        "Bargraph in DOT mode" => Dot = b'0',
        "Bargraph in BAR mode" => Bar = b'1'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAAfGain, SetVfoAAfGain, GetVfoBAfGain, SetVfoBAfGain
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the AF (audio) gain for the VFO A (main) receiver.

Unverified: no rig currently implemented in this crate exercises this command; the command byte
and framing follow the general Kenwood dialect convention but have not been confirmed against a
specific radio's programmer's reference.

# Command format

> `AG0;`

# Response format

> `AG0{n};`

Where *n* is the gain level. See [`Level`]." =>
    GetVfoAAfGain
);

define_cat_command!("Set the AF (audio) gain for the VFO A (main) receiver.

Unverified — see [`GetVfoAAfGain`].

# Command format

> `AG0{n};`

Where *n* is the gain level. See [`Level`]." =>
    SetVfoAAfGain {
        level: Level
    }
);

define_cat_command!("Get the AF (audio) gain for the VFO B (sub) receiver.

Unverified — see [`GetVfoAAfGain`].

# Command format

> `AG1;`

# Response format

> `AG1{n};`

Where *n* is the gain level. See [`Level`]." =>
    GetVfoBAfGain
);

define_cat_command!("Set the AF (audio) gain for the VFO B (sub) receiver.

Unverified — see [`GetVfoAAfGain`].

# Command format

> `AG1{n};`

Where *n* is the gain level. See [`Level`]." =>
    SetVfoBAfGain {
        level: Level
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoADisplayAndIcons, VfoADisplayAndIcons, VfoAAnnunciatorData,
//      VfoAAnnunciatorFlashData
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get VFO A display text and icons (`DS`).

Returns everything needed to reproduce the information shown on the K2 LCD.

# Command format

> `DS;`

# Response format

> `DS{tttttttt}{a}{f};`

Where *tttttttt* is the LCD text and decimal point data, *a* is annunciator data, and *f* is
annunciator flash data. These fields are detailed below.

**TEXT and decimal point data**: This field contains 8 bytes, with values from `0x30` to `0xFF`
(hexadecimal). The first of the eight bytes is the left-most displayed character. Bit 7 (MSB) of
each byte is used to indicate whether the decimal point to the left of each character is on (`1`) or
off (`0`). The remaining 7 bits (b6-b0) contain an ASCII character that corresponds to the displayed
character.

Some ASCII characters cannot be shown on a 7-segment display (e.g., 'X', 'M'). The K2 uses these
characters as placeholders for special characters that can be displayed, in some cases lower-case
versions of letters to enhance display readability. For this reason, the characters returned by the
`DS` command must sometimes be converted to other characters by the software application. The
following table shows all of these conversions. The table assumes that the decimal-point flag
(bit 7) has been cleared from the text-field characters.

| DS chr.   | Converts to       | DS chr.   | Converts to   | DS chr.   | Converts to       |
|-----------|-------------------|-----------|---------------|-----------|-------------------|
| `<`       | small-caps `L`    | `M`       | `N`           | `X`       | c-bar.            |
| `>`       | dash              | `Q`       | `O`           | `Z`       | lowercase `c`     |
| `@`       | space (blank)     | `V`       | `U`           | `[`       | r-bar             |
| `K`       | `H`               | `W`       | `I`           |           |                   |


**Annunciator data**: This field is a single byte whose value is between `0x80` and `0xFF`. Bit 7 is
always 1. The other 7 bits indicate the flash/non-flashed states of the 8 annunciators, providing
useful status information (such as whether the transceiver is operating in SPLIT mode). The bits are
defined as follows:

| Bit | Meaning                         | Bit | Meaning                                 |
|-----|---------------------------------|-----|-----------------------------------------|
| 7   | Always `1`                      | 3   | `1`=ATT on                              |
| 6   | `1`=NB flashing (LO THR)*       | 2   | `0`=VFO A selected (always `0` for K3)  |
| 5   | `1`=ANT2 flashing (not used)    | 1   | `1`=RIT on                              |
| 4   | `1`=PRE flashing (not used)     | 0   | `1`=XIT on                              |

**Icon flash data or additional K3 icons**: This field is a single byte whose value is between 
`0x80` and `0xFF`. Bit 7 is always `1`. The other 7 bits indicate the flash/non-flashed states of
the 8 annunciators, providing useful status information (such as whether the transceiver is
operating in SPLIT mode). The bits are defined as follows:

| Bit | Meaning                         | Bit | Meaning                                     |
|-----|---------------------------------|-----|---------------------------------------------|
| 7   | Always `1`                      | 3   | `1`=ATT flashing (not used)                 |
| 6   | `1`=NB flashing (LO THR)        | 2   | `1`=active VFO flashing (SPLIT mode)        |
| 5   | `1`=ANT2 flashing (not used)    | 1   | `1`=RIT flashing (RIT/XIT range > minimum)  |
| 4   | `1`=PRE flashing (not used)     | 0   | `1`=XIT flashing (RIT/XIT range > minimum)  |
" =>
    GetVfoADisplayAndIcons
);

define_command_struct!(
    "VFO A display text and icons from the `DS` response." =>
    VfoADisplayAndIcons no_copy {
        "VFO A display text (8 bytes)." => text: String,
        "VFO A display decimal point indicators." => decimal_points: [bool; 8],
        "VFO A annunciator data (1 byte)." => annunciator_data: VfoAAnnunciatorData,
        "VFO A annunciator flash data or additional K3 icons (1 byte)." => annunciator_flash_data: VfoAAnnunciatorFlashData
    }
);

define_command_struct!(
    "VFO A icon data from the `DS` response." =>
    VfoAAnnunciatorData {
        "Noise blanker (NB) is on." => noise_blanker_on: bool,
        "Antenna 2 is selected." => antenna_2_selected: bool,
        "Preamp (PRE) is on." => preamp_on: bool,
        "Attenuator (ATT) is on." => attenuator_on: bool,
        "VFO A is selected." => vfo_a_selected: bool,
        "RIT is on." => rit_on: bool,
        "XIT is on." => xit_on: bool
    }
);

define_command_struct!(
    "VFO A icon flash data or additional K3 icons from the `DS` response." =>
    VfoAAnnunciatorFlashData {
        "Noise blanker (NB) flashing; LO THR." => noise_blanker_flashing: bool,
        "Antenna 2 flashing (not used)." => antenna_2_flashing: bool,
        "PRE flashing (not used)." => preamp_flashing: bool,
        "ATT flashing (not used.)" => attenuator_flashing: bool,
        "Active VFO flashing; in SPLIT mode." => active_vfo_flashing: bool,
        "RIT flashing; RIT/XIT range > minimum." => rit_flashing: bool,
        "XIT flashing; RIT/XIT range > minimum." => xit_flashing: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAOperatingFrequency, SetVfoAOperatingFrequency, GetVfoBOperatingFrequency,
//      SetVfoBOperatingFrequency
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the VFO A operating frequency.

# Command format

> `FA;`

# Response format

> `FA{nnnnnnnnnnn};`

Where *nnnnnnnnnnn* is the frequency, in Hz, as an 11-digit zero-padded value.

**Notes**:

1. **K2**; the first two digits (00-99 GHz) and the last digit (0-9 Hz) are ignored.
2. **K2**; if the specified frequency is in a different amateur band than the present one, the
   K2 will change to the new band, and will automatically report the new values of parameters that
   may have changed. If the specified frequency is over 30 MHz and is within a valid transverter
   band (as specified by the operator using the K2's **TRN1-3** menu entries), the K2 will switch
   to that transverter band. If the specified frequency is one that the K2 VFO cannot be tuned to,
   the K2 will switch to the amateur band closest to the requested one, and the last-used VFO A and
   VFO B values for that band will be retrieved.
3. **K3**; the Hz digit is ignored if the K3 is not in FINE mode (1-Hz tuning; use SWT49).
4. **K3**; if the specified frequency is in a different amateur band than the present one, the
   K3 will change to the new band, and will automatically report the new values of parameters that
   may have changed.
5. **K3**; band changes typically take 0.5 seconds; all command handling is deferred until this
   process is complete. 
6. **K3**; if the specified frequency is over 30 MHz and is within a valid transverter band (as
   specified by the operator using the K3's **XVTR** menu entries), the K3 will switch to that
   transverter band. If the specified frequency is outside the range of 500 kHz-30 MHz and
   48-54 MHz, the K3 will switch to the amateur band closest to the requested one, and the last-used
   VFO A and VFO B values for that band will be retrieved. (KSYN3A extends low range to 100 kHz.)
7. **K4**; frequency range is 100 kHz to 54 MHz. The digit count (length of *n*) affects
   interpretation:
   1. 1 or 2 digits: *n* = MHz (e.g., `FA7;` = 7 MHz)
   2. 3 to 5 digits: *n* = kHz (e.g., `FA1234;` = 1234 kHz)
   3. 6+ digits: *n* = Hz (e.g., `FA1234567;` = 12,345,67 Hz).
7. For radios with two VFOs, when the VFOs are linked (non-SPLIT), `FA` also sets VFO B to the same
   frequency as VFO A." =>
    GetVfoAOperatingFrequency
);

define_cat_command!("Set the VFO A operating frequency.

# Command format

> `FA{nnnnnnnnnnn};`

Where *nnnnnnnnnnn* is the frequency, in Hz, as an 11-digit zero-padded value." =>
    SetVfoAOperatingFrequency {
        frequency: Frequency
    }
);

define_cat_command!("Get the VFO B operating frequency.

# Command format

> `FB;`

# Response format

> `FB{nnnnnnnnnnn};`

Where *nnnnnnnnnnn* is the frequency, in Hz, as an 11-digit zero-padded value." =>
    GetVfoBOperatingFrequency
);

define_cat_command!("Set the VFO B operating frequency.

# Command format

> `FB{nnnnnnnnnnn};`

Where *nnnnnnnnnnn* is the frequency, in Hz, as an 11-digit zero-padded value." =>
    SetVfoBOperatingFrequency {
        frequency: Frequency
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverId
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the transceiver's radio ID code.

# Command format

> `ID;`

# Response format

> `ID{nnn};`

Where *nnn* is a manufacturer-defined numeric code identifying the radio model." =>
    GetTransceiverId
);

// ------------------------------------------------------------------------------------------------
// Public Types: MoveVfoAFrequencyDown
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move VFO A down by one tuning step (`DN`).

# Command format

> `DN;`

# Extended Command format

> `DN{n};`

Where *n* is an optional VFO change specification: `0`=1 Hz; `1` or not used=10 Hz; `2`=20 Hz;
`3`=50 Hz; `4`=1 kHz. See [`VfoFrequencyChangeStep`] for details." =>
    MoveVfoAFrequencyDown {
        step: Option<VfoFrequencyChangeStep>
    }
);

define_command_enum!(
    "Represents a VFO frequency change step size." =>
    VfoFrequencyChangeStep {
        "10 Hz" => Step10Hz = b'0',
        "20 Hz" => Step20Hz = b'1',
        "50 Hz" => Step50Hz = b'2',
        "1 kHz" => Step1kHz = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetReceiveTransmitVfo
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set the receive/transmit VFO (`FR`).

# Command format

> `FR{n};`

Where *n* is `0` for VFO A and `1` for VFO B. See [`Vfo`] for details.

**Note**: sending an `FR` command always cancels SPLIT mode." =>
    SetReceiveTransmitVfo {
        vfo: Vfo
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetTransmitVfo
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set the transmit-mode VFO (`FT`).

# Command format

> `FT{n};`

Where *n* is `0` for VFO A and `1` for VFO B. See [`Vfo`] for details.

**Note**: if the transmit VFO is not the same as the receive VFO, the K2 will by definition be in
SPLIT mode." =>
    SetTransmitVfo {
        vfo: Vfo
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFilterBandwidth, SetFilterBandwidth
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get filter bandwidth in Hz (`FW`).

The K2's limited support for the basic FW command is provided only for compatibility with existing
application software. New or modified software should use the extended version of the command.

# Command format

> `FW;`

# Response format

> `FW{ffff};`

Where *ffff* is the approximate bandwidth in Hz if the mode is CW. If the mode is SSB or RTTY, 
*ffff* is `0000` (*narrow*) when the CW filter is selected, and `2500` (*wide*) if OP1 is selected." =>
    GetFilterBandwidth
);

define_cat_command!("Set filter bandwidth in Hz (`FW`).

The K2's limited support for the basic FW command is provided only for compatibility with existing
application software. New or modified software should use the extended version of the command.

# Command format

> `FW{ffff};`

Where *ffff* is the frequency in Hertz between `0000` and `9999` in Hz.

**Note**: the value of *ffff* is ignored. The next available crystal filter is selected." =>
    SetFilterBandwidth {
        bandwidth_hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFilterBandwidthExtended, SetFilterBandwidthExtended, FilterBandwidthAndNumber,
//      FilterNumber, AudioFilterMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get filter bandwidth in Hz, filter number, and mode (`FW`).

New or modified software should use this *extended* version of the command.

# Command format

> `FW;`

# Response format

> `FW{ffff}{n}{m};`

Where:

* *ffff*; is the approximate bandwidth in Hz if the mode is CW. If the mode is SSB or RTTY, *ffff* is
  `0000` (*narrow*) when the CW filter is selected, and `2500` (*wide*) if OP1 is selected.
* *n*; the filter number, either `1`, `2`, `3`, or `4`. See [`FilterNumber`] for details.
* *m*; the audio filter mode, either `0`, `1`, or `2`. See [`FilterNumber`] for details.

**Example**: a response of `FW040031;` indicates a 400-Hz bandwidth crystal filter, filter FL3;
and an audio filter mode of 1 (AF1). The range of KAF2 modes is `0`-`2`, where `0` is OFF (2.5 kHz
LPF only), `1` is AF1 (first stage of CW band-pass filter) and `2` is AF2 (second stage of CW
band-pass filter).

**Note**: the audio filter mode can only be changed by using the **AFIL** switch or the equivalent
`SW` switch emulation command." =>
    GetFilterBandwidthExtended
);

define_cat_command!("Set filter bandwidth in Hz, filter number, and mode (`FW`).

New or modified software should use this *extended* version of the command.

# Command format

> `FW{ffff}{n}{m};`

Where:

* *ffff*; is the approximate bandwidth in Hz if the mode is CW. If the mode is SSB or RTTY, *ffff* is
  `0000` (*narrow*) when the CW filter is selected, and `2500` (*wide*) if OP1 is selected.
* *n*; the filter number, either `1`, `2`, `3`, or `4`. See [`FilterNumber`] for details.
* *m*; the audio filter mode, either `0`, `1`, or `2`. See [`FilterNumber`] for details.

**Example**: a command of `FW040031;` indicates a 400-Hz bandwidth crystal filter, filter FL3;
and an audio filter mode of 1 (AF1). The range of KAF2 modes is `0`-`2`, where `0` is OFF (2.5 kHz
LPF only), `1` is AF1 (first stage of CW band-pass filter) and `2` is AF2 (second stage of CW
band-pass filter).

**Note**: the audio filter mode can only be changed by using the **AFIL** switch or the equivalent
`SW` switch emulation command." =>
    SetFilterBandwidthExtended {
        bandwidth_hz: u16,
        filter_number: FilterNumber
    }
);

define_command_struct!(
    "Represents the parsed filter bandwidth and number response." =>
    FilterBandwidthAndNumber {
        "Filter bandwidth in Hertz." => bandwidth_hz: u16,
        "Filter number (1-4) or None if not applicable." => filter_number: FilterNumber,
        "Audio filter mode or None if not applicable." => audio_filter_mode: AudioFilterMode
    }
);

define_command_enum!(
    "Filter number, as used in [`GetFilterBandwidthExtended`] and [`SetFilterBandwidthExtended`]." =>
    FilterNumber {
        "Filter number 1 (FIL1)" => Filter1 = b'1',
        "Filter number 2 (FIL2)" => Filter2 = b'2',
        "Filter number 3 (FIL3)" => Filter3 = b'3',
        "Filter number 4 (FIL4)" => Filter4 = b'4'
    }
);

define_command_enum!(
    "Audio filter mode, as used in [`GetFilterBandwidthExtended`] and [`SetFilterBandwidthExtended`]." =>
    AudioFilterMode {
        "Audio filter mode OFF; 2.5 kHz low-pass filter (LPF) only." => Off = b'0',
        "Audio filter AF1; first stage of CW band-pass filter." => AudioFilter1 = b'1',
        "Audio filter AF2; second stage of CW band-pass filter." => AudioFilter2 = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverInformation, TransceiverInformation
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transceiver information (`IF`).

# Command format

> `IF;`

# Response format

> `IF{fffffffffff}␣␣␣␣␣{s}{yyyy}{r}{x}␣00{t}{m}{v}{s2}{p}{b}01␣;`

| Bits   | Field         | Width | Description                                                       |
|--------|---------------|-------|-------------------------------------------------------------------|
| 00..11 | *fffffffffff* | 11    | Operating frequency in Hz, zero-padded (same as `FA`)             |
| 11..16 | `␣`           | 5     | Reserved Spaces                                                   |
| 16     | *s*           | 1     | RIT/XIT sign, `+` or `-`                                          |
| 17..21 | *yyyy*        | 4     | RIT/XIT offset Hz (`0000`-`9999`)                                 |
| 21     | *r*           | 1     | `1` if RIT is on, else `0`                                        |
| 22     | *x*           | 1     | `1` if XIT is on, else `0`                                        |
| 23     | `␣`           | 1     | Reserved Space                                                    |
| 24..26 | `00`          | 2     | Reserved, always `0`                                              |
| 26     | *t*           | 1     | `1` if in transmit mode                                           |
| 27     | *m*           | 1     | Operating mode digit (see `MD`)                                   |
| 28     | *v*           | 1     | Receive VFO: `0` = A, `1` = B (K4: always `0`)                    |
| 29     | *s2*          | 1     | `1` if scan in progress, else `0`                                 |
| 30     | *p*           | 1     | `1` if in split mode, else `0`                                    |
| 31     | *b*           | 1     | In K22 mode: `1` on band change, else `0`. Always `0` in basic.   |
| 32     | `0`           | 1     | Reserved, always `0`                                              |
| 33     | `1`           | 1     | Reserved, always `1`                                              |
| 34     | `␣`           | 1     | Reserved space                                                    |

**Note**: the fixed-value fields (' ', '0', and '1') are provided for syntactic compatibility with
existing software." =>
    GetTransceiverInformation
);

define_command_struct!(
    "Decoded transceiver state from the `IF` response." =>
    TransceiverInformation {
        "Current VFO operating frequency, excluding any RIT/XIT offset, 11 digits; see `FA` command." => operating_frequency: Frequency,
        "Sign of RIT/XIT offset, either `+` or `-`." => rit_xit_sign_negative: bool,
        "RIT/XIT offset in Hz, range is `-9990` to `+9990` Hz when computer-controlled." => rit_xit_offset_hz: u16,
        "Indicates whether RIT is on." => rit_on: bool,
        "Indicates whether XIT is on." => xit_on: bool,
        "Indicates whether the transceiver is in transmit mode." => in_transmit_mode: bool,
        "Current VFO operating mode; see `MD` command." => operating_mode: OperatingMode,
        "Receive-mode VFO selection." => receive_mode_vfo: Vfo,
        "Indicates whether a scan is in progress." => scan_in_progress: bool,
        "Indicates whether the transceiver is in split mode." => in_split_mode: bool,
        "Indicates whether this response is due to a transceiver band-change." => event_on_band_change: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoLockState, SetVfoALockState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get VFO A lock on/off state (`LK`).

# Command format

> `LK;`

# Response format

> `LK{n};`

Where `n` is the boolean state `0` (unlocked) or `1` (locked)." =>
    GetVfoLockState
);

define_cat_command!("Set VFO A lock on/off state (`LK`).

Setting lock state to `true` disables the VFO A encoder.

# Command format

> `LK{n};`

Where `n` is the boolean state `0` (unlocked) or `1` (locked)." =>
    SetVfoLockState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoOperatingMode, SetVfoOperatingMode, OperatingMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current VFO's operating mode (`MD`).

# Command format

> `MD;`

# Response format

> `MD{n};`

Where *n* is one of:

* `1`; LSB
* `2`; USB
* `3`; CW
* `6`; DATA A (AFSK A)
* `7`; CW-REV
* `9`; DATA B (FSK D / PSK D).

See [`OperatingMode`] for details." =>
    GetVfoOperatingMode
);

define_cat_command!("Set the current VFO's operating mode (`MD`).

# Command format

> `MD{n};`

Where *n* is one of:

* `1`; LSB
* `2`; USB
* `3`; CW
* `6`; DATA A (AFSK A)
* `7`; CW-REV
* `9`; DATA B (FSK D / PSK D).

See [`OperatingMode`] for details." =>
    SetVfoOperatingMode {
        mode: OperatingMode
    }
);

define_command_enum!(
    "Operating mode for [`GetVfoOperatingMode`] and [`SetVfoOperatingMode`] commands." =>
    OperatingMode {
        LowerSideBand = b'1',
        UpperSideBand = b'2',
        ContinuousWave = b'3',
        DataA = b'6',
        ContinuousWaveReverse = b'7',
        DataB = b'9'
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetK2CommandMode => b"K2");
impl_cat_command_with_response!(GetK2CommandMode => try_from 1 K2CommandMode);

impl_cat_command!(SetK2CommandMode => b"K2" with Some |cmd: &SetK2CommandMode| {
    vec![
            b'0' + ((cmd.mode.extended as u8) << 1) + (cmd.mode.rtty_off as u8),
        ]
});

impl Display for K2CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.extended, self.rtty_off) {
            (false, false) => "K2 Normal Mode",
            (false, true) => "K2 Normal/RTTY off",
            (true, false) => "K2 Extended Mode",
            (true, true) => "K2 Extended/RTTY off",
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K2CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K2CommandMode: expecting 1 byte, given {}", value.len());
            Err(invalid_response_length(1, value.len()))
        } else {
            Ok(Self {
                extended: value[0] & 0b0010 != 0,
                rtty_off: value[0] & 0b0001 != 0,
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAutoInfoMode => b"AI");
impl_cat_command_with_response!(GetAutoInfoMode => try_from enum AutoInfoMode);

impl_cat_command!(SetAutoInfoMode => b"AI" for as byte mode);

impl SetAutoInfoMode {
    pub const fn off() -> Self {
        Self {
            mode: AutoInfoMode::Off,
        }
    }
    pub const fn for_frequency_and_mode_related() -> Self {
        Self {
            mode: AutoInfoMode::OnlyFrequencyAndModeRelated,
        }
    }
    pub const fn for_any_front_panel() -> Self {
        Self {
            mode: AutoInfoMode::AnyFrontPanel,
        }
    }
    pub const fn for_compatibility_combination() -> Self {
        Self {
            mode: AutoInfoMode::CompatibilityCombination,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAntennaSelection => b"AN");
impl_cat_command_with_response!(GetAntennaSelection => try_from enum SelectedAntenna);

impl_cat_command!(SetAntennaSelection => b"AN" for as byte antenna);

impl SetAntennaSelection {
    pub const fn antenna_1() -> Self {
        Self {
            antenna: SelectedAntenna::Antenna1,
        }
    }
    pub const fn antenna_2() -> Self {
        Self {
            antenna: SelectedAntenna::Antenna2,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBargraphValue => b"BG");
impl_cat_command_with_response!(GetBargraphValue => 2, |bytes: &[u8]| {
    let value = u8_from_ascii(&bytes)?;
    match value {
        0..=10 => Ok(BargraphValue {
            value,
            mode: BargraphMode::Dot,
        }),
        12..=22 => Ok(BargraphValue {
            value: value - 12,
            mode: BargraphMode::Bar,
        }),
        _ => {
            Err(invalid_response_data(bytes))
        }
    }
} => BargraphValue);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(MoveVfoAFrequencyDown => b"DN" with Some |cmd: &MoveVfoAFrequencyDown| {
    if let Some(step) = cmd.step {
        vec![step as u8]
    } else {
        vec![]
    }
});

impl Default for MoveVfoAFrequencyDown {
    fn default() -> Self {
        Self { step: None }
    }
}

impl From<VfoFrequencyChangeStep> for Frequency {
    fn from(step: VfoFrequencyChangeStep) -> Self {
        Frequency(match step {
            VfoFrequencyChangeStep::Step10Hz => 10,
            VfoFrequencyChangeStep::Step20Hz => 20,
            VfoFrequencyChangeStep::Step50Hz => 50,
            VfoFrequencyChangeStep::Step1kHz => 1000,
        })
    }
}

impl MoveVfoAFrequencyDown {
    #[inline(always)]
    pub const fn step_by(step: VfoFrequencyChangeStep) -> Self {
        Self { step: Some(step) }
    }
    #[inline(always)]
    pub const fn step_10hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step10Hz)
    }
    #[inline(always)]
    pub const fn step_20hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step20Hz)
    }
    #[inline(always)]
    pub const fn step_50hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step50Hz)
    }
    #[inline(always)]
    pub const fn step_1khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1kHz)
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoADisplayAndIcons => b"DS");
impl_cat_command_with_response!(
    GetVfoADisplayAndIcons => 10, |bytes: &[u8]| {
        let (text, decimal_points) = parse_text_and_decimal_flags(&bytes[0..8])?;
        Ok(VfoADisplayAndIcons {
            text,
            decimal_points,
            annunciator_data: VfoAAnnunciatorData::try_from(bytes[8])?,
            annunciator_flash_data: VfoAAnnunciatorFlashData::try_from(bytes[9])?,
        })

    } => VfoADisplayAndIcons
);

pub(crate) fn parse_text_and_decimal_flags(bytes: &[u8]) -> Result<(String, [bool; 8]), RigError> {
    if bytes.len() != 8 {
        error!(
            "VfoADisplayAndIcons: expected 8 bytes of text, got {}",
            bytes.len()
        );
        Err(invalid_response_data(bytes))
    } else if bytes.iter().any(|&b| b < 0x30) {
        error!("VfoADisplayAndIcons: text part contains non-ASCII bytes");
        Err(invalid_response_data(bytes))
    } else {
        let (text, decimal_points) = bytes.iter().enumerate().fold(
            (Vec::with_capacity(8), [false; 8]),
            |(mut text, mut decimal_points), (i, &b)| {
                if b & 0b1000_0000 != 0 {
                    decimal_points[i] = true;
                    text.push(b & 0b0111_1111);
                } else {
                    text.push(b);
                }
                (text, decimal_points)
            },
        );
        Ok((string_from_ascii(&text)?, decimal_points))
    }
}

impl TryFrom<u8> for VfoAAnnunciatorData {
    type Error = RigError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if parse_bit_flag!(value[7] OFF) {
            Err(invalid_response_data(&[value]))
        } else {
            Ok(Self {
                xit_on: parse_bit_flag!(value[0] ON),
                rit_on: parse_bit_flag!(value[1] ON),
                vfo_a_selected: parse_bit_flag!(value[2] ON),
                attenuator_on: parse_bit_flag!(value[3] ON),
                preamp_on: parse_bit_flag!(value[4] ON),
                antenna_2_selected: parse_bit_flag!(value[5] ON),
                noise_blanker_on: parse_bit_flag!(value[6] ON),
            })
        }
    }
}

impl TryFrom<u8> for VfoAAnnunciatorFlashData {
    type Error = RigError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if parse_bit_flag!(value[7] OFF) {
            Err(invalid_response_data(&[value]))
        } else {
            Ok(Self {
                xit_flashing: parse_bit_flag!(value[0] ON),
                rit_flashing: parse_bit_flag!(value[1] ON),
                active_vfo_flashing: parse_bit_flag!(value[2] ON),
                attenuator_flashing: parse_bit_flag!(value[3] ON),
                preamp_flashing: parse_bit_flag!(value[4] ON),
                antenna_2_flashing: parse_bit_flag!(value[5] ON),
                noise_blanker_flashing: parse_bit_flag!(value[6] ON),
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransceiverId => b"ID");
impl_cat_command_with_response!(GetTransceiverId => string);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAOperatingFrequency => b"FA");
impl_cat_command_with_response!(GetVfoAOperatingFrequency => try_from 11 Frequency);

impl_cat_command!(SetVfoAOperatingFrequency => b"FA" with Some |cmd: &SetVfoAOperatingFrequency| {
    cmd.frequency.into()
});
impl_cat_command_with_response!(SetVfoAOperatingFrequency => try_from 11 Frequency);

impl From<Frequency> for SetVfoAOperatingFrequency {
    fn from(frequency: Frequency) -> Self {
        Self { frequency }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoBOperatingFrequency => b"FB");
impl_cat_command_with_response!(GetVfoBOperatingFrequency => try_from 11 Frequency);

impl_cat_command!(SetVfoBOperatingFrequency => b"FB" with Some |cmd: &SetVfoBOperatingFrequency| {
    cmd.frequency.into()
});
impl_cat_command_with_response!(SetVfoBOperatingFrequency => try_from 11 Frequency);

impl From<Frequency> for SetVfoBOperatingFrequency {
    fn from(frequency: Frequency) -> Self {
        Self { frequency }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAAfGain => b"AG0");
impl_cat_command_with_response!(GetVfoAAfGain => try_from 1 Level);

impl_cat_command!(SetVfoAAfGain => b"AG0" with Some |cmd: &SetVfoAAfGain| {
    vec![u8::from(cmd.level)]
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoBAfGain => b"AG1");
impl_cat_command_with_response!(GetVfoBAfGain => try_from 1 Level);

impl_cat_command!(SetVfoBAfGain => b"AG1" with Some |cmd: &SetVfoBAfGain| {
    vec![u8::from(cmd.level)]
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SetReceiveTransmitVfo => b"FR" for as byte vfo);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SetTransmitVfo => b"FT" for as byte vfo);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFilterBandwidth => b"BW");
impl_cat_command_with_response!(GetFilterBandwidth => 4, u16_from_ascii => u16);

impl_cat_command!(SetFilterBandwidth => b"BW" format bandwidth_hz uint 4);

impl_cat_command!(GetFilterBandwidthExtended => b"BW");
impl_cat_command_with_response!(
    GetFilterBandwidthExtended =>
    6, filter_bandwidth_extended_from_ascii =>
    FilterBandwidthAndNumber
);

impl_cat_command!(SetFilterBandwidthExtended => b"BW" with Some format_bandwidth_and_number);

fn filter_bandwidth_extended_from_ascii(
    bytes: &[u8],
) -> Result<FilterBandwidthAndNumber, RigError> {
    Ok(FilterBandwidthAndNumber {
        bandwidth_hz: u16_from_ascii(&bytes[0..4])?,
        filter_number: FilterNumber::from_repr(bytes[4]).ok_or_else(|| {
            error!(
                "FilterBandwidthAndNumber: invalid filter number byte: {}",
                bytes[4]
            );
            invalid_response_data(bytes)
        })?,
        audio_filter_mode: AudioFilterMode::from_repr(bytes[5]).ok_or_else(|| {
            error!(
                "FilterBandwidthAndNumber: invalid audio filter mode byte: {}",
                bytes[5]
            );
            invalid_response_data(bytes)
        })?,
    })
}

fn format_bandwidth_and_number(cmd: &SetFilterBandwidthExtended) -> Vec<u8> {
    format!("{:04}{}", cmd.bandwidth_hz, cmd.filter_number as u8).into_bytes()
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransceiverInformation => b"IF");
impl_cat_command_with_response!(GetTransceiverInformation => try_from 35 TransceiverInformation);

impl TryFrom<&[u8]> for TransceiverInformation {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let operating_frequency = Frequency::try_from(&value[0..11])?;
        let rit_xit_sign_negative = value[16] == b'-';
        let rit_xit_offset_hz = u16_from_ascii(&value[17..21])?;
        let rit_on = bool_from_ascii_1_0(value[21])?;
        let xit_on = bool_from_ascii_1_0(value[22])?;
        let in_transmit_mode = bool_from_ascii_1_0(value[26])?;
        let operating_mode = OperatingMode::from_repr(value[27]).ok_or_else(|| {
            error!(
                "TransceiverInformation: invalid operating mode byte: {}",
                value[27]
            );
            invalid_response_data(value)
        })?;
        let receive_mode_vfo = Vfo::from_repr(value[28]).ok_or_else(|| {
            error!(
                "TransceiverInformation: invalid receive-mode VFO byte: {}",
                value[28]
            );
            invalid_response_data(value)
        })?;
        let scan_in_progress = bool_from_ascii_1_0(value[29])?;
        let in_split_mode = bool_from_ascii_1_0(value[30])?;
        let event_on_band_change = bool_from_ascii_1_0(value[31])?;
        assert_all_bytes_eq(&value[11..16], b' ')?;
        assert_byte_eq(value[23], b' ')?;
        assert_all_bytes_eq(&value[24..26], b'0')?;
        assert_byte_eq(value[32], b'0')?;
        assert_byte_eq(value[33], b'1')?;
        assert_byte_eq(value[34], b' ')?;
        Ok(Self {
            operating_frequency,
            rit_xit_sign_negative,
            rit_xit_offset_hz,
            rit_on,
            xit_on,
            in_transmit_mode,
            operating_mode,
            receive_mode_vfo,
            scan_in_progress,
            in_split_mode,
            event_on_band_change,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoLockState => b"LK");
impl_cat_command_with_response!(GetVfoLockState => boolean);

impl_cat_command!(SetVfoLockState => b"LK" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoOperatingMode => b"MD");
impl_cat_command_with_response!(GetVfoOperatingMode => try_from enum OperatingMode);

impl_cat_command!(SetVfoOperatingMode => b"MD" for as byte mode);

impl_set_cat_command_from_enum!(SetVfoOperatingMode, OperatingMode => mode {
    LowerSideBand => to_lower_sideband,
    UpperSideBand => to_upper_sideband,
    ContinuousWave => to_cw,
    DataA => to_data_a,
    ContinuousWaveReverse => to_cw_reverse,
    DataB => to_data_b
});
