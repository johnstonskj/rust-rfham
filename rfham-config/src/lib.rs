//! All configuration types for RF-Ham.
//!
//! [`Configuration`] is the root serialisable type, stored as TOML in the user's
//! XDG/platform config directory under `rfham/rfham-config.toml`.
//!
//! # Examples
//!
//! ```rust,no_run
//! use rfham_config::Configuration;
//!
//! let config = Configuration::load().unwrap_or_default();
//! if let Some(station) = config.station() {
//!     println!("Callsign: {}", station.callsign());
//! }
//! ```

use crate::{
    error::{ConfigError, ConfigResult},
    fields::{
        CFG_FIELD_EQUIPMENT, CFG_FIELD_LANGUAGE, CFG_FIELD_LENGTH_UNITS, CFG_FIELD_LOCALE,
        CFG_FIELD_PATH, CFG_FIELD_SERVICES, CFG_FIELD_STATION, CFG_FIELD_TEMPERATURE_UNITS,
        CFG_FIELD_TIME_DISPLAY,
    },
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathTarget, Value},
};
use language_tags::LanguageTag;
use rfham_core::{StringLike, fmt::FormattedWriter, names::Name};
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, RwLock, RwLockReadGuard},
};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use tracing::trace;

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Configuration {
    #[serde(skip)]
    path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    station: Option<Station>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    locale: Option<Locale>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    equipment: Vec<Equipment>,
    #[serde(default)]
    services: Services,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Locale {
    length_units: Units,
    temperature_units: Units,
    time_format: TimeFormat,
    language: Option<LanguageTag>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    EnumDisplay,
    EnumIs,
    EnumString,
    EnumIter,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Units {
    #[strum(serialize = "metric")]
    #[default]
    Metric,
    #[strum(serialize = "imperial")]
    Imperial,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    EnumDisplay,
    EnumIs,
    EnumString,
    EnumIter,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum TimeFormat {
    #[strum(serialize = "am-pm")]
    AmPm,
    #[strum(serialize = "military")]
    #[default]
    Military,
}

pub const CONFIG_DIR_NAME: &str = "rfham";
pub const CONFIG_FILE_NAME: &str = "rfham-config.toml";

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Functions
// ────────────────────────────────────────────────────────────────────────────────────────────────

static SHARED_CONFIG: LazyLock<Arc<RwLock<Configuration>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Configuration::default())));

pub fn load_global_config() -> Result<(), ConfigError> {
    set_global_config(Configuration::load()?)
}

pub fn set_global_config(config: Configuration) -> Result<(), ConfigError> {
    let mut write_lock = SHARED_CONFIG
        .write()
        .map_err(|e| ConfigError::LockPoison(e.to_string()))?;
    *write_lock = config;
    Ok(())
}

pub fn get_global_config() -> Result<RwLockReadGuard<'static, Configuration>, ConfigError> {
    Ok(SHARED_CONFIG
        .read()
        .map_err(|e| ConfigError::LockPoison(e.to_string()))?)
}

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Implementations
// ────────────────────────────────────────────────────────────────────────────────────────────────

impl FormattedWriter for Configuration {
    type Options = FormatterOptions;
    type Error = ConfigError;

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        match options.output_kind() {
            OutputKind::MarkdownList => {
                header(writer, options.nesting_depth(), "Current Configuration")?;
                blank_line(writer)?;
                if let Some(path) = &self.path {
                    bulleted_list_item(writer, 1, format!("Path to file: {path:?}"))?;
                    blank_line(writer)?;
                }
                if let Some(locale) = &self.locale {
                    locale.write_with(writer, &options.with_additional_depth(1))?;
                }
                if let Some(station) = &self.station {
                    station.write_with(writer, &options.with_additional_depth(1))?;
                }
                if !self.equipment.is_empty() {
                    blank_line(writer)?;
                    header(writer, options.nesting_depth() + 1, "Equipment")?;
                    blank_line(writer)?;
                    for equipment in &self.equipment {
                        equipment.write_with(writer, &options.with_additional_depth(2))?;
                    }
                }
                self.services
                    .write_with(writer, &options.with_additional_depth(1))?;
            }
            OutputKind::MarkdownTable => {
                todo!()
            }
            OutputKind::Toml => writer.write_all(toml::to_string_pretty(self)?.as_bytes())?,
        }
        Ok(())
    }
}

const CFG_FILE_ROOT: &str = "<<root>>";

impl PathTarget for Configuration {
    fn path_name(&self) -> Option<Name> {
        None
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let (head, tail) = path.split();
        match head.as_str() {
            name if name == CFG_FIELD_PATH => {
                if let Some(path) = &self.path {
                    Ok(Value::Path(path.clone()))
                } else {
                    Ok(Value::None)
                }
            }
            name if name == CFG_FIELD_LOCALE => {
                if let Some(locale) = &self.locale
                    && tail.is_some()
                {
                    locale.value(tail.as_ref().unwrap())
                } else if self.locale.is_some() && tail.is_none() {
                    Err(ConfigError::PathTooShort(
                        head.to_string(),
                        CFG_FILE_ROOT,
                        self.locale.as_ref().unwrap().value_names().collect(),
                    ))
                } else {
                    Ok(Value::None)
                }
            }
            name if name == CFG_FIELD_STATION => {
                if let Some(station) = &self.station
                    && tail.is_some()
                {
                    station.value(tail.as_ref().unwrap())
                } else if self.station.is_some() && tail.is_none() {
                    Err(ConfigError::PathTooShort(
                        head.to_string(),
                        CFG_FILE_ROOT,
                        self.station.as_ref().unwrap().value_names().collect(),
                    ))
                } else {
                    Ok(Value::None)
                }
            }
            name if name == CFG_FIELD_EQUIPMENT => {
                if !self.equipment.is_empty() && tail.is_some() {
                    // parse head as usize
                    todo!()
                } else if !self.equipment.is_empty() && tail.is_none() {
                    Err(ConfigError::PathTooShort(
                        head.to_string(),
                        CFG_FILE_ROOT,
                        vec![], // TODO: this needs to be static
                    ))
                } else {
                    Ok(Value::None)
                }
            }
            name if name == CFG_FIELD_SERVICES => {
                if tail.is_some() {
                    self.services.value(tail.as_ref().unwrap())
                } else {
                    Err(ConfigError::PathTooShort(
                        head.to_string(),
                        CFG_FILE_ROOT,
                        self.services.value_names().collect(),
                    ))
                }
            }
            _ => Err(ConfigError::InvalidPathComponent(
                head.to_string(),
                CFG_FILE_ROOT,
                self.value_names().collect(),
            )),
        }
    }

    fn value_names(&self) -> impl Iterator<Item = &'static str> {
        [
            CFG_FIELD_PATH,
            CFG_FIELD_LOCALE,
            CFG_FIELD_STATION,
            CFG_FIELD_EQUIPMENT,
            CFG_FIELD_SERVICES,
        ]
        .into_iter()
    }
}

impl Configuration {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn with_path<P: Into<PathBuf>>(mut self, path: Option<P>) -> Self {
        self.path = path.map(|p| p.into());
        self
    }

    pub fn with_locale(mut self, locale: Option<Locale>) -> Self {
        self.locale = locale;
        self
    }

    pub fn with_station(mut self, station: Option<Station>) -> Self {
        self.station = station;
        self
    }

    pub fn with_equipment(mut self, equipment: Vec<Equipment>) -> Self {
        self.equipment = equipment;
        self
    }

    pub fn with_services(mut self, services: Services) -> Self {
        self.services = services;
        self
    }

    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Field accessors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn set_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.path = Some(path.into())
    }

    pub fn unset_path(&mut self) {
        self.path = None
    }

    pub fn locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.locale = Some(locale)
    }

    pub fn unset_locale(&mut self) {
        self.locale = None
    }

    pub fn station(&self) -> Option<&Station> {
        self.station.as_ref()
    }

    pub fn set_station(&mut self, station: Station) {
        self.station = Some(station)
    }

    pub fn unset_station(&mut self) {
        self.station = None
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut Services {
        &mut self.services
    }

    pub fn set_services(&mut self, services: Services) {
        self.services = services
    }

    pub fn exists(&self) -> bool {
        Self::default_file_path()
            .map(|path| path.exists())
            .unwrap_or_default()
    }

    pub fn default_file_path() -> ConfigResult<PathBuf> {
        Ok(xdirs::config_dir_for(CONFIG_DIR_NAME)
            .ok_or(ConfigError::ConfigDir)?
            .join(CONFIG_FILE_NAME))
    }

    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // File I/O
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn load() -> ConfigResult<Self> {
        Self::load_from(Self::default_file_path()?)
    }

    pub fn load_from<P: AsRef<Path>>(path: P) -> ConfigResult<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        let mut config: Configuration = toml::from_str(&content)?;
        config.set_path(PathBuf::from(path.as_ref()));
        Ok(config)
    }

    pub fn save_to<P: AsRef<Path>>(&mut self, path: P, overwrite: bool) -> ConfigResult<()> {
        self.path = Some(PathBuf::from(path.as_ref()));
        let parent_dir = self.path.as_ref().map(|p| p.parent()).unwrap_or_default();
        if let Some(actual_parent_dir) = parent_dir
            && !actual_parent_dir.is_dir()
        {
            trace!("creating parent directory for config file");
            create_dir_all(actual_parent_dir)?;
        }
        let mut file = if overwrite {
            File::create(path)
        } else {
            File::create_new(path)
        }?;
        let content = toml::to_string_pretty(self)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn save(&mut self, overwrite: bool) -> ConfigResult<()> {
        let path = if let Some(path) = &self.path {
            path.clone()
        } else {
            Self::default_file_path()?
        };
        self.save_to(path, overwrite)
    }
}

// ────────────────────────────────────────────────────────────────────────────────────────────────

impl FormattedWriter for Locale {
    type Options = FormatterOptions;
    type Error = ConfigError;

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        match options.output_kind() {
            OutputKind::MarkdownList => {
                header(writer, options.nesting_depth(), "Locale")?;
                blank_line(writer)?;

                bulleted_list_item(writer, 1, format!("Length Units: {}", self.length_units))?;
                bulleted_list_item(
                    writer,
                    1,
                    format!("Temperature Units: {}", self.temperature_units),
                )?;
                bulleted_list_item(writer, 1, format!("Time Format: {}", self.time_format))?;

                if let Some(language) = &self.language {
                    bulleted_list_item(writer, 1, format!("Language: {}", language))?;
                }
            }
            OutputKind::MarkdownTable => {
                todo!()
            }
            OutputKind::Toml => writer.write_all(toml::to_string_pretty(self)?.as_bytes())?,
        }
        Ok(())
    }
}

impl PathTarget for Locale {
    fn path_name(&self) -> Option<Name> {
        Some(Name::new_unchecked(CFG_FIELD_LOCALE))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let head = path.head();
        match head.as_str() {
            name if name == CFG_FIELD_LENGTH_UNITS => {
                Ok(Value::EnumValue(self.length_units.to_string()))
            }
            name if name == CFG_FIELD_TEMPERATURE_UNITS => {
                Ok(Value::EnumValue(self.temperature_units.to_string()))
            }
            name if name == CFG_FIELD_TIME_DISPLAY => {
                Ok(Value::EnumValue(self.time_format.to_string()))
            }
            name if name == CFG_FIELD_LANGUAGE => {
                if let Some(language) = &self.language {
                    Ok(Value::String(language.to_string()))
                } else {
                    Ok(Value::None)
                }
            }
            _ => Err(ConfigError::InvalidPathComponent(
                head.to_string(),
                CFG_FIELD_LOCALE,
                self.value_names().collect(),
            )),
        }
    }

    fn value_names(&self) -> impl Iterator<Item = &'static str> {
        [
            CFG_FIELD_LENGTH_UNITS,
            CFG_FIELD_TEMPERATURE_UNITS,
            CFG_FIELD_TIME_DISPLAY,
            CFG_FIELD_EQUIPMENT,
            CFG_FIELD_LANGUAGE,
        ]
        .into_iter()
    }
}

impl Locale {
    pub fn with_length_units(mut self, length_units: Units) -> Self {
        self.length_units = length_units;
        self
    }

    pub fn with_temperature_units(mut self, temperature_units: Units) -> Self {
        self.temperature_units = temperature_units;
        self
    }

    pub fn with_time_format(mut self, time_format: TimeFormat) -> Self {
        self.time_format = time_format;
        self
    }

    pub fn with_language(mut self, language: Option<LanguageTag>) -> Self {
        self.language = language;
        self
    }
}

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Sub-modules
// ────────────────────────────────────────────────────────────────────────────────────────────────

pub mod equipment;
pub use equipment::Equipment;
pub mod error;
pub mod fields;
pub mod fmt;
pub mod locations;
pub use locations::{Location, LocationKind};
pub mod paths;
pub mod services;
pub use services::Services;
pub mod stations;
pub use stations::Station;

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Unit Tests
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use crate::{
        Configuration,
        paths::{ConfigPath, PathTarget, Value},
        stations::Station,
    };
    use pretty_assertions::assert_eq;
    use rfham_core::{StringLike, callsigns::CallSign, names::Name};
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_toml_round_trip() {
        let mut config = Configuration::default()
            .with_path(Some("/path/to/config.toml"))
            .with_station(Some(Station::new(CallSign::from_str("N0CALL").unwrap())));
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialised_config: Configuration = toml::from_str(&toml_str).unwrap();
        // The below is necessary because Serde will not serialize the path.
        config.unset_path();
        assert_eq!(config, deserialised_config);
    }

    #[test]
    fn test_config_pathto_path() {
        let mut config = Configuration::default();
        assert_eq!(
            Value::None,
            config
                .value(&ConfigPath::from(Name::new_unchecked("path")))
                .unwrap()
        );

        config.set_path("/path/to/config.toml");
        assert_eq!(
            Value::Path(PathBuf::from("/path/to/config.toml")),
            config
                .value(&ConfigPath::from(Name::new_unchecked("path")))
                .unwrap()
        );
    }
}
