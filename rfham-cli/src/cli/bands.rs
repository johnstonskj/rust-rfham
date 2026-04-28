use crate::{
    OnceCommand,
    command::bands::{ListBandPlans, ShowBandPlan, ShowItuAllocations},
    error::CliError,
};
use clap::{Args, Subcommand};
use rfham_core::countries::CountryCode;
use rfham_itu::allocations::FrequencyAllocation;
use std::process::ExitCode;
use tracing::instrument;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum BandPlanCommands {
    /// List currently known band plans by country.
    List,
    /// Show the ITU band allocations for each region.
    Itu,
    /// Show a given country's band plan.
    Show(CmdShowBandPlan),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CmdShowBandPlan {
    /// Show only this band
    #[arg(short = 'b', long, requires = "country")]
    band: Vec<FrequencyAllocation>,

    /// Show the band plan for this country
    #[arg(env = "RFHAM_COUNTRY")]
    country: CountryCode,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for BandPlanCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::List => ListBandPlans.execute(),
            Self::Itu => ShowItuAllocations.execute(),
            Self::Show(cmd_show_band_plan) => cmd_show_band_plan.execute(),
        }
    }
}

impl OnceCommand for CmdShowBandPlan {
    type Output = ExitCode;
    type Error = CliError;

    #[instrument(name = "bandplan_show")]
    fn execute(self) -> Result<Self::Output, Self::Error> {
        ShowBandPlan::new(self.country, self.band).execute()
    }
}
