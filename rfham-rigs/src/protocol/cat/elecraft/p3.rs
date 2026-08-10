//!
//! Serial commands for the Elecraft P3 panadapter.
//!
//! Commands follow the **A7** programmer's reference
//! (Elecraft P3 Programmer's Reference, rev. A7, Apr 2016).
//!
//! All P3 commands and responses use a leading `#` prefix, for example `#AVG3;`.
//! The special `=` command (product ID query) has no prefix and no semicolon terminator.
//! The GET form of a command is the bare command letters with no data: `#AVG;`.
//!

use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// = — Product Identification (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the P3 product identification string.
///
/// # Reference (P3 rev. A7, §=)
///
/// **GET** format: `=` (no semicolon, no `#` prefix).
/// **RSP** format: `ELECRAFT P3` (plain ASCII, no terminator).
///
/// This command does not follow the normal `#CMD;` convention.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetProductId;

// ------------------------------------------------------------------------------------------------

impl Command for GetProductId {
    fn command_id(&self) -> &[u8] { b"=" }
}
impl CommandWithResponse for GetProductId {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 11 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        Ok(bytes.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// #AVG — Display Averaging (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the display averaging level.
///
/// # Reference (P3 rev. A7, §#AVG)
///
/// **GET** format: `#AVG;`
/// **SET/RSP** format: `#AVGn;` — `n` = 0–9 (0 = off, higher = more averaging).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDisplayAveraging;

/// Set the display averaging level (0=off, 9=maximum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDisplayAveraging {
    pub level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetDisplayAveraging {
    fn command_id(&self) -> &[u8] { b"#AVG" }
}
impl CommandWithResponse for GetDisplayAveraging {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#AVG", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetDisplayAveraging {
    fn command_id(&self) -> &[u8] { b"#AVG" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.level.min(9)]) }
}

// ------------------------------------------------------------------------------------------------
// #BMP — Bitmap Capture (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Trigger a bitmap screenshot transfer from the P3 display.
///
/// # Reference (P3 rev. A7, §#BMP)
///
/// **SET** format: `#BMP;` — the P3 responds with raw bitmap data.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureBitmap;

// ------------------------------------------------------------------------------------------------

impl Command for CaptureBitmap {
    fn command_id(&self) -> &[u8] { b"#BMP" }
}

// ------------------------------------------------------------------------------------------------
// #BR — Baud Rate (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the P3 serial port baud rate.
///
/// # Reference (P3 rev. A7, §#BR)
///
/// **GET** format: `#BR;`
/// **SET/RSP** format: `#BRn;` — `n` = 0 (4800), 1 (9600), 2 (19200), 3 (38400).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBaudRate;

/// Set the serial port baud rate (0=4800, 1=9600, 2=19200, 3=38400).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBaudRate {
    pub rate: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBaudRate {
    fn command_id(&self) -> &[u8] { b"#BR" }
}
impl CommandWithResponse for GetBaudRate {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#BR", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetBaudRate {
    fn command_id(&self) -> &[u8] { b"#BR" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.rate.min(3)]) }
}

// ------------------------------------------------------------------------------------------------
// #CTF — Center Frequency Track (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the panadapter tracks the transceiver center frequency.
///
/// # Reference (P3 rev. A7, §#CTF)
///
/// **GET** format: `#CTF;`
/// **SET/RSP** format: `#CTFn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCenterFreqTrack;

/// Enable or disable center frequency tracking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetCenterFreqTrack {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetCenterFreqTrack {
    fn command_id(&self) -> &[u8] { b"#CTF" }
}
impl CommandWithResponse for GetCenterFreqTrack {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#CTF", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetCenterFreqTrack {
    fn command_id(&self) -> &[u8] { b"#CTF" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #DSM — Display Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the display mode.
///
/// # Reference (P3 rev. A7, §#DSM)
///
/// **GET** format: `#DSM;`
/// **SET/RSP** format: `#DSMn;` — `n` = 0 (spectrum only), 1 (spectrum + waterfall),
/// 2 (waterfall only), 3 (scope).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDisplayMode;

/// Set the display mode (0=spectrum, 1=spectrum+waterfall, 2=waterfall, 3=scope).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDisplayMode {
    pub mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetDisplayMode {
    fn command_id(&self) -> &[u8] { b"#DSM" }
}
impl CommandWithResponse for GetDisplayMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#DSM", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetDisplayMode {
    fn command_id(&self) -> &[u8] { b"#DSM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.mode.min(3)]) }
}

// ------------------------------------------------------------------------------------------------
// #FNL / #FNX — Frequency Span Limits (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the lower frequency span limit in Hz.
///
/// # Reference (P3 rev. A7, §#FNL)
///
/// **GET** format: `#FNL;`
/// **SET/RSP** format: `#FNLnnnnnnnn;` — 8-digit Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSpanLowerLimit;

/// Set the lower frequency span limit in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpanLowerLimit {
    pub hz: u32,
}

///
/// Query the upper frequency span limit in Hz.
///
/// # Reference (P3 rev. A7, §#FNX)
///
/// **GET** format: `#FNX;`
/// **SET/RSP** format: `#FNXnnnnnnnn;` — 8-digit Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSpanUpperLimit;

/// Set the upper frequency span limit in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpanUpperLimit {
    pub hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSpanLowerLimit {
    fn command_id(&self) -> &[u8] { b"#FNL" }
}
impl CommandWithResponse for GetSpanLowerLimit {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#FNL", 8)?;
        parse_u32(d)
    }
}
impl Command for SetSpanLowerLimit {
    fn command_id(&self) -> &[u8] { b"#FNL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

impl Command for GetSpanUpperLimit {
    fn command_id(&self) -> &[u8] { b"#FNX" }
}
impl CommandWithResponse for GetSpanUpperLimit {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#FNX", 8)?;
        parse_u32(d)
    }
}
impl Command for SetSpanUpperLimit {
    fn command_id(&self) -> &[u8] { b"#FNX" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// #FON — Font Size (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the display font size.
///
/// # Reference (P3 rev. A7, §#FON)
///
/// **GET** format: `#FON;`
/// **SET/RSP** format: `#FONn;` — `n` = 0 (small), 1 (large).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFontSize;

/// Set the display font size (false=small, true=large).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFontSize {
    pub large: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFontSize {
    fn command_id(&self) -> &[u8] { b"#FON" }
}
impl CommandWithResponse for GetFontSize {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#FON", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetFontSize {
    fn command_id(&self) -> &[u8] { b"#FON" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.large { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #FXA / #FXT — Fixed Span Frequencies (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fixed-span center frequency A in Hz.
///
/// # Reference (P3 rev. A7, §#FXA)
///
/// **GET** format: `#FXA;`
/// **SET/RSP** format: `#FXAnnnnnnnn;` — 8-digit Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFixedSpanFreqA;

/// Set fixed-span center frequency A in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFixedSpanFreqA {
    pub hz: u32,
}

///
/// Query the fixed-span center frequency B in Hz.
///
/// # Reference (P3 rev. A7, §#FXT)
///
/// **GET** format: `#FXT;`
/// **SET/RSP** format: `#FXTnnnnnnnn;` — 8-digit Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFixedSpanFreqB;

/// Set fixed-span center frequency B in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFixedSpanFreqB {
    pub hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFixedSpanFreqA {
    fn command_id(&self) -> &[u8] { b"#FXA" }
}
impl CommandWithResponse for GetFixedSpanFreqA {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#FXA", 8)?;
        parse_u32(d)
    }
}
impl Command for SetFixedSpanFreqA {
    fn command_id(&self) -> &[u8] { b"#FXA" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

impl Command for GetFixedSpanFreqB {
    fn command_id(&self) -> &[u8] { b"#FXT" }
}
impl CommandWithResponse for GetFixedSpanFreqB {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#FXT", 8)?;
        parse_u32(d)
    }
}
impl Command for SetFixedSpanFreqB {
    fn command_id(&self) -> &[u8] { b"#FXT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// #LBL — Band-Label Display (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether band labels are shown on the display.
///
/// # Reference (P3 rev. A7, §#LBL)
///
/// **GET** format: `#LBL;`
/// **SET/RSP** format: `#LBLn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandLabelDisplay;

/// Show or hide band labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandLabelDisplay {
    pub visible: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBandLabelDisplay {
    fn command_id(&self) -> &[u8] { b"#LBL" }
}
impl CommandWithResponse for GetBandLabelDisplay {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#LBL", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetBandLabelDisplay {
    fn command_id(&self) -> &[u8] { b"#LBL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.visible { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #MFA / #MFB — Marker Frequencies (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the marker A frequency in Hz.
///
/// # Reference (P3 rev. A7, §#MFA)
///
/// **GET** format: `#MFA;`
/// **SET/RSP** format: `#MFAnnnnnnnn;` — 8-digit Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMarkerFreqA;

/// Set marker A frequency in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMarkerFreqA {
    pub hz: u32,
}

///
/// Query the marker B frequency in Hz.
///
/// # Reference (P3 rev. A7, §#MFB)
///
/// **GET** format: `#MFB;`
/// **SET/RSP** format: `#MFBnnnnnnnn;` — 8-digit Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMarkerFreqB;

/// Set marker B frequency in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMarkerFreqB {
    pub hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetMarkerFreqA {
    fn command_id(&self) -> &[u8] { b"#MFA" }
}
impl CommandWithResponse for GetMarkerFreqA {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#MFA", 8)?;
        parse_u32(d)
    }
}
impl Command for SetMarkerFreqA {
    fn command_id(&self) -> &[u8] { b"#MFA" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

impl Command for GetMarkerFreqB {
    fn command_id(&self) -> &[u8] { b"#MFB" }
}
impl CommandWithResponse for GetMarkerFreqB {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#MFB", 8)?;
        parse_u32(d)
    }
}
impl Command for SetMarkerFreqB {
    fn command_id(&self) -> &[u8] { b"#MFB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// #MKA / #MKB — Marker Active (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether marker A is active.
///
/// # Reference (P3 rev. A7, §#MKA)
///
/// **GET** format: `#MKA;`
/// **SET/RSP** format: `#MKAn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMarkerAActive;

/// Enable or disable marker A.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMarkerAActive {
    pub active: bool,
}

///
/// Query whether marker B is active.
///
/// # Reference (P3 rev. A7, §#MKB)
///
/// **GET** format: `#MKB;`
/// **SET/RSP** format: `#MKBn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMarkerBActive;

/// Enable or disable marker B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMarkerBActive {
    pub active: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetMarkerAActive {
    fn command_id(&self) -> &[u8] { b"#MKA" }
}
impl CommandWithResponse for GetMarkerAActive {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#MKA", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetMarkerAActive {
    fn command_id(&self) -> &[u8] { b"#MKA" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.active { b'1' } else { b'0' }])
    }
}

impl Command for GetMarkerBActive {
    fn command_id(&self) -> &[u8] { b"#MKB" }
}
impl CommandWithResponse for GetMarkerBActive {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#MKB", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetMarkerBActive {
    fn command_id(&self) -> &[u8] { b"#MKB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.active { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #NB — Noise Blanker Level (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the display noise-blanker level.
///
/// # Reference (P3 rev. A7, §#NB)
///
/// **GET** format: `#NB;`
/// **SET/RSP** format: `#NBn;` — `n` = 0 (off), 1–9 (blanker strength).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetNoiseBlankerLevel;

/// Set the display noise-blanker level (0=off, 9=maximum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetNoiseBlankerLevel {
    pub level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetNoiseBlankerLevel {
    fn command_id(&self) -> &[u8] { b"#NB" }
}
impl CommandWithResponse for GetNoiseBlankerLevel {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#NB", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetNoiseBlankerLevel {
    fn command_id(&self) -> &[u8] { b"#NB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.level.min(9)]) }
}

// ------------------------------------------------------------------------------------------------
// #NBL — Noise-Blanker Low-Pass (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the noise-blanker low-pass filter state.
///
/// # Reference (P3 rev. A7, §#NBL)
///
/// **GET** format: `#NBL;`
/// **SET/RSP** format: `#NBLn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetNoiseBlankerLowPass;

/// Enable or disable the noise-blanker low-pass filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetNoiseBlankerLowPass {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetNoiseBlankerLowPass {
    fn command_id(&self) -> &[u8] { b"#NBL" }
}
impl CommandWithResponse for GetNoiseBlankerLowPass {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#NBL", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetNoiseBlankerLowPass {
    fn command_id(&self) -> &[u8] { b"#NBL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #PKM — Peak-Hold Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the spectrum peak-hold mode.
///
/// # Reference (P3 rev. A7, §#PKM)
///
/// **GET** format: `#PKM;`
/// **SET/RSP** format: `#PKMn;` — `n` = 0 (off), 1 (peak hold), 2 (peak + decay).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPeakHoldMode;

/// Set the peak-hold mode (0=off, 1=hold, 2=hold+decay).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPeakHoldMode {
    pub mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPeakHoldMode {
    fn command_id(&self) -> &[u8] { b"#PKM" }
}
impl CommandWithResponse for GetPeakHoldMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#PKM", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetPeakHoldMode {
    fn command_id(&self) -> &[u8] { b"#PKM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.mode.min(2)]) }
}

// ------------------------------------------------------------------------------------------------
// #PS — Power Save (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the display power-save mode.
///
/// # Reference (P3 rev. A7, §#PS)
///
/// **GET** format: `#PS;`
/// **SET/RSP** format: `#PSn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerSave;

/// Enable or disable display power-save.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPowerSave {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerSave {
    fn command_id(&self) -> &[u8] { b"#PS" }
}
impl CommandWithResponse for GetPowerSave {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#PS", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetPowerSave {
    fn command_id(&self) -> &[u8] { b"#PS" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #PT — Pass-Through Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the P3 is in serial pass-through mode.
///
/// # Reference (P3 rev. A7, §#PT)
///
/// **GET** format: `#PT;`
/// **SET/RSP** format: `#PTn;` — `n` = 0 (off), 1 (on, routes PC serial to transceiver).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPassThroughMode;

/// Enable or disable serial pass-through mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPassThroughMode {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPassThroughMode {
    fn command_id(&self) -> &[u8] { b"#PT" }
}
impl CommandWithResponse for GetPassThroughMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#PT", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetPassThroughMode {
    fn command_id(&self) -> &[u8] { b"#PT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #QSY — Tune Transceiver to Marker (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Command the transceiver to QSY to the active marker frequency.
///
/// # Reference (P3 rev. A7, §#QSY)
///
/// **SET** format: `#QSY;` — no response.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QsyToMarker;

// ------------------------------------------------------------------------------------------------

impl Command for QsyToMarker {
    fn command_id(&self) -> &[u8] { b"#QSY" }
}

// ------------------------------------------------------------------------------------------------
// #RCF — Reference Frequency Calibration Offset (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the reference frequency calibration offset.
///
/// # Reference (P3 rev. A7, §#RCF)
///
/// **GET** format: `#RCF;`
/// **SET/RSP** format: `#RCFsnnnn;` — `s` = `+`/`-`, `nnnn` = 0–9999 Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRefCalibOffset;

/// Set the reference frequency calibration offset in Hz (signed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetRefCalibOffset {
    pub offset_hz: i16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetRefCalibOffset {
    fn command_id(&self) -> &[u8] { b"#RCF" }
}
impl CommandWithResponse for GetRefCalibOffset {
    type Response = i16;
    fn expected_response_length(&self) -> usize { 5 }
    fn parse(&self, bytes: &[u8]) -> Result<i16, RigError> {
        let d = validate_response(bytes, b"#RCF", 5)?;
        parse_signed_i16(d)
    }
}
impl Command for SetRefCalibOffset {
    fn command_id(&self) -> &[u8] { b"#RCF" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let sign = if self.offset_hz >= 0 { b'+' } else { b'-' };
        Some(format!("{}{:04}", sign as char, self.offset_hz.unsigned_abs()).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// #REF — Reference Level (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the spectrum display reference level in dBm.
///
/// # Reference (P3 rev. A7, §#REF)
///
/// **GET** format: `#REF;`
/// **SET/RSP** format: `#REFsnn;` — `s` = sign, `nn` = 0–99 dBm.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetReferenceLevel;

/// Set the spectrum reference level in dBm (signed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetReferenceLevel {
    pub dbm: i8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetReferenceLevel {
    fn command_id(&self) -> &[u8] { b"#REF" }
}
impl CommandWithResponse for GetReferenceLevel {
    type Response = i8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<i8, RigError> {
        let d = validate_response(bytes, b"#REF", 3)?;
        parse_signed_dbm(bytes, d)
    }
}
impl Command for SetReferenceLevel {
    fn command_id(&self) -> &[u8] { b"#REF" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(format_signed_dbm(self.dbm)) }
}

// ------------------------------------------------------------------------------------------------
// #RST — Reset (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Reset the P3 to factory defaults.
///
/// # Reference (P3 rev. A7, §#RST)
///
/// **SET** format: `#RST;` — no response.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResetToDefaults;

// ------------------------------------------------------------------------------------------------

impl Command for ResetToDefaults {
    fn command_id(&self) -> &[u8] { b"#RST" }
}

// ------------------------------------------------------------------------------------------------
// #RVM — Firmware Version (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the P3 firmware version.
///
/// # Reference (P3 rev. A7, §#RVM)
///
/// **GET** format: `#RVM;`
/// **RSP** format: `#RVMnn.nn;` — e.g. `#RVM01.25;`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFirmwareVersion;

// ------------------------------------------------------------------------------------------------

impl Command for GetFirmwareVersion {
    fn command_id(&self) -> &[u8] { b"#RVM" }
}
impl CommandWithResponse for GetFirmwareVersion {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 5 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"#RVM", 5)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// #RVS — Sub-Receiver Version (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the sub-receiver module version string.
///
/// # Reference (P3 rev. A7, §#RVS)
///
/// **GET** format: `#RVS;`
/// **RSP** format: `#RVSss;` — 2-character module ID.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSubRxVersion;

// ------------------------------------------------------------------------------------------------

impl Command for GetSubRxVersion {
    fn command_id(&self) -> &[u8] { b"#RVS" }
}
impl CommandWithResponse for GetSubRxVersion {
    type Response = [u8; 2];
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<[u8; 2], RigError> {
        let d = validate_response(bytes, b"#RVS", 2)?;
        Ok([d[0], d[1]])
    }
}

// ------------------------------------------------------------------------------------------------
// #SCL — Display Span (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current display span in Hz.
///
/// # Reference (P3 rev. A7, §#SCL)
///
/// **GET** format: `#SCL;`
/// **SET/RSP** format: `#SCLnnnnnn;` — span in Hz (6 digits, e.g. `040000` = 40 kHz).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDisplaySpan;

/// Set the display span in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDisplaySpan {
    pub hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetDisplaySpan {
    fn command_id(&self) -> &[u8] { b"#SCL" }
}
impl CommandWithResponse for GetDisplaySpan {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 6 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"#SCL", 6)?;
        parse_u32(d)
    }
}
impl Command for SetDisplaySpan {
    fn command_id(&self) -> &[u8] { b"#SCL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:06}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// #SPM / #SPN — Spectrum Display Levels (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the lower spectrum display level in dBm.
///
/// # Reference (P3 rev. A7, §#SPM)
///
/// **GET** format: `#SPM;`
/// **SET/RSP** format: `#SPMsnn;` — signed dBm.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSpectrumLowerLevel;

/// Set the lower spectrum display level in dBm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpectrumLowerLevel {
    pub dbm: i8,
}

///
/// Query the upper spectrum display level in dBm.
///
/// # Reference (P3 rev. A7, §#SPN)
///
/// **GET** format: `#SPN;`
/// **SET/RSP** format: `#SPNsnn;` — signed dBm.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSpectrumUpperLevel;

/// Set the upper spectrum display level in dBm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpectrumUpperLevel {
    pub dbm: i8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSpectrumLowerLevel {
    fn command_id(&self) -> &[u8] { b"#SPM" }
}
impl CommandWithResponse for GetSpectrumLowerLevel {
    type Response = i8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<i8, RigError> {
        let d = validate_response(bytes, b"#SPM", 3)?;
        parse_signed_dbm(bytes, d)
    }
}
impl Command for SetSpectrumLowerLevel {
    fn command_id(&self) -> &[u8] { b"#SPM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(format_signed_dbm(self.dbm)) }
}

impl Command for GetSpectrumUpperLevel {
    fn command_id(&self) -> &[u8] { b"#SPN" }
}
impl CommandWithResponse for GetSpectrumUpperLevel {
    type Response = i8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<i8, RigError> {
        let d = validate_response(bytes, b"#SPN", 3)?;
        parse_signed_dbm(bytes, d)
    }
}
impl Command for SetSpectrumUpperLevel {
    fn command_id(&self) -> &[u8] { b"#SPN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(format_signed_dbm(self.dbm)) }
}

// ------------------------------------------------------------------------------------------------
// #VFB — VFO B Marker Display (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether VFO B is shown as a marker on the display.
///
/// # Reference (P3 rev. A7, §#VFB)
///
/// **GET** format: `#VFB;`
/// **SET/RSP** format: `#VFBn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetVfoBDisplay;

/// Show or hide the VFO B marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetVfoBDisplay {
    pub visible: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetVfoBDisplay {
    fn command_id(&self) -> &[u8] { b"#VFB" }
}
impl CommandWithResponse for GetVfoBDisplay {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#VFB", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetVfoBDisplay {
    fn command_id(&self) -> &[u8] { b"#VFB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.visible { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #WFA / #WFC — Waterfall Level Limits (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the waterfall minimum level in dBm.
///
/// # Reference (P3 rev. A7, §#WFA)
///
/// **GET** format: `#WFA;`
/// **SET/RSP** format: `#WFAsnn;` — signed dBm.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetWaterfallMin;

/// Set the waterfall minimum level in dBm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetWaterfallMin {
    pub dbm: i8,
}

///
/// Query the waterfall maximum level in dBm.
///
/// # Reference (P3 rev. A7, §#WFC)
///
/// **GET** format: `#WFC;`
/// **SET/RSP** format: `#WFCsnn;` — signed dBm.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetWaterfallMax;

/// Set the waterfall maximum level in dBm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetWaterfallMax {
    pub dbm: i8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetWaterfallMin {
    fn command_id(&self) -> &[u8] { b"#WFA" }
}
impl CommandWithResponse for GetWaterfallMin {
    type Response = i8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<i8, RigError> {
        let d = validate_response(bytes, b"#WFA", 3)?;
        parse_signed_dbm(bytes, d)
    }
}
impl Command for SetWaterfallMin {
    fn command_id(&self) -> &[u8] { b"#WFA" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(format_signed_dbm(self.dbm)) }
}

impl Command for GetWaterfallMax {
    fn command_id(&self) -> &[u8] { b"#WFC" }
}
impl CommandWithResponse for GetWaterfallMax {
    type Response = i8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<i8, RigError> {
        let d = validate_response(bytes, b"#WFC", 3)?;
        parse_signed_dbm(bytes, d)
    }
}
impl Command for SetWaterfallMax {
    fn command_id(&self) -> &[u8] { b"#WFC" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(format_signed_dbm(self.dbm)) }
}

// ------------------------------------------------------------------------------------------------
// #WFM — Waterfall Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the waterfall display mode.
///
/// # Reference (P3 rev. A7, §#WFM)
///
/// **GET** format: `#WFM;`
/// **SET/RSP** format: `#WFMn;` — `n` = 0 (gradient), 1 (line), 2 (highlight).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetWaterfallMode;

/// Set the waterfall display mode (0=gradient, 1=line, 2=highlight).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetWaterfallMode {
    pub mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetWaterfallMode {
    fn command_id(&self) -> &[u8] { b"#WFM" }
}
impl CommandWithResponse for GetWaterfallMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#WFM", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetWaterfallMode {
    fn command_id(&self) -> &[u8] { b"#WFM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.mode.min(2)]) }
}

// ------------------------------------------------------------------------------------------------
// #XCV — Transceiver Connected (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query whether a transceiver is connected to the P3.
///
/// # Reference (P3 rev. A7, §#XCV)
///
/// **GET** format: `#XCV;`
/// **RSP** format: `#XCVn;` — `n` = 0 (no transceiver), 1 (connected).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransceiverConnected;

// ------------------------------------------------------------------------------------------------

impl Command for GetTransceiverConnected {
    fn command_id(&self) -> &[u8] { b"#XCV" }
}
impl CommandWithResponse for GetTransceiverConnected {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#XCV", 1)?;
        Ok(d[0] == b'1')
    }
}

// ------------------------------------------------------------------------------------------------
// Private parse and format helpers
// ------------------------------------------------------------------------------------------------

fn parse_u32(bytes: &[u8]) -> Result<u32, RigError> {
    let mut n = 0u32;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(RigError::InvalidResponseData { data: bytes.to_vec() });
        }
        n = n * 10 + u32::from(b - b'0');
    }
    Ok(n)
}

fn parse_u16(bytes: &[u8]) -> Result<u16, RigError> {
    let mut n = 0u32;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(RigError::InvalidResponseData { data: bytes.to_vec() });
        }
        n = n * 10 + u32::from(b - b'0');
    }
    Ok(n as u16)
}

fn parse_signed_i16(bytes: &[u8]) -> Result<i16, RigError> {
    if bytes.is_empty() {
        return Err(RigError::InvalidResponseData { data: bytes.to_vec() });
    }
    let (sign, digits) = match bytes[0] {
        b'+' => (1i32, &bytes[1..]),
        b'-' => (-1i32, &bytes[1..]),
        _ => (1i32, bytes),
    };
    let mag = parse_u16(digits)? as i32;
    i16::try_from(sign * mag).map_err(|_| RigError::InvalidResponseData { data: bytes.to_vec() })
}

fn parse_signed_dbm(raw: &[u8], d: &[u8]) -> Result<i8, RigError> {
    let sign: i16 = if d[0] == b'-' { -1 } else { 1 };
    let mag = parse_u16(&d[1..3])? as i16;
    i8::try_from(sign * mag).map_err(|_| RigError::InvalidResponseData { data: raw.to_vec() })
}

fn format_signed_dbm(dbm: i8) -> Vec<u8> {
    let sign = if dbm >= 0 { b'+' } else { b'-' };
    format!("{}{:02}", sign as char, dbm.unsigned_abs()).into_bytes()
}
