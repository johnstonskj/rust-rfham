use crate::{OnceCommand, command::antennas::CalculateAntennaLengths, error::CliError};
use clap::{Args, Subcommand, ValueEnum};
use rfham_antennas::AntennaForm;
use rfham_core::countries::CountryCode;
use rfham_config::load_global_config;
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

    /// Calculate for a random length antenna for multiple bands
    #[arg(short = 'r', long, default_value_t = false, conflicts_with = "band")]
    random_length: bool,

    /// Use the band plan for this country in calculating mid-points, etc.
    #[arg(short = 'c', long, env = "RFHAM_COUNTRY")]
    country: CountryCode,

    /// Calculate the component lengths for this kind of antenna.
    #[arg(short = 'k', long, default_value_t = AntennaForm::Dipole)]
    kind: AntennaForm,

    /// Set the length units displayed.
    #[arg(short = 'u', long)]
    units: Option<LengthUnits>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LengthUnits {
    #[default]
    Meters,
    Feet,
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
        load_global_config()?;
        CalculateAntennaLengths::new(self.country, self.band).execute()
    }
}
