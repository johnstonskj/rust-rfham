use crate::error::RigError;
use crate::transport::ActiveConnectionKind;
use rfham_iri::UniversalRigName;
use std::{
    fmt::Debug,
    io::{ErrorKind, Read, Write},
    thread,
    time::Duration,
};
use tracing::{error, info, trace, warn};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! command {
    ($cmd_type:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $cmd_type;
    };
    ($cmd_type:ident => $( $field:ident : $type:ty),* ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $cmd_type {
            $(pub $field: $type),*
        }
    };
}

macro_rules! impl_command {
    ($type:ident, $id:literal) => {
        impl $crate::protocol::cat::Command for $type {
            fn command_id(&self) -> &[u8] {
                $id
            }
        }
    };
}

macro_rules! impl_command_with_response {
    ($type:ident => string) => {
        impl $crate::protocol::cat::CommandWithResponse for $type {
            type Response = String;

            fn expected_response_length(&self) -> usize {
                0
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                Ok($crate::protocol::cat::common::to_ascii_string(bytes))
            }
        }
    };
    ($type:ident => try_from $len:literal $inner:ty) => {
        impl $crate::protocol::cat::CommandWithResponse for $type {
            type Response = $inner;

            fn expected_response_length(&self) -> usize {
                $len
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                let response =
                    validate_response(bytes, self.command_id(), self.expected_response_length())?;
                Ok(
                    <$inner as ::std::convert::TryFrom<&[u8]>>::try_from(response).map_err(
                        |_| RigError::InvalidResponseData {
                            data: response.to_vec(),
                        },
                    )?,
                )
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Command: Debug {
    fn command_id(&self) -> &[u8];

    fn argument_bytes(&self) -> Option<Vec<u8>> {
        None
    }

    fn to_message(&self) -> Result<Vec<u8>, RigError> {
        Ok(self
            .command_id()
            .iter()
            .copied()
            .chain(
                self.argument_bytes()
                    .unwrap_or_else(Vec::new)
                    .iter()
                    .copied(),
            )
            .chain(std::iter::once(b';'))
            .collect())
    }
}

pub trait CommandWithResponse: Command {
    type Response;

    fn expected_response_length(&self) -> usize;

    fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vfo {
    A,
    B,
    C,
    SubA,
    SubB,
    SubC,
    ReceiveA,
    ReceiveB,
    ReceiveC,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Antenna {
    One,
    Two,
    Three,
    ReceiveOne,
    ReceiveTwo,
    ReceiveThree,
}

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

    pub fn send_and_receive<C>(&mut self, command: C) -> Result<Option<C::Response>, RigError>
    where
        C: Command + CommandWithResponse,
    {
        trace!("CatWrapper::send_and_receive({command:?})");

        self.send(&command)?;

        if let Some(response) = self.receive()? {
            Ok(Some(command.parse(&response)?))
        } else {
            Ok(None)
        }
    }

    pub fn send<C>(&mut self, command: &C) -> Result<(), RigError>
    where
        C: Command,
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

    pub fn receive(&mut self) -> Result<Option<Vec<u8>>, RigError> {
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
                        self.handle_syntax_or_state_error_response()?;
                    } else if &response[0..2] == COMMUNICATION_ERROR_RESPONSE {
                        self.handle_communication_error_response()?;
                    } else if &response[0..2] == OVERFLOW_ERROR_RESPONSE {
                        self.handle_overflow_error_response()?;
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

    pub fn handle_syntax_or_state_error_response(&mut self) -> Result<Vec<u8>, RigError> {
        error!("Received syntax or state error response from rig");
        todo!()
    }

    pub fn handle_communication_error_response(&mut self) -> Result<Vec<u8>, RigError> {
        error!("Received communication error response from rig");
        todo!()
    }

    pub fn handle_overflow_error_response(&mut self) -> Result<Vec<u8>, RigError> {
        error!("Received overflow error response from rig");
        todo!()
    }

    pub fn rig_name(&self) -> &UniversalRigName {
        &self.rig_name
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod common;
pub mod elecraft;
pub mod kenwood;
pub mod yaesu;
