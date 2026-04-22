//!
//! One-line description.
//!
//! More detailed description.
//!
//! # Examples
//!
//! ```rust
//! ```
//!
//! # Features
//!
//! - **feature-name**; Feature description
//!

use crate::error::{ConfigError, ConfigResult};
use rfham_core::{CountryCode, Name, Power, callsign::CallSign, error::CoreError};
use rfham_geo::grid::maidenhead::MaidenheadLocator;
use rfham_itu::{bands::FrequencyBand, regions::Region};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::Display,
    fs::{self, File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing::trace;

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Configuration {
    #[serde(skip)]
    path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    station: Option<Station>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    equipment: Vec<Equipment>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Station {
    callsign: CallSign,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    kind: LocationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_locator: Option<MaidenheadLocator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    itu_region: Option<Region>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mailing_address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Equipment {
    brand: Name,
    model: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    usage: Vec<Usage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    modes: Vec<Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobility: Option<Mobility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_power: Option<Power>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bands: Vec<FrequencyBand>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    using: Vec<Equipment>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub enum LocationKind {
    #[default]
    Home,
    Alternate,
    Remote,
    Club,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub enum Mobility {
    StationFixed,
    Portable,
    Mobile,
    Handheld,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub enum Usage {
    Local,
    Qrp,
    Dx,
    EmComm,
    Activation,
    Satellite,
    Scanning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub enum Mode {
    Am,
    Fm,
    Ssb,
    Rtty,
    Digital,
    Image,
}

pub const CONFIG_DIR_NAME: &str = "rfham";
pub const CONFIG_FILE_NAME: &str = "rfham-config.toml";

pub trait Dump {
    fn dump<W: Write>(&self, writer: &mut W, prefix: &str) -> ConfigResult<()>;
}

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Implementations
// ────────────────────────────────────────────────────────────────────────────────────────────────

impl Dump for Configuration {
    fn dump<W: Write>(&self, writer: &mut W, prefix: &str) -> ConfigResult<()> {
        writeln!(writer, "Configuration:")?;
        if let Some(path) = &self.path {
            writeln!(writer, "{prefix}└── path to file: {path:?}")?;
        }
        if let Some(station) = &self.station {
            station.dump(writer, "    ")?;
        }
        if !self.equipment.is_empty() {
            writeln!(writer, "{prefix}└── Equipment:")?;
            for equipment in &self.equipment {
                equipment.dump(writer, "    ")?;
            }
        }
        Ok(())
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

    pub fn with_station(mut self, station: Option<Station>) -> Self {
        self.station = station;
        self
    }

    pub fn with_equipment(mut self, equipment: Vec<Equipment>) -> Self {
        self.equipment = equipment;
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

    pub fn station(&self) -> Option<&Station> {
        self.station.as_ref()
    }

    pub fn set_station(&mut self, station: Station) {
        self.station = Some(station)
    }

    pub fn unset_station(&mut self) {
        self.station = None
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

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Dump for Station {
    fn dump<W: Write>(&self, writer: &mut W, prefix: &str) -> ConfigResult<()> {
        writeln!(writer, "{prefix}Station:")?;
        writeln!(writer, "{prefix}└── call sign: {}", self.callsign)?;
        if let Some(operator_name) = &self.operator_name {
            writeln!(writer, "{prefix}└── operator name: {operator_name}")?;
        }
        if let Some(location) = &self.location {
            location.dump(writer, &format!("{prefix}    "))?;
        }
        Ok(())
    }
}

impl Station {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub const fn new(callsign: CallSign) -> Self {
        Self {
            callsign,
            location: None,
            operator_name: None,
        }
    }

    pub fn with_location(mut self, location: Option<Location>) -> Self {
        self.location = location;
        self
    }

    pub fn with_operator_name<S: Into<String>>(mut self, operator_name: Option<S>) -> Self {
        self.operator_name = operator_name.map(|s| s.into());
        self
    }

    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Field Accessors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub const fn callsign(&self) -> &CallSign {
        &self.callsign
    }

    pub fn set_callsign(&mut self, callsign: CallSign) {
        self.callsign = callsign
    }

    pub const fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    pub fn unset_location(&mut self) {
        self.location = None
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
}

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Dump for Location {
    fn dump<W: Write>(&self, writer: &mut W, prefix: &str) -> ConfigResult<()> {
        writeln!(writer, "{prefix}Location:")?;
        if let Some(grid_locator) = &self.grid_locator {
            writeln!(writer, "{prefix}└── grid locator: {grid_locator}")?;
        }
        if let Some(region) = &self.itu_region {
            writeln!(writer, "{prefix}└── ITU region: {region}")?;
        }
        if let Some(country) = &self.country {
            writeln!(writer, "{prefix}└── country: {country}")?;
        }
        if let Some(mailing_address) = &self.mailing_address {
            writeln!(writer, "{prefix}└── mailing address: {mailing_address}")?;
        }
        Ok(())
    }
}

impl Location {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn new(kind: LocationKind) -> Self {
        Self {
            kind,
            label: None,
            grid_locator: None,
            itu_region: None,
            country: None,
            mailing_address: None,
        }
    }

    pub fn with_label<S: Into<String>>(mut self, label: Option<S>) -> Self {
        self.label = label.map(|s| s.into());
        self
    }

    pub fn with_grid_locator(mut self, grid_locator: Option<MaidenheadLocator>) -> Self {
        self.grid_locator = grid_locator;
        self
    }

    pub fn with_itu_region(mut self, itu_region: Option<Region>) -> Self {
        self.itu_region = itu_region;
        self
    }

    pub fn with_country(mut self, country: Option<CountryCode>) -> Self {
        self.country = country;
        self
    }

    pub fn with_mailing_address<S: Into<String>>(mut self, mailing_address: Option<S>) -> Self {
        self.mailing_address = mailing_address.map(|s| s.into());
        self
    }

    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Field Accessors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub const fn kind(&self) -> LocationKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: LocationKind) {
        self.kind = kind;
    }

    pub const fn grid_locator(&self) -> Option<&MaidenheadLocator> {
        self.grid_locator.as_ref()
    }

    pub fn set_grid_locator(&mut self, grid_locator: MaidenheadLocator) {
        self.grid_locator = Some(grid_locator)
    }

    pub fn unset_grid_locator(&mut self) {
        self.grid_locator = None;
    }

    pub const fn itu_region(&self) -> Option<&Region> {
        self.itu_region.as_ref()
    }

    pub fn set_itu_region(&mut self, itu_region: Region) {
        self.itu_region = Some(itu_region)
    }

    pub fn unset_itu_region(&mut self) {
        self.itu_region = None;
    }

    pub const fn country(&self) -> Option<&CountryCode> {
        self.country.as_ref()
    }

    pub fn set_country(&mut self, country: CountryCode) {
        self.country = Some(country)
    }

    pub fn unset_country(&mut self) {
        self.country = None
    }

    pub const fn mailing_address(&self) -> Option<&String> {
        self.mailing_address.as_ref()
    }

    pub fn set_mailing_address<S: Into<String>>(&mut self, mailing_address: S) {
        self.mailing_address = Some(mailing_address.into())
    }

    pub fn unset_mailing_address(&mut self) {
        self.mailing_address = None
    }
}

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Dump for Equipment {
    fn dump<W: Write>(&self, writer: &mut W, prefix: &str) -> ConfigResult<()> {
        if let Some(label) = &self.label {
            writeln!(writer, "{prefix}{label}:")?;
            writeln!(writer, "{prefix}└── brand: {}", self.brand)?;
            writeln!(writer, "{prefix}└── model: {}", self.model)?;
        } else {
            writeln!(writer, "{prefix}{} {}:", self.brand, self.model)?;
        }
        if let Some(max_power) = &self.max_power {
            writeln!(writer, "{prefix}└── max power: {}", max_power)?;
        }
        if !self.bands.is_empty() {
            writeln!(
                writer,
                "{prefix}└── operating bands: {}",
                self.bands
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        if !self.using.is_empty() {
            writeln!(writer, "{prefix}└── Using:")?;
            for equipment in &self.using {
                equipment.dump(writer, &format!("{prefix}    "))?;
            }
        }
        Ok(())
    }
}

impl Equipment {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn new<S: Into<String>>(brand: Name, model: Name) -> Self {
        Self {
            brand,
            model,
            label: None,
            usage: Vec::default(),
            modes: Vec::default(),
            mobility: None,
            max_power: None,
            bands: Vec::default(),
            using: Vec::default(),
        }
    }

    pub fn with_label<S: Into<String>>(mut self, label: Option<S>) -> Self {
        self.label = label.map(|s| s.into());
        self
    }

    pub fn with_mobility(mut self, mobility: Option<Mobility>) -> Self {
        self.mobility = mobility;
        self
    }

    pub fn with_max_power(mut self, max_power: Option<Power>) -> Self {
        self.max_power = max_power;
        self
    }

    pub fn with_usage(mut self, usage: Vec<Usage>) -> Self {
        self.usage = usage;
        self
    }

    pub fn with_modes(mut self, modes: Vec<Mode>) -> Self {
        self.modes = modes;
        self
    }

    pub fn with_bands(mut self, bands: Vec<FrequencyBand>) -> Self {
        self.bands = bands;
        self
    }

    pub fn with_using(mut self, using: Vec<Equipment>) -> Self {
        self.using = using;
        self
    }
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Field Accessors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
}

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Display for LocationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::Home => "Home or Primary licensed station",
                    Self::Alternate => "Alternate licensed station",
                    Self::Remote => "Remote operating location",
                    Self::Club => "Club location",
                }
            } else {
                match self {
                    Self::Home => "home",
                    Self::Alternate => "alternate",
                    Self::Remote => "remote",
                    Self::Club => "club",
                }
            }
        )
    }
}

impl FromStr for LocationKind {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "home" => Ok(Self::Home),
            "alternate" => Ok(Self::Alternate),
            "remote" => Ok(Self::Alternate),
            "club" => Ok(Self::Club),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "LocationKind",
            )),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::Local => "Local area, including repeaters",
                    Self::Qrp => "QRP, low power, operation",
                    Self::Dx => "DX, distance, operation",
                    Self::EmComm => "Emergency Communications, including ARES/RACES",
                    Self::Activation => "POTA, SOTA, IOTA, etc. activation",
                    Self::Satellite => "Satellite opertions",
                    Self::Scanning => "Scanning",
                }
            } else {
                match self {
                    Self::Local => "local",
                    Self::Qrp => "qrp",
                    Self::Dx => "dx",
                    Self::EmComm => "emcomm",
                    Self::Activation => "activation",
                    Self::Satellite => "satellite",
                    Self::Scanning => "scanning",
                }
            }
        )
    }
}

impl FromStr for Usage {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(Self::Local),
            "qrp" => Ok(Self::Qrp),
            "dx" => Ok(Self::Dx),
            "emcomm" => Ok(Self::EmComm),
            "activation" => Ok(Self::Activation),
            "satellite" => Ok(Self::Satellite),
            "scanning" => Ok(Self::Scanning),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "Usage")),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::Am => "AM",
                    Self::Fm => "FM",
                    Self::Ssb => "SSB",
                    Self::Rtty => "RTTY",
                    Self::Digital => "Digital modes",
                    Self::Image => "Images and SSTV",
                }
            } else {
                match self {
                    Self::Am => "am",
                    Self::Fm => "fm",
                    Self::Ssb => "ssb",
                    Self::Rtty => "rtty",
                    Self::Digital => "digital",
                    Self::Image => "image",
                }
            }
        )
    }
}

impl FromStr for Mode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "am" => Ok(Self::Am),
            "fm" => Ok(Self::Fm),
            "ssb" => Ok(Self::Ssb),
            "rtty" => Ok(Self::Rtty),
            "digital" => Ok(Self::Digital),
            "image" => Ok(Self::Image),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "Mode")),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════════════════════════

impl Display for Mobility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::StationFixed => "Station only",
                    Self::Portable => "Portable from station",
                    Self::Mobile => "Mobile, vehicle/vessel mounted",
                    Self::Handheld => "Handheld",
                }
            } else {
                match self {
                    Self::StationFixed => "station-fixed",
                    Self::Portable => "portable",
                    Self::Mobile => "mobile",
                    Self::Handheld => "handheld",
                }
            }
        )
    }
}

impl FromStr for Mobility {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "station-fixed" => Ok(Self::StationFixed),
            "portable" => Ok(Self::Portable),
            "mobile" => Ok(Self::Mobile),
            "handheld" => Ok(Self::Handheld),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "Mobility")),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Sub-modules
// ────────────────────────────────────────────────────────────────────────────────────────────────

pub mod error;
pub mod paths;
