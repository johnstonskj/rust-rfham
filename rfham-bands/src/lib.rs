//! Amateur radio band plan data structures.
//!
//! A [`BandPlan`] groups frequency segments, license classes, calling frequencies, and
//! power/mode restrictions for a specific regulatory authority and ITU region.  Concrete
//! plans (US ARRL, UK RSGB) live in the sub-modules [`us_fcc`] and [`uk_rsgb`].
//!
//! | Type | Purpose |
//! |------|---------|
//! | [`BandPlan`] | Top-level plan with agency metadata and a map of [`PlanBand`]s |
//! | [`Band`] | A frequency range for one ITU allocation |
//! | [`PlanBand`] | A `Band` plus restrictions, segments, and calling frequencies |
//! | [`Segment`] | A sub-range within a band with its own restrictions |
//! | [`BandRestrictions`] | License, usage, power, and bandwidth constraints |
//!
//! # Examples
//!
//! ```rust,no_run
//! use rfham_bands::us_fcc::arrl_voluntary_band_plan;
//! use rfham_markdown::ToMarkdownWith;
//! use std::io::stdout;
//!
//! arrl_voluntary_band_plan()
//!     .write_markdown_with(&mut stdout(), vec![])
//!     .unwrap();
//! ```

use colored::Colorize;
use rfham_core::{
    agencies::Agency,
    countries::CountryCode,
    error::CoreError,
    frequencies::{Frequency, FrequencyRange},
    power::Power,
};
use rfham_itu::{allocations::FrequencyAllocation, regions::Region};
use rfham_markdown::{
    MarkdownError, Table, ToMarkdown, ToMarkdownWith, blank_line, bulleted_list,
    bulleted_list_item, header, link_to_string, numbered_list_item, plain_text,
};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{cmp::Ordering, collections::HashMap, fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type DisplayName = String;
pub type LicenseKey = String;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BandPlan {
    name: DisplayName,
    maintainer: Agency,
    #[serde(skip_serializing_if = "Option::is_none")]
    regulator: Option<Agency>,
    region: Region,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    countries: Vec<CountryCode>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    licenses: HashMap<LicenseKey, LicenseClass>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    bands: HashMap<FrequencyAllocation, PlanBand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_max_power: Option<Power>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    notes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Band {
    allocation: FrequencyAllocation,
    #[serde(flatten)]
    range: FrequencyRange,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PlanBand {
    band: Band,
    is_primary_user: bool,
    #[serde(skip_serializing_if = "BandRestrictions::is_unrestricted")]
    restrictions: BandRestrictions,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    segments: Vec<Segment>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    calling_frequencies: Vec<CallingFrequency>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    notes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Segment {
    #[serde(flatten)]
    range: FrequencyRange,
    #[serde(skip_serializing_if = "BandRestrictions::is_unrestricted")]
    restrictions: BandRestrictions,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    notes: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BandRestrictions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    license_restrictions: Vec<LicenseKey>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    usage_restrictions: Vec<UsageRestriction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power_restriction: Option<PowerRestriction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth_restriction: Option<BandwidthRestriction>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CallingFrequency {
    frequency: Frequency,
    name: Option<DisplayName>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    usage_restrictions: Vec<UsageRestriction>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LicenseClass {
    ordinal: u32,
    name: DisplayName,
    is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum UsageRestriction {
    Beacon,
    CW,
    Data,
    Digital,
    Dx,
    EarthMoonEarth,
    Phone,
    Qrp,
    RemoteControl,
    Repeater(RepeaterUsage),
    Rtty,
    Satellite(SatelliteUsage),
    Simplex,
    SlowScanTv,
    Test(RepeaterUsage),
}

#[derive(Clone, Copy, Debug, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum PhoneMode {
    AmplitudeModulated,
    FrequencyModulated,
    SingleSideband,
    SingleSidebandLower,
    SingleSidebandUpper,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum PowerMeasure {
    #[default]
    PeakEnvelopePower,
    EffectiveRadiatedPower,
    EffectiveIsotropicRadiatedPower,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct PowerRestriction {
    max_power: Power,
    measure: PowerMeasure,
}

#[derive(Clone, Copy, Debug, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum RepeaterUsage {
    Input,
    Output,
}

#[derive(Clone, Copy, Debug, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum SatelliteUsage {
    Downlink,
    Uplink,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct BandwidthRestriction {
    max_bandwidth: Frequency,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const START_COL_WIDTH: usize = 10;
const END_COL_WIDTH: usize = 10;
const CLASS_COL_WIDTH: usize = 14;
const USAGE_COL_WIDTH: usize = 20;
const POWER_COL_WIDTH: usize = 14;
const BANDWIDTH_COL_WIDTH: usize = 14;
const CLASS_OR_USAGE_DEFAULT: &str = "any";
const POWER_DEFAULT: &str = "full power";
const BANDWIDTH_DEFAULT: &str = "-";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ToMarkdownWith for BandPlan {
    type Context = Vec<FrequencyAllocation>;

    fn write_markdown_with<W: std::io::Write>(
        &self,
        writer: &mut W,
        context: Self::Context,
    ) -> Result<(), MarkdownError> {
        header(writer, 1, &self.name)?;
        blank_line(writer)?;

        if let Some(regulator) = self.regulator.as_ref() {
            bulleted_list_item(
                writer,
                1,
                &if let Some(url) = &regulator.url() {
                    format!(
                        "Regulatory Agency: {}",
                        link_to_string(regulator.to_string(), url)
                    )
                } else {
                    format!("Regulatory Agency: {}", regulator)
                },
            )?;
        }
        bulleted_list_item(
            writer,
            1,
            &if let Some(url) = &self.maintainer.url() {
                format!(
                    "Maintaining Agency: {}",
                    link_to_string(self.maintainer.to_string(), url)
                )
            } else {
                format!("Maintaining Agency: {}", self.maintainer)
            },
        )?;
        bulleted_list_item(writer, 1, format!("Region: {:#}", self.region))?;
        if !self.countries.is_empty() {
            bulleted_list_item(
                writer,
                1,
                format!(
                    "Countries: {}",
                    self.countries
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )?;
        }
        if let Some(default_max_power) = &self.default_max_power {
            bulleted_list_item(
                writer,
                1,
                format!("Default maximum power: {default_max_power}"),
            )?;
        }
        blank_line(writer)?;

        if !self.notes.is_empty() {
            plain_text(writer, "Notes:")?;
            blank_line(writer)?;
            bulleted_list(writer, 1, &self.notes)?;
            blank_line(writer)?;
        }

        header(writer, 2, "License Classes")?;
        blank_line(writer)?;

        let mut ordered_licenses = self.licenses.iter().collect::<Vec<_>>();
        ordered_licenses.sort_by(|a, b| a.1.ordinal.cmp(&b.1.ordinal));

        for (number, (key, license)) in ordered_licenses.iter().enumerate() {
            numbered_list_item(
                writer,
                1,
                number,
                format!(
                    "{} ({}){}",
                    license.name,
                    key,
                    if !license.is_active {
                        format!(" {}", "inactive class".italic())
                    } else {
                        String::default()
                    }
                ),
            )?;
        }
        blank_line(writer)?;

        let mut ordered_bands = self.bands.values().collect::<Vec<_>>();
        ordered_bands.sort_by(|a, b| {
            a.band
                .range
                .start()
                .value()
                .partial_cmp(&b.band.range.start().value())
                .unwrap_or(Ordering::Equal)
        });

        for band in ordered_bands {
            if context.is_empty() || context.contains(&band.band.allocation) {
                band.write_markdown(writer)?;
            }
        }

        Ok(())
    }
}

impl BandPlan {
    // TODO: factor this out in the future.
    #[allow(clippy::too_many_arguments)]
    const fn inner_new(
        maintaining_agency: Agency,
        name: String,
        region: Region,
        countries: Vec<CountryCode>,
        licenses: HashMap<String, LicenseClass>,
        bands: HashMap<FrequencyAllocation, PlanBand>,
        default_max_power: Option<Power>,
        notes: Vec<String>,
    ) -> Self {
        Self {
            maintainer: maintaining_agency,
            regulator: None,
            name,
            region,
            countries,
            licenses,
            bands,
            default_max_power,
            notes,
        }
    }

    pub fn new<S: Into<String>>(maintainer: Agency, region: Region, name: S) -> Self {
        Self::inner_new(
            maintainer,
            name.into(),
            region,
            Vec::default(),
            HashMap::default(),
            HashMap::default(),
            Option::default(),
            Vec::default(),
        )
    }

    pub fn with_regulator(mut self, regulator: Agency) -> Self {
        self.regulator = Some(regulator);
        self
    }

    pub fn in_country(mut self, country: CountryCode) -> Self {
        self.countries.push(country);
        self
    }

    pub fn in_countries(mut self, countries: &[CountryCode]) -> Self {
        self.countries.extend(countries.to_vec());
        self
    }

    pub fn with_licenses(mut self, licenses: HashMap<LicenseKey, LicenseClass>) -> Self {
        self.licenses = licenses;
        self
    }

    pub fn with_license_list(mut self, licenses: Vec<(DisplayName, bool)>) -> Self {
        let licenses: HashMap<LicenseKey, LicenseClass> = licenses
            .into_iter()
            .enumerate()
            .map(|(ordinal, (name, is_active))| {
                (
                    name.clone(),
                    LicenseClass::new(ordinal as u32, &name, is_active),
                )
            })
            .collect();
        self.licenses = licenses;
        self
    }

    pub fn with_bands(mut self, bands: HashMap<FrequencyAllocation, PlanBand>) -> Self {
        self.bands = bands;
        self
    }

    pub fn with_bands_list(self, bands: Vec<PlanBand>) -> Self {
        self.with_bands(
            bands
                .into_iter()
                .map(|band| (band.band.allocation, band))
                .collect(),
        )
    }

    pub fn with_default_max_power(mut self, default_max_power: Power) -> Self {
        self.default_max_power = Some(default_max_power);
        self
    }

    pub fn with_notes(mut self, notes: Vec<String>) -> Self {
        self.notes = notes;
        self
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn band(&self, itu: &FrequencyAllocation) -> Option<&PlanBand> {
        self.bands.get(itu)
    }

    pub fn maintaining_agency(&self) -> &Agency {
        &self.maintainer
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.allocation, self.range)
    }
}

impl Band {
    pub fn new_default(allocation: FrequencyAllocation, region: Region) -> Self {
        let range = allocation
            .range(region)
            .expect("Could not create band from allocation missing in {region:#}.");
        Self::new(allocation, range.start(), range.end())
    }

    pub fn new(allocation: FrequencyAllocation, start: Frequency, end: Frequency) -> Self {
        assert!(start < end);
        Self {
            allocation,
            range: FrequencyRange::new(start, end),
        }
    }

    pub fn allocation(&self) -> &FrequencyAllocation {
        &self.allocation
    }

    pub fn range(&self) -> &FrequencyRange {
        &self.range
    }
}

// ------------------------------------------------------------------------------------------------

impl ToMarkdown for PlanBand {
    fn write_markdown<W: std::io::Write>(&self, writer: &mut W) -> Result<(), MarkdownError> {
        let actual = &self.band;
        header(
            writer,
            2,
            format!("{} Band ({})", actual.allocation, actual.range),
        )?;
        blank_line(writer)?;

        plain_text(
            writer,
            format!(
                "Note: amateur operators are the {} users on this band.",
                if self.is_primary_user {
                    "primary"
                } else {
                    "secondary"
                }
                .bold()
            ),
        )?;
        blank_line(writer)?;

        if !self.calling_frequencies.is_empty() {
            plain_text(writer, "Calling Frequencies:")?;
            blank_line(writer)?;

            let mut ordered_calling = self.calling_frequencies.iter().collect::<Vec<_>>();
            ordered_calling.sort_by(|a, b| {
                a.frequency
                    .value()
                    .partial_cmp(&b.frequency.value())
                    .unwrap_or(Ordering::Equal)
            });

            for (i, calling) in ordered_calling.iter().enumerate() {
                numbered_list_item(
                    writer,
                    1,
                    i,
                    format!(
                        "{}{}{}",
                        calling.frequency,
                        if let Some(label) = &calling.name {
                            format!(" {label}")
                        } else {
                            String::default()
                        },
                        if !calling.usage_restrictions.is_empty() {
                            format!(
                                " ({})",
                                calling
                                    .usage_restrictions
                                    .iter()
                                    .map(|r| r.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        } else {
                            String::default()
                        }
                    ),
                )?;
            }
        }

        if !self.restrictions.is_unrestricted() || !self.segments.is_empty() {
            plain_text(writer, "Band restrictions:")?;
            blank_line(writer)?;

            let table = Table::new(vec![
                ("Start", START_COL_WIDTH).into(),
                ("End", END_COL_WIDTH).into(),
                ("License Class", CLASS_COL_WIDTH).into(),
                ("Usage/Mode", USAGE_COL_WIDTH).into(),
                ("Power", POWER_COL_WIDTH).into(),
                ("Max Bandwidth", BANDWIDTH_COL_WIDTH).into(),
            ]);
            table.headers(writer)?;

            if !self.restrictions.is_unrestricted() {
                self.restrictions
                    .write_markdown_with(writer, (actual.range.clone(), table.clone()))?;
            }

            if !self.segments.is_empty() {
                let mut ordered_segments = self.segments.iter().collect::<Vec<_>>();
                ordered_segments.sort_by(|a, b| {
                    a.range
                        .start()
                        .value()
                        .partial_cmp(&b.range.start().value())
                        .unwrap_or(Ordering::Equal)
                });

                for segment in ordered_segments {
                    segment.write_markdown_with(writer, (segment.range.clone(), table.clone()))?;
                }
                blank_line(writer)?;
            }
        }

        if !self.notes.is_empty() {
            plain_text(writer, "Notes:")?;
            blank_line(writer)?;

            bulleted_list(writer, 1, &self.notes)?;
        }
        blank_line(writer)?;
        Ok(())
    }
}

impl PlanBand {
    fn inner_new(
        band: Band,
        primary_user: bool,
        restrictions: BandRestrictions,
        segments: Vec<Segment>,
        calling_frequencies: Vec<CallingFrequency>,
        notes: Vec<String>,
    ) -> Self {
        Self {
            band,
            is_primary_user: primary_user,
            restrictions,
            segments,
            calling_frequencies,
            notes,
        }
    }
    pub fn new(band: Band) -> Self {
        Self::inner_new(
            band,
            true,
            BandRestrictions::none(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
        )
    }
    pub fn with_is_primary_user(mut self, is_primary_user: bool) -> Self {
        self.is_primary_user = is_primary_user;
        self
    }
    pub fn with_restrictions(mut self, restrictions: BandRestrictions) -> Self {
        self.restrictions = restrictions;
        self
    }
    pub fn with_segments(mut self, segments: Vec<Segment>) -> Self {
        self.segments = segments;
        self
    }
    pub fn with_calling_frequencies(mut self, calling_frequencies: Vec<CallingFrequency>) -> Self {
        self.calling_frequencies = calling_frequencies;
        self
    }
    pub fn with_notes(mut self, notes: Vec<String>) -> Self {
        self.notes = notes;
        self
    }

    pub fn band(&self) -> &Band {
        &self.band
    }
}

// ------------------------------------------------------------------------------------------------

impl ToMarkdownWith for Segment {
    type Context = (FrequencyRange, Table);

    fn write_markdown_with<W: std::io::Write>(
        &self,
        writer: &mut W,
        context: Self::Context,
    ) -> Result<(), MarkdownError> {
        let (range, table) = context;
        if self.restrictions.is_unrestricted() {
            table.data_row(
                writer,
                &[
                    range.start().to_string(),
                    range.end().to_string(),
                    CLASS_OR_USAGE_DEFAULT.italic().to_string(),
                    CLASS_OR_USAGE_DEFAULT.italic().to_string(),
                    POWER_DEFAULT.italic().to_string(),
                    BANDWIDTH_DEFAULT.to_string(),
                ],
            )?;
        } else {
            table.data_row(
                writer,
                &[
                    range.start().to_string(),
                    range.end().to_string(),
                    if self.restrictions.license_restrictions.is_empty() {
                        CLASS_OR_USAGE_DEFAULT.to_string()
                    } else {
                        #[allow(clippy::iter_cloned_collect)] // false positive
                        self.restrictions
                            .license_restrictions
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                    if self.restrictions.usage_restrictions.is_empty() {
                        CLASS_OR_USAGE_DEFAULT.to_string()
                    } else if self.restrictions.usage_restrictions.len() == 1 {
                        format!(
                            "{} only",
                            self.restrictions.usage_restrictions.first().unwrap()
                        )
                    } else {
                        #[allow(clippy::iter_cloned_collect)] // false positive
                        self.restrictions
                            .usage_restrictions
                            .iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                    if let Some(power_restriction) = self.restrictions.power_restriction() {
                        power_restriction.to_string()
                    } else {
                        POWER_DEFAULT.to_string()
                    },
                    if let Some(bandwidth_restriction) = self.restrictions.bandwidth_restriction() {
                        bandwidth_restriction.to_string()
                    } else {
                        BANDWIDTH_DEFAULT.to_string()
                    },
                ],
            )?;
        }
        Ok(())
    }
}

impl Segment {
    pub fn new(frequency_start: Frequency, frequency_end: Frequency) -> Self {
        Self {
            range: FrequencyRange::new(frequency_start, frequency_end),
            restrictions: Default::default(),
            notes: Vec::default(),
        }
    }
    pub fn with_restrictions(mut self, restrictions: BandRestrictions) -> Self {
        self.restrictions = restrictions;
        self
    }
    pub fn with_notes(mut self, notes: Vec<String>) -> Self {
        self.notes = notes;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl ToMarkdownWith for BandRestrictions {
    type Context = (FrequencyRange, Table);

    fn write_markdown_with<W: std::io::Write>(
        &self,
        writer: &mut W,
        context: Self::Context,
    ) -> Result<(), MarkdownError> {
        let (range, table) = context;
        table.data_row(
            writer,
            &[
                range.start().to_string(),
                range.end().to_string(),
                if self.license_restrictions.is_empty() {
                    CLASS_OR_USAGE_DEFAULT.to_string()
                } else {
                    #[allow(clippy::iter_cloned_collect)] // false positive
                    self.license_restrictions
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                },
                if self.usage_restrictions.is_empty() {
                    CLASS_OR_USAGE_DEFAULT.to_string()
                } else if self.usage_restrictions.len() == 1 {
                    format!("{} only", self.usage_restrictions.first().unwrap())
                } else {
                    #[allow(clippy::iter_cloned_collect)] // false positive
                    self.usage_restrictions
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                },
                if let Some(power_restriction) = self.power_restriction {
                    power_restriction.to_string()
                } else {
                    POWER_DEFAULT.to_string()
                },
                if let Some(bandwidth_restriction) = self.bandwidth_restriction() {
                    bandwidth_restriction.to_string()
                } else {
                    BANDWIDTH_DEFAULT.to_string()
                },
            ],
        )?;
        blank_line(writer)?;
        Ok(())
    }
}

impl BandRestrictions {
    pub fn none() -> Self {
        Self::default()
    }
    pub fn with_license_restrictions(mut self, license_restrictions: Vec<LicenseKey>) -> Self {
        self.license_restrictions = license_restrictions;
        self
    }
    pub fn with_usage_restrictions(mut self, usage_restrictions: Vec<UsageRestriction>) -> Self {
        self.usage_restrictions = usage_restrictions;
        self
    }
    pub fn with_power_restriction(mut self, power_restriction: PowerRestriction) -> Self {
        self.power_restriction = Some(power_restriction);
        self
    }
    pub fn with_bandwidth_restriction(mut self, max_bandwidth: BandwidthRestriction) -> Self {
        self.bandwidth_restriction = Some(max_bandwidth);
        self
    }
    pub fn is_unrestricted(&self) -> bool {
        self.license_restrictions.is_empty()
            && self.usage_restrictions.is_empty()
            && self.power_restriction.is_none()
    }
    pub fn license_restrictions(&self) -> impl Iterator<Item = &LicenseKey> {
        self.license_restrictions.iter()
    }
    pub fn usage_restrictions(&self) -> impl Iterator<Item = &UsageRestriction> {
        self.usage_restrictions.iter()
    }
    pub fn power_restriction(&self) -> Option<&PowerRestriction> {
        self.power_restriction.as_ref()
    }
    pub fn bandwidth_restriction(&self) -> Option<&BandwidthRestriction> {
        self.bandwidth_restriction.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for CallingFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.3}: Calling{}",
            self.frequency,
            if self.usage_restrictions.is_empty() {
                String::default()
            } else {
                self.usage_restrictions
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        )
    }
}

impl CallingFrequency {
    pub const fn new(frequency: Frequency, usage_restrictions: Vec<UsageRestriction>) -> Self {
        Self {
            frequency,
            name: None,
            usage_restrictions,
        }
    }
    pub const fn unrestricted(frequency: Frequency) -> Self {
        Self {
            frequency,
            name: None,
            usage_restrictions: Vec::new(),
        }
    }
    pub fn with_usage_restrictions(mut self, usage_restrictions: Vec<UsageRestriction>) -> Self {
        self.usage_restrictions = usage_restrictions;
        self
    }
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.name = Some(label.into());
        self
    }
    pub const fn frequency(&self) -> Frequency {
        self.frequency
    }
    pub fn usage_restrictions(&self) -> impl Iterator<Item = &UsageRestriction> {
        self.usage_restrictions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LicenseClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                format!(
                    "{:02}: {} ({})",
                    self.ordinal,
                    self.name,
                    if self.is_active { "active" } else { "inactive" }
                )
            } else {
                format!(
                    "{}{}",
                    self.name,
                    if self.is_active { "" } else { " (inactive)" }
                )
            }
        )
    }
}

impl LicenseClass {
    pub fn new(ordinal: u32, name: &str, is_active: bool) -> Self {
        Self {
            name: name.to_string(),
            ordinal,
            is_active,
        }
    }
    pub fn active(ordinal: u32, name: &str) -> Self {
        Self {
            name: name.to_string(),
            ordinal,
            is_active: true,
        }
    }
    pub fn inactive(ordinal: u32, name: &str) -> Self {
        Self {
            name: name.to_string(),
            ordinal,
            is_active: false,
        }
    }

    pub fn name(&self) -> &DisplayName {
        &self.name
    }

    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for UsageRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UsageRestriction::Beacon => "beacon".to_string(),
                UsageRestriction::CW => "CW".to_string(),
                UsageRestriction::Data => "data".to_string(),
                UsageRestriction::Digital => "digital".to_string(),
                UsageRestriction::Dx => "DX".to_string(),
                UsageRestriction::EarthMoonEarth => "EME".to_string(),
                UsageRestriction::Phone => "phone".to_string(),
                UsageRestriction::Qrp => "QRP".to_string(),
                UsageRestriction::RemoteControl => "remote-control".to_string(),
                UsageRestriction::Repeater(repeater_usage) => format!("repeater {repeater_usage}"),
                UsageRestriction::Rtty => "RTTY".to_string(),
                UsageRestriction::Satellite(satellite_usage) =>
                    format!("satellite {satellite_usage}"),
                UsageRestriction::Simplex => "simplex".to_string(),
                UsageRestriction::SlowScanTv => "SSTV".to_string(),
                UsageRestriction::Test(repeater_usage) => format!("test-pair {repeater_usage}"),
            }
        )
    }
}

impl FromStr for UsageRestriction {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "beacon" => Ok(Self::Beacon),
            "CW" => Ok(Self::CW),
            "data" => Ok(Self::Data),
            "digital" => Ok(Self::Digital),
            "DX" => Ok(Self::Dx),
            "EQE" => Ok(Self::EarthMoonEarth),
            "QRP" => Ok(Self::Qrp),
            "remote-control" => Ok(Self::RemoteControl),
            "RTTY" => Ok(Self::Rtty),
            "simplex" => Ok(Self::Simplex),
            "SSTV" => Ok(Self::SlowScanTv),
            _ => {
                if let Some(repeater) = s.strip_prefix("repeater") {
                    Ok(Self::Repeater(RepeaterUsage::from_str(repeater.trim())?))
                } else if let Some(satellite) = s.strip_prefix("satellite") {
                    Ok(Self::Satellite(SatelliteUsage::from_str(satellite.trim())?))
                } else if let Some(test_pair) = s.strip_prefix("test-pair") {
                    Ok(Self::Test(RepeaterUsage::from_str(test_pair.trim())?))
                } else {
                    Err(CoreError::InvalidValueFromStr(
                        s.to_string(),
                        "UsageRestriction",
                    ))
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PhoneMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (PhoneMode::AmplitudeModulated, true) => "Amplitude Modulation (AM)",
                (PhoneMode::AmplitudeModulated, false) => "AM",
                (PhoneMode::FrequencyModulated, true) => "FrequencyModulation (FM)",
                (PhoneMode::FrequencyModulated, false) => "FM",
                (PhoneMode::SingleSideband, true) => "Single Sideband (SSB)",
                (PhoneMode::SingleSideband, false) => "SSB",
                (PhoneMode::SingleSidebandLower, true) => "Lower Sideband (LSB)",
                (PhoneMode::SingleSidebandLower, false) => "LSB",
                (PhoneMode::SingleSidebandUpper, true) => "Upper Sideband (USB)",
                (PhoneMode::SingleSidebandUpper, false) => "USB",
            }
        )
    }
}

impl FromStr for PhoneMode {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AM" => Ok(Self::AmplitudeModulated),
            "FM" => Ok(Self::FrequencyModulated),
            "SSB" => Ok(Self::SingleSideband),
            "LSB" => Ok(Self::SingleSidebandLower),
            "USB" => Ok(Self::SingleSidebandUpper),
            _ => Err(CoreError::InvalidValueFromStr(s.to_string(), "PhoneMode")),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PowerMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (PowerMeasure::PeakEnvelopePower, true) => "Peak Envelope Power (PEP)",
                (PowerMeasure::PeakEnvelopePower, false) => "PEP",
                (PowerMeasure::EffectiveRadiatedPower, true) => "Effective Radiated Power (ERP)",
                (PowerMeasure::EffectiveRadiatedPower, false) => "ERP",
                (PowerMeasure::EffectiveIsotropicRadiatedPower, true) =>
                    "Effective Isotropic Radiated Power (EIRP)",
                (PowerMeasure::EffectiveIsotropicRadiatedPower, false) => "EIRP",
            }
        )
    }
}

impl FromStr for PowerMeasure {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PEP" => Ok(Self::PeakEnvelopePower),
            "ERP" => Ok(Self::EffectiveRadiatedPower),
            "EIRP" => Ok(Self::EffectiveIsotropicRadiatedPower),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "PowerMeasure",
            )),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PowerRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.max_power, self.measure)
    }
}

impl<T> From<T> for PowerRestriction
where
    T: Into<Power>,
{
    fn from(value: T) -> Self {
        Self::pep(value.into())
    }
}

impl PowerRestriction {
    pub fn new(max_power: Power, measure: PowerMeasure) -> Self {
        Self { max_power, measure }
    }
    pub fn pep(max_power: Power) -> Self {
        Self {
            max_power,
            measure: PowerMeasure::PeakEnvelopePower,
        }
    }
    pub fn erp(max_power: Power) -> Self {
        Self {
            max_power,
            measure: PowerMeasure::EffectiveRadiatedPower,
        }
    }
    pub fn eirp(max_power: Power) -> Self {
        Self {
            max_power,
            measure: PowerMeasure::EffectiveIsotropicRadiatedPower,
        }
    }
    pub fn max_power(&self) -> Power {
        self.max_power
    }
    pub fn measure(&self) -> PowerMeasure {
        self.measure
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RepeaterUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RepeaterUsage::Input => "input",
                RepeaterUsage::Output => "output",
            }
        )
    }
}

impl FromStr for RepeaterUsage {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "input" => Ok(Self::Input),
            "output" => Ok(Self::Output),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "RepeaterUsage",
            )),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SatelliteUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SatelliteUsage::Downlink => "downlink",
                SatelliteUsage::Uplink => "uplink",
            }
        )
    }
}

impl FromStr for SatelliteUsage {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "downlink" => Ok(Self::Downlink),
            "uplink" => Ok(Self::Uplink),
            _ => Err(CoreError::InvalidValueFromStr(
                s.to_string(),
                "SatelliteUsage",
            )),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for BandwidthRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max_bandwidth)
    }
}

impl From<Frequency> for BandwidthRestriction {
    fn from(value: Frequency) -> Self {
        Self::new(value)
    }
}

impl From<f64> for BandwidthRestriction {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl BandwidthRestriction {
    pub fn new<F: Into<Frequency>>(max_bandwidth: F) -> Self {
        Self {
            max_bandwidth: max_bandwidth.into(),
        }
    }

    pub fn max_bandwidth(&self) -> Frequency {
        self.max_bandwidth
    }
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------
pub mod uk_rsgb;
pub mod us_fcc;
