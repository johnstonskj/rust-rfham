use crate::{
    OnceCommand,
    command::connect::{ConnectionString, TestTransport},
    error::CliError,
};
use clap::{Args, Subcommand};
use rfham_config::load_global_config;
use std::{path::PathBuf, process::ExitCode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    /// Test radio connection at the transport layer.
    Transport(CmdTestTransport),

    /// Test radio connection at the protocol layer.
    Protocol(CmdTestProtocol),

    List,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CmdTestTransport {
    #[arg(short = 'p', long, group = "connection")]
    profile: Option<String>,

    /// Override the default configuration file path.
    #[arg(long, requires = "profile")]
    config_file: Option<PathBuf>,

    /// Connect to a radio over TCP/IP
    ///
    /// Connection string:
    ///
    /// <host-or-ip>:<port>(;timeout=<timeout>)?
    ///
    #[arg(short = 'i', long, group = "connection")]
    ip: Option<String>,

    /// Connect to a radio over a Serial port
    ///
    /// Connection string:
    ///
    /// <path>:<baud-rate>
    /// (;data-bits=<data-bits>)?(;stop-bits=<stop-bits>)?
    /// (;flow-control=<flow-control>)?(;parity=<parity>)?
    /// (;timeout=<timeout>)?
    #[arg(short = 's', long, group = "connection")]
    serial: Option<String>,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CmdTestProtocol {
    callsign: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for ConnectionCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Transport(cmd) => cmd.execute(),
            Self::Protocol(cmd) => cmd.execute(),
            Self::List => todo!(),
        }
    }
}

impl OnceCommand for CmdTestTransport {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let command = TestTransport::new(
            self.config_file,
            if let Some(name) = self.profile {
                ConnectionString::Profile(name)
            } else if let Some(ip) = self.ip {
                ConnectionString::Ip(ip)
            } else if let Some(serial) = self.serial {
                ConnectionString::Serial(serial)
            } else {
                unreachable!()
            },
        );
        command.execute()
    }
}

impl OnceCommand for CmdTestProtocol {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
