//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use colored::Colorize;
use inquire::error::InquireError;
use rfham_config::error::ConfigError;
use rfham_core::error::CoreError;
use rfham_geo::error::GeoError;
use rfham_markdown::MarkdownError;
use rfham_services::error::ServiceError;
use std::{
    fmt::{Display, Error as FmtError},
    path::Path,
};
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use thiserror::Error;
use tracing_subscriber::filter::FromEnvError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum CliError {
    #[error("An error occured in the rfham libraries; error: {0}")]
    Core(#[from] CoreError),

    #[error(
        "Could not retrieve value from environment variable for command-line argument; error: {0}"
    )]
    EnvError(#[from] FromEnvError),

    #[error("An error occured during formatting; error: {0}")]
    FmtError(#[from] FmtError),

    #[error("An error occured loading or initializing the configuration; error: {0}")]
    Config(#[from] ConfigError),

    #[error("An error occured with the geo crate datatypes; error: {0}")]
    Geo(#[from] GeoError),

    #[error("An error occured writing markdown output; error: {0}")]
    Markdown(#[from] MarkdownError),

    #[error("An error occured initializing or calling a service; error: {0}")]
    Service(#[from] ServiceError),

    #[error("Error in interactive input (inquire crate); error: {0}")]
    Interactive(#[from] InquireError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliMessage {
    severity: Severity,
    message: String,
    context: Option<String>,
    notes: Vec<Note>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    content: String,
    kind: NoteKind,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, EnumIter)]
pub enum Severity {
    Information,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, EnumIter)]
pub enum NoteKind {
    Help,
    Note,
    #[strum(serialize = "See Also")]
    SeeAlso,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn config_value_not_found<S1: AsRef<str>, S2: AsRef<str>>(
    field_name: S1,
    component_name: S2,
    possible_names: &[&'static str],
) -> CliMessage {
    CliMessage::error(format!(
        "the config path name `{}` is not valid in the matching component",
        field_name.as_ref()
    ))
    .with_context(format!("Component `{}`", component_name.as_ref()))
    .with_note(Note::help(format!(
        "Possible names: {}",
        possible_names
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )))
}

pub fn config_path_too_short<S1: AsRef<str>, S2: AsRef<str>>(
    field_name: S1,
    component_name: S2,
    possible_names: &[&'static str],
) -> CliMessage {
    CliMessage::error(format!(
        "the config path name `{}` expected additional path elements",
        field_name.as_ref()
    ))
    .with_context(format!("Component `{}`", component_name.as_ref()))
    .with_note(Note::help(format!(
        "Possible names: {}",
        possible_names
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )))
}

pub fn no_band_plan_for_country<S1: AsRef<str>>(country: S1) -> CliMessage {
    CliMessage::error("no band plan found for provided country")
        .with_context(format!("Country `{}`", country.as_ref()))
        .with_note(Note::help(
            "to see a list of known band plans, run `rfham band-plan list`",
        ))
}

pub fn config_file_exists<P: AsRef<Path>>(path: P) -> CliMessage {
    CliMessage::error("Failed to write configuration file, it already exists")
        .with_context(format!("Path `{:?}`", path.as_ref()))
        .with_note(Note::help(
            "to overwrite the existing file, run `rfham config init --overwrite`",
        ))
}

pub fn config_file_missing<P: AsRef<Path>>(path: P) -> CliMessage {
    CliMessage::error("Configuration file was not found".to_string())
        .with_context(format!("Default path `{:?}`", path.as_ref()))
        .with_note(Note::help("try running `rfham config init`"))
}

pub fn missing_executable<S: AsRef<str>>(name: S) -> CliMessage {
    CliMessage::error("no executable found for external command")
        .with_context(format!("Command `{}`", name.as_ref()))
        .with_note(Note::help(format!(
            "check that the executable `rfham-{}` exists and is in $PATH",
            name.as_ref()
        )))
}

pub fn sub_process_error<P: AsRef<Path>, E: std::error::Error>(path: P, error: E) -> CliMessage {
    CliMessage::error("an error occured trying to run a sub-process")
        .with_context(format!("Executable path `{:?}`", path.as_ref()))
        .with_note(Note::note(format!("reported error: {error}")))
}

pub fn unhandled_error<E: std::error::Error>(error: E) -> CliMessage {
    CliMessage::error("an unhandled error has been detected, sorry about that")
        .with_context(format!("Error {error}"))
        .with_notes([
            Note::note("if this re-occurs, please file a bug"),
            Note::see_also("https://github.com/johnstonskj/rust-rfham/issues"),
        ])
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for CliMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (icon, severity, message) = match self.severity {
            Severity::Information => ("💡", self.severity.to_string(), self.message.to_string()),
            Severity::Warning => (
                "⚠️",
                self.severity.to_string().bold().yellow().to_string(),
                self.message.to_string(),
            ),
            Severity::Error => (
                "🛑",
                self.severity.to_string().bold().red().to_string(),
                self.message.to_string().bold().to_string(),
            ),
        };
        writeln!(f, "{} {}: {}", icon, severity, message)?;
        if let Some(context) = &self.context {
            writeln!(
                f,
                "{} {}",
                if self.notes.is_empty() {
                    "   └── 🔎"
                } else {
                    "   ├── 🔎"
                },
                context.blue()
            )?;
        }
        for (note, is_last) in self
            .notes
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i == self.notes.len() - 1))
        {
            writeln!(
                f,
                "{} {}",
                if is_last {
                    "   └──"
                } else {
                    "   ├──"
                },
                format!(
                    "{}  {} {}",
                    match note.kind {
                        NoteKind::Help => "ℹ️",
                        NoteKind::Note => "🗒️",
                        NoteKind::SeeAlso => "🔗",
                    },
                    note.kind,
                    note.content
                )
                .dimmed()
            )?;
        }
        Ok(())
    }
}

impl CliMessage {
    pub fn new<S: Into<String>>(severity: Severity, message: S) -> Self {
        Self {
            severity,
            message: message.into(),
            context: Default::default(),
            notes: Default::default(),
        }
    }

    pub fn error<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Error, message)
    }

    #[allow(dead_code)]
    pub fn warning<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Warning, message)
    }

    #[allow(dead_code)]
    pub fn infomation<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Information, message)
    }

    pub fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_notes<I: IntoIterator<Item = Note>>(mut self, notes: I) -> Self {
        self.notes = notes.into_iter().collect();
        self
    }

    pub fn with_note(mut self, note: Note) -> Self {
        self.notes = vec![note];
        self
    }

    pub fn print(&self) {
        match self.severity {
            Severity::Information => println!("{self}"),
            Severity::Warning => print!("{self}"),
            Severity::Error => eprint!("{self}"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Note {
    pub fn new<S: Into<String>>(kind: NoteKind, content: S) -> Self {
        Self {
            kind,
            content: content.into(),
        }
    }

    pub fn help<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::Help, content)
    }

    pub fn note<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::Note, content)
    }

    pub fn see_also<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::SeeAlso, content)
    }
}
