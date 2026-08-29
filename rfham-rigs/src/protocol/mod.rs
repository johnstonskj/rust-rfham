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
use strum::{AsRefStr, Display as EnumDisplay, EnumIs, EnumString};
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

///
/// ```rust
/// define_command_enum!(
/// "doc string" => CommandType, u8 {
///    "variant doc string" => VariantName = 0x01
/// });
/// define_command_enum!(
/// "doc string" => CommandType {
///    "variant doc string" => VariantName = 0x01
/// });
/// ```
///
/// * `"doc string"` is the documentation string for the enum.
/// * `CommandType` is the name of the command enum that will be generated.
/// * `u8` is the underlying representation type for the enum, if not specified `u8` is the default.
/// * `"variant doc string"` is the documentation string for the enum variant.
/// * `VariantName` is the name of the enum variant.
/// * `0x01` is the value of the enum variant.
///
macro_rules! define_command_enum {
    //(
    //    $doc:literal => $name:ident, $repr_name:ty {
    //        $( $( $variant_doc:literal => )? $variant_name:ident = $value:literal),+
    //    }
    //) => {
    //    #[doc = $doc]
    //    #[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIs, strum::FromRepr, strum::AsRefStr)]
    //    #[repr($repr_name)]
    //    #[strum(serialize_all = "kebab-case")]
    //    pub enum $name {
    //        $(
    //            $( #[doc = $variant_doc] )?
    //            $variant_name = $value
    //        ),+
    //    }
    //};
    (
        $doc:literal => $name:ident {
            $( $( $variant_doc:literal => )? $variant_name:ident = $value:literal),+
        }
    ) => {
        // TODO: define_command_enum!($doc, $name, u8 => $( $( $variant_doc, )? $variant_name = $value),+ );
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIs, strum::FromRepr, strum::AsRefStr)]
        #[repr(u8)]
        #[strum(serialize_all = "kebab-case")]
        pub enum $name {
            $(
                $( #[doc = $variant_doc] )?
                $variant_name = $value
            ),+
        }
    };
}

macro_rules! define_command_struct {
    (
        $doc_str:literal => $cmd_type:ident no_copy {
            $(
                $( $field_doc:literal => )?
                $field_name:ident : $field_type:ty
            ),+
        }
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $cmd_type {
            $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field_name: $field_type
            ),+
        }
    };
    (
        $doc_str:literal => $cmd_type:ident {
            $(
                $( $field_doc:literal => )?
                $field_name:ident : $field_type:ty
            ),+
        }
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $cmd_type {
            $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field_name: $field_type
            ),+
        }
    };
    // ($doc_str:literal, $cmd_type:ident => state) => {
    //     define_command_struct!($doc_str => $cmd_type => on: bool);
    // };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

pub trait CommandWithResponse: Command {
    type Response;

    fn expected_response_length(&self) -> usize {
        0
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError>;
}

pub trait ProtocolHandler {
    ///
    /// First send a message using the [`send`] method, and if this succeeds, return the
    /// value of the [`response`] method. This method also uses the type signature to also
    /// convert from the lower-level method's return type of `Vec<u8>` to a response object
    /// that implements [`CommandWthResponse`].
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, AsRefStr)]
pub enum Vfo {
    #[strum(serialize = "A")]
    A,
    #[strum(serialize = "B")]
    B,
    #[strum(serialize = "C")]
    C,
    #[strum(serialize = "Sub-A")]
    SubA,
    #[strum(serialize = "Sub-B")]
    SubB,
    #[strum(serialize = "Sub-C")]
    SubC,
    #[strum(serialize = "Rcv-A")]
    ReceiveA,
    #[strum(serialize = "Rcv-B")]
    ReceiveB,
    #[strum(serialize = "Rcv-C")]
    ReceiveC,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, AsRefStr)]
pub enum Antenna {
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "Rcv-1")]
    ReceiveOne,
    #[strum(serialize = "Rcv-2")]
    ReceiveTwo,
    #[strum(serialize = "Rcv-3")]
    ReceiveThree,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, AsRefStr)]
pub enum Filter {
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct Frequency(u64);

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
    pub fn value(&self) -> u64 {
        self.0
    }

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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod cat;
pub mod civ;
