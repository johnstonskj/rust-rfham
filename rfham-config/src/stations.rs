//! Station configuration types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fields::{CFG_FIELD_CALLSIGN, CFG_FIELD_LOCATION, CFG_FIELD_OPERATOR_NAME, CFG_FIELD_STATION},
    fmt::{FormatterOptions, OutputKind},
    locations::Location,
    paths::{ConfigPath, PathTarget, Value},
};
use rfham_core::{StringLike, callsigns::CallSign, fmt::FormattedWriter, names::Name};
use rfham_itu::callsigns::ItuSeriesAllocation;
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use std::io::Write;

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Station {
    callsign: CallSign,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    operator_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    location: Option<Location>,
}

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Implementations
// ────────────────────────────────────────────────────────────────────────────────────────────────

impl FormattedWriter for Station {
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
                header(writer, options.nesting_depth(), "Station")?;
                blank_line(writer)?;

                bulleted_list_item(writer, 1, format!("Callsign: {}", self.callsign))?;

                if let Some(allocation) = ItuSeriesAllocation::from_callsign(&self.callsign) {
                    bulleted_list_item(writer, 2, format!("ITU allocation; {allocation:#}"))?;
                }

                if let Some(operator_name) = &self.operator_name {
                    bulleted_list_item(writer, 1, format!("Operator name: {operator_name}"))?;
                }

                if let Some(location) = &self.location {
                    location.write_with(writer, &options.with_additional_depth(1))?;
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

impl PathTarget for Station {
    fn path_name(&self) -> Option<Name> {
        Some(Name::new_unchecked(CFG_FIELD_STATION))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let (head, tail) = path.split();
        match head.as_str() {
            CFG_FIELD_CALLSIGN => Ok(Value::String(self.callsign.to_string())),
            name if name == CFG_FIELD_OPERATOR_NAME => {
                if let Some(operator_name) = &self.operator_name {
                    Ok(Value::String(operator_name.to_string()))
                } else {
                    Ok(Value::None)
                }
            }
            name if name == CFG_FIELD_LOCATION => {
                if let Some(location) = &self.location
                    && tail.is_some()
                {
                    location.value(tail.as_ref().unwrap())
                } else if tail.is_none() {
                    Err(ConfigError::PathTooShort(
                        head.to_string(),
                        CFG_FIELD_STATION,
                        self.value_names().collect(),
                    ))
                } else {
                    Ok(Value::None)
                }
            }
            _ => Err(ConfigError::InvalidPathComponent(
                head.to_string(),
                CFG_FIELD_STATION,
                self.value_names().collect(),
            )),
        }
    }

    fn value_names(&self) -> impl Iterator<Item = &'static str> {
        [
            CFG_FIELD_CALLSIGN,
            CFG_FIELD_OPERATOR_NAME,
            CFG_FIELD_LOCATION,
        ]
        .into_iter()
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
