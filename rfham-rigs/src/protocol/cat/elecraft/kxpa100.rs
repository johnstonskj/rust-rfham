//!
//! Serial commands for the Elecraft KXPA100 100-watt solid-state HF amplifier.
//!
//! Commands follow the **rev. 01.18** programmer's reference
//! (Elecraft KXPA100 Amplifier Command Reference, Feb 2014).
//!
//! All KXPA100 commands and responses use a leading caret (`^`), for example `^BN05;`.
//! The GET form of a command is the bare command letters with no data: `^BN;`.
//!

use crate::{
    error::RigError,
    protocol::cat::{Command, CommandWithResponse, common::validate_response},
};

// ------------------------------------------------------------------------------------------------
// ^AD — ADC Readings (GET only)
// ------------------------------------------------------------------------------------------------

///
/// ADC readings returned by [`GetAdcReadings`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdcReadings {
    /// PA drain voltage in tenths of a volt (e.g. 135 = 13.5 V).
    pub drain_voltage_dv: u16,
    /// PA drain current in tenths of an ampere (e.g. 87 = 8.7 A).
    pub drain_current_da: u16,
    /// Supply voltage in tenths of a volt.
    pub supply_voltage_dv: u16,
}

///
/// Query ADC readings (drain voltage, drain current, supply voltage).
///
/// # Reference (KXPA100 rev. 01.18, §^AD)
///
/// **GET** format: `^AD;`
/// **RSP** format: `^ADvvv iii sss;` — tenths of volt/amp for each field.
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
// ^AE — Auto-Bias Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether auto-bias is enabled.
///
/// # Reference (KXPA100 rev. 01.18, §^AE)
///
/// **GET** format: `^AE;`
/// **SET/RSP** format: `^AEn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAutoBiasEnable;

/// Enable or disable auto-bias.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAutoBiasEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAutoBiasEnable {
    fn command_id(&self) -> &[u8] { b"^AE" }
}
impl CommandWithResponse for GetAutoBiasEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AE", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAutoBiasEnable {
    fn command_id(&self) -> &[u8] { b"^AE" }
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
/// # Reference (KXPA100 rev. 01.18, §^AN)
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
// ^AT — ATU Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the internal ATU is enabled.
///
/// # Reference (KXPA100 rev. 01.18, §^AT)
///
/// **GET** format: `^AT;`
/// **SET/RSP** format: `^ATn;` — `n` = 0 (bypass), 1 (in-line).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuEnable;

/// Enable or bypass the internal ATU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAtuEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetAtuEnable {
    fn command_id(&self) -> &[u8] { b"^AT" }
}
impl CommandWithResponse for GetAtuEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^AT", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetAtuEnable {
    fn command_id(&self) -> &[u8] { b"^AT" }
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
/// # Reference (KXPA100 rev. 01.18, §^BN)
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
// ^BRP / ^BRX — Serial I/O Speed (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the PC RS232 port data rate.
///
/// # Reference (KXPA100 rev. 01.18, §^BRP)
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
/// # Reference (KXPA100 rev. 01.18, §^BRX)
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
// ^BY — Busy/PTT Status (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Busy and PTT status returned by [`GetBusyStatus`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BusyStatus {
    pub busy: bool,
    pub ptt_asserted: bool,
}

///
/// Query busy and PTT status.
///
/// # Reference (KXPA100 rev. 01.18, §^BY)
///
/// **GET** format: `^BY;`
/// **RSP** format: `^BYbp;` — `b` = 0/1 (busy), `p` = 0/1 (PTT asserted).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBusyStatus;

// ------------------------------------------------------------------------------------------------

impl Command for GetBusyStatus {
    fn command_id(&self) -> &[u8] { b"^BY" }
}
impl CommandWithResponse for GetBusyStatus {
    type Response = BusyStatus;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<BusyStatus, RigError> {
        let d = validate_response(bytes, b"^BY", 2)?;
        Ok(BusyStatus {
            busy: d[0] == b'1',
            ptt_asserted: d[1] == b'1',
        })
    }
}

// ------------------------------------------------------------------------------------------------
// ^CR — Configuration Reset (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Reset configuration to factory defaults.
///
/// # Reference (KXPA100 rev. 01.18, §^CR)
///
/// **SET** format: `^CR;` — no data, no response.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResetConfiguration;

// ------------------------------------------------------------------------------------------------

impl Command for ResetConfiguration {
    fn command_id(&self) -> &[u8] { b"^CR" }
}

// ------------------------------------------------------------------------------------------------
// ^DM — Demo Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query or set demo mode.
///
/// # Reference (KXPA100 rev. 01.18, §^DM)
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
// ^EC — Error Count (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the accumulated error count since last reset.
///
/// # Reference (KXPA100 rev. 01.18, §^EC)
///
/// **GET** format: `^EC;`
/// **RSP** format: `^ECnnnn;` — unsigned 16-bit count.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetErrorCount;

// ------------------------------------------------------------------------------------------------

impl Command for GetErrorCount {
    fn command_id(&self) -> &[u8] { b"^EC" }
}
impl CommandWithResponse for GetErrorCount {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 4 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^EC", 4)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^EM — Error Message (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the most recent error message string.
///
/// # Reference (KXPA100 rev. 01.18, §^EM)
///
/// **GET** format: `^EM;`
/// **RSP** format: `^EMssss…;` — variable-length ASCII string.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetErrorMessage;

// ------------------------------------------------------------------------------------------------

impl Command for GetErrorMessage {
    fn command_id(&self) -> &[u8] { b"^EM" }
}
impl CommandWithResponse for GetErrorMessage {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 0 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"^EM", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// ^F — Frequency (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current operating frequency reported to the amplifier.
///
/// # Reference (KXPA100 rev. 01.18, §^F)
///
/// **GET** format: `^F;`
/// **SET/RSP** format: `^Fnnnnnnnn;` — 8-digit frequency in Hz.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFrequency;

/// Inform the amplifier of the operating frequency in Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFrequency {
    pub hz: u32,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFrequency {
    fn command_id(&self) -> &[u8] { b"^F" }
}
impl CommandWithResponse for GetFrequency {
    type Response = u32;
    fn expected_response_length(&self) -> usize { 8 }
    fn parse(&self, bytes: &[u8]) -> Result<u32, RigError> {
        let d = validate_response(bytes, b"^F", 8)?;
        parse_u32(d)
    }
}
impl Command for SetFrequency {
    fn command_id(&self) -> &[u8] { b"^F" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:08}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^FE — Frequency Entry Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the frequency entry source.
///
/// # Reference (KXPA100 rev. 01.18, §^FE)
///
/// **GET** format: `^FE;`
/// **SET/RSP** format: `^FEn;` — `n` = 0 (automatic from transceiver), 1 (manual via `^F`).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFrequencyEntryMode;

/// Set the frequency entry mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFrequencyEntryMode {
    pub manual: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFrequencyEntryMode {
    fn command_id(&self) -> &[u8] { b"^FE" }
}
impl CommandWithResponse for GetFrequencyEntryMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^FE", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetFrequencyEntryMode {
    fn command_id(&self) -> &[u8] { b"^FE" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.manual { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^FL — Fault Code (GET / CLEAR)
// ------------------------------------------------------------------------------------------------

///
/// Query the current fault code.
///
/// # Reference (KXPA100 rev. 01.18, §^FL)
///
/// **GET** format: `^FL;`
/// **RSP** format: `^FLnn;` — `nn` = fault code; `00` = no fault.
/// Use [`ClearFault`] for `^FLC;`.
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
// ^FT — Fan Threshold Temperature (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the fan-on threshold temperature.
///
/// # Reference (KXPA100 rev. 01.18, §^FT)
///
/// **GET** format: `^FT;`
/// **SET/RSP** format: `^FTnnn;` — temperature in °C at which fan activates.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFanThreshold;

/// Set the fan-on threshold temperature (°C).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFanThreshold {
    pub celsius: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFanThreshold {
    fn command_id(&self) -> &[u8] { b"^FT" }
}
impl CommandWithResponse for GetFanThreshold {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^FT", 3)?;
        parse_u8(d)
    }
}
impl Command for SetFanThreshold {
    fn command_id(&self) -> &[u8] { b"^FT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.celsius).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^I — Drain Current (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the PA drain current in tenths of an ampere.
///
/// # Reference (KXPA100 rev. 01.18, §^I)
///
/// **GET** format: `^I;`
/// **RSP** format: `^Innn;` — tenths of amp (e.g. 087 = 8.7 A).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDrainCurrent;

// ------------------------------------------------------------------------------------------------

impl Command for GetDrainCurrent {
    fn command_id(&self) -> &[u8] { b"^I" }
}
impl CommandWithResponse for GetDrainCurrent {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^I", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^LR — Low-pass Relay (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the low-pass relay state.
///
/// # Reference (KXPA100 rev. 01.18, §^LR)
///
/// **GET** format: `^LR;`
/// **SET/RSP** format: `^LRn;` — `n` = 0 (bypass), 1 (in-line).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetLowPassRelay;

/// Set the low-pass relay state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetLowPassRelay {
    pub in_line: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetLowPassRelay {
    fn command_id(&self) -> &[u8] { b"^LR" }
}
impl CommandWithResponse for GetLowPassRelay {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^LR", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetLowPassRelay {
    fn command_id(&self) -> &[u8] { b"^LR" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.in_line { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^MD — Operating Mode (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the current operating mode.
///
/// # Reference (KXPA100 rev. 01.18, §^MD)
///
/// **GET** format: `^MD;`
/// **SET/RSP** format: `^MDn;` — `n` = 0 (Standby), 1 (Operate).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOperatingMode;

/// Set standby (false) or operate (true) mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOperatingMode {
    pub operate: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetOperatingMode {
    fn command_id(&self) -> &[u8] { b"^MD" }
}
impl CommandWithResponse for GetOperatingMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^MD", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetOperatingMode {
    fn command_id(&self) -> &[u8] { b"^MD" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.operate { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^MT — Meter Display (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query which meter is currently displayed.
///
/// # Reference (KXPA100 rev. 01.18, §^MT)
///
/// **GET** format: `^MT;`
/// **SET/RSP** format: `^MTn;` — `n` = 0 (power out), 1 (SWR), 2 (current), 3 (voltage).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMeterDisplay;

/// Set the meter display selection (0–3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMeterDisplay {
    pub selection: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetMeterDisplay {
    fn command_id(&self) -> &[u8] { b"^MT" }
}
impl CommandWithResponse for GetMeterDisplay {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^MT", 1)?;
        Ok(d[0] - b'0')
    }
}
impl Command for SetMeterDisplay {
    fn command_id(&self) -> &[u8] { b"^MT" }
    fn argument_bytes(&self) -> Option<Vec<u8>> { Some(vec![b'0' + self.selection.min(3)]) }
}

// ------------------------------------------------------------------------------------------------
// ^OP — Output Power (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the current output power in watts.
///
/// # Reference (KXPA100 rev. 01.18, §^OP)
///
/// **GET** format: `^OP;`
/// **RSP** format: `^OPnnn;` — watts (000–100).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOutputPower;

// ------------------------------------------------------------------------------------------------

impl Command for GetOutputPower {
    fn command_id(&self) -> &[u8] { b"^OP" }
}
impl CommandWithResponse for GetOutputPower {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^OP", 3)?;
        parse_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^PC — Peak Power Control (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the peak-power control setting.
///
/// # Reference (KXPA100 rev. 01.18, §^PC)
///
/// **GET** format: `^PC;`
/// **SET/RSP** format: `^PCnnn;` — 0–100% of rated output.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPeakPowerControl;

/// Set the peak-power control percentage (0–100).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPeakPowerControl {
    pub percent: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetPeakPowerControl {
    fn command_id(&self) -> &[u8] { b"^PC" }
}
impl CommandWithResponse for GetPeakPowerControl {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^PC", 3)?;
        parse_u8(d)
    }
}
impl Command for SetPeakPowerControl {
    fn command_id(&self) -> &[u8] { b"^PC" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.percent.min(100)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^PD — PTT Delay (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the PTT delay in milliseconds.
///
/// # Reference (KXPA100 rev. 01.18, §^PD)
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
/// # Reference (KXPA100 rev. 01.18, §^PF)
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
// ^PI — Power Input Voltage (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the DC supply voltage in tenths of a volt.
///
/// # Reference (KXPA100 rev. 01.18, §^PI)
///
/// **GET** format: `^PI;`
/// **RSP** format: `^PInnn;` — tenths of volt (e.g. 135 = 13.5 V).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerInput;

// ------------------------------------------------------------------------------------------------

impl Command for GetPowerInput {
    fn command_id(&self) -> &[u8] { b"^PI" }
}
impl CommandWithResponse for GetPowerInput {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^PI", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^PV — PA Drain Voltage (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the PA drain voltage in tenths of a volt.
///
/// # Reference (KXPA100 rev. 01.18, §^PV)
///
/// **GET** format: `^PV;`
/// **RSP** format: `^PVnnn;` — tenths of volt.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPaVoltage;

// ------------------------------------------------------------------------------------------------

impl Command for GetPaVoltage {
    fn command_id(&self) -> &[u8] { b"^PV" }
}
impl CommandWithResponse for GetPaVoltage {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^PV", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^RS — RF Sense Level (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the RF sense level (0–100, relative).
///
/// # Reference (KXPA100 rev. 01.18, §^RS)
///
/// **GET** format: `^RS;`
/// **RSP** format: `^RSnnn;` — relative RF input level.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRfSense;

// ------------------------------------------------------------------------------------------------

impl Command for GetRfSense {
    fn command_id(&self) -> &[u8] { b"^RS" }
}
impl CommandWithResponse for GetRfSense {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^RS", 3)?;
        parse_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^RV — Firmware Version (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the firmware version string.
///
/// # Reference (KXPA100 rev. 01.18, §^RV)
///
/// **GET** format: `^RV;`
/// **RSP** format: `^RVnn.nn;` — e.g. `^RV01.18;`.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFirmwareVersion;

// ------------------------------------------------------------------------------------------------

impl Command for GetFirmwareVersion {
    fn command_id(&self) -> &[u8] { b"^RV" }
}
impl CommandWithResponse for GetFirmwareVersion {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize { 5 }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"^RV", 5)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// ^SI — SWR Inhibit Threshold (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the SWR protection inhibit threshold.
///
/// # Reference (KXPA100 rev. 01.18, §^SI)
///
/// **GET** format: `^SI;`
/// **SET/RSP** format: `^SInn;` — SWR × 10 (e.g. 30 = 3.0:1).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSwrInhibitThreshold;

/// Set the SWR inhibit threshold (SWR × 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSwrInhibitThreshold {
    pub swr_d: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSwrInhibitThreshold {
    fn command_id(&self) -> &[u8] { b"^SI" }
}
impl CommandWithResponse for GetSwrInhibitThreshold {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^SI", 2)?;
        parse_u8(d)
    }
}
impl Command for SetSwrInhibitThreshold {
    fn command_id(&self) -> &[u8] { b"^SI" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.swr_d).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^SM — SWR Meter (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the current SWR reading (SWR × 10; zero when not transmitting).
///
/// # Reference (KXPA100 rev. 01.18, §^SM)
///
/// **GET** format: `^SM;`
/// **RSP** format: `^SMnn;` — SWR × 10.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSwrMeter;

// ------------------------------------------------------------------------------------------------

impl Command for GetSwrMeter {
    fn command_id(&self) -> &[u8] { b"^SM" }
}
impl CommandWithResponse for GetSwrMeter {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 2 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^SM", 2)?;
        parse_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^SN — Serial Number (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the KXPA100 serial number.
///
/// # Reference (KXPA100 rev. 01.18, §^SN)
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
// ^SV — Supply Voltage (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the DC supply voltage in tenths of a volt.
///
/// # Reference (KXPA100 rev. 01.18, §^SV)
///
/// **GET** format: `^SV;`
/// **RSP** format: `^SVnnn;` — tenths of volt.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSupplyVoltage;

// ------------------------------------------------------------------------------------------------

impl Command for GetSupplyVoltage {
    fn command_id(&self) -> &[u8] { b"^SV" }
}
impl CommandWithResponse for GetSupplyVoltage {
    type Response = u16;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"^SV", 3)?;
        parse_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// ^SW — SWR Fault Enable (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query whether SWR fault protection is enabled.
///
/// # Reference (KXPA100 rev. 01.18, §^SW)
///
/// **GET** format: `^SW;`
/// **SET/RSP** format: `^SWn;` — `n` = 0 (off), 1 (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSwrFaultEnable;

/// Enable or disable SWR fault protection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSwrFaultEnable {
    pub enabled: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSwrFaultEnable {
    fn command_id(&self) -> &[u8] { b"^SW" }
}
impl CommandWithResponse for GetSwrFaultEnable {
    type Response = bool;
    fn expected_response_length(&self) -> usize { 1 }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"^SW", 1)?;
        Ok(d[0] == b'1')
    }
}
impl Command for SetSwrFaultEnable {
    fn command_id(&self) -> &[u8] { b"^SW" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enabled { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ^TM — PA Temperature (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the PA temperature in degrees Celsius.
///
/// # Reference (KXPA100 rev. 01.18, §^TM)
///
/// **GET** format: `^TM;`
/// **RSP** format: `^TMnnn;` — 000–150 °C.
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
/// Query the tune (antenna tuner) power level.
///
/// # Reference (KXPA100 rev. 01.18, §^TP)
///
/// **GET** format: `^TP;`
/// **SET/RSP** format: `^TPnnn;` — watts (typically 005–010).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTunePower;

/// Set the tune power level in watts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTunePower {
    pub watts: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetTunePower {
    fn command_id(&self) -> &[u8] { b"^TP" }
}
impl CommandWithResponse for GetTunePower {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^TP", 3)?;
        parse_u8(d)
    }
}
impl Command for SetTunePower {
    fn command_id(&self) -> &[u8] { b"^TP" }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.watts).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ^TU — Initiate ATU Tune (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Initiate an ATU tune cycle.
///
/// # Reference (KXPA100 rev. 01.18, §^TU)
///
/// **SET** format: `^TU;` — no data; triggers a tune cycle if ATU is enabled.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitiateTune;

// ------------------------------------------------------------------------------------------------

impl Command for InitiateTune {
    fn command_id(&self) -> &[u8] { b"^TU" }
}

// ------------------------------------------------------------------------------------------------
// ^XI — Radio Interface Selection (GET/SET)
// ------------------------------------------------------------------------------------------------

///
/// Query the radio interface type.
///
/// # Reference (KXPA100 rev. 01.18, §^XI)
///
/// **GET** format: `^XI;`
/// **SET/RSP** format: `^XInno;` — `nn` = interface type (00=K3, 01=BCD, 02=Analog, 03=Serial),
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
// ^XP — Transceiver Drive Power (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query the transceiver drive power detected by the amplifier (0–100, relative).
///
/// # Reference (KXPA100 rev. 01.18, §^XP)
///
/// **GET** format: `^XP;`
/// **RSP** format: `^XPnnn;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransceiverPowerLevel;

// ------------------------------------------------------------------------------------------------

impl Command for GetTransceiverPowerLevel {
    fn command_id(&self) -> &[u8] { b"^XP" }
}
impl CommandWithResponse for GetTransceiverPowerLevel {
    type Response = u8;
    fn expected_response_length(&self) -> usize { 3 }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"^XP", 3)?;
        parse_u8(d)
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
