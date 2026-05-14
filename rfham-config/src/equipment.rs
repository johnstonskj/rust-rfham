//! Equipment configuration types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fmt::{FormatterOptions, OutputKind},
};
use rfham_core::{Name, Power, error::CoreError, fmt::FormattedWriter};
use rfham_itu::bands::FrequencyBand;
use rfham_markdown::{bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, io::Write, str::FromStr};

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Equipment {
    brand: Name,
    model: Name,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    asset: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    label: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    usage: Vec<Usage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    modes: Vec<Mode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mobility: Option<Mobility>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    max_power: Option<Power>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    bands: Vec<FrequencyBand>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    using: Vec<Equipment>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Asset {
    manufacturer_serial_number: Option<String>,
    purchased_date: String,
    puchased_from: String,
    last_service_date: Option<String>,
    last_serviced_by: Option<String>,
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
pub enum Mobility {
    StationFixed,
    Portable,
    Mobile,
    Handheld,
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

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Implementations
// ────────────────────────────────────────────────────────────────────────────────────────────────

impl FormattedWriter for Equipment {
    type Options = FormatterOptions;
    type Error = ConfigError;

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        match options.output_kind() {
            OutputKind::MarkdownList => {
                header(
                    writer,
                    options.nesting_depth(),
                    format!("{} {}", self.brand, self.model),
                )?;
                if let Some(label) = &self.label {
                    bulleted_list_item(writer, 1, format!("Label: {label}"))?;
                }
                if let Some(max_power) = &self.max_power {
                    bulleted_list_item(writer, 1, format!("Max Power: {max_power}"))?;
                }
                if !self.bands.is_empty() {
                    bulleted_list_item(
                        writer,
                        1,
                        format!(
                            "Operating Bands: {}",
                            self.bands
                                .iter()
                                .map(|b| b.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )?;
                }
                if !self.using.is_empty() {
                    header(writer, options.nesting_depth() + 1, "Using Equipment")?;
                    for equipment in &self.using {
                        equipment.write_with(writer, &options.with_additional_depth(2))?;
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

impl Equipment {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn new<S: Into<String>>(brand: Name, model: Name) -> Self {
        Self {
            brand,
            model,
            asset: None,
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

    pub fn with_asset_information(mut self, asset: Option<Asset>) -> Self {
        self.asset = asset;
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
