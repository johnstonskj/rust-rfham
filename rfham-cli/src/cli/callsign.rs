use crate::{
    OnceCommand,
    command::callsign::{LookupCallSign, ValidateCallSign},
    error::CliError,
};
use clap::{Args, Subcommand};
use rfham_config::load_global_config;
use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum CallSignCommands {
    /// Show the current configuration details.
    Validate(CmdValidateCallSign),

    Lookup(CmdLookupCallSign),
}

#[derive(Debug, Args)]
pub struct CmdValidateCallSign {
    callsign: String,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CmdLookupCallSign {
    callsign: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for CallSignCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        load_global_config()?;
        match self {
            Self::Validate(cmd) => cmd.execute(),
            Self::Lookup(cmd) => cmd.execute(),
        }
    }
}

impl OnceCommand for CmdValidateCallSign {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        ValidateCallSign::new(self.callsign).execute()
    }
}

impl OnceCommand for CmdLookupCallSign {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        LookupCallSign::new(self.callsign).execute()
    }
}
