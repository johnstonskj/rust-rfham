//! Markdown formatting utilities and traits for RF-Ham output.
//!
//! This crate provides two traits and a set of free formatting functions that write
//! coloured Markdown to any `Write` sink. Terminal output is styled with ANSI colours
//! via the `colored` crate; plain writers receive the same text without escape codes.
//!
//! | Trait | Purpose |
//! |-------|---------|
//! | [`ToMarkdown`] | Convert a value to Markdown without external context |
//! | [`ToMarkdownWith`] | Convert a value to Markdown given a caller-supplied context |
//!
//! Key free functions: [`header`], [`plain_text`], [`bulleted_list_item`],
//! [`numbered_list_item`], [`bold_to_string`], [`italic_to_string`], [`link_to_string`],
//! [`fenced_code_block_start`] / [`fenced_code_block_end`].
//!
//! [`Table`] and [`Column`] support fixed-width columnar output.
//!
//! # Examples
//!
//! ```rust
//! use rfham_markdown::{header, plain_text, bulleted_list_item};
//!
//! let mut out = Vec::new();
//! header(&mut out, 1, "My Section").unwrap();
//! plain_text(&mut out, "Some content.").unwrap();
//! bulleted_list_item(&mut out, 1, "First item").unwrap();
//! let s = String::from_utf8(out).unwrap();
//! assert!(s.contains("My Section"));
//! assert!(s.contains("Some content."));
//! ```

use colored::Colorize as _;
use std::{fmt::Display, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ToMarkdown {
    fn write_markdown<W: Write>(&self, writer: &mut W) -> Result<(), MarkdownError>;
    fn to_markdown_string(&self) -> Result<String, MarkdownError> {
        let mut buffer = Vec::new();
        self.write_markdown(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

pub trait ToMarkdownWith {
    type Context: Sized;

    fn write_markdown_with<W: Write>(
        &self,
        writer: &mut W,
        context: Self::Context,
    ) -> Result<(), MarkdownError>;
    fn to_markdown_string_with(&self, context: Self::Context) -> Result<String, MarkdownError> {
        let mut buffer = Vec::new();
        self.write_markdown_with(&mut buffer, context)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl<T: ToMarkdownWith<Context = C>, C: Default> ToMarkdown for T {
    fn write_markdown<W: Write>(&self, writer: &mut W) -> Result<(), MarkdownError> {
        self.write_markdown_with(writer, C::default())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnJustification {
    #[default]
    Left,
    Right,
    Centered,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    label: String,
    justification: Option<ColumnJustification>,
    width: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Table {
    super_labels: Vec<Column>,
    columns: Vec<Column>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const VERTICAL_SEPARATOR_END: &str = "|";
const VERTICAL_SEPARATOR_INNER: &str = " | ";
const BULLET_LIST_BULLET: &str = "*";
const NUMBER_LIST_SEPARATOR: &str = ".";
const HEADING_PREFIX: &str = "#";
const DEFN_LIST_TERM_PREFIX: &str = ";";
const DEFN_LIST_DEFINITION_PREFIX: &str = ":";
const FMT_ITALIC_DELIM: &str = "*";
const FMT_BOLD_DELIM: &str = "**";
const FMT_STRIKETHROUGH_DELIM: &str = "~~";
const FMT_CODE_DELIM: &str = "`";
const BLOCK_QUOTE_PREFIX: &str = ">";

pub fn blank_line<W: Write>(w: &mut W) -> Result<(), MarkdownError> {
    writeln!(w)?;
    Ok(())
}

pub fn plain_text<W: Write, S: AsRef<str>>(w: &mut W, content: S) -> Result<(), MarkdownError> {
    writeln!(w, "{}", content.as_ref().normal())?;
    Ok(())
}

pub fn block_quote<W: Write, S: AsRef<str>>(w: &mut W, content: S) -> Result<(), MarkdownError> {
    writeln!(w, "{} {}", BLOCK_QUOTE_PREFIX, content.as_ref().italic())?;
    Ok(())
}

pub fn bold_to_string<S: AsRef<str>>(content: S) -> String {
    format!(
        "{}{}{}",
        FMT_BOLD_DELIM,
        content.as_ref().bold(),
        FMT_BOLD_DELIM
    )
}

pub fn bold<W: Write, S: AsRef<str>>(w: &mut W, content: S) -> Result<(), MarkdownError> {
    write!(w, "{}", bold_to_string(content))?;
    Ok(())
}

pub fn code_to_string<S: AsRef<str>>(content: S) -> String {
    format!(
        "{}{}{}",
        FMT_CODE_DELIM,
        content.as_ref().white().dimmed(),
        FMT_CODE_DELIM
    )
}

pub fn code<W: Write, S: AsRef<str>>(w: &mut W, content: S) -> Result<(), MarkdownError> {
    write!(w, "{}", code_to_string(content))?;
    Ok(())
}

pub fn italic_to_string<S: AsRef<str>>(content: S) -> String {
    format!(
        "{}{}{}",
        FMT_ITALIC_DELIM,
        content.as_ref().italic(),
        FMT_ITALIC_DELIM
    )
}

pub fn italic<W: Write, S: AsRef<str>>(w: &mut W, content: S) -> Result<(), MarkdownError> {
    write!(w, "{}", italic_to_string(content))?;
    Ok(())
}

pub fn strikethrough_to_string<S: AsRef<str>>(content: S) -> String {
    format!(
        "{}{}{}",
        FMT_STRIKETHROUGH_DELIM,
        content.as_ref().strikethrough(),
        FMT_STRIKETHROUGH_DELIM
    )
}

pub fn strikethrough<W: Write, S: AsRef<str>>(w: &mut W, content: S) -> Result<(), MarkdownError> {
    write!(w, "{}", strikethrough_to_string(content))?;
    Ok(())
}

pub fn link_to_string<S1: AsRef<str>, S2: AsRef<str>>(text: S1, url: S2) -> String {
    format!("[{}]({})", text.as_ref(), url.as_ref())
        .magenta()
        .underline()
        .to_string()
}

pub fn link<W: Write, S1: AsRef<str>, S2: AsRef<str>>(
    w: &mut W,
    text: S1,
    url: S2,
) -> Result<(), MarkdownError> {
    write!(w, "{}", link_to_string(text, url))?;
    Ok(())
}

pub fn header<W: Write, S: AsRef<str>>(
    w: &mut W,
    level: u16,
    content: S,
) -> Result<(), MarkdownError> {
    assert!(level > 0);
    writeln!(w, "{}", header_to_string(level, content))?;
    Ok(())
}

pub fn header_to_string<S: AsRef<str>>(level: u16, content: S) -> String {
    format!(
        "{} {}",
        HEADING_PREFIX.repeat(level as usize),
        content.as_ref()
    )
    .blue()
    .bold()
    .to_string()
}

const CODE_FENCE_STR: &str = "```";

pub fn fenced_code_block_start<W: Write>(w: &mut W) -> Result<(), MarkdownError> {
    writeln!(w, "{}", format!("{CODE_FENCE_STR}text").dimmed())?;
    Ok(())
}

pub fn fenced_code_block_start_for<W: Write, S: AsRef<str>>(
    w: &mut W,
    language: S,
) -> Result<(), MarkdownError> {
    writeln!(
        w,
        "{}",
        format!("{CODE_FENCE_STR}{}", language.as_ref()).dimmed()
    )?;
    Ok(())
}

pub fn fenced_code_block_end<W: Write>(w: &mut W) -> Result<(), MarkdownError> {
    writeln!(w, "{}", CODE_FENCE_STR.dimmed())?;
    Ok(())
}

pub fn bulleted_list<W: Write, S: AsRef<str>>(
    w: &mut W,
    level: u16,
    content: &[S],
) -> Result<(), MarkdownError> {
    let result: Result<Vec<()>, MarkdownError> = content
        .iter()
        .map(|content| bulleted_list_item(w, level, content))
        .collect();
    result.map(|_| ())
}

pub fn bulleted_list_item<W: Write, S: AsRef<str>>(
    w: &mut W,
    level: u16,
    content: S,
) -> Result<(), MarkdownError> {
    assert!(level > 0);
    writeln!(
        w,
        "{}",
        format!(
            "{}{} {}",
            " ".repeat((level as usize - 1) * 2_usize),
            BULLET_LIST_BULLET,
            content.as_ref()
        )
        .yellow()
    )?;
    Ok(())
}

pub fn numbered_list<W: Write, S: AsRef<str>>(
    w: &mut W,
    level: u16,
    content: &[S],
) -> Result<(), MarkdownError> {
    let result: Result<Vec<()>, MarkdownError> = content
        .iter()
        .enumerate()
        .map(|(number, content)| numbered_list_item(w, level, number, content))
        .collect();
    result.map(|_| ())
}

pub fn numbered_list_item<W: Write, S: AsRef<str>>(
    w: &mut W,
    level: u16,
    number: usize,
    content: S,
) -> Result<(), MarkdownError> {
    assert!(level > 0);
    writeln!(
        w,
        "{}",
        format!(
            "{}{}{} {}",
            " ".repeat((level as usize - 1) * 3_usize),
            number,
            NUMBER_LIST_SEPARATOR,
            content.as_ref()
        )
        .yellow()
    )?;
    Ok(())
}

pub fn definition_list_item<W: Write, S1: AsRef<str>, S2: AsRef<str>>(
    w: &mut W,
    term: S1,
    definition: S2,
) -> Result<(), MarkdownError> {
    writeln!(
        w,
        "{}",
        format!("{} {}", DEFN_LIST_TERM_PREFIX, term.as_ref()).yellow()
    )?;
    writeln!(
        w,
        "{}",
        format!("{} {}", DEFN_LIST_DEFINITION_PREFIX, definition.as_ref()).yellow()
    )?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if let Some(width) = &self.width {
                match self.justification {
                    Some(ColumnJustification::Left) => format!("{:<width$}", self.label),
                    Some(ColumnJustification::Right) => format!("{:>width$}", self.label),
                    Some(ColumnJustification::Centered) => format!("{:^width$}", self.label),
                    None => format!("{:width$}", self.label),
                }
            } else {
                self.label.to_string()
            }
        )
    }
}

impl From<String> for Column {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<(&str, usize)> for Column {
    fn from(value: (&str, usize)) -> Self {
        Self::new(value.0).with_width(value.1)
    }
}

impl From<(String, usize)> for Column {
    fn from(value: (String, usize)) -> Self {
        Self::new(value.0).with_width(value.1)
    }
}

impl Column {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            label: content.into(),
            justification: None,
            width: None,
        }
    }

    pub fn left_justified<S: Into<String>>(content: S) -> Self {
        Self::new(content).with_justification(ColumnJustification::Left)
    }

    pub fn right_justified<S: Into<String>>(content: S) -> Self {
        Self::new(content).with_justification(ColumnJustification::Right)
    }

    pub fn centered<S: Into<String>>(content: S) -> Self {
        Self::new(content).with_justification(ColumnJustification::Centered)
    }

    pub fn fill(fill_char: char, width: usize) -> Self {
        Self::new(fill_char.to_string().repeat(width)).with_width(width)
    }

    pub fn with_justification(mut self, justification: ColumnJustification) -> Self {
        self.justification = Some(justification);
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn row_separator(&self) -> Self {
        match (self.justification, self.width) {
            (Some(ColumnJustification::Left), Some(width)) if width >= 2 => Self {
                label: format!(":{}", "-".repeat(width - 1)),
                ..*self
            },
            (Some(ColumnJustification::Right), Some(width)) if width >= 2 => Self {
                label: format!("{}:", "-".repeat(width - 1)),
                ..*self
            },
            (Some(ColumnJustification::Centered), Some(width)) if width >= 3 => Self {
                label: format!(":{}:", "-".repeat(width - 2)),
                ..*self
            },
            (None, Some(width)) => Self {
                label: "-".repeat(width),
                ..*self
            },
            _ => Self {
                label: "-".repeat(2),
                ..*self
            },
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Vec<Column>> for Table {
    fn from(value: Vec<Column>) -> Self {
        Self::new(value)
    }
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            super_labels: Default::default(),
            columns,
        }
    }

    pub fn with_super_labels<S>(mut self, labels: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        assert_eq!(labels.len(), self.columns.len());
        self.super_labels = labels
            .into_iter()
            .zip(self.columns.iter())
            .map(|(label, col)| Column {
                label: label.into(),
                ..col.clone()
            })
            .collect();
        self
    }

    pub fn headers<W>(&self, w: &mut W) -> Result<(), MarkdownError>
    where
        W: Write,
    {
        if !self.super_labels.is_empty() {
            self.write_row(w, &self.super_labels, true)?;
        }
        self.write_row(w, &self.columns, true)?;
        self.write_row(
            w,
            &self
                .columns
                .iter()
                .map(|c| c.row_separator())
                .collect::<Vec<_>>(),
            true,
        )?;
        Ok(())
    }

    pub fn data_row<W, S>(&self, w: &mut W, row: &[S]) -> Result<(), MarkdownError>
    where
        W: Write,
        S: Into<String>,
        String: for<'a> From<&'a S>,
    {
        let row: Vec<Column> = row
            .iter()
            .zip(self.columns.iter())
            .map(|(label, col): (&S, &Column)| Column {
                label: String::from(label),
                ..col.clone()
            })
            .collect();
        self.write_row(w, &row, false)?;
        Ok(())
    }

    fn write_row<W>(&self, w: &mut W, row: &[Column], is_header: bool) -> Result<(), MarkdownError>
    where
        W: Write,
    {
        let row_string = format!(
            "{} {} {}",
            VERTICAL_SEPARATOR_END.bold(),
            row.iter()
                .map(|cell| if is_header {
                    cell.to_string().bold()
                } else {
                    cell.to_string().normal()
                }
                .to_string())
                .collect::<Vec<_>>()
                .join(&VERTICAL_SEPARATOR_INNER.bold()),
            VERTICAL_SEPARATOR_END.bold()
        );
        writeln!(w, "{}", row_string)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;
pub use error::{MarkdownError, MarkdownResult};

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(f: impl FnOnce(&mut Vec<u8>) -> Result<(), MarkdownError>) -> String {
        let mut buf = Vec::new();
        f(&mut buf).unwrap();
        // Strip ANSI escape sequences for comparison
        let s = String::from_utf8(buf).unwrap();
        // Remove ESC[...m sequences
        let re_free: String = s
            .chars()
            .fold((String::new(), false), |(mut acc, in_esc), c| {
                if c == '\x1b' {
                    (acc, true)
                } else if in_esc && c == 'm' {
                    (acc, false)
                } else if !in_esc {
                    acc.push(c);
                    (acc, false)
                } else {
                    (acc, true)
                }
            })
            .0;
        re_free
    }

    #[test]
    fn test_header_level_1() {
        let s = collect(|w| header(w, 1, "Title"));
        assert!(s.contains("# Title"), "got: {s:?}");
    }

    #[test]
    fn test_plain_text() {
        let s = collect(|w| plain_text(w, "Hello world"));
        assert!(s.contains("Hello world"), "got: {s:?}");
    }

    #[test]
    fn test_bulleted_list_item() {
        let s = collect(|w| bulleted_list_item(w, 1, "Item one"));
        assert!(s.contains("* Item one"), "got: {s:?}");
    }

    #[test]
    fn test_bold_to_string() {
        let s = bold_to_string("strong");
        assert!(s.contains("strong"));
        assert!(s.contains("**"));
    }

    #[test]
    fn test_italic_to_string() {
        let s = italic_to_string("em");
        assert!(s.contains("em"));
        assert!(s.contains("*"));
    }

    #[test]
    fn test_link_to_string() {
        let s = link_to_string("ARRL", "https://arrl.org");
        assert!(s.contains("[ARRL]"));
        assert!(s.contains("https://arrl.org"));
    }

    #[test]
    fn test_to_markdown_string() {
        struct Dummy;
        impl ToMarkdown for Dummy {
            fn write_markdown<W: std::io::Write>(
                &self,
                w: &mut W,
            ) -> Result<(), MarkdownError> {
                plain_text(w, "dummy content")
            }
        }
        let s = Dummy.to_markdown_string().unwrap();
        assert!(s.contains("dummy content"));
    }
}
