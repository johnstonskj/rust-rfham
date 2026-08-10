//!
//! CAT commands for the Elecraft KH1 portable transceiver.
//!
//! Commands follow the **B2** programmer's reference
//! (Elecraft KH1 Programmer's Reference, rev. B2, Jan 2026).
//!
//! The KH1 is a minimalist QRP CW/DATA transceiver. Its command set is much
//! smaller than the K3/K4, and most commands are SET-only. Notable differences:
//!
//! - **FA** uses 10 Hz resolution in an 8-digit field (not the 11-digit Hz used by
//!   K3/K4). `FA00014074;` = 14.074 MHz (last digit = tens of Hz).
//! - Most commands have no GET form; the radio pushes state changes via AI mode.
//! - Some commands mirror K3/KX equivalents in format but with restricted ranges.
//!

use super::meta::parse_decimal_u8;
use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// AG — AF Gain (SET only; separate headphones and speaker values)
// ------------------------------------------------------------------------------------------------

///
/// Set headphone AF gain (KH1, SET only).
///
/// RSP format: `AG0nnn;` — `nnn` = 000–060.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetHeadphoneGain {
    pub level: u8,
}
///
/// Set speaker AF gain (KH1, SET only).
///
/// RSP format: `AG1nnn;` — `nnn` = 000–060.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpeakerGain {
    pub level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetHeadphoneGain {
    fn command_id(&self) -> &[u8] {
        b"AG"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let mut v = vec![b'0'];
        v.extend_from_slice(format!("{:03}", self.level.min(60)).as_bytes());
        Some(v)
    }
}

impl Command for SetSpeakerGain {
    fn command_id(&self) -> &[u8] {
        b"AG"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let mut v = vec![b'1'];
        v.extend_from_slice(format!("{:03}", self.level.min(60)).as_bytes());
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// DS — Display Text (GET/SET; line 1 or 2, 16 chars)
// ------------------------------------------------------------------------------------------------

///
/// Query a display line on the KH1.
///
/// RSP format: `DSn ssssssssssssssss;` — `n` = 1 or 2 (line number),
/// followed by up to 16 ASCII characters (space-padded).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDisplayText {
    pub line: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetDisplayText {
    fn command_id(&self) -> &[u8] {
        b"DS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.line.clamp(1, 2)])
    }
}

///
/// Set a display line on the KH1 (up to 16 characters).
///
/// RSP format: `DSn ssssssssssssssss;` — `n` = 1 or 2 (line number),
/// text is space-padded to exactly 16 characters.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetDisplayText {
    /// Line 1 or 2.
    pub line: u8,
    /// Text content (truncated and padded to 16 characters).
    pub text: Vec<u8>,
}

// ------------------------------------------------------------------------------------------------

impl CommandWithResponse for GetDisplayText {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        17
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"DS", 17)?;
        Ok(d[1..].to_vec())
    }
}

impl Command for SetDisplayText {
    fn command_id(&self) -> &[u8] {
        b"DS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let mut v = vec![b'0' + self.line.clamp(1, 2), b' '];
        let len = self.text.len().min(16);
        v.extend_from_slice(&self.text[..len]);
        while v.len() < 18 {
            v.push(b' ');
        }
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// EN — Enable/Disable Front Panel (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Enable or disable the front panel controls (KH1, SET only).
///
/// RSP format: `ENn;` — `n` = `0` (panel disabled / remote-only) or `1` (panel enabled).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFrontPanel {
    pub enabled: bool,
}

impl Command for SetFrontPanel {
    fn command_id(&self) -> &[u8] {
        b"EN"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// FA — VFO A Frequency (KH1 format: 8 digits, 10 Hz units)
// ------------------------------------------------------------------------------------------------

///
/// Set VFO A frequency on the KH1 (SET only).
///
/// RSP format: `FAnnnnnnnn;` — `nnnnnnnn` = 8-digit frequency in **tens of Hz**.
/// For example, `14074000` × 10 Hz = 140,740,000 Hz = 14.074 MHz.
///
/// **Note:** This differs from the K3/K4 `FA` command which uses 11-digit 1 Hz resolution.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOperatingFrequency {
    pub freq_10hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetOperatingFrequency {
    fn command_id(&self) -> &[u8] {
        b"FA"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.freq_10hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// FO — Filter Offset (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Set the filter offset (KH1, SET only).
///
/// RSP format: `FOnnn;` — `nnn` = 000–999 (offset in Hz; adjusts filter passband center).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFilterOffset {
    pub offset_hz: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetFilterOffset {
    fn command_id(&self) -> &[u8] {
        b"FO"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.offset_hz.min(999)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// H — Battery/Power Status (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query battery and power status (KH1, GET only).
///
/// RSP format: `Hnnns;` — `nnn` = battery level (0–100 percent), `s` = source
/// indicator (`B`=battery, `U`=USB/external).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerStatus;

///
/// Battery / power status returned by `GetPowerStatus`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerStatus {
    /// Battery level in percent (0–100).
    pub battery_pct: u8,
    /// `true` = powered from USB/external; `false` = battery.
    pub external_power: bool,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetPowerStatus, b"H");

impl CommandWithResponse for GetPowerStatus {
    type Response = PowerStatus;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<PowerStatus, RigError> {
        let d = validate_response(bytes, b"H", 4)?;
        Ok(PowerStatus {
            battery_pct: parse_decimal_u8(&d[0..3])?,
            external_power: d[3] == b'U',
        })
    }
}

// ------------------------------------------------------------------------------------------------
// HK — Emulate Hardware Key Press (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Emulate a hardware key press on the KH1 (SET only).
///
/// RSP format: `HKnn;` — `nn` = key code (01–12; see B2 reference for key assignments).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmulateKeyPress {
    pub key_code: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for EmulateKeyPress {
    fn command_id(&self) -> &[u8] {
        b"HK"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.key_code.clamp(1, 12)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// I — Transceiver Information (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query comprehensive transceiver status (KH1, GET only).
///
/// RSP format: `Innnnnnnnmmmsbbwwwtt;` — packed status fields covering
/// frequency (8 digits, 10 Hz), mode, S-meter, band, filter BW, and TX state.
/// Returns raw bytes for caller to decode against B2 reference.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransceiverInfo;

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverInfo, b"I");

impl CommandWithResponse for GetTransceiverInfo {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        20
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"I", 20)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// LD — LED Brightness (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Set LED brightness (KH1, SET only).
///
/// RSP format: `LDn;` — `n` = 0 (off), 1 (dim), 2 (medium), 3 (bright).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetLedBrightness {
    pub level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetLedBrightness {
    fn command_id(&self) -> &[u8] {
        b"LD"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.level.min(3)])
    }
}

// ------------------------------------------------------------------------------------------------
// LG — Play CW Message (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Send a stored CW message from the internal message store (KH1, SET only).
///
/// RSP format: `LGn;` — `n` = message number 1–5; `n` = 0 stops current message.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SendCwMessage {
    pub message: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for SendCwMessage {
    fn command_id(&self) -> &[u8] {
        b"LG"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.message.min(5)])
    }
}

// ------------------------------------------------------------------------------------------------
// MD — Operating Mode (SET only; restricted set)
// ------------------------------------------------------------------------------------------------

///
/// Set operating mode on the KH1 (SET only).
///
/// RSP format: `MDn;` — `n` = 0 (LSB), 1 (USB), 2 (CW), 4 (DATA).
/// **Note:** The KH1 supports only modes 0, 1, 2, and 4.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOperatingMode {
    pub mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetOperatingMode {
    fn command_id(&self) -> &[u8] {
        b"MD"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode])
    }
}

// ------------------------------------------------------------------------------------------------
// MN — Select Menu Item (SET only; 3-char ID)
// ------------------------------------------------------------------------------------------------

///
/// Navigate to a menu item by 3-character ID (KH1, SET only).
///
/// RSP format: `MNabc;` — `abc` = 3-character ASCII menu item identifier
/// (e.g., `MNK S;` for keyer speed). See B2 reference for the full menu ID list.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectMenuItem {
    /// 3-character menu item ID (ASCII, space-padded if shorter).
    pub item_id: [u8; 3],
}

// ------------------------------------------------------------------------------------------------

impl Command for SelectMenuItem {
    fn command_id(&self) -> &[u8] {
        b"MN"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(self.item_id.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// MP — Menu Parameter (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current menu parameter value (KH1).
///
/// RSP format: `MPnnn;` — `nnn` = 000–255 (value for the currently selected menu item).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMenuParameter;

///
/// Set the current menu parameter value (KH1).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMenuParameter {
    pub value: u8,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetMenuParameter, b"MP");

impl CommandWithResponse for GetMenuParameter {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MP", 3)?;
        parse_decimal_u8(d)
    }
}

impl Command for SetMenuParameter {
    fn command_id(&self) -> &[u8] {
        b"MP"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.value).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// RV — Firmware Revision (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query firmware revision string (KH1, GET only).
///
/// RSP format: `RVvvvv;` — `vvvv` = firmware version string (e.g., `1.08`).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFirmwareRevision;

// ------------------------------------------------------------------------------------------------

impl_command!(GetFirmwareRevision, b"RV");

impl CommandWithResponse for GetFirmwareRevision {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"RV", 4)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// SN — Serial Number (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the transceiver serial number (KH1, GET only).
///
/// RSP format: `SNnnnnn;` — `nnnnn` = 5-digit serial number.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSerialNumber;

// ------------------------------------------------------------------------------------------------

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
// ST — Status Summary (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query combined KH1 status (GET only).
///
/// RSP format: `STnsa;` — `n` = status byte, `s` = S-meter (0–9), `a` = antenna (0–2).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetStatus;

///
/// KH1 status summary returned by `GetStatus`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status {
    /// Raw status byte (bit-field; see B2 reference for flag layout).
    pub status: u8,
    /// S-meter reading (0–9).
    pub s_meter: u8,
    /// Active antenna (0=whip, 1=BNC, 2=AXE).
    pub antenna: u8,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetStatus, b"ST");

impl CommandWithResponse for GetStatus {
    type Response = Status;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<Status, RigError> {
        let d = validate_response(bytes, b"ST", 3)?;
        Ok(Status {
            status: d[0],
            s_meter: d[1] - b'0',
            antenna: d[2] - b'0',
        })
    }
}

// ------------------------------------------------------------------------------------------------
// SW — Button Press/Hold (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Emulate a button tap on the KH1 (SET only).
///
/// RSP format: `SWnT;` — `n` = button number 1–6; `T` = tap.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmulateButtonTap {
    pub button: u8,
}

///
/// Emulate a button hold on the KH1 (SET only).
///
/// RSP format: `SWnH;` — `n` = button number 1–6; `H` = hold.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmulateButtonHold {
    pub button: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for EmulateButtonTap {
    fn command_id(&self) -> &[u8] {
        b"SW"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.button.clamp(1, 6), b'T'])
    }
}

impl Command for EmulateButtonHold {
    fn command_id(&self) -> &[u8] {
        b"SW"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.button.clamp(1, 6), b'H'])
    }
}

// ------------------------------------------------------------------------------------------------
// TXL / TXH — TX Frequency Limits (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the lower TX frequency limit for the current band (KH1, GET only).
///
/// RSP format: `TXLnnnnnn;` — `nnnnnn` = lower limit in kHz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTxLowLimit;

///
/// Query the upper TX frequency limit for the current band (KH1, GET only).
///
/// RSP format: `TXHnnnnnn;` — `nnnnnn` = upper limit in kHz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTxHighLimit;

// ------------------------------------------------------------------------------------------------

impl_command!(GetTxLowLimit, b"TXL");

impl CommandWithResponse for GetTxLowLimit {
    type Response = u32;
    fn expected_response_length(&self) -> usize {
        6
    }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"TXL", 6)?;
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

impl_command!(GetTxHighLimit, b"TXH");

impl CommandWithResponse for GetTxHighLimit {
    type Response = u32;
    fn expected_response_length(&self) -> usize {
        6
    }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"TXH", 6)?;
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
