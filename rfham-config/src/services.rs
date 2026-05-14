//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::{
    error::ConfigError,
    fields::{CFG_FIELD_CREDENTIAL_STORAGE, CFG_FIELD_CREDENTIALS, CFG_FIELD_SERVICES},
    fmt::{FormatterOptions, OutputKind},
    paths::{ConfigPath, PathTarget, Value},
};
use rfham_core::{StringLike, fmt::FormattedWriter, names::Name};
use rfham_markdown::{blank_line, bulleted_list_item, header};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::HashMap, io::Write};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Services {
    #[serde(default)]
    storage_kind: CredentialStorageKind,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    credentials: HashMap<Name, Credentials>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Credentials {
    identifier: String,
    secret: String,
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
    EnumIter,
    EnumString,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum CredentialStorageKind {
    #[default]
    #[strum(serialize = "plain-text")]
    PlainText,
    #[strum(serialize = "password-store")]
    PasswordStore,
    #[strum(serialize = "keychain")]
    KeyChain,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FormattedWriter for Services {
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
                header(writer, options.nesting_depth(), "Services")?;
                blank_line(writer)?;

                bulleted_list_item(
                    writer,
                    1,
                    format!("Credential storage: {}", self.storage_kind),
                )?;
                blank_line(writer)?;

                if !self.credentials.is_empty() {
                    header(writer, options.nesting_depth() + 1, "Credentialed Services")?;
                    blank_line(writer)?;

                    for (service, credentials) in &self.credentials {
                        bulleted_list_item(
                            writer,
                            1,
                            format!("**{service}** for {}", credentials.identifier),
                        )?;
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

impl PathTarget for Services {
    fn path_name(&self) -> Option<Name> {
        Some(Name::new_unchecked(CFG_FIELD_SERVICES))
    }

    fn value(&self, path: &ConfigPath) -> Result<Value, ConfigError> {
        let (head, _tail) = path.split();
        match head.as_str() {
            name if name == CFG_FIELD_CREDENTIAL_STORAGE => {
                Ok(Value::EnumValue(self.storage_kind.to_string()))
            }
            name if name == CFG_FIELD_CREDENTIALS => Err(ConfigError::RestrictedPath),
            name => Err(ConfigError::InvalidPathComponent(
                name.to_string(),
                CFG_FIELD_SERVICES,
                self.value_names().collect(),
            )),
        }
    }

    fn value_names(&self) -> impl Iterator<Item = &'static str> {
        [CFG_FIELD_CREDENTIAL_STORAGE, CFG_FIELD_CREDENTIALS].into_iter()
    }
}

impl Services {
    pub fn new(storage_kind: CredentialStorageKind) -> Self {
        Self {
            storage_kind,
            credentials: HashMap::default(),
        }
    }

    pub fn storage_kind(&self) -> CredentialStorageKind {
        self.storage_kind
    }

    pub fn set_storage_kind(&mut self, storage_kind: CredentialStorageKind) {
        self.storage_kind = storage_kind;
    }

    pub fn get_credentials(&self, service: &Name) -> Result<Option<&Credentials>, ConfigError> {
        match self.storage_kind {
            CredentialStorageKind::PlainText => Ok(self.credentials.get(service)),
            _ => Err(ConfigError::CredentialStore("unsupported kind".to_string())),
        }
    }

    pub fn set_credentials(
        &mut self,
        service: Name,
        credentials: Credentials,
    ) -> Result<Option<Credentials>, ConfigError> {
        match self.storage_kind {
            CredentialStorageKind::PlainText => Ok(self.credentials.insert(service, credentials)),
            _ => Err(ConfigError::CredentialStore("unsupported kind".to_string())),
        }
    }

    pub fn drop_credentials(&mut self, service: &Name) -> Result<Option<Credentials>, ConfigError> {
        match self.storage_kind {
            CredentialStorageKind::PlainText => Ok(self.credentials.remove(service)),
            _ => Err(ConfigError::CredentialStore("unsupported kind".to_string())),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Credentials {
    pub fn new(identifier: String, secret: String) -> Self {
        Self { identifier, secret }
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn user_name(&self) -> &str {
        self.identifier()
    }

    pub fn password(&self) -> &str {
        self.secret()
    }
}
// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
