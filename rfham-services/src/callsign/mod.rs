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

// https://haminfo.tetranz.com/map/K7SKJ
// https://www.radioreference.com/db/ham/callsign/?cs=K7SKJ
// https://www.radioqth.net/lookup
// https://wireless2.fcc.gov/UlsApp/UlsSearch/searchLicense.jsp

use crate::error::ServiceError;
use lat_long::Coordinate;
use rfham_core::{StringLike, callsigns::CallSign, licenses::LicenseKey, names::Name};
use rfham_maidenhead::MaidenheadLocator;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{AsRefStr, Display as EnumDisplay, EnumIs, EnumIter, EnumString};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait CallSignInfoService {
    fn lookup(&self, callsign: &CallSign) -> Result<CallSignInfo, ServiceError>;
    fn provider(&self) -> CallSignInfoProvider;
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    AsRefStr,
    EnumDisplay,
    EnumIs,
    EnumIter,
    EnumString,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum CallSignInfoProvider {
    #[default]
    #[strum(serialize = "qrz-api")]
    Qrz,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CallSignInfo {
    sign: CallSigns,
    dx: DxInfo,
    name: PersonName,
    address: Address,
    license: Option<LicenseInfo>,
    // `email` email address
    email: Option<String>,
    // `lat` and `lon`
    location: Option<Coordinate>,
    // `grid` grid locator
    locator: Option<MaidenheadLocator>,
    // `geoloc` describes source of lat/long data
    location_source: GeoSource,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CallSigns {
    // `call` callsign
    current: CallSign,
    // `p_call` previous callsign
    previous: Option<CallSign>,
    // `aliases` Other callsigns that resolve to this record
    aliases: Vec<CallSign>,
    queried: Option<CallSign>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LicenseInfo {
    // `class` license class
    class: LicenseKey,
    // `codes` license type codes (USA)
    codes: Option<String>,
    // `efdate` license effective date (USA)
    effective_date: Option<String>,
    // `expdate` license expiration date (USA)
    expiration_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DxInfo {
    // `dxcc` DXCC entity ID (country code) for the callsign
    country_code: u32,
    // `land` DXCC country name of the callsign
    country_name: String,
    // `ccode` DXCC entity code for the mailing address country
    mailing_country_code: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PersonName {
    // `fname` first name
    given: Vec<String>,
    // `name` last name
    family: String,
    // `nickname` A different or shortened name used on the air
    nick_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Address {
    // `attn` Attention address line, this line should be prepended to the address
    attn: Option<String>,
    // `addr1` address line 1 (i.e. house # and street)
    street_1: Option<String>,
    // `addr2` address line 2 (i.e, city name)
    street_2: Option<String>,
    // `county` county name (USA)
    county: Option<String>,
    // `state` state (USA Only)
    region: Option<String>,
    // `zip` Zip/postal code
    postal_code: Option<String>,
    // `country` country name for the QSL mailing address
    country_name: String,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    EnumDisplay,
    EnumIs,
    EnumIter,
    EnumString,
    DeserializeFromStr,
    SerializeDisplay,
)]
pub enum GeoSource {
    /// the value was input by the user
    #[strum(serialize = "user")]
    User,
    /// The value was derived from USA Geocoding data
    #[strum(serialize = "geocode")]
    GeoCode,
    /// The value was derived from a user supplied grid square
    #[strum(serialize = "grid")]
    Grid,
    /// The value was derived from the callsign's USA Zip Code
    #[strum(serialize = "zip")]
    Zip,
    /// The value was derived from the callsign's USA State
    #[strum(serialize = "state")]
    State,
    /// The value was derived from the callsign's DXCC entity (country)
    #[strum(serialize = "dxcc")]
    DxCc,
    /// No value could be determined
    #[default]
    #[strum(serialize = "none")]
    None,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn get_default_provider() -> Result<impl CallSignInfoService, ServiceError> {
    get_provider(CallSignInfoProvider::default())
}

pub fn get_provider(
    provider: CallSignInfoProvider,
) -> Result<impl CallSignInfoService, ServiceError> {
    match provider {
        CallSignInfoProvider::Qrz => Ok(qrz::Lookup::default()),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl CallSignInfo {
    pub const fn new(
        sign: CallSigns,
        dx: DxInfo,
        name: PersonName,
        address: Address,
        location_source: GeoSource,
    ) -> Self {
        Self {
            sign,
            dx,
            name,
            address,
            license: None,
            email: None,
            location: None,
            locator: None,
            location_source,
        }
    }

    pub fn with_license(mut self, license: Option<LicenseInfo>) -> Self {
        self.license = license;
        self
    }

    pub fn with_email(mut self, email: Option<String>) -> Self {
        self.email = email;
        self
    }

    pub fn with_location(mut self, location: Option<Coordinate>) -> Self {
        self.location = location;
        self
    }

    pub fn with_locator(mut self, locator: Option<MaidenheadLocator>) -> Self {
        self.locator = locator;
        self
    }

    pub const fn sign(&self) -> &CallSigns {
        &self.sign
    }

    pub const fn license(&self) -> Option<&LicenseInfo> {
        self.license.as_ref()
    }

    pub const fn dx(&self) -> &DxInfo {
        &self.dx
    }

    pub const fn name(&self) -> &PersonName {
        &self.name
    }

    pub const fn address(&self) -> &Address {
        &self.address
    }

    pub const fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    pub const fn location(&self) -> Option<&Coordinate> {
        self.location.as_ref()
    }

    pub const fn location_source(&self) -> &GeoSource {
        &self.location_source
    }

    pub const fn locator(&self) -> Option<&MaidenheadLocator> {
        self.locator.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl DxInfo {
    pub const fn new(country_code: u32, country_name: String, mailing_country_code: u32) -> Self {
        Self {
            country_code,
            country_name,
            mailing_country_code,
        }
    }

    pub const fn country_code(&self) -> u32 {
        self.country_code
    }

    pub const fn country_name(&self) -> &String {
        &self.country_name
    }

    pub const fn mailing_country_code(&self) -> u32 {
        self.mailing_country_code
    }
}

// ------------------------------------------------------------------------------------------------

impl CallSigns {
    pub const fn new(current: CallSign) -> Self {
        Self {
            current,
            previous: None,
            aliases: Vec::new(),
            queried: None,
        }
    }

    pub fn with_previous(mut self, previous: Option<CallSign>) -> Self {
        self.previous = previous;
        self
    }

    pub fn with_aliases<I: IntoIterator<Item = CallSign>>(mut self, aliases: I) -> Self {
        self.aliases = Vec::from_iter(aliases);
        self
    }

    pub fn with_queried(mut self, queried: Option<CallSign>) -> Self {
        self.queried = queried;
        self
    }

    pub const fn current(&self) -> &CallSign {
        &self.current
    }

    pub const fn previous(&self) -> Option<&CallSign> {
        self.previous.as_ref()
    }

    pub const fn aliases(&self) -> &Vec<CallSign> {
        &self.aliases
    }

    pub const fn queried(&self) -> Option<&CallSign> {
        self.queried.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl LicenseInfo {
    pub const fn new(class: LicenseKey) -> Self {
        Self {
            class,
            codes: None,
            effective_date: None,
            expiration_date: None,
        }
    }

    pub fn with_codes(mut self, codes: Option<String>) -> Self {
        self.codes = codes;
        self
    }

    pub fn with_effective_date(mut self, effective_date: Option<String>) -> Self {
        self.effective_date = effective_date;
        self
    }

    pub fn with_expiration_date(mut self, expiration_date: Option<String>) -> Self {
        self.expiration_date = expiration_date;
        self
    }

    pub fn class(&self) -> &LicenseKey {
        &self.class
    }

    pub fn codes(&self) -> Option<&String> {
        self.codes.as_ref()
    }

    pub fn effective_date(&self) -> Option<&String> {
        self.effective_date.as_ref()
    }

    pub fn expiration_date(&self) -> Option<&String> {
        self.expiration_date.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl PersonName {
    pub fn new<S1, S2>(given_names: Vec<S1>, family_name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            given: given_names.into_iter().map(|i| i.into()).collect(),
            family: family_name.into(),
            nick_name: None,
        }
    }

    pub fn with_nick_name(mut self, nick_name: Option<String>) -> Self {
        self.nick_name = nick_name;
        self
    }

    pub fn given_names(&self) -> &Vec<String> {
        &self.given
    }

    pub fn family_name(&self) -> &String {
        &self.family
    }

    pub fn nick_name(&self) -> Option<&String> {
        self.nick_name.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl Address {
    pub fn new(country_name: String) -> Self {
        Self {
            attn: None,
            street_1: None,
            street_2: None,
            county: None,
            region: None,
            postal_code: None,
            country_name,
        }
    }

    pub fn with_for_attention(mut self, attn: Option<String>) -> Self {
        self.attn = attn;
        self
    }

    pub fn with_street_1(mut self, street_1: Option<String>) -> Self {
        self.street_1 = street_1;
        self
    }

    pub fn with_street_2(mut self, street_2: Option<String>) -> Self {
        self.street_2 = street_2;
        self
    }

    pub fn with_county(mut self, county: Option<String>) -> Self {
        self.county = county;
        self
    }

    pub fn with_region(mut self, region: Option<String>) -> Self {
        self.region = region;
        self
    }

    pub fn with_postal_code(mut self, postal_code: Option<String>) -> Self {
        self.postal_code = postal_code;
        self
    }

    pub fn for_attention(&self) -> Option<&String> {
        self.attn.as_ref()
    }

    pub fn street_line_1(&self) -> Option<&String> {
        self.street_1.as_ref()
    }

    pub fn street_line_2(&self) -> Option<&String> {
        self.street_2.as_ref()
    }

    pub fn county(&self) -> Option<&String> {
        self.county.as_ref()
    }

    pub fn region(&self) -> Option<&String> {
        self.region.as_ref()
    }

    pub fn postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }

    pub fn country_name(&self) -> &String {
        &self.country_name
    }
}

// ------------------------------------------------------------------------------------------------

impl From<CallSignInfoProvider> for Name {
    fn from(value: CallSignInfoProvider) -> Self {
        Self::new_unchecked(value.as_ref())
    }
}

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

mod qrz;
