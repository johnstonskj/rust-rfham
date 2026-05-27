//! Location configuration types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fields::{
        CFG_FIELD_COUNTRY, CFG_FIELD_GRID_LOCATOR, CFG_FIELD_ITU_REGION, CFG_FIELD_KIND,
        CFG_FIELD_LABEL, CFG_FIELD_LOCATION, CFG_FIELD_MAILING_ADDRESS,
    },
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathElement, PathTarget, Value},
};
use rfham_core::{CountryCode, StringLike, fmt::FormattedWriter, names::Name};
use rfham_itu::{regions::Region, zones::Zone};
use rfham_maidenhead::MaidenheadLocator;
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use std::io::Write;

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    grid_locator: Option<MaidenheadLocator>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    itu_region: Option<Region>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    itu_zone: Option<Zone>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    country: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    mailing_address: Option<String>,
}

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Implementations
// ────────────────────────────────────────────────────────────────────────────────────────────────

impl FormattedWriter for Location {
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
                header(writer, options.nesting_depth(), "Location")?;
                blank_line(writer)?;

                if let Some(grid_locator) = &self.grid_locator {
                    bulleted_list_item(writer, 1, format!("Grid locator: {grid_locator}"))?;
                }
                if let Some(region) = &self.itu_region {
                    bulleted_list_item(writer, 1, format!("ITU region: {region}"))?;
                }
                if let Some(country) = &self.country {
                    bulleted_list_item(writer, 1, format!("Country: {country}"))?;
                }
                if let Some(mailing_address) = &self.mailing_address {
                    bulleted_list_item(writer, 1, format!("Mailing Address: {mailing_address}"))?;
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

impl PathTarget for Location {
    fn path_name() -> Option<Name> {
        Some(Name::new_unchecked(CFG_FIELD_LOCATION))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let head = path.head();
        if let PathElement::Name(name) = head {
            match name.as_str() {
                name if name == CFG_FIELD_GRID_LOCATOR => {
                    if let Some(grid_locator) = &self.grid_locator {
                        Ok(Value::String(grid_locator.to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                name if name == CFG_FIELD_ITU_REGION => {
                    if let Some(itu_region) = &self.itu_region {
                        Ok(Value::String(itu_region.to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                name if name == CFG_FIELD_COUNTRY => {
                    if let Some(country) = &self.country {
                        Ok(Value::String(country.to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                name if name == CFG_FIELD_MAILING_ADDRESS => {
                    if let Some(mailing_address) = &self.mailing_address {
                        Ok(Value::String(mailing_address.to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                _ => Err(ConfigError::InvalidPathComponent(
                    head.to_string(),
                    CFG_FIELD_LOCATION,
                    Self::value_names().collect(),
                )),
            }
        } else {
            Err(ConfigError::InvalidPathElementName(
                head.to_string(),
                CFG_FIELD_LOCATION,
            ))
        }
    }

    fn value_names() -> impl Iterator<Item = &'static str> {
        [
            CFG_FIELD_KIND,
            CFG_FIELD_LABEL,
            CFG_FIELD_GRID_LOCATOR,
            CFG_FIELD_ITU_REGION,
            CFG_FIELD_COUNTRY,
            CFG_FIELD_MAILING_ADDRESS,
        ]
        .into_iter()
    }
}

impl Location {
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Constructors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub fn with_grid_locator(mut self, grid_locator: Option<MaidenheadLocator>) -> Self {
        self.grid_locator = grid_locator;
        self
    }

    pub fn with_itu_region(mut self, itu_region: Option<Region>) -> Self {
        self.itu_region = itu_region;
        self
    }

    pub fn with_itu_zone(mut self, itu_zone: Option<Zone>) -> Self {
        self.itu_zone = itu_zone;
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

    pub const fn itu_zone(&self) -> Option<&Zone> {
        self.itu_zone.as_ref()
    }

    pub fn set_itu_zone(&mut self, itu_zone: Zone) {
        self.itu_zone = Some(itu_zone)
    }

    pub fn unset_itu_zone(&mut self) {
        self.itu_zone = None;
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
