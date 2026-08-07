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
//! let config = Configuration::load().unwrap();
//! println!("Callsign: {}", config.callsign());
//! ```

use crate::{
    connections::Connection,
    error::{ConfigError, ConfigResult},
    fields::{
        CFG_FIELD_CALLSIGN, CFG_FIELD_CONNECTION, CFG_FIELD_CONNECTIONS, CFG_FIELD_EQUIPMENT,
        CFG_FIELD_LANGUAGE, CFG_FIELD_LENGTH_UNITS, CFG_FIELD_LOCALE, CFG_FIELD_NAME,
        CFG_FIELD_OPERATOR_NAME, CFG_FIELD_PATH, CFG_FIELD_SERVICES, CFG_FIELD_STATION,
        CFG_FIELD_TEMPERATURE_UNITS, CFG_FIELD_TIME_DISPLAY,
    },
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathElement, PathTarget, Value},
};
use language_tags::LanguageTag;
use rfham_core::{StringLike, callsigns::CallSign, fmt::FormattedWriter, names::Name};
use rfham_itu::callsigns::ItuSeriesAllocation;
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    collections::HashMap,
    fs::{self, File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, LazyLock, RwLock, RwLockReadGuard},
};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use tracing::trace;

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Configuration {
    #[serde(skip)]
    path: Option<PathBuf>,
    callsign: CallSign,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    operator_name: Option<String>,
    default_station: usize,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    stations: Vec<Station>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    locale: Option<Locale>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    equipment: Vec<Equipment>,
    #[serde(default)]
    services: Services,
    #[serde(default)]
    connections: HashMap<String, Connection>,
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
    #[strum(serialize = "military")]
    #[default]
    Military,
    #[strum(serialize = "am-pm")]
    AmPm,
}

pub const CONFIG_DIR_NAME: &str = "rfham";
pub const CONFIG_FILE_NAME: &str = "rfham-config.toml";

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Functions
// ────────────────────────────────────────────────────────────────────────────────────────────────

const DEFAULT_FAKE_CALLSIGN: &str = "N0CALL";

static SHARED_CONFIG: LazyLock<Arc<RwLock<Configuration>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(Configuration::new(
        CallSign::from_str(DEFAULT_FAKE_CALLSIGN).expect("That is unexpected"),
    )))
});

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
                blank_line(writer)?;
                header(writer, options.nesting_depth(), "Current Configuration")?;
                blank_line(writer)?;
                if let Some(path) = &self.path {
                    bulleted_list_item(writer, 1, format!("Path to file: {path:?}"))?;
                    blank_line(writer)?;
                }
                bulleted_list_item(writer, 1, format!("Operator callsign: {}", self.callsign))?;

                if let Some(allocation) = ItuSeriesAllocation::from_callsign(&self.callsign) {
                    bulleted_list_item(
                        writer,
                        2,
                        format!("Callsign's ITU allocation; {allocation:#}"),
                    )?;
                }

                if let Some(operator_name) = &self.operator_name {
                    bulleted_list_item(writer, 1, format!("Operator name: {operator_name}"))?;
                }

                if let Some(locale) = &self.locale {
                    locale.write_with(writer, &options.with_additional_depth(1))?;
                }

                for (i, station) in self.stations().enumerate() {
                    if i == self.default_station {
                        station.write_with(
                            writer,
                            &options.with_additional_depth(1).with_flag_as_default(true),
                        )?;
                    } else {
                        station.write_with(writer, &options.with_additional_depth(1))?;
                    }
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

                if !self.connections.is_empty() {
                    blank_line(writer)?;
                    header(writer, options.nesting_depth() + 1, "Rig Connections")?;
                    blank_line(writer)?;
                    for (name, conn) in &self.connections {
                        header(
                            writer,
                            options.nesting_depth() + 2,
                            format!("{name} ({})", if conn.is_ip() { "IP" } else { "Serial" }),
                        )?;
                        conn.write_with(writer, &options.with_additional_depth(3))?;
                    }
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

const CFG_FILE_ROOT: &str = "<<root>>";

impl PathTarget for Configuration {
    fn path_name() -> Option<Name> {
        None
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let (head, tail) = path.split();
        if let PathElement::Name(head) = head {
            match head.as_str() {
                name if name == CFG_FIELD_PATH => {
                    if let Some(path) = &self.path {
                        Ok(Value::Path(path.clone()))
                    } else {
                        Ok(Value::None)
                    }
                }
                CFG_FIELD_CALLSIGN => Ok(Value::String(self.callsign.to_string())),
                name if name == CFG_FIELD_OPERATOR_NAME => {
                    if let Some(operator_name) = &self.operator_name {
                        Ok(Value::String(operator_name.to_string()))
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
                            Locale::value_names().collect(),
                        ))
                    } else {
                        Ok(Value::None)
                    }
                }
                name if name == CFG_FIELD_STATION => {
                    if let Some(path) = tail {
                        let (head, tail) = path.split();
                        if let PathElement::Index(index) = head {
                            if tail.is_none() {
                                Err(ConfigError::PathTooShort(
                                    head.to_string(),
                                    CFG_FILE_ROOT,
                                    vec![], // TODO: this needs to be static
                                ))
                            } else if let Some(station) = self.stations.get(*index) {
                                station.value(&tail.unwrap())
                            } else {
                                Err(ConfigError::InvalidPathIndex(*index, CFG_FIELD_STATION))
                            }
                        } else {
                            Err(ConfigError::InvalidPathElementIndex(
                                head.to_string(),
                                CFG_FIELD_STATION,
                            ))
                        }
                    } else {
                        Err(ConfigError::PathTooShort(
                            head.to_string(),
                            CFG_FILE_ROOT,
                            vec![], // TODO: this needs to be static
                        ))
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
                            Services::value_names().collect(),
                        ))
                    }
                }
                name if name == CFG_FIELD_CONNECTIONS => {
                    if let Some(path) = tail {
                        let (head, tail) = path.split();
                        if let PathElement::Name(name) = head {
                            if tail.is_none() {
                                Err(ConfigError::PathTooShort(
                                    head.to_string(),
                                    CFG_FIELD_CONNECTION,
                                    vec![], // TODO: this needs to be static
                                ))
                            } else if let Some(connection) = self.connections.get(&name.to_string())
                            {
                                connection.value(&tail.unwrap())
                            } else {
                                Err(ConfigError::InvalidPathComponent(
                                    name.to_string(),
                                    CFG_FIELD_CONNECTION,
                                    vec![],
                                ))
                            }
                        } else {
                            Err(ConfigError::InvalidPathComponent(
                                head.to_string(),
                                CFG_FIELD_NAME,
                                vec![],
                            ))
                        }
                    } else {
                        Err(ConfigError::PathTooShort(
                            head.to_string(),
                            CFG_FIELD_CONNECTIONS,
                            vec![CFG_FIELD_NAME],
                        ))
                    }
                }
                _ => Err(ConfigError::InvalidPathComponent(
                    head.to_string(),
                    CFG_FILE_ROOT,
                    Self::value_names().collect(),
                )),
            }
        } else {
            Err(ConfigError::InvalidPathComponent(
                head.to_string(),
                CFG_FILE_ROOT,
                Self::value_names().collect(),
            ))
        }
    }

    fn value_names() -> impl Iterator<Item = &'static str> {
        [
            CFG_FIELD_PATH,
            CFG_FIELD_LOCALE,
            CFG_FIELD_STATION,
            CFG_FIELD_EQUIPMENT,
            CFG_FIELD_SERVICES,
            CFG_FIELD_CONNECTIONS,
        ]
        .into_iter()
    }
}

impl Configuration {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn new(callsign: CallSign) -> Self {
        Self {
            path: None,
            callsign,
            operator_name: None,
            default_station: 0,
            stations: Vec::default(),
            locale: None,
            equipment: Vec::default(),
            services: Services::default(),
            connections: HashMap::default(),
        }
    }

    pub fn with_path<P: Into<PathBuf>>(mut self, path: Option<P>) -> Self {
        self.path = path.map(|p| p.into());
        self
    }

    pub fn with_operator_name<S: Into<String>>(mut self, operator_name: Option<S>) -> Self {
        self.operator_name = operator_name.map(|s| s.into());
        self
    }

    pub fn with_locale(mut self, locale: Option<Locale>) -> Self {
        self.locale = locale;
        self
    }

    pub fn with_station(self, station: Station) -> Self {
        self.with_stations(vec![station])
    }

    pub fn with_stations<I>(mut self, stations: I) -> Self
    where
        I: IntoIterator<Item = Station>,
    {
        let stations: Vec<Station> = stations.into_iter().collect();
        assert!(!stations.is_empty());
        self.stations = stations;
        self
    }

    pub fn with_equipment<I>(mut self, equipment: I) -> Self
    where
        I: IntoIterator<Item = Equipment>,
    {
        self.equipment = Vec::from_iter(equipment.into_iter());
        self
    }

    pub fn with_services(mut self, services: Services) -> Self {
        self.services = services;
        self
    }

    pub fn with_connections<I>(mut self, connections: I) -> Self
    where
        I: IntoIterator<Item = (String, Connection)>,
    {
        self.connections = HashMap::from_iter(connections.into_iter());
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

    pub const fn callsign(&self) -> &CallSign {
        &self.callsign
    }

    pub fn set_callsign(&mut self, callsign: CallSign) {
        self.callsign = callsign
    }

    pub const fn operator_name(&self) -> Option<&String> {
        self.operator_name.as_ref()
    }

    pub fn set_operator_name<S: Into<String>>(&mut self, operator_name: S) {
        self.operator_name = Some(operator_name.into())
    }

    pub fn unset_operator_name(&mut self) {
        self.operator_name = None
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

    pub fn stations(&self) -> impl Iterator<Item = &Station> {
        self.stations.iter()
    }

    pub fn add_station(&mut self, station: Station) {
        self.stations.push(station);
    }

    pub fn set_stations<I>(&mut self, stations: I)
    where
        I: IntoIterator<Item = Station>,
    {
        let stations: Vec<Station> = stations.into_iter().collect();
        assert!(!stations.is_empty());
        self.stations = stations;
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

    pub fn connections(&self) -> impl Iterator<Item = (&String, &Connection)> {
        self.connections.iter()
    }

    pub fn connection(&self, name: &String) -> Option<&Connection> {
        self.connections.get(name)
    }

    pub fn connection_names(&self) -> impl Iterator<Item = &String> {
        self.connections.keys()
    }

    pub fn add_connection(&mut self, name: String, connection: Connection) {
        self.connections.insert(name, connection);
    }

    pub fn set_connections<I: IntoIterator<Item = (String, Connection)>>(
        &mut self,
        connections: I,
    ) {
        self.connections = HashMap::from_iter(connections.into_iter());
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
                blank_line(writer)?;
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
    fn path_name() -> Option<Name> {
        Some(Name::new_unchecked(CFG_FIELD_LOCALE))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let head = path.head();
        if let PathElement::Name(name) = head {
            match name.as_str() {
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
                    Self::value_names().collect(),
                )),
            }
        } else {
            Err(ConfigError::InvalidPathElementName(
                head.to_string(),
                CFG_FIELD_LOCALE,
            ))
        }
    }

    fn value_names() -> impl Iterator<Item = &'static str> {
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

pub mod connections;
pub mod equipment;
pub use equipment::Equipment;
pub mod error;
pub mod fields;
pub mod fmt;
pub mod locations;
pub use locations::Location;
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
        Configuration, DEFAULT_FAKE_CALLSIGN,
        paths::{ConfigPath, PathTarget, Value},
    };
    use pretty_assertions::assert_eq;
    use rfham_core::{StringLike, callsigns::CallSign, names::Name};
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_toml_round_trip() {
        let mut config = Configuration::new(CallSign::from_str(DEFAULT_FAKE_CALLSIGN).unwrap())
            .with_path(Some("/path/to/config.toml"));
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialised_config: Configuration = toml::from_str(&toml_str).unwrap();
        // The below is necessary because Serde will not serialize the path.
        config.unset_path();
        assert_eq!(config, deserialised_config);
    }

    #[test]
    fn test_config_pathto_path() {
        let mut config = Configuration::new(CallSign::from_str(DEFAULT_FAKE_CALLSIGN).unwrap());
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
