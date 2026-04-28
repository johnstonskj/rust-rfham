use crate::{OnceCommand, error::CliError};
use colored::Colorize;
use rfham_bands::{uk_rsgb::rsgb_band_plan, us_fcc::arrl_voluntary_band_plan};
use rfham_core::countries::CountryCode;
use rfham_itu::allocations::FrequencyAllocation;
use rfham_markdown::ToMarkdownWith;
use std::{io::stdout, process::ExitCode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ListBandPlans;

#[derive(Debug, Default)]
pub struct ShowItuAllocations;

#[derive(Debug)]
pub struct ShowBandPlan {
    country: CountryCode,
    bands: Vec<FrequencyAllocation>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for ShowItuAllocations {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        FrequencyAllocation::write_markdown(&mut stdout())?;
        Ok(ExitCode::SUCCESS)
    }
}

// ------------------------------------------------------------------------------------------------

impl OnceCommand for ShowBandPlan {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        match self.country.as_str() {
            "UK" => rsgb_band_plan().write_markdown_with(&mut stdout(), self.bands)?,
            "US" => arrl_voluntary_band_plan().write_markdown_with(&mut stdout(), self.bands)?,
            _ => println!(
                "Do not have access to a band plan for country {}",
                self.country
            ),
        }
        Ok(ExitCode::SUCCESS)
    }
}

impl ShowBandPlan {
    pub fn new(country: CountryCode, bands: Vec<FrequencyAllocation>) -> Self {
        Self { country, bands }
    }
}

// ------------------------------------------------------------------------------------------------

impl OnceCommand for ListBandPlans {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let separator = "|".bold();
        println!("{}", "| Country | Plan Name             | Region |".bold());
        println!("{}", "| ------- | --------------------- | ------ |".bold());
        println!(
            "{separator} US      {separator} US Amateur Radio Band {separator}      2 {separator}"
        );

        Ok(ExitCode::SUCCESS)
    }
}
