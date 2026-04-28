//! Single-crate re-export facade for the entire RF-Ham library suite.
//!
//! Add `rfham` as your only dependency to get access to all `rfham_*` crate types
//! without listing each crate individually.
//!
//! | Module | Re-exports from |
//! |--------|-----------------|
//! | [`core`] | `rfham_core` — callsigns, frequency, power, agency, country |
//! | [`geo`] | `rfham_geo` + `rfham_maidenhead` — grid traits, Maidenhead locator |
//! | [`itu`] | `rfham_itu` — ITU bands, regions, allocations, callsign series |
//! | [`bands`] | `rfham_bands` — band plans, segments, restrictions |
//! | [`config`] | `rfham_config` — station configuration |
//! | [`antennas`] | `rfham_antennas` — antenna types and dipole calculator |
//! | [`markdown`] | `rfham_markdown` — Markdown output traits and functions |

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod core {
    pub use rfham_core::agencies::{self, Agency};
    pub use rfham_core::callsigns::CallSign;
    pub use rfham_core::countries::{self, CountryCode, CountryCodeNumeric};
    pub use rfham_core::error::CoreError;
    pub use rfham_core::fmt;
    pub use rfham_core::frequencies::{self, Frequency, FrequencyRange, Wavelength};
    pub use rfham_core::names::{self, Name};
    pub use rfham_core::non_si;
    pub use rfham_core::power::{self, Power};
}

pub mod geo {
    pub use rfham_geo::grid::{GridIdentifier, GridPolygon, GridSystem};
    pub use rfham_maidenhead::{
        Maidenhead, MaidenheadLocator, MaidenheadPrecision, MaidenheadSquare,
    };
}

pub mod itu {
    pub use rfham_itu::allocations::FrequencyAllocation;
    pub use rfham_itu::bands::FrequencyBand;
    pub use rfham_itu::callsigns::{ItuInternationalOrganization, ItuSeriesAllocation};
    pub use rfham_itu::regions::Region;
}

pub mod bands {
    pub use rfham_bands::{
        Band, BandPlan, BandRestrictions, BandwidthRestriction, CallingFrequency, LicenseClass,
        LicenseKey, PlanBand, PowerMeasure, PowerRestriction, RepeaterUsage, SatelliteUsage,
        Segment, UsageRestriction,
    };
    pub use rfham_bands::{uk_rsgb, us_fcc};
}

pub mod config {
    pub use rfham_config::error::ConfigError;
    pub use rfham_config::paths::ConfigPath;
    pub use rfham_config::{
        self, Configuration, Equipment, Location, LocationKind, Mobility, Mode, Station, Usage,
    };
}

pub mod antennas {
    pub use rfham_antennas::AntennaForm;
    pub use rfham_antennas::dipoles::SimpleDipole;
}

pub use rfham_markdown as markdown;
