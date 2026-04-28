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

use crate::{Band, BandPlan, LicenseClass};
use rfham_core::{
    agencies::{agency_ofcom, agency_rsgb},
    countries::country_code_uk,
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
    Band::new_default(Band2200M, Region::One)
}

pub fn band_630m() -> Band {
    Band::new_default(Band630M, Region::One)
}

pub fn band_160m() -> Band {
    Band::new_default(Band160M, Region::One)
}

pub fn band_80m() -> Band {
    Band::new_default(Band80M, Region::One)
}

pub fn band_60m() -> Band {
    Band::new_default(Band60M, Region::One)
}

pub fn band_40m() -> Band {
    Band::new_default(Band40M, Region::One)
}

pub fn band_30m() -> Band {
    Band::new_default(Band30M, Region::One)
}

pub fn band_20m() -> Band {
    Band::new_default(Band20M, Region::One)
}

pub fn band_17m() -> Band {
    Band::new_default(Band17M, Region::One)
}

pub fn band_15m() -> Band {
    Band::new_default(Band15M, Region::One)
}

pub fn band_12m() -> Band {
    Band::new_default(Band12M, Region::One)
}

pub fn band_10m() -> Band {
    Band::new_default(Band10M, Region::One)
}

pub fn band_6m() -> Band {
    Band::new_default(Band6M, Region::One)
}

pub fn band_2m() -> Band {
    Band::new_default(Band2M, Region::One)
}

pub fn band_1_25m() -> Band {
    Band::new_default(Band1_25M, Region::One)
}

pub fn band_70cm() -> Band {
    Band::new_default(Band70Cm, Region::One)
}

pub fn band_33cm() -> Band {
    Band::new_default(Band33Cm, Region::One)
}

pub fn band_23cm() -> Band {
    Band::new_default(Band23Cm, Region::One)
}

pub fn band_13cm() -> Band {
    Band::new_default(Band13Cm, Region::One)
}

pub fn band_9cm() -> Band {
    Band::new_default(Band9Cm, Region::One)
}

pub fn band_5cm() -> Band {
    Band::new_default(Band5Cm, Region::One)
}

pub fn band_3cm() -> Band {
    Band::new_default(Band3Cm, Region::One)
}

pub fn band_1_2cm() -> Band {
    Band::new_default(Band1_2Cm, Region::One)
}

pub fn band_6mm() -> Band {
    Band::new_default(Band6Mm, Region::One)
}

pub fn band_4mm() -> Band {
    Band::new_default(Band4Mm, Region::One)
}

pub fn band_2_5mm() -> Band {
    Band::new_default(Band2_5Mm, Region::One)
}

pub fn band_2mm() -> Band {
    Band::new_default(Band2Mm, Region::One)
}

pub fn band_1mm() -> Band {
    Band::new_default(Band1Mm, Region::One)
}

pub fn rsgb_band_plan() -> BandPlan {
    BandPlan::new(agency_rsgb(), Region::One, "UK Amateur Radio Band Plan")
        .with_regulator(agency_ofcom())
        .in_country(country_code_uk())
        .with_licenses(
            vec![
                ("f".to_string(), LicenseClass::new(1, "Foundation", true)),
                (
                    "fc".to_string(),
                    LicenseClass::new(1, "Foundation (Club)", true),
                ),
                ("I".to_string(), LicenseClass::new(2, "Intermediate", true)),
                (
                    "Ic".to_string(),
                    LicenseClass::new(2, "Intermediate (Club)", true),
                ),
                ("F".to_string(), LicenseClass::new(3, "Full", true)),
                ("Fc".to_string(), LicenseClass::new(3, "Full (Club)", true)),
            ]
            .into_iter()
            .collect(),
        )
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
