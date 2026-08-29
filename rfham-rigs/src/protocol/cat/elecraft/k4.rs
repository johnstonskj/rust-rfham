//!
//! CAT commands specific to or extended on the Elecraft K4 transceiver.
//!
//! Many K3/KX commands from [`super::k3_kx`] also work unchanged on the K4; this module documents
//! only commands that are unique to the K4, or that differ significantly in format or range from
//! their K3/KX counterparts.
//!
//! The K4 meta-command mode (K41) is set via [`super::meta::SetK4CommandMode`]; many commands below
//! require K41 mode to be active.
//!
//! Where a command addresses a specific VFO, the wire protocol distinguishes VFO A and VFO B by
//! appending a `$` to the command identifier (e.g. `FP` vs `FP$`) rather than by an argument byte.
//! This module therefore models each VFO as a distinct command type (e.g. [`GetVfoAFilterPresetSlot`]
//! and [`GetVfoBFilterPresetSlot`]) instead of a single type carrying a VFO field.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [K4 Programmer's Reference, rev. D11](https://ftp.elecraft.com/K4/Manuals%20Downloads/K4%20Programmer's%20Reference,%20rev.%20D12.pdf), May 2026.
//! 2. [K4 Programmer's Reference, rev. C7](https://lutz-electronics.ch/pdf/ELECRAFT/K4_Programmers_Reference_rev.C7.pdf), 2022.
//!

use crate::{
    error::RigError,
    protocol::cat::{
        Command, CommandWithResponse,
        common::{
            bytes_to_vec, format_int_ascii, sign_from_ascii_strict, u8_from_ascii, u16_from_ascii,
            u32_from_ascii, validate_response,
        },
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: CopyVfoAToB, SwapVfoAB
// ------------------------------------------------------------------------------------------------

define_command!("Copy VFO A's frequency to VFO B (`AB0`).

# Command format

> `AB0;`

The trailing `0` distinguishes this action from [`SwapVfoAandVfoB`], which shares the same `AB` command
identifier." =>
    CopyVfoAtoVfoB
);

define_command!("Swap the frequencies of VFO A and VFO B (`AB1`).

# Command format

> `AB1;`

The trailing `1` distinguishes this action from [`CopyVfoAtoVfoB`], which shares the same `AB` command
identifier." =>
    SwapVfoAandVfoB
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAtuMode, SetAtuMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the ATU (antenna tuner) mode (`AT`).

# Command format

> `AT;`

# Response format

> `AT{n};`

Where *n* is one of [`AtuMode`]." =>
    GetAtuMode
);

define_command!("Set the ATU (antenna tuner) mode (`AT`).

# Command format

> `AT{n};`

Where *n* is one of[`AtuMode`]." =>
    SetAtuMode {
        mode: AtuMode
    }
);

define_command_enum!(
    "Represents the ATU mode for [`GetAtuMode`] and [`SetAtuMode`]."=>
    AtuMode {
        "ATU bypassed" => Bypassed = b'0',
        "ATU in-line (auto-tunes on transmit)" => Inline = b'1',
        "ATU tuning in progress" => Tuning = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandIndependence, SetBandIndependence
// ------------------------------------------------------------------------------------------------

define_command!("Get the band independence state (`BI`).

# Command format

> `BI;`

# Response format

> `BI{n};`

Where `n` is the boolean state `0` (off; VFO A and B share a band) or `1` (on; each VFO can be on a
different band independently)." =>
    GetBandIndependenceState
);

define_command!("Set the band independence state (`BI`).

# Command format

> `BI{n};`

Where `n` is the boolean state `0` (off; VFO A and B share a band) or `1` (on; each VFO can be on a
different band independently)." =>
    SetBandIndependenceState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetCwSidetonePitch
// ------------------------------------------------------------------------------------------------

define_command!("Set the CW sidetone pitch in Hz (`CW`).

On the K4 this command also sets the received CW pitch offset used for zero-beat tuning; on K3/KX
transceivers the equivalent command is GET only.

# Command format

> `CW{nnn};`

Where *nnn* is the pitch, between `300` and `800` Hz." =>
    SetCwSidetonePitch {
        pitch_hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDigitalAudio, SetDigitalAudio
// ------------------------------------------------------------------------------------------------

define_command!("Get the digital audio routing mode (`DA`).

# Command format

> `DA;`

# Response format

> `DA{n};`

Where *n* is one of [`DigitalAudioRoutingMode`]." =>
    GetDigitalAudioRoutingMode
);

define_command!("Set the digital audio routing mode (`DA`).

# Command format

> `DA{n};`

Where *n* is one of [`DigitalAudioRoutingMode`]." =>
    SetDigitalAudioRoutingMode {
        mode: DigitalAudioRoutingMode
    }
);

define_command_enum!(
    "Digital audio routing mode for [`GetDigitalAudioRoutingMode`] and [`SetDigitalAudioRoutingMode`]." =>
    DigitalAudioRoutingMode {
        "Analog audio routing." => Analog = b'0',
        "Digital audio out to USB." => DigitalOut = b'1',
        "Digital audio in from USB." => DigitalIn = b'2',
        "Full digital audio I/O via USB." => FullDigital = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDigOut1, SetDigOut1
// ------------------------------------------------------------------------------------------------

define_command!("Get the DIGOUT1 (digital output pin 1) state (`DO`).

# Command format

> `DO;`

# Response format

> `DO{n};`

Where `n` is the boolean state `0` (low) or `1` (high)." =>
    GetDigitalOutputPin1State
);

define_command!("Set the DIGOUT1 (digital output pin 1) state (`DO`).

# Command format

> `DO{n};`

Where `n` is the boolean state `0` (low) or `1` (high)." =>
    SetDigitalOutputPin1State {
        high: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTxDataBandwidth, SetTxDataBandwidth
// ------------------------------------------------------------------------------------------------

define_command!("Get the TX DATA (AFSK/FSK/PSK) bandwidth (`DW`).

# Command format

> `DW;`

# Response format

> `DW{nnnn};`

Where *nnnn* is the bandwidth, between `0000` and `9999`, in units of 10 Hz." =>
    GetTransmitDataBandwidth
);

define_command!("Set the TX DATA (AFSK/FSK/PSK) bandwidth (`DW`).

# Command format

> `DW{nnnn};`

Where *nnnn* is the bandwidth, between `0000` and `9999`, in units of 10 Hz." =>
    SetTransmitDataBandwidth {
        bandwidth_10hz: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetCommandEcho
// ------------------------------------------------------------------------------------------------

define_command!("Set whether commands received on RS-232 are echoed back to the sender (`EC`).

# Command format

> `EC{n};`

Where `n` is the boolean state `0` off or `1` on, echo commands back to sender." =>
    SetCommandEchoState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetErrorReporting, SetErrorReporting
// ------------------------------------------------------------------------------------------------

define_command!("Get whether unsolicited error reports are enabled (`ER`).

# Command format

> `ER;`

# Response format

> `ER{n};`

Where `n` is the boolean state `0` (disabled) or `1` (enabled; error RSPs are sent unsolicited to
the serial port when command errors occur)." =>
    GetErrorReportingState
);

define_command!("Set whether unsolicited error reports are enabled (`ER`).

# Command format

> `ER{n};`

Where `n` is the boolean state `0` (disabled) or `1` (enabled)." =>
    SetErrorReportingState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: CenterPanadapterA, CenterPanadapterB
// ------------------------------------------------------------------------------------------------

define_command!("Center the panadapter on VFO A's current frequency (`FC`).

# Command format

> `FC;`" =>
    CenterPanadapterOnVfoA
);

define_command!("Center the panadapter on VFO B's current frequency (`FC$`).

# Command format

> `FC$;`" =>
    CenterPanadapterOnVfoB
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAFilterPresetSlot, GetVfoBFilterPresetSlot, SetVfoAFilterPresetSlot,
//      SetVfoBFilterPresetSlot
// ------------------------------------------------------------------------------------------------

define_command!("Get the active filter preset slot for VFO A (`FP`).

# Command format

> `FP;`

# Response format

> `FP{n};`

Where *n* is the preset slot, between `1` and `8`." =>
    GetVfoAFilterPresetSlot
);

define_command!("Get the active filter preset slot for VFO B (`FP$`).

# Command format

> `FP$;`

# Response format

> `FP${n};`

Where *n* is the preset slot, between `1` and `8`." =>
    GetVfoBFilterPresetSlot
);

define_command!("Set the active filter preset slot for VFO A (`FP`).

# Command format

> `FP{n};`

Where *n* is the preset slot, between `1` and `8`." =>
    SetVfoAFilterPresetSlot {
        preset: u8
    }
);

define_command!("Set the active filter preset slot for VFO B (`FP$`).

# Command format

> `FP${n};`

Where *n* is the preset slot, between `1` and `8`." =>
    SetVfoBFilterPresetSlot {
        preset: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAAgcMode, GetVfoBAgcMode, SetVfoAAgcMode, SetVfoBAgcMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the AGC mode for VFO A via the K4-extended command (`GT`).

K4 extended command; K41 mode required.

# Command format

> `GT;`

# Response format

> `GT{nn};`

Where *nn* is one of [`AgcMode`]." =>
    GetVfoAAgcMode
);

define_command!("Get the AGC mode for VFO B via the K4-extended command (`GT$`).

K4 extended command; K41 mode required.

# Command format

> `GT$;`

# Response format

> `GT${nn};`

Where *nn* is one of [`AgcMode`]." =>
    GetVfoBAgcMode
);

define_command!("Set the AGC mode for VFO A via the K4-extended command (`GT`).

K4 extended command; K41 mode required.

# Command format

> `GT{nn};`

Where *nn* is one of [`AgcMode`]." =>
    SetVfoAAgcMode {
        mode: u8
    }
);

define_command!("Set the AGC mode for VFO B via the K4-extended command (`GT$`).

K4 extended command; K41 mode required.

# Command format

> `GT${nn};`

Where *nn* is one of [`AgcMode`]." =>
    SetVfoBAgcMode {
        mode: u8
    }
);

define_command_enum!("AGC mode for [`GetVfoAAgcMode`], [`GetVfoBAgcMode`], [`SetVfoAAgcMode`], and [`SetVfoBAgcMode`]." =>
    AgcMode {
        "AGC off." => Off = b'0',
        "Fast AGC." => Fast = b'1',
        "Slow AGC." => Slow = b'2',
        "Auto AGC." => Auto = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransceiverId
// ------------------------------------------------------------------------------------------------

define_command!("Get the radio identification string (`ID`).

K4 extended command; K41 mode required.

# Command format

> `ID;`

# Response format

> `ID{nnn};`

Where *nnn* is a numeric model ID; the K4 returns `018`, the K3 returns `017`." =>
    GetTransceiverId
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetK4IfCenterPitchA, GeGetVfoAIfCenterPitch
// ------------------------------------------------------------------------------------------------

define_command!("Get the IF center pitch for VFO A (`IS`).

K4 extended command; K41 mode required.

# Command format

> `IS;`

# Response format

> `IS{±nnnn};`

Where *±nnnn* is the sign-prefixed pitch offset, in Hz, from the IF center frequency." =>
    GetVfoAIfCenterPitch
);

define_command!("Get the IF center pitch for VFO B (`IS$`).

K4 extended command; K41 mode required.

# Command format

> `IS$;`

# Response format

> `IS${±nnnn};`

Where *±nnnn* is the sign-prefixed pitch offset, in Hz, from the IF center frequency." =>
    GetVfoBIfCenterPitch
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetKeyerPaddleEmulationMode, SetKeyerPaddleEmulationMode, KeyerPaddleEmulationMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the keyer paddle emulation mode (`KP`).

# Command format

> `KP;`

# Response format

> `KP{n};`

Where *n* is the keyer paddle mode; see [`KeyerPaddleEmulationMode`]." =>
    GetKeyerPaddleEmulationMode
);

define_command!("Set the keyer paddle emulation mode (`KP`) .

# Command format

> `KP{n};`

Where *n* is the keyer paddle mode; see [`KeyerPaddleEmulationMode`]." =>
    SetKeyerPaddleEmulationMode {
        mode: KeyerPaddleEmulationMode
    }
);

define_command_enum!("Keyer paddle emulation mode (K4 only)." =>
    KeyerPaddleEmulationMode {
        "Normal paddle operation." => Normal = b'0',
        "Dit-only; the paddle emulates a straight key sending continuous dits." => DitOnly = b'1',
        "Dah-only; the paddle emulates a straight key sending continuous dahs." => DahOnly = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetK4KeyerSpeed
// ------------------------------------------------------------------------------------------------

define_command!("Set the keyer speed in WPM (`KS`).

K4 extended command; K41 mode required.

On K3/KX transceivers the equivalent set command is limited to 8-50 WPM; use
[`super::k3_kx::GetKeyerSpeed`] to query the current speed.

# Command format

> `KS{nnn};`

Where *nnn* is the speed, between `008` and `100` WPM." =>
    SetKeyerSpeed {
        wpm: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAudioLineInputLevel, SetAudioLineInputLevel
// ------------------------------------------------------------------------------------------------

define_command!("Get the line audio input level (`LI`).

# Command format

> `LI;`

# Response format

> `LI{nnn};`

Where *nnn* is the level, between `000` and `060`." =>
    GetAudioLineInputLevel
);

define_command!("Set the line audio input level (`LI`).

# Command format

> `LI{nnn};`

Where *nnn* is the level, between `000` and `060`." =>
    SetAudioLineInputLevel {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAudioLineOutputLevel, SetAudioLineOutputLevel
// ------------------------------------------------------------------------------------------------

define_command!("Get the line audio output level (`LO`).

# Command format

> `LO;`

# Response format

> `LO{nnn};`

Where *nnn* is the level, between `000` and `060`." =>
    GetAudioLineOutputLevel
);

define_command!("Set the line audio output level (`LO`).

# Command format

> `LO{nnn};`

Where *nnn* is the level, between `000` and `060`." =>
    SetAudioLineOutputLevel {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAModeAlternates, GetVfoBModeAlternatesGetVfoAModeAlternates
// ------------------------------------------------------------------------------------------------

define_command!("Get the list of mode alternates available on the current band, for VFO A (`MA`).

# Command format

> `MA;`

# Response format

> `MA...;`

The response is a variable-length string of mode character codes indicating which modes are
available on the current band, returned as raw bytes." =>
    GetVfoAModeAlternates
);

define_command!("Get the list of mode alternates available on the current band, for VFO B (`MA$`).

# Command format

> `MA$;`

# Response format

> `MA$...;`

The response is a variable-length string of mode character codes indicating which modes are
available on the current band, returned as raw bytes." =>
    GetVfoBModeAlternates
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMicInputSource, SetMicInputSource, MicInputSource
// ------------------------------------------------------------------------------------------------

define_command!("Get the microphone input source (`MI`).

# Command format

> `MI;`

# Response format

> `MI{n};`

Where *n* is the microphone input source; see [`MicInputSource`]." =>
    GetMicInputSource
);

define_command!("Set the microphone input source (`MI`) (K4 only).

# Command format

> `MI{n};`

Where *n* is the microphone input source; see [`MicInputSource`]." =>
    SetMicInputSource {
        input: MicInputSource
    }
);

define_command_enum!("Microphone input source." =>
    MicInputSource {
        "Front panel microphone jack." => Front = b'0',
        "Rear panel microphone jack." => Rear = b'1',
        "USB audio input." => Usb = b'2',
        "Bluetooth audio input." => Bluetooth = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAudioMixRatio, SetAudioMixRatio
// ------------------------------------------------------------------------------------------------

define_command!("Get the main/sub receiver audio mix ratio (`MX`).

# Command format

> `MX;`

# Response format

> `MX{nn};`

Where *nn* is the mix ratio, between `00` (all main) and `99` (all sub)." =>
    GetAudioMixRatio
);

define_command!("Set the main/sub receiver audio mix ratio (`MX`).

# Command format

> `MX{nn};`

Where *nn* is the mix ratio, between `00` (all main) and `99` (all sub)." =>
    SetAudioMixRatio {
        ratio: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoAAutoNotchState, GetVfoBAutoNotchState, SetVfoAAutoNotchState,
//      SetVfoBAutoNotchState
// ------------------------------------------------------------------------------------------------

define_command!("Get the auto notch state for VFO A (`NA`).

# Command format

> `NA;`

# Response format

> `NA{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetVfoAAutoNotchState
);

define_command!("Get the auto notch state for VFO B (`NA$`).

# Command format

> `NA$;`

# Response format

> `NA${n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetVfoBAutoNotchState
);

define_command!("Set the auto notch state for VFO A (`NA`).

# Command format

> `NA{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetVfoAAutoNotchState { state }
);

define_command!("Set the auto notch state for VFO B (`NA$`).

# Command format

> `NA${n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetVfoBAutoNotchState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetManualNotchA, GetManualNotchB, SetManualNotchA, SetManualNotchB, ManualNotch
// ------------------------------------------------------------------------------------------------

define_command!("Get the manual notch frequency and state for VFO A (`NM`).

# Command format

> `NM;`

# Response format

> `NM{n}{s}{±nnnn};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* `s` is a step-size digit, present in the response but not currently interpreted.
* *±nnnn* is the notch offset, in Hz, from the passband center." =>
    GetVfoAManualNotchSettings
);

define_command!("Get the manual notch frequency and state for VFO B (`NM$`).

# Command format

> `NM$;`

# Response format

> `NM${n}{s}{±nnnn};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* `s` is a step-size digit, present in the response but not currently interpreted.
* *±nnnn* is the notch offset, in Hz, from the passband center." =>
    GetVfoBManualNotchSettings
);

define_command!("Set the manual notch frequency and state for VFO A (`NM`).

# Command format

> `NM{n}0{±nnnn};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* *s* the step-size digit is always sent as `0`.
* *±nnnn* is the notch offset, in Hz, from the passband center, between `-9999` and `9999`." =>
    SetVfoAManualNotchSettings {
        state: bool,
        offset_hz: i16
    }
);

define_command!("Set the manual notch frequency and state for VFO B (`NM$`).

# Command format

> `NM${n}0{±nnnn};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* *s* the step-size digit is always sent as `0`.
* *±nnnn* is the notch offset, in Hz, from the passband center, between `-9999` and `9999`." =>
    SetVfoBManualNotchSettings {
        state: bool,
        offset_hz: i16
    }
);

/// The parsed manual notch state and frequency offset returned by [`GetVfoAManualNotchSettings`]
/// and [`GetVfoBManualNotchSettings`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManualNotch {
    /// `true` if the manual notch is active.
    pub state: bool,
    /// Notch frequency offset in Hz relative to the passband center.
    pub offset_hz: i16,
}

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoANoiseReductionSettings, GetVfoBNoiseReductionSettings,
//      SetVfoANoiseReductionSettings, SetVfoBNoiseReductionSettings, NoiseReduction
// ------------------------------------------------------------------------------------------------

define_command!("Get the noise reduction (LMS) state and level for VFO A (`NR`).

# Command format

> `NR;`

# Response format

> `NR{n}{l};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* *l* is the noise reduction level, between `0` and `9`." =>
    GetVfoANoiseReductionSettings
);

define_command!("Get the noise reduction (LMS) state and level for VFO B (`NR$`).

# Command format

> `NR$;`

# Response format

> `NR${n}{l};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* *l* is the noise reduction level, between `0` and `9`." =>
    GetVfoBNoiseReductionSettings
);

define_command!("Set the noise reduction (LMS) state and level for VFO A (`NR`).

# Command format

> `NR{n}{l};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* *l* is the noise reduction level, between `0` and `9`." =>
    SetVfoANoiseReductionSettings {
        state: bool,
        level: u8
    }
);

define_command!("Set the noise reduction (LMS) state and level for VFO B (`NR$`).

# Command format

> `NR${n}{l};`

Where:

* `n` is the boolean state `0` (off) or `1` (on).
* *l* is the noise reduction level, between `0` and `9`." =>
    SetVfoBNoiseReductionSettings {
        state: bool,
        level: u8
    }
);

define_command_struct!(
    "The parsed noise reduction state and level returned by [`GetVfoANoiseReductionSettings`] and [`GetVfoBNoiseReductionSettings`]." =>
    NoiseReduction {
        state: bool,
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: PlayDvrMessage
// ------------------------------------------------------------------------------------------------

define_command!("Play a DVR (digital voice recorder) message (`PB`).

# Command format

> `PB{n};`

Where *n* is the message number, between `1` and `8`; `0` stops playback." =>
    PlayDvrMessage {
        message: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoACtssTone, GetVfoBCtssTone, SetVfoACtssTone, SetVfoBCtssTone
// ------------------------------------------------------------------------------------------------

define_command!("Get the CTCSS (PL) tone code for VFO A (`PL`).

FM mode only.

# Command format

> `PL;`

# Response format

> `PL{nnn};`

Where *nnn* is the tone code, between `000` (no tone) and `038`." =>
    GetVfoACtssTone
);

define_command!("Get the CTCSS (PL) tone code for VFO B (`PL$`).

FM mode only.

# Command format

> `PL$;`

# Response format

> `PL${nnn};`

Where *nnn* is the tone code, between `000` (no tone) and `038`." =>
    GetVfoBCtssTone
);

define_command!("Set the CTCSS (PL) tone code for VFO A (`PL`).

FM mode only.

# Command format

> `PL{nnn};`

Where *nnn* is the tone code, between `000` (no tone) and `038`.

**Note**: see the tone frequency table in the D12 reference for the mapping from code to frequency." =>
    SetVfoACtssTone {
        tone_code: u8
    }
);

define_command!("Set the CTCSS (PL) tone code for VFO B (`PL$`).

FM mode only.

# Command format

> `PL${nnn};`

Where *nnn* is the tone code, between `000` (no tone) and `038`.

**Note**: see the tone frequency table in the D12 reference for the mapping from code to frequency." =>
    SetVfoBCtssTone {
        tone_code: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCurrentBandPowerLimit
// ------------------------------------------------------------------------------------------------

define_command!("Get the per-band power limit setting for the current band (`PP`).

# Command format

> `PP;`

# Response format

> `PP{nnn};`

Where *nnn* is the stored power limit for the current band, in watts, between `000` and `110`." =>
    GetCurrentBandPowerLimit
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatus, SetPowerStatus, PowerStatus
// ------------------------------------------------------------------------------------------------

define_command!("Get the transceiver power state (`PS`).

K4 extended command; K41 mode required.

# Command format

> `PS;`

# Response format

> `PS{n};`

Where *n* is the power state; see [`PowerStatus`]." =>
    GetPowerStatus
);

define_command!("Set the transceiver power state (`PS`).

K4 extended command; K41 mode required.

# Command format

> `PS{n};`

Where *n* is the power state; see [`PowerStatus`]. 

**Note**: Setting [`FirmwareRestart`](PowerStatus::FirmwareRestart) triggers a controlled firmware
restart." =>
    SetPowerStatus {
        state: PowerStatus
    }
);

define_command_enum!("Transceiver power state." =>
    PowerStatus {
        "Power off." => PowerOff = b'0',
        "Power on." => PowerOn = b'1',
        "Trigger a controlled firmware restart." => FirmwareRestart = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetActiveSoftwareReleaseChannel, SetActiveSoftwareReleaseChannel,
//      SoftwareReleaseChannel
// ------------------------------------------------------------------------------------------------

define_command!("Get the active software release channel (`RL`).

# Command format

> `RL;`

# Response format

> `RL{n};`

Where *n* is the release channel; see [`SoftwareReleaseChannel`]." =>
    GetActiveSoftwareReleaseChannel
);

define_command!("Set the active software release channel (`RL`).

# Command format

> `RL{n};`

Where *n* is the release channel; see [`SoftwareReleaseChannel`]." =>
    SetActiveSoftwareReleaseChannel {
        channel: SoftwareReleaseChannel
    }
);

define_command_enum!("Software release channel." =>
    SoftwareReleaseChannel {
        "Stable release channel." => Stable = b'0',
        "Beta release channel." => Beta = b'1',
        "Alpha release channel." => Alpha = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRepeaterOffset, SetRepeaterOffset, RepeaterOffset
// ------------------------------------------------------------------------------------------------

define_command!("Get the repeater offset direction and frequency (`RP`).

FM mode only.

# Command format

> `RP;`

# Response format

> `RP{n}{s}{nnnnnn};`

Where:

* *n* is the offset direction, see [`RepeaterOffsetDirection`].
* `s` is a split flag, present in the response but not currently interpreted.
* *nnnnnn* is the offset, in Hz." =>
    GetRepeaterOffset
);

define_command!("Set the repeater offset direction and frequency (`RP`).

FM mode only.

# Command format

> `RP{n}0{nnnnnn};`

Where:

* *n* is the offset direction, see [`RepeaterOffsetDirection`].
* *s* the split flag is always sent as `0`.
* *nnnnnn* is the offset, in Hz, between `000000` and `999999`." =>
    SetRepeaterOffset {
        direction: RepeaterOffsetDirection,
        offset_hz: u32
    }
);

/// The parsed repeater offset direction and frequency returned by [`GetRepeaterOffset`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RepeaterOffset {
    /// Offset direction: `0` = off, `1` = positive, `2` = negative.
    pub direction: RepeaterOffsetDirection,
    /// Offset in Hz.
    pub offset_hz: u32,
}

define_command_enum!("Repeater offset direction for [`GetRepeaterOffset`] and [`SetRepeaterOffset`]." =>
    RepeaterOffsetDirection {
        "No offset." => Off = b'0',
        "Positive offset." => Positive = b'1',
        "Negative offset." => Negative = b'2'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetScreenCount
// ------------------------------------------------------------------------------------------------

define_command!("Get the number of display screens available (`SC`).

# Command format

> `SC;`

# Response format

> `SC{nn};`

Where *nn* is the number of VFO-screen combinations available, between `00` and `99`." =>
    GetScreenCount
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetK4Delay
// ------------------------------------------------------------------------------------------------

define_command!("Set the QSK or VOX delay, in milliseconds (`SD`).

K4 extended command; K41 mode required.

# Command format

> `SD{nnnn};`

Where *nnnn* is the delay, between `0000` (full QSK / instant VOX) and `2000` ms." =>
    SetK4QskOrVoxDelay {
        delay_ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetSystemAutoInfo
// ------------------------------------------------------------------------------------------------

define_command!("Configure the system auto-info interval (`SI`).

# Command format

> `SI{nnnn};`

Where *nnnn* is the interval, in ms, between unsolicited periodic status reports, between `0000`
(disabled) and `9999`." =>
    SetSystemAutoInfoInterval {
        interval_ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetStreamingLatency, SetStreamingLatency
// ------------------------------------------------------------------------------------------------

define_command!("Get the remote audio streaming latency setting (`SL`).

# Command format

> `SL;`

# Response format

> `SL{nn};`

Where *nn* is the latency class, between `00` (lowest latency) and `99`." =>
    GetStreamingLatencyClass
);

define_command!("Set the remote audio streaming latency setting (`SL`).

# Command format

> `SL{nn};`

Where *nn* is the latency class, between `00` (lowest latency) and `99`." =>
    SetStreamingLatencyClass {
        latency: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSerialNumber
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
// Public Types: CaptureScreenshot
// ------------------------------------------------------------------------------------------------

define_command!("Capture a screenshot to the SD card (`SS`).

# Command format

> `SS;`

Saves a PNG screenshot of the current display to the SD card; there is no argument or response
beyond the usual acknowledgement." =>
    CaptureScreenshot
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTxGainConstant
// ------------------------------------------------------------------------------------------------

define_command!("Get the transmit gain constant used for calibration (`TA`).

# Command format

> `TA;`

# Response format

> `TA{nnn};`

Where *nnn* is the internal DAC calibration value, between `000` and `255`." =>
    GetTransmitGainConstant
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoATextDecodeMode, GetVfoBTextDecodeMode, SetVfoATextDecodeMode,
//      SetVfoBTextDecodeMode, TextDecodeMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the text decode/encode mode for VFO A (`TD`).

# Command format

> `TD;`

# Response format

> `TD{n};`

Where *n* is the text decode/encode mode; see [`TextDecodeMode`]." =>
    GetVfoATextDecodeMode
);

define_command!("Get the text decode/encode mode for VFO B (`TD$`).

# Command format

> `TD$;`

# Response format

> `TD${n};`

Where *n* is text decode/encode mode; see [`TextDecodeMode`]." =>
    GetVfoBTextDecodeMode
);

define_command!("Set the text decode/encode mode for VFO A (`TD`).

# Command format

> `TD{n};`

Where *n* is text decode/encode mode; see [`TextDecodeMode`]." =>
    SetVfoATextDecodeMode {
        mode: TextDecodeMode
    }
);

define_command!("Set the text decode/encode mode for VFO B (`TD$`).

# Command format

> `TD${n};`

Where *n* is text decode/encode mode; see [`TextDecodeMode`]." =>
    SetVfoBTextDecodeMode {
        mode: TextDecodeMode
    }
);

define_command_enum!("Text decode/encode mode." =>
    TextDecodeMode {
        "Off." => Off = b'0',
        "CW decode." => Cw = b'1',
        "RTTY decode." => Rtty = b'2',
        "PSK decode." => Psk = b'3'
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransmitGain
// ------------------------------------------------------------------------------------------------

define_command!("Get the current transmit gain setting (`TG`).

# Command format

> `TG;`

# Response format

> `TG{nnn};`

Where *nnn* is the internal DAC value, between `000` and `255`." =>
    GetTransmitGain
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTransmitTestModeState, SetTransmitTestModeState
// ------------------------------------------------------------------------------------------------

define_command!("Get the TX test mode state (`TS`).

# Command format

> `TS;`

# Response format

> `TS{n};`

Where `n` is the boolean state `0` (normal TX) or `1` (transmit test mode; continuous carrier)." =>
    GetTransmitTestModeState
);

define_command!("Set the TX test mode state (`TS`).

Transmits a continuous carrier when enabled; intended for calibration and test use only.

# Command format

> `TS{n};`

Where `n` is the boolean state `0` (normal TX) or `1` (transmit test mode; continuous carrier)." =>
    SetTransmitTestModeState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: SetTune
// ------------------------------------------------------------------------------------------------

define_command!("Start or stop the ATU tuning sequence (K4 only, SET only).

# Command format

> `TU{n};`

Where `n` is the boolean state `0` (stop tuning) or `1` (start ATU tune)." =>
    SetAtuTuningState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetUtcTimestamp
// ------------------------------------------------------------------------------------------------

define_command!("Get the current UTC date and time from the K4 real-time clock (`UT`).

# Command format

> `UT;`

# Response format

> `UT{hh}{mm}{ss}{MM}{DD}{YYYY};`

Where *hh* is the hour, *mm* the minute, *ss* the second, *MM* the month, *DD* the day and *YYYY*
the year, returned as raw bytes." =>
    GetUtcTimestamp
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCoarseTuningStep, SetCoarseTuningStep
// ------------------------------------------------------------------------------------------------

define_command!("Get the coarse VFO tune step size (`VC`).

# Command format

> `VC;`

# Response format

> `VC{nn};`

Where *nn* is the coarse tuning step multiplier, between `00` and `99`." =>
    GetCoarseTuningStep
);

define_command!("Set the coarse VFO tune step size (`VC`).

# Command format

> `VC{nn};`

Where *nn* is the coarse tuning step multiplier, between `00` and `99`." =>
    SetCoarseTuningStep {
        step: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVoxGain, SetVoxGain
// ------------------------------------------------------------------------------------------------

define_command!("Get the VOX gain (`VG`).

# Command format

> `VG;`

# Response format

> `VG{nnn};`

Where *nnn* is the gain, between `000` (off / minimum sensitivity) and `009` (maximum)." =>
    GetVoxGain
);

define_command!("Set the VOX gain (`VG`).

# Command format

> `VG{nnn};`

Where *nnn* is the gain, between `000` (off / minimum sensitivity) and `009` (maximum)." =>
    SetVoxGain {
        gain: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVoxInhibit, SetVoxInhibit
// ------------------------------------------------------------------------------------------------

define_command!("Get the VOX inhibit state (`VI`).

# Command format

> `VI;`

# Response format

> `VI{n};`

Where `n` is the boolean state `0` (VOX enabled) or `1` (VOX inhibited / muted)." =>
    GetVoxInhibitState
);

define_command!("Set the VOX inhibit state (`VI`).

# Command format

> `VI{n};`

Where `n` is the boolean state `0` (VOX enabled) or `1` (VOX inhibited / muted)." =>
    SetVoxInhibitState { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoATransverterOffset, GetVfoBTransverterOffset
// ------------------------------------------------------------------------------------------------

define_command!("Get the VFO frequency offset used for transverter operation, for VFO A (`VO`).

# Command format

> `VO;`

# Response format

> `VO{nnnnnnnnnn};`

Where *nnnnnnnnnn* is the 10-digit Hz offset value, returned as raw bytes for the caller to
interpret; the value may be signed ASCII." =>
    GetVfoATransverterOffset
);

define_command!("Get the VFO frequency offset used for transverter operation, for VFO B (`VO$`).

# Command format

> `VO$;`

# Response format

> `VO${nnnnnnnnnn};`

Where *nnnnnnnnnn* is the 10-digit Hz offset value, returned as raw bytes for the caller to
interpret; the value may be signed ASCII." =>
    GetVfoBTransverterOffset
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoTuningStepA, GetVfoTuningStepB, SetVfoTuningStepA, SetVfoTuningStepB
// ------------------------------------------------------------------------------------------------

define_command!("Get the VFO tuning step size, in Hz, for VFO A (`VT`).

# Command format

> `VT;`

# Response format

> `VT{nnnnnn};`

Where *nnnnnn* is the step size in Hz, e.g. `000001` for 1 Hz or `001000` for 1 kHz." =>
    GetVfoATuningStep
);

define_command!("Get the VFO tuning step size, in Hz, for VFO B (`VT$`).

# Command format

> `VT$;`

# Response format

> `VT${nnnnnn};`

Where *nnnnnn* is the step size in Hz, e.g. `000001` for 1 Hz or `001000` for 1 kHz." =>
    GetVfoBTuningStep
);

define_command!("Set the VFO tuning step size, in Hz, for VFO A (`VT`).

# Command format

> `VT{nnnnnn};`

Where *nnnnnn* is the step size in Hz, between `000000` and `999999`." =>
    SetVfoATuningStep {
        step_hz: u32
    }
);

define_command!("Set the VFO tuning step size, in Hz, for VFO B (`VT$`).

# Command format

> `VT${nnnnnn};`

Where *nnnnnn* is the step size in Hz, between `000000` and `999999`." =>
    SetVfoBTuningStep {
        step_hz: u32
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetWattmeterCalibrationConstant, SetWattmeterCalibrationConstant
// ------------------------------------------------------------------------------------------------

define_command!("Get the wattmeter calibration value (`WM`).

# Command format

> `WM;`

# Response format

> `WM{nnn};`

Where *nnn* is the internal calibration constant, between `000` and `255`." =>
    GetWattmeterCalibrationConstant
);

define_command!("Set the wattmeter calibration value (`WM`).

# Command format

> `WM{nnn};`

Where *nnn* is the internal calibration constant, between `000` and `255`." =>
    SetWattmeterCalibrationConstant {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetVfoATransverterActiveBandSlot, GetVfoBTransverterActiveBandSlot,
//      SetVfoATransverterActiveBandSlot, SetVfoBTransverterActiveBandSlot
// ------------------------------------------------------------------------------------------------

define_command!("Get the active transverter band slot for VFO A (`XV`).

# Command format

> `XV;`

# Response format

> `XV{nn};`

Where *nn* is the transverter band slot, between `00` and `08`." =>
    GetVfoATransverterActiveBandSlot
);

define_command!("Get the active transverter band slot for VFO B (`XV$`).

# Command format

> `XV$;`

# Response format

> `XV${nn};`

Where *nn* is the transverter band slot, between `00` and `08`." =>
    GetVfoBTransverterActiveBandSlot
);

define_command!("Set the active transverter band slot for VFO A (`XV`).

# Command format

> `XV{nn};`

Where *nn* is the transverter band slot, between `00` and `08`." =>
    SetVfoATransverterActiveBandSlot {
        band_slot: u8
    }
);

define_command!("Set the active transverter band slot for VFO B (`XV$`).

# Command format

> `XV${nn};`

Where *nn* is the transverter band slot, between `00` and `08`." =>
    SetVfoBTransverterActiveBandSlot {
        band_slot: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_command!(CopyVfoAtoVfoB => b"AB" with Some |_cmd: &CopyVfoAtoVfoB| {
    vec![b'0']
});

// ------------------------------------------------------------------------------------------------

impl_command!(SwapVfoAandVfoB => b"AB" with Some |_cmd: &SwapVfoAandVfoB| {
    vec![b'1']
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetAtuMode => b"AT");
impl_command_with_response!(GetAtuMode => try_from enum AtuMode);

impl_command!(SetAtuMode => b"AT" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetBandIndependenceState => b"BI");
impl_command_with_response!(GetBandIndependenceState => boolean);

impl_command!(SetBandIndependenceState => b"BI" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(SetCwSidetonePitch => b"CW"
    format pitch_hz uint 3, if |cmd: &SetCwSidetonePitch| {
    if (300..=800).contains(&cmd.pitch_hz) {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "hz",
            type_name: "u16",
            value: cmd.pitch_hz.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetDigitalAudioRoutingMode => b"DA");
impl_command_with_response!(GetDigitalAudioRoutingMode => try_from enum DigitalAudioRoutingMode);

impl_command!(SetDigitalAudioRoutingMode => b"DA" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetDigitalOutputPin1State => b"DO");
impl_command_with_response!(GetDigitalOutputPin1State => boolean);

impl_command!(SetDigitalOutputPin1State => b"DO" for boolean high);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransmitDataBandwidth => b"DW");
impl_command_with_response!(GetTransmitDataBandwidth => 4, u16_from_ascii => u16);

impl_command!(SetTransmitDataBandwidth => b"DW" with Some |cmd: &SetTransmitDataBandwidth| {
    format_u16_ascii_4(cmd.bandwidth_10hz)
}, if |cmd: &SetTransmitDataBandwidth| {
    if cmd.bandwidth_10hz <= 9999 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "bandwidth_10hz",
            type_name: "u16",
            value: cmd.bandwidth_10hz.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(SetCommandEchoState => b"EC" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetErrorReportingState => b"ER");
impl_command_with_response!(GetErrorReportingState => boolean);

impl_command!(SetErrorReportingState => b"ER" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(CenterPanadapterOnVfoA => b"FC");

// ------------------------------------------------------------------------------------------------

impl_command!(CenterPanadapterOnVfoB => b"FC$");

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoAFilterPresetSlot => b"FP");
impl_command_with_response!(GetVfoAFilterPresetSlot => 1, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBFilterPresetSlot => b"FP$");
impl_command_with_response!(GetVfoBFilterPresetSlot => 1, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoAFilterPresetSlot => b"FP"
    format preset uint 1,
    if |cmd: &SetVfoAFilterPresetSlot| {
        if (1..=8).contains(&cmd.preset) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "preset",
                type_name: "u8",
                value: cmd.preset.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoBFilterPresetSlot => b"FP$"
    format preset uint 1,
    if |cmd: &SetVfoBFilterPresetSlot| {
        if (1..=8).contains(&cmd.preset) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "preset",
                type_name: "u8",
                value: cmd.preset.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoAAgcMode => b"GT");
impl_command_with_response!(GetVfoAAgcMode => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBAgcMode => b"GT$");
impl_command_with_response!(GetVfoBAgcMode => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoAAgcMode => b"GT"
    format mode uint 2,
    if |cmd: &SetVfoAAgcMode| {
        if cmd.mode <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "mode",
                type_name: "u8",
                value: cmd.mode.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoBAgcMode => b"GT$"
    format mode uint 2,
    if |cmd: &SetVfoBAgcMode| {
        if cmd.mode <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "mode",
                type_name: "u8",
                value: cmd.mode.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverId => b"ID");
impl_command_with_response!(GetTransceiverId => 3, u16_from_ascii => u16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoAIfCenterPitch => b"IS");
impl_command_with_response!(GetVfoAIfCenterPitch => 5, parse_signed_hz_4 => i16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBIfCenterPitch => b"IS$");
impl_command_with_response!(GetVfoBIfCenterPitch => 5, parse_signed_hz_4 => i16);

// ------------------------------------------------------------------------------------------------

impl_command!(GetKeyerPaddleEmulationMode => b"KP");
impl_command_with_response!(GetKeyerPaddleEmulationMode => try_from enum KeyerPaddleEmulationMode);

impl_command!(SetKeyerPaddleEmulationMode => b"KP" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetKeyerSpeed => b"KS"
    format wpm uint 3,
    if |cmd: &SetKeyerSpeed| {
        if (8..=100).contains(&cmd.wpm) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "wpm",
                type_name: "u8",
                value: cmd.wpm.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAudioLineInputLevel => b"LI");
impl_command_with_response!(GetAudioLineInputLevel => 3, u8_from_ascii => u8);

impl_command!(
    SetAudioLineInputLevel => b"LI"
    format level uint 3,
    if |cmd: &SetAudioLineInputLevel| {
        if cmd.level <= 60 {
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

impl_command!(GetAudioLineOutputLevel => b"LO");
impl_command_with_response!(GetAudioLineOutputLevel => 3, u8_from_ascii => u8);

impl_command!(
    SetAudioLineOutputLevel => b"LO"
    format level uint 3,
    if |cmd: &SetAudioLineOutputLevel| {
        if cmd.level <= 60 {
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

impl_command!(GetVfoAModeAlternates => b"MA");

impl CommandWithResponse for GetVfoAModeAlternates {
    type Response = Vec<u8>;

    fn expected_response_length(&self) -> usize {
        0
    }

    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, self.command_id(), 0)?;
        bytes_to_vec(d)
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBModeAlternates => b"MA$");

impl CommandWithResponse for GetVfoBModeAlternates {
    type Response = Vec<u8>;

    fn expected_response_length(&self) -> usize {
        0
    }

    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, self.command_id(), 0)?;
        bytes_to_vec(d)
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetMicInputSource => b"MI");
impl_command_with_response!(GetMicInputSource => try_from enum MicInputSource);

impl_command!(SetMicInputSource => b"MI" for as byte input);

// ------------------------------------------------------------------------------------------------

impl_command!(GetAudioMixRatio => b"MX");
impl_command_with_response!(GetAudioMixRatio => 2, u8_from_ascii => u8);

impl_command!(
    SetAudioMixRatio => b"MX"
    format ratio uint 2,
    if |cmd: &SetAudioMixRatio| {
        if cmd.ratio <= 99 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "ratio",
                type_name: "u8",
                value: cmd.ratio.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoAAutoNotchState => b"NA");
impl_command_with_response!(GetVfoAAutoNotchState => boolean);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBAutoNotchState => b"NA$");
impl_command_with_response!(GetVfoBAutoNotchState => boolean);

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoAAutoNotchState => b"NA" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoBAutoNotchState => b"NA$" for state);

// ------------------------------------------------------------------------------------------------
// NM/NM$ pack the on/off flag, an ignored step digit, and a sign-prefixed 4-digit Hz offset into a
// single argument, so `impl_command!`'s field shorthands cannot express the encoding; the
// implementations below are hand-rolled.
// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoAManualNotchSettings => b"NM");

impl CommandWithResponse for GetVfoAManualNotchSettings {
    type Response = ManualNotch;

    fn expected_response_length(&self) -> usize {
        7
    }

    fn parse(&self, bytes: &[u8]) -> Result<ManualNotch, RigError> {
        let d = validate_response(bytes, self.command_id(), self.expected_response_length())?;
        Ok(ManualNotch {
            state: d[0] == b'1',
            offset_hz: parse_signed_hz_4(&d[2..7])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBManualNotchSettings => b"NM$");

impl CommandWithResponse for GetVfoBManualNotchSettings {
    type Response = ManualNotch;

    fn expected_response_length(&self) -> usize {
        7
    }

    fn parse(&self, bytes: &[u8]) -> Result<ManualNotch, RigError> {
        let d = validate_response(bytes, self.command_id(), self.expected_response_length())?;
        Ok(ManualNotch {
            state: d[0] == b'1',
            offset_hz: parse_signed_hz_4(&d[2..7])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoAManualNotchSettings => b"NM" with |cmd: &SetVfoAManualNotchSettings| {
    Ok(Some(manual_k4_notch_argument_bytes(cmd.state, cmd.offset_hz)))
}, if |cmd: &SetVfoAManualNotchSettings| { validate_k4_manual_notch_offset(cmd.offset_hz) });

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoBManualNotchSettings => b"NM$" with |cmd: &SetVfoBManualNotchSettings| {
    Ok(Some(manual_k4_notch_argument_bytes(cmd.state, cmd.offset_hz)))
}, if |cmd: &SetVfoBManualNotchSettings| { validate_k4_manual_notch_offset(cmd.offset_hz) });

// ------------------------------------------------------------------------------------------------
// NR/NR$ pack the on/off flag and the level into a single two-byte argument, so
// `impl_command!`'s field shorthands cannot express the encoding; the implementations below are
// hand-rolled.
// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoANoiseReductionSettings => b"NR");

impl CommandWithResponse for GetVfoANoiseReductionSettings {
    type Response = NoiseReduction;

    fn expected_response_length(&self) -> usize {
        2
    }

    fn parse(&self, bytes: &[u8]) -> Result<NoiseReduction, RigError> {
        let d = validate_response(bytes, self.command_id(), self.expected_response_length())?;
        Ok(NoiseReduction {
            state: d[0] == b'1',
            level: d[1] - b'0',
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBNoiseReductionSettings => b"NR$");

impl CommandWithResponse for GetVfoBNoiseReductionSettings {
    type Response = NoiseReduction;

    fn expected_response_length(&self) -> usize {
        2
    }

    fn parse(&self, bytes: &[u8]) -> Result<NoiseReduction, RigError> {
        let d = validate_response(bytes, self.command_id(), self.expected_response_length())?;
        Ok(NoiseReduction {
            state: d[0] == b'1',
            level: d[1] - b'0',
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoANoiseReductionSettings => b"NR" with Some |cmd: &SetVfoANoiseReductionSettings| {
    vec![if cmd.state { b'1' } else { b'0' }, b'0' + cmd.level]
}, if |cmd: &SetVfoANoiseReductionSettings| { validate_k4_noise_reduction_level(cmd.level) });

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoBNoiseReductionSettings => b"NR$" with Some |cmd: &SetVfoBNoiseReductionSettings| {
    vec![if cmd.state { b'1' } else { b'0' }, b'0' + cmd.level]
}, if |cmd: &SetVfoBNoiseReductionSettings| { validate_k4_noise_reduction_level(cmd.level) });

// ------------------------------------------------------------------------------------------------

impl_command!(
    PlayDvrMessage => b"PB"
    format message uint 1,
    if |cmd: &PlayDvrMessage| {
        if cmd.message <= 8 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "message",
                type_name: "u8",
                value: cmd.message.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoACtssTone => b"PL");
impl_command_with_response!(GetVfoACtssTone => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBCtssTone => b"PL$");
impl_command_with_response!(GetVfoBCtssTone => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoACtssTone => b"PL"
    format tone_code uint 3,
    if |cmd: &SetVfoACtssTone| { validate_k4_pl_tone_code(cmd.tone_code) }
);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoBCtssTone => b"PL$"
    format tone_code uint 3,
    if |cmd: &SetVfoBCtssTone| { validate_k4_pl_tone_code(cmd.tone_code) }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetCurrentBandPowerLimit => b"PP");
impl_command_with_response!(GetCurrentBandPowerLimit => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerStatus => b"PS");
impl_command_with_response!(GetPowerStatus => try_from enum PowerStatus);

impl_command!(SetPowerStatus => b"PS" for as byte state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetActiveSoftwareReleaseChannel => b"RL");
impl_command_with_response!(GetActiveSoftwareReleaseChannel => try_from enum SoftwareReleaseChannel);

impl_command!(SetActiveSoftwareReleaseChannel => b"RL" for as byte channel);

// ------------------------------------------------------------------------------------------------
// RP packs the offset direction, an ignored split flag, and a 6-digit Hz offset into a single
// argument, so `impl_command!`'s field shorthands cannot express the encoding; the implementations
// below are hand-rolled.
// ------------------------------------------------------------------------------------------------

impl_command!(GetRepeaterOffset => b"RP");

impl CommandWithResponse for GetRepeaterOffset {
    type Response = RepeaterOffset;

    fn expected_response_length(&self) -> usize {
        8
    }

    fn parse(&self, bytes: &[u8]) -> Result<RepeaterOffset, RigError> {
        let d = validate_response(bytes, self.command_id(), self.expected_response_length())?;
        Ok(RepeaterOffset {
            direction: RepeaterOffsetDirection::from_repr(d[0]).ok_or(
                RigError::InvalidArgumentValue {
                    argument_name: "direction",
                    type_name: "RepeaterOffsetDirection",
                    value: d[0].to_string(),
                },
            )?,
            offset_hz: u32_from_ascii(&d[2..8])?,
        })
    }
}

impl_command!(SetRepeaterOffset => b"RP" with Some |cmd: &SetRepeaterOffset| {
    let mut v = vec![cmd.direction as u8];
    v.extend_from_slice(&format_u32_ascii_6(cmd.offset_hz));
    v
}, if |cmd: &SetRepeaterOffset| {
    if cmd.offset_hz > 999_999 {
        Err(RigError::InvalidArgumentValue {
            argument_name: "offset_hz",
            type_name: "u32",
            value: cmd.offset_hz.to_string(),
        })
    } else {
        Ok(())
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetScreenCount => b"SC");
impl_command_with_response!(GetScreenCount => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(SetK4QskOrVoxDelay => b"SD" with Some |cmd: &SetK4QskOrVoxDelay| {
    format_u16_ascii_4(cmd.delay_ms)
}, if |cmd: &SetK4QskOrVoxDelay| {
    if cmd.delay_ms <= 2000 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "ms",
            type_name: "u16",
            value: cmd.delay_ms.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(SetSystemAutoInfoInterval => b"SI" with Some |cmd: &SetSystemAutoInfoInterval| {
    format_u16_ascii_4(cmd.interval_ms)
}, if |cmd: &SetSystemAutoInfoInterval| {
    if cmd.interval_ms <= 9999 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "interval_ms",
            type_name: "u16",
            value: cmd.interval_ms.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_command!(GetStreamingLatencyClass => b"SL");
impl_command_with_response!(GetStreamingLatencyClass => 2, u8_from_ascii => u8);

impl_command!(
    SetStreamingLatencyClass => b"SL"
    format latency uint 2,
    if |cmd: &SetStreamingLatencyClass| {
        if cmd.latency <= 99 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "latency",
                type_name: "u8",
                value: cmd.latency.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverSerialNumber => b"SN");
impl_command_with_response!(GetTransceiverSerialNumber => 5, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_command!(CaptureScreenshot => b"SS");

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransmitGainConstant => b"TA");
impl_command_with_response!(GetTransmitGainConstant => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoATextDecodeMode => b"TD");
impl_command_with_response!(GetVfoATextDecodeMode => try_from enum TextDecodeMode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBTextDecodeMode => b"TD$");
impl_command_with_response!(GetVfoBTextDecodeMode => try_from enum TextDecodeMode);

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoATextDecodeMode => b"TD" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoBTextDecodeMode => b"TD$" for as byte mode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransmitGain => b"TG");
impl_command_with_response!(GetTransmitGain => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransmitTestModeState => b"TS");
impl_command_with_response!(GetTransmitTestModeState => boolean);

impl_command!(SetTransmitTestModeState => b"TS" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(SetAtuTuningState => b"TU" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetUtcTimestamp => b"UT");
impl_command_with_response!(GetUtcTimestamp => 14, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(GetCoarseTuningStep => b"VC");
impl_command_with_response!(GetCoarseTuningStep => 2, u8_from_ascii => u8);

impl_command!(
    SetCoarseTuningStep => b"VC"
    format step uint 2,
    if |cmd: &SetCoarseTuningStep| {
        if cmd.step <= 99 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "step",
                type_name: "u8",
                value: cmd.step.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVoxGain => b"VG");
impl_command_with_response!(GetVoxGain => 3, u8_from_ascii => u8);

impl_command!(
    SetVoxGain => b"VG"
    format gain uint 3,
    if |cmd: &SetVoxGain| {
        if cmd.gain <= 9 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "gain",
                type_name: "u8",
                value: cmd.gain.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVoxInhibitState => b"VI");
impl_command_with_response!(GetVoxInhibitState => boolean);

impl_command!(SetVoxInhibitState => b"VI" for state);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoATransverterOffset => b"VO");
impl_command_with_response!(GetVfoATransverterOffset => 10, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBTransverterOffset => b"VO$");
impl_command_with_response!(GetVfoBTransverterOffset => 10, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoATuningStep => b"VT");
impl_command_with_response!(GetVfoATuningStep => 6, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBTuningStep => b"VT$");
impl_command_with_response!(GetVfoBTuningStep => 6, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoATuningStep => b"VT" with Some |cmd: &SetVfoATuningStep| {
    format_u32_ascii_6(cmd.step_hz)
}, if |cmd: &SetVfoATuningStep| { validate_k4_vfo_tuning_step(cmd.step_hz) });

// ------------------------------------------------------------------------------------------------

impl_command!(SetVfoBTuningStep => b"VT$" with Some |cmd: &SetVfoBTuningStep| {
    format_u32_ascii_6(cmd.step_hz)
}, if |cmd: &SetVfoBTuningStep| { validate_k4_vfo_tuning_step(cmd.step_hz) });

// ------------------------------------------------------------------------------------------------

impl_command!(GetWattmeterCalibrationConstant => b"WM");
impl_command_with_response!(GetWattmeterCalibrationConstant => 3, u8_from_ascii => u8);

impl_command!(SetWattmeterCalibrationConstant => b"WM" format value uint 3);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoATransverterActiveBandSlot => b"XV");
impl_command_with_response!(GetVfoATransverterActiveBandSlot => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBTransverterActiveBandSlot => b"XV$");
impl_command_with_response!(GetVfoBTransverterActiveBandSlot => 2, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoATransverterActiveBandSlot => b"XV"
    format band_slot uint 2,
    if |cmd: &SetVfoATransverterActiveBandSlot| { validate_k4_transverter_band_slot(cmd.band_slot) }
);

// ------------------------------------------------------------------------------------------------

impl_command!(
    SetVfoBTransverterActiveBandSlot => b"XV$"
    format band_slot uint 2,
    if |cmd: &SetVfoBTransverterActiveBandSlot| { validate_k4_transverter_band_slot(cmd.band_slot) }
);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

/// Parse a sign byte followed by four ASCII digits into a signed Hz offset, as used by the
/// `IS`/`IS$` and `NM`/`NM$` commands.
fn parse_signed_hz_4(bytes: &[u8]) -> Result<i16, RigError> {
    let sign = sign_from_ascii_strict::<i32>(bytes[0])?;
    let magnitude = i16::try_from(u16_from_ascii(&bytes[1..5])?).map_err(|_| {
        RigError::InvalidResponseData {
            data: bytes.to_vec(),
        }
    })?;
    Ok((sign as i16) * magnitude)
}

/// Format a `u16` as 4 zero-padded ASCII digits, as used by the `DW`, `SD` and `SI` commands.
fn format_u16_ascii_4(n: u16) -> Vec<u8> {
    format!("{n:04}").into_bytes()
}

/// Format a `u32` as 6 zero-padded ASCII digits, as used by the `RP` and `VT`/`VT$` commands.
fn format_u32_ascii_6(n: u32) -> Vec<u8> {
    format!("{n:06}").into_bytes()
}

/// Build the argument bytes for `NM`/`NM$`: the on/off flag, a fixed `0` step digit, and the
/// sign-prefixed 4-digit Hz offset.
fn manual_k4_notch_argument_bytes(state: bool, offset_hz: i16) -> Vec<u8> {
    let mut v = vec![if state { b'1' } else { b'0' }, b'0'];
    v.extend_from_slice(&format_int_ascii(offset_hz, 4));
    v
}

/// Validate the notch offset used by `SetManualNotchA`/`SetManualNotchB`; the wire format has room
/// for only 4 decimal digits of magnitude.
fn validate_k4_manual_notch_offset(offset_hz: i16) -> Result<(), RigError> {
    if (-9999..=9999).contains(&offset_hz) {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "offset_hz",
            type_name: "i16",
            value: offset_hz.to_string(),
        })
    }
}

/// Validate the noise reduction level used by `SetNoiseReductionA`/`SetNoiseReductionB`.
fn validate_k4_noise_reduction_level(level: u8) -> Result<(), RigError> {
    if level <= 9 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "level",
            type_name: "u8",
            value: level.to_string(),
        })
    }
}

/// Validate the CTCSS tone code used by `SetPlToneA`/`SetPlToneB`.
fn validate_k4_pl_tone_code(tone_code: u8) -> Result<(), RigError> {
    if tone_code <= 38 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "tone_code",
            type_name: "u8",
            value: tone_code.to_string(),
        })
    }
}

/// Validate the text decode/encode mode used by `SetTextDecodeModeA`/`SetTextDecodeModeB`.
fn validate_k4_text_decode_mode(mode: u8) -> Result<(), RigError> {
    if mode <= 3 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "mode",
            type_name: "u8",
            value: mode.to_string(),
        })
    }
}

/// Validate the tuning step used by `SetVfoTuningStepA`/`SetVfoTuningStepB`; the wire format has
/// room for only 6 decimal digits.
fn validate_k4_vfo_tuning_step(step_hz: u32) -> Result<(), RigError> {
    if step_hz <= 999_999 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "step_hz",
            type_name: "u32",
            value: step_hz.to_string(),
        })
    }
}

/// Validate the transverter band slot used by `SetTransverterBandA`/`SetTransverterBandB`.
fn validate_k4_transverter_band_slot(band_slot: u8) -> Result<(), RigError> {
    if band_slot <= 8 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "band_slot",
            type_name: "u8",
            value: band_slot.to_string(),
        })
    }
}
