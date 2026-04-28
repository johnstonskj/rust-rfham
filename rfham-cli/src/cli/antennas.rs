use crate::{OnceCommand, command::antennas::CalculateAntennaLengths, error::CliError};
use clap::{Args, Subcommand};
use rfham_antennas::AntennaForm;
use rfham_core::countries::CountryCode;
use rfham_itu::allocations::FrequencyAllocation;
use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum AntennaCommands {
    /// Calculate lengths for antenna type and band.
    Length(CmdAntennaLength),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CmdAntennaLength {
    /// Calculate for an antenna working on this band.
    #[arg(short = 'b', long, requires = "country")]
    band: FrequencyAllocation,

    /// Use the band plan for this country in calculating mid-points, etc.
    #[arg(short = 'c', long, env = "RFHAM_COUNTRY")]
    country: CountryCode,

    /// Calculate the component lengths for this kind of antenna.
    #[arg(short = 'k', long, default_value_t = AntennaForm::Dipole)]
    kind: AntennaForm,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for AntennaCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Length(cmd_antenna_length) => cmd_antenna_length.execute(),
        }
    }
}

impl OnceCommand for CmdAntennaLength {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        CalculateAntennaLengths::new(self.country, self.band).execute()
    }
}
