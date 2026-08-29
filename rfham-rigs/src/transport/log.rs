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

use chrono::{DateTime, Utc};
use std::{
    fmt::Display,
    io::{Write, stderr, stdout},
};
use strum::{AsRefStr, Display as EnumDisplay, EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait MessageLogger {
    fn log(&self, kind: MessageKind, body: MessageBody<'_>);
    fn log_command(&self, body: MessageBody<'_>) {
        self.log(MessageKind::Command, body);
    }
    fn log_reply(&self, body: MessageBody<'_>) {
        self.log(MessageKind::Reply, body);
    }
    fn log_echo(&self, body: MessageBody<'_>) {
        self.log(MessageKind::Echo, body);
    }
    fn log_error(&self, body: MessageBody<'_>) {
        self.log(MessageKind::Error, body);
    }
    fn log_protocol(&self, body: MessageBody<'_>) {
        self.log(MessageKind::Protocol, body);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIs, EnumTryAs)]
pub enum MessageBody<'a> {
    String(&'a str),
    Bytes(&'a [u8]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIs, AsRefStr, EnumDisplay)]

pub enum MessageKind {
    Command,
    Reply,
    Echo,
    Error,
    Protocol,
}

#[derive(Clone, Debug)]
pub struct TracingMessageLogger {
    conn_name: String,
}

#[derive(Clone, Debug)]
pub struct StdioMessageLogger {
    conn_name: String,
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
// Implementations
// ------------------------------------------------------------------------------------------------

impl MessageLogger for TracingMessageLogger {
    fn log(&self, kind: MessageKind, body: MessageBody<'_>) {
        match kind {
            MessageKind::Error => {
                tracing::error!(kind = kind.as_ref(), connection = self.conn_name, "{body}")
            }
            MessageKind::Protocol => {
                tracing::debug!(kind = kind.as_ref(), connection = self.conn_name, "{body}")
            }
            _ => tracing::info!(kind = kind.as_ref(), connection = self.conn_name, "{body}"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl MessageLogger for StdioMessageLogger {
    fn log(&self, kind: MessageKind, body: MessageBody<'_>) {
        let time_now: DateTime<Utc> = Utc::now();
        let message = format!(
            "{} :: {} :: {kind} :: {body}",
            time_now.format("%+"),
            self.conn_name
        );
        match kind {
            MessageKind::Error => {
                let mut port = stderr().lock();
                let _ = writeln!(port, "{message}");
            }
            _ => {
                let mut port = stdout().lock();
                let _ = writeln!(port, "{message}");
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for MessageBody<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bytes(b) => format!("{b:02X?}").fmt(f),
            Self::String(s) => format!("{s:?}").fmt(f),
        }
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
mod tests {}
