//!
//! Serial commands for the Elecraft KPA1500 1500-watt solid-state HF/6m amplifier.
//!
//! Commands follow the **V3** programmer's reference
//! (KPA1500 Programming Reference V3.03, Jun 2026).
//!
//! All KPA1500 commands and responses use a leading caret (`^`), for example `^BN05;`.
//! The GET form of a command is the bare command letters with no data: `^BN;`.
//!

use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// ^AA — Auto-Antenna Selection (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query automatic antenna selection mode.
///
/// # Reference (KPA1500 V3, §^AA)
///
/// **GET** format: `^AA;`
/// **SET/RSP** format: `^AAn;` — `n` = 0 (off), 1 (automatic by band).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAutoAntennaSelection;

/// Enable or disable automatic antenna selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAutoAntennaSelection {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAutoAntennaSelection {
    fn command_id(&self) -> &[u8] { b"^AA" }
}
impl CommandWithResponse for GetAutoAntennaSelection {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AA", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAutoAntennaSelection {
    fn command_id(&self) -> &[u8] { b"^AA" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^AB — Antenna Band Map (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the antenna assigned to the current band.
///
/// # Reference (KPA1500 V3, §^AB)
///
/// **GET** format: `^AB;`
/// **SET/RSP** format: `^ABn;` — `n` = 1 or 2.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAntennaBandMap;

/// Set the antenna for the current band (1 or 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAntennaBandMap {
    pub antenna: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAntennaBandMap {
    fn command_id(&self) -> &[u8] { b"^AB" }
}
impl CommandWithResponse for GetAntennaBandMap {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^AB", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetAntennaBandMap {
    fn command_id(&self) -> &[u8] { b"^AB" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.antenna.clamp(1, 2)])
    }
}

// ------------------------------------------------------------------------------------------------
// ^AD — ADC Readings (GET only)
// ------------------------------------------------------------------------------------------------

///
/// ADC readings returned by [`GetAdcReadings`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdcReadings {
    /// PA drain voltage in tenths of a volt.
    pub drain_voltage_dv: u16,
    /// PA drain current in tenths of an ampere.
    pub drain_current_da: u16,
    /// Supply voltage in tenths of a volt.
    pub supply_voltage_dv: u16,
}

///
/// Query ADC readings (drain voltage, drain current, supply voltage).
///
/// # Reference (KPA1500 V3, §^AD)
///
/// **GET** format: `^AD;`
/// **RSP** format: `^ADvvv iii sss;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAdcReadings;

// ------------------------------------------------------------------------------------------------

impl Command for GetAdcReadings {
    fn command_id(&self) -> &[u8] { b"^AD" }
}
impl CommandWithResponse for GetAdcReadings {
    type Response = AdcReadings;
    fn expected_response_length(&self) -> usize { 11 }
    fn parse(&self, bytes: &[u8]) -> Result<AdcReadings, RigError> {
        let d = validate_response(bytes, b"^AD", 11)?;
        Ok(AdcReadings {
            drain_voltage_dv: parse_u16(&d[0..3])?,
            drain_current_da: parse_u16(&d[4..7])?,
            supply_voltage_dv: parse_u16(&d[8..11])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^AE — ALC Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the ALC (Automatic Level Control) output is enabled.
///
/// # Reference (KPA1500 V3, §^AE)
///
/// **GET** format: `^AE;`
/// **SET/RSP** format: `^AEn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAlcEnable;

/// Enable or disable the ALC output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAlcEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAlcEnable {
    fn command_id(&self) -> &[u8] { b"^AE" }
}
impl CommandWithResponse for GetAlcEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AE", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAlcEnable {
    fn command_id(&self) -> &[u8] { b"^AE" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^AI — Auto-Information Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the auto-information (AI) mode.
///
/// # Reference (KPA1500 V3, §^AI)
///
/// **GET** format: `^AI;`
/// **SET/RSP** format: `^AIn;` — `n` = 0 (off), 1 (report status changes automatically).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAutoInfoMode;

/// Set auto-information mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAutoInfoMode {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAutoInfoMode {
    fn command_id(&self) -> &[u8] { b"^AI" }
}
impl CommandWithResponse for GetAutoInfoMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AI", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAutoInfoMode {
    fn command_id(&self) -> &[u8] { b"^AI" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^AL — ALC Threshold (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the ALC threshold for the current band.
///
/// # Reference (KPA1500 V3, §^AL)
///
/// **GET** format: `^AL;`
/// **SET/RSP** format: `^ALnnn;` — 000–210.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAlcThreshold;

/// Set the ALC threshold (0–210).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAlcThreshold {
    pub value: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAlcThreshold {
    fn command_id(&self) -> &[u8] { b"^AL" }
}
impl CommandWithResponse for GetAlcThreshold {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^AL", 3)?;
        parse_u16(d)
    }
}
impl Command for SetAlcThreshold {
    fn command_id(&self) -> &[u8] { b"^AL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.value.min(210)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^AM — AM Mode Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether AM mode is enabled.
///
/// # Reference (KPA1500 V3, §^AM)
///
/// **GET** format: `^AM;`
/// **SET/RSP** format: `^AMn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAmModeEnable;

/// Enable or disable AM mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAmModeEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAmModeEnable {
    fn command_id(&self) -> &[u8] { b"^AM" }
}
impl CommandWithResponse for GetAmModeEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AM", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAmModeEnable {
    fn command_id(&self) -> &[u8] { b"^AM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^AN — Antenna Selection (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the selected antenna port.
///
/// # Reference (KPA1500 V3, §^AN)
///
/// **GET** format: `^AN;`
/// **SET/RSP** format: `^ANn;` — `n` = 1 or 2.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAntennaSelection;

/// Select antenna port (1 or 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAntennaSelection {
    pub antenna: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAntennaSelection {
    fn command_id(&self) -> &[u8] { b"^AN" }
}
impl CommandWithResponse for GetAntennaSelection {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^AN", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetAntennaSelection {
    fn command_id(&self) -> &[u8] { b"^AN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.antenna.clamp(1, 2)])
    }
}

// ------------------------------------------------------------------------------------------------
// ^AP — ATU Preset Recall (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether an ATU preset is loaded for the current frequency.
///
/// # Reference (KPA1500 V3, §^AP)
///
/// **GET** format: `^AP;`
/// **RSP** format: `^APn;` — `n` = 0 (no preset), 1 (preset loaded).
/// **SET** format: `^AP;` — triggers recall of the stored ATU preset (use [`RecallAtuPreset`]).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuPreset;

/// Trigger ATU preset recall.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecallAtuPreset;

// ------------------------------------------------------------------------------------------------

impl Command for GetAtuPreset {
    fn command_id(&self) -> &[u8] { b"^AP" }
}
impl CommandWithResponse for GetAtuPreset {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AP", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for RecallAtuPreset {
    fn command_id(&self) -> &[u8] { b"^AP" }
}

// ------------------------------------------------------------------------------------------------
// ^AR — Attenuator Fault Release Time (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the attenuator fault release time.
///
/// # Reference (KPA1500 V3, §^AR)
///
/// **GET** format: `^AR;`
/// **SET/RSP** format: `^ARnnnn;` — milliseconds (1400–5000).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAttenuatorReleaseTime;

/// Set the attenuator fault release time in milliseconds (1400–5000).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAttenuatorReleaseTime {
    pub ms: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAttenuatorReleaseTime {
    fn command_id(&self) -> &[u8] { b"^AR" }
}
impl CommandWithResponse for GetAttenuatorReleaseTime {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 4 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^AR", 4)?;
        parse_u16(d)
    }
}
impl Command for SetAttenuatorReleaseTime {
    fn command_id(&self) -> &[u8] { b"^AR" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.ms.clamp(1400, 5000)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^AS — ATU Status (GET only)
// ------------------------------------------------------------------------------------------------

///
/// ATU status returned by [`GetAtuStatus`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtuStatus {
    pub tuning: bool,
    pub in_line: bool,
    pub preset_loaded: bool,
}

///
/// Query ATU status.
///
/// # Reference (KPA1500 V3, §^AS)
///
/// **GET** format: `^AS;`
/// **RSP** format: `^AStip;` — `t` = tuning, `i` = in-line, `p` = preset loaded.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuStatus;

// ------------------------------------------------------------------------------------------------

impl Command for GetAtuStatus {
    fn command_id(&self) -> &[u8] { b"^AS" }
}
impl CommandWithResponse for GetAtuStatus {
    type Response = AtuStatus;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<AtuStatus, RigError> {
        let d = validate_response(bytes, b"^AS", 3)?;
        Ok(AtuStatus {
            tuning: d[0] == b'1',
            in_line: d[1] == b'1',
            preset_loaded: d[2] == b'1',
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^BC — STBY on Band Change (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the amplifier returns to standby on band change.
///
/// # Reference (KPA1500 V3, §^BC)
///
/// **GET** format: `^BC;`
/// **SET/RSP** format: `^BCn;` — `n` = 0 (return to prior state), 1 (stay in STBY).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetStandbyOnBandChange;

/// Set standby-on-band-change behaviour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetStandbyOnBandChange {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetStandbyOnBandChange {
    fn command_id(&self) -> &[u8] { b"^BC" }
}
impl CommandWithResponse for GetStandbyOnBandChange {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^BC", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetStandbyOnBandChange {
    fn command_id(&self) -> &[u8] { b"^BC" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^BN — Band Selection (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the currently selected band.
///
/// # Reference (KPA1500 V3, §^BN)
///
/// **GET** format: `^BN;`
/// **SET/RSP** format: `^BNnn;` — 00=160m, 01=80m, 02=60m, 03=40m, 04=30m,
/// 05=20m, 06=17m, 07=15m, 08=12m, 09=10m, 10=6m.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandSelection;

/// Set the active band (00–10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandSelection {
    pub band: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBandSelection {
    fn command_id(&self) -> &[u8] { b"^BN" }
}
impl CommandWithResponse for GetBandSelection {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^BN", 2)?;
        parse_u8(d)
    }
}
impl Command for SetBandSelection {
    fn command_id(&self) -> &[u8] { b"^BN" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.band.min(10)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^BP — Bypass Relay (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the RF bypass relay is engaged.
///
/// # Reference (KPA1500 V3, §^BP)
///
/// **GET** format: `^BP;`
/// **SET/RSP** format: `^BPn;` — `n` = 0 (amplifier in path), 1 (bypassed).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBypassRelay;

/// Set RF bypass relay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBypassRelay {
    pub bypassed: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBypassRelay {
    fn command_id(&self) -> &[u8] { b"^BP" }
}
impl CommandWithResponse for GetBypassRelay {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^BP", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetBypassRelay {
    fn command_id(&self) -> &[u8] { b"^BP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.bypassed { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^BRP / ^BRX — Serial I/O Speed (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the PC RS232 port data rate.
///
/// # Reference (KPA1500 V3, §^BRP)
///
/// **GET** format: `^BRP;`
/// **SET/RSP** format: `^BRPn;` — 0=4800, 1=9600, 2=19200, 3=38400.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPcBaudRate;

/// Set the PC serial port data rate (0–3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPcBaudRate {
    pub rate: u8,
}

///
/// Query the transceiver serial port data rate.
///
/// # Reference (KPA1500 V3, §^BRX)
///
/// **GET** format: `^BRX;`
/// **SET/RSP** format: `^BRXn;` — same codes as `^BRP`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetXcvrBaudRate;

/// Set the transceiver serial port data rate (0–3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetXcvrBaudRate {
    pub rate: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPcBaudRate {
    fn command_id(&self) -> &[u8] { b"^BRP" }
}
impl CommandWithResponse for GetPcBaudRate {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^BRP", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetPcBaudRate {
    fn command_id(&self) -> &[u8] { b"^BRP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.rate.min(3)]) }
}

impl Command for GetXcvrBaudRate {
    fn command_id(&self) -> &[u8] { b"^BRX" }
}
impl CommandWithResponse for GetXcvrBaudRate {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^BRX", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetXcvrBaudRate {
    fn command_id(&self) -> &[u8] { b"^BRX" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.rate.min(3)]) }
}

// ------------------------------------------------------------------------------------------------
// ^DM — Demo Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set demo mode.
///
/// # Reference (KPA1500 V3, §^DM)
///
/// **GET** format: `^DM;`
/// **SET/RSP** format: `^DMn;` — `n` = 0 (off), 1 (on).
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
    fn command_id(&self) -> &[u8] { b"^DM" }
}
impl CommandWithResponse for GetDemoMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^DM", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetDemoMode {
    fn command_id(&self) -> &[u8] { b"^DM" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^DS — Display Select (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the active display screen.
///
/// # Reference (KPA1500 V3, §^DS)
///
/// **GET** format: `^DS;`
/// **SET/RSP** format: `^DSn;` — `n` = 0 (main), 1 (ATU), 2 (meters), 3 (fault log).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDisplaySelect;

/// Set the active display screen (0–3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDisplaySelect {
    pub screen: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetDisplaySelect {
    fn command_id(&self) -> &[u8] { b"^DS" }
}
impl CommandWithResponse for GetDisplaySelect {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^DS", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetDisplaySelect {
    fn command_id(&self) -> &[u8] { b"^DS" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.screen.min(3)]) }
}

// ------------------------------------------------------------------------------------------------
// ^FC — Fan Minimum Speed (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fan minimum control level.
///
/// # Reference (KPA1500 V3, §^FC)
///
/// **GET** format: `^FC;`
/// **SET/RSP** format: `^FCn;` — `n` = 0 (off) to 6 (high).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFanMinimumSpeed;

/// Set the fan minimum speed (0=off, 6=high).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFanMinimumSpeed {
    pub level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFanMinimumSpeed {
    fn command_id(&self) -> &[u8] { b"^FC" }
}
impl CommandWithResponse for GetFanMinimumSpeed {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^FC", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetFanMinimumSpeed {
    fn command_id(&self) -> &[u8] { b"^FC" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.level.min(6)]) }
}

// ------------------------------------------------------------------------------------------------
// ^FL — Fault Code (GET / CLEAR)
// ------------------------------------------------------------------------------------------------

///
/// Query the current fault code.
///
/// # Reference (KPA1500 V3, §^FL)
///
/// **GET** format: `^FL;`
/// **RSP** format: `^FLnn;` — `nn` = fault code; `00` = no fault.
/// Use [`ClearFault`] to send `^FLC;`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultCode;

/// Clear the current fault (`^FLC;`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearFault;

// ------------------------------------------------------------------------------------------------

impl Command for GetFaultCode {
    fn command_id(&self) -> &[u8] { b"^FL" }
}
impl CommandWithResponse for GetFaultCode {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^FL", 2)?;
        parse_u8(d)
    }
}
impl Command for ClearFault {
    fn command_id(&self) -> &[u8] { b"^FL" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'C']) }
}

// ------------------------------------------------------------------------------------------------
// ^FQ — Frequency (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the frequency the amplifier is currently using for band determination.
///
/// # Reference (KPA1500 V3, §^FQ)
///
/// **GET** format: `^FQ;`
/// **SET/RSP** format: `^FQnnnnnnnn;` — 8-digit frequency in Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFrequency;

/// Set the operating frequency in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFrequency {
    pub hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFrequency {
    fn command_id(&self) -> &[u8] { b"^FQ" }
}
impl CommandWithResponse for GetFrequency {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"^FQ", 8)?;
        parse_u32(d)
    }
}
impl Command for SetFrequency {
    fn command_id(&self) -> &[u8] { b"^FQ" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^NH — INHIBIT# Input Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the INHIBIT# input pin is enabled.
///
/// # Reference (KPA1500 V3, §^NH)
///
/// **GET** format: `^NH;`
/// **SET/RSP** format: `^NHn;` — `n` = 0 (disabled), 1 (enabled).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetInhibitInput;

/// Enable or disable the INHIBIT# input pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetInhibitInput {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetInhibitInput {
    fn command_id(&self) -> &[u8] { b"^NH" }
}
impl CommandWithResponse for GetInhibitInput {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^NH", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetInhibitInput {
    fn command_id(&self) -> &[u8] { b"^NH" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^ON — Power Status (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the KPA1500 is powered on.
///
/// # Reference (KPA1500 V3, §^ON)
///
/// **GET** format: `^ON;`
/// **RSP** format: `^ONn;` — `n` = 1 (on). No response if off.
/// **SET** format: `^ON0;` turns the KPA1500 off.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerStatus;

/// Turn the KPA1500 off (`^ON0;`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerOff;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerStatus {
    fn command_id(&self) -> &[u8] { b"^ON" }
}
impl CommandWithResponse for GetPowerStatus {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^ON", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for PowerOff {
    fn command_id(&self) -> &[u8] { b"^ON" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0']) }
}

// ------------------------------------------------------------------------------------------------
// ^OP — Output Power (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the current output power in watts.
///
/// # Reference (KPA1500 V3, §^OP)
///
/// **GET** format: `^OP;`
/// **RSP** format: `^OPnnnn;` — watts (0000–1500).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOutputPower;

// ------------------------------------------------------------------------------------------------

impl Command for GetOutputPower {
    fn command_id(&self) -> &[u8] { b"^OP" }
}
impl CommandWithResponse for GetOutputPower {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 4 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^OP", 4)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^OS — Operate/Standby Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current operate/standby mode.
///
/// # Reference (KPA1500 V3, §^OS)
///
/// **GET** format: `^OS;`
/// **SET/RSP** format: `^OSn;` — `n` = 0 (Standby), 1 (Operate).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOperateMode;

/// Set operate (true) or standby (false) mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOperateMode {
    pub operate: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetOperateMode {
    fn command_id(&self) -> &[u8] { b"^OS" }
}
impl CommandWithResponse for GetOperateMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^OS", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetOperateMode {
    fn command_id(&self) -> &[u8] { b"^OS" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.operate { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^PC — Peak Power Control (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the peak-power control limit.
///
/// # Reference (KPA1500 V3, §^PC)
///
/// **GET** format: `^PC;`
/// **SET/RSP** format: `^PCnnnn;` — output power limit in watts (0000–1500).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPeakPowerControl;

/// Set the output power limit in watts (0–1500).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPeakPowerControl {
    pub watts: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPeakPowerControl {
    fn command_id(&self) -> &[u8] { b"^PC" }
}
impl CommandWithResponse for GetPeakPowerControl {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 4 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^PC", 4)?;
        parse_u16(d)
    }
}
impl Command for SetPeakPowerControl {
    fn command_id(&self) -> &[u8] { b"^PC" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.watts.min(1500)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^PD — PTT Delay (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the PTT delay in milliseconds.
///
/// # Reference (KPA1500 V3, §^PD)
///
/// **GET** format: `^PD;`
/// **SET/RSP** format: `^PDnnn;` — milliseconds (0–500).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPttDelay;

/// Set the PTT delay in milliseconds (0–500).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPttDelay {
    pub ms: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPttDelay {
    fn command_id(&self) -> &[u8] { b"^PD" }
}
impl CommandWithResponse for GetPttDelay {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^PD", 3)?;
        parse_u16(d)
    }
}
impl Command for SetPttDelay {
    fn command_id(&self) -> &[u8] { b"^PD" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.ms.min(500)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^PF — Protection Fault Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether protection faults are enabled.
///
/// # Reference (KPA1500 V3, §^PF)
///
/// **GET** format: `^PF;`
/// **SET/RSP** format: `^PFn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetProtectionFaultEnable;

/// Enable or disable protection faults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetProtectionFaultEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetProtectionFaultEnable {
    fn command_id(&self) -> &[u8] { b"^PF" }
}
impl CommandWithResponse for GetProtectionFaultEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^PF", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetProtectionFaultEnable {
    fn command_id(&self) -> &[u8] { b"^PF" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^PJ — Power Adjustment (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the power adjustment for the current band.
///
/// # Reference (KPA1500 V3, §^PJ)
///
/// **GET** format: `^PJ;`
/// **SET/RSP** format: `^PJnnn;` — 080–120 (percent of rated output).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerAdjustment;

/// Set the power adjustment percentage (80–120).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPowerAdjustment {
    pub value: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerAdjustment {
    fn command_id(&self) -> &[u8] { b"^PJ" }
}
impl CommandWithResponse for GetPowerAdjustment {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^PJ", 3)?;
        parse_u8(d)
    }
}
impl Command for SetPowerAdjustment {
    fn command_id(&self) -> &[u8] { b"^PJ" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.value.clamp(80, 120)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^PWR — Power Status Summary (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Power status summary returned by [`GetPowerStatusSummary`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerStatusSummary {
    /// Output power in watts.
    pub power_w: u16,
    /// SWR × 10 (e.g. 15 = 1.5:1).
    pub swr_d: u16,
    /// Reflected power in watts.
    pub reflected_w: u16,
    /// Input power in watts.
    pub input_w: u16,
}

///
/// Query the complete power status summary.
///
/// # Reference (KPA1500 V3, §^PWR)
///
/// **GET** format: `^PWR;`
/// **RSP** format: `^PWRpppp ssss rrrr iiii;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerStatusSummary;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerStatusSummary {
    fn command_id(&self) -> &[u8] { b"^PWR" }
}
impl CommandWithResponse for GetPowerStatusSummary {
    type Response = PowerStatusSummary;
    fn expected_response_length(&self) -> usize { 19 }
    fn parse(&self, bytes: &[u8]) -> Result<PowerStatusSummary, RigError> {
        let d = validate_response(bytes, b"^PWR", 19)?;
        Ok(PowerStatusSummary {
            power_w: parse_u16(&d[0..4])?,
            swr_d: parse_u16(&d[5..9])?,
            reflected_w: parse_u16(&d[10..14])?,
            input_w: parse_u16(&d[15..19])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^RVM — Firmware Version (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the firmware version string.
///
/// # Reference (KPA1500 V3, §^RVM)
///
/// **GET** format: `^RVM;`
/// **RSP** format: `^RVMnn.nn;` — e.g. `^RVM03.03;`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFirmwareVersion;

// ------------------------------------------------------------------------------------------------

impl Command for GetFirmwareVersion {
    fn command_id(&self) -> &[u8] { b"^RVM" }
}
impl CommandWithResponse for GetFirmwareVersion {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 5 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"^RVM", 5)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// ^SN — Serial Number (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the KPA1500 serial number.
///
/// # Reference (KPA1500 V3, §^SN)
///
/// **GET** format: `^SN;`
/// **RSP** format: `^SNnnnnn;` — 5-digit serial number.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSerialNumber;

// ------------------------------------------------------------------------------------------------

impl Command for GetSerialNumber {
    fn command_id(&self) -> &[u8] { b"^SN" }
}
impl CommandWithResponse for GetSerialNumber {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 5 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"^SN", 5)?;
        parse_u32(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^SP — Fault Speaker (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fault speaker on/off state.
///
/// # Reference (KPA1500 V3, §^SP)
///
/// **GET** format: `^SP;`
/// **SET/RSP** format: `^SPn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultSpeaker;

/// Enable or disable the fault speaker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFaultSpeaker {
    pub on: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFaultSpeaker {
    fn command_id(&self) -> &[u8] { b"^SP" }
}
impl CommandWithResponse for GetFaultSpeaker {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^SP", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetFaultSpeaker {
    fn command_id(&self) -> &[u8] { b"^SP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^TM — PA Temperature (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the PA temperature in degrees Celsius.
///
/// # Reference (KPA1500 V3, §^TM)
///
/// **GET** format: `^TM;`
/// **RSP** format: `^TMnnn;` — 000–200 °C.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPaTemperature;

// ------------------------------------------------------------------------------------------------

impl Command for GetPaTemperature {
    fn command_id(&self) -> &[u8] { b"^TM" }
}
impl CommandWithResponse for GetPaTemperature {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^TM", 3)?;
        parse_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^TP — Tune Power (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the tune power level.
///
/// # Reference (KPA1500 V3, §^TP)
///
/// **GET** format: `^TP;`
/// **SET/RSP** format: `^TPnnnn;` — watts.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTunePower;

/// Set the tune power level in watts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTunePower {
    pub watts: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetTunePower {
    fn command_id(&self) -> &[u8] { b"^TP" }
}
impl CommandWithResponse for GetTunePower {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 4 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^TP", 4)?;
        parse_u16(d)
    }
}
impl Command for SetTunePower {
    fn command_id(&self) -> &[u8] { b"^TP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.watts).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^TR — T/R Delay (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the T/R (transmit-to-receive) delay time.
///
/// # Reference (KPA1500 V3, §^TR)
///
/// **GET** format: `^TR;`
/// **SET/RSP** format: `^TRnn;` — milliseconds (00–50).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTrDelay;

/// Set the T/R delay in milliseconds (0–50).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTrDelay {
    pub ms: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetTrDelay {
    fn command_id(&self) -> &[u8] { b"^TR" }
}
impl CommandWithResponse for GetTrDelay {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^TR", 2)?;
        parse_u8(d)
    }
}
impl Command for SetTrDelay {
    fn command_id(&self) -> &[u8] { b"^TR" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.ms.min(50)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^TV — Transceiver Supply Voltage (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the transceiver supply voltage in tenths of a volt.
///
/// # Reference (KPA1500 V3, §^TV)
///
/// **GET** format: `^TV;`
/// **RSP** format: `^TVnnn;` — tenths of volt.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransceiverVoltage;

// ------------------------------------------------------------------------------------------------

impl Command for GetTransceiverVoltage {
    fn command_id(&self) -> &[u8] { b"^TV" }
}
impl CommandWithResponse for GetTransceiverVoltage {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^TV", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^VI — PA Voltage and Current (GET only)
// ------------------------------------------------------------------------------------------------

///
/// PA voltage and current reading returned by [`GetPaVoltageCurrent`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaVoltageCurrent {
    /// PA supply voltage in tenths of a volt.
    pub voltage_dv: u16,
    /// PA drain current in tenths of an ampere.
    pub current_da: u16,
}

///
/// Query PA voltage and current.
///
/// # Reference (KPA1500 V3, §^VI)
///
/// **GET** format: `^VI;`
/// **RSP** format: `^VIvvvv iiii;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPaVoltageCurrent;

// ------------------------------------------------------------------------------------------------

impl Command for GetPaVoltageCurrent {
    fn command_id(&self) -> &[u8] { b"^VI" }
}
impl CommandWithResponse for GetPaVoltageCurrent {
    type Response = PaVoltageCurrent;
    fn expected_response_length(&self) -> usize { 9 }
    fn parse(&self, bytes: &[u8]) -> Result<PaVoltageCurrent, RigError> {
        let d = validate_response(bytes, b"^VI", 9)?;
        Ok(PaVoltageCurrent {
            voltage_dv: parse_u16(&d[0..4])?,
            current_da: parse_u16(&d[5..9])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^WS — Output Power and SWR (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Power and SWR reading returned by [`GetPowerAndSwr`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerAndSwr {
    /// Output power in watts.
    pub power_w: u16,
    /// SWR × 10. Zero when not transmitting.
    pub swr_d: u16,
}

///
/// Query the output power and SWR.
///
/// # Reference (KPA1500 V3, §^WS)
///
/// **GET** format: `^WS;`
/// **RSP** format: `^WSppppp sssss;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerAndSwr;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerAndSwr {
    fn command_id(&self) -> &[u8] { b"^WS" }
}
impl CommandWithResponse for GetPowerAndSwr {
    type Response = PowerAndSwr;
    fn expected_response_length(&self) -> usize { 11 }
    fn parse(&self, bytes: &[u8]) -> Result<PowerAndSwr, RigError> {
        let d = validate_response(bytes, b"^WS", 11)?;
        Ok(PowerAndSwr {
            power_w: parse_u16(&d[0..5])?,
            swr_d: parse_u16(&d[6..11])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^XI — Radio Interface Selection (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the radio interface type.
///
/// # Reference (KPA1500 V3, §^XI)
///
/// **GET** format: `^XI;`
/// **SET/RSP** format: `^XInno;` — `nn` = interface (00=K3, 01=BCD, 02=Analog, 03=Serial),
/// `o` = option bit.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRadioInterface;

/// Set the radio interface type and option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetRadioInterface {
    pub interface_type: u8,
    pub option: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetRadioInterface {
    fn command_id(&self) -> &[u8] { b"^XI" }
}
impl CommandWithResponse for GetRadioInterface {
    type Response = (u8, u8);
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<(u8, u8), RigError> {
        let d = validate_response(bytes, b"^XI", 3)?;
        Ok((parse_u8(&d[0..2])?, d[2] - b'0'))
    }
}
impl Command for SetRadioInterface {
    fn command_id(&self) -> &[u8] { b"^XI" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let mut v = format!("{:02}", self.interface_type.min(3)).into_bytes();
        v.push(b'0' + self.option.min(1));
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// Private parse helpers
// ------------------------------------------------------------------------------------------------

fn parse_u8(bytes: &[u8]) -> Result<u8, RigError> {
    let n = parse_u16(bytes)?;
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
    Ok(n as u16)
}

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
