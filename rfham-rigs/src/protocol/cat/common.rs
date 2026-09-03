//!
//! Kenwood-dialect CAT baseline commands.
//!
//! These commands express the common Kenwood CAT dialect that Elecraft, Kenwood, and Yaesu
//! transceivers all descend from. Vendor sub-modules (`elecraft`, `kenwood`, `yaesu`) build on
//! this baseline with model-specific commands and encodings. Not every command here is
//! implemented, or implemented identically, by every rig — confirm against the specific radio's
//! own programmer's reference before relying on a command from this module.
//!

use crate::{
    error::{
        RigError, invalid_response_command_id, invalid_response_data, invalid_response_length,
        invalid_response_terminator,
    },
    protocol::cat::MESSAGE_TERMINATOR,
};
use core::fmt::Display;
use num::{FromPrimitive, Integer, Signed, Unsigned};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Constants
// ------------------------------------------------------------------------------------------------

pub(crate) const ASCII_SIGN_POSITIVE: u8 = b'+';
pub(crate) const ASCII_SIGN_NEGATIVE: u8 = b'-';
pub(crate) const ASCII_SPACE: u8 = b' ';
pub(crate) const ASCII_DIGIT_ZERO: u8 = b'0';
pub(crate) const ASCII_DIGIT_ONE: u8 = b'1';

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn validate_response<'a>(
    bytes_read: &'a [u8],
    command_id: &[u8],
    expected_value_len: usize,
) -> Result<&'a [u8], RigError> {
    let expected_read_len = expected_value_len + command_id.len() + 1;
    if bytes_read.len() != expected_read_len {
        Err(invalid_response_length(expected_read_len, bytes_read.len()))
    } else if !bytes_read.starts_with(command_id) {
        error!(
            "Invalid response command id in {}",
            dbg_string_from_ascii(bytes_read)
        );
        Err(invalid_response_command_id(command_id))
    } else if !bytes_read.ends_with(&[MESSAGE_TERMINATOR]) {
        Err(invalid_response_terminator(b";"))
    } else {
        Ok(&bytes_read[command_id.len()..bytes_read.len() - 1])
    }
}

#[inline(always)]
pub(crate) fn bytes_to_vec(bytes: &[u8]) -> Result<Vec<u8>, RigError> {
    Ok(bytes.to_vec())
}

pub(crate) fn string_from_ascii(ascii: &[u8]) -> Result<String, RigError> {
    if ascii
        .iter()
        .all(|b| b.is_ascii_graphic() || *b == ASCII_SPACE)
    {
        Ok(ascii.iter().map(|b| *b as char).collect())
    } else {
        Err(invalid_response_data(ascii))
    }
}

pub(crate) fn dbg_string_from_ascii(ascii: &[u8]) -> String {
    if ascii
        .iter()
        .all(|b| b.is_ascii_graphic() || *b == ASCII_SPACE)
    {
        ascii.iter().map(|b| *b as char).collect()
    } else {
        format!("{:02X?}", ascii)
    }
}

pub(crate) fn u32_from_ascii(ascii: &[u8]) -> Result<u32, RigError> {
    let mut n = 0u64;
    for b in ascii {
        if !b.is_ascii_digit() {
            return Err(invalid_response_data(ascii));
        }
        n = n * 10 + u64::from(b - ASCII_DIGIT_ZERO);
    }
    u32::try_from(n).map_err(|_| invalid_response_data(ascii))
}

pub(crate) fn u16_from_ascii(ascii: &[u8]) -> Result<u16, RigError> {
    let mut n = 0u32;
    for &b in ascii {
        if !b.is_ascii_digit() {
            return Err(invalid_response_data(ascii));
        }
        n = n * 10 + u32::from(b - b'0');
    }
    u16::try_from(n).map_err(|_| invalid_response_data(ascii))
}

#[allow(unused)]
pub(crate) fn i16_from_ascii(ascii: &[u8]) -> Result<i16, RigError> {
    let mut n = 0u16;
    for &b in ascii {
        if !b.is_ascii_digit() {
            return Err(invalid_response_data(ascii));
        }
        n = n * 10 + u16::from(b - b'0');
    }
    i16::try_from(n).map_err(|_| invalid_response_data(ascii))
}

pub(crate) fn u8_from_ascii(ascii: &[u8]) -> Result<u8, RigError> {
    let mut n = 0u8;
    for b in ascii {
        if !b.is_ascii_digit() {
            return Err(invalid_response_data(ascii));
        }
        n = n * 10 + (b - ASCII_DIGIT_ZERO);
    }
    Ok(n)
}

pub(crate) fn sign_from_ascii_strict<T>(ascii: u8) -> Result<T, RigError>
where
    T: Integer + Signed + FromPrimitive,
{
    match ascii {
        ASCII_SIGN_NEGATIVE => Ok(T::from_i32(-1).unwrap()),
        ASCII_SIGN_POSITIVE => Ok(T::from_i32(1).unwrap()),
        _ => Err(RigError::ParseSign { value: ascii }),
    }
}

pub(crate) fn sign_from_ascii_loose(ascii: u8) -> Result<i32, RigError> {
    match ascii {
        ASCII_SPACE => Ok(1),
        _ => sign_from_ascii_strict(ascii),
    }
}

#[allow(unused)]
pub(crate) fn i8_from_ascii(ascii: &[u8]) -> Result<i8, RigError> {
    if ascii.len() < 2 || ascii.len() > 3 {
        Err(invalid_response_length(2..=3, ascii.len()))
    } else {
        let sign = sign_from_ascii_loose(ascii[0])? as i16;
        let mag = u16_from_ascii(&ascii[1..3])? as i16;
        i8::try_from(sign * mag).map_err(|_| invalid_response_data(ascii.to_vec()))
    }
}

#[allow(unused)]
pub(crate) fn i8_from_ascii_split(raw: &[u8], d: &[u8]) -> Result<i8, RigError> {
    let sign = sign_from_ascii_loose(d[0])? as i16;
    let mag = u16_from_ascii(&d[1..3])? as i16;
    i8::try_from(sign * mag).map_err(|_| invalid_response_data(raw.to_vec()))
}

#[allow(unused)]
pub(crate) fn bool_from_ascii_1_0(ascii: u8) -> Result<bool, RigError> {
    match ascii {
        ASCII_DIGIT_ONE => Ok(true),
        ASCII_DIGIT_ZERO => Ok(false),
        _ => Err(RigError::ParseBoolean { value: ascii }),
    }
}

#[allow(unused)]
pub(crate) fn assert_byte_eq(value: u8, expected: u8) -> Result<(), RigError> {
    if value != expected {
        error!("Expected fixed byte value {expected:02X}, got {value:02X}");
        Err(invalid_response_data(&[value]))
    } else {
        Ok(())
    }
}

#[allow(unused)]
pub(crate) fn assert_all_bytes_eq(values: &[u8], expected: u8) -> Result<(), RigError> {
    if values.iter().any(|&v| v != expected) {
        error!("Expected fixed byte value {expected:02X} for all, got {values:02X?}");
        Err(invalid_response_data(values))
    } else {
        Ok(())
    }
}

#[allow(unused)]
pub(crate) fn assert_byte_slice_eq(value: &[u8], expected: &[u8]) -> Result<(), RigError> {
    if value != expected {
        error!("Expected fixed byte string {expected:02X?}, got {value:02X?}");
        Err(invalid_response_data(value))
    } else {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub(crate) fn format_uint_ascii<N>(n: N, width: usize) -> Vec<u8>
where
    N: Unsigned + Integer + Copy + Display,
{
    format!("{:0width$}", n, width = width).into_bytes()
}

#[inline(always)]
pub(crate) fn format_int_ascii<N>(n: N, width: usize) -> Vec<u8>
where
    N: Signed + Integer + Copy + Display,
{
    let mut bytes = vec![if n.is_negative() {
        ASCII_SIGN_NEGATIVE
    } else {
        ASCII_SIGN_POSITIVE
    }];
    bytes.extend(format!("{:0width$}", n.abs(), width = width).into_bytes());
    bytes
}

// ------------------------------------------------------------------------------------------------

///
/// Check that *value* is between *min* and *max* inclusive. If not an `InvalidArgumentValue`
/// error is returned with the provided argument name and concrete type name.
///
pub(crate) fn validate_integer_in_range<N>(
    argument_name: &'static str,
    type_name: &'static str,
    value: N,
    min: N,
    max: N,
) -> Result<(), RigError>
where
    N: Integer + Copy + Display,
{
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        error!("{argument_name} value {value} out of range {min}..={max}");
        Err(RigError::InvalidArgumentValue {
            argument_name,
            type_name: type_name,
            value: value.to_string(),
        })
    }
}
