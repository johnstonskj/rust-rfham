//!
//! CAT commands for the Elecraft K3, K3S, KX2, and KX3 transceivers.
//!
//! Commands follow the **G5** programmer's reference
//! (K3S&K3&KX3&KX2 Pgmrs Ref, rev. G5, Feb 2019).
//!
//! Commands marked `*` apply to K3/K3S only.
//! Commands marked `**` apply to KX3/KX2 only.
//! The `$` suffix targets VFO B / sub receiver where applicable.
//!

use crate::{
    AnalogVoiceMode, DataMode, Frequency, MorseMode, OperatingMode,
    error::RigError,
    protocol::cat::{
        Command, CommandWithResponse, Vfo,
        common::validate_response,
        elecraft::meta::{parse_decimal_u8, parse_decimal_u16, parse_signed_i16},
    },
};
use rfham_itu::allocations::AllocationBand;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// AI — Auto-Information Mode
// ------------------------------------------------------------------------------------------------

///
/// Query auto-information mode. See [`SetAutoInfoMode`] for mode codes.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAutoInfoMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoInfoMode {
    Off = b'0',
    K2 = b'1',
    K3 = b'2',
    K3Extended = b'3',
}

///
/// Set auto-information mode.
///
/// RSP format: `AIm;` where `m` is:
/// - `0` = AI off (manual query only)
/// - `1` = AI on for K2 responses (basic unsolicited RSP)
/// - `2` = AI on for K3 responses
/// - `3` = AI on for K3 responses in extended mode
///
/// In AI mode the transceiver spontaneously sends RSPs whenever state changes.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAutoInfoMode {
    pub mode: AutoInfoMode,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetAutoInfoMode, b"AI");

impl CommandWithResponse for GetAutoInfoMode {
    type Response = AutoInfoMode;
    fn expected_response_length(&self) -> usize {
        1
    }

    fn parse(&self, bytes: &[u8]) -> Result<AutoInfoMode, RigError> {
        let response = validate_response(bytes, b"AI", 1)?;
        match response[0] {
            b'0' => Ok(AutoInfoMode::Off),
            b'1' => Ok(AutoInfoMode::K2),
            b'2' => Ok(AutoInfoMode::K3),
            b'3' => Ok(AutoInfoMode::K3Extended),
            _ => {
                error!(
                    "Invalid AI response byte {:02X?} (expecting 0,1,2,3)",
                    response[0]
                );
                Err(RigError::InvalidResponseData {
                    data: bytes.to_vec(),
                })
            }
        }
    }
}

impl Command for SetAutoInfoMode {
    fn command_id(&self) -> &[u8] {
        b"AI"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![self.mode as u8])
    }
}

// ------------------------------------------------------------------------------------------------
// AK — ATU Network Values (KX3/KX2 only **)
// ------------------------------------------------------------------------------------------------

///
/// Read ATU (antenna tuner) network values (KX3/KX2 GET only).
///
/// RSP format: `AKiiccclllaaa;` where `ii`=input switch, `ccc`=capacitor,
/// `lll`=inductor, `aaa`=antenna (all decimal).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAtuNetworkValues;

///
/// ATU network values from the AK response (KX3/KX2 GET only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtuNetworkValues {
    /// Input switch position (00–16).
    pub input_switch: u8,
    /// Capacitor DAC value (000–255).
    pub capacitor: u8,
    /// Inductor DAC value (000–255).
    pub inductor: u8,
    /// Antenna switch position (000–255).
    pub antenna: u8,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetAtuNetworkValues, b"AK");

impl CommandWithResponse for GetAtuNetworkValues {
    type Response = AtuNetworkValues;
    fn expected_response_length(&self) -> usize {
        11
    }
    fn parse(&self, bytes: &[u8]) -> Result<AtuNetworkValues, RigError> {
        let d = validate_response(bytes, b"AK", 11)?;
        Ok(AtuNetworkValues {
            input_switch: parse_decimal_u8(&d[0..2])?,
            capacitor: parse_decimal_u8(&d[2..5])?,
            inductor: parse_decimal_u8(&d[5..8])?,
            antenna: parse_decimal_u8(&d[8..11])?,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// AP — Audio Peaking Filter
// ------------------------------------------------------------------------------------------------

///
/// Query CW Audio Peaking Filter (APF) on/off state.
///
/// RSP format: `APn;` — `n` = `0` (off) or `1` (on). CW mode only.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAudioPeakingFilter;

///
/// Set CW Audio Peaking Filter on/off (CW mode only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAudioPeakingFilter {
    pub on: bool,
}

impl_command!(GetAudioPeakingFilter, b"AP");

// ------------------------------------------------------------------------------------------------

impl CommandWithResponse for GetAudioPeakingFilter {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AP", 1)?;
        Ok(d[0] == b'1')
    }
}

impl Command for SetAudioPeakingFilter {
    fn command_id(&self) -> &[u8] {
        b"AP"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// AR — RX Antenna (K3/K3S only *)
// ------------------------------------------------------------------------------------------------

///
/// Query the RX-only antenna state (K3/K3S only).
///
/// RSP format: `ARn;` — `n` = `0` (use TX antenna) or `1` (use RX-only antenna).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRxAntenna;

///
/// Set RX-only antenna. `rx_only = true` selects the RX-only antenna (K3/K3S only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetRxAntenna {
    rx_only: bool,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetRxAntenna, b"AR");

impl CommandWithResponse for GetRxAntenna {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"AR", 1)?;
        Ok(d[0] == b'1')
    }
}

impl Command for SetRxAntenna {
    fn command_id(&self) -> &[u8] {
        b"AR"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.rx_only { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// BG — Bargraph Read (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read the TX bargraph meter value (GET only).
///
/// RSP format: `BGnn;` — `nn` = 00–21 (approximately 0–100% of full scale in 5% steps).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBargraph;

// ------------------------------------------------------------------------------------------------

impl_command!(GetBargraph, b"BG");

impl CommandWithResponse for GetBargraph {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"BG", 2)?;
        parse_decimal_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// BN — Band Number
// ------------------------------------------------------------------------------------------------

/// Query the current band number.
///
/// RSP format: `BNnn;` (VFO A) or `BN$nn;` (VFO B).
///
/// Band codes: `00`=160 m, `01`=80 m, `02`=60 m, `03`=40 m, `04`=30 m, `05`=20 m,
/// `06`=17 m, `07`=15 m, `08`=12 m, `09`=10 m, `10`=6 m, `11`=2 m (XVTR),
/// `12`=222 MHz, `13`=432 MHz, `14`=1.25 cm, `18`=General.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBandNumber {
    vfo: Vfo,
}

///
/// Set band number (see [`GetBandNumber`] for codes).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBandNumber {
    vfo: Vfo,
    band: AllocationBand,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetBandNumber {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"BN",
            Vfo::B => b"BN$",
            _ => panic!("GetBandNumber: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetBandNumber {
    type Response = AllocationBand;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<AllocationBand, RigError> {
        let response = validate_response(bytes, self.command_id(), 2)?;
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
            _ => {
                error!(
                    "Invalid BN response data {:02X?} (expecting 00–10)",
                    response
                );
                Err(RigError::InvalidResponseData {
                    data: response.to_vec(),
                })
            }
        }
    }
}

impl Command for SetBandNumber {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"BN",
            Vfo::B => b"BN$",
            _ => panic!("SetBandNumber: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.band).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// BR — Baud Rate (K3/K3S only *, SET only)
// ------------------------------------------------------------------------------------------------

///
/// Set RS-232 baud rate (K3/K3S only, SET only).
///
/// RSP format: `BRn;` — `n` = 0 (4800), 1 (9600), 2 (19200), 3 (38400), 4 (57600),
/// 5 (115200). The new rate takes effect after the command is acknowledged.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetBaudRate {
    // TODO: should this be an enum in transport instead of a u8? (0–5)?
    rate: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetBaudRate {
    fn command_id(&self) -> &[u8] {
        b"BR"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.rate.min(5)])
    }
}

// ------------------------------------------------------------------------------------------------
// BW — Filter Bandwidth
// ------------------------------------------------------------------------------------------------

///
/// Query active filter bandwidth in units of 10 Hz.
///
/// RSP format: `BWnnnn;` (VFO A) or `BW$nnnn;` (VFO B / sub). `nnnn` = 0000–9999,
/// e.g. `0500` = 5000 Hz. Prefer this command over the legacy FW command.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFilterBandwidth {
    vfo: Vfo,
}

///
/// Set filter bandwidth in units of 10 Hz (e.g. `500` = 5000 Hz).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetFilterBandwidth {
    vfo: Vfo,
    bandwidth_10hz: u16,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetFilterBandwidth {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"BW",
            Vfo::B => b"BW$",
            _ => panic!("GetFilterBandwidth: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetFilterBandwidth {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, self.command_id(), 4)?;
        parse_decimal_u16(d)
    }
}

impl Command for SetFilterBandwidth {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"BW",
            Vfo::B => b"BW$",
            _ => panic!("SetFilterBandwidth: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.bandwidth_10hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// CP — Speech Compression
// ------------------------------------------------------------------------------------------------

///
/// Query speech compression level (0–40, 0=off).
///
/// RSP format: `CPnn;` — `nn` = 00–40 (percent).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSpeechCompression;

///
/// Set speech compression level (0–40, 0=off).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSpeechCompression {
    level: u8,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetSpeechCompression, b"CP");

impl CommandWithResponse for GetSpeechCompression {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"CP", 2)?;
        parse_decimal_u8(d)
    }
}

impl Command for SetSpeechCompression {
    fn command_id(&self) -> &[u8] {
        b"CP"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.level.min(40)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// CW — CW Sidetone Pitch (GET only on K3/K3S)
// ------------------------------------------------------------------------------------------------

///
/// Query CW sidetone pitch in Hz (GET only on K3/K3S; GET/SET on K4).
///
/// RSP format: `CWnnn;` — `nnn` = 300–800 Hz.
/// The pitch also determines the RIT/XIT operating offset for zero-beat CW tuning.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCwSidetonePitch;

// ------------------------------------------------------------------------------------------------

impl_command!(GetCwSidetonePitch, b"CW");

impl CommandWithResponse for GetCwSidetonePitch {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"CW", 3)?;
        parse_decimal_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// DB — VFO B Display Text
// ------------------------------------------------------------------------------------------------

///
/// Query VFO B display text (8 ASCII characters).
///
/// RSP format: `DBssssssss;` — 8-char ASCII display contents.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetVfoBDisplayText;

///
/// Set VFO B display text (8 ASCII characters, space-padded).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetVfoBDisplayText {
    text: [u8; 8],
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetVfoBDisplayText, b"DB");

impl CommandWithResponse for GetVfoBDisplayText {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        8
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"DB", 8)?;
        Ok(d.to_vec())
    }
}

impl Command for SetVfoBDisplayText {
    fn command_id(&self) -> &[u8] {
        b"DB"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(self.text.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// DE — Command Processing Delay (K3/K3S only *, SET only)
// ------------------------------------------------------------------------------------------------

///
/// Set command processing delay in units of 5 ms (K3/K3S only, SET only).
///
/// RSP format: `DEnn;` — `nn` = 00–99. `00` disables the delay.
/// Slows down command processing for software that cannot keep up with responses.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetCommandDelay {
    delay_5ms: u8,
}

impl Command for SetCommandDelay {
    fn command_id(&self) -> &[u8] {
        b"DE"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.delay_5ms).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// DN / DNB — VFO Move Down (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Move VFO A down by one tuning step (SET only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VfoADown;

/// Move VFO B down by one tuning step (SET only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VfoBDown;

// ------------------------------------------------------------------------------------------------

impl_command!(VfoADown, b"DN");

impl_command!(VfoBDown, b"DNB");

// ------------------------------------------------------------------------------------------------
// DS — VFO A Display Read (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read VFO A display text and icons (GET only).
///
/// RSP format: `DSls...s;` — `l` = line (`1`=VFO A, `2`=VFO B sub), `s...s` = 16 ASCII chars.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetVfoADisplayText {
    line: u8,
}

impl Command for GetVfoADisplayText {
    fn command_id(&self) -> &[u8] {
        b"DS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.line.min(2)])
    }
}

impl CommandWithResponse for GetVfoADisplayText {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        17
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"DS", 17)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// DT — Data Sub-Mode
// ------------------------------------------------------------------------------------------------

///
/// Query DATA mode sub-type.
///
/// RSP format: `DTn;` — `n` = 0 (DATA A/AFSK audio), 1 (AFSK A), 2 (FSK D/RTTY),
/// 3 (PSK D). Only meaningful when operating in DATA mode (MD6 or MD9).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDataSubMode;

///
/// Set DATA sub-mode (0=DATA A, 1=AFSK A, 2=FSK D, 3=PSK D).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDataSubMode {
    sub_mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetDataSubMode, b"DT");

impl CommandWithResponse for GetDataSubMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"DT", 1)?;
        Ok(d[0] - b'0')
    }
}

impl Command for SetDataSubMode {
    fn command_id(&self) -> &[u8] {
        b"DT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.sub_mode.min(3)])
    }
}

// ------------------------------------------------------------------------------------------------
// DV — Diversity Mode (K3 only *)
// ------------------------------------------------------------------------------------------------

///
/// Query Diversity receive mode (K3 only, requires KRX3A).
///
/// RSP format: `DVn;` — `n` = `0` (off) or `1` (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetDiversityMode;

///
/// Set Diversity mode on/off (K3 only, requires KRX3A sub-receiver).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetDiversityMode {
    on: bool,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetDiversityMode, b"DV");

impl CommandWithResponse for GetDiversityMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"DV", 1)?;
        Ok(d[0] == b'1')
    }
}

impl Command for SetDiversityMode {
    fn command_id(&self) -> &[u8] {
        b"DV"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// EL — Error Logging (KX3/KX2 only **, SET only)
// ------------------------------------------------------------------------------------------------

///
/// Enable or disable error logging (KX3/KX2 only, SET only).
///
/// RSP format: `ELn;` — `n` = `0` (disable) or `1` (enable).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetErrorLogging {
    enable: bool,
}

///
/// Set ESSB mode on/off (K3/K3S only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetEssbMode {
    on: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetErrorLogging {
    fn command_id(&self) -> &[u8] {
        b"EL"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.enable { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// ES — ESSB Mode (K3/K3S only *)
// ------------------------------------------------------------------------------------------------

///
/// Query Enhanced SSB (ESSB) transmit mode (K3/K3S only).
///
/// RSP format: `ESn;` — `n` = `0` (off) or `1` (on).
/// ESSB enables a wider TX bandwidth (up to 4 kHz) for improved SSB audio quality.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetEssbMode;

// ------------------------------------------------------------------------------------------------

impl_command!(GetEssbMode, b"ES");

impl CommandWithResponse for GetEssbMode {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"ES", 1)?;
        Ok(d[0] == b'1')
    }
}

impl Command for SetEssbMode {
    fn command_id(&self) -> &[u8] {
        b"ES"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// FI — IF Center Frequency (K3 only *, GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read IF center frequency in Hz (K3 only, GET only).
///
/// RSP format: `FInnnn;` — `nnnn` = 0000–9999 Hz (the center point of IF shift).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetIfCenterFrequency;

impl_command!(GetIfCenterFrequency, b"FI");

impl CommandWithResponse for GetIfCenterFrequency {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"FI", 4)?;
        parse_decimal_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// FR — Receive VFO (K2 compatibility)
// ------------------------------------------------------------------------------------------------

/// Query active receive VFO (K2 compatibility command).
///
/// RSP format: `FRn;` — `n` = `0` (VFO A) or `1` (VFO B).
/// Query active receive VFO (K2 compatibility command).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetReceiveVfo;

impl_command!(GetReceiveVfo, b"FR");

impl CommandWithResponse for GetReceiveVfo {
    type Response = Vfo;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vfo, RigError> {
        let d = validate_response(bytes, b"FR", 1)?;
        match d[0] {
            b'0' => Ok(Vfo::A),
            b'1' => Ok(Vfo::B),
            _ => Err(RigError::InvalidResponseData { data: d.to_vec() }),
        }
    }
}

/// Set receive VFO (K2 compatibility).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetReceiveVfo {
    vfo: Vfo,
}

impl Command for SetReceiveVfo {
    fn command_id(&self) -> &[u8] {
        b"FR"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![match self.vfo {
            Vfo::A => b'0',
            _ => b'1',
        }])
    }
}

// ------------------------------------------------------------------------------------------------
// FT — TX VFO / Split Enable
// ------------------------------------------------------------------------------------------------

/// Query the transmit VFO selection (split mode state).
///
/// RSP format: `FTn;` — `n` = `0` (VFO A transmits; no split) or
/// `1` (VFO B transmits; split on).
/// Query the transmit VFO selection (split mode state).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransmitVfo;

impl_command!(GetTransmitVfo, b"FT");

impl CommandWithResponse for GetTransmitVfo {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"FT", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set transmit VFO. `split = true` → VFO B transmits (split mode on).
/// Set transmit VFO. `split = true` → VFO B transmits (split mode on).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTransmitVfo {
    split: bool,
}

impl Command for SetTransmitVfo {
    fn command_id(&self) -> &[u8] {
        b"FT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.split { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// FW — Filter Bandwidth / Number (K2 legacy)
// ------------------------------------------------------------------------------------------------

/// Query filter bandwidth in Hz via the K2 legacy command (prefer BW on K3/KX).
///
/// RSP format: `FWnnnn;` (VFO A) or `FW$nnnn;` (VFO B). `nnnn` = Hz (0000–9999).
/// Query filter bandwidth in Hz via the K2 legacy command (prefer BW on K3/KX).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetLegacyFilterBandwidth {
    vfo: Vfo,
}

impl Command for GetLegacyFilterBandwidth {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FW",
            Vfo::B => b"FW$",
            _ => panic!("GetLegacyFilterBandwidth: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetLegacyFilterBandwidth {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, self.command_id(), 4)?;
        parse_decimal_u16(d)
    }
}

/// Set filter bandwidth in Hz (K2 legacy; prefer BW on K3/KX).
/// Set filter bandwidth in Hz (K2 legacy; prefer BW on K3/KX).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetLegacyFilterBandwidth {
    vfo: Vfo,
    bandwidth_hz: u16,
}

impl Command for SetLegacyFilterBandwidth {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FW",
            Vfo::B => b"FW$",
            _ => panic!("SetLegacyFilterBandwidth: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.bandwidth_hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// GT — AGC Time Constant
// ------------------------------------------------------------------------------------------------

/// Query AGC time constant.
///
/// RSP format: `GTnn;` — `nn` = 00 (off), 01 (fast), 02 (slow), 03 (auto).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetAgcTimeConstant;

impl_command!(GetAgcTimeConstant, b"GT");

impl CommandWithResponse for GetAgcTimeConstant {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"GT", 2)?;
        parse_decimal_u8(d)
    }
}

/// Set AGC time constant (0=off, 1=fast, 2=slow, 3=auto).
/// Set AGC time constant (0=off, 1=fast, 2=slow, 3=auto).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetAgcTimeConstant {
    mode: u8,
}

impl Command for SetAgcTimeConstant {
    fn command_id(&self) -> &[u8] {
        b"GT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.mode.min(3)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// OM — Get Installed Options
// ------------------------------------------------------------------------------------------------

///
/// Query installed option modules.
///
/// # Reference (G5 §OM; D12 §OM)
///
/// **GET** only. The 13-byte value differs by radio model; this command dispatches to
/// the appropriate variant via [`InstalledOptions`].
///
/// - **K3/K3S RSP**: `OM APXSDFfLVR--;`
/// - **K4 RSP**: `OM APXSHML14---;`
/// - **KX3 RSP**: `OM APF---TBXI02;`
/// - **KX2 RSP**: `OM APF---TBXI01;`
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetInstalledOptions;

impl_command!(GetInstalledOptions, b"OM");
impl_command_with_response!(GetInstalledOptions => try_from 13 InstalledOptions);

/// Installed hardware options — dispatches to the model-specific variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstalledOptions {
    K3(K3InstalledOptions),
    K4(K4InstalledOptions),
    KX(KXInstalledOptions),
}

// ------------------------------------------------------------------------------------------------

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

///
/// K3/K3S installed option modules.
///
/// # Reference (G5 §OM)
///
/// Option string after leading space: `APXSDFfLVR--`
///
/// | Pos | Letter | Module |
/// |-----|--------|--------|
/// | 0 | `A` | ATU (KAT3A) |
/// | 1 | `P` | PA (KPA3A) |
/// | 2 | `X` | XVTR / RX I/O (KXV3/KXV3A/KXV3B) |
/// | 3 | `S` | Sub Receiver (KRX3A) |
/// | 4 | `D` | DVR (KDVR3) |
/// | 5 | `F` | Band-Pass Filter main (KBPF3A) |
/// | 6 | `f` | Band-Pass Filter sub (KBPF3A) |
/// | 7 | `L` | Low-Noise Amp on current band (KXV3B, preamp 2) |
/// | 8 | `V` | KSYN3A synthesizer (extends VFO to 100 kHz) |
/// | 9 | `R` | K3S RF board (preferred way to distinguish K3 vs K3S) |
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K3InstalledOptions {
    pub has_kat3a_atu: bool,
    pub has_kpa3a_pa: bool,
    pub has_kxv3_xvtr: bool,
    pub has_krx3a_sub_receiver: bool,
    pub has_kdvr3_dvr: bool,
    pub has_kbpf3a_main_bpf: bool,
    pub has_kbpf3a_sub_bpf: bool,
    pub has_kxv3b_low_noise_amp: bool,
    pub has_ksyn3a: bool,
    pub has_k3s: bool,
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
            has_kat3a_atu: parse_option!(has_kat3a_atu          => value[0] == b'A'),
            has_kpa3a_pa: parse_option!(has_kpa3a_pa           => value[1] == b'P'),
            has_kxv3_xvtr: parse_option!(has_kxv3_xvtr          => value[2] == b'X'),
            has_krx3a_sub_receiver: parse_option!(has_krx3a_sub_receiver => value[3] == b'S'),
            has_kdvr3_dvr: parse_option!(has_kdvr3_dvr          => value[4] == b'D'),
            has_kbpf3a_main_bpf: parse_option!(has_kbpf3a_main_bpf    => value[5] == b'F'),
            has_kbpf3a_sub_bpf: parse_option!(has_kbpf3a_sub_bpf     => value[6] == b'f'),
            has_kxv3b_low_noise_amp: parse_option!(has_kxv3b_low_noise_amp=> value[7] == b'L'),
            has_ksyn3a: parse_option!(has_ksyn3a             => value[8] == b'V'),
            has_k3s: parse_option!(has_k3s                => value[9] == b'R'),
        })
    }
}

///
/// K4 installed option modules.
///
/// # Reference (D12 §OM)
///
/// Option string after leading space: `APXSHML14---`
///
/// | Pos | Letter | Module |
/// |-----|--------|--------|
/// | 0 | `A` | ATU (KAT4) |
/// | 1 | `P` | PA (KPA4) |
/// | 2 | `X` | XVTR |
/// | 3 | `S` | Sub RX (KRX4 + 2nd KDDC4; standard in K4D) |
/// | 4 | `H` | HDR Module (KHDR4 + KDDC4-2; standard in K4HD) |
/// | 5 | `M` | K40 ("Mini") |
/// | 6 | `L` | Linear amp detected (KPA500 or KPA1500) |
/// | 7 | `1` | KPA1500 specifically |
/// | 8 | `4` | Radio identified as K4 (S+4 = K4D; S+H+4 = K4HD) |
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K4InstalledOptions {
    pub has_kat4_atu: bool,
    pub has_kpa4_pa: bool,
    pub has_xvtr: bool,
    pub has_krx4_sub_receiver: bool,
    pub has_khdr4_hdr: bool,
    pub has_k40_mini: bool,
    pub has_kpa_linear_pa: bool,
    pub has_kpa1500_pa: bool,
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
            has_kat4_atu: parse_option!(has_kat4_atu          => value[0] == b'A'),
            has_kpa4_pa: parse_option!(has_kpa4_pa           => value[1] == b'P'),
            has_xvtr: parse_option!(has_xvtr              => value[2] == b'X'),
            has_krx4_sub_receiver: parse_option!(has_krx4_sub_receiver => value[3] == b'S'),
            has_khdr4_hdr: parse_option!(has_khdr4_hdr         => value[4] == b'H'),
            has_k40_mini: parse_option!(has_k40_mini          => value[5] == b'M'),
            has_kpa_linear_pa: parse_option!(has_kpa_linear_pa     => value[6] == b'L'),
            has_kpa1500_pa: parse_option!(has_kpa1500_pa        => value[7] == b'1'),
        })
    }
}

///
/// KX3/KX2 installed option modules.
///
/// # Reference (G5 §OM)
///
/// Option string after leading space: `APF---TBXI0n`
///
/// | Pos | Letter | Module |
/// |-----|--------|--------|
/// | 0 | `A` | ATU (KXAT3 or KXAT2) |
/// | 1 | `P` | External 100-W PA (KXPA100) |
/// | 2 | `F` | Roofing filter (KXFL3) |
/// | 6 | `T` | External 100-W ATU (KXAT100, a KXPA100 internal option) |
/// | 7 | `B` | NiMH battery-charger / real-time clock (KXBC3) |
/// | 8 | `X` | KX3-2M or KX3-4M transverter module |
/// | 9 | `I` | KXIO2 RTC I/O module |
/// | 11 | `2`/`1` | Product ID: `2` = KX3, `1` = KX2 |
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KXInstalledOptions {
    pub has_kxat_atu: bool,
    pub has_kxpa100_pa: bool,
    pub has_kxfl3_roofing_filter: bool,
    pub has_kxat100_external_atu: bool,
    pub has_kxbc3_realtime_clock: bool,
    pub has_kx3m_transverter: bool,
    pub has_kxio2_rtc_io: bool,
    pub is_kx3: bool,
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
            has_kxat_atu: parse_option!(has_kxat_atu            => value[0] == b'A'),
            has_kxpa100_pa: parse_option!(has_kxpa100_pa          => value[1] == b'P'),
            has_kxfl3_roofing_filter: parse_option!(has_kxfl3_roofing_filter=> value[2] == b'F'),
            has_kxat100_external_atu: parse_option!(has_kxat100_external_atu=> value[6] == b'T'),
            has_kxbc3_realtime_clock: parse_option!(has_kxbc3_realtime_clock=> value[7] == b'B'),
            has_kx3m_transverter: parse_option!(has_kx3m_transverter    => value[8] == b'X'),
            has_kxio2_rtc_io: parse_option!(has_kxio2_rtc_io        => value[9] == b'I'),
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
// IC — Get K3 Icons and Status
// ------------------------------------------------------------------------------------------------

///
/// Query K3/K3S icon and status flags.
///
/// # Reference (G5 §IC)
///
/// **GET** only. RSP format: `ICbbbbb;` — five binary-packed bytes encoding 40 status
/// bits covering sub-receiver, diversity, text-to-terminal, repeater tone, VOX, scan,
/// ESSB, DVR playback, AM-sync, and related states.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetK3IconsAndStatus;

/// Decoded K3/K3S icon and status flags from the IC response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K3IconsAndStatus {
    pub preset_1: bool,
    pub preset_2: bool,
    pub band_sel: bool,
    pub msg_playing: bool,
    pub message_bank_1: bool,
    pub message_bank_2: bool,
    pub mw_power_level: bool,
    pub tx_test: bool,
    pub bset: bool,
    pub sub_rx_on: bool,
    pub sub_rx_nb_on: bool,
    pub sub_rx_aux_bnc: bool,
    pub sub_ant_main: bool,
    pub sub_ant_aux: bool,
    pub diversity_mode: bool,
    pub vfo_ab_bands_independent: bool,
    pub vfo_ab_linked: bool,
    pub text_to_terminal_on: bool,
    pub sync_data: bool,
    pub fsk_tx_polarity_normal: bool,
    pub fsk_dual_tone_filter_on: bool,
    pub vox_on_for_cw_fsk_psk: bool,
    pub dual_passband_cf_apf_on: bool,
    pub full_qsk: bool,
    pub repeater_tx_offset_negative: bool,
    pub repeater_tx_offset_positive: bool,
    pub fm_pl_tone_on: bool,
    pub am_sync_rx: bool,
    pub noise_gate_on: bool,
    pub essb: bool,
    pub vox_on_for_voice_data_afsk: bool,
    pub fast_play_in_effect: bool,
    pub ofs_led_is_on: bool,
    pub vfob_led_on: bool,
    pub sub_rx_nr_on: bool,
    pub sub_rx_squelched: bool,
    pub main_rx_squelched: bool,
    pub am_sync_usb: bool,
    pub am_sync_lsb: bool,
    pub shift_10_hz: bool,
    pub shift_50_hz: bool,
}
// ------------------------------------------------------------------------------------------------

impl_command!(GetK3IconsAndStatus, b"IC");
impl_command_with_response!(GetK3IconsAndStatus => try_from 5 K3IconsAndStatus);

impl TryFrom<&[u8]> for K3IconsAndStatus {
    type Error = RigError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(K3IconsAndStatus {
            preset_1: parse_flag!(0:0 OFF in bytes),
            preset_2: parse_flag!(0:0 ON  in bytes),
            band_sel: parse_flag!(0:1 OFF in bytes),
            msg_playing: parse_flag!(0:2     in bytes),
            message_bank_1: parse_flag!(0:3 OFF in bytes),
            message_bank_2: parse_flag!(0:3 ON  in bytes),
            mw_power_level: parse_flag!(0:4     in bytes),
            tx_test: parse_flag!(0:5     in bytes),
            bset: parse_flag!(0:6     in bytes),
            sub_rx_on: parse_flag!(1:0     in bytes),
            sub_rx_nb_on: parse_flag!(1:1     in bytes),
            sub_rx_aux_bnc: parse_flag!(1:2     in bytes),
            sub_ant_main: parse_flag!(1:3 ON  in bytes),
            sub_ant_aux: parse_flag!(1:3 OFF in bytes),
            diversity_mode: parse_flag!(1:4     in bytes),
            vfo_ab_bands_independent: parse_flag!(1:5     in bytes),
            vfo_ab_linked: parse_flag!(1:6     in bytes),
            text_to_terminal_on: parse_flag!(2:0     in bytes),
            sync_data: parse_flag!(2:1     in bytes),
            fsk_tx_polarity_normal: parse_flag!(2:2     in bytes),
            fsk_dual_tone_filter_on: parse_flag!(2:3     in bytes),
            vox_on_for_cw_fsk_psk: parse_flag!(2:4     in bytes),
            dual_passband_cf_apf_on: parse_flag!(2:5     in bytes),
            full_qsk: parse_flag!(2:6     in bytes),
            repeater_tx_offset_negative: parse_flag!(3:0     in bytes),
            repeater_tx_offset_positive: parse_flag!(3:1     in bytes),
            fm_pl_tone_on: parse_flag!(3:2     in bytes),
            am_sync_rx: parse_flag!(3:3     in bytes),
            noise_gate_on: parse_flag!(3:4     in bytes),
            essb: parse_flag!(3:5     in bytes),
            vox_on_for_voice_data_afsk: parse_flag!(3:6     in bytes),
            fast_play_in_effect: parse_flag!(4:0     in bytes),
            ofs_led_is_on: parse_flag!(4:1 ON  in bytes),
            vfob_led_on: parse_flag!(4:1 OFF in bytes),
            sub_rx_nr_on: parse_flag!(4:2     in bytes),
            sub_rx_squelched: parse_flag!(4:3     in bytes),
            main_rx_squelched: parse_flag!(4:4     in bytes),
            am_sync_usb: parse_flag!(4:5 ON  in bytes),
            am_sync_lsb: parse_flag!(4:5 OFF in bytes),
            shift_10_hz: parse_flag!(4:6 ON  in bytes),
            shift_50_hz: parse_flag!(4:6 OFF in bytes),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// IF — Get Transceiver Information
// ------------------------------------------------------------------------------------------------

///
/// Query transceiver state (frequency, RIT/XIT, mode, split, scan, etc.).
///
/// # Reference (G5 §IF; D12 §IF)
///
/// **GET** only. On K4 this command is provided for K3 compatibility (K22/K31 meta-mode
/// variant); K4-native software should use specific commands instead.
///
/// RSP format: `IF[f]*****+yyyyrx*00tmvspbd1*;`
///
/// | Field | Width | Description |
/// |-------|-------|-------------|
/// | `[f]` | 11 | Operating frequency in Hz, zero-padded (same as FA) |
/// | `     ` | 5 | Spaces (0x20) |
/// | `+/-` | 1 | RIT/XIT sign |
/// | `yyyy` | 4 | RIT/XIT offset Hz (0000–9999) |
/// | `r` | 1 | `1` if RIT is on |
/// | `x` | 1 | `1` if XIT is on |
/// | ` ` | 1 | Space |
/// | `00` | 2 | Fixed zeros |
/// | `t` | 1 | `1` if in transmit mode |
/// | `m` | 1 | Operating mode digit (see MD) |
/// | `v` | 1 | Receive VFO: `0`=A, `1`=B (K4: always `0`) |
/// | `s` | 1 | `1` if scan in progress |
/// | `p` | 1 | `1` if in split mode |
/// | `b` | 1 | K22: `1` on band change; else `0` |
/// | `d` | 1 | K31: DATA sub-mode (0–3); else `0` |
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransceiverInformation;

/// Decoded transceiver state from the IF response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransceiverInformation {
    pub operating_frequency: Frequency,
    pub rit_xit_sign_negative: bool,
    pub rit_xit_offset_hz: u16,
    pub rit_on: bool,
    pub xit_on: bool,
    pub in_transmit_mode: bool,
    pub operating_mode: u8,
    pub receive_mode_vfo: Vfo,
    pub scan_in_progress: bool,
    pub in_split_mode: bool,
    pub data_sub_mode: Option<u8>,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverInformation, b"IF");
impl_command_with_response!(GetTransceiverInformation => try_from 28 TransceiverInformation);

impl TryFrom<&[u8]> for TransceiverInformation {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 28 {
            error!(
                "TransceiverInformation: expected ≥28 bytes, got {}",
                value.len()
            );
            return Err(RigError::InvalidResponseLength {
                expecting: 28,
                given: value.len(),
            });
        }
        let freq = Frequency::try_from(&value[0..11])?;
        // bytes 11-15: spaces; byte 16: sign; bytes 17-20: offset; 21: rit; 22: xit
        let rit_sign_neg = value[16] == b'-';
        let rit_offset = parse_decimal_u16(&value[17..21])?;
        let rit_on = value[21] == b'1';
        let xit_on = value[22] == b'1';
        // bytes 23: space; 24-25: "00"; 26: tx; 27: mode
        let in_tx = value[26] == b'1';
        let mode = value[27] - b'0';
        // byte 28 might be out of range for the 28-byte minimum slice
        let vfo = if value.get(28).copied().unwrap_or(b'0') == b'0' {
            Vfo::A
        } else {
            Vfo::B
        };
        let scan = value.get(29).copied().unwrap_or(b'0') == b'1';
        let split = value.get(30).copied().unwrap_or(b'0') == b'1';
        let data_sub = value
            .get(32)
            .and_then(|&b| if b != b'0' { Some(b - b'0') } else { None });
        Ok(Self {
            operating_frequency: freq,
            rit_xit_sign_negative: rit_sign_neg,
            rit_xit_offset_hz: rit_offset,
            rit_on,
            xit_on,
            in_transmit_mode: in_tx,
            operating_mode: mode,
            receive_mode_vfo: vfo,
            scan_in_progress: scan,
            in_split_mode: split,
            data_sub_mode: data_sub,
        })
    }
}

// ------------------------------------------------------------------------------------------------
// MD — Get/Set Operating Mode
// ------------------------------------------------------------------------------------------------

///
/// Query operating mode.
///
/// # Reference (G5 §MD; D12 §MD)
///
/// **SET/RSP** format: `MDn;` (VFO A) or `MD$n;` (VFO B / sub receiver).
///
/// Mode codes:
/// | `n` | Mode |
/// |-----|------|
/// | `1` | LSB |
/// | `2` | USB |
/// | `3` | CW |
/// | `4` | FM |
/// | `5` | AM |
/// | `6` | DATA A (AFSK A) |
/// | `7` | CW-REV |
/// | `9` | DATA B (FSK D / PSK D) |
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetOperatingMode {
    pub vfo: Vfo,
}

/// Set operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetOperatingMode {
    pub vfo: Vfo,
    pub mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetOperatingMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"MD",
            Vfo::B => b"MD$",
            _ => panic!("GetOperatingMode: only VFO A and VFO B are supported"),
        }
    }
}

impl_command_with_response!(GetOperatingMode => try_from 1 OperatingMode);

impl TryFrom<&[u8]> for OperatingMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("OperatingMode: expecting 1 byte, given {}", value.len());
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        match value[0] {
            b'1' => Ok(OperatingMode::AnalogVoice(AnalogVoiceMode::LowerSideBand)),
            b'2' => Ok(OperatingMode::AnalogVoice(AnalogVoiceMode::UpperSideBand)),
            b'3' => Ok(OperatingMode::Morse(MorseMode::ContinuousWave)),
            b'4' => Ok(OperatingMode::AnalogVoice(
                AnalogVoiceMode::FrequencyModulated,
            )),
            b'5' => Ok(OperatingMode::AnalogVoice(
                AnalogVoiceMode::AmplitudeModulated,
            )),
            b'6' => Ok(OperatingMode::Data(DataMode::Unspecified)),
            b'7' => Ok(OperatingMode::Morse(MorseMode::ContinuousWaveReverse)),
            b'9' => Ok(OperatingMode::Data(DataMode::Unspecified)),
            _ => {
                error!("OperatingMode: unknown mode byte {:02X?}", value[0]);
                Err(RigError::InvalidResponseData {
                    data: value.to_vec(),
                })
            }
        }
    }
}

impl Command for SetOperatingMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"MD",
            Vfo::B => b"MD$",
            _ => panic!("SetOperatingMode: only VFO A and VFO B are supported"),
        }
    }

    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode])
    }
}

// ------------------------------------------------------------------------------------------------
// IS — IF Shift
// ------------------------------------------------------------------------------------------------

/// Query IF Shift (passband tuning) in Hz.
///
/// RSP format: `IS ±nnnn;` (VFO A) or `IS$ ±nnnn;` (VFO B / sub receiver).
/// Range: −2999 to +2999 Hz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetIfShift {
    vfo: Vfo,
}

impl Command for GetIfShift {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"IS",
            Vfo::B => b"IS$",
            _ => panic!("GetIfShift: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetIfShift {
    type Response = i16;
    fn expected_response_length(&self) -> usize {
        5
    }
    fn parse(&self, bytes: &[u8]) -> Result<i16, RigError> {
        let d = validate_response(bytes, self.command_id(), 5)?;
        parse_signed_i16(d)
    }
}

/// Set IF Shift in Hz (−2999 to +2999).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetIfShift {
    vfo: Vfo,
    offset_hz: i16,
}

impl Command for SetIfShift {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"IS",
            Vfo::B => b"IS$",
            _ => panic!("SetIfShift: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let sign = if self.offset_hz < 0 { b'-' } else { b'+' };
        let mag = self.offset_hz.unsigned_abs();
        let mut v = vec![b' ', sign];
        v.extend_from_slice(format!("{:04}", mag).as_bytes());
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// KS — Keyer Speed
// ------------------------------------------------------------------------------------------------

/// Query keyer speed in WPM.
///
/// RSP format: `KSnnn;` — `nnn` = 008–050 (K3/KX); up to 100 on K4.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetKeyerSpeed;

impl_command!(GetKeyerSpeed, b"KS");

impl CommandWithResponse for GetKeyerSpeed {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"KS", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set keyer speed in WPM (8–50 on K3/KX; 8–100 on K4).
/// Set keyer speed in WPM (8–50 on K3/KX; 8–100 on K4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetKeyerSpeed {
    wpm: u8,
}

impl Command for SetKeyerSpeed {
    fn command_id(&self) -> &[u8] {
        b"KS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.wpm).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// KY — CW / Data Text (SET only)
// ------------------------------------------------------------------------------------------------

/// Send CW or data text to the keyer buffer (SET only).
///
/// RSP format: `KYn ssssssssssssssssssssssss;` — `n` = 0 (send now) or 1 (buffer only),
/// followed by a space and up to 24 ASCII characters. Sending empty text aborts TX.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendCwText {
    pub buffer_only: bool,
    pub text: Vec<u8>,
}

impl Command for SendCwText {
    fn command_id(&self) -> &[u8] {
        b"KY"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let flag = if self.buffer_only { b'1' } else { b'0' };
        let mut v = vec![flag, b' '];
        let len = self.text.len().min(24);
        v.extend_from_slice(&self.text[..len]);
        while v.len() < 26 {
            v.push(b' ');
        }
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// LK — VFO Lock
// ------------------------------------------------------------------------------------------------

/// Query VFO lock state.
///
/// RSP format: `LKn;` (VFO A) or `LK$n;` (VFO B) — `n` = `0` (unlocked) or `1` (locked).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetVfoLock {
    vfo: Vfo,
}

impl Command for GetVfoLock {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"LK",
            Vfo::B => b"LK$",
            _ => panic!("GetVfoLock: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetVfoLock {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set VFO lock. `locked = true` disables the VFO encoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetVfoLock {
    vfo: Vfo,
    locked: bool,
}

impl Command for SetVfoLock {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"LK",
            Vfo::B => b"LK$",
            _ => panic!("SetVfoLock: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.locked { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// LN — Link VFOs (K3 only *)
// ------------------------------------------------------------------------------------------------

/// Query VFO A/B link state (K3 only).
///
/// RSP format: `LNn;` — `n` = `0` (unlinked) or `1` (linked; both VFOs track each other).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetVfoLink;

impl_command!(GetVfoLink, b"LN");

impl CommandWithResponse for GetVfoLink {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"LN", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set VFO link on/off (K3 only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetVfoLink {
    linked: bool,
}

impl Command for SetVfoLink {
    fn command_id(&self) -> &[u8] {
        b"LN"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.linked { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// MC — Memory Channel
// ------------------------------------------------------------------------------------------------

/// Query active memory channel (1–100).
///
/// RSP format: `MCnnn;` — `nnn` = 001–100.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMemoryChannel;

impl_command!(GetMemoryChannel, b"MC");

impl CommandWithResponse for GetMemoryChannel {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MC", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set memory channel (1–100).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMemoryChannel {
    channel: u8,
}

impl Command for SetMemoryChannel {
    fn command_id(&self) -> &[u8] {
        b"MC"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.channel.clamp(1, 100)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MG — Mic Gain
// ------------------------------------------------------------------------------------------------

/// Query microphone gain (0–60).
///
/// RSP format: `MGnnn;` — `nnn` = 000–060.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMicGain;

impl_command!(GetMicGain, b"MG");

impl CommandWithResponse for GetMicGain {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MG", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set mic gain (0–60).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMicGain {
    gain: u8,
}

impl Command for SetMicGain {
    fn command_id(&self) -> &[u8] {
        b"MG"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.gain.min(60)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// ML — Monitor Level
// ------------------------------------------------------------------------------------------------

/// Query TX monitor level (0–60).
///
/// RSP format: `MLnnn;` — `nnn` = 000–060.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMonitorLevel;

impl_command!(GetMonitorLevel, b"ML");

impl CommandWithResponse for GetMonitorLevel {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"ML", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set TX monitor level (0–60).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMonitorLevel {
    level: u8,
}

impl Command for SetMonitorLevel {
    fn command_id(&self) -> &[u8] {
        b"ML"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.level.min(60)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MN — Menu Selection
// ------------------------------------------------------------------------------------------------

/// Open a front-panel menu item by number (SET only).
///
/// RSP format: `MNnnn;` — `nnn` is the 3-digit menu item number.
/// Item numbers differ between K3 (Table 5), KX3 (Table 6), and KX2 (Table 6A) in G5.
/// Send `MN000;` to close the menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectMenu {
    item: u16,
}

impl Command for SelectMenu {
    fn command_id(&self) -> &[u8] {
        b"MN"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.item).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MP — Menu Parameter (8-bit)
// ------------------------------------------------------------------------------------------------

/// Query the current menu item's 8-bit parameter (must send MN first).
///
/// RSP format: `MPnn;` — `nn` = 00–99 (BCD-encoded 8-bit value).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMenuParameter;

impl_command!(GetMenuParameter, b"MP");

impl CommandWithResponse for GetMenuParameter {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"MP", 2)?;
        parse_decimal_u8(d)
    }
}

/// Set menu parameter (0–99 BCD encoded; must send MN first).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMenuParameter {
    value: u8,
}

impl Command for SetMenuParameter {
    fn command_id(&self) -> &[u8] {
        b"MP"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.value).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// MQ — 16-bit Menu Parameter (KX3/KX2 only **)
// ------------------------------------------------------------------------------------------------

/// Query the current menu item's 16-bit parameter (KX3/KX2 only; must send MN first).
///
/// RSP format: `MQnnnn;` — `nnnn` = 0000–9999 (16-bit value).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetMenuParameter16;

impl_command!(GetMenuParameter16, b"MQ");

impl CommandWithResponse for GetMenuParameter16 {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"MQ", 4)?;
        parse_decimal_u16(d)
    }
}

/// Set 16-bit menu parameter (KX3/KX2 only; must send MN first).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetMenuParameter16 {
    value: u16,
}

impl Command for SetMenuParameter16 {
    fn command_id(&self) -> &[u8] {
        b"MQ"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.value).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// NB — Noise Blanker
// ------------------------------------------------------------------------------------------------

/// Query Noise Blanker state.
///
/// RSP format: `NBn;` (VFO A / main) or `NB$n;` (VFO B / sub) — `n` = `0`/`1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetNoiseBlanker {
    vfo: Vfo,
}

impl Command for GetNoiseBlanker {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NB",
            Vfo::B => b"NB$",
            _ => panic!("GetNoiseBlanker: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetNoiseBlanker {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set Noise Blanker on/off.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetNoiseBlanker {
    vfo: Vfo,
    on: bool,
}

impl Command for SetNoiseBlanker {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NB",
            Vfo::B => b"NB$",
            _ => panic!("SetNoiseBlanker: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// NL — Noise Blanker Level
// ------------------------------------------------------------------------------------------------

/// Query Noise Blanker level.
///
/// RSP format: `NLnn;` (VFO A) or `NL$nn;` (VFO B / sub). Range: 00–21 (K3/K3S),
/// 00–09 (KX3/KX2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetNoiseBlankerLevel {
    vfo: Vfo,
}

impl Command for GetNoiseBlankerLevel {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NL",
            Vfo::B => b"NL$",
            _ => panic!("GetNoiseBlankerLevel: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetNoiseBlankerLevel {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        parse_decimal_u8(d)
    }
}

/// Set Noise Blanker level (0–21 on K3; 0–9 on KX).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetNoiseBlankerLevel {
    vfo: Vfo,
    level: u8,
}

impl Command for SetNoiseBlankerLevel {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"NL",
            Vfo::B => b"NL$",
            _ => panic!("SetNoiseBlankerLevel: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.level).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// PA — Receive Preamp
// ------------------------------------------------------------------------------------------------

/// Query receive preamplifier selection.
///
/// RSP format: `PAn;` (VFO A) or `PA$n;` (VFO B / sub) — `n` = 0 (off), 1 (preamp 1),
/// 2 (preamp 2 / low-noise amp; K3S/KXV3B on 12/10/6 m only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPreamp {
    vfo: Vfo,
}

impl Command for GetPreamp {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"PA",
            Vfo::B => b"PA$",
            _ => panic!("GetPreamp: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetPreamp {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] - b'0')
    }
}

/// Set preamplifier (0=off, 1=preamp 1, 2=preamp 2/LNA).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPreamp {
    vfo: Vfo,
    preamp: u8,
}

impl Command for SetPreamp {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"PA",
            Vfo::B => b"PA$",
            _ => panic!("SetPreamp: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.preamp.min(2)])
    }
}

// ------------------------------------------------------------------------------------------------
// PC — Power Control
// ------------------------------------------------------------------------------------------------

/// Query TX power level in watts.
///
/// RSP format: `PCnnn;` — `nnn` = 000–110 (K3/K3S) or 000–012 (KX3/KX2) watts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerControl;

impl_command!(GetPowerControl, b"PC");

impl CommandWithResponse for GetPowerControl {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"PC", 3)?;
        parse_decimal_u8(d)
    }
}

/// Set TX power in watts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPowerControl {
    watts: u8,
}

impl Command for SetPowerControl {
    fn command_id(&self) -> &[u8] {
        b"PC"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.watts).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// PO — Actual Power Output (KX3/KX2 only **, GET only)
// ------------------------------------------------------------------------------------------------

/// Read actual RF power output in tenths of a watt (KX3/KX2 only, GET only).
///
/// RSP format: `POnnnn;` — `nnnn` = power × 10 in mW (e.g. `0050` = 5.0 W).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetActualPowerOutput;

impl_command!(GetActualPowerOutput, b"PO");

impl CommandWithResponse for GetActualPowerOutput {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"PO", 4)?;
        parse_decimal_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// PS — Power Status
// ------------------------------------------------------------------------------------------------

/// Query transceiver power state.
///
/// RSP format: `PSn;` — `n` = `0` (off) or `1` (on).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetPowerStatus;

impl_command!(GetPowerStatus, b"PS");

impl CommandWithResponse for GetPowerStatus {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"PS", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set power state. `on = false` initiates a controlled power-off sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetPowerStatus {
    on: bool,
}

impl Command for SetPowerStatus {
    fn command_id(&self) -> &[u8] {
        b"PS"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// RA — Receive Attenuator
// ------------------------------------------------------------------------------------------------

/// Query receive attenuator level.
///
/// RSP format: `RAnn;` (VFO A) or `RA$nn;` (VFO B / sub).
/// Range: 00–15 (K3/K3S, 2 dB steps up to ~30 dB); 00–01 (KX3/KX2, 0=off, 1=10 dB).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetReceiveAttenuator {
    vfo: Vfo,
}

impl Command for GetReceiveAttenuator {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"RA",
            Vfo::B => b"RA$",
            _ => panic!("GetReceiveAttenuator: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetReceiveAttenuator {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        parse_decimal_u8(d)
    }
}

/// Set receive attenuator level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetReceiveAttenuator {
    vfo: Vfo,
    level: u8,
}

impl Command for SetReceiveAttenuator {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"RA",
            Vfo::B => b"RA$",
            _ => panic!("SetReceiveAttenuator: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.level).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// RC — RIT Clear (SET only)
// ------------------------------------------------------------------------------------------------

/// Reset the RIT/XIT offset to ±0 Hz (SET only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearRit;

impl_command!(ClearRit, b"RC");

// ------------------------------------------------------------------------------------------------
// RD — RIT Offset Down (SET only)
// ------------------------------------------------------------------------------------------------

/// Decrease the RIT/XIT offset by `hz` Hz (SET only).
///
/// RSP format: `RDnnnn;` — `nnnn` = Hz to decrease (0000 = decrement one step).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RitOffsetDown {
    hz: u16,
}

impl Command for RitOffsetDown {
    fn command_id(&self) -> &[u8] {
        b"RD"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// RG — RF Gain
// ------------------------------------------------------------------------------------------------

/// Query RF gain (DAC value, 190–250).
///
/// RSP format: `RGnnn;` (VFO A) or `RG$nnn;` (VFO B / sub).
/// 190 = minimum useful gain; 250 = maximum gain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRfGain {
    vfo: Vfo,
}

impl Command for GetRfGain {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"RG",
            Vfo::B => b"RG$",
            _ => panic!("GetRfGain: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetRfGain {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        3
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 3)?;
        parse_decimal_u8(d)
    }
}

/// Set RF gain (DAC value 190–250).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetRfGain {
    vfo: Vfo,
    gain: u8,
}

impl Command for SetRfGain {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"RG",
            Vfo::B => b"RG$",
            _ => panic!("SetRfGain: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:03}", self.gain).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// RO — RIT/XIT Offset (absolute)
// ------------------------------------------------------------------------------------------------

/// Query RIT/XIT offset in Hz (−9999 to +9999).
///
/// RSP format: `RO±nnnn;` — sign followed by 4-digit Hz value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRitXitOffset;

impl_command!(GetRitXitOffset, b"RO");

impl CommandWithResponse for GetRitXitOffset {
    type Response = i16;
    fn expected_response_length(&self) -> usize {
        5
    }
    fn parse(&self, bytes: &[u8]) -> Result<i16, RigError> {
        let d = validate_response(bytes, b"RO", 5)?;
        parse_signed_i16(d)
    }
}

/// Set RIT/XIT offset in Hz (−9999 to +9999).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetRitXitOffset {
    offset_hz: i16,
}

impl Command for SetRitXitOffset {
    fn command_id(&self) -> &[u8] {
        b"RO"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        let sign = if self.offset_hz < 0 { b'-' } else { b'+' };
        let mag = self.offset_hz.unsigned_abs();
        let mut v = vec![sign];
        v.extend_from_slice(format!("{:04}", mag).as_bytes());
        Some(v)
    }
}

// ------------------------------------------------------------------------------------------------
// RT — RIT Control
// ------------------------------------------------------------------------------------------------

/// Query RIT (Receive Incremental Tuning) on/off state.
///
/// RSP format: `RTn;` — `n` = `0` (off) or `1` (on).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetRitControl;

impl_command!(GetRitControl, b"RT");

impl CommandWithResponse for GetRitControl {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"RT", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set RIT on/off.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetRitControl {
    on: bool,
}

impl Command for SetRitControl {
    fn command_id(&self) -> &[u8] {
        b"RT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// RU — RIT Offset Up (SET only)
// ------------------------------------------------------------------------------------------------

/// Increase the RIT/XIT offset by `hz` Hz (SET only).
///
/// RSP format: `RUnnnn;` — `nnnn` = Hz to increase (0000 = increment one step).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RitOffsetUp {
    hz: u16,
}

impl Command for RitOffsetUp {
    fn command_id(&self) -> &[u8] {
        b"RU"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:04}", self.hz).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// RV — Firmware Revisions (GET only)
// ------------------------------------------------------------------------------------------------

/// Query firmware revision information (GET only).
///
/// RSP format: `RVxx.xx yy.yy;` — main MCU firmware version and DSP firmware version
/// (DSP field may be absent on older radios). Returns raw ASCII bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetFirmwareRevisions;

impl_command!(GetFirmwareRevisions, b"RV");

impl CommandWithResponse for GetFirmwareRevisions {
    type Response = Vec<u8>;
    fn expected_response_length(&self) -> usize {
        11
    }
    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"RV", 11)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------
// RX — Receive Mode (SET only)
// ------------------------------------------------------------------------------------------------

/// Exit transmit and return to receive mode (SET only). Equivalent to releasing PTT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GoToReceive;

impl_command!(GoToReceive, b"RX");

// ------------------------------------------------------------------------------------------------
// SB — Sub Receiver (K3/K3S only *)
// ------------------------------------------------------------------------------------------------

/// Query sub receiver state (K3/K3S only, requires KRX3A).
///
/// RSP format: `SBn;` — `n` = `0` (off) or `1` (on).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSubReceiver;

impl_command!(GetSubReceiver, b"SB");

impl CommandWithResponse for GetSubReceiver {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"SB", 1)?;
        Ok(d[0] == b'1')
    }
}

/// Set sub receiver on/off (K3/K3S only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSubReceiver {
    on: bool,
}

impl Command for SetSubReceiver {
    fn command_id(&self) -> &[u8] {
        b"SB"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// SD — QSK Delay (GET only in K3/KX)
// ------------------------------------------------------------------------------------------------

/// Read QSK delay in milliseconds (GET only).
///
/// RSP format: `SDnnnn;` — `nnnn` = 0000–0900 ms. `0000` = full QSK (no delay). CW only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetQskDelay;

impl_command!(GetQskDelay, b"SD");

impl CommandWithResponse for GetQskDelay {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"SD", 4)?;
        parse_decimal_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// SM — S-Meter (GET only)
// ------------------------------------------------------------------------------------------------

/// Read S-meter value (GET only).
///
/// RSP format: `SMnn;` (VFO A) or `SM$nn;` (VFO B / sub) — `nn` = 00–42.
/// 00–30 correspond to S0–S9+20 dB in half-unit steps; 31–42 = over-scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSMeter {
    vfo: Vfo,
}

impl Command for GetSMeter {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"SM",
            Vfo::B => b"SM$",
            _ => panic!("GetSMeter: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetSMeter {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        parse_decimal_u8(d)
    }
}

// ------------------------------------------------------------------------------------------------
// SMH — High-Resolution S-Meter (K3/K3S only *, GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read high-resolution S-meter in 0.1 dBm units (K3/K3S only, GET only).
///
/// RSP format: `SMHnnnn;` — `nnnn` = unsigned 0.1 dBm units (0000 = weakest detectable).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetHighResolutionSMeter;

// ------------------------------------------------------------------------------------------------

impl_command!(GetHighResolutionSMeter, b"SMH");

impl CommandWithResponse for GetHighResolutionSMeter {
    type Response = u16;
    fn expected_response_length(&self) -> usize {
        4
    }
    fn parse(&self, bytes: &[u8]) -> Result<u16, RigError> {
        let d = validate_response(bytes, b"SMH", 4)?;
        parse_decimal_u16(d)
    }
}

// ------------------------------------------------------------------------------------------------
// SQ — Squelch Level
// ------------------------------------------------------------------------------------------------

///
/// Query squelch level (0–29, 0=squelch open/off).
///
/// RSP format: `SQnn;` (VFO A) or `SQ$nn;` (VFO B / sub).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetSquelch {
    vfo: Vfo,
}

///
/// Set squelch level (0–29; 0 = squelch open).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetSquelch {
    vfo: Vfo,
    level: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetSquelch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"SQ",
            Vfo::B => b"SQ$",
            _ => panic!("GetSquelch: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetSquelch {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        2
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 2)?;
        parse_decimal_u8(d)
    }
}

impl Command for SetSquelch {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"SQ",
            Vfo::B => b"SQ$",
            _ => panic!("SetSquelch: only VFO A and B supported"),
        }
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.level.min(29)).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// SWT / SWH — Switch Emulation (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Emulate a front-panel button tap (momentary press, SET only).
///
/// RSP format: `SWTnn;` — `nn` is the switch number from Table 7 (K3),
/// Table 8 (KX3), or Table 8A (KX2) in the G5 programmer's reference.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmulateButtonTap {
    switch_number: u8,
}

///
/// Emulate a front-panel button hold (long press, SET only).
///
/// RSP format: `SWHnn;` — `nn` is the switch number (same tables as [`EmulateButtonTap`]).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmulateButtonHold {
    switch_number: u8,
}

// ------------------------------------------------------------------------------------------------

impl Command for EmulateButtonTap {
    fn command_id(&self) -> &[u8] {
        b"SWT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.switch_number).into_bytes())
    }
}

impl Command for EmulateButtonHold {
    fn command_id(&self) -> &[u8] {
        b"SWH"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(format!("{:02}", self.switch_number).into_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// TB — Buffered Text (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read text from the decoded text buffer (GET only).
///
/// RSP format: `TBn ssssssss;` — `n` = 0 (empty), 1 (more follows), 2 (last segment);
/// then a space and up to 8 ASCII characters of decoded CW/RTTY text.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetBufferedText;

// ------------------------------------------------------------------------------------------------

impl_command!(GetBufferedText, b"TB");

impl CommandWithResponse for GetBufferedText {
    type Response = (u8, Vec<u8>);
    fn expected_response_length(&self) -> usize {
        9
    }
    fn parse(&self, bytes: &[u8]) -> Result<(u8, Vec<u8>), RigError> {
        let d = validate_response(bytes, b"TB", 9)?;
        let status = d[0] - b'0';
        let text = d
            .get(2..)
            .unwrap_or(&[])
            .iter()
            .copied()
            .take_while(|&b| b != b' ')
            .collect();
        Ok((status, text))
    }
}

// ------------------------------------------------------------------------------------------------
// TBX — TX Buffered Text (KX3/KX2 only **, GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read the outgoing CW/data text buffer (KX3/KX2 only, GET only).
///
/// RSP format: `TBXn ssssssss;` — same structure as TB but for the TX buffer.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTxBufferedText;

// ------------------------------------------------------------------------------------------------

impl_command!(GetTxBufferedText, b"TBX");

impl CommandWithResponse for GetTxBufferedText {
    type Response = (u8, Vec<u8>);
    fn expected_response_length(&self) -> usize {
        9
    }
    fn parse(&self, bytes: &[u8]) -> Result<(u8, Vec<u8>), RigError> {
        let d = validate_response(bytes, b"TBX", 9)?;
        let status = d[0] - b'0';
        let text = d
            .get(2..)
            .unwrap_or(&[])
            .iter()
            .copied()
            .take_while(|&b| b != b' ')
            .collect();
        Ok((status, text))
    }
}

// ------------------------------------------------------------------------------------------------
// TE — TX Equalizer (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Set transmit audio equalizer parameters (SET only).
///
/// Consult the G5 programmer's reference for parameter encoding details.
/// Sent as raw bytes; no fixed-width format defined at the command level.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetTxEqualizer {
    pub params: Vec<u8>,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetTxEqualizer {
    fn command_id(&self) -> &[u8] {
        b"TE"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(self.params.clone())
    }
}

// ------------------------------------------------------------------------------------------------
// TM — TX Meter Mode (K3/K3S only *)
// ------------------------------------------------------------------------------------------------

///
/// Query TX meter display mode (K3/K3S only).
///
/// RSP format: `TMn;` — `n` = 0 (power out), 1 (ALC), 2 (compression), 3 (drive),
/// 4 (PA plate current, requires KPA3A), 5 (SWR).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTxMeterMode;

/// Set TX meter display mode (K3/K3S only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTxMeterMode {
    mode: u8,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetTxMeterMode, b"TM");

impl CommandWithResponse for GetTxMeterMode {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, b"TM", 1)?;
        Ok(d[0] - b'0')
    }
}

impl Command for SetTxMeterMode {
    fn command_id(&self) -> &[u8] {
        b"TM"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![b'0' + self.mode.min(5)])
    }
}

// ------------------------------------------------------------------------------------------------
// TQ — Transmit Query (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Query whether the transceiver is currently transmitting (GET only).
///
/// RSP format: `TQn;` — `n` = `0` (receive) or `1` (transmit).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransmitStatus;

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransmitStatus, b"TQ");

impl CommandWithResponse for GetTransmitStatus {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"TQ", 1)?;
        Ok(d[0] == b'1')
    }
}

// ------------------------------------------------------------------------------------------------
// TT — Text-to-Terminal (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Enable or disable decoded CW/RTTY text output to the serial port (SET only).
///
/// RSP format: `TTn;` — `n` = `0` (off) or `1` (on). When on, decoded characters
/// are sent unsolicited as `TBn ssssssss;` responses.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetTextToTerminal {
    on: bool,
}

// ------------------------------------------------------------------------------------------------

impl Command for SetTextToTerminal {
    fn command_id(&self) -> &[u8] {
        b"TT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// TX — Transmit Mode (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Assert PTT and enter transmit mode (SET only). Follow with `RX;` to return to receive.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GoToTransmit;

// ------------------------------------------------------------------------------------------------

impl_command!(GoToTransmit, b"TX");

// ------------------------------------------------------------------------------------------------
// UP / UPB — VFO Move Up (SET only)
// ------------------------------------------------------------------------------------------------

///
/// Move VFO A up by one tuning step (SET only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VfoAUp;

///
/// Move VFO B up by one tuning step (SET only).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VfoBUp;

// ------------------------------------------------------------------------------------------------

impl_command!(VfoAUp, b"UP");

impl_command!(VfoBUp, b"UPB");

// ------------------------------------------------------------------------------------------------
// VX — VOX State
// ------------------------------------------------------------------------------------------------

///
/// Query VOX (Voice-Operated eXchange) state.
///
/// RSP format: `VXn;` — `n` = `0` (off) or `1` (on).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetVox;

/// Set VOX on/off.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetVox {
    on: bool,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetVox, b"VX");

impl CommandWithResponse for GetVox {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"VX", 1)?;
        Ok(d[0] == b'1')
    }
}

impl Command for SetVox {
    fn command_id(&self) -> &[u8] {
        b"VX"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// XF — XFIL Number (GET only)
// ------------------------------------------------------------------------------------------------

///
/// Read the current roofing filter (XFIL) slot number (GET only).
///
/// RSP format: `XFn;` (VFO A) or `XF$n;` (VFO B / sub) — `n` = 0–7 indicating the
/// selected XFIL slot.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetXfilNumber {
    vfo: Vfo,
}

// ------------------------------------------------------------------------------------------------

impl Command for GetXfilNumber {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"XF",
            Vfo::B => b"XF$",
            _ => panic!("GetXfilNumber: only VFO A and B supported"),
        }
    }
}

impl CommandWithResponse for GetXfilNumber {
    type Response = u8;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<u8, RigError> {
        let d = validate_response(bytes, self.command_id(), 1)?;
        Ok(d[0] - b'0')
    }
}

// ------------------------------------------------------------------------------------------------
// XT — XIT Control
// ------------------------------------------------------------------------------------------------

///
/// Query XIT (Transmit Incremental Tuning) state.
///
/// RSP format: `XTn;` — `n` = `0` (off) or `1` (on).
/// XIT shifts only the transmit frequency by the RIT/XIT offset (RO command).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetXitControl;

///
/// Set XIT on/off.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetXitControl {
    on: bool,
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetXitControl, b"XT");

impl CommandWithResponse for GetXitControl {
    type Response = bool;
    fn expected_response_length(&self) -> usize {
        1
    }
    fn parse(&self, bytes: &[u8]) -> Result<bool, RigError> {
        let d = validate_response(bytes, b"XT", 1)?;
        Ok(d[0] == b'1')
    }
}

impl Command for SetXitControl {
    fn command_id(&self) -> &[u8] {
        b"XT"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.on { b'1' } else { b'0' }])
    }
}
