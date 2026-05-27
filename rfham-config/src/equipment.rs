//! Equipment configuration types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fmt::{FormatterOptions, OutputKind},
};
use rfham_core::{Power, fmt::FormattedWriter};
use rfham_itu::bands::FrequencyBand;
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::HashSet, io::Write};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Equipment {
    brand: String,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    asset: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    label: Option<String>,
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    usage: HashSet<Usage>,
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    modes: HashSet<Mode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mobility: Option<Mobility>,
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    bands: HashSet<FrequencyBand>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    max_power: Option<Power>,
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

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumDisplay,
    EnumIs,
    EnumString,
    EnumIter,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Usage {
    Local,
    #[strum(serialize = "QRP")]
    Qrp,
    #[strum(serialize = "DX")]
    Dx,
    EmComm,
    Activation,
    Satellite,
    Scanning,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumDisplay,
    EnumIs,
    EnumString,
    EnumIter,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Mobility {
    StationFixed,
    Portable,
    Mobile,
    Handheld,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumDisplay,
    EnumIs,
    EnumString,
    EnumIter,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum Mode {
    #[strum(serialize = "AM")]
    Am,
    #[strum(serialize = "FM")]
    Fm,
    #[strum(serialize = "SSB")]
    Ssb,
    #[strum(serialize = "RTTY")]
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
                if let Some(label) = &self.label {
                    header(writer, options.nesting_depth(), label)?;
                } else {
                    header(
                        writer,
                        options.nesting_depth(),
                        format!("{} {}", self.brand, self.model),
                    )?;
                }
                blank_line(writer)?;
                bulleted_list_item(writer, 1, format!("Make: {}", self.brand))?;
                bulleted_list_item(writer, 1, format!("Model: {}", self.model))?;
                if !self.usage.is_empty() {
                    bulleted_list_item(
                        writer,
                        1,
                        format!(
                            "Usage: {}",
                            self.usage
                                .iter()
                                .map(|b| b.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )?;
                }
                if !self.modes.is_empty() {
                    bulleted_list_item(
                        writer,
                        1,
                        format!(
                            "Modes: {}",
                            self.modes
                                .iter()
                                .map(|b| b.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )?;
                }
                if let Some(mobility) = &self.mobility {
                    bulleted_list_item(writer, 1, format!("Mobility: {mobility}"))?;
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
                if let Some(max_power) = &self.max_power {
                    bulleted_list_item(writer, 1, format!("Max Power: {max_power}"))?;
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

    pub fn new<S1: Into<String>, S2: Into<String>>(brand: S1, model: S2) -> Self {
        Self {
            brand: brand.into(),
            model: model.into(),
            asset: None,
            label: None,
            usage: HashSet::default(),
            modes: HashSet::default(),
            mobility: None,
            max_power: None,
            bands: HashSet::default(),
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

    pub fn with_usage(mut self, usage: HashSet<Usage>) -> Self {
        self.usage = usage;
        self
    }

    pub fn with_modes(mut self, modes: HashSet<Mode>) -> Self {
        self.modes = modes;
        self
    }

    pub fn with_bands(mut self, bands: HashSet<FrequencyBand>) -> Self {
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
