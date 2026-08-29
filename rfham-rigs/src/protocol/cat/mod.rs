//!
//! This module provides implementations of the various CAT protocols for various transceivers and
//! amplifiers, including Elecraft, Kenwood, Yaesu and others.
//!
//! CAT is a serial protocol, primarily using ASCII text commands, with one or more characters for
//! the command identifier, followed by optional arguments, and terminated with a semicolon (`;`).
//! Beyond that and some very basic *similar commands*, the protocol is largely vendor-, and in many
//! cases model-, specific.
//!

use crate::{
    error::RigError,
    protocol::{Command, CommandWithResponse, ProtocolHandler},
    transport::ActiveConnectionKind,
};
use core::{fmt::Debug, time::Duration};
use rfham_iri::UniversalRigName;
use std::{
    io::{ErrorKind, Read, Write},
    thread,
};
use tracing::{info, trace, warn};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct CatWrapper {
    rig_name: UniversalRigName,
    port: ActiveConnectionKind,
    post_write_delay: Duration,
    read_partial_delay: Duration,
    // busy_transmit_delay: Duration,
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

impl ProtocolHandler for CatWrapper {
    fn send<C>(&mut self, command: &C) -> Result<(), RigError>
    where
        C: crate::protocol::Command,
    {
        trace!(
            "CatWrapper::send({command:?}) with post_write_delay: {:?}",
            self.post_write_delay
        );
        let message = command.to_message()?;

        trace!(
            "CatWrapper::sending {} bytes => {message:02X?}",
            message.len()
        );
        self.port.write_all(&message)?;
        self.port.flush()?;

        thread::sleep(self.post_write_delay);

        Ok(())
    }

    fn receive(&mut self) -> Result<Option<Vec<u8>>, RigError> {
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
            thread::sleep(self.read_partial_delay);
        }

        Ok(Some(response[0..total_length].to_vec()))
    }

    fn port(&mut self) -> &mut ActiveConnectionKind {
        &mut self.port
    }

    fn rig_name(&self) -> &UniversalRigName {
        &self.rig_name
    }
}

impl CatWrapper {
    pub fn new(port: ActiveConnectionKind, rig_name: UniversalRigName) -> Self {
        info!("CatWrapper::new(..., {rig_name:?})");
        assert!(rig_name.is_rig(), "UniversalRigName must be a rig name");
        Self {
            rig_name,
            port,
            post_write_delay: Duration::from_millis(50),
            read_partial_delay: Duration::from_millis(10),
            // busy_transmit_delay: Duration::from_millis(100),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn make_message(command_id: &[u8], argument_bytes: Option<Vec<u8>>, terminator: u8) -> Vec<u8> {
    command_id
        .iter()
        .copied()
        .chain(argument_bytes.unwrap_or_default().iter().copied())
        .chain(std::iter::once(terminator))
        .collect()
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod common;
pub mod elecraft;
pub mod kenwood;
pub mod yaesu;
