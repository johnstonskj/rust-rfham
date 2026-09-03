//!
//! This module provides implementations of the various CI-V protocol variants for Icom
//! and Xiegu transceivers.
//!

use crate::{
    error::{RigError, invalid_bcd_digit},
    protocol::{Command, Frequency},
};
use core::{
    fmt::{Debug, Display},
    iter,
    str::FromStr,
};
use serde::{Deserialize, Serialize};

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

pub trait CivCommand: Command + Default {
    fn send_to(address: BusAddress) -> Self;

    fn broadcast() -> Self;

    fn send_to_address(&self) -> BusAddress;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ BusAddress
// ------------------------------------------------------------------------------------------------

impl From<BusAddress> for u8 {
    fn from(address: BusAddress) -> u8 {
        address.0
    }
}

impl Default for BusAddress {
    fn default() -> Self {
        Self::new(BusAddress::DEFAULT_ADDR)
    }
}

impl BusAddress {
    const BROADCAST_ADDR: u8 = 0x00;
    const CONTROLLER_ADDR: u8 = 0xE0;
    const DEFAULT_ADDR: u8 = 0xFF;

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
// Implementations ❯ BcdString
// ------------------------------------------------------------------------------------------------

impl Display for BcdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "BCD {{ {} }}", self.decode_to_string())
        } else {
            write!(f, "{:02X?}", self.0)
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
    pub fn encode(in_bytes: &[u8]) -> Result<Self, RigError> {
        Self::encode_padded(in_bytes, 0)
    }

    pub fn encode_padded(in_bytes: &[u8], min_width: usize) -> Result<Self, RigError> {
        let input_len = in_bytes.len();
        let input_pad = input_len % 2;

        let trailing_zeros = min_width.saturating_sub(input_len);
        let trailing_zero_pad = trailing_zeros % 2;

        let bytes_count = (input_len + input_pad + trailing_zeros + trailing_zero_pad) / 2;
        let mut out_bytes = Vec::with_capacity(bytes_count);

        for i in (0..=(input_len + input_pad)).rev().step_by(2).skip(1) {
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
            .flat_map(|d| [d & 0xF0 >> 4, d & 0x0F])
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

#[allow(unused)]
fn make_message(cmd: &impl Command, to_address: u8, terminator: u8) -> Result<Vec<u8>, RigError> {
    let full_preamble = &[
        0xFE, // ↴
        0xFE, // two byte preamble
        to_address,
        BusAddress::CONTROLLER_ADDR,
    ];
    Ok(full_preamble
        .iter()
        .copied()
        .chain(cmd.command_id().iter().copied())
        .chain(cmd.argument_bytes()?.unwrap_or_default().iter().copied())
        .chain(std::iter::once(terminator))
        .collect())
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

#[cfg(feature = "icom")]
pub mod icom;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::BcdString;
    use crate::protocol::Frequency;
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
}
