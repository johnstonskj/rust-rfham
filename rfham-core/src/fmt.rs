//! Custom formatting trait for rfham types.
//!
//! [`Formatter`] is implemented by types that support configurable text rendering beyond what
//! [`std::fmt::Display`] provides. [`NumericFormatOptions`] currently carries an optional decimal
//! precision specifier.

use std::{error::Error, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Formatter {
    type Options: Default;

    fn fmt(&self) -> String {
        self.fmt_with(&Self::Options::default())
    }

    fn fmt_with(&self, options: &Self::Options) -> String;
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NumericFormatOptions {
    precision: Option<usize>,
}

pub trait FormattedWriter {
    type Options: Default;
    type Error: Error;

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.write_with(writer, &Self::Options::default())
    }

    fn write_with<W: Write>(
        &self,
        writer: &mut W,
        options: &Self::Options,
    ) -> Result<(), Self::Error>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl NumericFormatOptions {
    pub const fn with_precision(mut self, precision: usize) -> Self {
        self.precision = Some(precision);
        self
    }
    pub const fn precision(&self) -> Option<usize> {
        self.precision
    }
}
