//!
//! This module provides implementations of the various CAT protocols for various transceivers and
//! amplifiers, including Elecraft, Kenwood, Yaesu and others.
//!
//! CAT is a serial protocol, primarily using ASCII text commands, with one or more characters for
//! the command identifier, followed by optional arguments, and terminated with a semicolon (`;`).
//! Beyond that and some very basic *similar commands*, the protocol is largely vendor-, and in many
//! cases model-, specific.
//!
//! # Modules
//!
//! The modules `elecraft`, `kenwood`, and `yaesu` **are** the public API and implementations
//! of the CAT protocol for the respective vendors/families. Each of these modules contains
//! sub-modules based on model families or other logical groupings.
//!
//! The module `common` contains validation, parsing, and formatting functions used
//! by the various CAT command implementations, and is also *not*considered part of the public API.
//!

use crate::{
    error::RigError,
    protocol::{Command, CommandWithResponse, ProtocolHandler},
    transport::{Message, Transport},
};
use core::{fmt::Debug, time::Duration};
use rfham_iri::UniversalRigName;
use std::{io::ErrorKind, thread};
use tracing::{info, trace, warn};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct CatWrapper<T: Transport> {
    rig_name: UniversalRigName,
    port: T,
    post_write_delay: Duration,
    read_partial_delay: Duration,
}

pub(crate) const MESSAGE_TERMINATOR: u8 = b';';
pub(crate) const STATE_OR_SYNTAX_ERROR_RESPONSE_ID: u8 = b'?';
pub(crate) const COMMUNICATION_ERROR_RESPONSE_ID: u8 = b'E';
pub(crate) const OVERFLOW_ERROR_RESPONSE_ID: u8 = b'O';

pub(crate) const STATE_OR_SYNTAX_ERROR_RESPONSE: &[u8] =
    &[STATE_OR_SYNTAX_ERROR_RESPONSE_ID, MESSAGE_TERMINATOR];
pub(crate) const COMMUNICATION_ERROR_RESPONSE: &[u8] =
    &[COMMUNICATION_ERROR_RESPONSE_ID, MESSAGE_TERMINATOR];
pub(crate) const OVERFLOW_ERROR_RESPONSE: &[u8] = &[OVERFLOW_ERROR_RESPONSE_ID, MESSAGE_TERMINATOR];

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const NO_DELAY: Duration = Duration::from_millis(0);

impl<T: Transport> ProtocolHandler for CatWrapper<T> {
    fn send<C>(&mut self, command: &C) -> Result<(), RigError>
    where
        C: crate::protocol::Command,
    {
        trace!(
            "CatWrapper::send({command:?}) with post_write_delay: {:?}",
            self.post_write_delay
        );
        let message = command.to_message()?;

        trace!("CatWrapper::sending {message:#}");
        self.port.write_all(message.as_bytes())?;

        if self.post_write_delay > NO_DELAY {
            thread::sleep(self.post_write_delay);
        }

        Ok(())
    }

    fn receive(&mut self) -> Result<Option<Message<'_>>, RigError> {
        trace!(
            "CatWrapper::receive() with read_partial_delay: {:?}",
            self.read_partial_delay
        );
        let mut response = [0u8; 64];
        let mut total_length = 0;

        loop {
            match self.port.read(&mut response[total_length..]) {
                Ok(length) if length > 0 => {
                    trace!(
                        "CatWrapper::receive: read {length} (total: {}) bytes => {:02X?}",
                        total_length + length,
                        &response[total_length..total_length + length]
                    );
                    total_length += length;
                    if total_length > 2 && response[total_length - 1] == MESSAGE_TERMINATOR {
                        trace!("CatWrapper::receive looks like a complete read");
                        break;
                    } else if &response[0..2] == STATE_OR_SYNTAX_ERROR_RESPONSE {
                        self.handle_syntax_or_state_error()?;
                    } else if &response[0..2] == COMMUNICATION_ERROR_RESPONSE {
                        self.handle_communication_error()?;
                    } else if &response[0..2] == OVERFLOW_ERROR_RESPONSE {
                        self.handle_buffer_overflow_error()?;
                    } else {
                        eprintln!("unexpected data?!?");
                    }
                }
                Ok(_) => {
                    warn!("CatWrapper::receive read 0 bytes, retrying");
                }
                Err(e) if e.kind() == ErrorKind::TimedOut => {
                    trace!("CatWrapper::receive timed out");
                    return Ok(None);
                }
                Err(e) => return Err(e.into()),
            }
            if self.read_partial_delay > NO_DELAY {
                thread::sleep(self.read_partial_delay);
            }
        }

        Ok(Some(Message::from(response[0..total_length].to_vec())))
    }

    fn port(&mut self) -> &mut impl Transport {
        &mut self.port
    }

    fn rig_name(&self) -> &UniversalRigName {
        &self.rig_name
    }
}

impl<T: Transport> CatWrapper<T> {
    pub fn new(port: T, rig_name: UniversalRigName) -> Self {
        info!("CatWrapper::new(..., {rig_name:?})");
        assert!(rig_name.is_rig(), "UniversalRigName must be a rig name");
        Self {
            rig_name,
            port,
            post_write_delay: Duration::from_millis(50),
            read_partial_delay: Duration::from_millis(10),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn make_message(command_id: &[u8], argument_bytes: Option<Vec<u8>>, terminator: u8) -> Message<'_> {
    command_id
        .iter()
        .copied()
        .chain(argument_bytes.unwrap_or_default().iter().copied())
        .chain(std::iter::once(terminator))
        .collect::<Vec<u8>>()
        .into()
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod common;

#[cfg(feature = "elecraft")]
pub mod elecraft;

#[cfg(feature = "kenwood")]
pub mod kenwood;

#[cfg(feature = "yaesu")]
pub mod yaesu;
