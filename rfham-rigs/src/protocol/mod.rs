//!
//! Provides a protocol-specific implementation for various radio communication protocols.
//!
//! Currently, this module supports the following protocols:
//!
//! - CAT (Computer Aided Transceiver)
//! - CI-V (Icom's Computer Interface)
//!
//!

#![allow(rustdoc::private_doc_tests)]

use crate::{error::RigError, transport::ActiveConnectionKind};
use core::{
    fmt::{Debug, Display},
    str::FromStr,
};
use rfham_iri::UniversalRigName;
use serde::{Deserialize, Serialize};
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A command is a structure that represents a specific instruction or query sent to a connected
/// device.
///
/// A command has an identifier and zero or more arguments. The command is serialized into a
/// *message*, a byte vector, that is sent to the device.
///
pub trait Command: Debug {
    const MESSAGE_TERMINATOR: u8;

    fn command_id(&self) -> &[u8];

    ///
    /// This method is called before a command is sent to a device and allows the handler to ensure
    /// the command is valid  *before* sending it to the transport layer.
    ///
    /// The intent is that, given command fields are public and therefore a client may set invalid
    /// values or combinations, the command has a chance to signal this before send. It is
    /// therefore expected that the most common form of error is `InvalidArgumentValue`, although
    /// this isn't a requirement.
    ///
    fn validate(&self) -> Result<(), RigError> {
        Ok(())
    }

    ///
    /// Almost identical to [`Self::validate`] except that it gives the command a chance to adjust
    /// the values the client provided to make them valid.
    ///
    /// How the protocol handler determines whether to call `validate` or `validate_or_fix` is
    /// not yet defined.
    ///
    fn validate_or_fix(&mut self) -> Result<(), RigError> {
        Ok(())
    }

    fn argument_bytes(&self) -> Result<Option<Vec<u8>>, RigError> {
        Ok(None)
    }

    fn message_preamble(&self) -> Option<&[u8]>;

    fn to_message(&self) -> Result<Vec<u8>, RigError>;
}

///
/// Some commands expect a response from the device, this trait is implemented for those commands
/// and provides the necessary methods to handle the response.
///
pub trait CommandWithResponse: Command {
    type Response;

    fn expected_response_length(&self) -> usize {
        0
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError>;
}

///
/// TBD
///
pub trait ProtocolHandler {
    ///
    /// First send a message using the [`send`](#tymethod.send) method, and if this succeeds, return
    /// the value of the [`receive`](#tymethod.receive) method. This method also uses the type
    /// signature to also convert from the lower-level method's return type of `Vec<u8>` to a
    /// response object that implements [`CommandWithResponse`].
    ///
    fn send_and_receive<C>(&mut self, command: C) -> Result<Option<C::Response>, RigError>
    where
        C: Command + CommandWithResponse,
    {
        trace!("ProtocolHandler::send_and_receive({command:?})");

        command.validate()?;

        self.send(&command)?;

        if let Some(response) = self.receive()? {
            Ok(Some(command.parse(&response)?))
        } else {
            Ok(None)
        }
    }

    ///
    /// Send the command provided.
    ///
    fn send<C>(&mut self, command: &C) -> Result<(), RigError>
    where
        C: Command;

    ///
    /// Attempt to receive a message. If successfull it returns a
    ///
    fn receive(&mut self) -> Result<Option<Vec<u8>>, RigError>;

    ///
    /// Handle a synatx error, i.e. badly formed message, *or* a state error, i.e. unexpected
    /// message.
    ///
    fn handle_syntax_or_state_error(&mut self) -> Result<Vec<u8>, RigError> {
        error!("ProtocolHandler::handle_syntax_or_state_error() called");
        todo!()
    }

    ///
    /// Handle a transport-specific communication failure.
    ///
    fn handle_communication_error(&mut self) -> Result<Vec<u8>, RigError> {
        error!("ProtocolHandler::handle_communication_error() called");
        todo!()
    }

    ///
    /// Handle a buffer overflow error reported by the transport or protocol.
    ///
    fn handle_buffer_overflow_error(&mut self) -> Result<Vec<u8>, RigError> {
        error!("ProtocolHandler::handle_buffer_overflow_error() called");
        todo!()
    }

    ///
    /// Return the underlying connection this handler reads and writes to.
    ///
    fn port(&mut self) -> &mut ActiveConnectionKind;

    ///
    /// Return the name of the currently connected rig.
    ///
    fn rig_name(&self) -> &UniversalRigName;
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// In general protocols commands treat frequency values as unsigned integers in Hertz, there are no
/// fractional component.
///
/// This type translates to/from the `rfham_core::Frequency` type, which is a  floating-point value
/// in Hertz, loosing any fractional precision.
///
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct Frequency(u64);

///
/// There are some protocol commands that treat frequency values as signed integers in Hertz,
/// similar to the unsigned [`Frequency`] type, but with a sign bit.
///
/// One significant difference between [`SignedFrequency`] and the underlying [`i64`] is that the
/// sign indicator is *always* expressed in the serialized form even for positive values.
///
/// Additionally, the Elecraft protocol allows a space character `' '` to be used as a sign
/// indicator for positive values, which is not supported by the implementation of `FromStr`.
///
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct SignedFrequency(i64);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = self.0.to_string();
        let digits = as_string.len();
        if f.alternate() {
            if digits > 12 {
                // THz
                write!(
                    f,
                    "{}.{}.{}.{}.{}",
                    &as_string[..digits - 12],
                    &as_string[digits - 12..digits - 9],
                    &as_string[digits - 9..digits - 6],
                    &as_string[digits - 6..digits - 3],
                    &as_string[digits - 3..]
                )
            } else if digits > 9 {
                // GHz
                write!(
                    f,
                    "{}.{}.{}.{}",
                    &as_string[..digits - 9],
                    &as_string[digits - 9..digits - 6],
                    &as_string[digits - 6..digits - 3],
                    &as_string[digits - 3..]
                )
            } else if digits > 6 {
                // MHz
                write!(
                    f,
                    "{}.{}.{}",
                    &as_string[..digits - 6],
                    &as_string[digits - 6..digits - 3],
                    &as_string[digits - 3..]
                )
            } else if digits > 3 {
                // kHz
                write!(
                    f,
                    "{}.{}",
                    &as_string[..digits - 3],
                    &as_string[digits - 3..]
                )
            } else {
                // Hz
                write!(f, "{}", as_string)
            }
        } else {
            write!(f, "{}", as_string)
        }
    }
}

impl From<rfham_core::Frequency> for Frequency {
    fn from(frequency: rfham_core::Frequency) -> Self {
        let hertz = frequency.as_hertz();
        Self(hertz as u64)
    }
}

impl From<Frequency> for rfham_core::Frequency {
    fn from(frequency: Frequency) -> Self {
        rfham_core::Frequency::hertz(frequency.0 as f64)
    }
}

impl From<u64> for Frequency {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Frequency> for u64 {
    fn from(frequency: Frequency) -> u64 {
        frequency.0
    }
}

impl FromStr for Frequency {
    type Err = RigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u64::from_str(s).map_err(|e| RigError::ParseFrequency {
            value: s.to_string(),
            error: e,
        })?;
        Ok(Self(value))
    }
}

impl TryFrom<&[u8]> for Frequency {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = cat::common::string_from_ascii(value)?;
        let frequency = u64::from_str(&string).map_err(|e| RigError::ParseFrequency {
            value: string,
            error: e,
        })?;
        Ok(Self(frequency))
    }
}

impl From<Frequency> for Vec<u8> {
    fn from(frequency: Frequency) -> Vec<u8> {
        format!("{:011}", frequency.0).into_bytes()
    }
}

impl Frequency {
    ///
    /// Return the underlying value as an unsigned integer representing the frequency in Hertz.
    ///
    #[inline(always)]
    pub const fn value(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_with_floor(0)
    }

    pub fn to_string_with_floor(&self, floor: usize) -> String {
        let string = self.to_string();
        if floor > 0 {
            assert!(floor <= string.len());
            (string[0..string.len() - floor]).to_string()
        } else {
            string
        }
    }

    pub fn to_bytes_with_floor(&self, floor: usize) -> Vec<u8> {
        self.to_string_with_floor(floor)
            .chars()
            .map(|c| c as u8 - b'0')
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SignedFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_negative() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "+{}", self.0)
        }
    }
}

impl From<rfham_core::Frequency> for SignedFrequency {
    fn from(frequency: rfham_core::Frequency) -> Self {
        let hertz = frequency.as_hertz();
        Self(hertz as i64)
    }
}

impl From<SignedFrequency> for rfham_core::Frequency {
    fn from(frequency: SignedFrequency) -> Self {
        rfham_core::Frequency::hertz(frequency.0 as f64)
    }
}

impl From<i64> for SignedFrequency {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<SignedFrequency> for i64 {
    fn from(frequency: SignedFrequency) -> i64 {
        frequency.0
    }
}

impl FromStr for SignedFrequency {
    type Err = RigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = i64::from_str(s).map_err(|e| RigError::ParseFrequency {
            value: s.to_string(),
            error: e,
        })?;
        Ok(Self(value))
    }
}

impl TryFrom<&[u8]> for SignedFrequency {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = cat::common::string_from_ascii(value)?;
        let frequency = i64::from_str(&string).map_err(|e| RigError::ParseFrequency {
            value: string,
            error: e,
        })?;
        Ok(Self(frequency))
    }
}

impl From<SignedFrequency> for Vec<u8> {
    fn from(frequency: SignedFrequency) -> Vec<u8> {
        format!("{:011}", frequency.0).into_bytes()
    }
}

impl SignedFrequency {
    ///
    /// Return the underlying value as a signed integer representing the frequency in Hertz.
    ///
    #[inline(always)]
    pub const fn value(&self) -> i64 {
        self.0
    }

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[inline(always)]
    pub const fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    #[inline(always)]
    pub const fn abs(&self) -> i64 {
        self.0.abs()
    }

    #[inline(always)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_with_floor(0)
    }

    pub fn to_string_with_floor(&self, floor: usize) -> String {
        let string = self.to_string();
        if floor > 0 {
            assert!(floor <= string.len());
            (string[0..string.len() - floor]).to_string()
        } else {
            string
        }
    }

    pub fn to_bytes_with_floor(&self, floor: usize) -> Vec<u8> {
        self.to_string_with_floor(floor)
            .chars()
            .map(|c| c as u8 - b'0')
            .collect()
    }

    #[inline(always)]
    pub fn as_frequency(&self) -> Option<rfham_core::Frequency> {
        if self.0.is_negative() {
            None
        } else {
            Some(rfham_core::Frequency::hertz(self.0 as f64))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

#[cfg(feature = "proto-cat")]
pub mod cat;

#[cfg(feature = "proto-civ")]
pub mod civ;
