//!
//! Serial commands for the Elecraft KAT500 automatic antenna tuner.
//!
//! Commands follow the **02.12** programmer's reference
//! (Elecraft KAT500 Programmer's Reference, rev. 02.12).
//!
//! Unlike the amplifier and panadapter modules there is **no wire prefix** — command IDs are
//! sent exactly as shown (e.g. `AN`, `BN`, `VSWR`).
//!

use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// AB — Auto Bypass (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the auto-bypass setting.
///
/// # Reference (KAT500 rev. 02.12, §AB)
///
/// **GET** format: `AB;`
/// **SET/RSP** format: `ABn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAutoBypass;

/// Enable or disable auto-bypass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAutoBypass {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAutoBypass {
    fn command_id(&self) -> &[u8] { b"AB" }
}
impl CommandWithResponse for GetAutoBypass {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AB", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAutoBypass {
    fn command_id(&self) -> &[u8] { b"AB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// AE — Auto Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the ATU is enabled (auto-tuning allowed).
///
/// # Reference (KAT500 rev. 02.12, §AE)
///
/// **GET** format: `AE;`
/// **SET/RSP** format: `AEn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAutoEnable;

/// Enable or disable the ATU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAutoEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAutoEnable {
    fn command_id(&self) -> &[u8] { b"AE" }
}
impl CommandWithResponse for GetAutoEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AE", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAutoEnable {
    fn command_id(&self) -> &[u8] { b"AE" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// AFT — ATU Fault (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the ATU has a fault condition.
///
/// # Reference (KAT500 rev. 02.12, §AFT)
///
/// **GET** format: `AFT;`
/// **RSP** format: `AFTn;` — `n` = 0 (no fault), 1 (fault present).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuFault;

// ------------------------------------------------------------------------------------------------

impl Command for GetAtuFault {
    fn command_id(&self) -> &[u8] { b"AFT" }
}
impl CommandWithResponse for GetAtuFault {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AFT", 1)?;
        Ok(d[0] == b'1')
    }
}

// ------------------------------------------------------------------------------------------------
// AKIP — ATU Keep In Place (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the ATU keep-in-place setting.
///
/// # Reference (KAT500 rev. 02.12, §AKIP)
///
/// **GET** format: `AKIP;`
/// **SET/RSP** format: `AKIPn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuKeepInPlace;

/// Enable or disable ATU keep-in-place mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAtuKeepInPlace {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAtuKeepInPlace {
    fn command_id(&self) -> &[u8] { b"AKIP" }
}
impl CommandWithResponse for GetAtuKeepInPlace {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AKIP", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAtuKeepInPlace {
    fn command_id(&self) -> &[u8] { b"AKIP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// AMPI — Amplifier Interface (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the amplifier interface relay state.
///
/// # Reference (KAT500 rev. 02.12, §AMPI)
///
/// **GET** format: `AMPI;`
/// **SET/RSP** format: `AMPIn;` — `n` = 0 (open), 1 (closed).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAmplifierInterface;

/// Set the amplifier interface relay state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAmplifierInterface {
    pub closed: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAmplifierInterface {
    fn command_id(&self) -> &[u8] { b"AMPI" }
}
impl CommandWithResponse for GetAmplifierInterface {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AMPI", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAmplifierInterface {
    fn command_id(&self) -> &[u8] { b"AMPI" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.closed { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// AN — Antenna (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the currently selected antenna port.
///
/// # Reference (KAT500 rev. 02.12, §AN)
///
/// **GET** format: `AN;`
/// **SET/RSP** format: `ANn;` — `n` = 1–6.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAntenna;

/// Select the antenna port (1–6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAntenna {
    pub antenna: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAntenna {
    fn command_id(&self) -> &[u8] { b"AN" }
}
impl CommandWithResponse for GetAntenna {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"AN", 1)?;
        parse_u8(d)
    }
}
impl Command for SetAntenna {
    fn command_id(&self) -> &[u8] { b"AN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{}", self.antenna).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// AP — ATU Preset (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current ATU preset slot.
///
/// # Reference (KAT500 rev. 02.12, §AP)
///
/// **GET** format: `AP;`
/// **SET/RSP** format: `APnnn;` — 3-digit preset slot number.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuPreset;

/// Recall the specified ATU preset slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAtuPreset {
    pub preset: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAtuPreset {
    fn command_id(&self) -> &[u8] { b"AP" }
}
impl CommandWithResponse for GetAtuPreset {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"AP", 3)?;
        parse_u16(d)
    }
}
impl Command for SetAtuPreset {
    fn command_id(&self) -> &[u8] { b"AP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.preset).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ATTN — Attenuator (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the built-in attenuator is enabled.
///
/// # Reference (KAT500 rev. 02.12, §ATTN)
///
/// **GET** format: `ATTN;`
/// **SET/RSP** format: `ATTNn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAttenuator;

/// Enable or disable the attenuator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAttenuator {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAttenuator {
    fn command_id(&self) -> &[u8] { b"ATTN" }
}
impl CommandWithResponse for GetAttenuator {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"ATTN", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAttenuator {
    fn command_id(&self) -> &[u8] { b"ATTN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// BN — Band Number (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set the current band number.
///
/// # Reference (KAT500 rev. 02.12, §BN)
///
/// **GET** format: `BN;`
/// **SET/RSP** format: `BNnn;` — 2-digit band number (00–13).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBand;

/// Set the current band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBand {
    pub band: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBand {
    fn command_id(&self) -> &[u8] { b"BN" }
}
impl CommandWithResponse for GetBand {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"BN", 2)?;
        parse_u8(d)
    }
}
impl Command for SetBand {
    fn command_id(&self) -> &[u8] { b"BN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.band).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// BR — Baud Rate (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the serial port baud rate.
///
/// # Reference (KAT500 rev. 02.12, §BR)
///
/// **GET** format: `BR;`
/// **SET/RSP** format: `BRn;` — `n` = 0 (4800), 1 (9600), 2 (19200), 3 (38400).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBaudRate;

/// Set the serial port baud rate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBaudRate {
    pub rate_index: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBaudRate {
    fn command_id(&self) -> &[u8] { b"BR" }
}
impl CommandWithResponse for GetBaudRate {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"BR", 1)?;
        parse_u8(d)
    }
}
impl Command for SetBaudRate {
    fn command_id(&self) -> &[u8] { b"BR" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{}", self.rate_index).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// BYP — Bypass (SET only — no query)
// ------------------------------------------------------------------------------------------------

/// Force the ATU into bypass mode immediately.
///
/// # Reference (KAT500 rev. 02.12, §BYP)
///
/// **SET** format: `BYP;` — no argument; result is immediate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bypass;

// ------------------------------------------------------------------------------------------------

impl Command for Bypass {
    fn command_id(&self) -> &[u8] { b"BYP" }
}

// ------------------------------------------------------------------------------------------------
// C — Capacitor (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set the tuning capacitor value.
///
/// # Reference (KAT500 rev. 02.12, §C)
///
/// **GET** format: `C;`
/// **SET/RSP** format: `Cnnn;` — 3-digit capacitor value (0–255).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCapacitor;

/// Set the tuning capacitor value directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetCapacitor {
    pub value: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetCapacitor {
    fn command_id(&self) -> &[u8] { b"C" }
}
impl CommandWithResponse for GetCapacitor {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"C", 3)?;
        parse_u8(d)
    }
}
impl Command for SetCapacitor {
    fn command_id(&self) -> &[u8] { b"C" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.value).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// CT — Capacitor Topology (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set the capacitor topology (hi-Z or lo-Z).
///
/// # Reference (KAT500 rev. 02.12, §CT)
///
/// **GET** format: `CT;`
/// **SET/RSP** format: `CTn;` — `n` = 0 (lo-Z / cap on output), 1 (hi-Z / cap on input).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCapacitorTopology;

/// Set the capacitor topology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetCapacitorTopology {
    pub hi_z: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetCapacitorTopology {
    fn command_id(&self) -> &[u8] { b"CT" }
}
impl CommandWithResponse for GetCapacitorTopology {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"CT", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetCapacitorTopology {
    fn command_id(&self) -> &[u8] { b"CT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.hi_z { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// DM — Demo Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query demo-mode state.
///
/// # Reference (KAT500 rev. 02.12, §DM)
///
/// **GET** format: `DM;`
/// **SET/RSP** format: `DMn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDemoMode;

/// Enable or disable demo mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDemoMode {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetDemoMode {
    fn command_id(&self) -> &[u8] { b"DM" }
}
impl CommandWithResponse for GetDemoMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"DM", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetDemoMode {
    fn command_id(&self) -> &[u8] { b"DM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// EEINIT — EEPROM Init (SET only)
// ------------------------------------------------------------------------------------------------

/// Re-initialise EEPROM to factory defaults.
///
/// # Reference (KAT500 rev. 02.12, §EEINIT)
///
/// **SET** format: `EEINIT;` — no argument; destructive — erases all stored presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EepromInit;

// ------------------------------------------------------------------------------------------------

impl Command for EepromInit {
    fn command_id(&self) -> &[u8] { b"EEINIT" }
}

// ------------------------------------------------------------------------------------------------
// EM — Error Message (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the last error message string.
///
/// # Reference (KAT500 rev. 02.12, §EM)
///
/// **GET** format: `EM;`
/// **RSP** format: `EM<text>;` — variable-length ASCII text.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetErrorMessage;

// ------------------------------------------------------------------------------------------------

impl Command for GetErrorMessage {
    fn command_id(&self) -> &[u8] { b"EM" }
}
impl CommandWithResponse for GetErrorMessage {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 0 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"EM", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// F — Frequency (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set the operating frequency in Hz.
///
/// # Reference (KAT500 rev. 02.12, §F)
///
/// **GET** format: `F;`
/// **SET/RSP** format: `F00000000;` — 8-digit Hz, zero-padded.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFrequency;

/// Set the operating frequency in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFrequency {
    pub freq_hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFrequency {
    fn command_id(&self) -> &[u8] { b"F" }
}
impl CommandWithResponse for GetFrequency {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"F", 8)?;
        parse_u32(d)
    }
}
impl Command for SetFrequency {
    fn command_id(&self) -> &[u8] { b"F" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.freq_hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// FA / FB — Forward Power Meter A/B (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the forward power reading on meter channel A (deci-watts).
///
/// # Reference (KAT500 rev. 02.12, §FA)
///
/// **GET** format: `FA;`
/// **RSP** format: `FAnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetForwardPowerA;

///
/// Query the forward power reading on meter channel B (deci-watts).
///
/// # Reference (KAT500 rev. 02.12, §FB)
///
/// **GET** format: `FB;`
/// **RSP** format: `FBnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetForwardPowerB;

// ------------------------------------------------------------------------------------------------

impl Command for GetForwardPowerA {
    fn command_id(&self) -> &[u8] { b"FA" }
}
impl CommandWithResponse for GetForwardPowerA {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FA", 3)?;
        parse_u16(d)
    }
}

impl Command for GetForwardPowerB {
    fn command_id(&self) -> &[u8] { b"FB" }
}
impl CommandWithResponse for GetForwardPowerB {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FB", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// FC — Fan Control threshold (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fan-on power threshold (watts).
///
/// # Reference (KAT500 rev. 02.12, §FC)
///
/// **GET** format: `FC;`
/// **SET/RSP** format: `FCnnn;` — 3-digit threshold in watts.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFanThreshold;

/// Set the fan-on power threshold in watts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFanThreshold {
    pub threshold_w: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFanThreshold {
    fn command_id(&self) -> &[u8] { b"FC" }
}
impl CommandWithResponse for GetFanThreshold {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FC", 3)?;
        parse_u16(d)
    }
}
impl Command for SetFanThreshold {
    fn command_id(&self) -> &[u8] { b"FC" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.threshold_w).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// FDT — Fault Delay Time (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fault delay time (ms).
///
/// # Reference (KAT500 rev. 02.12, §FDT)
///
/// **GET** format: `FDT;`
/// **SET/RSP** format: `FDTnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultDelayTime;

/// Set the fault delay time in milliseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFaultDelayTime {
    pub delay_ms: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFaultDelayTime {
    fn command_id(&self) -> &[u8] { b"FDT" }
}
impl CommandWithResponse for GetFaultDelayTime {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FDT", 3)?;
        parse_u16(d)
    }
}
impl Command for SetFaultDelayTime {
    fn command_id(&self) -> &[u8] { b"FDT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.delay_ms).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// FLT / FLTC — Fault Status / Clear Fault (GET / SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fault status code.
///
/// # Reference (KAT500 rev. 02.12, §FLT)
///
/// **GET** format: `FLT;`
/// **RSP** format: `FLTnn;` — 2-digit code (00 = no fault).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultStatus;

/// Clear the current fault.
///
/// # Reference (KAT500 rev. 02.12, §FLTC)
///
/// **SET** format: `FLTC;` — no argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearFault;

// ------------------------------------------------------------------------------------------------

impl Command for GetFaultStatus {
    fn command_id(&self) -> &[u8] { b"FLT" }
}
impl CommandWithResponse for GetFaultStatus {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"FLT", 2)?;
        parse_u8(d)
    }
}
impl Command for ClearFault {
    fn command_id(&self) -> &[u8] { b"FLTC" }
}

// ------------------------------------------------------------------------------------------------
// FTNS — Tune SWR Threshold (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the SWR threshold used to consider a tune successful.
///
/// # Reference (KAT500 rev. 02.12, §FTNS)
///
/// **GET** format: `FTNS;`
/// **SET/RSP** format: `FTNSnnn;` — SWR × 10 (e.g. `150` = 1.5:1).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTuneSatisfiedSwr;

/// Set the tune-satisfied SWR threshold (SWR × 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTuneSatisfiedSwr {
    pub swr_d: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetTuneSatisfiedSwr {
    fn command_id(&self) -> &[u8] { b"FTNS" }
}
impl CommandWithResponse for GetTuneSatisfiedSwr {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FTNS", 3)?;
        parse_u16(d)
    }
}
impl Command for SetTuneSatisfiedSwr {
    fn command_id(&self) -> &[u8] { b"FTNS" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.swr_d).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// FT0 / FT1 — Fault Threshold Low / High (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the lower fault SWR threshold (SWR × 10).
///
/// # Reference (KAT500 rev. 02.12, §FT0)
///
/// **GET** format: `FT0;`
/// **SET/RSP** format: `FT0nnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultThresholdLow;

/// Set the lower fault SWR threshold (SWR × 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFaultThresholdLow {
    pub swr_d: u16,
}

///
/// Query the upper fault SWR threshold (SWR × 10).
///
/// # Reference (KAT500 rev. 02.12, §FT1)
///
/// **GET** format: `FT1;`
/// **SET/RSP** format: `FT1nnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultThresholdHigh;

/// Set the upper fault SWR threshold (SWR × 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFaultThresholdHigh {
    pub swr_d: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFaultThresholdLow {
    fn command_id(&self) -> &[u8] { b"FT0" }
}
impl CommandWithResponse for GetFaultThresholdLow {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FT0", 3)?;
        parse_u16(d)
    }
}
impl Command for SetFaultThresholdLow {
    fn command_id(&self) -> &[u8] { b"FT0" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.swr_d).into_bytes())
    }
}

impl Command for GetFaultThresholdHigh {
    fn command_id(&self) -> &[u8] { b"FT1" }
}
impl CommandWithResponse for GetFaultThresholdHigh {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FT1", 3)?;
        parse_u16(d)
    }
}
impl Command for SetFaultThresholdHigh {
    fn command_id(&self) -> &[u8] { b"FT1" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.swr_d).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// FX — Fixed L/C (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether fixed L/C mode is active.
///
/// # Reference (KAT500 rev. 02.12, §FX)
///
/// **GET** format: `FX;`
/// **SET/RSP** format: `FXn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFixedLc;

/// Enable or disable fixed L/C mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFixedLc {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFixedLc {
    fn command_id(&self) -> &[u8] { b"FX" }
}
impl CommandWithResponse for GetFixedLc {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"FX", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetFixedLc {
    fn command_id(&self) -> &[u8] { b"FX" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// FY — Fixed Bypass (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether fixed bypass mode is active.
///
/// # Reference (KAT500 rev. 02.12, §FY)
///
/// **GET** format: `FY;`
/// **SET/RSP** format: `FYn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFixedBypass;

/// Enable or disable fixed bypass mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFixedBypass {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFixedBypass {
    fn command_id(&self) -> &[u8] { b"FY" }
}
impl CommandWithResponse for GetFixedBypass {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"FY", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetFixedBypass {
    fn command_id(&self) -> &[u8] { b"FY" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// I — Inductance (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set the tuning inductance tap (0–63).
///
/// # Reference (KAT500 rev. 02.12, §I)
///
/// **GET** format: `I;`
/// **SET/RSP** format: `Innn;` — 3-digit inductor tap value.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetInductance;

/// Set the tuning inductance tap directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetInductance {
    pub tap: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetInductance {
    fn command_id(&self) -> &[u8] { b"I" }
}
impl CommandWithResponse for GetInductance {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"I", 3)?;
        parse_u8(d)
    }
}
impl Command for SetInductance {
    fn command_id(&self) -> &[u8] { b"I" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.tap).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// IF — Inhibit Fan (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the fan is inhibited.
///
/// # Reference (KAT500 rev. 02.12, §IF)
///
/// **GET** format: `IF;`
/// **SET/RSP** format: `IFn;` — `n` = 0 (fan enabled), 1 (fan inhibited).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetInhibitFan;

/// Inhibit or enable the cooling fan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetInhibitFan {
    pub inhibit: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetInhibitFan {
    fn command_id(&self) -> &[u8] { b"IF" }
}
impl CommandWithResponse for GetInhibitFan {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"IF", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetInhibitFan {
    fn command_id(&self) -> &[u8] { b"IF" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.inhibit { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// L — Inductance Switch (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the inductor switch bitmask.
///
/// # Reference (KAT500 rev. 02.12, §L)
///
/// **GET** format: `L;`
/// **SET/RSP** format: `Lnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetInductorSwitch;

/// Set the inductor switch bitmask directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetInductorSwitch {
    pub mask: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetInductorSwitch {
    fn command_id(&self) -> &[u8] { b"L" }
}
impl CommandWithResponse for GetInductorSwitch {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"L", 3)?;
        parse_u8(d)
    }
}
impl Command for SetInductorSwitch {
    fn command_id(&self) -> &[u8] { b"L" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.mask).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MD — Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current operating mode.
///
/// # Reference (KAT500 rev. 02.12, §MD)
///
/// **GET** format: `MD;`
/// **SET/RSP** format: `MDn;` — `n` = 0 (auto), 1 (semi-auto), 2 (manual).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOperatingMode;

/// Set the operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOperatingMode {
    pub mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetOperatingMode {
    fn command_id(&self) -> &[u8] { b"MD" }
}
impl CommandWithResponse for GetOperatingMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MD", 1)?;
        parse_u8(d)
    }
}
impl Command for SetOperatingMode {
    fn command_id(&self) -> &[u8] { b"MD" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{}", self.mode).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MT — Meter Type (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the meter display type.
///
/// # Reference (KAT500 rev. 02.12, §MT)
///
/// **GET** format: `MT;`
/// **SET/RSP** format: `MTn;` — `n` = 0 (SWR), 1 (power), 2 (reflected).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMeterType;

/// Set the meter display type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMeterType {
    pub meter: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetMeterType {
    fn command_id(&self) -> &[u8] { b"MT" }
}
impl CommandWithResponse for GetMeterType {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MT", 1)?;
        parse_u8(d)
    }
}
impl Command for SetMeterType {
    fn command_id(&self) -> &[u8] { b"MT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{}", self.meter).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// PS — Power Status (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the power-on status.
///
/// # Reference (KAT500 rev. 02.12, §PS)
///
/// **GET** format: `PS;`
/// **RSP** format: `PSn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerStatus;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerStatus {
    fn command_id(&self) -> &[u8] { b"PS" }
}
impl CommandWithResponse for GetPowerStatus {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"PS", 1)?;
        Ok(d[0] == b'1')
    }
}

// ------------------------------------------------------------------------------------------------
// PSI — Power Sensor Input (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the forward power from the internal sensor (deci-watts).
///
/// # Reference (KAT500 rev. 02.12, §PSI)
///
/// **GET** format: `PSI;`
/// **RSP** format: `PSInnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerSensorInput;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerSensorInput {
    fn command_id(&self) -> &[u8] { b"PSI" }
}
impl CommandWithResponse for GetPowerSensorInput {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"PSI", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// RSTx — Reset (SET only)
// ------------------------------------------------------------------------------------------------

/// Perform a soft reset of the KAT500.
///
/// # Reference (KAT500 rev. 02.12, §RSTx)
///
/// **SET** format: `RSTX;` — triggers firmware restart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResetDevice;

// ------------------------------------------------------------------------------------------------

impl Command for ResetDevice {
    fn command_id(&self) -> &[u8] { b"RSTX" }
}

// ------------------------------------------------------------------------------------------------
// RV — Firmware Version (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the firmware version string.
///
/// # Reference (KAT500 rev. 02.12, §RV)
///
/// **GET** format: `RV;`
/// **RSP** format: `RV<version>;` — e.g. `RV02.12;`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFirmwareVersion;

// ------------------------------------------------------------------------------------------------

impl Command for GetFirmwareVersion {
    fn command_id(&self) -> &[u8] { b"RV" }
}
impl CommandWithResponse for GetFirmwareVersion {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 0 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"RV", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// SIDE — Antenna Side (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the antenna side selection.
///
/// # Reference (KAT500 rev. 02.12, §SIDE)
///
/// **GET** format: `SIDE;`
/// **SET/RSP** format: `SIDEn;` — `n` = 0 (left), 1 (right).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAntennaSide;

/// Set the antenna side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAntennaSide {
    pub right: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAntennaSide {
    fn command_id(&self) -> &[u8] { b"SIDE" }
}
impl CommandWithResponse for GetAntennaSide {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"SIDE", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAntennaSide {
    fn command_id(&self) -> &[u8] { b"SIDE" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.right { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// SL — Speed Limit (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the tuning speed limit setting.
///
/// # Reference (KAT500 rev. 02.12, §SL)
///
/// **GET** format: `SL;`
/// **SET/RSP** format: `SLn;` — `n` = 0 (fastest) … 9 (slowest).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSpeedLimit;

/// Set the tuning speed limit (0–9).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpeedLimit {
    pub level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSpeedLimit {
    fn command_id(&self) -> &[u8] { b"SL" }
}
impl CommandWithResponse for GetSpeedLimit {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"SL", 1)?;
        parse_u8(d)
    }
}
impl Command for SetSpeedLimit {
    fn command_id(&self) -> &[u8] { b"SL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{}", self.level).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// SM — SWR Meter (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current SWR reading (SWR × 10).
///
/// # Reference (KAT500 rev. 02.12, §SM)
///
/// **GET** format: `SM;`
/// **RSP** format: `SMnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSwrMeter;

// ------------------------------------------------------------------------------------------------

impl Command for GetSwrMeter {
    fn command_id(&self) -> &[u8] { b"SM" }
}
impl CommandWithResponse for GetSwrMeter {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"SM", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// SN — Serial Number (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the unit serial number.
///
/// # Reference (KAT500 rev. 02.12, §SN)
///
/// **GET** format: `SN;`
/// **RSP** format: `SN<number>;` — variable-length decimal string.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSerialNumber;

// ------------------------------------------------------------------------------------------------

impl Command for GetSerialNumber {
    fn command_id(&self) -> &[u8] { b"SN" }
}
impl CommandWithResponse for GetSerialNumber {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 0 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"SN", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// ST — Start Tune (SET only)
// ------------------------------------------------------------------------------------------------

/// Initiate a tuning cycle.
///
/// # Reference (KAT500 rev. 02.12, §ST)
///
/// **SET** format: `ST;` — no argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartTune;

// ------------------------------------------------------------------------------------------------

impl Command for StartTune {
    fn command_id(&self) -> &[u8] { b"ST" }
}

// ------------------------------------------------------------------------------------------------
// T — Tune State (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether a tuning cycle is in progress.
///
/// # Reference (KAT500 rev. 02.12, §T)
///
/// **GET** format: `T;`
/// **RSP** format: `Tn;` — `n` = 0 (idle), 1 (tuning).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTuneState;

// ------------------------------------------------------------------------------------------------

impl Command for GetTuneState {
    fn command_id(&self) -> &[u8] { b"T" }
}
impl CommandWithResponse for GetTuneState {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"T", 1)?;
        Ok(d[0] == b'1')
    }
}

// ------------------------------------------------------------------------------------------------
// TP — Tune Power (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the tune power level (watts).
///
/// # Reference (KAT500 rev. 02.12, §TP)
///
/// **GET** format: `TP;`
/// **SET/RSP** format: `TPnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTunePower;

/// Set the tune power level in watts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTunePower {
    pub power_w: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetTunePower {
    fn command_id(&self) -> &[u8] { b"TP" }
}
impl CommandWithResponse for GetTunePower {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"TP", 3)?;
        parse_u16(d)
    }
}
impl Command for SetTunePower {
    fn command_id(&self) -> &[u8] { b"TP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.power_w).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// VFWD — Forward Voltage (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the ADC forward voltage reading (raw counts).
///
/// # Reference (KAT500 rev. 02.12, §VFWD)
///
/// **GET** format: `VFWD;`
/// **RSP** format: `VFWDnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetForwardVoltage;

// ------------------------------------------------------------------------------------------------

impl Command for GetForwardVoltage {
    fn command_id(&self) -> &[u8] { b"VFWD" }
}
impl CommandWithResponse for GetForwardVoltage {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"VFWD", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// VRFL — Reflected Voltage (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the ADC reflected voltage reading (raw counts).
///
/// # Reference (KAT500 rev. 02.12, §VRFL)
///
/// **GET** format: `VRFL;`
/// **RSP** format: `VRFLnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetReflectedVoltage;

// ------------------------------------------------------------------------------------------------

impl Command for GetReflectedVoltage {
    fn command_id(&self) -> &[u8] { b"VRFL" }
}
impl CommandWithResponse for GetReflectedVoltage {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"VRFL", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// VSWR — SWR Reading (GET)
// ------------------------------------------------------------------------------------------------

///
/// Query the computed SWR (SWR × 10).
///
/// # Reference (KAT500 rev. 02.12, §VSWR)
///
/// **GET** format: `VSWR;`
/// **RSP** format: `VSWRnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSwr;

// ------------------------------------------------------------------------------------------------

impl Command for GetSwr {
    fn command_id(&self) -> &[u8] { b"VSWR" }
}
impl CommandWithResponse for GetSwr {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"VSWR", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// VSWRB — SWR Bypass Threshold (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the SWR threshold above which bypass is engaged (SWR × 10).
///
/// # Reference (KAT500 rev. 02.12, §VSWRB)
///
/// **GET** format: `VSWRB;`
/// **SET/RSP** format: `VSWRBnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSwrBypassThreshold;

/// Set the SWR bypass threshold (SWR × 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSwrBypassThreshold {
    pub swr_d: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSwrBypassThreshold {
    fn command_id(&self) -> &[u8] { b"VSWRB" }
}
impl CommandWithResponse for GetSwrBypassThreshold {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"VSWRB", 3)?;
        parse_u16(d)
    }
}
impl Command for SetSwrBypassThreshold {
    fn command_id(&self) -> &[u8] { b"VSWRB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.swr_d).into_bytes())
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

fn parse_u16(bytes: &[u8]) -> Result<u16, RigError> {
    let mut n = 0u32;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(RigError::InvalidResponseData { data: bytes.to_vec() });
        }
        n = n * 10 + u32::from(b - b'0');
    }
    u16::try_from(n).map_err(|_| RigError::InvalidResponseData { data: bytes.to_vec() })
}

fn parse_u32(bytes: &[u8]) -> Result<u32, RigError> {
    let mut n = 0u64;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(RigError::InvalidResponseData { data: bytes.to_vec() });
        }
        n = n * 10 + u64::from(b - b'0');
    }
    u32::try_from(n).map_err(|_| RigError::InvalidResponseData { data: bytes.to_vec() })
}
