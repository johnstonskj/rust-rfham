//!
//! CAT commands specific to or extended on the Elecraft K4 transceiver.
//!
//! Commands follow the **D12** programmer's reference
//! (K4 Programmer's Reference, rev. D12, May 2026).
//!
//! Many K3/KX commands from [`super::k3_kx`] also work on the K4. This module
//! covers commands that are:
//! - Unique to the K4 (not present on K3/KX), or
//! - Significantly different in format or range on the K4.
//!
//! The K4 meta-command mode (K41) is set via [`super::meta::SetK4CommandMode`].
//! Many commands below require K41 mode to be active.
//!

#![allow(unused_doc_comments)]

use super::meta::{parse_decimal_u8, parse_decimal_u16, parse_signed_i16};
use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, Vfo, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// AB — VFO A↔B Copy / Swap (K4 only, SET only)
// ------------------------------------------------------------------------------------------------

/// Copy VFO A frequency to VFO B (K4 only, SET only).
///
/// RSP format: `AB0;` — sets VFO B = VFO A (no response beyond ?/E).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyVfoAToB;

impl Command for CopyVfoAToB {
    fn command_id(&self) -> &[u8] {
        b"AB"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0'])
    }
}

/// Swap VFO A and VFO B frequencies (K4 only, SET only).
///
/// RSP format: `AB1;` — exchanges VFO A and VFO B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwapVfoAB;

impl Command for SwapVfoAB {
    fn command_id(&self) -> &[u8] {
        b"AB"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'1'])
    }
}

// ------------------------------------------------------------------------------------------------
// AT — ATU Mode (K4 extended)
// ------------------------------------------------------------------------------------------------

/// Query ATU mode.
///
/// RSP format: `ATn;` — `n` = 0 (ATU bypass), 1 (ATU in-line / auto), 2 (ATU tuning).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuMode;

impl_command!(GetAtuMode, b"AT");

impl CommandWithResponse for GetAtuMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"AT", 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set ATU mode (0=bypass, 1=auto, 2=start tuning).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAtuMode {
    mode: u8,
}

impl Command for SetAtuMode {
    fn command_id(&self) -> &[u8] {
        b"AT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode.min(2)])
    }
}

// ------------------------------------------------------------------------------------------------
// BI — Band Independence (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query band independence state (K4 only).
///
/// RSP format: `BIn;` — `n` = `0` (off; VFO A and B share band) or
/// `1` (on; each VFO can be on a different band independently).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandIndependence;

impl_command!(GetBandIndependence, b"BI");

impl CommandWithResponse for GetBandIndependence {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"BI", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set band independence on/off (K4 only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandIndependence {
    on: bool,
}

impl Command for SetBandIndependence {
    fn command_id(&self) -> &[u8] {
        b"BI"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// CW — CW Sidetone Pitch (GET/SET on K4; GET only on K3/KX)
// ------------------------------------------------------------------------------------------------

/// Set CW sidetone pitch in Hz (K4 supports both GET and SET; K3/KX is GET only).
///
/// RSP format: `CWnnn;` — `nnn` = 300–800 Hz.
/// On K4 this also controls the received CW pitch offset for zero-beat tuning.
command! { SetCwSidetonePitch => hz: u16 }

impl Command for SetCwSidetonePitch {
    fn command_id(&self) -> &[u8] {
        b"CW"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.hz.clamp(300, 800)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// DA — Digital Audio Control (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query digital audio routing (K4 only).
///
/// RSP format: `DAn;` — `n` = 0 (analog audio routing), 1 (digital audio to USB),
/// 2 (digital audio from USB), 3 (full digital audio I/O via USB).
command!(GetDigitalAudio);

impl_command!(GetDigitalAudio, b"DA");

impl CommandWithResponse for GetDigitalAudio {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"DA", 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set digital audio routing (K4 only).
command! { SetDigitalAudio => mode: u8 }

impl Command for SetDigitalAudio {
    fn command_id(&self) -> &[u8] {
        b"DA"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode.min(3)])
    }
}

// ------------------------------------------------------------------------------------------------
// DO — DIGOUT1 State (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query DIGOUT1 (digital output pin 1) state (K4 only).
///
/// RSP format: `DOn;` — `n` = `0` (low) or `1` (high).
command!(GetDigOut1);

impl_command!(GetDigOut1, b"DO");

impl CommandWithResponse for GetDigOut1 {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"DO", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set DIGOUT1 state (K4 only).
command! { SetDigOut1 => high: bool }

impl Command for SetDigOut1 {
    fn command_id(&self) -> &[u8] {
        b"DO"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.high { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// DW — TX DATA Bandwidth (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query TX DATA (AFSK/FSK/PSK) bandwidth in Hz (K4 only).
///
/// RSP format: `DWnnnn;` — `nnnn` = 0000–9999 Hz in 10 Hz units.
command!(GetTxDataBandwidth);

impl_command!(GetTxDataBandwidth, b"DW");

impl CommandWithResponse for GetTxDataBandwidth {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"DW", 4)?;
        parse_decimal_u16(d)
    }
}

/// Set TX DATA bandwidth in units of 10 Hz (K4 only).
command! { SetTxDataBandwidth => bandwidth_10hz: u16 }

impl Command for SetTxDataBandwidth {
    fn command_id(&self) -> &[u8] {
        b"DW"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.bandwidth_10hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// EC — Echo Command to RS-232 (K4 only, SET only)
// ------------------------------------------------------------------------------------------------

/// Enable or disable command echo to RS-232 (K4 only, SET only).
///
/// RSP format: `ECn;` — `n` = `0` (no echo) or `1` (echo commands back to sender).
command! { SetCommandEcho => on: bool }

impl Command for SetCommandEcho {
    fn command_id(&self) -> &[u8] {
        b"EC"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ER — Error Reporting (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query error reporting state (K4 only).
///
/// RSP format: `ERn;` — `n` = `0` (disabled) or `1` (enabled; error RSPs are sent
/// unsolicited to the serial port when command errors occur).
command!(GetErrorReporting);

impl_command!(GetErrorReporting, b"ER");

impl CommandWithResponse for GetErrorReporting {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"ER", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set error reporting on/off (K4 only).
command! { SetErrorReporting => on: bool }

impl Command for SetErrorReporting {
    fn command_id(&self) -> &[u8] {
        b"ER"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// FC$ — Center Panadapter on VFO (K4 only, SET only)
// ------------------------------------------------------------------------------------------------

/// Center the panadapter on the current VFO frequency (K4 only, SET only).
///
/// RSP format: `FC;` or `FC$;` (no arguments). VFO A uses `FC`, VFO B uses `FC$`.
command! { CenterPanadapter => vfo: Vfo }

impl Command for CenterPanadapter {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FC",
            Vfo::B => b"FC$",
            _ => panic!("CenterPanadapter: only VFO A and B supported"),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// FP$ — Filter Preset (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query active filter preset slot (K4 only).
///
/// RSP format: `FPn;` or `FP$n;` — `n` = 1–8 (preset slot number).
command! { GetFilterPreset => vfo: Vfo }

impl Command for GetFilterPreset {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FP",
            Vfo::B => b"FP$",
            _ => panic!("GetFilterPreset: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetFilterPreset {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set filter preset slot (1–8).
command! { SetFilterPreset => vfo: Vfo, preset: u8 }

impl Command for SetFilterPreset {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FP",
            Vfo::B => b"FP$",
            _ => panic!("SetFilterPreset: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.preset.clamp(1, 8)])
    }
}

// ------------------------------------------------------------------------------------------------
// GT$ — AGC Mode (K4 extended, K41 mode)
// ------------------------------------------------------------------------------------------------

/// Query AGC mode via K4-extended command (K41 mode required).
///
/// RSP format: `GTnn;` or `GT$nn;` — `nn` = 00 (off), 01 (fast), 02 (slow), 03 (auto).
command! { GetK4AgcMode => vfo: Vfo }

impl Command for GetK4AgcMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"GT",
            Vfo::B => b"GT$",
            _ => panic!("GetK4AgcMode: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetK4AgcMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        parse_decimal_u8(d)
    }
}

/// Set AGC mode (K4 extended; 0=off, 1=fast, 2=slow, 3=auto).
command! { SetK4AgcMode => vfo: Vfo, mode: u8 }

impl Command for SetK4AgcMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"GT",
            Vfo::B => b"GT$",
            _ => panic!("SetK4AgcMode: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.mode.min(3)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ID — Radio Identification (K4 extended)
// ------------------------------------------------------------------------------------------------

/// Query radio identification string (K4 extended).
///
/// RSP format: `IDnnn;` — `nnn` is a numeric model ID (K4 returns `018`; K3 returns `017`).
command!(GetRadioId);

impl_command!(GetRadioId, b"ID");

impl CommandWithResponse for GetRadioId {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"ID", 3)?;
        parse_decimal_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// IS$ — IF Center Pitch (K4 extended, K41 mode)
// ------------------------------------------------------------------------------------------------

/// Query IF center pitch in Hz (K4 extended, K41 mode required).
///
/// RSP format: `IS±nnnn;` or `IS$±nnnn;` — sign-prefixed 4-digit Hz value per VFO.
command! { GetK4IfCenterPitch => vfo: Vfo }

impl Command for GetK4IfCenterPitch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"IS",
            Vfo::B => b"IS$",
            _ => panic!("GetK4IfCenterPitch: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetK4IfCenterPitch {
    type Response = i16;
    fn expected_response_length(&self) -> usize {
        5
    }
    fn parse(&self, bytes: &[u8]) -> Result<i16, RigError> {
        let d = validate_response(bytes, self.command_id(), 5)?;
        parse_signed_i16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// KP — Keyer Paddle (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query keyer paddle emulation mode (K4 only).
///
/// RSP format: `KPn;` — `n` = 0 (normal), 1 (dit only), 2 (dah only).
command!(GetKeyerPaddle);

impl_command!(GetKeyerPaddle, b"KP");

impl CommandWithResponse for GetKeyerPaddle {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"KP", 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set keyer paddle mode (0=normal, 1=dit, 2=dah).
command! { SetKeyerPaddle => mode: u8 }

impl Command for SetKeyerPaddle {
    fn command_id(&self) -> &[u8] {
        b"KP"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode.min(2)])
    }
}

// ------------------------------------------------------------------------------------------------
// KS — Keyer Speed (K4 extended range: 8–100 WPM)
// ------------------------------------------------------------------------------------------------

/// Set keyer speed in WPM (K4 extended range: 8–100; K3/KX limit is 50).
///
/// RSP format: `KSnnn;` — `nnn` = 008–100. Use [`super::k3_kx::GetKeyerSpeed`] to query.
command! { SetK4KeyerSpeed => wpm: u8 }

impl Command for SetK4KeyerSpeed {
    fn command_id(&self) -> &[u8] {
        b"KS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.wpm.clamp(8, 100)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// LI — Line Input (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query line audio input level (K4 only).
///
/// RSP format: `LInnn;` — `nnn` = 000–060.
command!(GetLineInput);

impl_command!(GetLineInput, b"LI");

impl CommandWithResponse for GetLineInput {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"LI", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set line audio input level (0–60).
command! { SetLineInput => level: u8 }

impl Command for SetLineInput {
    fn command_id(&self) -> &[u8] {
        b"LI"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.level.min(60)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// LO — Line Output (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query line audio output level (K4 only).
///
/// RSP format: `LOnnn;` — `nnn` = 000–060.
command!(GetLineOutput);

impl_command!(GetLineOutput, b"LO");

impl CommandWithResponse for GetLineOutput {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"LO", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set line audio output level (0–60).
command! { SetLineOutput => level: u8 }

impl Command for SetLineOutput {
    fn command_id(&self) -> &[u8] {
        b"LO"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.level.min(60)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MA$ — Mode Alternates (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Query the list of mode alternates for the current band (K4 only, GET only).
///
/// RSP format: `MAmm...;` or `MA$mm...;` — a variable-length string of mode character
/// codes indicating which modes are available on the current band/VFO.
command! { GetModeAlternates => vfo: Vfo }

impl Command for GetModeAlternates {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"MA",
            Vfo::B => b"MA$",
            _ => panic!("GetModeAlternates: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetModeAlternates {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        8
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, self.command_id(), 8)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// MI — Mic Input Select (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query mic input selection (K4 only).
///
/// RSP format: `MIn;` — `n` = 0 (front mic), 1 (rear mic), 2 (USB audio), 3 (Bluetooth).
command!(GetMicInput);

impl_command!(GetMicInput, b"MI");

impl CommandWithResponse for GetMicInput {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MI", 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set mic input (0=front, 1=rear, 2=USB, 3=Bluetooth).
command! { SetMicInput => input: u8 }

impl Command for SetMicInput {
    fn command_id(&self) -> &[u8] {
        b"MI"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.input.min(3)])
    }
}

// ------------------------------------------------------------------------------------------------
// MX — Main/Sub Audio Mix (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query main/sub audio mix ratio (K4 only).
///
/// RSP format: `MXnn;` — `nn` = 00–99 (0=all main, 99=all sub).
command!(GetAudioMix);

impl_command!(GetAudioMix, b"MX");

impl CommandWithResponse for GetAudioMix {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MX", 2)?;
        parse_decimal_u8(d)
    }
}

/// Set main/sub audio mix ratio (0=all main, 99=all sub).
command! { SetAudioMix => ratio: u8 }

impl Command for SetAudioMix {
    fn command_id(&self) -> &[u8] {
        b"MX"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.ratio.min(99)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// NA$ — Auto Notch (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query auto notch state (K4 only).
///
/// RSP format: `NAn;` or `NA$n;` — `n` = `0` (off) or `1` (on) per VFO.
command! { GetAutoNotch => vfo: Vfo }

impl Command for GetAutoNotch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NA",
            Vfo::B => b"NA$",
            _ => panic!("GetAutoNotch: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetAutoNotch {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set auto notch on/off.
command! { SetAutoNotch => vfo: Vfo, on: bool }

impl Command for SetAutoNotch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NA",
            Vfo::B => b"NA$",
            _ => panic!("SetAutoNotch: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// NM$ — Manual Notch (K4 only)
// ------------------------------------------------------------------------------------------------

/// Manual notch state returned by `GetManualNotch`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManualNotch {
    /// `true` = notch active.
    pub on: bool,
    /// Notch frequency offset in Hz relative to passband center.
    pub offset_hz: i16,
}

/// Query manual notch frequency and state (K4 only).
///
/// RSP format: `NMns±nnnn;` or `NM$ns±nnnn;` — `n`=on/off, `s`=step (ignored),
/// `±nnnn`=notch Hz offset from passband center.
command! { GetManualNotch => vfo: Vfo }

impl Command for GetManualNotch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NM",
            Vfo::B => b"NM$",
            _ => panic!("GetManualNotch: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetManualNotch {
    type Response = ManualNotch;
    fn expected_response_length(&self) -> usize {
        7
    }
    fn parse(&self, bytes: &[u8]) -> Result<ManualNotch, RigError> {
        let d = validate_response(bytes, self.command_id(), 7)?;
        Ok(ManualNotch {
            on: d[0] == b'1',
            offset_hz: parse_signed_i16(&d[2..7])?,
        })
    }
}

/// Set manual notch (on/off and frequency offset in Hz).
command! { SetManualNotch => vfo: Vfo, on: bool, offset_hz: i16 }

impl Command for SetManualNotch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NM",
            Vfo::B => b"NM$",
            _ => panic!("SetManualNotch: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let flag = if self.on { b'1' } else { b'0' };
        let sign = if self.offset_hz < 0 { b'-' } else { b'+' };
        let mag = self.offset_hz.unsigned_abs();
        let mut v = vec![flag, b'0', sign];
        v.extend_from_slice(format!("{:04}", mag).as_bytes());
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// NR$ — Noise Reduction (K4 only)
// ------------------------------------------------------------------------------------------------

/// Noise reduction state returned by `GetNoiseReduction`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoiseReduction {
    /// `true` = noise reduction active.
    pub on: bool,
    /// Level 0–9.
    pub level: u8,
}

/// Query noise reduction (LMS) state and level (K4 only).
///
/// RSP format: `NRnl;` or `NR$nl;` — `n` = on/off, `l` = level (0–9).
command! { GetNoiseReduction => vfo: Vfo }

impl Command for GetNoiseReduction {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NR",
            Vfo::B => b"NR$",
            _ => panic!("GetNoiseReduction: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetNoiseReduction {
    type Response = NoiseReduction;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<NoiseReduction, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        Ok(NoiseReduction {
            on: d[0] == b'1',
            level: d[1] - b'0',
        })
    }
}

/// Set noise reduction on/off and level (0–9).
command! { SetNoiseReduction => vfo: Vfo, on: bool, level: u8 }

impl Command for SetNoiseReduction {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NR",
            Vfo::B => b"NR$",
            _ => panic!("SetNoiseReduction: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![
            if self.on { b'1' } else { b'0' },
            b'0' + self.level.min(9),
        ])
    }
}

// ------------------------------------------------------------------------------------------------
// PB — DVR Message Playback (K4 only)
// ------------------------------------------------------------------------------------------------

/// Play a DVR (digital voice recorder) message (K4 only, SET only).
///
/// RSP format: `PBn;` — `n` = 1–8 (message number); `n` = 0 stops playback.
command! { PlayDvrMessage => message: u8 }

impl Command for PlayDvrMessage {
    fn command_id(&self) -> &[u8] {
        b"PB"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.message.min(8)])
    }
}

// ------------------------------------------------------------------------------------------------
// PL$ — PL / CTCSS Tone (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query CTCSS (PL) tone code (K4 only, FM mode).
///
/// RSP format: `PLnnn;` or `PL$nnn;` — `nnn` = tone code 000–038.
/// `000` = no tone. See Table in D12 reference for tone frequencies.
command! { GetPlTone => vfo: Vfo }

impl Command for GetPlTone {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"PL",
            Vfo::B => b"PL$",
            _ => panic!("GetPlTone: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetPlTone {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 3)?;
        parse_decimal_u8(d)
    }
}

/// Set PL/CTCSS tone code (0=off, 1–38=tone).
command! { SetPlTone => vfo: Vfo, tone_code: u8 }

impl Command for SetPlTone {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"PL",
            Vfo::B => b"PL$",
            _ => panic!("SetPlTone: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.tone_code.min(38)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// PP — Per-Band Power (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Query the per-band power limit setting (K4 only, GET only).
///
/// RSP format: `PPnnn;` — `nnn` = 000–110 watts (stored limit for the current band).
command!(GetPerBandPower);

impl_command!(GetPerBandPower, b"PP");

impl CommandWithResponse for GetPerBandPower {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"PP", 3)?;
        parse_decimal_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// PS — Power On/Off/Restart (K4 extended)
// ------------------------------------------------------------------------------------------------

/// Set transceiver power state (K4 extended).
///
/// RSP format: `PSn;` — `n` = 0 (power off), 1 (power on), 2 (firmware restart).
/// On K4, `PS2;` triggers a controlled firmware restart.
command! { SetK4PowerStatus => state: u8 }

impl Command for SetK4PowerStatus {
    fn command_id(&self) -> &[u8] {
        b"PS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.state.min(2)])
    }
}

// ------------------------------------------------------------------------------------------------
// RL — Software Release Selection (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query the active software release channel (K4 only).
///
/// RSP format: `RLn;` — `n` = 0 (stable), 1 (beta), 2 (alpha).
command!(GetSoftwareRelease);

impl_command!(GetSoftwareRelease, b"RL");

impl CommandWithResponse for GetSoftwareRelease {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"RL", 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set software release channel (0=stable, 1=beta, 2=alpha).
command! { SetSoftwareRelease => channel: u8 }

impl Command for SetSoftwareRelease {
    fn command_id(&self) -> &[u8] {
        b"RL"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.channel.min(2)])
    }
}

// ------------------------------------------------------------------------------------------------
// RP — Repeater Offset (K4 only)
// ------------------------------------------------------------------------------------------------

/// Repeater offset state returned by `GetRepeaterOffset`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RepeaterOffset {
    /// 0=off, 1=positive offset, 2=negative offset.
    pub direction: u8,
    /// Offset in Hz.
    pub offset_hz: u32,
}

/// Query repeater offset direction and frequency (K4 only, FM mode).
///
/// RSP format: `RPnsnnnnnn;` — `n` = direction (0=off, 1=+, 2=−),
/// `s` = split flag, `nnnnnn` = offset in Hz.
command!(GetRepeaterOffset);

impl_command!(GetRepeaterOffset, b"RP");

impl CommandWithResponse for GetRepeaterOffset {
    type Response = RepeaterOffset;
    fn expected_response_length(&self) -> usize {
        8
    }
    fn parse(&self, bytes: &[u8]) -> Result<RepeaterOffset, RigError> {
        let d = validate_response(bytes, b"RP", 8)?;
        let direction = d[0] - b'0';
        let mut n = 0u32;
        for &b in &d[2..8] {
            if !(b'0'..=b'9').contains(&b) {
                return Err(RigError::InvalidResponseData { data: d.to_vec() });
            }
            n = n * 10 + u32::from(b - b'0');
        }
        Ok(RepeaterOffset {
            direction,
            offset_hz: n,
        })
    }
}

/// Set repeater offset direction (0=off, 1=positive, 2=negative) and Hz.
command! { SetRepeaterOffset => direction: u8, offset_hz: u32 }

impl Command for SetRepeaterOffset {
    fn command_id(&self) -> &[u8] {
        b"RP"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let mut v = vec![b'0' + self.direction.min(2), b'0'];
        v.extend_from_slice(format!("{:06}", self.offset_hz).as_bytes());
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// SC — Screen Count (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Query the number of available display screens (K4 only, GET only).
///
/// RSP format: `SCnn;` — `nn` = 00–99 (number of VFO-screen combos available).
command!(GetScreenCount);

impl_command!(GetScreenCount, b"SC");

impl CommandWithResponse for GetScreenCount {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"SC", 2)?;
        parse_decimal_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// SD — QSK/VOX Delay (K4 extended range)
// ------------------------------------------------------------------------------------------------

/// Set QSK or VOX delay in milliseconds (K4 extended range: 0–2000 ms).
///
/// RSP format: `SDnnnn;` — `nnnn` = 0000–2000 ms (`0000` = full QSK / instant VOX).
command! { SetK4Delay => ms: u16 }

impl Command for SetK4Delay {
    fn command_id(&self) -> &[u8] {
        b"SD"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.ms.min(2000)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// SI — System Auto Info (K4 only, SET only)
// ------------------------------------------------------------------------------------------------

/// Configure system auto-info interval (K4 only, SET only).
///
/// RSP format: `SInnnn;` — `nnnn` = 0000–9999 ms interval between unsolicited
/// periodic status reports. `0000` = disable periodic reports.
command! { SetSystemAutoInfo => interval_ms: u16 }

impl Command for SetSystemAutoInfo {
    fn command_id(&self) -> &[u8] {
        b"SI"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.interval_ms).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// SL — Remote Streaming Audio Latency (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query remote audio streaming latency setting (K4 only).
///
/// RSP format: `SLnn;` — `nn` = 00–99 (latency class; 00 = lowest latency).
command!(GetStreamingLatency);

impl_command!(GetStreamingLatency, b"SL");

impl CommandWithResponse for GetStreamingLatency {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"SL", 2)?;
        parse_decimal_u8(d)
    }
}

/// Set remote audio streaming latency (0–99).
command! { SetStreamingLatency => latency: u8 }

impl Command for SetStreamingLatency {
    fn command_id(&self) -> &[u8] {
        b"SL"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.latency.min(99)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// SN — Serial Number (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Query the transceiver serial number (K4 only, GET only).
///
/// RSP format: `SNnnnnn;` — `nnnnn` = 5-digit serial number.
command!(GetSerialNumber);

impl_command!(GetSerialNumber, b"SN");

impl CommandWithResponse for GetSerialNumber {
    type Response = u32;
    fn expected_response_length(&self) -> usize {
        5
    }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"SN", 5)?;
        let mut n = 0u32;
        for &b in d {
            if !(b'0'..=b'9').contains(&b) {
                return Err(RigError::InvalidResponseData { data: d.to_vec() });
            }
            n = n * 10 + u32::from(b - b'0');
        }
        Ok(n)
    }
}

// ------------------------------------------------------------------------------------------------
// SS — Screenshot Capture (K4 only, SET only)
// ------------------------------------------------------------------------------------------------

/// Capture a screenshot to the SD card (K4 only, SET only).
///
/// RSP format: `SS;` (no arguments). Saves a PNG screenshot to the SD card.
command!(CaptureScreenshot);

impl_command!(CaptureScreenshot, b"SS");

// ------------------------------------------------------------------------------------------------
// TA — TX Gain Constant (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Query the TX gain constant used for calibration (K4 only, GET only).
///
/// RSP format: `TAnnn;` — `nnn` = 000–255 (internal DAC calibration value).
command!(GetTxGainConstant);

impl_command!(GetTxGainConstant, b"TA");

impl CommandWithResponse for GetTxGainConstant {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"TA", 3)?;
        parse_decimal_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// TD$ — Text Decode/Encode Mode (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query text decode/encode mode (K4 only).
///
/// RSP format: `TDn;` or `TD$n;` — `n` = 0 (off), 1 (CW decode), 2 (RTTY decode),
/// 3 (PSK decode).
command! { GetTextDecodeMode => vfo: Vfo }

impl Command for GetTextDecodeMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"TD",
            Vfo::B => b"TD$",
            _ => panic!("GetTextDecodeMode: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetTextDecodeMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set text decode/encode mode (0=off, 1=CW, 2=RTTY, 3=PSK).
command! { SetTextDecodeMode => vfo: Vfo, mode: u8 }

impl Command for SetTextDecodeMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"TD",
            Vfo::B => b"TD$",
            _ => panic!("SetTextDecodeMode: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode.min(3)])
    }
}

// ------------------------------------------------------------------------------------------------
// TG — TX Gain (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Read current TX gain setting (K4 only, GET only).
///
/// RSP format: `TGnnn;` — `nnn` = 000–255 (internal DAC value).
command!(GetTxGain);

impl_command!(GetTxGain, b"TG");

impl CommandWithResponse for GetTxGain {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"TG", 3)?;
        parse_decimal_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// TS — TX Test Mode (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query TX test mode state (K4 only).
///
/// RSP format: `TSn;` — `n` = `0` (normal TX) or `1` (TX test mode; continuous carrier).
command!(GetTxTestMode);

impl_command!(GetTxTestMode, b"TS");

impl CommandWithResponse for GetTxTestMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"TS", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set TX test mode on/off (K4 only; transmits a continuous carrier when enabled).
command! { SetTxTestMode => on: bool }

impl Command for SetTxTestMode {
    fn command_id(&self) -> &[u8] {
        b"TS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// TU — ATU Tune (K4 only, SET only)
// ------------------------------------------------------------------------------------------------

/// Start or stop the ATU tuning sequence (K4 only, SET only).
///
/// RSP format: `TUn;` — `n` = `0` (stop tuning) or `1` (start ATU tune).
command! { SetTune => start: bool }

impl Command for SetTune {
    fn command_id(&self) -> &[u8] {
        b"TU"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.start { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// UT — UTC Timestamp (K4 only, GET only)
// ------------------------------------------------------------------------------------------------

/// Query the current UTC time and date from the K4 real-time clock (GET only).
///
/// RSP format: `UThhmmssMMDDYYYY;` — `hh` hour, `mm` minute, `ss` second,
/// `MM` month, `DD` day, `YYYY` year.
command!(GetUtcTimestamp);

impl_command!(GetUtcTimestamp, b"UT");

impl CommandWithResponse for GetUtcTimestamp {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        14
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"UT", 14)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// VC — Coarse Tune Step (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query the coarse VFO tune step size (K4 only).
///
/// RSP format: `VCnn;` — `nn` = 00–99 (step size multiplier for coarse tuning).
command!(GetCoarseTuneStep);

impl_command!(GetCoarseTuneStep, b"VC");

impl CommandWithResponse for GetCoarseTuneStep {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"VC", 2)?;
        parse_decimal_u8(d)
    }
}

/// Set coarse tune step (0–99).
command! { SetCoarseTuneStep => step: u8 }

impl Command for SetCoarseTuneStep {
    fn command_id(&self) -> &[u8] {
        b"VC"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.step.min(99)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// VG — VOX Gain (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query VOX gain (K4 only).
///
/// RSP format: `VGnnn;` — `nnn` = 000–009 (0=off/minimum sensitivity, 9=maximum).
command!(GetVoxGain);

impl_command!(GetVoxGain, b"VG");

impl CommandWithResponse for GetVoxGain {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"VG", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set VOX gain (0–9).
command! { SetVoxGain => gain: u8 }

impl Command for SetVoxGain {
    fn command_id(&self) -> &[u8] {
        b"VG"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.gain.min(9)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// VI — VOX Inhibit (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query VOX inhibit state (K4 only).
///
/// RSP format: `VIn;` — `n` = `0` (VOX enabled) or `1` (VOX inhibited / muted).
command!(GetVoxInhibit);

impl_command!(GetVoxInhibit, b"VI");

impl CommandWithResponse for GetVoxInhibit {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"VI", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set VOX inhibit on/off.
command! { SetVoxInhibit => inhibit: bool }

impl Command for SetVoxInhibit {
    fn command_id(&self) -> &[u8] {
        b"VI"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.inhibit { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// VO$ — VFO Frequency Offset (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query the VFO frequency offset for transverter operation (K4 only).
///
/// RSP format: `VOnnnnnnnnnn;` or `VO$nnnnnnnnnn;` — 10-digit Hz offset value
/// (returned as raw bytes for caller to interpret; may be signed ASCII).
command! { GetVfoOffset => vfo: Vfo }

impl Command for GetVfoOffset {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"VO",
            Vfo::B => b"VO$",
            _ => panic!("GetVfoOffset: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetVfoOffset {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        10
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, self.command_id(), 10)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// VT$ — VFO Tuning Step (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query VFO tuning step size in Hz (K4 only).
///
/// RSP format: `VTnnnnnn;` or `VT$nnnnnn;` — `nnnnnn` = step in Hz
/// (e.g., `000001` = 1 Hz, `001000` = 1 kHz).
command! { GetVfoTuningStep => vfo: Vfo }

impl Command for GetVfoTuningStep {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"VT",
            Vfo::B => b"VT$",
            _ => panic!("GetVfoTuningStep: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetVfoTuningStep {
    type Response = u32;
    fn expected_response_length(&self) -> usize {
        6
    }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, self.command_id(), 6)?;
        let mut n = 0u32;
        for &b in d {
            if !(b'0'..=b'9').contains(&b) {
                return Err(RigError::InvalidResponseData { data: d.to_vec() });
            }
            n = n * 10 + u32::from(b - b'0');
        }
        Ok(n)
    }
}

/// Set VFO tuning step in Hz.
command! { SetVfoTuningStep => vfo: Vfo, step_hz: u32 }

impl Command for SetVfoTuningStep {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"VT",
            Vfo::B => b"VT$",
            _ => panic!("SetVfoTuningStep: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:06}", self.step_hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// WM — Wattmeter Calibration (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query wattmeter calibration value (K4 only).
///
/// RSP format: `WMnnn;` — `nnn` = 000–255 (internal calibration constant).
command!(GetWattmeterCalibration);

impl_command!(GetWattmeterCalibration, b"WM");

impl CommandWithResponse for GetWattmeterCalibration {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"WM", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set wattmeter calibration value (0–255).
command! { SetWattmeterCalibration => value: u8 }

impl Command for SetWattmeterCalibration {
    fn command_id(&self) -> &[u8] {
        b"WM"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.value).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// XV$ — Transverter Band Select (K4 only)
// ------------------------------------------------------------------------------------------------

/// Query the active transverter band slot (K4 only).
///
/// RSP format: `XVnn;` or `XV$nn;` — `nn` = 00–08 (transverter band slot number).
command! { GetTransverterBand => vfo: Vfo }

impl Command for GetTransverterBand {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"XV",
            Vfo::B => b"XV$",
            _ => panic!("GetTransverterBand: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetTransverterBand {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        parse_decimal_u8(d)
    }
}

/// Set transverter band slot (0–8).
command! { SetTransverterBand => vfo: Vfo, band_slot: u8 }

impl Command for SetTransverterBand {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"XV",
            Vfo::B => b"XV$",
            _ => panic!("SetTransverterBand: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.band_slot.min(8)).into_bytes())
    }
}
