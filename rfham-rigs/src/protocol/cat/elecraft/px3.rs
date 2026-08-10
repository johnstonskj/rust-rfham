//!
//! Serial commands for the Elecraft PX3 panadapter.
//!
//! Commands follow the **A6** programmer's reference
//! (Elecraft PX3 Programmer's Reference, rev. A6, Feb 2017).
//!
//! The PX3 is a superset of the [`super::p3`] module. All P3 commands are supported here via
//! re-exports; PX3-specific additions follow below.
//!
//! All commands use the `#` prefix. The `=` product-ID query has no prefix.
//!

pub use super::p3::{
    CaptureBitmap, GetBandLabelDisplay, GetBaudRate, GetCenterFreqTrack, GetDisplayAveraging,
    GetDisplayMode, GetDisplaySpan, GetFixedSpanFreqA, GetFixedSpanFreqB, GetFirmwareVersion,
    GetFontSize, GetMarkerAActive, GetMarkerBActive, GetMarkerFreqA, GetMarkerFreqB,
    GetNoiseBlankerLevel, GetNoiseBlankerLowPass, GetPassThroughMode, GetPeakHoldMode,
    GetPowerSave, GetProductId, GetRefCalibOffset, GetReferenceLevel, GetSpanLowerLimit,
    GetSpanUpperLimit, GetSpectrumLowerLevel, GetSpectrumUpperLevel, GetSubRxVersion,
    GetTransceiverConnected, GetVfoBDisplay, GetWaterfallMax, GetWaterfallMin, GetWaterfallMode,
    QsyToMarker, ResetToDefaults, SetBandLabelDisplay, SetBaudRate, SetCenterFreqTrack,
    SetDisplayAveraging, SetDisplayMode, SetDisplaySpan, SetFixedSpanFreqA, SetFixedSpanFreqB,
    SetFontSize, SetMarkerAActive, SetMarkerBActive, SetMarkerFreqA, SetMarkerFreqB,
    SetNoiseBlankerLevel, SetNoiseBlankerLowPass, SetPassThroughMode, SetPeakHoldMode,
    SetPowerSave, SetRefCalibOffset, SetReferenceLevel, SetSpanLowerLimit, SetSpanUpperLimit,
    SetSpectrumLowerLevel, SetSpectrumUpperLevel, SetVfoBDisplay, SetWaterfallMax,
    SetWaterfallMin, SetWaterfallMode,
};

use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// #BCI — Bandscope Channel Indicator (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query the bandscope channel indicator setting.
///
/// # Reference (PX3 rev. A6, §#BCI)
///
/// **GET** format: `#BCI;`
/// **SET/RSP** format: `#BCIn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandscopeChannelIndicator;

/// Enable or disable the bandscope channel indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandscopeChannelIndicator {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBandscopeChannelIndicator {
    fn command_id(&self) -> &[u8] { b"#BCI" }
}
impl CommandWithResponse for GetBandscopeChannelIndicator {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#BCI", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetBandscopeChannelIndicator {
    fn command_id(&self) -> &[u8] { b"#BCI" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #BCL — Bandscope Channel List (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query or set the bandscope channel list overlay.
///
/// # Reference (PX3 rev. A6, §#BCL)
///
/// **GET** format: `#BCL;`
/// **SET/RSP** format: `#BCLn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandscopeChannelList;

/// Enable or disable the bandscope channel list overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandscopeChannelList {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBandscopeChannelList {
    fn command_id(&self) -> &[u8] { b"#BCL" }
}
impl CommandWithResponse for GetBandscopeChannelList {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#BCL", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetBandscopeChannelList {
    fn command_id(&self) -> &[u8] { b"#BCL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #BCN — Bandscope Channel Name (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query or set the bandscope channel name overlay.
///
/// # Reference (PX3 rev. A6, §#BCN)
///
/// **GET** format: `#BCN;`
/// **SET/RSP** format: `#BCNn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandscopeChannelName;

/// Enable or disable the bandscope channel name overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandscopeChannelName {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBandscopeChannelName {
    fn command_id(&self) -> &[u8] { b"#BCN" }
}
impl CommandWithResponse for GetBandscopeChannelName {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#BCN", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetBandscopeChannelName {
    fn command_id(&self) -> &[u8] { b"#BCN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #CAL — Calibration Signal (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query or control the internal calibration signal.
///
/// # Reference (PX3 rev. A6, §#CAL)
///
/// **GET** format: `#CAL;`
/// **SET/RSP** format: `#CALn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCalibSignal;

/// Enable or disable the internal calibration signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetCalibSignal {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetCalibSignal {
    fn command_id(&self) -> &[u8] { b"#CAL" }
}
impl CommandWithResponse for GetCalibSignal {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#CAL", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetCalibSignal {
    fn command_id(&self) -> &[u8] { b"#CAL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #MAA / #MBA — Memory Antenna Assignments (GET only) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query the antenna band-map stored in memory slot A.
///
/// # Reference (PX3 rev. A6, §#MAA)
///
/// **GET** format: `#MAA;`
/// **RSP** format: `#MAA…;` — variable-length antenna map data.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMemoryAntennaA;

///
/// Query the antenna band-map stored in memory slot B.
///
/// # Reference (PX3 rev. A6, §#MBA)
///
/// **GET** format: `#MBA;`
/// **RSP** format: `#MBA…;` — variable-length antenna map data.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMemoryAntennaB;

// ------------------------------------------------------------------------------------------------

impl Command for GetMemoryAntennaA {
    fn command_id(&self) -> &[u8] { b"#MAA" }
}
impl CommandWithResponse for GetMemoryAntennaA {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 0 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"#MAA", 0)?;
        Ok(d.to_vec())
    }
}

impl Command for GetMemoryAntennaB {
    fn command_id(&self) -> &[u8] { b"#MBA" }
}
impl CommandWithResponse for GetMemoryAntennaB {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 0 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"#MBA", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// #OSBP / #OSBA — Off-screen Bandscope (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query the off-screen bandscope vertical position (pixels from top).
///
/// # Reference (PX3 rev. A6, §#OSBP)
///
/// **GET** format: `#OSBP;`
/// **SET/RSP** format: `#OSBPnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOffscreenBandscopePosition;

/// Set the off-screen bandscope vertical position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOffscreenBandscopePosition {
    pub position: u8,
}

///
/// Query whether the off-screen bandscope is active.
///
/// # Reference (PX3 rev. A6, §#OSBA)
///
/// **GET** format: `#OSBA;`
/// **SET/RSP** format: `#OSBAn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOffscreenBandscopeActive;

/// Enable or disable the off-screen bandscope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOffscreenBandscopeActive {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetOffscreenBandscopePosition {
    fn command_id(&self) -> &[u8] { b"#OSBP" }
}
impl CommandWithResponse for GetOffscreenBandscopePosition {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"#OSBP", 2)?;
        parse_u8(d)
    }
}
impl Command for SetOffscreenBandscopePosition {
    fn command_id(&self) -> &[u8] { b"#OSBP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.position).into_bytes())
    }
}

impl Command for GetOffscreenBandscopeActive {
    fn command_id(&self) -> &[u8] { b"#OSBA" }
}
impl CommandWithResponse for GetOffscreenBandscopeActive {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#OSBA", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetOffscreenBandscopeActive {
    fn command_id(&self) -> &[u8] { b"#OSBA" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #TXH / #TXM — TX Hold and TX Marker (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query the TX hold display setting.
///
/// # Reference (PX3 rev. A6, §#TXH)
///
/// **GET** format: `#TXH;`
/// **SET/RSP** format: `#TXHn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTxHold;

/// Enable or disable TX hold display.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTxHold {
    pub enabled: bool,
}

///
/// Query the TX marker display setting.
///
/// # Reference (PX3 rev. A6, §#TXM)
///
/// **GET** format: `#TXM;`
/// **SET/RSP** format: `#TXMn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTxMarker;

/// Enable or disable the TX frequency marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTxMarker {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetTxHold {
    fn command_id(&self) -> &[u8] { b"#TXH" }
}
impl CommandWithResponse for GetTxHold {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#TXH", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetTxHold {
    fn command_id(&self) -> &[u8] { b"#TXH" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

impl Command for GetTxMarker {
    fn command_id(&self) -> &[u8] { b"#TXM" }
}
impl CommandWithResponse for GetTxMarker {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#TXM", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetTxMarker {
    fn command_id(&self) -> &[u8] { b"#TXM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// #USB — USB Audio Enable (GET/SET) — PX3 only
// ------------------------------------------------------------------------------------------------

///
/// Query whether USB audio output is enabled.
///
/// # Reference (PX3 rev. A6, §#USB)
///
/// **GET** format: `#USB;`
/// **SET/RSP** format: `#USBn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetUsbAudioEnable;

/// Enable or disable USB audio output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetUsbAudioEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetUsbAudioEnable {
    fn command_id(&self) -> &[u8] { b"#USB" }
}
impl CommandWithResponse for GetUsbAudioEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"#USB", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetUsbAudioEnable {
    fn command_id(&self) -> &[u8] { b"#USB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// Private parse helpers
// ------------------------------------------------------------------------------------------------

fn parse_u8(bytes: &[u8]) -> Result<u8, RigError> {
    let mut n = 0u16;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(RigError::InvalidResponseData { data: bytes.to_vec() });
        }
        n = n * 10 + u16::from(b - b'0');
    }
    u8::try_from(n).map_err(|_| RigError::InvalidResponseData { data: bytes.to_vec() })
}
