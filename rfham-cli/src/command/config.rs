use crate::{OnceCommand, error::CliError};
use clap::Args;
use colored::Colorize;
use rfham_config::{
    Configuration, Dump, Location, LocationKind, Station, error::ConfigError, paths::ConfigPath,
};
use rfham_core::{callsigns::CallSign, countries::CountryCode};
use rfham_itu::regions::Region;
use rfham_maidenhead::MaidenheadLocator;
use std::{io::stdout, path::PathBuf, process::ExitCode};
use tracing::info;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ShowCurrentConfig {
    config_file: Option<PathBuf>,
    path: Option<ConfigPath>,
}

#[derive(Debug, Args)]
pub struct InitializeConfig {
    config_file: Option<PathBuf>,
    overwrite: bool,
    callsign: CallSign,
    operator_name: Option<String>,
    locator: Option<MaidenheadLocator>,
    itu_region: Option<Region>,
    country: Option<CountryCode>,
    mailing_address: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for ShowCurrentConfig {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let config_file_path = if let Some(config_file) = self.config_file {
            config_file
        } else {
            Configuration::default_file_path()?
        };
        info!(
            "configuration file path is {config_file_path:?}, exists: {}",
            config_file_path.is_file()
        );
        if !config_file_path.is_file() {
            println!("No current configuration file exists.");
            println!("{}", "└── Help: try running `rfham config init`.".dimmed());
            Ok(ExitCode::FAILURE)
        } else {
            let config = Configuration::load_from(config_file_path)?;
            if let Some(path_name) = self.path {
                println!("Lookup by path ({path_name:?} not yet implemented");
                Ok(ExitCode::FAILURE)
            } else {
                config
                    .dump(&mut stdout(), "")
                    .map(|()| ExitCode::SUCCESS)
                    .map_err(|e: ConfigError| Self::Error::from(e))
            }
        }
    }
}

impl ShowCurrentConfig {
    pub fn new<P1: Into<PathBuf>, P2: Into<ConfigPath>>(
        config_file: Option<P1>,
        path: Option<P2>,
    ) -> Self {
        Self {
            config_file: config_file.map(|p| p.into()),
            path: path.map(|p| p.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl OnceCommand for InitializeConfig {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let config_file_path = if let Some(config_file) = self.config_file {
            config_file
        } else {
            Configuration::default_file_path()?
        };
        info!(
            "configuration file path is {config_file_path:?}, exists: {}, overwrite: {}",
            config_file_path.is_file(),
            self.overwrite
        );
        if config_file_path.is_file() && !self.overwrite {
            println!("A configuration file already exists.");
            println!(
                "{}",
                "└── Help: to overwrite the existing file, run `rfham config init --overwrite`."
                    .dimmed()
            );
            Ok(ExitCode::FAILURE)
        } else {
            let location = Location::new(LocationKind::Home)
                .with_grid_locator(self.locator)
                .with_itu_region(self.itu_region)
                .with_country(self.country)
                .with_mailing_address(self.mailing_address);
            let station = Station::new(self.callsign)
                .with_operator_name(self.operator_name)
                .with_location(Some(location));
            let mut config = Configuration::default()
                .with_path(Some(config_file_path))
                .with_station(Some(station));
            info!("about to write config {config:?}");
            config
                .save(self.overwrite)
                .map(|()| ExitCode::SUCCESS)
                .map_err(|e: ConfigError| Self::Error::from(e))
        }
    }
}

impl InitializeConfig {
    #[allow(clippy::too_many_arguments)] // TODO: factor out to a From<CliCommand> ...
    pub fn new<P: Into<PathBuf>>(
        config_file: Option<P>,
        overwrite: bool,
        callsign: CallSign,
        operator_name: Option<String>,
        locator: Option<MaidenheadLocator>,
        itu_region: Option<Region>,
        country: Option<CountryCode>,
        mailing_address: Option<String>,
    ) -> Self {
        Self {
            config_file: config_file.map(|p| p.into()),
            overwrite,
            callsign,
            operator_name,
            locator,
            itu_region,
            country,
            mailing_address,
        }
    }
}
