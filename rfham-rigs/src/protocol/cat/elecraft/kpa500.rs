//!
//! Serial commands for the Elecraft KPA500 solid-state HF amplifier.
//!
//! Commands follow the **A2** programmer's reference (KPA500 Programmers Ref, rev. A2).
//!
//! All KPA500 commands and responses use a leading caret (`^`), for example `^BN05;`.
//! The GET form of a command is the bare command letters with no data: `^BN;`.
//!

use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// ^AL — ALC Threshold (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the ALC threshold for the current band.
///
/// # Reference (KPA500 rev. A2, §^AL)
///
/// **GET** format: `^AL;`
/// **SET/RSP** format: `^ALnnn;` where `nnn` = 000–210.
/// The ALC value is saved per band; the response reflects the current band.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAlcThreshold;

///
/// Set the ALC threshold for the current band (0–210).
///
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
// ^AR — Attenuator Fault Release Time (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the attenuator fault release time.
///
/// # Reference (KPA500 rev. A2, §^AR)
///
/// **GET** format: `^AR;`
/// **SET/RSP** format: `^ARnnnn;` where `nnnn` = 1400–5000 milliseconds.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAttenuatorReleaseTime;

///
/// Set the attenuator fault release time in milliseconds (1400–5000).
///
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
// ^BC — STBY on Band Change (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the amplifier returns to standby on band change.
///
/// # Reference (KPA500 rev. A2, §^BC)
///
/// **GET** format: `^BC;`
/// **SET/RSP** format: `^BCn;` — `n` = 1 (stay STBY after band change), 0 (return to prior state).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetStandbyOnBandChange;

///
/// Set standby-on-band-change behaviour.
///
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
/// # Reference (KPA500 rev. A2, §^BN)
///
/// **GET** format: `^BN;`
/// **SET/RSP** format: `^BNnn;` — band codes: 00=160m, 01=80m, 02=60m, 03=40m, 04=30m,
/// 05=20m, 06=17m, 07=15m, 08=12m, 09=10m, 10=6m.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandSelection;

///
/// Set the active band (00–10).
///
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
// ^BRP / ^BRX — PC and Transceiver Serial I/O Speed (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the PC RS232 port data rate.
///
/// # Reference (KPA500 rev. A2, §^BRP)
///
/// **GET** format: `^BRP;`
/// **SET/RSP** format: `^BRPn;` — `n` = 0 (4800), 1 (9600), 2 (19200), 3 (38400).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPcBaudRate;

/// Set the PC RS232 port data rate (0–3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPcBaudRate {
    pub rate: u8,
}

///
/// Query the transceiver RS232 port data rate.
///
/// # Reference (KPA500 rev. A2, §^BRX)
///
/// **GET** format: `^BRX;`
/// **SET/RSP** format: `^BRXn;` — same codes as `^BRP`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetXcvrBaudRate;

/// Set the transceiver RS232 port data rate (0–3).
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
// ^DMO — Demo Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set the demo mode flag.
///
/// # Reference (KPA500 rev. A2, §^DMO)
///
/// **GET** format: `^DMO;`
/// **SET/RSP** format: `^DMOn;` — `n` = 0 (off), 1 (on).
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
    fn command_id(&self) -> &[u8] { b"^DMO" }
}
impl CommandWithResponse for GetDemoMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^DMO", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetDemoMode {
    fn command_id(&self) -> &[u8] { b"^DMO" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^FC — Fan Minimum Speed (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fan minimum control level.
///
/// # Reference (KPA500 rev. A2, §^FC)
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
/// # Reference (KPA500 rev. A2, §^FL)
///
/// **GET** format: `^FL;`
/// **RSP** format: `^FLnn;` — `nn` = fault identifier; `00` = no active fault.
/// **CLEAR** format: `^FLC;` — use [`ClearFault`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFaultCode;

///
/// Clear the current fault (`^FLC;`).
///
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
// ^NH — INHIBIT# Input Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the INHIBIT# input pin is enabled.
///
/// # Reference (KPA500 rev. A2, §^NH)
///
/// **GET** format: `^NH;`
/// **SET/RSP** format: `^NHn;` — `n` = 1 (enable INHIBIT# pin), 0 (disable).
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
/// Query whether the KPA500 is powered on.
///
/// # Reference (KPA500 rev. A2, §^ON)
///
/// **GET** format: `^ON;`
/// **RSP** format: `^ONn;` — `n` = 1 (on). No response if off.
/// **SET** format: `^ON0;` turns the KPA500 off. Power-on requires the boot-loader `P` command.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerStatus;

/// Turn the KPA500 off (`^ON0;`).
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
// ^OS — Operate/Standby Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current operate/standby mode.
///
/// # Reference (KPA500 rev. A2, §^OS)
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
// ^PJ — Power Adjustment (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the power adjustment setting for the current band.
///
/// # Reference (KPA500 rev. A2, §^PJ)
///
/// **GET** format: `^PJ;`
/// **SET/RSP** format: `^PJnnn;` — `nnn` = 080–120 (percent of rated output).
/// Saved per band; response reflects the current band.
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
// ^RVM — Firmware Version (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the firmware version string.
///
/// # Reference (KPA500 rev. A2, §^RVM)
///
/// **GET** format: `^RVM;`
/// **RSP** format: `^RVMnn.nn;` — e.g. `^RVM01.13;`.
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
/// Query the KPA500 serial number.
///
/// # Reference (KPA500 rev. A2, §^SN)
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
/// # Reference (KPA500 rev. A2, §^SP)
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
/// # Reference (KPA500 rev. A2, §^TM)
///
/// **GET** format: `^TM;`
/// **RSP** format: `^TMnnn;` — `nnn` = 000–150 °C.
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
// ^TR — T/R Delay (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the T/R (transmit-to-receive) delay time.
///
/// # Reference (KPA500 rev. A2, §^TR)
///
/// **GET** format: `^TR;`
/// **SET/RSP** format: `^TRnn;` — `nn` = 00–50 milliseconds.
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
// ^VI — PA Voltage and Current (GET only)
// ------------------------------------------------------------------------------------------------

///
/// PA voltage and current reading returned by [`GetPaVoltageCurrent`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PaVoltageCurrent {
    /// PA voltage in tenths of a volt (e.g. 485 = 48.5 V).
    pub voltage_dv: u16,
    /// PA current in tenths of an ampere (e.g. 123 = 12.3 A).
    pub current_da: u16,
}

///
/// Query PA voltage and current.
///
/// # Reference (KPA500 rev. A2, §^VI)
///
/// **GET** format: `^VI;`
/// **RSP** format: `^VIvvv iii;` — `vvv` = voltage (tenths of volt), `iii` = current (tenths of amp).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPaVoltageCurrent;

// ------------------------------------------------------------------------------------------------

impl Command for GetPaVoltageCurrent {
    fn command_id(&self) -> &[u8] { b"^VI" }
}
impl CommandWithResponse for GetPaVoltageCurrent {
    type Response = PaVoltageCurrent;
    fn expected_response_length(&self) -> usize { 7 }
    fn parse(&self, bytes: &[u8]) -> Result<PaVoltageCurrent, RigError> {
        let d = validate_response(bytes, b"^VI", 7)?;
        Ok(PaVoltageCurrent {
            voltage_dv: parse_u16(&d[0..3])?,
            current_da: parse_u16(&d[4..7])?,
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
    /// Output power in watts (0–999).
    pub power_w: u16,
    /// SWR in tenths (e.g. 15 = 1.5:1). Zero when not transmitting.
    pub swr_d: u16,
}

///
/// Query the output power and SWR.
///
/// # Reference (KPA500 rev. A2, §^WS)
///
/// **GET** format: `^WS;`
/// **RSP** format: `^WSppp sss;` — `ppp` = output power (W), `sss` = SWR × 10.
/// Returns `000 000` when not transmitting.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerAndSwr;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerAndSwr {
    fn command_id(&self) -> &[u8] { b"^WS" }
}
impl CommandWithResponse for GetPowerAndSwr {
    type Response = PowerAndSwr;
    fn expected_response_length(&self) -> usize { 7 }
    fn parse(&self, bytes: &[u8]) -> Result<PowerAndSwr, RigError> {
        let d = validate_response(bytes, b"^WS", 7)?;
        Ok(PowerAndSwr {
            power_w: parse_u16(&d[0..3])?,
            swr_d: parse_u16(&d[4..7])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^XI — Radio Interface Selection (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the radio interface type.
///
/// # Reference (KPA500 rev. A2, §^XI)
///
/// **GET** format: `^XI;`
/// **SET/RSP** format: `^XInno;` — `nn` = interface type, `o` = option bit:
/// - `00` = K3 (always returns `o` = 1 in firmware V1.04+)
/// - `01` = BCD
/// - `02` = Analog (Icom voltage levels)
/// - `03` = Elecraft/Kenwood serial I/O (`o` = 1 enables radio frequency polling)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRadioInterface;

/// Set radio interface type and option byte.
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
