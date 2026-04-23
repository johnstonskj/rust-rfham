//!
//! One-line description.
//!
//! More detailed description.
//!
//! # Examples
//!
//! ```rust
//! use rfham_antennas::SimpleDipole;
//! use rfham_bands::us_fcc::arrl_voluntary_band_plan;
//! use rfham_itu::allocations::FrequencyAllocation::Band2M;
//!
//! let my_dipole = SimpleDipole::new_in_plan(
//!     Band2M,
//!     arrl_voluntary_band_plan()
//! );
//! assert_eq!(
//!     Some("1.0266865 m".to_string()),
//!     my_dipole.antenna_length().map(|v|v.to_string())
//! );
//! ```
//!

use colored::Colorize;
use rfham_bands::BandPlan;
use rfham_core::{
    error::CoreError,
    frequency::{FrequencyRange, Wavelength, meters},
};
use rfham_itu::allocations::FrequencyAllocation;
use rfham_markdown::{
    ToMarkdown, blank_line, fenced_code_block_end, fenced_code_block_start, header,
    italic_to_string, numbered_list_item, plain_text,
};
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Describe this struct.
///
/// # Fields
///
/// - `band` (`FrequencyAllocation`) - Describe this field.
/// - `band_plan` (`Option<BandPlan>`) - Describe this field.
///
/// # Examples
///
/// ```
/// use rfham_antennas::SimpleDipole;
/// use rfham_bands::us_fcc::arrl_voluntary_band_plan;
/// use rfham_itu::allocations::FrequencyAllocation::Band2M;
/// use rfham_markdown::ToMarkdown;
/// use std::io::stdout;
///
/// let my_dipole = SimpleDipole::new_in_plan(
///     Band2M,
///     arrl_voluntary_band_plan()
/// );
/// my_dipole.write_markdown(&mut stdout()).unwrap();
/// ```
///
/// Results in the following output.
///
/// ```markdown
/// # Classical half-wave dipole antenna for 2m band.
///
/// ~~~text
/// |<──────────────────────── λ/2 = 1.027 meters ─────────────────────────>|
/// |<─── λ/4 = 51.334 centimeters ───>| |<─── λ/4 = 51.334 centimeters ───>|
/// ────────────────────────────────────┳────────────────────────────────────
///                                     │  ∧
///                                     │  │
///                                     │  │ λ/2 = 1.027 meters
///                                     │  │
///                                     │  ∨
/// ~~~
///
/// Notes:
///
/// 1. Frequency range for 2m band is 144.000 MHz - 148.000 MHz.
///    1. From the *US Amateur Radio Bands* by The American Radio Relay League (ARRL).
/// 2. Mid-point of band is 146.000 MHz.
/// 3. Wavelength of mid-point is 2.053 m.
/// 4. Half-wave length is λ/2 = 1.027 meters for overall antenna.
/// 5. Quarter-wave length is λ/4 = 51.334 centimeters for each antenna pole.
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleDipole {
    band: FrequencyAllocation,
    band_plan: Option<BandPlan>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SimpleDipole {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ToMarkdown for SimpleDipole {
    fn write_markdown<W: std::io::Write>(&self, writer: &mut W) -> Result<(), CoreError> {
        if let Some(quarter_wavelength) = self.pole_length() {
            const QUARTER_WAVE_PADDING: usize = "|<--- ".len() + " --->|".len();
            const HALF_WAVE_PADDING: usize = "|< ".len() + " >|".len();

            let wl_4 = format!("λ/4 = {quarter_wavelength:#.3}");
            let wl_4_len: usize = wl_4.len() - 1;
            let wl_4_padded_len = wl_4_len + QUARTER_WAVE_PADDING;
            let wl_2 = meters(quarter_wavelength.value() * 2.0);
            let wl_2 = format!("λ/2 = {wl_2:#.3}");
            let wl_2_len = wl_2.len() - 1;
            let width = wl_4_padded_len * 2 + 1;
            let pad_width = (width - (wl_2_len + HALF_WAVE_PADDING)) / 2;
            let pad_str = "─".repeat(pad_width);

            header(
                writer,
                1,
                format!("Classical half-wave dipole antenna for {} band.", self.band),
            )?;
            blank_line(writer)?;
            fenced_code_block_start(writer)?;
            let left_pad = format!("|<{pad_str}").blue().dimmed();
            let right_pad = format!("{pad_str}{}>|", if wl_2_len % 2 == 1 { "" } else { "─" },)
                .blue()
                .dimmed();
            writeln!(writer, "{} {} {}", left_pad, wl_2.bold(), right_pad)?;
            let quarter_measure = format!(
                "{} {} {}",
                "|<───".blue().dimmed(),
                wl_4.bold(),
                "───>|".blue().dimmed(),
            );
            writeln!(writer, "{quarter_measure} {quarter_measure}",)?;
            plain_text(
                writer,
                format!(
                    "{}┳{}",
                    "─".repeat(wl_4_padded_len),
                    "─".repeat(wl_4_padded_len)
                ),
            )?;
            writeln!(
                writer,
                "{}│  {}",
                " ".repeat(wl_4_padded_len),
                "∧".blue().dimmed()
            )?;
            writeln!(
                writer,
                "{}│  {}",
                " ".repeat(wl_4_padded_len),
                "│".blue().dimmed()
            )?;
            writeln!(
                writer,
                "{}│  {} {}",
                " ".repeat(wl_4_padded_len),
                "│".blue().dimmed(),
                wl_2.bold()
            )?;
            writeln!(
                writer,
                "{}│  {}",
                " ".repeat(wl_4_padded_len),
                "│".blue().dimmed()
            )?;
            writeln!(
                writer,
                "{}│  {}",
                " ".repeat(wl_4_padded_len),
                "∨".blue().dimmed()
            )?;
            fenced_code_block_end(writer)?;
            blank_line(writer)?;

            plain_text(writer, "Notes:")?;
            blank_line(writer)?;

            // This is safe because it's required to calculate the side length.
            let range = self.band_range().unwrap();
            numbered_list_item(
                writer,
                1,
                1,
                format!("Frequency range for {} band is {:.3}.", self.band, range,),
            )?;
            numbered_list_item(
                writer,
                2,
                1,
                format!(
                    "From the {}.",
                    if let Some(band_plan) = &self.band_plan {
                        format!(
                            "{} by {}",
                            italic_to_string(band_plan.name()),
                            band_plan.maintaining_agency()
                        )
                    } else {
                        "ITU frequency allocation".to_string()
                    }
                ),
            )?;
            numbered_list_item(
                writer,
                1,
                2,
                format!("Mid-point of band is {:.3}.", range.mid_band()),
            )?;
            numbered_list_item(
                writer,
                1,
                3,
                format!(
                    "Wavelength of mid-point is {:.3}.",
                    range.mid_band().to_wavelength()
                ),
            )?;
            numbered_list_item(
                writer,
                1,
                4,
                format!("Half-wave length is {wl_2} for overall antenna."),
            )?;
            numbered_list_item(
                writer,
                1,
                5,
                format!("Quarter-wave length is {wl_4} for each antenna pole."),
            )?;
        } else {
            println!(
                "{}",
                "Error: could not determine wavelength for antenna".red()
            );
        }
        Ok(())
    }
}

impl SimpleDipole {
    pub fn new(band: FrequencyAllocation) -> Self {
        Self {
            band,
            band_plan: None,
        }
    }

    pub fn new_in_plan(band: FrequencyAllocation, band_plan: BandPlan) -> Self {
        Self {
            band,
            band_plan: Some(band_plan),
        }
    }

    fn band_range(&self) -> Option<FrequencyRange> {
        if let Some(band_plan) = &self.band_plan {
            band_plan
                .band(&self.band)
                .map(|band| band.band().range())
                .cloned()
        } else {
            Some(self.band.total_range())
        }
    }

    pub fn antenna_length(&self) -> Option<Wavelength> {
        if let Some(range) = self.band_range() {
            let mid_band = range.mid_band();
            let wavelength = mid_band.to_wavelength();
            Some(meters(wavelength.value() / 2.0))
        } else {
            None
        }
    }

    pub fn pole_length(&self) -> Option<Wavelength> {
        self.antenna_length().map(|v| meters(v.value() / 2.0))
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
