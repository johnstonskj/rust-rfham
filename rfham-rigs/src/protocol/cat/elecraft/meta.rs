//!
//! Meta-commands common to all Elecraft transceivers.
//!
//! Covers: K2/K3/K4 command-mode selection
//!

use crate::{
    error::RigError,
    protocol::cat::{Command, common::validate_response},
};
use std::fmt::Display;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// K2 -- Get command mode
// ------------------------------------------------------------------------------------------------

///
/// Query K2 meta-command mode.
///
/// # Reference (KIO2 rev. E, §K2)
///
/// **SET/RSP** format: `K2n;` where `n` is:
/// - `0` = Normal RSP, RTTY decode on
/// - `1` = Normal RSP, RTTY decode off
/// - `2` = Extended RSP, RTTY decode on
/// - `3` = Extended RSP, RTTY decode off
///
/// The K3/K3S also accepts this command and behaves identically.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetK2CommandMode;

///
/// Set K2 command mode.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetK2CommandMode {
    pub mode: K2CommandMode,
}

/// Parsed K2 command-mode response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K2CommandMode {
    pub extended: bool,
    pub rtty_off: bool,
}

// ------------------------------------------------------------------------------------------------

impl Display for K2CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.extended, self.rtty_off) {
            (false, false) => "K2 Normal Mode",
            (false, true) => "K2 Normal/RTTY off",
            (true, false) => "K2 Extended Mode",
            (true, true) => "K2 Extended/RTTY off",
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K2CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K2CommandMode: expecting 1 byte, given {}", value.len());
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        Ok(Self {
            extended: value[0] & 0b0010 != 0,
            rtty_off: value[0] & 0b0001 != 0,
        })
    }
}

impl_command!(GetK2CommandMode, b"K2");
impl_command_with_response!(GetK2CommandMode => try_from 1 K2CommandMode);

impl Command for SetK2CommandMode {
    fn command_id(&self) -> &[u8] {
        b"K2"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![
            b'0' + ((self.mode.extended as u8) << 1) + (self.mode.rtty_off as u8),
        ])
    }
}

// ------------------------------------------------------------------------------------------------
// K3 -- Get command mode
// ------------------------------------------------------------------------------------------------

///
/// Query K3 meta-command mode.
///
/// # Reference (G5, §K3)
///
/// **SET/RSP** format: `K3n;` where `n` is:
/// - `0` = Normal RSP format (default after power-on)
/// - `1` = Extended RSP format (K31 mode — enables extra fields in IF, etc.)
///
/// Software should set K31 immediately after connecting to enable full functionality.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetK3CommandMode;

///
/// Set K3 command mode.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetK3CommandMode {
    pub mode: K3CommandMode,
}
/// Parsed K3 command-mode response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K3CommandMode {
    pub extended: bool,
}

// ------------------------------------------------------------------------------------------------

impl Display for K3CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.extended {
            "K3 Extended Mode"
        } else {
            "K3 Normal Mode"
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K3CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K3CommandMode: expecting 1 byte, given {}", value.len());
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        Ok(Self {
            extended: value[0] == b'1',
        })
    }
}

impl_command!(GetK3CommandMode, b"K3");
impl_command_with_response!(GetK3CommandMode => try_from 1 K3CommandMode);

impl Command for SetK3CommandMode {
    fn command_id(&self) -> &[u8] {
        b"K3"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.mode.extended { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// K4 -- Get command mode
// ------------------------------------------------------------------------------------------------

///
/// Query K4 meta-command mode.
///
/// # Reference (D12, §K4)
///
/// **SET/RSP** format: `K4n;` where `n` is:
/// - `0` = Normal (K3-compatible RSP format; default after power-on)
/// - `1` = Advanced (K4 extended RSP format — K41 mode)
///
/// K41 mode is required to use many K4-specific commands and response fields.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetK4CommandMode;

/// Set K4 command mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetK4CommandMode {
    pub mode: K4CommandMode,
}

/// Parsed K4 command-mode response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K4CommandMode {
    pub advanced: bool,
}

// ------------------------------------------------------------------------------------------------

impl Display for K4CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.advanced {
            "K4 Advanced Mode"
        } else {
            "K4 Normal Mode"
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K4CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K4CommandMode: expecting 1 byte, given {}", value.len());
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        Ok(Self {
            advanced: value[0] == b'1',
        })
    }
}

impl_command!(GetK4CommandMode, b"K4");
impl_command_with_response!(GetK4CommandMode => try_from 1 K4CommandMode);

impl Command for SetK4CommandMode {
    fn command_id(&self) -> &[u8] {
        b"K4"
    }
    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(vec![if self.mode.advanced { b'1' } else { b'0' }])
    }
}

// ------------------------------------------------------------------------------------------------
// Private helpers shared via super:: with sibling sub-modules
// ------------------------------------------------------------------------------------------------

/// Parse ASCII decimal bytes (no sign) as `u16`.
pub(super) fn parse_decimal_u16(bytes: &[u8]) -> Result<u16, RigError> {
    let mut n = 0u32;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(RigError::InvalidResponseData {
                data: bytes.to_vec(),
            });
        }
        n = n * 10 + u32::from(b - b'0');
    }
    u16::try_from(n).map_err(|_| RigError::InvalidResponseData {
        data: bytes.to_vec(),
    })
}

/// Parse ASCII decimal bytes (no sign) as `u8`.
pub(super) fn parse_decimal_u8(bytes: &[u8]) -> Result<u8, RigError> {
    let n = parse_decimal_u16(bytes)?;
    u8::try_from(n).map_err(|_| RigError::InvalidResponseData {
        data: bytes.to_vec(),
    })
}

/// Parse a sign-prefixed ASCII decimal as `i16` (`+nnnn` or `-nnnn`).
pub(super) fn parse_signed_i16(bytes: &[u8]) -> Result<i16, RigError> {
    if bytes.is_empty() {
        return Err(RigError::InvalidResponseData {
            data: bytes.to_vec(),
        });
    }
    let (sign, digits) = match bytes[0] {
        b'+' => (1i32, &bytes[1..]),
        b'-' => (-1i32, &bytes[1..]),
        _ => (1i32, bytes),
    };
    let magnitude = i32::from(parse_decimal_u16(digits)?);
    i16::try_from(sign * magnitude).map_err(|_| RigError::InvalidResponseData {
        data: bytes.to_vec(),
    })
}
