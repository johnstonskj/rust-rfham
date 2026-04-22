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
    Band, BandPlan, BandRestrictions, LicenseClass, PlanBand, PowerRestriction, Segment,
    UsageRestriction,
};
use rfham_core::{
    agency::{agency_arrl, agency_fcc},
    country::country_code_us,
    frequency::megahertz,
    power::watts,
};
use rfham_itu::{allocations::FrequencyAllocation::*, regions::Region};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn band_2200m() -> Band {
    Band::new_default(Band2200M, Region::Two)
}

pub fn band_630m() -> Band {
    Band::new_default(Band630M, Region::Two)
}

pub fn band_160m() -> Band {
    Band::new_default(Band160M, Region::Two)
}

pub fn band_80m() -> Band {
    Band::new_default(Band80M, Region::Two)
}

pub fn band_60m() -> Band {
    Band::new_default(Band60M, Region::Two)
}

pub fn band_40m() -> Band {
    Band::new_default(Band40M, Region::Two)
}

pub fn band_30m() -> Band {
    Band::new_default(Band30M, Region::Two)
}

pub fn band_20m() -> Band {
    Band::new_default(Band20M, Region::Two)
}

pub fn band_17m() -> Band {
    Band::new_default(Band17M, Region::Two)
}

pub fn band_15m() -> Band {
    Band::new_default(Band15M, Region::Two)
}

pub fn band_12m() -> Band {
    Band::new_default(Band12M, Region::Two)
}

pub fn band_10m() -> Band {
    Band::new_default(Band10M, Region::Two)
}

pub fn band_6m() -> Band {
    Band::new_default(Band6M, Region::Two)
}

pub fn band_2m() -> Band {
    Band::new_default(Band2M, Region::Two)
}

pub fn band_1_25m() -> Band {
    Band::new_default(Band1_25M, Region::Two)
}

pub fn band_70cm() -> Band {
    Band::new_default(Band70Cm, Region::Two)
}

pub fn band_33cm() -> Band {
    Band::new_default(Band33Cm, Region::Two)
}

pub fn band_23cm() -> Band {
    Band::new_default(Band23Cm, Region::Two)
}

pub fn band_13cm() -> Band {
    Band::new_default(Band13Cm, Region::Two)
}

pub fn band_9cm() -> Band {
    Band::new_default(Band9Cm, Region::Two)
}

pub fn band_5cm() -> Band {
    Band::new_default(Band5Cm, Region::Two)
}

pub fn band_3cm() -> Band {
    Band::new_default(Band3Cm, Region::Two)
}

pub fn band_1_2cm() -> Band {
    Band::new_default(Band1_2Cm, Region::Two)
}

pub fn band_6mm() -> Band {
    Band::new_default(Band6Mm, Region::Two)
}

pub fn band_4mm() -> Band {
    Band::new_default(Band4Mm, Region::Two)
}

pub fn band_2_5mm() -> Band {
    Band::new_default(Band2_5Mm, Region::Two)
}

pub fn band_2mm() -> Band {
    Band::new_default(Band2Mm, Region::Two)
}

pub fn band_1mm() -> Band {
    Band::new_default(Band1Mm, Region::Two)
}

pub fn arrl_voluntary_band_plan() -> BandPlan {
    BandPlan::new(agency_arrl(), Region::Two, "US Amateur Radio Bands")
            .with_regulator(agency_fcc())
            .in_country(country_code_us())
            .with_default_max_power(watts(1500.0))
            .with_notes(vec![
                "An amateur station must use the minimum transmitter power necessary to carry out the desired communications.".to_string()
            ])
            .with_licenses(
                vec![
                    ("N".to_string(), LicenseClass::new(1, "Novice", false)),
                    ("T".to_string(), LicenseClass::new(2, "Technician", true)),
                    ("G".to_string(), LicenseClass::new(3, "General", true)),
                    ("A".to_string(), LicenseClass::new(4, "Advanced", false)),
                    ("E".to_string(), LicenseClass::new(5, "Amateur Extra", true)),
                ]
                .into_iter()
                .collect(),
            )
            .with_bands_list(vec![
                PlanBand::new(band_2200m()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data, UsageRestriction::Phone])
                        .with_power_restriction(PowerRestriction::eirp(watts(1.0))),
                ),
                PlanBand::new(band_630m()).with_is_primary_user(false)
                .with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data, UsageRestriction::Phone])
                        .with_power_restriction(PowerRestriction::eirp(watts(5.0))),
                ),
                PlanBand::new(band_160m()).with_restrictions(
                    BandRestrictions::default().with_license_restrictions(
                        vec!["G", "A", "E"]
                            .into_iter()
                            .map(|n| n.to_string())
                            .collect(),
                    )
                    .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data, UsageRestriction::Phone])
,
                )
                .with_notes(vec!["Avoid interference to radiolocation operations from 1.9MHz-2.0MHz.".to_string()]),
                PlanBand::new(band_80m()).with_segments(vec![
                    Segment::new(megahertz(3.5), megahertz(3.6)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data])
                        .with_license_restrictions(vec!["E".to_string()])
                    ),
                    Segment::new(megahertz(3.6), megahertz(4.0)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Phone])
                        .with_license_restrictions(vec!["E".to_string()])
                    ),
                    Segment::new(megahertz(3.525), megahertz(3.6)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data])
                        .with_license_restrictions(vec!["A".to_string()])
                    ),
                    Segment::new(megahertz(3.7), megahertz(4.0)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Phone])
                        .with_license_restrictions(vec!["A".to_string()])
                    ),
                    Segment::new(megahertz(3.525), megahertz(3.6)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data])
                        .with_license_restrictions(vec!["G".to_string()])
                    ),
                    Segment::new(megahertz(3.8), megahertz(4.0)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Phone])
                        .with_license_restrictions(vec!["G".to_string()])
                    ),
                    Segment::new(megahertz(3.525), megahertz(3.6)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::CW])
                        .with_license_restrictions(vec!["N".to_string(), "T".to_string()])
                    ),
                ]),
                PlanBand::new(band_60m()), // TODO: complete segments
                PlanBand::new(band_40m()), // TODO: complete segments
                PlanBand::new(band_30m()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )
                        .with_power_restriction(PowerRestriction::pep(watts(200.0))),
                ),
                PlanBand::new(band_20m()), // TODO: complete segments
                PlanBand::new(band_17m()).with_segments(vec![
                    Segment::new(megahertz(18.068), megahertz(18.110)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data])
                        .with_license_restrictions(vec!["G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),)
                    ),
                    Segment::new(megahertz(18.110), megahertz(18.168)).with_restrictions(
                        BandRestrictions::default()
                        .with_usage_restrictions(vec![UsageRestriction::Phone])
                        .with_license_restrictions(vec!["G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),)
                    ),
                    ]),
                PlanBand::new(band_15m()), // TODO: complete segments
                PlanBand::new(band_12m()), // TODO: complete segments
                PlanBand::new(band_10m()), // TODO: complete segments
                PlanBand::new(band_6m()).with_segments(vec![
                    Segment::new(megahertz(50.0), megahertz(50.1)).with_restrictions(
                        BandRestrictions::default()
                            .with_usage_restrictions(vec![UsageRestriction::CW]),
                    ),
                    Segment::new(megahertz(50.1), megahertz(54.0)),
                ]),
                PlanBand::new(band_2m()).with_segments(vec![
                    Segment::new(megahertz(144.0), megahertz(144.1)).with_restrictions(
                        BandRestrictions::default()
                            .with_usage_restrictions(vec![UsageRestriction::CW]),
                    ),
                    Segment::new(megahertz(144.1), megahertz(148.0)),
                ]),
                PlanBand::new(band_1_25m()), // TODO: complete segments
                PlanBand::new(band_70cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data, UsageRestriction::Phone])
                ),
                PlanBand::new(band_33cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )
                        .with_usage_restrictions(vec![UsageRestriction::Rtty, UsageRestriction::Data, UsageRestriction::Phone])
                ),
                PlanBand::new(band_23cm()), // TODO: complete segments
                PlanBand::new(band_13cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_9cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_5cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_3cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_1_2cm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_6mm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_4mm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_2_5mm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_2mm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
                PlanBand::new(band_1mm()).with_restrictions(
                    BandRestrictions::default()
                        .with_license_restrictions(
                            vec!["T", "G", "A", "E"]
                                .into_iter()
                                .map(|n| n.to_string())
                                .collect(),
                        )),
            ])
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use rfham_markdown::ToMarkdown;

    #[test]
    fn test_write_json_band_plan() {
        let plan = arrl_voluntary_band_plan();
        println!("{}", serde_json::to_string_pretty(&plan).unwrap());
    }

    #[test]
    fn test_write_markdown_band_plan() {
        let plan = arrl_voluntary_band_plan();
        plan.write_markdown(&mut std::io::stdout()).unwrap();
    }
}
