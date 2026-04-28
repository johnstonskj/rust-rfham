use crate::{
    OnceCommand,
    command::config::{InitializeConfig, ShowCurrentConfig},
    error::CliError,
};
use clap::{Args, Subcommand};
use rfham_config::paths::ConfigPath;
use rfham_core::{callsigns::CallSign, countries::CountryCode};
use rfham_maidenhead::MaidenheadLocator;
use rfham_itu::regions::Region;
use std::{path::PathBuf, process::ExitCode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Show the current configuration details.
    Show(CmdShowConfig),

    /// Initialize configuration settings.
    Init(CmdInitConfig),
}

#[derive(Debug, Args)]
pub struct CmdShowConfig {
    /// Override the default configuration file path.
    #[arg(long)]
    config_file: Option<PathBuf>,

    /// The path to a field in the configuration.
    ///
    /// This uses a dotted notation, e.g. `station.callsign`.
    path: Option<ConfigPath>,
}

#[derive(Debug, Args)]
pub struct CmdInitConfig {
    /// Override the default configuration file path.
    #[arg(long)]
    config_file: Option<PathBuf>,

    /// If set this will overwrite any existing configuration.
    #[arg(short = 'w', long, default_value_t = false)]
    overwrite: bool,

    /// Your name, typically use the same name as your license.
    #[arg(short = 'n', long)]
    operator_name: Option<String>,

    /// Yoiur location/QTH as a Maidenhead grid locator.
    #[arg(short = 'l', long)]
    locator: Option<MaidenheadLocator>,

    /// Your ITU region, either '1'/'one', '2'/'two', or '3'/'three'.
    #[arg(short = 'r', long)]
    pub itu_region: Option<Region>,

    /// Your country code in two-letter form.
    #[arg(short = 'c', long)]
    pub country: Option<CountryCode>,

    /// Your mailing address, for QSO.
    #[arg(short = 'a', long)]
    pub mailing_address: Option<String>,

    /// The primary operator's callsign.
    callsign: CallSign,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for ConfigCommands {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Show(cmd) => cmd.execute(),
            Self::Init(cmd) => cmd.execute(),
        }
    }
}

impl OnceCommand for CmdShowConfig {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        ShowCurrentConfig::new(self.config_file, self.path).execute()
    }
}

impl OnceCommand for CmdInitConfig {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        InitializeConfig::new(
            self.config_file,
            self.overwrite,
            self.callsign,
            self.operator_name,
            self.locator,
            self.itu_region,
            self.country,
            self.mailing_address,
        )
        .execute()
    }
}
