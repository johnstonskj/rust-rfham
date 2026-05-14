use crate::{
    callsign::{
        Address, CallSignInfo, CallSignInfoProvider, CallSignInfoService, CallSigns, DxInfo,
        GeoSource, LicenseInfo, PersonName,
    },
    error::ServiceError,
};
use lat_long::{Coordinate, Latitude, Longitude};
use rfham_config::get_global_config;
use rfham_core::{callsigns::CallSign, licenses::LicenseKey};
use rfham_maidenhead::MaidenheadLocator;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::RwLock};
use tracing::{error, info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) struct Lookup {
    session: RwLock<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "QRZDatabase")]
struct Response {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "Callsign")]
    callsign: Option<CallsignResponse>,
    #[serde(rename = "Session")]
    session: SessionResponse,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct SessionResponse {
    #[serde(rename = "Error")]
    error: Option<String>,
    #[serde(rename = "Key")]
    key: Option<String>,
    #[serde(rename = "Count")]
    count: Option<u64>,
    #[serde(rename = "SubExp")]
    sub_exp: Option<String>,
    #[serde(rename = "GMTime")]
    gm_time: String,
    #[serde(rename = "Remark")]
    remark: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct CallsignResponse {
    serial: Option<String>,
    user: Option<CallSign>,
    call: CallSign,
    xref: Option<CallSign>,
    aliases: Option<String>,
    p_call: Option<CallSign>,
    dxcc: Option<u32>,
    ccode: Option<u32>,
    land: Option<String>,
    fname: String,
    name: String,
    nickname: Option<String>,
    name_fmt: Option<String>,
    attn: Option<String>,
    addr1: Option<String>,
    addr2: Option<String>,
    state: Option<String>,
    fips: Option<String>,
    county: Option<String>,
    zip: Option<String>,
    country: String,
    lat: Option<Latitude>,
    #[serde(rename = "lon")]
    long: Option<Longitude>,
    grid: Option<MaidenheadLocator>,
    cqzone: Option<u32>,
    ituzone: Option<u32>,
    geoloc: Option<GeoSource>,
    #[serde(rename = "MSA")]
    msa: Option<String>,
    #[serde(rename = "AreaCode")]
    area_code: Option<u32>,
    iota: Option<String>,
    efdate: Option<String>,
    expdate: Option<String>,
    class: Option<String>,
    codes: Option<String>,
    qslmgr: Option<String>,
    eqsl: Option<String>,
    mqsl: Option<String>,
    lotw: Option<String>,
    email: Option<String>,
    u_views: Option<u64>,
    bio: Option<String>,
    biodate: Option<String>,
    moddate: Option<String>,
    image: Option<String>,
    imageinfo: Option<String>,
    born: Option<u32>,
    #[serde(rename = "TimeZone")]
    time_zone: Option<String>,
    #[serde(rename = "GMTOffset")]
    gmt_offset: Option<String>,
    #[serde(rename = "DST")]
    dst: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Service
// ------------------------------------------------------------------------------------------------

const QRZ_LOGIN_URL: &str =
    "https://xmldata.qrz.com/xml/current/?username={u};password={p};agent={a}";

const QRZ_LOOKUP_URL: &str = "https://xmldata.qrz.com/xml/current/?s={s};callsign={c}";

impl CallSignInfoService for Lookup {
    fn lookup(&self, callsign: &CallSign) -> Result<CallSignInfo, ServiceError> {
        trace!("Lookup::lookup({callsign}), {:#?}", self.session);
        if !self.has_session()? {
            trace!("Need to acquire a new session");
            let config = get_global_config()?;
            if let Some(credentials) = config.services().get_credentials(&self.provider().into())? {
                self.login(credentials.user_name(), credentials.password())?;
            } else {
                error!("No credentials found for service {}", self.provider());
                return Err(ServiceError::MissingCredentials(
                    self.provider().to_string(),
                ));
            }
        }
        self.query(callsign)
    }

    fn provider(&self) -> CallSignInfoProvider {
        CallSignInfoProvider::Qrz
    }
}

impl Lookup {
    fn has_session(&self) -> Result<bool, ServiceError> {
        Ok(!self
            .session
            .read()
            .map_err(|e| ServiceError::Poison(e.to_string()))?
            .is_empty())
    }

    fn login(&self, user_name: &str, password: &str) -> Result<(), ServiceError> {
        let request_url = QRZ_LOGIN_URL
            .replace("{u}", user_name)
            .replace("{p}", password)
            .replace("{a}", &crate::user_agent_string());
        let response = reqwest::blocking::get(request_url)?;
        if response.status().is_success() {
            let body = response.text()?;
            let parsed: Response = serde_xml_rs::from_str(&body)?;
            trace!("parsed response: {parsed:?}");

            if let Some(key) = &parsed.session.key {
                let mut session_key = self
                    .session
                    .write()
                    .map_err(|e| ServiceError::Poison(e.to_string()))?;
                info!("Saving session key {key}");
                *session_key = key.clone();
                Ok(())
            } else if let Some(error) = &parsed.session.error {
                error!("service response: {:?}", error);
                return Err(ServiceError::Authentication(
                    CallSignInfoProvider::Qrz.to_string(),
                    error.to_string(),
                ));
            } else {
                error!("Response was not an error, but also failed to provide a session key");
                return Err(ServiceError::Authentication(
                    CallSignInfoProvider::Qrz.to_string(),
                    "".to_string(),
                ));
            }
        } else {
            error!("Legacy::lookup => status: {}", response.status());
            return Err(ServiceError::Http(response.status()));
        }
    }

    fn query(&self, callsign: &CallSign) -> Result<CallSignInfo, ServiceError> {
        assert!(self.has_session()?);
        let request_url = QRZ_LOOKUP_URL
            .replace(
                "{s}",
                self.session
                    .read()
                    .map_err(|e| ServiceError::Poison(e.to_string()))?
                    .as_str(),
            )
            .replace("{c}", &callsign.to_string());
        info!("query for callsign; url: {request_url}");
        let response = reqwest::blocking::get(request_url)?;
        if response.status().is_success() {
            let body = response.text()?;
            let parsed: Response = serde_xml_rs::from_str(&body)?;
            trace!("parsed response: {parsed:?}");
            if let Some(info) = parsed.callsign {
                Ok(CallSignInfo::try_from(info)?)
            } else if let Some(error) = &parsed.session.error {
                error!("service response: {:?}", error);
                return Err(ServiceError::Authentication(
                    CallSignInfoProvider::Qrz.to_string(),
                    error.to_string(),
                ));
            } else {
                unreachable!()
            }
        } else {
            error!("Legacy::lookup => status: {}", response.status());
            return Err(ServiceError::Http(response.status()));
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data
// ------------------------------------------------------------------------------------------------

impl TryFrom<&CallsignResponse> for CallSigns {
    type Error = ServiceError;

    fn try_from(response: &CallsignResponse) -> Result<Self, Self::Error> {
        Ok(Self::new(response.call.clone())
            .with_queried(response.xref.as_ref().cloned())
            .with_previous(response.p_call.as_ref().cloned())
            .with_aliases(
                response
                    .aliases
                    .as_ref()
                    .map(|s| {
                        s.split(' ')
                            .map(|s| CallSign::from_str(&s.trim()).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| Vec::new()),
            ))
    }
}

impl TryFrom<&CallsignResponse> for LicenseInfo {
    type Error = ServiceError;

    fn try_from(response: &CallsignResponse) -> Result<Self, Self::Error> {
        Ok(LicenseInfo::new(
            response
                .class
                .as_ref()
                .map(|s| LicenseKey::from_str(s).unwrap())
                .unwrap(),
        )
        .with_codes(response.codes.as_ref().cloned())
        .with_effective_date(response.efdate.as_ref().cloned())
        .with_expiration_date(response.expdate.as_ref().cloned()))
    }
}

impl TryFrom<&CallsignResponse> for DxInfo {
    type Error = ServiceError;

    fn try_from(response: &CallsignResponse) -> Result<Self, Self::Error> {
        Ok(DxInfo::new(
            response.dxcc.unwrap(),
            response.land.as_ref().cloned().unwrap(),
            response.ccode.unwrap(),
        ))
    }
}

impl TryFrom<&CallsignResponse> for PersonName {
    type Error = ServiceError;

    fn try_from(response: &CallsignResponse) -> Result<Self, Self::Error> {
        Ok(PersonName::new(
            response.fname.split(' ').collect::<Vec<_>>(),
            response.name.clone(),
        )
        .with_nick_name(response.nickname.as_ref().cloned()))
    }
}

impl TryFrom<&CallsignResponse> for Address {
    type Error = ServiceError;

    fn try_from(response: &CallsignResponse) -> Result<Self, Self::Error> {
        Ok(Address::new(response.country.to_string())
            .with_for_attention(response.attn.as_ref().cloned())
            .with_street_1(response.addr1.as_ref().cloned())
            .with_street_2(response.addr2.as_ref().cloned())
            .with_county(response.county.as_ref().cloned())
            .with_region(response.state.as_ref().cloned())
            .with_postal_code(response.zip.as_ref().cloned()))
    }
}

impl TryFrom<CallsignResponse> for CallSignInfo {
    type Error = ServiceError;

    fn try_from(response: CallsignResponse) -> Result<Self, Self::Error> {
        Ok(CallSignInfo::new(
            CallSigns::try_from(&response)?,
            DxInfo::try_from(&response)?,
            PersonName::try_from(&response)?,
            Address::try_from(&response)?,
            response.geoloc.unwrap_or_default(),
        )
        .with_license(if response.class.is_some() {
            Some(LicenseInfo::try_from(&response)?)
        } else {
            None
        })
        .with_email(response.email.as_ref().cloned())
        .with_location(if response.lat.is_some() && response.long.is_some() {
            Some(Coordinate::new(
                response.lat.as_ref().cloned().unwrap(),
                response.long.as_ref().cloned().unwrap(),
            ))
        } else {
            None
        })
        .with_locator(response.grid.as_ref().cloned()))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
