use crate::{
    OnceCommand,
    error::{CliError, no_band_plan_for_country},
};
use rfham_bands::{uk_rsgb::rsgb_band_plan, us_fcc::arrl_voluntary_band_plan};
use rfham_core::{StringLike, countries::CountryCode};
use rfham_itu::allocations::FrequencyAllocation;
use rfham_markdown::{Table, ToMarkdownWith, blank_line, header};
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
            _ => no_band_plan_for_country(self.country).print(),
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
        let mut writer = std::io::stdout();

        header(&mut writer, 1, "Configured/Known Band Plans")?;
        blank_line(&mut writer)?;

        let table = Table::new(vec![
            ("Country", 8).into(),
            ("Plan Name", 30).into(),
            ("Region", 8).into(),
        ]);
        table.headers(&mut writer)?;

        table.data_row(
            &mut writer,
            &["UK", "UK Amateur Radio Band Plan", "1"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>(),
        )?;
        table.data_row(
            &mut writer,
            &["US", "US Amateur Radio Band Plan", "2"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>(),
        )?;
        blank_line(&mut writer)?;

        Ok(ExitCode::SUCCESS)
    }
}
