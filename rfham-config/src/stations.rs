//! Station configuration types for RF-Ham.
//!

use crate::{
    error::ConfigError,
    fields::{
        CFG_FIELD_CALLSIGN, CFG_FIELD_KIND, CFG_FIELD_LABEL, CFG_FIELD_LOCATION,
        CFG_FIELD_OPERATOR_NAME, CFG_FIELD_STATION,
    },
    fmt::{FormatterOptions, OutputKind},
    locations::Location,
    paths::{ConfigPath, PathElement, PathTarget, Value},
};
use rfham_core::{StringLike, callsigns::CallSign, fmt::FormattedWriter, names::Name};
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::io::Write;
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};

// ────────────────────────────────────────────────────────────────────────────────────────────────
// Public Types
// ────────────────────────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Station {
    #[serde(default)]
    kind: StationKind,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    callsign: Option<CallSign>,
    location: Location,
}

#[derive(
    Clone,
    Copy,
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
pub enum StationKind {
    #[default]
    Home,
    Alternate,
    Remote,
    Club,
    Temporary,
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
                let is_default_str = if options.is_default() {
                    " (default)"
                } else {
                    ""
                };
                if let Some(label) = &self.label {
                    header(
                        writer,
                        options.nesting_depth(),
                        format!("{label}{is_default_str}"),
                    )?;
                } else {
                    header(
                        writer,
                        options.nesting_depth(),
                        format!("{} Station{is_default_str}", self.kind),
                    )?;
                }

                if let Some(callsign) = &self.callsign {
                    blank_line(writer)?;
                    bulleted_list_item(writer, 1, format!("Callsign: {}", callsign))?;
                }

                self.location
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

impl PathTarget for Station {
    fn path_name() -> Option<Name> {
        Some(Name::new_unchecked(CFG_FIELD_STATION))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let (head, tail) = path.split();
        if let PathElement::Name(name) = head {
            match name.as_str() {
                name if name == CFG_FIELD_KIND => Ok(Value::String(self.kind.to_string())),
                name if name == CFG_FIELD_LABEL => {
                    if let Some(label) = &self.label {
                        Ok(Value::String(label.to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                name if name == CFG_FIELD_CALLSIGN => {
                    if let Some(callsign) = &self.callsign {
                        Ok(Value::String(callsign.to_string()))
                    } else {
                        Ok(Value::None)
                    }
                }
                name if name == CFG_FIELD_LOCATION => {
                    if let Some(tail_value) = &tail {
                        self.location.value(tail_value)
                    } else {
                        Err(ConfigError::PathTooShort(
                            head.to_string(),
                            CFG_FIELD_STATION,
                            Self::value_names().collect(),
                        ))
                    }
                }
                _ => Err(ConfigError::InvalidPathComponent(
                    head.to_string(),
                    CFG_FIELD_STATION,
                    Self::value_names().collect(),
                )),
            }
        } else {
            Err(ConfigError::InvalidPathElementName(
                head.to_string(),
                CFG_FIELD_STATION,
            ))
        }
    }

    fn value_names() -> impl Iterator<Item = &'static str> {
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

    pub fn new(kind: StationKind, location: Location) -> Self {
        Self {
            kind,
            label: None,
            callsign: None,
            location,
        }
    }

    pub fn with_callsign(mut self, callsign: Option<CallSign>) -> Self {
        self.callsign = callsign;
        self
    }

    pub fn with_label<S: Into<String>>(mut self, label: Option<S>) -> Self {
        self.label = label.map(|s| s.into());
        self
    }

    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈
    // Field Accessors
    // ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈

    pub const fn kind(&self) -> StationKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: StationKind) {
        self.kind = kind;
    }

    pub const fn callsign(&self) -> Option<&CallSign> {
        self.callsign.as_ref()
    }

    pub fn set_callsign(&mut self, callsign: CallSign) {
        self.callsign = Some(callsign)
    }

    pub fn unset_callsign(&mut self) {
        self.callsign = None
    }

    pub const fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label<S: Into<String>>(&mut self, label: S) {
        self.label = Some(label.into())
    }

    pub fn unset_label(&mut self) {
        self.label = None
    }

    pub const fn location(&self) -> &Location {
        &self.location
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}

// ────────────────────────────────────────────────────────────────────────────────────────────────

impl StationKind {
    pub fn label(&self) -> &str {
        match self {
            Self::Home => "My home/primary location",
            Self::Alternate => "Alternate registered location",
            Self::Remote => "A remotely operation location",
            Self::Club => "A club location",
            Self::Temporary => "A temporary/travling location",
        }
    }
}
