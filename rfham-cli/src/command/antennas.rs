use crate::{OnceCommand, error::CliError};
use rfham_antennas::SimpleDipole;
use rfham_bands::{uk_rsgb::rsgb_band_plan, us_fcc::arrl_voluntary_band_plan};
use rfham_core::country::CountryCode;
use rfham_itu::allocations::FrequencyAllocation;
use rfham_markdown::ToMarkdown;
use std::{io::stdout, process::ExitCode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct CalculateAntennaLengths {
    country: CountryCode,
    band: FrequencyAllocation,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for CalculateAntennaLengths {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let band_plan = match self.country.as_str() {
            "UK" => rsgb_band_plan(),
            "US" => arrl_voluntary_band_plan(),
            _ => {
                println!(
                    "Do not have access to a band plan for country {}",
                    self.country
                );
                return Ok(ExitCode::FAILURE);
            }
        };
        let antenna = SimpleDipole::new_in_plan(self.band, band_plan);
        antenna.write_markdown(&mut stdout())?;
        Ok(ExitCode::SUCCESS)
    }
}

impl CalculateAntennaLengths {
    pub fn new(country: CountryCode, band: FrequencyAllocation) -> Self {
        Self { country, band }
    }
}
