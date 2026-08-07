use crate::{
    Frequency,
    error::{RigError, invalid_bcd_digit},
    script::{Function, ToFunction},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    io::{BufWriter, Error, Write},
    iter,
    str::FromStr,
};
use strum::{AsRefStr, EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BusAddress(u8);

pub const IC_705_DEFAULT_ADDRESS: BusAddress = BusAddress::new(0xA4);
pub const IC_905_DEFAULT_ADDRESS: BusAddress = BusAddress::new(0xAC);
pub const IC_7300_DEFAULT_ADDRESS: BusAddress = BusAddress::new(0x94);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BcdString(Vec<u8>);

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr, EnumIs, EnumTryAs,
)]
pub enum Command {
    SendFrequencyData,
    SendModeData,
    ReadBandEdgeFrequencies,
    ReadOperatingFrequency,
    ReadOperatingMode,
    SetOperatingFrequency(Frequency),
    SetOperatingMode(OperatingMode, Option<OperatingFilterMode>),
    SelectVfoMode(VfoMode),
    SelectMemoryMode(MemoryMode),
    MemoryWrite,
    MemoryCopyToVfo,
    MemoryClear,
    ReadFrequencyOffset, // ()
    SendFrequencyOffset, // ()
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr, EnumIs, EnumTryAs,
)]
pub enum VfoMode {
    SelectVfoA,
    SelectVfoB,
    EqualizeVfos,
    ExchangeVfos,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr, EnumIs, EnumTryAs,
)]
pub enum MemoryMode {
    MemoryChannel(u8),
    MemoryGroup(u8),
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr, EnumIs, EnumTryAs,
)]
pub enum OperatingMode {
    LowerSidedBand = 0x00,
    UpperSidedBand = 0x01,
    AmplitudeModulation = 0x02,
    ContinuousWave = 0x03,
    RadioTeletype = 0x04,
    FrequencyModulation = 0x05,
    WideFrequencyModulation = 0x06,
    ReverseContinuousWave = 0x07,
    ReverseRadioTeletype = 0x08,
    DigitalVoice = 0x17,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, AsRefStr, EnumIs, EnumTryAs,
)]
pub enum OperatingFilterMode {
    Filter1 = 0x01,
    Filter2 = 0x02,
    Filter3 = 0x03,
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait CommandPart {
    fn sub_command_id(&self) -> Option<u8>;
    fn size_hint(&self) -> usize;

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Error>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ BusAddress
// ------------------------------------------------------------------------------------------------

impl From<BusAddress> for u8 {
    fn from(address: BusAddress) -> u8 {
        address.0
    }
}

impl BusAddress {
    const BROADCAST_ADDR: u8 = 0x00;
    const CONTROLLER_ADDR: u8 = 0xE0;

    pub const fn new(address: u8) -> Self {
        debug_assert!(address != Self::BROADCAST_ADDR);
        Self(address)
    }

    pub const fn broadcast() -> Self {
        Self(Self::BROADCAST_ADDR)
    }

    pub const fn controller() -> Self {
        Self(Self::CONTROLLER_ADDR)
    }

    pub const fn is_broadcast(&self) -> bool {
        self.0 == Self::BROADCAST_ADDR
    }

    pub const fn is_controller(&self) -> bool {
        self.0 == Self::CONTROLLER_ADDR
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Command
// ------------------------------------------------------------------------------------------------

impl CommandPart for Command {
    fn sub_command_id(&self) -> Option<u8> {
        match self {
            Self::SelectVfoMode(sub_cmd) => sub_cmd.sub_command_id(),
            Self::SelectMemoryMode(sub_cmd) => sub_cmd.sub_command_id(),
            _ => None,
        }
    }

    fn size_hint(&self) -> usize {
        let sub_size = match self {
            Self::SetOperatingFrequency(_) => 5,
            Self::SetOperatingMode(_, _) => 2,
            Self::SelectVfoMode(sub_cmd) => sub_cmd.size_hint(),
            Self::SelectMemoryMode(sub_cmd) => sub_cmd.size_hint(),
            _ => 0,
        };
        sub_size + 1 /* command identifier */
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        w.write_all(&[self.command_id()])?;
        match self {
            Self::SetOperatingFrequency(frequency) => {
                let bytes = BcdString::from(*frequency);
                w.write_all(bytes.as_bytes())?;
            }
            Self::SetOperatingMode(op_mode, filter_mode) => {
                w.write_all(&[op_mode.as_ref().as_bytes()[0]])?;
                if let Some(filter_mode) = filter_mode {
                    w.write_all(&[filter_mode.as_ref().as_bytes()[0]])?;
                }
            }
            Self::SelectVfoMode(sub_cmd) => sub_cmd.write(w)?,
            Self::SelectMemoryMode(sub_cmd) => sub_cmd.write(w)?,
            _ => (),
        }
        Ok(())
    }
}

impl ToFunction for Command {
    fn to_function(&self) -> Function {
        match self {
            Self::ReadOperatingFrequency => Function::VfoGetFrequency(None),
            Self::SetOperatingFrequency(frequency) => Function::VfoSetFrequency(None, *frequency),
            _ => unimplemented!(
                "Conversion from Command to Function is not implemented for {:?}",
                self
            ),
        }
    }
}

impl Command {
    pub const fn command_id(&self) -> u8 {
        match self {
            Self::SendFrequencyData => 0x00,
            Self::SendModeData => 0x01,
            Self::ReadBandEdgeFrequencies => 0x02,
            Self::ReadOperatingFrequency => 0x03,
            Self::ReadOperatingMode => 0x04,
            Self::SetOperatingFrequency(_) => 0x05,
            Self::SetOperatingMode(_, _) => 0x06,
            Self::SelectVfoMode(_) => 0x07,
            Self::SelectMemoryMode(_) => 0x08,
            Self::MemoryWrite => 0x09,
            Self::MemoryCopyToVfo => 0x0A,
            Self::MemoryClear => 0x0B,
            Self::ReadFrequencyOffset => 0x0C,
            Self::SendFrequencyOffset => 0x0D,
        }
    }
    pub fn into_message_from(
        &self,
        to_addr: BusAddress,
        from_addr: BusAddress,
    ) -> Result<Vec<u8>, Error> {
        let buffer: Vec<u8> = Vec::with_capacity(self.size_hint() + 5);
        let mut writer = BufWriter::new(buffer);

        // Common message header
        writer.write_all(&[0xFE, 0xFE, to_addr.into(), from_addr.into()])?;

        // Command-specific content.
        // 1. **at least** the command identifier (1 byte),
        // 2. *optionally* an additional sub-command identifier (1 byte),
        // 3. *optionally* any data required to be sent with the command.
        self.write(&mut writer)?;

        // Common message trailer
        writer.write_all(&[0xFD]).unwrap();
        Ok(writer.into_inner().unwrap())
    }

    pub fn into_message(&self, to_addr: BusAddress) -> Result<Vec<u8>, Error> {
        self.into_message_from(to_addr, BusAddress::controller())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ SelectVfoMode
// ------------------------------------------------------------------------------------------------

impl CommandPart for VfoMode {
    fn sub_command_id(&self) -> Option<u8> {
        match self {
            Self::SelectVfoA => Some(0x00),
            Self::SelectVfoB => Some(0x01),
            Self::EqualizeVfos => Some(0xA0),
            Self::ExchangeVfos => Some(0xB0),
        }
    }
    fn size_hint(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        w.write_all(&[self.sub_command_id().unwrap()])
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ SelectMemoryMode
// ------------------------------------------------------------------------------------------------

impl CommandPart for MemoryMode {
    fn sub_command_id(&self) -> Option<u8> {
        match self {
            Self::MemoryChannel(_) => None,
            Self::MemoryGroup(_) => Some(0xA0),
        }
    }

    fn size_hint(&self) -> usize {
        2
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        match self {
            Self::MemoryChannel(data) => w.write_all(&[*data]),
            Self::MemoryGroup(data) => w.write_all(&[0xA0, *data]),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for BcdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            format!("").fmt(f)
        } else {
            format!("{:02X?}", self.0).fmt(f)
        }
    }
}

impl FromStr for BcdString {
    type Err = RigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::encode_str(s)
    }
}

impl TryFrom<Vec<u8>> for BcdString {
    type Error = RigError;

    fn try_from(in_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::encode(&in_bytes)
    }
}

impl From<Frequency> for BcdString {
    fn from(value: Frequency) -> Self {
        BcdString::encode_padded(&value.to_bytes(), 10).unwrap()
    }
}

impl BcdString {
    ///
    pub fn encode(in_bytes: &[u8]) -> Result<Self, RigError> {
        Self::encode_padded(in_bytes, 0)
    }

    pub fn encode_padded(in_bytes: &[u8], min_width: usize) -> Result<Self, RigError> {
        let input_len = in_bytes.len();
        let input_pad = input_len % 2;

        let trailing_zeros = if input_len < min_width {
            min_width - input_len
        } else {
            0
        };
        let trailing_zero_pad = trailing_zeros % 2;

        let bytes_count = (input_len + input_pad + trailing_zeros + trailing_zero_pad) / 2;
        let mut out_bytes = Vec::with_capacity(bytes_count);

        for i in (0..=(input_len + input_pad))
            .into_iter()
            .rev()
            .step_by(2)
            .skip(1)
        {
            let low_byte = in_bytes[i];
            let high_byte = if i == input_len - 1 {
                0x00
            } else {
                in_bytes[i + 1]
            };
            if low_byte > 0x09 {
                return Err(invalid_bcd_digit(low_byte));
            }
            if high_byte > 0x09 {
                return Err(invalid_bcd_digit(high_byte));
            }
            out_bytes.push(low_byte << 4 | high_byte);
        }

        out_bytes.extend(iter::repeat_n(0x00, trailing_zeros / 2));
        Ok(BcdString(out_bytes))
    }

    pub fn encode_str<S>(s: S) -> Result<Self, RigError>
    where
        S: AsRef<str>,
    {
        if s.as_ref().is_ascii() {
            Self::encode(
                &s.as_ref()
                    .chars()
                    .map(|c| c as u8 - b'0')
                    .collect::<Vec<_>>(),
            )
        } else {
            panic!()
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn decode(&self) -> Vec<u8> {
        self.0
            .iter()
            .map(|d| [d & 0x0F >> 4, d & 0xF0])
            .flatten()
            .rev()
            .collect()
    }

    pub fn decode_to_string(&self) -> String {
        self.decode().iter().map(|d| (d + b'0') as char).collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::{BcdString, Command, IC_705_DEFAULT_ADDRESS};
    use crate::{Frequency, script::ToFunction};
    use std::str::FromStr;

    #[test]
    fn test_bcd_encode_even() {
        assert_eq!(
            BcdString::try_from(vec![0x01, 0x02, 0x03, 0x04])
                .unwrap()
                .into_bytes(),
            vec![0x34, 0x12]
        );
    }

    #[test]
    fn test_bcd_encode_odd() {
        assert_eq!(
            BcdString::try_from(vec![0x01, 0x02, 0x03])
                .unwrap()
                .into_bytes(),
            vec![0x30, 0x12]
        );
    }

    #[test]
    fn test_bcd_encode_str_even() {
        assert_eq!(
            BcdString::from_str("1234").unwrap().into_bytes(),
            vec![0x34, 0x12]
        );
    }

    #[test]
    fn test_bcd_encode_str_odd() {
        assert_eq!(
            BcdString::from_str("123").unwrap().into_bytes(),
            vec![0x30, 0x12]
        );
    }

    #[test]
    fn test_bcd_encode_str_not_digit() {
        BcdString::from_str("x")
            .expect_err("Should fail because 'x' does not convert to a BCD digit");
    }

    #[test]
    fn test_bcd_encode_byte_not_digit() {
        BcdString::try_from(vec![0x0A]).expect_err("Should fail because 0x0A is not a BCD digit");
    }

    #[test]
    fn test_frequency_to_bytes() {
        let frequency = Frequency::from(14_175_000);
        let bytes = BcdString::from(frequency).into_bytes();
        assert_eq!(bytes, [0x00, 0x50, 0x17, 0x14, 0x00]);
    }

    #[test]
    fn test_read_operating_frequency_to_function() {
        let cmd = Command::ReadOperatingFrequency;

        assert_eq!(
            "(vfo-get-frequency)".to_string(),
            cmd.to_function().to_string()
        );

        let cmd = Command::SetOperatingFrequency(Frequency::from(145_000_000));

        assert_eq!(
            "(vfo-set-frequency (hertz 145000000))".to_string(),
            cmd.to_function().to_string()
        );
    }

    #[test]
    fn test_read_operating_frequency() {
        assert_eq!(
            "[FE, FE, A4, E0, 03, FD]".to_string(),
            format!(
                "{:02X?}",
                Command::ReadOperatingFrequency
                    .into_message(IC_705_DEFAULT_ADDRESS)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_set_operating_frequency() {
        assert_eq!(
            "[FE, FE, A4, E0, 05, 00, 00, 00, 50, 14, FD]".to_string(),
            format!(
                "{:02X?}",
                Command::SetOperatingFrequency(Frequency::from(145_000_000))
                    .into_message(IC_705_DEFAULT_ADDRESS)
                    .unwrap()
            )
        );
    }
}
