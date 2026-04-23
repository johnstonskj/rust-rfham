//!
//! This package provides a one-stop location for the set of rfham set of crate types
//! and functions.
//!
//! This package only re-exports types from the set of `rfham_*` packages. The goal is
//! to provide a single dependency for simpler consumers.
//!

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod core {
    pub use rfham_core::agency::{self, agency::Agency};
    pub use rfham_core::callsign::CallSign;
    pub use rfham_core::conversions;
    pub use rfham_core::country::{self, CountryCode, CountryCodeNumeric};
    pub use rfham_core::error::CoreError;
    pub use rfham_core::fmt;
    pub use rfham_core::frequency::{self, Frequency, FrequencyRange, Wavelength};
    pub use rfham_core::id::{self, Name};
    pub use rfham_core::power::{self, Power};
}

pub mod geo {
    pub use rfham_geo::grid::{GridIdentifier, GridPolygon, GridSystem};
    pub use rfham_geo::maidenhead::{
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
    pub use rfham_antennas::dipole::SimpleDipole;
}

pub use rfham_markdown as markdown;
