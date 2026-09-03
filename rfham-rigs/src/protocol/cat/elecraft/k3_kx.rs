//!
//! CAT commands for the Elecraft K3, K3S, KX2, and KX3 transceivers.
//!
//! Covers auto-info mode, ATU network readback, audio/RF gain and filter controls, VFO and band
//! selection, RIT/XIT, keyer and CW text, menu access, meter readback, installed-option and
//! status queries, and PTT control.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! As the K3/KX family builds upon the K2 lineage, most K2 commands are also supported. Commands
//! in this module are either specific to one of the K3/KX models, or are K2 commands that have been
//! extended or modified in the K3/KX family. For this reason the feature `k3-kx` is dependent on
//! `k2-kio2` rather than re-exporting the [`super::k2`] module commands.
//!
//! # Notes
//!
//! For consistency the command `GetOperatingFrequency` which is documented as a single logical
//! command but with two command IDs for VFO A and B, is therefore separated into
//! `GetVfoAOperatingFrequency` and `GetVfoBOperatingFrequency`.
//!
//! Similarly `GetOperatingMode` becomes `GetVfoAOperatingMode` and `GetVfoBOperatingMode`.
//!
//! # References
//!
//! 1. [ElecraftK3S/K3/KX3/KX2 Programmer's Reference, rev. G5](https://ftp.elecraft.com/K3S/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G5.pdf), Feb 2019.
//!

use crate::{
    error::{RigError, enum_parse, invalid_response_data, invalid_response_length},
    protocol::{
        Frequency,
        cat::{
            common::{
                assert_all_bytes_eq, assert_byte_eq, bool_from_ascii_1_0, bytes_to_vec,
                format_uint_ascii, sign_from_ascii_loose, u8_from_ascii, u16_from_ascii,
                validate_integer_in_range,
            },
            elecraft::{Vfo, k2::parse_text_and_decimal_flags},
        },
    },
};
use core::fmt::Display;
use rfham_itu::allocations::AllocationBand;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAAfGain, SetVfoAAfGain
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetK3CommandMode, SetK3CommandMode, K3CommandMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the K3 meta-command mode.

Software should set K31 immediately after connecting to enable full functionality.

# Command format

> `K3;`

# Response format

> `K3{n};`

Where *n* is:

* `0`; Normal RSP format, the default after power-on
* `1`; Extended RSP format, K31 mode, enables extra fields in IF, etc." =>
    GetK3CommandMode
);

define_cat_command!("Set the K3 meta-command mode.

Software should set K31 immediately after connecting to enable full functionality.

# Command format

> `K3{n};`

Where *n* is:

* `0`; Normal RSP format, the default after power-on
* `1`; Extended RSP format, K31 mode, enables extra fields in IF, etc." =>
    SetK3CommandMode {
        mode: K3CommandMode
    }
);

define_command_struct!(
    "Represents the parsed K3 command-mode response." =>
    K3CommandMode {
        "`true` for extended RSP format, `false` for normal RSP format." =>
        extended: bool
    }
);

define_cat_command!("Get Audio Frequency (AF) gain level for VFO A (`AG`).

# Command format

> `AG;`

# Response format

> `AG{nnn};`

Where *nnn* is the AF gain level, between `000` and `255`." =>
    GetVfoAAfGain
);

define_cat_command!("Set Audio Frequency (AF) gain level for VFO A (`AG`).

The AF gain value is stored separately for phones and speaker.
AF gain can be incremented/decremented using the ENAU/ENAD commands.

# Command format

> `AG{nnn};`

Where *nnn* is the AF gain level, between `000` and `255`." =>
    SetVfoAAfGain {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoBAfGain, SetVfoBAfGain
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Audio Frequency (AF) gain level for VFO B (`AG$`).

# Command format

> `AG$;`

# Response format

> `AG${nnn};`

Where *nnn* is the AF gain level, between `000` and `255`." =>
    GetVfoBAfGain
);

define_cat_command!("Set Audio Frequency (AF) gain level for VFO B (`AG$`).

The AF gain value is stored separately for phones and speaker.
AF gain can be incremented/decremented using the ENAU/ENAD commands.

# Command format

> `AG${nnn};`

Where *nnn* is the AF gain level, between `000` and `255`." =>
    SetVfoBAfGain {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAutoInfoMode, SetAutoInfoMode, AutoInfoMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Auto-Information (AI) mode (`AI`).

In AI mode the transceiver spontaneously sends RSPs whenever state changes.

# Command format

> `AI;`

# Response format

> `AI{n};`

Where *n* is a value between `0` and `3`; see Meta-commands for details.

**Note**: The *AI* power-up default is normally `AI0`, corresponding to K3 menu setting
*CONFIG:AUTOINF* = `NOR`. *AUTOINF* can also be set to **AUTO1**, which makes the default `AI1` on
power-up. This is useful for K3s controlling a StepIR antenna, etc." =>
    GetAutoInfoMode
);

define_cat_command!("Set Auto-Information (AI) mode (`AI`).

In AI mode the transceiver spontaneously sends RSPs whenever state changes. See Meta-commands for
details.

# Command format

> `AI{n};`

Where *n* is a value between `0` and `3`; see Meta-commands for details.

**Note**: The *AI* power-up default is normally `AI0`, corresponding to K3 menu setting
*CONFIG:AUTOINF* = `NOR`. *AUTOINF* can also be set to **AUTO1**, which makes the default `AI1` on
power-up. This is useful for K3s controlling a StepIR antenna, etc." =>
    SetAutoInfoMode {
        mode: AutoInfoMode
    }
);

define_command_enum!(
    "Auto-information mode." =>
    AutoInfoMode {
        "AI off (manual query only)" => Off = b'0',
        "AI on for K2 responses (basic unsolicited RSP)" => K2 = b'1',
        "AI on for K3 responses" => K3 = b'2',
        "AI on for K3 responses in extended mode" => K3Extended = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuNetworkValues, AtuNetworkValues
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get ATU (antenna tuner) network values (`AK`).

KX3/KX2 only.

# Command format

> `AK;`

# Response format

> `AK{aa}{bb}{cc};`

Where: 

* *aa* = inductance IO bitmap (ASCII hex)
* *bb* = capacitance bitmap
* *cc* = misc relays bitmap

The *aa* and *bb* bitmaps can be equated to *L* and *C* values by looking at the KXAT3 or KXAT2
schematic. For example, a value of `01` would represent the smallest *L* or *C* value in the 
network. At present only bit `0` of byte *cc* is defined: 

* `00` = capacitors on the antenna side
* `01` = capacitors on the transmit side 

If the ATU is not installed or is in one of the Lx/Cx test settings, `AK000000;` is returned. In 
BYP mode, on some bands *L* and *C* are set to fixed non-zero values in order to cancel the ATU's
own reactance when working into a 50-ohm load. In AUTO mode, the working auto-tuned values are
shown." =>
    GetAtuNetworkValues
);

define_command_struct!(
    "ATU network values from the `AK` response (KX3/KX2 only, GET only)." =>
    AtuNetworkValues {
        "Inductance IO bitmap (ASCII hex)." => inductance_bitmap: u8,
        "Capacitance bitmap (ASCII hex)." => capacitance_bitmap: u8,
        "Misc, relays bitmap (ASCII hex)." => misc_relay_bitmap: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAudioPeakingFilterState, SetAudioPeakingFilterState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get CW Audio Peaking Filter (APF) on/off state (`AP`).

Applies to CW mode only, and only if *CONFIG:DUAL PB* is set to **APF**.

# Command format

> `AP;`

# Response format

> `AP{n};`

Where *n* is the boolean state `0` (off) or `1` (on)." =>
    GetAudioPeakingFilterState
);

define_cat_command!("Set CW Audio Peaking Filter (APF) on/off state.

Applies to CW mode only, and only if *CONFIG:DUAL PB* is set to **APF**.

# Command format

> `AP{n};`

Where *n* is the boolean state `0` (off) or `1` (on)." =>
    SetAudioPeakingFilterState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetReceiveAntenna, SetReceiveAntenna
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the Receive-only antenna state (`AR`).

K3/K3S only.

# Command format

> `AR;`

# Response format

> `AR{n};`

Where `n` is one of:

* `0`; use Transmit antenna.
* `1`; use Receive-only antenna." =>
    GetReceiveAntenna
);

define_cat_command!("Set Receive-only antenna (K3/K3S only).

# Command format

> `AR{n};`

Where *n* is one of:

* `0`; use Transmit antenna.
* `1`; use Receive-only antenna." =>
    SetReceiveAntenna {
        rx_only: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBargraph
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the bargraph meter value (`BG`).

Returns S-meter level in receive (also see `SM`/`SM$` command), and power or ALC level in transmit.

# Command format

> `BG;`

# Response format

> `BG{nn}{x};`

Where: 

* *nn* describes which bars are turned on
* *x* indicates receive `R` or transmit `T` K3 only.

**Notes**:

1. On the K3 and K3S, only, transmit metering mode can be set remotely using the `TM` command.
   CWT and CMP readings not yet available. The numeric value of SWR can be read using SW.
2. K3, Receive: *nn* is `00` - `21` (CWT off) or `00` - `09` (CWT on).
3. K3, Transmit: *nn* is `00` - `12` (PWR) or `00` - `07` (ALC) depending on **METER** setting.
   Also see `TM` command.
4. K2, Receive or Transmit: *nn* is `00` - `10` (**DOT** mode) or `12` - `22` (**BAR** mode)." =>
    GetBargraphValue
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandNumberA, GetBandNumberB, SetBandNumberA, SetBandNumberB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current band number for VFO A (`BN`).

# Command format

> `BN;`

# Response format

> `BN{nn};`

Where *nn* is a two-digit decimal band code:

* `00`; 160 m
* `01`; 80 m
* `02`; 60 m
* `03`; 40 m
* `04`; 30 m
* `05`; 20 m
* `06`; 17 m
* `07`; 15 m
* `08`; 12 m
* `09`; 10 m
* `10`; 6 m
* `11`; 2 m (XVTR)
* `12`; 222 MHz
* `13`; 432 MHz
* `14`; 1.25 cm
* `18`; General coverage (not representable as an [`AllocationBand`])." =>
    GetVfoABandNumber
);

define_cat_command!("Get the current band number for VFO B (`BN$`).

# Command format

> `BN$;`

# Response format

> `BN${nn};`

Where *nn* is a two-digit decimal band code:

* `00`; 160 m
* `01`; 80 m
* `02`; 60 m
* `03`; 40 m
* `04`; 30 m
* `05`; 20 m
* `06`; 17 m
* `07`; 15 m
* `08`; 12 m
* `09`; 10 m
* `10`; 6 m
* `11`; 2 m (XVTR)
* `12`; 222 MHz
* `13`; 432 MHz
* `14`; 1.25 cm
* `18`; General coverage (not representable as an [`AllocationBand`])." =>
    GetVfoBBandNumber
);

define_cat_command!("Set the current band number for VFO A (`BN`).

# Command format

> `BN{nn};`

Where *nn* is a two-digit decimal band code:

* `00`; 160 m
* `01`; 80 m
* `02`; 60 m
* `03`; 40 m
* `04`; 30 m
* `05`; 20 m
* `06`; 17 m
* `07`; 15 m
* `08`; 12 m
* `09`; 10 m
* `10`; 6 m
* `11`; 2 m (XVTR)
* `12`; 222 MHz
* `13`; 432 MHz
* `14`; 1.25 cm
* `18`; General coverage (not representable as an [`AllocationBand`])." =>
    SetVfoABandNumber {
        band: AllocationBand
    }
);

define_cat_command!("Set the current band number for VFO B (`BN$`).

# Command format

> `BN${nn};`

Where *nn* is a two-digit decimal band code:

* `00`; 160 m
* `01`; 80 m
* `02`; 60 m
* `03`; 40 m
* `04`; 30 m
* `05`; 20 m
* `06`; 17 m
* `07`; 15 m
* `08`; 12 m
* `09`; 10 m
* `10`; 6 m
* `11`; 2 m (XVTR)
* `12`; 222 MHz
* `13`; 432 MHz
* `14`; 1.25 cm
* `18`; General coverage (not representable as an [`AllocationBand`])." =>
    SetVfoBBandNumber {
        band: AllocationBand
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetBaudRate, BaudRate
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set RS-232 baud rate (`BR`).

The new rate takes effect after the command is acknowledged.

K3/K3S only.

# Command format

> `BR{n};`

Where *n* is one of:

* `0`; 4,800
* `1`; 9,600
* `2`; 19,200
* `3`; 38,400
* `4`; 57,600
* `5`; 115,200 

**Note**: The K3 firmware download utility automatically sets the K3 to 38400 baud for downloads,
then restores the baud rate to the user's selection (made using the K3's *CONFIG:RS232* menu
entry)." =>
    SetBaudRate {
        rate: BaudRate
    }
);

define_command_enum!(
    "RS-232 baud rate (K3/K3S only)." =>
    BaudRate {
        "4,800 baud" => Rate4800 = b'0',
        "9,600 baud" => Rate9600 = b'1',
        "19,200 baud" => Rate19200 = b'2',
        "38,400 baud" => Rate38400 = b'3',
        "57,600 baud" => Rate57600 = b'4',
        "115,200 baud" => Rate115200 = b'5'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFilterBandwidthA, GetFilterBandwidthB, SetFilterBandwidthA, SetFilterBandwidthB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get active filter bandwidth for VFO A, in units of 10 Hz (`BW`).

Prefer this command over the legacy [`GetVfoALegacyFilterBandwidth`] (`FW`) command.

# Command format

> `BW;`

# Response format

> `BW{nnnn};`

Where *nnnn* is a value between `0000` and `9999`, the bandwidth in 10-Hz units. May be quantized
and/or range limited based on the present operating mode.

**Notes**:

1. `BW` is a dervative of the legacy `FW` command. `BW` is safer to use in switch macros, because it
   makes no assumptions about meta-command settings (K2x and K3x). `FW` may be preferred in
   applications.
2. In diversity mode, `BW` matches the sub receiver's filter bandwidth to the main receiver's.
3. Both `BW` and `BW$` can be used in BSET mode (one exception: at present, `BW`/`BW$` SET can't be
   used in BSET mode with diversity receive in effect).
4. If a KX3/KX2 is in DUAL RX (dual watch) mode, `BW$` returns the value for `BW`." =>
    GetVfoAFilterBandwidth
);

define_cat_command!("Get active filter bandwidth for VFO B, in units of 10 Hz (`BW$`).

Prefer this command over the legacy [`GetVfoBLegacyFilterBandwidth`] (`FW`) command.

# Command format

> `BW$;`

# Response format

> `BW${nnnn};`

Where *nnnn* is a value between `0000` and `9999`, the bandwidth in 10-Hz units. May be quantized
and/or range limited based on the present operating mode.

**Notes**:

1. Both `BW` and `BW$` can be used in BSET mode (one exception: at present, `BW`/`BW$` SET can't be
   used in BSET mode with diversity receive in effect).
2. If a KX3/KX2 is in DUAL RX (dual watch) mode, `BW$` returns the value for `BW`." =>
    GetVfoBFilterBandwidth
);

define_cat_command!("Set active filter bandwidth for VFO A, in units of 10 Hz (`BW`).

Prefer this command over the legacy [`GetVfoALegacyFilterBandwidth`] (`FW`) command.

# Command format

> `BW{nnnn};`

Where *nnnn* is a value between `0000` and `9999`, the bandwidth in 10-Hz units. May be quantized
and/or range limited based on the present operating mode.

**Notes**:

1. `BW` is a dervative of the legacy `FW` command. `BW` is safer to use in switch macros, because it
   makes no assumptions about meta-command settings (K2x and K3x). `FW` may be preferred in
   applications.
2. In diversity mode, `BW` matches the sub receiver's filter bandwidth to the main receiver's.
3. Both `BW` and `BW$` can be used in BSET mode (one exception: at present, `BW`/`BW$` SET can't be
   used in BSET mode with diversity receive in effect).
4. If a KX3/KX2 is in DUAL RX (dual watch) mode, `BW$` returns the value for `BW`." =>
    SetVfoAFilterBandwidth {
        bandwidth_10hz: u16
    }
);

define_cat_command!("Set active filter bandwidth for VFO B, in units of 10 Hz (`BW$`).

Prefer this command over the legacy [`GetVfoBLegacyFilterBandwidth`] (`FW`) command.

# Command format

> `BW${nnnn};`

Where *nnnn* is a value between `0000` and `9999`, the bandwidth in 10-Hz units. May be quantized
and/or range limited based on the present operating mode.

**Notes**:

1. Both `BW` and `BW$` can be used in BSET mode (one exception: at present, `BW`/`BW$` SET can't be
   used in BSET mode with diversity receive in effect).
2. If a KX3/KX2 is in DUAL RX (dual watch) mode, `BW$` returns the value for `BW`." =>
    SetVfoBFilterBandwidth {
        bandwidth_10hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSpeechCompression, SetSpeechCompression
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get speech compression level (`CP`).

# Command format

> `CP;`

# Response format

> `CP{nnn};`

Where *nn* is a percentage value between `000` (off) and `040`." =>
    GetSpeechCompression
);

define_cat_command!("Set speech compression level (`CP`).

# Command format

> `CP{nnn};`

Where *nn* is a percentage value between `000` (off) and `040`." =>
    SetSpeechCompression {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCwSidetonePitch
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get CW sidetone pitch in Hz (`CW`).

The pitch also determines the RIT/XIT operating offset for zero-beat CW tuning.

# Command format

> `CW;`

# Response format

> `CW{nn};`

Where *nn* is a value between `30` and `80` Hz." =>
    GetCwSidetonePitch
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoBDisplayText, SetVfoBDisplayText
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get/Read VFO B display text (`DB`).

Returns text displayed on VFO B, including decimal points and colons if present.

VFO B normally displays only uppercase alphabetic characters. DB returns the following lower-case
characters that represent symbols: **a** (antenna), **b** (mu), **c** (slashed 0), **d** (itself),
**e** (sigma),**f** (<-), **g** (->), **h** (II), **i** (left-justified '1'), **j** (delta, large),
**k** (delta, small), **l** (right-justified '1'), **m** (superscript 'm'), **n** (lowercase 'w').

# Command format

> `DB;`

# Response format

> `DB{ss..};`

Where *ss* is the ASCII character display contents." =>
    GetVfoBDisplayText
);

define_cat_command!("Set/Write VFO B display text (`DB`).

# Command format One

> `DB{c};`

Where *c* is an ASCII character to send to VFO B, entering at the right end of the display and
scrolling left as additional characters are entered.

This can be used to create scrolling messages to alert the operator to something regarding the
computer, send extended help text, insert a newsfeed, report a DX spot, test special characters,
etc.

# Command format Two

> `DB{nn};`

Where *nn* is one of the available VFO B alternate display modes:

**K3**: `00`=normal, `01`=time, `02`=date, `03`=RIT/XIT offset, `04`=supply voltage, `05`=supply
current, `06`=PA heatsink temp, `07`=front panel temp, `08`=PLL1 voltage, `09`=PLL2 voltage,
`10`=AFV, `11`=dBV. *Note*: modes `08` and higher require *CONFIG:TECH MD*=**ON**.

**KX3**: `00`=normal, `01`=time, `02`=supply voltage, `03`=battery voltage; if KXBC3 installed,
`04`=supply current, `05`=PA temp (PA.I=KX3, PA.X=KXPA100), `06`=OSC temp, `07`=AFV, `08`=dBV.

**KX2**: `00`=normal, `01`=time, `02`=supply or battery voltage, `03`=N/A, `04`=supply current,
`05`=PA temp (PA.I=KX2, PA.X=KXPA100), `06`=N/A (TBD: OSC temp), `07`=AFV, `08`=dBV, `09`=amp hours.
**Note**: Amp-hours display is `X.XXX AH`. There's also an *AMP HRS* menu entry that shows the same
value. CLR can be used from within this menu entry to reset the value to 0." =>
    SetVfoBDisplayText {
        text: [u8; 8]
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetCommandDelay
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set command processing delay in units of 10 ms (`DE`).

Slows down command processing for software that cannot keep up with responses.

# Command format

> `DE{nnn};`

Where *nnn* is a value between `001` and `255` the delay value in 10-ms increments.

This is useful in switch or K-pod macros, where a delay may be desired to allow the radio to 
complete a previous operation before the next command is processed. Note: `DE001` may result in a
delay shorter than 10 ms, while `DE002` is guaranteed to provide a delay between 10 and 20 ms, etc.

**Note**: K3/K3S only." =>
    SetCommandProcessingDelay {
        delay_5ms: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetDspCommandDebugState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set DSP command debugging on/off state (`DL`).

# Command format

> `DL{n};`

Where *n* represents a boolean on/off state but with the value `2` turning on debuggin and `3`
turning it off.

When on, all commands sent from the MCU to the DSP are echoed to the K3's serial port, with a few
exceptions such as during program loading. The DVR icon will flash as a reminder.

**Note**: K3/K3S only." =>
    SetDspCommandDebugState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: VfoADFrequencyown, VfoBFrequencyDown
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move VFO A down by one tuning step (`DN`).

# Command format

> `DN;` or `DN{n};`

Where *n* is an optional VFO change specification  `0`=1 Hz; `1` or not used=10 Hz; `2`=20 Hz;
`3`=50 Hz; `4`=1 kHz; `5`=2 kHz; `6`=3 kHz; `7`=5 kHz; `8`=100 Hz; `9`=200 Hz.

Note: If the VFOs are linked (non-SPLIT), `DN`; and `DN{n}`; set VFO B to the same frequency as VFO A." =>
    MoveVfoAFrequencyDown {
        step: Option<VfoFrequencyChangeStep>
    }
);

define_cat_command!("Move VFO B down by one tuning step (`DNB`).

# Command format

> `DNB;` or `DNB{n};`

Where *n* is an optional VFO change specification  `0`=1 Hz; `1` or not used=10 Hz; `2`=20 Hz;
`3`=50 Hz; `4`=1 kHz; `5`=2 kHz; `6`=3 kHz; `7`=5 kHz; `8`=100 Hz; `9`=200 Hz.

Note: If the VFOs are linked (non-SPLIT), `DN`; and `DN{n}`; set VFO B to the same frequency as VFO A." =>
    MoveVfoBFrequencyDown {
        step: Option<VfoFrequencyChangeStep>
    }
);

define_command_enum!(
    "Represents a VFO frequency change step size." =>
    VfoFrequencyChangeStep {
        "1 Hz" => Step1Hz = b'0',
        "10 Hz" => Step10Hz = b'1',
        "20 Hz" => Step20Hz = b'2',
        "50 Hz" => Step50Hz = b'3',
        "1 kHz" => Step1kHz = b'4',
        "2 kHz" => Step2kHz = b'5',
        "3 kHz" => Step3kHz = b'6',
        "5 kHz" => Step5kHz = b'7',
        "100 Hz" => Step100Hz = b'8',
        "200 Hz" => Step200Hz = b'9'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoADisplayText, VfoADisplayTextAndIcons, VfoAIconFlashData
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get VFO A display text and icons (`DS`).

Returns everything needed to reproduce the contents of the VFO A display, as well as a basic subset
of the LCDs icons (also see `IC` command, which provides many more status indicators and does not
require that `K31` be in effect).

# Command format

> `DS;`

# Response format

> `DS{tttttttt}{a}{f};`

Where *tttttttt* is the LCD text and decimal point data, *a* is icon data, and *f* is icon flash
data (all `0` for the K3), or additional K3 icon data. These fields are detailed below.

**TEXT and decimal point data**: This field contains 8 bytes, with values `0x30` - `0xFF` (hex). The
first byte is the left-most displayed character. Bit 7 (MSB) of each byte indicates whether the
decimal point to the *left* of each character is on (`1`) or off (`0`). The other bits contain an
ASCII character that corresponds to the displayed character.

> *Note*: K2 decimal point flash status can be obtained directly; use `LK` for VFO lock, `IF` for
> scan on/off, and `GT` for AGC on/off.*

Some ASCII characters (e.g., 'X', 'M') cannot be shown on VFO A, which uses a 7-segment display. The
K3 uses such characters as placeholders for special characters that can be displayed -- in some
cases lowercase versions of uppercase letters -- to enhance display readability. For this reason,
the characters returned by the DS command must sometimes be converted to other characters by the
software application. Table 3 shows all possible conversions, some not used. The table assumes the
decimal-point flag (bit 7) is 0. The menu parameters for *MAIN:RX EQ / TX EQ* consist of 8 'mini
bar-graphs' with 5 possible 'levels.' These show up as the following characters in the DS response
string (level 1 through 5): '_', '=', '>', ']', and '^'. To see how these should appear in a
graphical application, go into RX EQ and vary one of the EQ bands over its full range.

| DS chr.   | Converts to       | DS chr.   | Converts to   | DS chr.   | Converts to       |
|-----------|-------------------|-----------|---------------|-----------|-------------------|
| `<`       | small-caps `L`    | `M`       | `N`           | `Z`       | lowercase `c`     |
| `>`       | dash              | `Q`       | `O`           | `[`       | r-bar             |
| `@`       | space (blank)     | `V`       | `U`           | `\\`      | lambda            |
| `K`       | `H`               | `W`       | `I`           | `]`       | RX/TX EQ level 4  |
|           |                   | `X`       | c-bar         | `^`       | RX/TX EQ level 5  |

**Icon data**: This field is a single byte whose value is between `0x80` and `0xFF`. Bit 7 is always
1. The other 7 bits indicate the on/off states of 8 icons common to the K2 and K3. The bits are
defined as follows (B7 = `0x80`).

| Bit | Meaning             | Bit | Meaning                                 |
|-----|---------------------|-----|-----------------------------------------|
| 7   | Always `1`          | 3   | `1`=ATT on                              |
| 6   | `1`=NB on*          | 2   | `0`=VFO A selected (always `0` for K3)  |
| 5   | `1`=ANT2 selected   | 1   | `1`=RIT on                              |
| 4   | `1`=PREAMP on       | 0   | `1`=XIT on                              |

**Icon flash data or additional K3 icons**: This field is a single byte whose value is between 
`0x80` and `0xFF`. Bit 7 is always `1`. In K3 normal mode (K30, or K2 emulation), the other 7 bits
are all `0`, since in general the K3 doesn't use flashing icons to indicate state. In K3 Extended
mode (K31), the bits are defined as follows (B7 = `0x80`):

| Bit | Meaning              | Bit | Meaning          |
|-----|----------------------|-----|------------------|
| 7   | Always `1`           | 3   | `1`=CWT on       |
| 6   | `1`=SUB on           | 2   | `1`=NR on        |
| 5   | `1`=RX ANT on        | 1   | `1`=NTCH on      |
| 4   | `1`=ATU on (in-line) | 0   | `1`=MAN NOTCH on |
" =>
    GetVfoADisplayAndIcons
);

define_command_struct!(
    "VFO A display text and icons from the `DS` response." =>
    VfoADisplayAndIcons no_copy {
        "VFO A display text (8 bytes)." => text: String,
        "VFO A display decimal point indicators." => decimal_points: [bool; 8],
        "VFO A icon data (1 byte)." => icon_data: VfoAIconData,
        "VFO A icon flash data or additional K3 icons (1 byte)." => icon_flash_data: VfoAIconFlashData
    }
);

define_command_struct!(
    "VFO A icon data from the `DS` response." =>
    VfoAIconData {
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
    VfoAIconFlashData {
        "SUB is on." => sub_on: bool,
        "RX ANT is on." => receive_antenna_on: bool,
        "ATU is on (in-line)." => atu_on: bool,
        "CWT is on." => cwt_on: bool,
        "NR is on." => noise_reduction_on: bool,
        "NTCH is on." => notch_on: bool,
        "MAN NOTCH is on." => manual_notch_on: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDataSubMode, SetDataSubMode, DataSubMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get DATA sub-mode (`DT`).

Only meaningful when operating in DATA mode (MD6 or MD9).

# Command format

> `DT;`

# Response format

> `DT{n};`

Where *n* is the data sub-mode last used with VFO A, whether or not DATA mode is in effect: `0` 
(DATA A), `1` (AFSK A), `2` (FSK D), or `3` (PSK D). See `MD` for setting data normal/reverse. In
Diversity Mode (K3 only, accessed by sending `DV1` or via a hold of **SUB**), sending `DTn` matches
the sub receiver's mode to the main receiver's. 

**Notes**:

1. Use `DT` only when the transceiver is in DATA mode; otherwise, the returned
2. In AI2/3 modes, changing the data sub-mode results in both `FW` and `IS` responses.
3. The present data sub-mode is also reported as part of the `IF` command, although this requires
   that K31 be in effect. Refer to the `IF` command for details." =>
    GetDataSubMode
);

define_cat_command!("Set DATA sub-mode (`DT`).

Only meaningful when operating in DATA mode (MD6 or MD9).

# Command format

> `DT{n};`

Where *n* is the data sub-mode last used with VFO A, whether or not DATA mode is in effect: `0` 
(DATA A), `1` (AFSK A), `2` (FSK D), or `3` (PSK D). See `MD` for setting data normal/reverse. In
Diversity Mode (K3 only, accessed by sending `DV1` or via a hold of **SUB**), sending `DTn` matches
the sub receiver's mode to the main receiver's. 

**Notes**:

1. Use `DT` only when the transceiver is in DATA mode; otherwise, the returned
2. In AI2/3 modes, changing the data sub-mode results in both `FW` and `IS` responses.
3. The present data sub-mode is also reported as part of the `IF` command, although this requires
   that K31 be in effect. Refer to the `IF` command for details." =>
    SetDataSubMode {
        sub_mode: DataSubMode
    }
);

define_command_enum!(
    "DATA mode sub-type." =>
    DataSubMode {
        "DATA A/AFSK audio" => DataAfsk = b'0',
        "AFSK A" => AfskA = b'1',
        "FSK D/RTTY" => FskD = b'2',
        "PSK D" => PskD = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDiversityMode, SetDiversityMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Diversity receive mode (`DV`).

K3 only, *and* requires the optional KRX3A internal sub-receiver to be installed.

# Command format

> `DV;`

# Response format

> `DV{n};`

Where n is `0` to turn diversity mode OFF, `1` to turn it ON, and `S` to toggle both the 
sub-receiver and diversity on/off together. 

**Note**: Turning the sub off also cancels diversity mode.

**See Also**: LN (VFO A/B link) and (sub receiver on/off)." =>
    GetDiversityMode
);

define_cat_command!("Set Diversity receive mode (`DV`).

K3 only, *and* requires the optional KRX3A internal sub-receiver to be installed.

# Command format

> `DV{n};`

Where n is `0` to turn diversity mode OFF, `1` to turn it ON, and `S` to toggle both the 
sub-receiver and diversity on/off together. 

**Note**: Turning the sub off also cancels diversity mode.

**See Also**: LN (VFO A/B link) and (sub receiver on/off)." =>
    SetDiversityMode { state: DiversityModeState }
);

define_command_enum!(
    "Diversity receive mode state." =>
    DiversityModeState {
        "Off" => Off = b'0',
        "On" => On = b'1',
        "Sub receiver and diversity" => SubAndDiversity = b'S'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetErrorLogging
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set error logging (`EL`) on/off state.

KX3/KX2 only.

# Command format

> `EL{n};`

Where `n` is the boolean state `0` (disable) or `1` (enable)." =>
    SetErrorLogging { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetEssbMode, SetEssbMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Enhanced SSB (ESSB) transmit mode on/off state (`ES`).

ESSB enables a wider TX bandwidth (up to 4 kHz) for improved SSB audio quality.

K3/K3S only.

# Command format

> `ES;`

# Response format

> `ES{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetEssbMode
);

define_cat_command!("Set Enhanced SSB (ESSB) transmit mode on/off state (`ES`).

K3/K3S only.

# Command format

> `ES{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetEssbMode { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetIfCenterFrequency
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Intermediate Frequency (IF) center frequency in Hz (`FI`).

K3 only.

# Command format

> `FI;`

# Response format

> `FI{nnnn};`

Where *nnnn* is the frequency in Hertz between `0000` and `9999` Hz; the center point of IF shift." =>
    GetIfCenterFrequency
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetReceiveVfo, SetReceiveVfo
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get active receive VFO (`FR`).

K2 compatibility command.

# Command format

> `FR;`

# Response format

> `FR{n};`

Where `n` is either `0` (VFO A) or `1` (VFO B)." =>
    GetReceiveVfo
);

define_cat_command!("Set active receive VFO (`FR`).

K2 compatibility command.

# Command format

> `FR{n};`

Where `n` is either `0` (VFO A) or `1` (VFO B). Only [`Vfo::A`] and [`Vfo::B`] are accepted." =>
    SetReceiveVfo {
        vfo: Vfo
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransmitVfoSplitModeState, SetTransmitVfoSplitModeState
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the transmit VFO selection, split mode on/off state (`FT`).

# Command format

> `FT;`

# Response format

> `FT{n};`

Where `n` is one of:

* `0`; VFO A transmits, no split.
* `1`; VFO B transmits, split on." =>
    GetTransmitVfoSplitModeState
);

define_cat_command!("Set the transmit VFO selection, split mode on/off state (`FT`).

# Command format

> `FT{n};`

Where `n` is one of:

* `0`; VFO A transmits, no split.
* `1`; VFO B transmits, split on." =>
    SetTransmitVfoSplitModeState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetLegacyFilterBandwidthA, GetLegacyFilterBandwidthB, SetLegacyFilterBandwidthA,
// SetLegacyFilterBandwidthB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get filter bandwidth in Hz for VFO A via the K2 legacy command (`FW`).

Prefer [`GetVfoAFilterBandwidth`] (`BW`) on K3/KX.

# Command format

> `FW;`

# Response format

> `FW{nnnn};`

Where *nnnn* is frequency in Hertz between `0000` and `9999` Hz." =>
    GetVfoALegacyFilterBandwidth
);

define_cat_command!("Get filter bandwidth in Hz for VFO B via the K2 legacy command(`FW$`).

Prefer [`GetVfoBFilterBandwidth`] (`BW`) on K3/KX.

# Command format

> `FW$;`

# Response format

> `FW${nnnn};`

# Response format

> `FW{nnnn};`

Where *nnnn* is frequency in Hertz between `0000` and `9999` Hz." =>
    GetVfoBLegacyFilterBandwidth
);

define_cat_command!("Set filter bandwidth in Hz for VFO A via the K2 legacy command (`FW`).

Prefer [`SetVfoAFilterBandwidth`] (`BW`) on K3/KX.

# Command format

> `FW{nnnn};`

# Response format

> `FW{nnnn};`

Where *nnnn* is frequency in Hertz between `0000` and `9999` Hz." =>
    SetVfoALegacyFilterBandwidth {
        bandwidth_hz: u16
    }
);

define_cat_command!("Set filter bandwidth in Hz for VFO B via the K2 legacy command (`FW$`).

Prefer [`SetVfoBFilterBandwidth`] (`BW`) on K3/KX.

# Command format

> `FW${nnnn};`

# Response format

> `FW{nnnn};`

Where *nnnn* is frequency in Hertz between `0000` and `9999` Hz." =>
    SetVfoBLegacyFilterBandwidth {
        bandwidth_hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAgcTimeConstant, SetAgcTimeConstant
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get AGC time constant (`GT`).

# Command format

> `GT;`

# Response format

> `GT{nn};`

Where *nn* is one of:

* `00`; off
* `01`; fast
* `02`; slow
* `03`; auto" =>
    GetAgcTimeConstant
);

define_cat_command!("Set AGC time constant (`GT`).

# Command format

> `GT{nn};`

Where *nn* is one of:

* `00`; off
* `01`; fast
* `02`; slow
* `03`; auto" =>
    SetAgcTimeConstant {
        mode: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInstalledOptions, InstalledOptions, K3InstalledOptions, K4InstalledOptions,
// KXInstalledOptions
// ------------------------------------------------------------------------------------------------

// Hand-rolled: the 13-byte option string differs by radio model and does not map onto a single
// enumerable set of values, so this is dispatched manually rather than via `impl_command_with_response!`.

define_cat_command!("Get installed option modules (`OM`).

The 13-byte value differs by radio model; this command dispatches to the appropriate variant via 
[`InstalledOptions`].

# Command format

> `OM;`

# Response format

> `OM␣{options};`

* K3/K3S RSP: `OM␣APXSDFfLVR--;`, see [`K3InstalledOptions`] for details.
* K4 RSP: `OM␣APXSHML14---;`, see [`K4InstalledOptions`] for details.
* KX3 RSP: `OM␣APF---TBXI02;`, see [`KXInstalledOptions`] for details.
* KX2 RSP: `OM␣APF---TBXI01;`, see [`KXInstalledOptions`] for details." =>
    GetInstalledOptions
);

/// Installed hardware options, dispatched to the model-specific variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstalledOptions {
    /// K3/K3S installed options.
    K3(K3InstalledOptions),
    /// K4 installed options.
    K4(K4InstalledOptions),
    /// KX3/KX2 installed options.
    KX(KXInstalledOptions),
}

define_command_struct!(
    "K3/K3S installed option modules.

Option string after leading space: `APXSDFfLVR--`

| Pos | Letter | Module           | Descrption                                        |
|-----|--------|------------------|---------------------------------------------------|
| 0   | `A`    | KAT3A            | Internal ATU                                      |
| 1   | `P`    | KPA3A            | Internal PA                                       |
| 2   | `X`    | KXV3/KXV3A/KXV3B | Transverter / Receiver /  I/O                     |
| 3   | `S`    | KRX3A            | Sub Receiver                                      |
| 4   | `D`    | KDVR3            | DVR                                               |
| 5   | `F`    | KBPF3A           | Band-Pass Filter main                             |
| 6   | `f`    | KBPF3A           | Band-Pass Filter sub                              |
| 7   | `L`    | KXV3B            | Low-Noise Amp on current band (preamp 2)          |
| 8   | `V`    | KSYN3A           | Synthesizer (extends VFO to 100 kHz)              |
| 9   | `R`    | K3S              | RF board; preferred way to distinguish K3 vs K3S  |" =>
    K3InstalledOptions {
        has_kat3a_atu: bool,
        has_kpa3a_pa: bool,
        has_kxv3_xvtr: bool,
        has_krx3a_sub_receiver: bool,
        has_kdvr3_dvr: bool,
        has_kbpf3a_main_bpf: bool,
        has_kbpf3a_sub_bpf: bool,
        has_kxv3b_low_noise_amp: bool,
        has_ksyn3a: bool,
        has_k3s: bool
    }
);

define_command_struct!(
    "K4 installed option modules.

Option string after leading space: `APXSHML14---`

| Pos | Letter | Module | Description                                                   |
|-----|--------|--------------------|---------------------------------------------------|
| 0   | `A`    | KAT4               | Internal ATU                                      |
| 1   | `P`    | KPA4               | PA                                                |
| 2   | `X`    | XVTR               | Transverter                                       |
| 3   | `S`    | KRX4 + 2nd KDDC4   | Sub RX; standard in K4D                           |
| 4   | `H`    | KHDR4 + KDDC4-2    | HDR Module; standard in K4HD                      |
| 5   | `M`    |                    | K40 (\"Mini\")                                    |
| 6   | `L`    | KPA500 or KPA1500  | Linear amp detected                               |
| 7   | `1`    | KPA1500            | Specifically                                      |
| 8   | `4`    |                    | Radio identified as K4 (S+4 = K4D; S+H+4 = K4HD). |" =>
    K4InstalledOptions {
        has_kat4_atu: bool,
        has_kpa4_pa: bool,
        has_xvtr: bool,
        has_krx4_sub_receiver: bool,
        has_khdr4_hdr: bool,
        has_k40_mini: bool,
        has_kpa_linear_pa: bool,
        has_kpa1500_pa: bool
    }
);

define_command_struct!(
    "KX3/KX2 installed option modules.

Option string after leading space: `APF---TBXI0n`

| Pos | Letter | Module |
|-----|--------|--------|
| 0 | `A` | ATU (KXAT3 or KXAT2) |
| 1 | `P` | External 100-W PA (KXPA100) |
| 2 | `F` | Roofing filter (KXFL3) |
| 6 | `T` | External 100-W ATU (KXAT100, a KXPA100 internal option) |
| 7 | `B` | NiMH battery-charger / real-time clock (KXBC3) |
| 8 | `X` | KX3-2M or KX3-4M transverter module |
| 9 | `I` | KXIO2 RTC I/O module |
| 11 | `2`/`1` | Product ID: `2` = KX3, `1` = KX2 |" =>
    KXInstalledOptions {
        has_kxat_atu: bool,
        has_kxpa100_pa: bool,
        has_kxfl3_roofing_filter: bool,
        has_kxat100_external_atu: bool,
        has_kxbc3_realtime_clock: bool,
        has_kx3m_transverter: bool,
        has_kxio2_rtc_io: bool,
        is_kx3: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetK3IconsAndStatus, K3IconsAndStatus
// ------------------------------------------------------------------------------------------------

define_cat_command!("Query icon and status flags (`IC`).

K3/K3S only.

# Command format

> `IC;`

# Response format

> `IC{bbbbb};`

Where *bbbbb* is five binary-packed bytes encoding 40 status bits covering sub-receiver,
diversity, text-to-terminal, repeater tone, VOX, scan, ESSB, DVR playback, AM-sync, and related
states. See [`K3IconsAndStatus`] for the decoded form." =>
    GetK3IconsAndStatus
);

define_command_struct!(
    "Decoded K3/K3S icon and status flags from the `IC` response." =>
    K3IconsAndStatus {
        preset_1: bool,
        preset_2: bool,
        band_sel: bool,
        msg_playing: bool,
        message_bank_1: bool,
        message_bank_2: bool,
        mw_power_level: bool,
        tx_test: bool,
        bset: bool,
        sub_rx_on: bool,
        sub_rx_nb_on: bool,
        sub_rx_aux_bnc: bool,
        sub_ant_main: bool,
        sub_ant_aux: bool,
        diversity_mode: bool,
        vfo_ab_bands_independent: bool,
        vfo_ab_linked: bool,
        text_to_terminal_on: bool,
        sync_data: bool,
        fsk_tx_polarity_normal: bool,
        fsk_dual_tone_filter_on: bool,
        vox_on_for_cw_fsk_psk: bool,
        dual_passband_cf_apf_on: bool,
        full_qsk: bool,
        repeater_tx_offset_negative: bool,
        repeater_tx_offset_positive: bool,
        fm_pl_tone_on: bool,
        am_sync_rx: bool,
        noise_gate_on: bool,
        essb: bool,
        vox_on_for_voice_data_afsk: bool,
        fast_play_in_effect: bool,
        ofs_led_is_on: bool,
        vfob_led_on: bool,
        sub_rx_nr_on: bool,
        sub_rx_squelched: bool,
        main_rx_squelched: bool,
        am_sync_usb: bool,
        am_sync_lsb: bool,
        shift_10_hz: bool,
        shift_50_hz: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverInformation, TransceiverInformation
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transceiver information (`IF`).

On K4 this command is provided for K3 compatibility (K22/K31 meta-mode variant);
K4-native software should use specific commands instead.

# Command format

> `IF;`

# Response format

> `IF{fffffffffff}␣␣␣␣␣{s}{yyyy}{r}{x}␣00{t}{m}{v}{s2}{p}{b}{d}1␣;`

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
| 31     | *b*           | 1     | In K22 mode: `1` on band change, else `0`. Always `0` in basic    |
| 32     | *d*           | 1     | K31: DATA sub-mode (`0`-`3`); else `0`                            |
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
        "Indicates whether this response is due to a transceiver band-change." => event_on_band_change: bool,
        data_sub_mode: Option<DataSubMode>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOperatingModeA, GetOperatingModeB, SetOperatingModeA, SetOperatingModeB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get operating mode for VFO A (`MD`).

# Command format

> `MD;`

# Response format

> `MD{n};`

Where *n* is one of:

* `1`; LSB
* `2`; USB
* `3`; CW
* `4`; FM
* `5`; AM
* `6`; DATA A (AFSK A)
* `7`; CW-REV
* `9`; DATA B (FSK D / PSK D)" =>
    GetVfoAOperatingMode
);

define_cat_command!("Get operating mode for VFO B receiver (`MD$`).

# Command format

> `MD$;`

# Response format

> `MD${n};`

Where *n* is one of:

* `1`; LSB
* `2`; USB
* `3`; CW
* `4`; FM
* `5`; AM
* `6`; DATA A (AFSK A)
* `7`; CW-REV
* `9`; DATA B (FSK D / PSK D)" =>
    GetVfoBOperatingMode
);

define_cat_command!("Set operating mode for VFO A (`MD`).

# Command format

> `MD{n};`

Where *n* is one of:

* `1`; LSB
* `2`; USB
* `3`; CW
* `4`; FM
* `5`; AM
* `6`; DATA A (AFSK A)
* `7`; CW-REV
* `9`; DATA B (FSK D / PSK D)" =>
    SetVfoAOperatingMode {
        mode: OperatingMode
    }
);

define_cat_command!("Set operating mode for VFO B receiver (`MD$`).

# Command format

> `MD${n};`

Where *n* is one of:

* `1`; LSB
* `2`; USB
* `3`; CW
* `4`; FM
* `5`; AM
* `6`; DATA A (AFSK A)
* `7`; CW-REV
* `9`; DATA B (FSK D / PSK D)" =>
    SetVfoBOperatingMode {
        mode: OperatingMode
    }
);

define_command_enum!(
    "Operating mode for [`GetVfoAOperatingMode`]/[`GetVfoBOperatingMode`] and [`SetVfoAOperatingMode`]/[`SetVfoBOperatingMode`] commands." =>
    OperatingMode {
        LowerSideBand = b'1',
        UpperSideBand = b'2',
        ContinuousWave = b'3',
        FrequencyModulation = b'4',
        AmplitudeModulation = b'5',
        DataA = b'6',
        ContinuousWaveReverse = b'7',
        DataB = b'9'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetIfShiftA, GetIfShiftB, SetIfShiftA, SetIfShiftB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Intermediate Frequency (IF) Shift (passband tuning) in Hz for VFO A (`IS`).

# Command format

> `IS;`

# Response format

> `IS{s}{nnnn};`

Where *s* is the sign (`+` or `-`) and *nnnn* is the offset between `0000` and `2999` Hz." =>
    GetVfoAIfShift
);

define_cat_command!("Get Intermediate Frequency (IF) Shift (passband tuning) in Hz for VFO B receiver (`IS$`).

# Command format

> `IS$;`

# Response format

> `IS${s}{nnnn};`

Where *s* is the sign (`+` or `-`) and *nnnn* is the offset between `0000` and `2999` Hz." =>
    GetVfoBIfShift
);

define_cat_command!("Set Intermediate Frequency (IF) Shift (passband tuning) in Hz for VFO A (`IS`).

# Command format

> `IS{s}{nnnn};`

Where *s* is the sign (`+` or `-`) and *nnnn* is the offset between `0000` and `2999` Hz." =>
    SetVfoAIfShift {
        offset_hz: i16
    }
);

define_cat_command!("Set Intermediate Frequency (IF) Shift (passband tuning) in Hz for VFO B receiver (`IS$`).

# Command format

> `IS${s}{nnnn};`

Where *s* is the sign (`+` or `-`) and *nnnn* is the offset between `0000` and `2999` Hz." =>
    SetVfoBIfShift {
        offset_hz: i16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetKeyerSpeed, SetKeyerSpeed
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get keyer speed in Words per Minute (WPM) (`KS`).

# Command format

> `KS;`

# Response format

> `KS{nnn};`

Where *nnn* is between `008` and `050` (K3/KX); up to `100` on K4." =>
    GetKeyerSpeed
);

define_cat_command!("Set keyer speed in Words per Minute (WPM) (`KS`).

# Command format

> `KS{nnn};`

Where *nnn* is between `008` and `050` (K3/KX); up to `100` on K4." =>
    SetKeyerSpeed {
        wpm: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SendCwText
// ------------------------------------------------------------------------------------------------

define_cat_command!("Send CW or data text to the keyer buffer (`KY`).

Sending empty text aborts transmission.

# Command format

> `KY{n} {ssssssssssssssssssssssss};`

Where *n* is `0` (send now) or `1` (buffer only), followed by a space and up to 24 ASCII
characters." =>
    SendCwText no_copy {
        buffer_only: bool,
        text: Vec<u8>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoLockA, GetVfoLockB, SetVfoLockA, SetVfoLockB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get VFO A lock on/off state (`LK`).

# Command format

> `LK;`

# Response format

> `LK{n};`

Where `n` is the boolean state `0` (unlocked) or `1` (locked)." =>
    GetVfoALockState
);

define_cat_command!("Get VFO B lock on/off state (`LK$`).

# Command format

> `LK$;`

# Response format

> `LK${n};`

Where `n` is the boolean state `0` (unlocked) or `1` (locked)." =>
    GetVfoBLockState
);

define_cat_command!("Set VFO A lock. `locked = true` disables the VFO A encoder (`LK`).

# Command format

> `LK{n};`

Where `n` is the boolean state `0` (unlocked) or `1` (locked)." =>
    SetVfoALockState { state }
);

define_cat_command!("Set VFO B lock. `locked = true` disables the VFO B encoder (`LK$`).

# Command format

> `LK${n};`

Where `n` is the boolean state `0` (unlocked) or `1` (locked)." =>
    SetVfoBLockState {state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoLink, SetVfoLink
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get VFO A/B linked state (`LN`).

K3 only.

# Command format

> `LN;`

# Response format

> `LN{n};`

Where `n` is the boolean state `0` (unlinked) or `1` (linked; both VFOs track each other)." =>
    GetVfoLinkedState
);

define_cat_command!("Set VFO linked on/off (`LN`).

K3 only.

# Command format

> `LN{n};`

Where `n` is the boolean state `0` (unlinked) or `1` (linked)." =>
    SetVfoLinkedState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMemoryChannel, SetMemoryChannel
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get active memory channel (`MC`).

# Command format

> `MC;`

# Response format

> `MC{nnn};`

Where *nnn* is between `001` and `100`." =>
    GetMemoryChannel
);

define_cat_command!("Set active memory channel (`MC`).

# Command format

> `MC{nnn};`

Where *nnn* is between `001` and `100`." =>
    SetMemoryChannel {
        channel: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMicGain, SetMicGain
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get microphone gain (`MG`).

# Command format

> `MG;`

# Response format

> `MG{nnn};`

Where *nnn* is between `000` and `060`." =>
    GetMicGain
);

define_cat_command!("Set microphone gain (`MG`).

# Command format

> `MG{nnn};`

Where *nnn* is between `000` and `060`." =>
    SetMicGain {
        gain: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMonitorLevel, SetMonitorLevel
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transmit monitor level (`ML`).

# Command format

> `ML;`

# Response format

> `ML{nnn};`

Where *nnn* is between `000` and `060`." =>
    GetMonitorLevel
);

define_cat_command!("Set transmit monitor level (`ML`).

# Command format

> `ML{nnn};`

Where *nnn* is between `000` and `060`." =>
    SetMonitorLevel {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SelectMenu
// ------------------------------------------------------------------------------------------------

define_cat_command!("Select a front-panel menu item by number (`MN`).

Item numbers differ between K3 (Table 5), KX3 (Table 6), and KX2 (Table 6A) in the G5 reference.
Send `MN000;` to close the menu.

# Command format

> `MN{nnn};`

Where *nnn* is the 3-digit menu item number." =>
    SelectMenuItem {
        item: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMenuParameter, SetMenuParameter
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current menu item's 8-bit parameter (`MP`).

Must call [`SelectMenuItem`] first.

# Command format

> `MP;`

# Response format

> `MP{nn};`

Where *nn* is between `00` and `99` (BCD-encoded 8-bit value)." =>
    GetMenuParameter
);

define_cat_command!("Set the current menu item's 8-bit parameter (`MP`).

Must call [`SelectMenuItem`] first.

# Command format

> `MP{nn};`

Where *nn* is between `00` and `99` (BCD-encoded 8-bit value)." =>
    SetMenuParameter {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMenuParameter16, SetMenuParameter16
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current menu item's 16-bit parameter (`MQ`).

KX3/KX2 only. Must call [`SelectMenuItem`] first.

# Command format

> `MQ;`

# Response format

> `MQ{nnnn};`

Where *nnnn* is between `0000` and `9999`." =>
    GetMenuParameter16
);

define_cat_command!("Set the current menu item's 16-bit parameter (`MQ`).

KX3/KX2 only. Must call [`SelectMenuItem`] first.

# Command format

> `MQ{nnnn};`

Where *nnnn* is between `0000` and `9999`." =>
    SetMenuParameter16 {
        value: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetNoiseBlankerA, GetNoiseBlankerB, SetNoiseBlankerA, SetNoiseBlankerB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Noise Blanker (NB) on/off state for VFO A/main (`NB`).

# Command format

> `NB;`

# Response format

> `NB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetVfoANoiseBlanker
);

define_cat_command!("Get Noise Blanker (NB) on/off state for VFO B (`NB$`).

# Command format

> `NB$;`

# Response format

> `NB${n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetVfoBNoiseBlanker
);

define_cat_command!("Set Noise Blanker on/off state for VFO A/main (`NB`).

# Command format

> `NB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetVfoANoiseBlanker { state }
);

define_cat_command!("Set Noise Blanker on/off state for VFO B (`NB$`).

# Command format

> `NB${n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetVfoBNoiseBlanker { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetNoiseBlankerLevelA, GetNoiseBlankerLevelB, SetNoiseBlankerLevelA,
// SetNoiseBlankerLevelB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Noise Blanker (NB) level for VFO A/main (`NL`).

# Command format

> `NL;`

# Response format

> `NL{nn};`

Where *nn* is between `00` and `21` (K3/K3S) or `00` and `09` (KX3/KX2)." =>
    GetVfoANoiseBlankerLevel
);

define_cat_command!("Get Noise Blanker (NB) level for VFO B (`NL$`).

# Command format

> `NL$;`

# Response format

> `NL${nn};`

Where *nn* is between `00` and `21` (K3/K3S) or `00` and `09` (KX3/KX2)." =>
    GetVfoBNoiseBlankerLevel
);

define_cat_command!("Set Noise Blanker (NB) level for VFO A (`NL`).

# Command format

> `NL{nn};`

Where *nn* is between `00` and `21` (K3/K3S) or `00` and `09` (KX3/KX2)." =>
    SetVfoANoiseBlankerLevel {
        level: u8
    }
);

define_cat_command!("Set Noise Blanker (NB) level for VFO B (`NL$`).

# Command format

> `NL${nn};`

Where *nn* is between `00` and `21` (K3/K3S) or `00` and `09` (KX3/KX2)." =>
    SetVfoBNoiseBlankerLevel {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPreampA, GetPreampB, SetPreampA, SetPreampB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get receive preamplifier selection for VFO A (`PA`).

# Command format

> `PA;`

# Response format

> `PA{n};`

Where *n* is one of:

* `0`; off
* `1`; preamp 1
* `2`; preamp 2 / low-noise amp (K3S/KXV3B on 12/10/6 m only)" =>
    GetVfoAPreamp
);

define_cat_command!("Get receive preamplifier selection for VFO B (`PA$`).

# Command format

> `PA$;`

# Response format

> `PA${n};`

Where *n* is one of:

* `0`; off
* `1`; preamp 1
* `2`; preamp 2 / low-noise amp (K3S/KXV3B on 12/10/6 m only)" =>
    GetVfoBPreamp
);

define_cat_command!("Set receive preamplifier for VFO A (`PA`).

# Command format

> `PA{n};`

Where *n* is one of:

* `0`; off
* `1`; preamp 1
* `2`; preamp 2 / low-noise amp (K3S/KXV3B on 12/10/6 m only)" =>
    SetVfoAPreamp {
        preamp: u8
    }
);

define_cat_command!("Set receive preamplifier for VFO B (`PA$`).

# Command format

> `PA${n};`

Where *n* is one of:

* `0`; off
* `1`; preamp 1
* `2`; preamp 2 / low-noise amp (K3S/KXV3B on 12/10/6 m only)" =>
    SetVfoBPreamp {
        preamp: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerControl, SetPowerControl
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transmit power level in watts (`PC`).

# Command format

> `PC;`

# Response format

> `PC{nnn};`

Where *nnn* is between `000` and `110` (K3/K3S) or `000` and `012` (KX3/KX2) watts." =>
    GetTransmitPowerControl
);

define_cat_command!("Set transmit power in watts (`PC`).

# Command format

> `PC{nnn};`

Where *nnn* is between `000` and `110` (K3/K3S) or `000` and `012` (KX3/KX2) watts." =>
    SetTransmitPowerControl {
        watts: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetActualPowerOutput
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get actual RF power output in tenths of a watt (`PO`).

KX3/KX2 only.

# Command format

> `PO;`

# Response format

> `PO{nnnn};`

Where *nnnn* is power x 10 in mW, e.g. `0050` = 5.0 W." =>
    GetActualPowerOutput
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatus, SetPowerStatus
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transceiver power on/off state (`PS`).

# Command format

> `PS;`

# Response format

> `PS{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetPowerStatus
);

define_cat_command!("Set transceiver power on/off state (`PS`).

Sending `true`/`1` initiates a controlled power-off sequence.

# Command format

> `PS{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetPowerStatus { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAReceiveAttenuator, GetVfoBReceiveAttenuator, SetVfoAReceiveAttenuator,
// SetVfoBReceiveAttenuator
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get receive attenuator level for VFO A (`RA`).

# Command format

> `RA;`

# Response format

> `RA{nn};`

Where *nn* is between `00` and `15` (K3/K3S, 2 dB steps up to ~30 dB) or `00` and `01` (KX3/KX2,
`0` = off, `1` = 10 dB)." =>
    GetVfoAReceiveAttenuator
);

define_cat_command!("Get receive attenuator level for VFO B (`RA$`).

# Command format

> `RA$;`

# Response format

> `RA${nn};`

Where *nn* is between `00` and `15` (K3/K3S, 2 dB steps up to ~30 dB) or `00` and `01` (KX3/KX2,
`0` = off, `1` = 10 dB)." =>
    GetVfoBReceiveAttenuator
);

define_cat_command!("Set receive attenuator level for VFO A (`RA`).

# Command format

> `RA{nn};`

Where *nn* is between `00` and `15` (K3/K3S, 2 dB steps up to ~30 dB) or `00` and `01` (KX3/KX2,
`0` = off, `1` = 10 dB)." =>
    SetVfoAReceiveAttenuator {
        level: u8
    }
);

define_cat_command!("Set receive attenuator level for VFO B (`RA$`).

# Command format

> `RA${nn};`

Where *nn* is between `00` and `15` (K3/K3S, 2 dB steps up to ~30 dB) or `00` and `01` (KX3/KX2,
`0` = off, `1` = 10 dB)." =>
    SetVfoBReceiveAttenuator {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: ClearRit
// ------------------------------------------------------------------------------------------------

define_cat_command!("Clear the Receive Incremental Tuning (RIT)/Transmit Incremental Tuning(XIT) offset, and reset to +-0 Hz (`RC`).

# Command format

> `RC;`" =>
    ClearRitOffset
);

// ------------------------------------------------------------------------------------------------
// Public Types: MoveRitOffsetDown
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move the Receive Incremental Tuning (RIT)/Transmit Incremental Tuning(XIT) offset down by `hz` Hz (`RD`).

# Command format

> `RD{nnnn};`

Where *nnnn* is Hz to decrease; `0000` decrements one step." =>
    MoveRitOffsetDown {
        hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoARfGain, GetVfoBRfGain, SetVfoARfGain, SetVfoBRfGain
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get RadioFrequency (RF) gain (DAC value) for VFO A (`RG`).

`190` is the minimum useful gain; `250` is the maximum gain.

# Command format

> `RG;`

# Response format

> `RG{nnn};`

Where *nnn* is between `190` and `250`." =>
    GetVfoARfGain
);

define_cat_command!("Get RadioFrequency (RF) gain (DAC value) for VFO B (`RG$`).

# Command format

> `RG$;`

# Response format

> `RG${nnn};`

Where *nnn* is between `190` and `250`." =>
    GetVfoBRfGain
);

define_cat_command!("Set RadioFrequency (RF) gain (DAC value) for VFO A (`RG`).

# Command format

> `RG{nnn};`

Where *nnn* is between `190` and `250`." =>
    SetVfoARfGain {
        gain: u8
    }
);

define_cat_command!("Set RadioFrequency (RF) gain (DAC value) for VFO B (`RG$`).

# Command format

> `RG${nnn};`

Where *nnn* is between `190` and `250`." =>
    SetVfoBRfGain {
        gain: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRitXitOffset, SetRitXitOffset
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get RIT/XIT offset in Hz (`RO`).

# Command format

> `RO;`

# Response format

> `RO{s}{nnnn};`

Where *s* is the sign (`+` or `-`) and *nnnn* is the offset between `0000` and `9999` Hz." =>
    GetRitXitOffset
);

define_cat_command!("Set RIT/XIT offset in Hz (`RO`).

# Command format

> `RO{s}{nnnn};`

Where *s* is the sign (`+` or `-`) and *nnnn* is the offset between `0000` and `9999` Hz." =>
    SetRitXitOffset {
        offset_hz: i16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRitControl, SetRitControl
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Receive Incremental Tuning (RIT) on/off state (`RT`).

# Command format

> `RT;`

# Response format

> `RT{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetRitControl
);

define_cat_command!("Set Receive Incremental Tuning (RIT) on/off (`RT`).

# Command format

> `RT{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetRitControl { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: MoveRitOffsetUp
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move the Receive Incremental Tuning (RIT)/Transmit Incremental Tuning(XIT) offset up by `hz` Hz (`RU`).

# Command format

> `RU{nnnn};`

Where *nnnn* is Hz to increase; `0000` increments one step." =>
    MoveRitOffsetUp {
        hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareRevision
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get firmware revision information (`RV`).

The DSP field may be absent on older radios.

# Command format

> `RV;`

# Response format

> `RV{xx.xx} {yy.yy};`

Where *xx.xx* is the main MCU firmware version and *yy.yy* is the DSP firmware version." =>
    GetFirmwareRevision
);

// ------------------------------------------------------------------------------------------------
// Public Types: GoToReceive
// ------------------------------------------------------------------------------------------------

define_cat_command!("Exit transmit and return to receive mode (`RX`).

Equivalent to releasing PTT.

# Command format

> `RX;`" =>
    GoToReceive
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSubReceiver, SetSubReceiver
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get sub-receiver on/off state (`SB`).

K3/K3S only, and requires KRX3A.

# Command format

> `SB;`

# Response format

> `SB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetSubReceiver
);

define_cat_command!("Set sub-receiver on/off state (`SB`).

K3/K3S only, and requires KRX3A.

# Command format

> `SB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetSubReceiver { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetQskDelay
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Full Break-In (QSK) delay in milliseconds (`SD`).

Only in CW mode.

# Command format

> `SD;`

# Response format

> `SD{nnnn};`

Where *nnnn* is between `0000` and `0900` ms.

**Note**: `0000` = full QSK (no delay)." =>
    GetQskDelay
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSMeterA, GetSMeterB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get S-meter value for VFO A (`SM`).

`00`-`30` correspond to S0-S9+20 dB in half-unit steps; `31`-`42` = over-scale.

# Command format

> `SM;`

# Response format

> `SM{nn};`

Where *nn* is between `00` and `42`." =>
    GetVfoASMeter
);

define_cat_command!("Get S-meter value for VFO B (`SM$`).

`00`-`30` correspond to S0-S9+20 dB in half-unit steps; `31`-`42` = over-scale.

# Command format

> `SM$;`

# Response format

> `SM${nn};`

Where *nn* is between `00` and `42`." =>
    GetVfoBSMeter
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetHighResolutionSMeter
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get high-resolution S-meter (`SMH`).

K3/K3S only.

# Command format

> `SMH;`

# Response format

> `SMH{nnnn};`

Where *nnnn* is an unsigned value in 0.1 dBm units.

**Note**: `0000` is the weakest detectable signal." =>
    GetHighResolutionSMeter
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSquelchA, GetSquelchB, SetSquelchA, SetSquelchB
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get squelch level for VFO A (`SQ`).

# Command format

> `SQ;`

# Response format

> `SQ{nn};`

Where *nn* is between `00` and `29`; `00` = squelch open/off." =>
    GetVfoASquelch
);

define_cat_command!("Get squelch level for VFO B (`SQ$`).

# Command format

> `SQ$;`

# Response format

> `SQ${nn};`

Where *nn* is between `00` and `29`; `00` = squelch open/off." =>
    GetVfoBSquelch
);

define_cat_command!("Set squelch level for VFO A (`SQ`).

# Command format

> `SQ{nn};`

Where *nn* is between `00` and `29`; `00` = squelch open/off." =>
    SetVfoASquelch {
        level: u8
    }
);

define_cat_command!("Set squelch level for VFO B (`SQ$`).

# Command format

> `SQ${nn};`

Where *nn* is between `00` and `29`; `00` = squelch open/off." =>
    SetVfoBSquelch {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: EmulateButtonTap, EmulateButtonHold
// ------------------------------------------------------------------------------------------------

define_cat_command!("Emulate a front-panel button tap (`SWT`).

Button *tap* is a momentary press of the button.

# Command format

> `SWT{nn};`

Where *nn* is the switch number from one of the tables below. See the G5 programmer's reference for
details.

| TAP      | HOLD     | nn | TAP              | HOLD        | nn | TAP      | HOLD      | nn |
| -------- | -------- | -- | ---------------- | ----------- | -- | -------- | --------- | -- |
| BAND-    | VOX      | 09 | FREQ ENT         | SCAN        | 41 | CWT (0)  | TEXT Dec  | 40 |
| BAND+    | QSK      | 10 | FINE             | COARSE      | 49 | AFX (<-) | DATA Md   | 43 |
| MODE-    | ALT      | 17 | RATE             | LOCK        | 50 | V->M     | AF REC    | 15 |
| MODE+    | TEST     | 18 | SUB              | DVRSTY*     | 48 | M->V     | AF PLAY   | 23 |
| MENU     | CONFIG   | 14 | A/B (1)          | BSET        | 11 | M1       | M1-RPT    | 21 |
| XMIT     | TUNE     | 16 | REV (FM/rpt) (2) | n/a         | 12 | M2       | M2-RPT    | 31 |
| RX ANT   | n/a      | 25 | A->B (3)         | SPLIT       | 13 | M3       | M3-RPT    | 35 |
| DISP     | METER    | 08 | PRE (4)          | ATT         | 24 | M4       | M4-RPT    | 39 |
| ATU      | Tune ATU | 19 | AGC (5)          | OFF         | 27 | REC      | MSG Bank  | 37 |
| ANT      | ANT Name | 26 | XFIL (6)         | DUAL PB/APF | 29 | RIT      | PF1       | 45 |
| SHIFT/LO | NORM     | 58 | NB (7)           | LEVEL       | 33 | XIT      | PF2       | 47 |
| WIDTH/HI | I/II     | 59 | NR (8)           | ADJ         | 34 | CLR      | n/a       | 53 |
| SPD/MIC  | DELAY    | 57 | NTCH (9)         | MANUAL      | 32 |          |           |    |
| CMP/PWR  | MON      | 56 | SPOT ('.')       | PITCH       | 42 |          |           |    |

**Table 7** Switch identifiers (**nn**) for the **K3** SWT/SWH command. (For the **KX3**, see Table
8, and for the **KX2**,Table 8A.) Table entries are organized similarly to the transceiver's front
panel (e.g. BAND is upper left on the K3). Numeric keypad switches (0-9, '.', '<-') are shown in
parentheses. DIVERSITY hold function was implemented in K3 rev. 5.10. Prior to this, a hold of the
SUB switch would link/unlink VFOs A and B. To replace the link/unlink function in 5.10 or later,
menu entry CONFIG:VFO LNK was added. VFO link/unlink can also be accomplished using the **LN**
command.

| TAP               | HOLD     | nn | TAP           | HOLD  | nn | TAP              | HOLD    | nn | TAP    | HOLD          | nn |
| ----------------- | -------- | -- | ------------- | ----- | -- | ---------------- | ------- | -- | ------ | ------------- | -- |
| BAND+             | RCL      | 08 | PRE (1)       | NR    | 19 | MODE             | ALT     | 14 | A/B    | REV (FM/rpt)  | 24 |
| BAND-             | STORE    | 41 | ATTN (2)      | NB    | 27 | DATA             | TEXT    | 17 | A->B   | SPLIT         | 25 |
| FREQ ENT          | SCAN     | 10 | APF (3)       | NTCH  | 20 | RIT              | PF1     | 18 | XIT    | PF2           | 26 |
| MSG (<-)          | REC      | 11 | SPOT (4)      | CWT   | 28 | RATE             | KHZ     | 12 | DISP   | MENU          | 09 |
| ATU TUNE ('.')    | ANT      | 44 | CMP (5)       | PITCH | 21 |                  |         |    |        |               |    |
| XMIT (0)          | TUNE     | 16 | DLY (6)       | VOX   | 29 |                  |         |    |        |               |    |
| AF/RF-SQL (7)     | MON      | 32 | PBT I/II (8)  | NORM  | 33 | KEYER/ MIC (9)   | PWR     | 34 | OFS/ B | CLR           | 35 |

**Table 8** Switch identifiers (**nn**) for the KX3 SWT/SWH command. Table entries are organized
similarly to the KX3's front panel (e.g. BAND is upper left); knob functions are shown in the last
row. Numeric keypad switches (0-9, '.', '<-') are shown in parentheses. **Note**: If *Fast Play* is
in effect, switch emulation commands for BAND+, BAND- and FREQ ENT are blocked (both SWT and SWH).
See byte (e), bit 0 of the **IC** response.

| TAP               | HOLD      | nn | TAP      | HOLD      | nn | TAP          | HOLD      | nn |
| ----------------- | --------- | -- | -------- | --------- | -- | ------------ | --------- | -- |
AF GAIN/MON (0)     | NB        | 32 | DATA     | TEXT      | 26 | MODE ('.')   | RCL       | 08 |
PRE (/ATTN) (1)     | NR        | 19 | MSG      | REC       | 11 | BAND (<-)    | STORE     | 14 |
FIL (2)             | APF/AN    | 27 | RATE     | FREQ/🔒   | 41 | A/B (6)      | A>B       | 44 |
ATU* (3)            | PFn       | 20 |          |           |    | RIT (7)      | SPLIT     | 18 |
XMIT (4)            | TUNE      | 16 |          |           |    | DISP (8)     | MENU      | 09 |
KYR-SPT/MIC (5)     | PWR       | 34 |          |           |    | OFS/B (9)    | CLR       | 35 |

**Table 8A** Switch identifiers (**nn**) for the **KX2** SWT/SWH command. Numeric keypad switches
(0-9, '.', '<-') are shown in parentheses.
" =>
    EmulateButtonTap {
        button: u8
    }
);

define_cat_command!("Emulate a front-panel button hold (long press).

Button *hold* is a long press of the button.

# Command format

> `SWH{nn};`

Where *nn* is the switch number from one of the tables below. See the G5 programmer's reference for
details.

| TAP      | HOLD     | nn | TAP              | HOLD        | nn | TAP      | HOLD      | nn |
| -------- | -------- | -- | ---------------- | ----------- | -- | -------- | --------- | -- |
| BAND-    | VOX      | 09 | FREQ ENT         | SCAN        | 41 | CWT (0)  | TEXT Dec  | 40 |
| BAND+    | QSK      | 10 | FINE             | COARSE      | 49 | AFX (<-) | DATA Md   | 43 |
| MODE-    | ALT      | 17 | RATE             | LOCK        | 50 | V->M     | AF REC    | 15 |
| MODE+    | TEST     | 18 | SUB              | DVRSTY*     | 48 | M->V     | AF PLAY   | 23 |
| MENU     | CONFIG   | 14 | A/B (1)          | BSET        | 11 | M1       | M1-RPT    | 21 |
| XMIT     | TUNE     | 16 | REV (FM/rpt) (2) | n/a         | 12 | M2       | M2-RPT    | 31 |
| RX ANT   | n/a      | 25 | A->B (3)         | SPLIT       | 13 | M3       | M3-RPT    | 35 |
| DISP     | METER    | 08 | PRE (4)          | ATT         | 24 | M4       | M4-RPT    | 39 |
| ATU      | Tune ATU | 19 | AGC (5)          | OFF         | 27 | REC      | MSG Bank  | 37 |
| ANT      | ANT Name | 26 | XFIL (6)         | DUAL PB/APF | 29 | RIT      | PF1       | 45 |
| SHIFT/LO | NORM     | 58 | NB (7)           | LEVEL       | 33 | XIT      | PF2       | 47 |
| WIDTH/HI | I/II     | 59 | NR (8)           | ADJ         | 34 | CLR      | n/a       | 53 |
| SPD/MIC  | DELAY    | 57 | NTCH (9)         | MANUAL      | 32 |          |           |    |
| CMP/PWR  | MON      | 56 | SPOT ('.')       | PITCH       | 42 |          |           |    |

**Table 7** Switch identifiers (**nn**) for the **K3** SWT/SWH command. (For the **KX3**, see Table
8, and for the **KX2**,Table 8A.) Table entries are organized similarly to the transceiver's front
panel (e.g. BAND is upper left on the K3). Numeric keypad switches (0-9, '.', '<-') are shown in
parentheses. DIVERSITY hold function was implemented in K3 rev. 5.10. Prior to this, a hold of the
SUB switch would link/unlink VFOs A and B. To replace the link/unlink function in 5.10 or later,
menu entry CONFIG:VFO LNK was added. VFO link/unlink can also be accomplished using the **LN**
command.

| TAP               | HOLD     | nn | TAP           | HOLD  | nn | TAP              | HOLD    | nn | TAP    | HOLD          | nn |
| ----------------- | -------- | -- | ------------- | ----- | -- | ---------------- | ------- | -- | ------ | ------------- | -- |
| BAND+             | RCL      | 08 | PRE (1)       | NR    | 19 | MODE             | ALT     | 14 | A/B    | REV (FM/rpt)  | 24 |
| BAND-             | STORE    | 41 | ATTN (2)      | NB    | 27 | DATA             | TEXT    | 17 | A->B   | SPLIT         | 25 |
| FREQ ENT          | SCAN     | 10 | APF (3)       | NTCH  | 20 | RIT              | PF1     | 18 | XIT    | PF2           | 26 |
| MSG (<-)          | REC      | 11 | SPOT (4)      | CWT   | 28 | RATE             | KHZ     | 12 | DISP   | MENU          | 09 |
| ATU TUNE ('.')    | ANT      | 44 | CMP (5)       | PITCH | 21 |                  |         |    |        |               |    |
| XMIT (0)          | TUNE     | 16 | DLY (6)       | VOX   | 29 |                  |         |    |        |               |    |
| AF/RF-SQL (7)     | MON      | 32 | PBT I/II (8)  | NORM  | 33 | KEYER/ MIC (9)   | PWR     | 34 | OFS/ B | CLR           | 35 |

**Table 8** Switch identifiers (**nn**) for the KX3 SWT/SWH command. Table entries are organized
similarly to the KX3's front panel (e.g. BAND is upper left); knob functions are shown in the last
row. Numeric keypad switches (0-9, '.', '<-') are shown in parentheses. **Note**: If *Fast Play* is
in effect, switch emulation commands for BAND+, BAND- and FREQ ENT are blocked (both SWT and SWH).
See byte (e), bit 0 of the **IC** response.

| TAP               | HOLD      | nn | TAP      | HOLD      | nn | TAP          | HOLD      | nn |
| ----------------- | --------- | -- | -------- | --------- | -- | ------------ | --------- | -- |
AF GAIN/MON (0)     | NB        | 32 | DATA     | TEXT      | 26 | MODE ('.')   | RCL       | 08 |
PRE (/ATTN) (1)     | NR        | 19 | MSG      | REC       | 11 | BAND (<-)    | STORE     | 14 |
FIL (2)             | APF/AN    | 27 | RATE     | FREQ/🔒   | 41 | A/B (6)      | A>B       | 44 |
ATU* (3)            | PFn       | 20 |          |           |    | RIT (7)      | SPLIT     | 18 |
XMIT (4)            | TUNE      | 16 |          |           |    | DISP (8)     | MENU      | 09 |
KYR-SPT/MIC (5)     | PWR       | 34 |          |           |    | OFS/B (9)    | CLR       | 35 |

**Table 8A** Switch identifiers (**nn**) for the **KX2** SWT/SWH command. Numeric keypad switches
(0-9, '.', '<-') are shown in parentheses.
" =>
    EmulateButtonHold {
        button: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBufferedText
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get text from the decoded text buffer (`TB`).

# Command format

> `TB;`

# Response format

> `TB{n}␣{ssssssss};`

Where *n* is `0` (empty), `1` (more follows), or `2` (last segment), followed by a space and up to
8 ASCII characters of decoded CW/RTTY text." =>
    GetBufferedText
);

// TODO: implement command with response for GetBufferedText.

define_command_enum!(
    "State of a response segment returned by the [`GetBufferedText`] command." =>
    BufferedTextSegmentState {
        Empty = 0,
        MoreFollows = 1,
        LastSegment = 2
    }
);

define_command_struct!(
    "A segment of decoded CW/RTTY text returned by the [`GetBufferedText`] command." =>
    BufferedTextSegment no_copy {
        state: BufferedTextSegmentState,
        text: String
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransmitBufferedText
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the transmit CW/data text buffer (`TBX`).

KX3/KX2 only.

# Command format

> `TBX;`

# Response format

> `TB{n}␣{ssssssss};`

Where *n* is `0` (empty), `1` (more follows), or `2` (last segment), followed by a space and up to
8 ASCII characters of decoded CW/RTTY text." =>
    GetTransmitBufferedText
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetTxEqualizer
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set transmit audio equalizer parameters (`TE`).

Consult the G5 programmer's reference for parameter encoding details. Sent as raw bytes; no fixed
width format is defined at the command level.

# Command format

> `TE{params};`" =>
    SetTransmitEqualizer no_copy {
        params: Vec<u8>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTxMeterMode, SetTxMeterMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transmit meter display mode (`TM`).

K3/K3S only.

# Command format

> `TM;`

# Response format

> `TM{n};`

Where *n* is one of:

* `0`; power out
* `1`; ALC
* `2`; compression
* `3`; drive
* `4`; PA plate current (requires KPA3A)
* `5`; SWR" =>
    GetTransmitMeterMode
);

define_cat_command!("Set transmit meter display mode (`TM`).

K3/K3S only.

# Command format

> `TM{n};`

Where *n* is one of:

* `0`; power out
* `1`; ALC
* `2`; compression
* `3`; drive
* `4`; PA plate current (requires KPA3A)
* `5`; SWR" =>
    SetTransmitMeterMode {
        mode: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransmitStatus
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get transceiver's current transmitting on/off state (`TQ`).

# Command format

> `TQ;`

# Response format

> `TQ{n};`

Where `n` is the boolean state `0` (receive) or `1` (transmit)." =>
    GetTransmitState
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetTextToTerminal
// ------------------------------------------------------------------------------------------------

define_cat_command!("Set decoded CW/RTTY text output to the serial port on/off (`TT`).

When on, decoded characters are sent unsolicited as `TBn ssssssss;` responses.

# Command format

> `TT{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetTextToTerminal { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GoToTransmit
// ------------------------------------------------------------------------------------------------

define_cat_command!("Assert PTT and enter transmit mode (`TX`).

Follow with [`GoToReceive`] to return to receive.

# Command format

> `TX;`" =>
    GoToTransmit
);

// ------------------------------------------------------------------------------------------------
// Public Types: VfoAFrequencyUp, VfoBFrequencyUp
// ------------------------------------------------------------------------------------------------

define_cat_command!("Move VFO A up by one tuning step (`UP`).

# Command format

> `UP;`" =>
    MoveVfoAFrequencyUp {
        step: Option<VfoFrequencyChangeStep>
    }
);

define_cat_command!("Move VFO B up by one tuning step (`UPB`).

# Command format

> `UPB;`" =>
    MoveVfoBFrequencyUp {
        step: Option<VfoFrequencyChangeStep>
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVox, SetVox
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Voice-Operated eXchange (VOX) on/off state (`VX`).

# Command format

> `VX;`

# Response format

> `VX{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetVox
);

define_cat_command!("Set Voice-Operated eXchange (VOX) on/off state (`VX`).

# Command format

> `VX{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetVox { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetXVfoAfilNumber, GetVfoBXfilNumber
// ------------------------------------------------------------------------------------------------

define_cat_command!("Read the current roofing filter (XFIL) slot number for VFO A (`XF`).

# Command format

> `XF;`

# Response format

> `XF{n};`

Where *n* is between `0` and `7`, indicating the selected XFIL slot." =>
    GetVfoAXfilNumber
);

define_cat_command!("Read the current roofing filter (XFIL) slot number for VFO B (`XF$`).

# Command format

> `XF$;`

# Response format

> `XF${n};`

Where *n* is between `0` and `7`, indicating the selected XFIL slot." =>
    GetVfoBXfilNumber
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetXitControl, SetXitControl
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get Transmit Incremental Tuning (XIT) on/off state (`XT`).

XIT shifts only the transmit frequency by the RIT/XIT offset ([`GetRitXitOffset`] / `RO`).

# Command format

> `XT;`

# Response format

> `XT{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetXitControl
);

define_cat_command!("Set Transmit Incremental Tuning (XIT) on/off state (`XT`).

# Command format

> `XT{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetXitControl { state }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetK3CommandMode => b"K3");
impl_cat_command_with_response!(GetK3CommandMode => try_from 1 K3CommandMode);

impl_cat_command!(SetK3CommandMode => b"K3" with Some |cmd: &SetK3CommandMode| {
    vec![if cmd.mode.extended { b'1' } else { b'0' }]
});

impl SetK3CommandMode {
    #[inline(always)]
    pub const fn to_extended() -> Self {
        Self {
            mode: K3CommandMode::extended(),
        }
    }

    #[inline(always)]
    pub const fn to_normal() -> Self {
        Self {
            mode: K3CommandMode::normal(),
        }
    }
}

impl Display for K3CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.extended {
            "K3 Extended Mode"
        } else {
            "K3 Normal Mode"
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K3CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K3CommandMode: expecting 1 byte, given {}", value.len());
            Err(invalid_response_length(1, value.len()))
        } else {
            Ok(Self {
                extended: value[0] == b'1',
            })
        }
    }
}

impl K3CommandMode {
    #[inline(always)]
    pub const fn extended() -> Self {
        Self { extended: true }
    }

    #[inline(always)]
    pub const fn normal() -> Self {
        Self { extended: false }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAAfGain => b"AG");
impl_cat_command_with_response!(GetVfoAAfGain => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetVfoAAfGain => b"AG"
    format level uint 3,
    if |cmd: &SetVfoAAfGain| validate_integer_in_range("level", "u8", cmd.level, 0, 255)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoBAfGain => b"AG$");
impl_cat_command_with_response!(GetVfoBAfGain => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetVfoBAfGain => b"AG$"
    format level uint 3,
    if |cmd: &SetVfoBAfGain| validate_integer_in_range("level", "u8", cmd.level, 0, 255)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAutoInfoMode => b"AI");
impl_cat_command_with_response!(GetAutoInfoMode => try_from enum AutoInfoMode);

impl_cat_command!(SetAutoInfoMode => b"AI" for as byte mode);

impl_set_cat_command_from_enum!(
    SetAutoInfoMode, AutoInfoMode => mode {
        Off => turn_off,
        K2 => to_k2_mode,
        K3 => to_k3_mode,
        K3Extended => to_k3_extended_mode
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAtuNetworkValues => b"AK");
impl_cat_command_with_response!(GetAtuNetworkValues => 6, |bytes: &[u8]| {
    Ok(AtuNetworkValues {
        capacitance_bitmap: u8_from_ascii(&bytes[0..2])?,
        inductance_bitmap: u8_from_ascii(&bytes[2..4])?,
        misc_relay_bitmap: u8_from_ascii(&bytes[4..6])?,
    })
} => AtuNetworkValues);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAudioPeakingFilterState => b"AP");
impl_cat_command_with_response!(GetAudioPeakingFilterState => boolean);

impl_cat_command!(SetAudioPeakingFilterState => b"AP" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetReceiveAntenna => b"AR");
impl_cat_command_with_response!(GetReceiveAntenna => boolean);

impl_cat_command!(SetReceiveAntenna => b"AR" for boolean rx_only);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBargraphValue => b"BG");
impl_cat_command_with_response!(GetBargraphValue => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoABandNumber => b"BN");
impl_cat_command_with_response!(GetVfoABandNumber => 2, parse_band_number => AllocationBand);

impl_cat_command!(GetVfoBBandNumber => b"BN$");
impl_cat_command_with_response!(GetVfoBBandNumber => 2, parse_band_number => AllocationBand);

impl_cat_command!(SetVfoABandNumber => b"BN" with |cmd: &SetVfoABandNumber| {
    Ok(Some(format_uint_ascii(allocation_band_code(cmd.band)?, 2)))
});

impl_cat_command!(SetVfoBBandNumber => b"BN$" with |cmd: &SetVfoBBandNumber| {
    Ok(Some(format_uint_ascii(allocation_band_code(cmd.band)?, 2)))
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SetBaudRate => b"BR" for as byte rate);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAFilterBandwidth => b"BW");
impl_cat_command_with_response!(GetVfoAFilterBandwidth => 4, u16_from_ascii => u16);

impl_cat_command!(GetVfoBFilterBandwidth => b"BW$");
impl_cat_command_with_response!(GetVfoBFilterBandwidth => 4, u16_from_ascii => u16);

impl_cat_command!(SetVfoAFilterBandwidth => b"BW" format bandwidth_10hz uint 4);

impl_cat_command!(SetVfoBFilterBandwidth => b"BW$" format bandwidth_10hz uint 4);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSpeechCompression => b"CP");
impl_cat_command_with_response!(GetSpeechCompression => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetSpeechCompression => b"CP"
    format level uint 2,
    if |cmd: &SetSpeechCompression| validate_integer_in_range("level", "u8", cmd.level, 0, 40)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetCwSidetonePitch => b"CW");
impl_cat_command_with_response!(GetCwSidetonePitch => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoBDisplayText => b"DB");
impl_cat_command_with_response!(GetVfoBDisplayText => 8, bytes_to_vec => Vec<u8>);

impl_cat_command!(SetVfoBDisplayText => b"DB" with Some |cmd: &SetVfoBDisplayText| {
    cmd.text.to_vec()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(
    SetCommandProcessingDelay => b"DE"
    format delay_5ms uint 2,
    if |cmd: &SetCommandProcessingDelay| validate_integer_in_range("delay_5ms", "u8", cmd.delay_5ms, 0, 99)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(
    SetDspCommandDebugState => b"DL" with Some |cmd: &SetDspCommandDebugState| {
        if cmd.on {
            vec![b'2']
        } else {
            vec![b'3']
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(MoveVfoAFrequencyDown => b"DN" with Some |cmd: &MoveVfoAFrequencyDown| {
    if let Some(step) = cmd.step {
        vec![step as u8]
    } else {
        vec![]
    }
});

impl MoveVfoAFrequencyDown {
    #[inline(always)]
    pub const fn step_by(step: VfoFrequencyChangeStep) -> Self {
        Self { step: Some(step) }
    }
    #[inline(always)]
    pub fn step_1hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1Hz)
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
    pub const fn step_100hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step100Hz)
    }
    #[inline(always)]
    pub const fn step_200hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step200Hz)
    }
    #[inline(always)]
    pub const fn step_50hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step50Hz)
    }
    #[inline(always)]
    pub const fn step_1khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1kHz)
    }
    #[inline(always)]
    pub const fn step_2khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step2kHz)
    }
    #[inline(always)]
    pub const fn step_3khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step3kHz)
    }
    #[inline(always)]
    pub const fn step_5khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step5kHz)
    }
}

impl_cat_command!(MoveVfoBFrequencyDown => b"DNB" with Some |cmd: &MoveVfoBFrequencyDown| {
        if let Some(step) = cmd.step {
        vec![step as u8]
    } else {
        vec![]
    }
});

impl MoveVfoBFrequencyDown {
    #[inline(always)]
    pub const fn step_by(step: VfoFrequencyChangeStep) -> Self {
        Self { step: Some(step) }
    }
    #[inline(always)]
    pub const fn step_1hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1Hz)
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
    pub const fn step_100hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step100Hz)
    }
    #[inline(always)]
    pub const fn step_200hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step200Hz)
    }
    #[inline(always)]
    pub const fn step_50hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step50Hz)
    }
    #[inline(always)]
    pub const fn step_1khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1kHz)
    }
    #[inline(always)]
    pub const fn step_2khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step2kHz)
    }
    #[inline(always)]
    pub const fn step_3khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step3kHz)
    }
    #[inline(always)]
    pub const fn step_5khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step5kHz)
    }
}

impl From<VfoFrequencyChangeStep> for Frequency {
    fn from(step: VfoFrequencyChangeStep) -> Self {
        Frequency(match step {
            VfoFrequencyChangeStep::Step1Hz => 1,
            VfoFrequencyChangeStep::Step10Hz => 10,
            VfoFrequencyChangeStep::Step20Hz => 20,
            VfoFrequencyChangeStep::Step50Hz => 50,
            VfoFrequencyChangeStep::Step100Hz => 100,
            VfoFrequencyChangeStep::Step200Hz => 200,
            VfoFrequencyChangeStep::Step1kHz => 1000,
            VfoFrequencyChangeStep::Step2kHz => 2000,
            VfoFrequencyChangeStep::Step3kHz => 3000,
            VfoFrequencyChangeStep::Step5kHz => 5000,
        })
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
            icon_data: VfoAIconData::try_from(bytes[8])?,
            icon_flash_data: VfoAIconFlashData::try_from(bytes[9])?,
        })

    } => VfoADisplayAndIcons
);

impl TryFrom<u8> for VfoAIconData {
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

impl TryFrom<u8> for VfoAIconFlashData {
    type Error = RigError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if parse_bit_flag!(value[7] OFF) {
            Err(invalid_response_data(&[value]))
        } else {
            Ok(Self {
                manual_notch_on: parse_bit_flag!(value[0] ON),
                notch_on: parse_bit_flag!(value[1] ON),
                noise_reduction_on: parse_bit_flag!(value[2] ON),
                cwt_on: parse_bit_flag!(value[3] ON),
                atu_on: parse_bit_flag!(value[4] ON),
                receive_antenna_on: parse_bit_flag!(value[5] ON),
                sub_on: parse_bit_flag!(value[6] ON),
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDataSubMode => b"DT");
impl_cat_command_with_response!(GetDataSubMode => try_from enum DataSubMode);

impl_cat_command!(SetDataSubMode => b"DT" for as byte sub_mode);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDiversityMode => b"DV");
impl_cat_command_with_response!(GetDiversityMode => try_from enum DiversityModeState);

impl_cat_command!(SetDiversityMode => b"DV" for as byte state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SetErrorLogging => b"EL" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetEssbMode => b"ES");
impl_cat_command_with_response!(GetEssbMode => boolean);

impl_cat_command!(SetEssbMode => b"ES" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetIfCenterFrequency => b"FI");
impl_cat_command_with_response!(GetIfCenterFrequency => 4, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetReceiveVfo => b"FR");
impl_cat_command_with_response!(GetReceiveVfo => 1, |bytes: &[u8]| {
    match bytes[0] {
        b'0' => Ok(Vfo::A),
        b'1' => Ok(Vfo::B),
        _ => {
            error!("GetReceiveVfo: unexpected VFO byte {:02X?}", bytes[0]);
            Err(RigError::InvalidResponseData { data: bytes.to_vec() })
        }
    }
} => Vfo);

impl_cat_command!(SetReceiveVfo => b"FR" with Some |cmd: &SetReceiveVfo| {
    vec![cmd.vfo  as u8]
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransmitVfoSplitModeState => b"FT");
impl_cat_command_with_response!(GetTransmitVfoSplitModeState => boolean);

impl_cat_command!(SetTransmitVfoSplitModeState => b"FT" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoALegacyFilterBandwidth => b"FW");
impl_cat_command_with_response!(GetVfoALegacyFilterBandwidth => 4, u16_from_ascii => u16);

impl_cat_command!(GetVfoBLegacyFilterBandwidth => b"FW$");
impl_cat_command_with_response!(GetVfoBLegacyFilterBandwidth => 4, u16_from_ascii => u16);

impl_cat_command!(SetVfoALegacyFilterBandwidth => b"FW" with Some |cmd: &SetVfoALegacyFilterBandwidth| {
    format!("{:04}", cmd.bandwidth_hz).into_bytes()
});

impl_cat_command!(SetVfoBLegacyFilterBandwidth => b"FW$" with Some |cmd: &SetVfoBLegacyFilterBandwidth| {
    format!("{:04}", cmd.bandwidth_hz).into_bytes()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAgcTimeConstant => b"GT");
impl_cat_command_with_response!(GetAgcTimeConstant => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetAgcTimeConstant => b"GT"
    format mode uint 2,
    if |cmd: &SetAgcTimeConstant| validate_integer_in_range("mode", "u8", cmd.mode, 0, 3)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetInstalledOptions => b"OM");
impl_cat_command_with_response!(GetInstalledOptions => try_from 13 InstalledOptions);

impl TryFrom<&[u8]> for InstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if !value.starts_with(b" ") {
            error!("InstalledOptions: missing leading space");
            return Err(RigError::InvalidResponseData {
                data: value.to_vec(),
            });
        }
        let inner = &value[1..];
        // K4 has '4' at position 7 of inner (product-ID byte)
        if inner.get(7) == Some(&b'4') {
            Ok(Self::K4(K4InstalledOptions::try_from(inner)?))
        // KX has '0' at position 10 of inner followed by product digit
        } else if inner.get(10) == Some(&b'0') {
            Ok(Self::KX(KXInstalledOptions::try_from(inner)?))
        } else {
            Ok(Self::K3(K3InstalledOptions::try_from(inner)?))
        }
    }
}

impl K3InstalledOptions {
    pub fn is_model_k3(&self) -> bool {
        !self.has_k3s
    }
    pub fn is_model_k3s(&self) -> bool {
        self.has_k3s
    }
}

impl TryFrom<&[u8]> for K3InstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            has_kat3a_atu: parse_installed_option!(has_kat3a_atu          => value[0] == b'A'),
            has_kpa3a_pa: parse_installed_option!(has_kpa3a_pa           => value[1] == b'P'),
            has_kxv3_xvtr: parse_installed_option!(has_kxv3_xvtr          => value[2] == b'X'),
            has_krx3a_sub_receiver: parse_installed_option!(has_krx3a_sub_receiver => value[3] == b'S'),
            has_kdvr3_dvr: parse_installed_option!(has_kdvr3_dvr          => value[4] == b'D'),
            has_kbpf3a_main_bpf: parse_installed_option!(has_kbpf3a_main_bpf    => value[5] == b'F'),
            has_kbpf3a_sub_bpf: parse_installed_option!(has_kbpf3a_sub_bpf     => value[6] == b'f'),
            has_kxv3b_low_noise_amp: parse_installed_option!(has_kxv3b_low_noise_amp=> value[7] == b'L'),
            has_ksyn3a: parse_installed_option!(has_ksyn3a             => value[8] == b'V'),
            has_k3s: parse_installed_option!(has_k3s                => value[9] == b'R'),
        })
    }
}

impl K4InstalledOptions {
    pub fn is_k4(&self) -> bool {
        !self.has_krx4_sub_receiver && !self.has_khdr4_hdr
    }
    pub fn is_k4d(&self) -> bool {
        self.has_krx4_sub_receiver && !self.has_khdr4_hdr
    }
    pub fn is_k4hd(&self) -> bool {
        self.has_krx4_sub_receiver && self.has_khdr4_hdr
    }
}

impl TryFrom<&[u8]> for K4InstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            has_kat4_atu: parse_installed_option!(has_kat4_atu          => value[0] == b'A'),
            has_kpa4_pa: parse_installed_option!(has_kpa4_pa           => value[1] == b'P'),
            has_xvtr: parse_installed_option!(has_xvtr              => value[2] == b'X'),
            has_krx4_sub_receiver: parse_installed_option!(has_krx4_sub_receiver => value[3] == b'S'),
            has_khdr4_hdr: parse_installed_option!(has_khdr4_hdr         => value[4] == b'H'),
            has_k40_mini: parse_installed_option!(has_k40_mini          => value[5] == b'M'),
            has_kpa_linear_pa: parse_installed_option!(has_kpa_linear_pa     => value[6] == b'L'),
            has_kpa1500_pa: parse_installed_option!(has_kpa1500_pa        => value[7] == b'1'),
        })
    }
}

impl KXInstalledOptions {
    pub fn is_model_kx2(&self) -> bool {
        !self.is_kx3
    }
    pub fn is_model_kx3(&self) -> bool {
        self.is_kx3
    }
}

impl TryFrom<&[u8]> for KXInstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            has_kxat_atu: parse_installed_option!(has_kxat_atu            => value[0] == b'A'),
            has_kxpa100_pa: parse_installed_option!(has_kxpa100_pa          => value[1] == b'P'),
            has_kxfl3_roofing_filter: parse_installed_option!(has_kxfl3_roofing_filter=> value[2] == b'F'),
            has_kxat100_external_atu: parse_installed_option!(has_kxat100_external_atu=> value[6] == b'T'),
            has_kxbc3_realtime_clock: parse_installed_option!(has_kxbc3_realtime_clock=> value[7] == b'B'),
            has_kx3m_transverter: parse_installed_option!(has_kx3m_transverter    => value[8] == b'X'),
            has_kxio2_rtc_io: parse_installed_option!(has_kxio2_rtc_io        => value[9] == b'I'),
            is_kx3: match value[11] {
                b'2' => true,
                b'1' => false,
                _ => {
                    error!("KXInstalledOptions: invalid product ID {:02X?}", value[11]);
                    return Err(RigError::InvalidResponseData {
                        data: value.to_vec(),
                    });
                }
            },
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetK3IconsAndStatus => b"IC");
impl_cat_command_with_response!(GetK3IconsAndStatus => try_from 5 K3IconsAndStatus);

impl TryFrom<&[u8]> for K3IconsAndStatus {
    type Error = RigError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if parse_bit_flag!(bytes[0:7] OFF)
            || parse_bit_flag!(bytes[1:7] OFF)
            || parse_bit_flag!(bytes[2:7] OFF)
            || parse_bit_flag!(bytes[3:7] OFF)
            || parse_bit_flag!(bytes[4:7] OFF)
        {
            error!("Missing error detection bit 7s.");
            Err(invalid_response_data(bytes))
        } else {
            Ok(K3IconsAndStatus {
                preset_1: parse_bit_flag!(bytes[0:0] OFF),
                preset_2: parse_bit_flag!(bytes[0:0] ON),
                band_sel: parse_bit_flag!(bytes[0:1] OFF),
                msg_playing: parse_bit_flag!(bytes[0:2] ON),
                message_bank_1: parse_bit_flag!(bytes[0:3] OFF),
                message_bank_2: parse_bit_flag!(bytes[0:3] ON),
                mw_power_level: parse_bit_flag!(bytes[0:4] ON),
                tx_test: parse_bit_flag!(bytes[0:5] ON),
                bset: parse_bit_flag!(bytes[0:6] ON),
                sub_rx_on: parse_bit_flag!(bytes[1:0]  ON),
                sub_rx_nb_on: parse_bit_flag!(bytes[1:1] ON),
                sub_rx_aux_bnc: parse_bit_flag!(bytes[1:2] ON),
                sub_ant_main: parse_bit_flag!(bytes[1:3] ON),
                sub_ant_aux: parse_bit_flag!(bytes[1:3] OFF),
                diversity_mode: parse_bit_flag!(bytes[1:4] ON),
                vfo_ab_bands_independent: parse_bit_flag!(bytes[1:5] ON),
                vfo_ab_linked: parse_bit_flag!(bytes[1:6] ON),
                text_to_terminal_on: parse_bit_flag!(bytes[2:0] ON),
                sync_data: parse_bit_flag!(bytes[2:1] ON),
                fsk_tx_polarity_normal: parse_bit_flag!(bytes[2:2] ON),
                fsk_dual_tone_filter_on: parse_bit_flag!(bytes[2:3] ON),
                vox_on_for_cw_fsk_psk: parse_bit_flag!(bytes[2:4]  ON),
                dual_passband_cf_apf_on: parse_bit_flag!(bytes[2:5] ON),
                full_qsk: parse_bit_flag!(bytes[2:6] ON),
                repeater_tx_offset_negative: parse_bit_flag!(bytes[3:0] ON),
                repeater_tx_offset_positive: parse_bit_flag!(bytes[3:1] ON),
                fm_pl_tone_on: parse_bit_flag!(bytes[3:2] ON),
                am_sync_rx: parse_bit_flag!(bytes[3:3] ON),
                noise_gate_on: parse_bit_flag!(bytes[3:4] ON),
                essb: parse_bit_flag!(bytes[3:5] ON),
                vox_on_for_voice_data_afsk: parse_bit_flag!(bytes[3:6] ON),
                fast_play_in_effect: parse_bit_flag!(bytes[4:0] ON),
                ofs_led_is_on: parse_bit_flag!(bytes[4:1] ON ),
                vfob_led_on: parse_bit_flag!(bytes[4:1] OFF),
                sub_rx_nr_on: parse_bit_flag!(bytes[4:2] ON),
                sub_rx_squelched: parse_bit_flag!(bytes[4:3] ON),
                main_rx_squelched: parse_bit_flag!(bytes[4:4] ON),
                am_sync_usb: parse_bit_flag!(bytes[4:5] ON),
                am_sync_lsb: parse_bit_flag!(bytes[4:5] OFF),
                shift_10_hz: parse_bit_flag!(bytes[4:6] ON),
                shift_50_hz: parse_bit_flag!(bytes[4:6] OFF),
            })
        }
    }
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
        let data_sub_mode = if value[32] != b'0' {
            Some(DataSubMode::from_repr(value[32]).ok_or(enum_parse(value[32], "DataSubMode"))?)
        } else {
            None
        };
        assert_all_bytes_eq(&value[11..16], b' ')?;
        assert_byte_eq(value[23], b' ')?;
        assert_all_bytes_eq(&value[24..26], b'0')?;
        assert_byte_eq(value[33], b'1')?;
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
            data_sub_mode,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAOperatingMode => b"MD");
impl_cat_command_with_response!(GetVfoAOperatingMode => try_from enum OperatingMode);

impl_cat_command!(GetVfoBOperatingMode => b"MD$");
impl_cat_command_with_response!(GetVfoBOperatingMode => try_from enum OperatingMode);

impl_cat_command!(SetVfoAOperatingMode => b"MD" for as byte mode);

impl_set_cat_command_from_enum!(SetVfoAOperatingMode, OperatingMode => mode {
    LowerSideBand => to_lower_sideband,
    UpperSideBand => to_upper_sideband,
    ContinuousWave => to_cw,
    FrequencyModulation => to_fm,
    AmplitudeModulation => to_am,
    DataA => to_data_a,
    ContinuousWaveReverse => to_cw_reverse,
    DataB => to_data_b
});

impl_cat_command!(SetVfoBOperatingMode => b"MD$" for as byte mode);

impl_set_cat_command_from_enum!(SetVfoBOperatingMode, OperatingMode => mode {
    LowerSideBand => to_lower_sideband,
    UpperSideBand => to_upper_sideband,
    ContinuousWave => to_cw,
    FrequencyModulation => to_fm,
    AmplitudeModulation => to_am,
    DataA => to_data_a,
    ContinuousWaveReverse => to_cw_reverse,
    DataB => to_data_b
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAIfShift => b"IS");
impl_cat_command_with_response!(GetVfoAIfShift => 5, parse_signed_offset_4 => i16);

impl_cat_command!(GetVfoBIfShift => b"IS$");
impl_cat_command_with_response!(GetVfoBIfShift => 5, parse_signed_offset_4 => i16);

impl_cat_command!(
    SetVfoAIfShift => b"IS"
    format offset_hz int 4,
    if |cmd: &SetVfoAIfShift| validate_integer_in_range("offset_hz", "i16", cmd.offset_hz, -2999, 2999)
);

impl_cat_command!(
    SetVfoBIfShift => b"IS$"
    format offset_hz int 4,
    if |cmd: &SetVfoBIfShift| validate_integer_in_range("offset_hz", "i16", cmd.offset_hz, -2999, 2999)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetKeyerSpeed => b"KS");
impl_cat_command_with_response!(GetKeyerSpeed => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetKeyerSpeed => b"KS"
    format wpm uint 3,
    if |cmd: &SetKeyerSpeed| validate_integer_in_range("wpm", "u8", cmd.wpm, 8, 100)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SendCwText => b"KY" with Some |cmd: &SendCwText| {
    let flag = if cmd.buffer_only { b'1' } else { b'0' };
    let mut v = vec![flag, b' '];
    let len = cmd.text.len().min(24);
    v.extend_from_slice(&cmd.text[..len]);
    while v.len() < 26 {
        v.push(b' ');
    }
    v
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoALockState => b"LK");
impl_cat_command_with_response!(GetVfoALockState => boolean);

impl_cat_command!(GetVfoBLockState => b"LK$");
impl_cat_command_with_response!(GetVfoBLockState => boolean);

impl_cat_command!(SetVfoALockState => b"LK" for state);

impl_cat_command!(SetVfoBLockState => b"LK$" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoLinkedState => b"LN");
impl_cat_command_with_response!(GetVfoLinkedState => boolean);

impl_cat_command!(SetVfoLinkedState => b"LN" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMemoryChannel => b"MC");
impl_cat_command_with_response!(GetMemoryChannel => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetMemoryChannel => b"MC"
    format channel uint 3,
    if |cmd: &SetMemoryChannel| validate_integer_in_range("channel", "u8", cmd.channel, 1, 100)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMicGain => b"MG");
impl_cat_command_with_response!(GetMicGain => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetMicGain => b"MG"
    format gain uint 3,
    if |cmd: &SetMicGain| validate_integer_in_range("gain", "u8", cmd.gain, 0, 60)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMonitorLevel => b"ML");
impl_cat_command_with_response!(GetMonitorLevel => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetMonitorLevel => b"ML"
    format level uint 3,
    if |cmd: &SetMonitorLevel| validate_integer_in_range("level", "u8", cmd.level, 0, 60)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(
    SelectMenuItem => b"MN"
    format item uint 3,
    if |cmd: &SelectMenuItem| validate_integer_in_range("item", "u16", cmd.item, 0, 999)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMenuParameter => b"MP");
impl_cat_command_with_response!(GetMenuParameter => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetMenuParameter => b"MP"
    format value uint 2,
    if |cmd: &SetMenuParameter| validate_integer_in_range("value", "u8", cmd.value, 0, 99)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetMenuParameter16 => b"MQ");
impl_cat_command_with_response!(GetMenuParameter16 => 4, u16_from_ascii => u16);

impl_cat_command!(
    SetMenuParameter16 => b"MQ"
    format value uint 4,
    if |cmd: &SetMenuParameter16| validate_integer_in_range("value", "u16", cmd.value, 0, 9999)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoANoiseBlanker => b"NB");
impl_cat_command_with_response!(GetVfoANoiseBlanker => boolean);

impl_cat_command!(GetVfoBNoiseBlanker => b"NB$");
impl_cat_command_with_response!(GetVfoBNoiseBlanker => boolean);

impl_cat_command!(SetVfoANoiseBlanker => b"NB" for state);

impl_cat_command!(SetVfoBNoiseBlanker => b"NB$" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoANoiseBlankerLevel => b"NL");
impl_cat_command_with_response!(GetVfoANoiseBlankerLevel => 2, u8_from_ascii => u8);

impl_cat_command!(GetVfoBNoiseBlankerLevel => b"NL$");
impl_cat_command_with_response!(GetVfoBNoiseBlankerLevel => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetVfoANoiseBlankerLevel => b"NL"
    format level uint 2,
    if |cmd: &SetVfoANoiseBlankerLevel| validate_integer_in_range("level", "u8", cmd.level, 0, 21)
);

impl_cat_command!(
    SetVfoBNoiseBlankerLevel => b"NL$"
    format level uint 2,
    if |cmd: &SetVfoBNoiseBlankerLevel| validate_integer_in_range("level", "u8", cmd.level, 0, 21)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAPreamp => b"PA");
impl_cat_command_with_response!(GetVfoAPreamp => 1, u8_from_ascii => u8);

impl_cat_command!(GetVfoBPreamp => b"PA$");
impl_cat_command_with_response!(GetVfoBPreamp => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetVfoAPreamp => b"PA"
    format preamp uint 1,
    if |cmd: &SetVfoAPreamp| validate_integer_in_range("preamp", "u8", cmd.preamp, 0, 2)
);

impl_cat_command!(
    SetVfoBPreamp => b"PA$"
    format preamp uint 1,
    if |cmd: &SetVfoBPreamp| validate_integer_in_range("preamp", "u8", cmd.preamp, 0, 2)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransmitPowerControl => b"PC");
impl_cat_command_with_response!(GetTransmitPowerControl => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetTransmitPowerControl => b"PC"
    format watts uint 3,
    if |cmd: &SetTransmitPowerControl| validate_integer_in_range("watts", "u8", cmd.watts, 0, 110)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetActualPowerOutput => b"PO");
impl_cat_command_with_response!(GetActualPowerOutput => 4, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerStatus => b"PS");
impl_cat_command_with_response!(GetPowerStatus => boolean);

impl_cat_command!(SetPowerStatus => b"PS" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAReceiveAttenuator => b"RA");
impl_cat_command_with_response!(GetVfoAReceiveAttenuator => 2, u8_from_ascii => u8);

impl_cat_command!(GetVfoBReceiveAttenuator => b"RA$");
impl_cat_command_with_response!(GetVfoBReceiveAttenuator => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetVfoAReceiveAttenuator => b"RA"
    format level uint 2,
    if |cmd: &SetVfoAReceiveAttenuator| validate_integer_in_range("level", "u8", cmd.level, 0, 15)
);

impl_cat_command!(
    SetVfoBReceiveAttenuator => b"RA$"
    format level uint 2,
    if |cmd: &SetVfoBReceiveAttenuator| validate_integer_in_range("level", "u8", cmd.level, 0, 15)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(ClearRitOffset => b"RC");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(MoveRitOffsetDown => b"RD"
    format hz uint 4,
    if |cmd: &MoveRitOffsetDown| validate_integer_in_range("hz", "u8", cmd.hz, 0, 9999)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoARfGain => b"RG");
impl_cat_command_with_response!(GetVfoARfGain => 3, u8_from_ascii => u8);

impl_cat_command!(GetVfoBRfGain => b"RG$");
impl_cat_command_with_response!(GetVfoBRfGain => 3, u8_from_ascii => u8);

impl_cat_command!(SetVfoARfGain => b"RG"
    format gain uint 3,
    if |cmd: &SetVfoARfGain| validate_integer_in_range("gain", "u8", cmd.gain, 190, 250)
);

impl_cat_command!(SetVfoBRfGain => b"RG$"
    format gain uint 3,
    if |cmd: &SetVfoBRfGain| validate_integer_in_range("gain", "u8", cmd.gain, 190, 250)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetRitXitOffset => b"RO");
impl_cat_command_with_response!(GetRitXitOffset => 5, parse_signed_offset_4 => i16);

impl_cat_command!(
    SetRitXitOffset => b"RO"
    format offset_hz int 4,
    if |cmd: &SetRitXitOffset| validate_integer_in_range("offset_hz", "i16", cmd.offset_hz, -9999, 9999)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetRitControl => b"RT");
impl_cat_command_with_response!(GetRitControl => boolean);

impl_cat_command!(SetRitControl => b"RT" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(
    MoveRitOffsetUp => b"RU"
    format hz uint 4,
    if |cmd: &MoveRitOffsetUp| validate_integer_in_range("hz", "u16", cmd.hz, 0, 9999)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFirmwareRevision => b"RV");
impl_cat_command_with_response!(GetFirmwareRevision => 11, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GoToReceive => b"RX");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSubReceiver => b"SB");
impl_cat_command_with_response!(GetSubReceiver => boolean);

impl_cat_command!(SetSubReceiver => b"SB" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetQskDelay => b"SD");
impl_cat_command_with_response!(GetQskDelay => 4, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoASMeter => b"SM");
impl_cat_command_with_response!(GetVfoASMeter => 2, u8_from_ascii => u8);

impl_cat_command!(GetVfoBSMeter => b"SM$");
impl_cat_command_with_response!(GetVfoBSMeter => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetHighResolutionSMeter => b"SMH");
impl_cat_command_with_response!(GetHighResolutionSMeter => 4, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoASquelch => b"SQ");
impl_cat_command_with_response!(GetVfoASquelch => 2, u8_from_ascii => u8);

impl_cat_command!(GetVfoBSquelch => b"SQ$");
impl_cat_command_with_response!(GetVfoBSquelch => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetVfoASquelch => b"SQ"
    format level uint 2,
    if |cmd: &SetVfoASquelch| validate_integer_in_range("level", "u8", cmd.level, 0, 29)
);

impl_cat_command!(
    SetVfoBSquelch => b"SQ$"
    format level uint 2,
    if |cmd: &SetVfoBSquelch| validate_integer_in_range("level", "u8", cmd.level, 0, 29)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(EmulateButtonTap => b"SWT" format button uint 2);

impl_cat_command!(EmulateButtonHold => b"SWH" format button uint 2);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBufferedText => b"TB");
impl_cat_command_with_response!(GetBufferedText => 9, parse_buffered_text => (u8, Vec<u8>));

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransmitBufferedText => b"TBX");
impl_cat_command_with_response!(GetTransmitBufferedText => 9, parse_buffered_text => (u8, Vec<u8>));

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SetTransmitEqualizer => b"TE" with Some |cmd: &SetTransmitEqualizer| {
    cmd.params.clone()
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransmitMeterMode => b"TM");
impl_cat_command_with_response!(GetTransmitMeterMode => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetTransmitMeterMode => b"TM"
    format mode uint 1,
    if |cmd: &SetTransmitMeterMode| validate_integer_in_range("mode", "u8", cmd.mode, 0, 5)
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTransmitState => b"TQ");
impl_cat_command_with_response!(GetTransmitState => boolean);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(SetTextToTerminal => b"TT" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GoToTransmit => b"TX");

// ------------------------------------------------------------------------------------------------

impl_cat_command!(MoveVfoAFrequencyUp => b"UP" with Some |cmd: &MoveVfoAFrequencyUp| {
    if let Some(step) = cmd.step {
        vec![step as u8]
    } else {
        vec![]
    }
});

impl MoveVfoAFrequencyUp {
    #[inline(always)]
    pub const fn step_by(step: VfoFrequencyChangeStep) -> Self {
        Self { step: Some(step) }
    }
    #[inline(always)]
    pub const fn step_1hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1Hz)
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
    pub const fn step_100hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step100Hz)
    }
    #[inline(always)]
    pub const fn step_200hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step200Hz)
    }
    #[inline(always)]
    pub const fn step_50hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step50Hz)
    }
    #[inline(always)]
    pub const fn step_1khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1kHz)
    }
    #[inline(always)]
    pub const fn step_2khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step2kHz)
    }
    #[inline(always)]
    pub const fn step_3khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step3kHz)
    }
    #[inline(always)]
    pub const fn step_5khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step5kHz)
    }
}

impl_cat_command!(MoveVfoBFrequencyUp => b"UPB" with Some |cmd: &MoveVfoBFrequencyUp| {
    if let Some(step) = cmd.step {
        vec![step as u8]
    } else {
        vec![]
    }
});

impl MoveVfoBFrequencyUp {
    #[inline(always)]
    pub const fn step_by(step: VfoFrequencyChangeStep) -> Self {
        Self { step: Some(step) }
    }
    #[inline(always)]
    pub const fn step_1hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1Hz)
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
    pub const fn step_100hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step100Hz)
    }
    #[inline(always)]
    pub const fn step_200hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step200Hz)
    }
    #[inline(always)]
    pub const fn step_50hz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step50Hz)
    }
    #[inline(always)]
    pub const fn step_1khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step1kHz)
    }
    #[inline(always)]
    pub const fn step_2khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step2kHz)
    }
    #[inline(always)]
    pub const fn step_3khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step3kHz)
    }
    #[inline(always)]
    pub const fn step_5khz() -> Self {
        Self::step_by(VfoFrequencyChangeStep::Step5kHz)
    }
}

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVox => b"VX");
impl_cat_command_with_response!(GetVox => boolean);

impl_cat_command!(SetVox => b"VX" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetVfoAXfilNumber => b"XF");
impl_cat_command_with_response!(GetVfoAXfilNumber => 1, u8_from_ascii => u8);

impl_cat_command!(GetVfoBXfilNumber => b"XF$");
impl_cat_command_with_response!(GetVfoBXfilNumber => 1, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetXitControl => b"XT");
impl_cat_command_with_response!(GetXitControl => boolean);

impl_cat_command!(SetXitControl => b"XT" for state);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

/// Parse a two-digit `BN` band code into an [`AllocationBand`].
///
/// Codes `00`-`14` map onto documented amateur bands. Code `18` (general coverage) has no
/// [`AllocationBand`] equivalent and is reported as an error.
fn parse_band_number(response: &[u8]) -> Result<AllocationBand, RigError> {
    match response {
        b"00" => Ok(AllocationBand::Band160M),
        b"01" => Ok(AllocationBand::Band80M),
        b"02" => Ok(AllocationBand::Band60M),
        b"03" => Ok(AllocationBand::Band40M),
        b"04" => Ok(AllocationBand::Band30M),
        b"05" => Ok(AllocationBand::Band20M),
        b"06" => Ok(AllocationBand::Band17M),
        b"07" => Ok(AllocationBand::Band15M),
        b"08" => Ok(AllocationBand::Band12M),
        b"09" => Ok(AllocationBand::Band10M),
        b"10" => Ok(AllocationBand::Band6M),
        b"11" => Ok(AllocationBand::Band2M),
        b"12" => Ok(AllocationBand::Band1_25M),
        b"13" => Ok(AllocationBand::Band70Cm),
        b"14" => Ok(AllocationBand::Band1_2Cm),
        _ => {
            error!(
                "Invalid or unrepresentable BN response data {:02X?} (band code `18`/general \
                 coverage has no AllocationBand equivalent)",
                response
            );
            Err(RigError::InvalidResponseData {
                data: response.to_vec(),
            })
        }
    }
}

/// Convert an [`AllocationBand`] back into its two-digit `BN` band code.
///
/// The inverse of [`parse_band_number`] for the bands it can represent; bands with no `BN` code
/// (anything outside the documented `00`-`14` range) are reported as an error.
fn allocation_band_code(band: AllocationBand) -> Result<u8, RigError> {
    match band {
        AllocationBand::Band160M => Ok(0),
        AllocationBand::Band80M => Ok(1),
        AllocationBand::Band60M => Ok(2),
        AllocationBand::Band40M => Ok(3),
        AllocationBand::Band30M => Ok(4),
        AllocationBand::Band20M => Ok(5),
        AllocationBand::Band17M => Ok(6),
        AllocationBand::Band15M => Ok(7),
        AllocationBand::Band12M => Ok(8),
        AllocationBand::Band10M => Ok(9),
        AllocationBand::Band6M => Ok(10),
        AllocationBand::Band2M => Ok(11),
        AllocationBand::Band1_25M => Ok(12),
        AllocationBand::Band70Cm => Ok(13),
        AllocationBand::Band1_2Cm => Ok(14),
        _ => Err(RigError::InvalidArgumentValue {
            argument_name: "band",
            type_name: "AllocationBand",
            value: format!("{band:?}"),
        }),
    }
}

/// Parse a 5-byte sign + 4-digit field (e.g. `IS`, `RO`) into a signed offset in Hz.
fn parse_signed_offset_4(bytes: &[u8]) -> Result<i16, RigError> {
    let sign = sign_from_ascii_loose(bytes[0])?;
    let magnitude = i32::from(u16_from_ascii(&bytes[1..5])?);
    i16::try_from(sign * magnitude).map_err(|_| RigError::InvalidResponseData {
        data: bytes.to_vec(),
    })
}

/// Parse a `TB`/`TBX`-style buffered-text response: a status digit, a space, and up to 8 ASCII
/// characters of text terminated by a space.
fn parse_buffered_text(bytes: &[u8]) -> Result<(u8, Vec<u8>), RigError> {
    let status = u8_from_ascii(&bytes[0..1])?;
    let text = bytes
        .get(2..)
        .unwrap_or(&[])
        .iter()
        .copied()
        .take_while(|&b| b != b' ')
        .collect();
    Ok((status, text))
}
