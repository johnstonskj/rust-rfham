use crate::{
    OnceCommand,
    error::{
        CliError, config_file_exists, config_file_missing, config_path_too_short,
        config_value_not_found,
    },
};
use colored::Colorize;
use icu_locale_core::Locale as OsLocale;
use inquire::{
    Confirm, InquireError, Password, PasswordDisplayMode, Select, Text,
    error::CustomUserError,
    ui::{Attributes, Color, RenderConfig, Styled},
    validator::Validation,
};
use rfham_config::{
    Configuration, Locale, Location, LocationKind, Services, Station, TimeFormat, Units,
    error::ConfigError,
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathTarget, Value},
    services::{CredentialStorageKind, Credentials},
};
use rfham_core::{callsigns::CallSign, countries::CountryCode, fmt::FormattedWriter};
use rfham_geo::grid::GridIdentifier;
use rfham_itu::{callsigns::ItuSeriesAllocation, regions::Region};
use rfham_maidenhead::MaidenheadLocator;
use rfham_services::{callsign::CallSignInfoProvider, location::get_default_provider};
use std::{io::stdout, path::PathBuf, process::ExitCode, str::FromStr};
use strum::IntoEnumIterator;
use sys_locale::get_locale;
use tracing::info;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ShowCurrentConfig {
    config_file: Option<PathBuf>,
    compact_output: bool,
    path: Option<ConfigPath>,
}

#[derive(Debug)]
pub struct InitializeConfig {
    is_interactive: bool,
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
            config_file_missing(config_file_path).print();
            Ok(ExitCode::FAILURE)
        } else {
            let config = Configuration::load_from(config_file_path)?;
            if let Some(config_path) = self.path {
                match config.value(&config_path) {
                    Ok(Value::None) => {
                        println!("field {} is not set", config_path.field_name());
                        Ok(ExitCode::SUCCESS)
                    }
                    Ok(value) => {
                        if self.compact_output {
                            println!(
                                "{}{} {} {} {}",
                                config_path.field_name(),
                                ":".dimmed(),
                                value.type_label().italic(),
                                "=".dimmed(),
                                value.to_string().bold()
                            );
                        } else {
                            println!("field: {}", config_path.field_name());
                            println!(" type: {}", value.type_label().italic());
                            println!("value: {}", value.to_string().bold());
                        }
                        Ok(ExitCode::SUCCESS)
                    }
                    Err(ConfigError::InvalidPathComponent(name, in_name, possible)) => {
                        config_value_not_found(name, in_name, &possible).print();
                        Ok(ExitCode::FAILURE)
                    }
                    Err(ConfigError::PathTooShort(name, in_name, possible)) => {
                        config_path_too_short(name, in_name, &possible).print();
                        Ok(ExitCode::FAILURE)
                    }
                    Err(e) => Err(e.into()),
                }
            } else {
                config
                    .write_with(
                        &mut stdout(),
                        &FormatterOptions::default().with_output_kind(OutputKind::MarkdownList),
                    )
                    .map(|()| ExitCode::SUCCESS)
                    .map_err(|e: ConfigError| Self::Error::from(e))
            }
        }
    }
}

impl ShowCurrentConfig {
    pub fn new<P1: Into<PathBuf>, P2: Into<ConfigPath>>(
        config_file: Option<P1>,
        compact_output: bool,
        path: Option<P2>,
    ) -> Self {
        Self {
            config_file: config_file.map(|p| p.into()),
            compact_output,
            path: path.map(|p| p.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl OnceCommand for InitializeConfig {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        // TODO: if the command-line `--interactive` flag is set then use the `inquire` crate.
        let config_file_path = if let Some(config_file) = &self.config_file {
            config_file.clone()
        } else {
            Configuration::default_file_path()?
        };
        info!(
            "configuration file path is {config_file_path:?}, exists: {}, overwrite: {}",
            config_file_path.is_file(),
            self.overwrite
        );
        if config_file_path.is_file() && !self.overwrite {
            config_file_exists(config_file_path).print();
            Ok(ExitCode::FAILURE)
        } else if self.is_interactive {
            match self.interactive(config_file_path) {
                Ok(code) => Ok(code),
                Err(CliError::Interactive(InquireError::OperationCanceled))
                | Err(CliError::Interactive(InquireError::OperationInterrupted)) => {
                    println!();
                    println!("Command cancelled by user.");
                    Ok(ExitCode::SUCCESS)
                }
                Err(e) => Err(e),
            }
        } else {
            self.immediate(config_file_path)
        }
    }
}

impl InitializeConfig {
    #[allow(clippy::too_many_arguments)] // TODO: factor out to a From<CliCommand> ...
    pub fn new<P: Into<PathBuf>>(
        is_interactive: bool,
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
            is_interactive,
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

    pub fn immediate(self, config_file_path: PathBuf) -> Result<ExitCode, CliError> {
        let ip_lookup = get_default_provider()?;
        let location = ip_lookup.lookup()?;
        println!("Looks like your location is {location:?}");
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
            .map_err(|e: ConfigError| CliError::from(e))
    }

    pub fn interactive(self, config_file_path: PathBuf) -> Result<ExitCode, CliError> {
        println!();
        println!(
            "Hi {}, let's build a new configuration together ...",
            self.callsign
        );
        println!();
        let render_cfg: RenderConfig = RenderConfig::default()
            .with_prompt_prefix(
                Styled::new("?")
                    .with_fg(Color::DarkRed)
                    .with_attr(Attributes::BOLD),
            )
            .with_answered_prompt_prefix(Styled::new("✓").with_fg(Color::DarkGreen))
            .with_highlighted_option_prefix(Styled::new("❯").with_attr(Attributes::BOLD));

        let name_default = whoami::realname().ok().unwrap_or_default();
        let operator_name: String = Text::new("What is your name?")
            .with_render_config(render_cfg)
            .with_default(name_default.as_str())
            .prompt()?;

        let location = Location::new(LocationKind::Home);

        let default_country_code: Option<String> = if let Some(country) = self.country {
            Some(country.to_string())
        } else if let Some(ItuSeriesAllocation::Country(country_code)) =
            ItuSeriesAllocation::from_callsign(&self.callsign)
        {
            println!("⊢ Using the country code derived from your callsign: '{country_code}'");
            Some(country_code.to_string())
        } else {
            let service = get_default_provider()?;
            if let Ok(geo) = service.lookup() {
                println!(
                    "⊢ Using the country code derived from your IP address: '{}'",
                    geo.country()
                );
                Some(geo.country().code().to_string())
            } else if let Some(locale_string) = get_locale() {
                if let Ok(locale) = OsLocale::from_str(&locale_string)
                    && locale.id.region.is_some()
                {
                    let region = locale.id.region.unwrap();
                    if CountryCode::is_known_country(region.as_str()) {
                        println!(
                            "⊢ Using the country code included in your system locale: '{locale_string}'"
                        );
                        Some(region.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        let country: String = Text::new("What country are you in?")
            .with_render_config(render_cfg)
            .with_validator(country_code_validator)
            .with_default(&default_country_code.unwrap_or_default())
            .prompt()?;
        let location = location.with_country(Some(CountryCode::from_str(&country)?));

        let default_locator: Option<String> = if let Some(locator) = &self.locator {
            Some(locator.to_string())
        } else {
            let service = get_default_provider()?;
            if let Ok(geo) = service.lookup() {
                println!(
                    "⊢ Using the latitude/logitude derived from your IP address: '{}'",
                    geo.geo().point()
                );
                MaidenheadLocator::try_from(geo.geo().point())
                    .ok()
                    .map(|v| v.to_string())
            } else {
                None
            }
        };
        let locator: String = Text::new("What is your grid square?")
            .with_render_config(render_cfg)
            .with_validator(maidenhead_validator)
            .with_default(&default_locator.unwrap_or_default())
            .prompt()?;
        let location = location.with_grid_locator(Some(MaidenheadLocator::from_str(&locator)?));

        let location = if Confirm::new("Do you want to add a mailing address?")
            .with_render_config(render_cfg)
            .with_default(false)
            .prompt()?
        {
            let address: String = Text::new("Address?")
                .with_render_config(render_cfg)
                .prompt()?;
            location.with_mailing_address(Some(address))
        } else {
            location
        };

        let station = Station::new(self.callsign.clone()).with_location(Some(location));
        let station = station.with_operator_name(Some(operator_name));

        let config_locale = if Confirm::new("Do you want to set locale-specific defaults?")
            .with_render_config(render_cfg)
            .with_default(false)
            .prompt()?
        {
            let locale = Locale::default();

            let options = Units::iter().map(|v| v.to_string()).collect::<Vec<_>>();
            let units = Select::new("Use which units for length?", options.clone())
                .with_render_config(render_cfg)
                .prompt()?;
            let locale = locale.with_length_units(Units::from_str(&units).unwrap());

            let units = Select::new("Use which units for temperature?", options)
                .with_render_config(render_cfg)
                .prompt()?;
            let locale = locale.with_temperature_units(Units::from_str(&units).unwrap());

            let options = TimeFormat::iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>();
            let format = Select::new("Use which format for time?", options.clone())
                .with_render_config(render_cfg)
                .prompt()?;
            let locale = locale.with_time_format(TimeFormat::from_str(&format).unwrap());

            // language tag

            Some(locale)
        } else {
            None
        };

        if Confirm::new("Do you want to add any equipment records?")
            .with_render_config(render_cfg)
            .with_default(false)
            .prompt()?
        {}

        let services: Option<Services> =
            if Confirm::new("Do you want to connect to any web services?")
                .with_render_config(render_cfg)
                .with_default(false)
                .prompt()?
            {
                let mut services = Services::new(CredentialStorageKind::default());
                if Confirm::new("Do you have an account on qrz.com for callsign lookup?")
                    .with_render_config(render_cfg)
                    .with_default(false)
                    .prompt()?
                {
                    let user_name: String = Text::new("Qrz user name?")
                        .with_render_config(render_cfg)
                        .with_default(&self.callsign.to_string())
                        .prompt()?;

                    let password = Password::new("Password:")
                        .with_render_config(render_cfg)
                        .with_display_toggle_enabled()
                        .with_display_mode(PasswordDisplayMode::Hidden)
                        .prompt()?;

                    services.set_credentials(
                        CallSignInfoProvider::Qrz.into(),
                        Credentials::new(user_name, password),
                    )?;
                }
                Some(services)
            } else {
                None
            };

        let write_config = Confirm::new("Are you sure you wish to write this configuration?")
            .with_render_config(render_cfg)
            .with_default(true)
            .prompt()?;
        if write_config {
            let mut config = Configuration::default()
                .with_path(Some(config_file_path.clone()))
                .with_station(Some(station))
                .with_locale(config_locale)
                .with_services(services.unwrap_or_default());

            info!("about to write config {config:?}");
            config
                .save(self.overwrite)
                .map(|()| ExitCode::SUCCESS)
                .map_err(|e: ConfigError| CliError::from(e))?;

            println!("✓ Configuration file saved as {config_file_path:?}");
        } else {
            info!("config write cancelled");
        }
        Ok(ExitCode::SUCCESS)
    }
}

// ------------------------------------------------------------------------------------------------
// Validators
// ------------------------------------------------------------------------------------------------

fn country_code_validator(input: &str) -> Result<Validation, CustomUserError> {
    if input.len() != 2 {
        Ok(Validation::Invalid(
            "Country codes must be 2 characters only".into(),
        ))
    } else if !input.chars().all(|c| c.is_ascii_alphabetic()) {
        Ok(Validation::Invalid(
            "Country codes must be ASCII alphabetic characters only".into(),
        ))
    } else if !CountryCode::is_known_country(input) {
        Ok(Validation::Invalid(
            "Input is not a known country code".into(),
        ))
    } else {
        Ok(Validation::Valid)
    }
}

fn maidenhead_validator(input: &str) -> Result<Validation, CustomUserError> {
    if input.len() < 4 {
        Ok(Validation::Invalid(
            "Grid locators must be at least 4 characters".into(),
        ))
    } else if !MaidenheadLocator::is_valid(input) {
        Ok(Validation::Invalid(
            "Input is not a valid Grid locator".into(),
        ))
    } else {
        Ok(Validation::Valid)
    }
}

// TODO: fn map_user_cancelled(e: InquireError) -> CliError
