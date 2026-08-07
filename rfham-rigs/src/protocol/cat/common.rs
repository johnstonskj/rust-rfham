//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::{
    Frequency, Level,
    error::RigError,
    protocol::cat::{Antenna, Command, MESSAGE_TERMINATOR, Vfo},
};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

command! { GetAfGain  =>
    vfo: Vfo
}

command! { SetAfGain =>
    vfo: Vfo,
    level: Level
}

command!(GetTransceiverId);

command! { GetOperatingFrequency =>
    vfo: Vfo
}

command! { SetOperatingFrequency =>
    vfo: Vfo,
    frequency: Frequency
}

command!(GetCurrentAntenna);

command! { SetCurrentAntenna =>
    antenna: Antenna
}

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
        error!(
            "Invalid response length; expecting {}, given {}",
            expected_read_len,
            bytes_read.len()
        );
        Err(RigError::InvalidResponseLength {
            expecting: expected_read_len,
            given: bytes_read.len(),
        })
    } else if !bytes_read.starts_with(command_id) {
        error!(
            "Invalid response command string; expecting {}, given {}",
            to_ascii_string(command_id),
            to_ascii_string(bytes_read)
        );
        Err(RigError::InvalidResponseCommandString {
            expecting: to_ascii_string(command_id),
        })
    } else if !bytes_read.ends_with(&[MESSAGE_TERMINATOR]) {
        error!("Invalid response terminator; expecting ';'");
        Err(RigError::InvalidResponseTerminator)
    } else {
        Ok(&bytes_read[command_id.len()..bytes_read.len() - 1])
    }
}

#[inline(always)]
pub(crate) fn to_ascii_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| *b as char).collect()
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for GetAfGain {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FA",
            Vfo::B => b"FB",
            _ => panic!(),
        }
    }
}

impl_command_with_response!(GetAfGain => try_from 1 Level);

// ------------------------------------------------------------------------------------------------

impl Command for SetAfGain {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FA",
            Vfo::B => b"FB",
            _ => panic!(),
        }
    }
}

impl_command_with_response!(SetAfGain => try_from 1 Level);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTransceiverId, b"ID");
impl_command_with_response!(GetTransceiverId => string);

// ------------------------------------------------------------------------------------------------

impl Command for GetOperatingFrequency {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FA",
            Vfo::B => b"FB",
            _ => panic!(),
        }
    }
}

impl_command_with_response!(GetOperatingFrequency => try_from 11 Frequency);

// ------------------------------------------------------------------------------------------------

impl Command for SetOperatingFrequency {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"FA",
            Vfo::B => b"FB",
            _ => panic!(),
        }
    }

    fn argument_bytes(&self) -> Option<Vec<u8>> {
        Some(self.frequency.into())
    }
}

impl_command_with_response!(SetOperatingFrequency => try_from 11 Frequency);

// ------------------------------------------------------------------------------------------------

impl_command!(GetCurrentAntenna, b"AN");
impl_command_with_response!(GetCurrentAntenna => string);

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
mod tests {}
