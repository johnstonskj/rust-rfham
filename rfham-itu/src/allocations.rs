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

use crate::{bands::FrequencyBand, regions::Region};
use core::{fmt::Display, str::FromStr};
use rfham_core::{
    error::CoreError,
    frequencies::{Frequency, FrequencyRange, gigahertz, hertz, kilohertz, megahertz},
};
use rfham_markdown::error::MarkdownError;
use rfham_markdown::{
    Column, ColumnJustification, Table, blank_line, bulleted_list_item, header, link_to_string,
    plain_text,
};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay)]
pub enum FrequencyAllocation {
    Band2200M,
    Band630M,
    Band160M,
    Band80M,
    Band60M,
    Band40M,
    Band30M,
    Band20M,
    Band17M,
    Band15M,
    Band12M,
    Band10M,
    Band8M,
    Band6M,
    Band5M,
    Band4M,
    Band2M,
    Band1_25M,
    Band70Cm,
    Band33Cm,
    Band23Cm,
    Band13Cm,
    Band9Cm,
    Band5Cm,
    Band3Cm,
    Band1_2Cm,
    Band6Mm,
    Band4Mm,
    Band2_5Mm,
    Band2Mm,
    Band1Mm,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for FrequencyAllocation {
    // This is only safe because we call display for the normal format only during the alternate.
    #[allow(clippy::recursive_format_impl)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::Band2200M => format!("{}/{}/135.7kHz", self, self.band()),
                    Self::Band630M => format!("{}/{}/472.0kHz", self, self.band()),
                    Self::Band160M => format!("{}/{}/1.81MHz", self, self.band()),
                    Self::Band80M => format!("{}/{}/3.5MHz", self, self.band()),
                    Self::Band60M => format!("{}/{}/5.3515MHz", self, self.band()),
                    Self::Band40M => format!("{}/{}/7.0MHz", self, self.band()),
                    Self::Band30M => format!("{}/{}/10.1MHz", self, self.band()),
                    Self::Band20M => format!("{}/{}/14.0MHz", self, self.band()),
                    Self::Band17M => format!("{}/{}/18.068MHz", self, self.band()),
                    Self::Band15M => format!("{}/{}/21.0MHz", self, self.band()),
                    Self::Band12M => format!("{}/{}/24.89MHz", self, self.band()),
                    Self::Band10M => format!("{}/{}/28.0MHz", self, self.band()),
                    Self::Band8M => format!("{}/{}/39.9MHz", self, self.band()),
                    Self::Band6M => format!("{}/{}/50.0MHz", self, self.band()),
                    Self::Band5M => format!("{}/{}/59.5.0MHz", self, self.band()),
                    Self::Band4M => format!("{}/{}/69.9MHz", self, self.band()),
                    Self::Band2M => format!("{}/{}/144.0MHz", self, self.band()),
                    Self::Band1_25M => format!("{}/{}/220.0MHz", self, self.band()),
                    Self::Band70Cm => format!("{}/{}/430.0MHz", self, self.band()),
                    Self::Band33Cm => format!("{}/{}/902.0MHz", self, self.band()),
                    Self::Band23Cm => format!("{}/{}/1.24GHz", self, self.band()),
                    Self::Band13Cm => format!("{}/{}/2.3GHz", self, self.band()),
                    Self::Band9Cm => format!("{}/{}/3.3GHz", self, self.band()),
                    Self::Band5Cm => format!("{}/{}/5.65GHz", self, self.band()),
                    Self::Band3Cm => format!("{}/{}/10.0GHz", self, self.band()),
                    Self::Band1_2Cm => format!("{}/{}/24.0GHz", self, self.band()),
                    Self::Band6Mm => format!("{}/{}/47.0GHz", self, self.band()),
                    Self::Band4Mm => format!("{}/{}/76.0GHz", self, self.band()),
                    Self::Band2_5Mm => format!("{}/{}/122.25GHz", self, self.band()),
                    Self::Band2Mm => format!("{}/{}/134.0GHz", self, self.band()),
                    Self::Band1Mm => format!("{}/{}/241.0GHz", self, self.band()),
                }
            } else {
                match self {
                    Self::Band2200M => "2200m",
                    Self::Band630M => "630m",
                    Self::Band160M => "160m",
                    Self::Band80M => "80m",
                    Self::Band60M => "60m",
                    Self::Band40M => "40m",
                    Self::Band30M => "30m",
                    Self::Band20M => "20m",
                    Self::Band17M => "17m",
                    Self::Band15M => "15m",
                    Self::Band12M => "12m",
                    Self::Band10M => "10m",
                    Self::Band8M => "8m",
                    Self::Band6M => "6m",
                    Self::Band5M => "5m",
                    Self::Band4M => "4m",
                    Self::Band2M => "2m",
                    Self::Band1_25M => "1.25m",
                    Self::Band70Cm => "70cm",
                    Self::Band33Cm => "33cm",
                    Self::Band23Cm => "23cm",
                    Self::Band13Cm => "13cm",
                    Self::Band9Cm => "9cm",
                    Self::Band5Cm => "5cm",
                    Self::Band3Cm => "3cm",
                    Self::Band1_2Cm => "1.2cm",
                    Self::Band6Mm => "6mm",
                    Self::Band4Mm => "4mm",
                    Self::Band2_5Mm => "2.5mm",
                    Self::Band2Mm => "2mm",
                    Self::Band1Mm => "1mm",
                }
                .to_string()
            }
        )
    }
}

impl FromStr for FrequencyAllocation {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2200m" => Ok(Self::Band2200M),
            "630m" => Ok(Self::Band630M),
            "160m" => Ok(Self::Band160M),
            "80m" => Ok(Self::Band80M),
            "60m" => Ok(Self::Band60M),
            "40m" => Ok(Self::Band40M),
            "30m" => Ok(Self::Band30M),
            "20m" => Ok(Self::Band20M),
            "17m" => Ok(Self::Band17M),
            "15m" => Ok(Self::Band15M),
            "12m" => Ok(Self::Band12M),
            "10m" => Ok(Self::Band10M),
            "8m" => Ok(Self::Band8M),
            "6m" => Ok(Self::Band6M),
            "5m" => Ok(Self::Band5M),
            "4m" => Ok(Self::Band4M),
            "2m" => Ok(Self::Band2M),
            "1.25m" => Ok(Self::Band1_25M),
            "70cm" => Ok(Self::Band70Cm),
            "33cm" => Ok(Self::Band33Cm),
            "23cm" => Ok(Self::Band23Cm),
            "13cm" => Ok(Self::Band13Cm),
            "9cm" => Ok(Self::Band9Cm),
            "5cm" => Ok(Self::Band5Cm),
            "3cm" => Ok(Self::Band3Cm),
            "1.2cm" => Ok(Self::Band1_2Cm),
            "6mm" => Ok(Self::Band6Mm),
            "4mm" => Ok(Self::Band4Mm),
            "2.5mm" => Ok(Self::Band2_5Mm),
            "2mm" => Ok(Self::Band2Mm),
            "1mm" => Ok(Self::Band1Mm),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "FrequencyAllocation",
            )),
        }
    }
}

impl FrequencyAllocation {
    pub const fn band(&self) -> FrequencyBand {
        match self {
            Self::Band2200M => FrequencyBand::Low,
            Self::Band630M | Self::Band160M => FrequencyBand::Medium,
            Self::Band80M
            | Self::Band60M
            | Self::Band40M
            | Self::Band30M
            | Self::Band20M
            | Self::Band17M
            | Self::Band15M
            | Self::Band12M
            | Self::Band10M => FrequencyBand::High,
            Self::Band8M
            | Self::Band6M
            | Self::Band5M
            | Self::Band4M
            | Self::Band2M
            | Self::Band1_25M => FrequencyBand::VeryHigh,
            Self::Band70Cm | Self::Band33Cm | Self::Band23Cm | Self::Band13Cm => {
                FrequencyBand::UltraHigh
            }
            Self::Band9Cm | Self::Band5Cm | Self::Band3Cm | Self::Band1_2Cm => {
                FrequencyBand::SuperHigh
            }
            Self::Band6Mm | Self::Band4Mm | Self::Band2_5Mm | Self::Band2Mm | Self::Band1Mm => {
                FrequencyBand::ExtremelyHigh
            }
        }
    }

    pub fn range(&self, in_region: Region) -> Option<FrequencyRange> {
        match (in_region, self) {
            // Common to all regions
            (_, Self::Band2200M) => Some(FrequencyRange::new(kilohertz(135.7), kilohertz(137.8))),
            (_, Self::Band630M) => Some(FrequencyRange::new(kilohertz(472.0), kilohertz(479.0))),
            (_, Self::Band60M) => Some(FrequencyRange::new(megahertz(5.3515), megahertz(5.3665))),
            (_, Self::Band40M) => Some(FrequencyRange::new(megahertz(7.0), megahertz(7.3))),
            (_, Self::Band30M) => Some(FrequencyRange::new(megahertz(10.1), megahertz(10.15))),
            (_, Self::Band20M) => Some(FrequencyRange::new(megahertz(14.0), megahertz(14.35))),
            (_, Self::Band17M) => Some(FrequencyRange::new(megahertz(18.068), megahertz(18.168))),
            (_, Self::Band15M) => Some(FrequencyRange::new(megahertz(21.0), megahertz(21.45))),
            (_, Self::Band12M) => Some(FrequencyRange::new(megahertz(24.89), megahertz(24.99))),
            (_, Self::Band10M) => Some(FrequencyRange::new(megahertz(28.0), megahertz(29.7))),
            (_, Self::Band2M) => Some(FrequencyRange::new(megahertz(144.0), megahertz(148.0))),
            (_, Self::Band70Cm) => Some(FrequencyRange::new(megahertz(430.0), megahertz(440.0))),
            (_, Self::Band23Cm) => Some(FrequencyRange::new(gigahertz(1.24), gigahertz(1.3))),
            (_, Self::Band13Cm) => Some(FrequencyRange::new(gigahertz(2.3), gigahertz(2.45))),
            (_, Self::Band3Cm) => Some(FrequencyRange::new(gigahertz(10.0), gigahertz(10.5))),
            (_, Self::Band1_2Cm) => Some(FrequencyRange::new(gigahertz(24.0), gigahertz(24.25))),
            (_, Self::Band6Mm) => Some(FrequencyRange::new(gigahertz(47.0), gigahertz(47.2))),
            (_, Self::Band4Mm) => Some(FrequencyRange::new(gigahertz(76.0), gigahertz(81.5))),
            (_, Self::Band2_5Mm) => Some(FrequencyRange::new(gigahertz(122.25), gigahertz(123.0))),
            (_, Self::Band2Mm) => Some(FrequencyRange::new(gigahertz(134.0), gigahertz(141.0))),
            (_, Self::Band1Mm) => Some(FrequencyRange::new(gigahertz(241.0), gigahertz(250.0))),
            // Region 1 specifics
            (Region::One, Self::Band160M) => {
                Some(FrequencyRange::new(megahertz(1.81), megahertz(2.0)))
            }
            (Region::One, Self::Band80M) => {
                Some(FrequencyRange::new(megahertz(3.5), megahertz(3.8)))
            }
            (Region::One, Self::Band8M) => {
                Some(FrequencyRange::new(megahertz(39.9), megahertz(40.75)))
            }
            (Region::One, Self::Band5M) => {
                Some(FrequencyRange::new(megahertz(59.5), megahertz(60.1)))
            }
            (Region::One, Self::Band4M) => {
                Some(FrequencyRange::new(megahertz(69.9), megahertz(70.5)))
            }
            (Region::One, Self::Band5Cm) => {
                Some(FrequencyRange::new(gigahertz(5.65), gigahertz(5.85)))
            }
            // Common to regions 2 & 3
            (_, Self::Band6M) => Some(FrequencyRange::new(megahertz(50.0), megahertz(54.0))),
            (_, Self::Band9Cm) => Some(FrequencyRange::new(gigahertz(3.3), gigahertz(3.5))),
            // Region 2 specific
            (Region::Two, Self::Band160M) => {
                Some(FrequencyRange::new(megahertz(1.8), megahertz(2.0)))
            }
            (Region::Two, Self::Band80M) => {
                Some(FrequencyRange::new(megahertz(3.5), megahertz(4.0)))
            }
            (Region::Two, Self::Band1_25M) => {
                Some(FrequencyRange::new(megahertz(220.0), megahertz(230.0)))
            }
            (Region::Two, Self::Band33Cm) => {
                Some(FrequencyRange::new(megahertz(902.0), megahertz(928.0)))
            }
            (Region::Two, Self::Band5Cm) => {
                Some(FrequencyRange::new(gigahertz(5.65), gigahertz(5.925)))
            }
            // Region 3 specific
            (Region::Three, Self::Band160M) => {
                Some(FrequencyRange::new(megahertz(1.8), megahertz(2.0)))
            }
            (Region::Three, Self::Band80M) => {
                Some(FrequencyRange::new(megahertz(3.5), megahertz(3.9)))
            }
            (Region::Three, Self::Band5Cm) => {
                Some(FrequencyRange::new(gigahertz(5.65), gigahertz(5.85)))
            }
            (_, _) => None,
        }
    }

    pub fn total_range(&self) -> FrequencyRange {
        let bounds = vec![
            self.range(Region::One),
            self.range(Region::Two),
            self.range(Region::Three),
        ]
        .into_iter()
        .flatten()
        .map(|range| (range.start().value(), range.end().value()))
        .collect::<Vec<_>>();

        // Note: unwrap is safe as there MUST be at least one range for each allocation.
        let start = bounds
            .iter()
            .map(|(start, _)| start)
            .min_by(|l, r| l.total_cmp(r))
            .unwrap();
        let end = bounds
            .iter()
            .map(|(_, end)| end)
            .max_by(|l, r| l.total_cmp(r))
            .unwrap();
        FrequencyRange::new(hertz(*start), hertz(*end))
    }

    pub fn classify(frequency: Frequency) -> Option<Self> {
        match frequency.value() {
            135700.0..137800.0 => Some(Self::Band2200M),
            472000.0..479000.0 => Some(Self::Band630M),
            1800000.0..2000000.0 => Some(Self::Band160M),
            3500000.0..4000000.0 => Some(Self::Band80M),
            5330000.0..5406000.0 => Some(Self::Band60M),
            7000000.0..7300000.0 => Some(Self::Band40M),
            10100000.0..10150000.0 => Some(Self::Band30M),
            14000000.0..14350000.0 => Some(Self::Band20M),
            18068000.0..18168000.0 => Some(Self::Band17M),
            21000000.0..21450000.0 => Some(Self::Band15M),
            24890000.0..24990000.0 => Some(Self::Band12M),
            28000000.0..29700000.0 => Some(Self::Band10M),
            39900000.0..40750000.0 => Some(Self::Band8M),
            50000000.0..54000000.0 => Some(Self::Band6M),
            144000000.0..148000000.0 => Some(Self::Band2M),
            219000000.0..220000000.0 | 222000000.0..225000000.0 => Some(Self::Band1_25M),
            420000000.0..450000000.0 => Some(Self::Band70Cm),
            902000000.0..928000000.0 => Some(Self::Band33Cm),
            1240000000.0..1300000000.0 => Some(Self::Band23Cm),
            2300000000.0..2310000000.0 | 2390000000.0..2450000000.0 => Some(Self::Band13Cm),
            3300000000.0..3500000000.0 => Some(Self::Band9Cm),
            5650000000.0..5925000000.0 => Some(Self::Band5Cm),
            10000000000.0..10500000000.0 => Some(Self::Band3Cm),
            24000000000.0..24250000000.0 => Some(Self::Band1_2Cm),
            47000000000.0..47200000000.0 => Some(Self::Band6Mm),
            75500000000.0..81000000000.0 => Some(Self::Band4Mm),
            122250000000.0..123000000000.0 => Some(Self::Band2_5Mm),
            134000000000.0..141000000000.0 => Some(Self::Band2Mm),
            241000000000.0..250000000000.0 => Some(Self::Band1Mm),
            _ => None,
        }
    }

    pub fn write_markdown<W: Write>(writer: &mut W) -> Result<(), MarkdownError> {
        const NAME_COL_WIDTH: usize = 5;
        const BAND_COL_WIDTH: usize = 5;
        const START_COL_WIDTH: usize = 10;
        const END_COL_WIDTH: usize = 10;

        header(writer, 1, "IARU/ITU Frequency Allocations")?;
        blank_line(writer)?;

        let table = Table::new(vec![
            ("Name", NAME_COL_WIDTH).into(),
            ("Band", BAND_COL_WIDTH).into(),
            Column::new("Start")
                .with_width(START_COL_WIDTH)
                .with_justification(ColumnJustification::Right),
            Column::new("End")
                .with_width(END_COL_WIDTH)
                .with_justification(ColumnJustification::Right),
            Column::new("Start")
                .with_width(START_COL_WIDTH)
                .with_justification(ColumnJustification::Right),
            Column::new("End")
                .with_width(END_COL_WIDTH)
                .with_justification(ColumnJustification::Right),
            Column::new("Start")
                .with_width(START_COL_WIDTH)
                .with_justification(ColumnJustification::Right),
            Column::new("End")
                .with_width(END_COL_WIDTH)
                .with_justification(ColumnJustification::Right),
        ])
        .with_super_labels(vec![
            "",
            "",
            "Region 1",
            "",
            "Region 2",
            "",
            "Region. 3",
            "",
        ]);

        table.headers(writer)?;

        for band in &[
            Self::Band2200M,
            Self::Band630M,
            Self::Band160M,
            Self::Band80M,
            Self::Band60M,
            Self::Band40M,
            Self::Band30M,
            Self::Band20M,
            Self::Band17M,
            Self::Band15M,
            Self::Band12M,
            Self::Band10M,
            Self::Band6M,
            Self::Band2M,
            Self::Band1_25M,
            Self::Band70Cm,
            Self::Band33Cm,
            Self::Band23Cm,
            Self::Band13Cm,
            Self::Band9Cm,
            Self::Band5Cm,
            Self::Band3Cm,
            Self::Band1_2Cm,
            Self::Band6Mm,
            Self::Band4Mm,
            Self::Band2_5Mm,
            Self::Band2Mm,
            Self::Band1Mm,
        ] {
            let region_1 = band.range(Region::One);
            let region_2 = band.range(Region::Two);
            let region_3 = band.range(Region::Three);

            table.data_row(
                writer,
                &[
                    band.to_string(),
                    band.band().to_string(),
                    region_1
                        .as_ref()
                        .map(|r| r.start().to_string())
                        .unwrap_or("-".to_string()),
                    region_1
                        .as_ref()
                        .map(|r| r.end().to_string())
                        .unwrap_or("-".to_string()),
                    region_2
                        .as_ref()
                        .map(|r| r.start().to_string())
                        .unwrap_or("-".to_string()),
                    region_2
                        .as_ref()
                        .map(|r| r.end().to_string())
                        .unwrap_or("-".to_string()),
                    region_3
                        .as_ref()
                        .map(|r| r.start().to_string())
                        .unwrap_or("-".to_string()),
                    region_3
                        .as_ref()
                        .map(|r| r.end().to_string())
                        .unwrap_or("-".to_string()),
                ],
            )?;
        }
        blank_line(writer)?;

        plain_text(writer, "For more information, see:")?;
        blank_line(writer)?;

        bulleted_list_item(
            writer,
            1,
            format!(
                "{}, IARU 2020.",
                link_to_string(
                    "Amateur and Amateur-satellite Service Spectrum",
                    "https://www.iaru.org/wp-content/uploads/2020/01/Amateur-Services-Spectrum-2020_.pdf",
                )
            ),
        )?;
        bulleted_list_item(
            writer,
            1,
            format!(
                "{}, IARU.",
                link_to_string(
                    "Regions",
                    "https://www.iaru.org/about-us/organisation-and-history/regions/",
                )
            ),
        )?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::FrequencyAllocation;
    use rfham_core::frequencies::{kilohertz, megahertz};

    #[test]
    fn test_write_markdown_band_plan() {
        FrequencyAllocation::write_markdown(&mut std::io::stdout()).unwrap();
    }

    #[test]
    fn test_total_range() {
        assert_eq!(
            "3.5 MHz - 4 MHz".to_string(),
            FrequencyAllocation::Band80M.total_range().to_string()
        );
    }

    #[test]
    fn test_frequency_classifier() {
        assert_eq!(None, FrequencyAllocation::classify(kilohertz(130.0)));
        assert_eq!(
            Some(FrequencyAllocation::Band2200M),
            FrequencyAllocation::classify(kilohertz(136.0))
        );
        assert_eq!(
            Some(FrequencyAllocation::Band1_25M),
            // National calling frequency for FM Simplex
            FrequencyAllocation::classify(megahertz(223.500))
        );
    }
}
