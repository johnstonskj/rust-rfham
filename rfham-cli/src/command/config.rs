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
    Confirm, InquireError, MultiSelect, Password, PasswordDisplayMode, Select, Text,
    error::CustomUserError,
    ui::{Attributes, Color, RenderConfig, Styled},
    validator::Validation,
};
use rfham_config::{
    Configuration, Equipment, Locale, Location, Services, Station, TimeFormat, Units,
    equipment::{Mobility, Mode, Usage},
    error::ConfigError,
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathTarget, Value},
    services::{CredentialStorageKind, Credentials},
    stations::StationKind,
};
use rfham_core::{
    callsigns::CallSign, countries::CountryCode, error::CoreError, fmt::FormattedWriter,
    power::Power,
};
use rfham_geo::grid::GridIdentifier;
use rfham_itu::{bands::FrequencyBand, callsigns::ItuSeriesAllocation, regions::Region};
use rfham_maidenhead::MaidenheadLocator;
use rfham_services::{callsign::CallSignInfoProvider, location::get_default_provider};
use std::{collections::HashSet, io::stdout, path::PathBuf, process::ExitCode, str::FromStr};
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
                        println!("field {} is not set", config_path.last());
                        Ok(ExitCode::SUCCESS)
                    }
                    Ok(value) => {
                        if self.compact_output {
                            println!(
                                "{}{} {} {} {}",
                                config_path.last(),
                                ":".dimmed(),
                                value.type_label().italic(),
                                "=".dimmed(),
                                value.to_string().bold()
                            );
                        } else {
                            println!("field: {}", config_path.last());
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
        let location = Location::default()
            .with_grid_locator(self.locator)
            .with_itu_region(self.itu_region)
            .with_country(self.country)
            .with_mailing_address(self.mailing_address);
        let station = Station::new(StationKind::Home, location);
        let mut config = Configuration::new(self.callsign)
            .with_path(Some(config_file_path))
            .with_operator_name(self.operator_name)
            .with_station(station);
        info!("about to write config {config:?}");
        config
            .save(self.overwrite)
            .map(|()| ExitCode::SUCCESS)
            .map_err(|e: ConfigError| CliError::from(e))
    }

    pub fn interactive(self, config_file_path: PathBuf) -> Result<ExitCode, CliError> {
        heading(
            1,
            &format!(
                "Hi {}, let's build a new configuration together ...",
                self.callsign
            ),
        );

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

        heading(2, "Now, let's talk about your station ...");

        let options = StationKind::iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        let station_kind = Select::new(
            "What kind of station should be your default?",
            options.clone(),
        )
        .with_render_config(render_cfg)
        .prompt()?;
        let station_kind = StationKind::from_str(&station_kind)?;

        let station_label: Option<String> = match Text::new("Label this station (esc to skip)?")
            .with_render_config(render_cfg)
            .with_default(station_kind.label())
            .prompt()
        {
            Ok(label) => Some(label),
            Err(InquireError::OperationCanceled) => None,
            Err(e) => return Err(CliError::from(e)),
        };

        let station_callsign: Option<CallSign> =
            match Text::new("Stattion-specific callsign (esc to use operator callsign)?")
                .with_render_config(render_cfg)
                .with_default(station_kind.label())
                .prompt()
            {
                Ok(callsign) => Some(CallSign::from_str(&callsign)?),
                Err(InquireError::OperationCanceled) => None,
                Err(e) => return Err(CliError::from(e)),
            };

        let location = Location::default();

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

        let station = Station::new(station_kind, location)
            .with_label(station_label)
            .with_callsign(station_callsign);

        heading(2, "Next, we can set some locale-specific defaults ...");

        let config_locale = if Confirm::new("Do you want to set locale-specific defaults?")
            .with_render_config(render_cfg)
            .with_default(false)
            .prompt()?
        {
            let locale = Locale::default();

            let options = Units::iter().map(|v| v.to_string()).collect::<Vec<_>>();
            let units = Select::new("Use which units for length?", options.clone())
                .with_render_config(render_cfg)
                .with_starting_cursor(if country == "US" || country == "GB" {
                    1
                } else {
                    0
                })
                .prompt()?;
            let locale = locale.with_length_units(Units::from_str(&units).unwrap());

            let units = Select::new("Use which units for temperature?", options)
                .with_render_config(render_cfg)
                .with_starting_cursor(if country == "US" || country == "GB" {
                    1
                } else {
                    0
                })
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

        heading(2, "Finally, we can set some optional connections ...");

        let mut equipment = Vec::default();
        if Confirm::new("Do you want to add any equipment records?")
            .with_render_config(render_cfg)
            .with_default(false)
            .prompt()?
        {
            loop {
                let brand: String = Text::new("Brand Name?")
                    .with_render_config(render_cfg)
                    .prompt()?;
                let model: String = Text::new("Model Name/ID?")
                    .with_render_config(render_cfg)
                    .prompt()?;
                let label_default = format!("{brand} {model}");

                let record = Equipment::new(brand, model);

                let record = match Text::new("Label this record (esc to skip)?")
                    .with_render_config(render_cfg)
                    .with_default(&label_default)
                    .prompt()
                {
                    Ok(label) => record.with_label(Some(label)),
                    Err(InquireError::OperationCanceled) => record,
                    Err(e) => return Err(CliError::from(e)),
                };

                let options = Usage::iter().map(|v| v.to_string()).collect::<Vec<_>>();
                let usage = MultiSelect::new("Used in?", options.clone())
                    .with_render_config(render_cfg)
                    .prompt()?;
                let record = if !usage.is_empty() {
                    let usage: Result<HashSet<Usage>, strum::ParseError> =
                        usage.iter().map(|s| Usage::from_str(s)).collect();
                    record.with_usage(usage?)
                } else {
                    record
                };

                let options = Mode::iter().map(|v| v.to_string()).collect::<Vec<_>>();
                let modes = MultiSelect::new("Used for?", options.clone())
                    .with_render_config(render_cfg)
                    .prompt()?;
                let record = if !modes.is_empty() {
                    let modes: Result<HashSet<Mode>, strum::ParseError> =
                        modes.iter().map(|s| Mode::from_str(s)).collect();
                    record.with_modes(modes?)
                } else {
                    record
                };

                let options = Mobility::iter().map(|v| v.to_string()).collect::<Vec<_>>();
                let mobility = Select::new("Mobility kind?", options.clone())
                    .with_render_config(render_cfg)
                    .prompt()?;
                let mobility = Some(Mobility::from_str(&mobility)?);
                let record = record.with_mobility(mobility);

                let options = FrequencyBand::iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>();
                let bands = MultiSelect::new("Supported Bands?", options.clone())
                    .with_render_config(render_cfg)
                    .with_starting_cursor(FrequencyBand::Medium as usize)
                    .prompt()?;
                let record = if !bands.is_empty() {
                    let bands: Result<HashSet<FrequencyBand>, CoreError> =
                        bands.iter().map(|s| FrequencyBand::from_str(s)).collect();
                    record.with_bands(bands?)
                } else {
                    record
                };

                let record = match Text::new("Max Power in Watts (esc to skip)?")
                    .with_render_config(render_cfg)
                    .with_validator(power_validator)
                    .prompt()
                {
                    Ok(power) => {
                        record.with_max_power(Some(Power::from_str(&format!("{power} W"))?))
                    }
                    Err(InquireError::OperationCanceled) => record,
                    Err(e) => return Err(CliError::from(e)),
                };

                equipment.push(record);

                if !Confirm::new("Add another?")
                    .with_render_config(render_cfg)
                    .with_default(false)
                    .prompt()?
                {
                    break;
                }
            }
        }

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
            let mut config = Configuration::new(self.callsign)
                .with_path(Some(config_file_path.clone()))
                .with_operator_name(Some(operator_name))
                .with_station(station)
                .with_locale(config_locale)
                .with_services(services.unwrap_or_default())
                .with_equipment(equipment);

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

fn power_validator(input: &str) -> Result<Validation, CustomUserError> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() > 2 {
        Ok(Validation::Invalid(
            "Power values may only have a single decimal point".into(),
        ))
    } else if parts.len() == 2 && parts[0].is_empty() {
        Ok(Validation::Invalid(
            "Power values must have digits before the decimal point".into(),
        ))
    } else if !input.chars().all(|c| c.is_ascii_digit() || c == '.') {
        Ok(Validation::Invalid(
            "Power values may only be numeric".into(),
        ))
    } else {
        Ok(Validation::Valid)
    }
}

// TODO: fn map_user_cancelled(e: InquireError) -> CliError

fn heading(level: u8, text: &str) {
    let text = text.bright_blue();
    let text = match level {
        1 => text.bold().underline(),
        2 => text.bold().italic(),
        3 => text.bold(),
        _ => text,
    };
    println!("\n{text}\n");
}
