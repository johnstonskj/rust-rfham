use crate::error::ServiceError;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use strum::{AsRefStr, Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait BrowserPageService {
    fn url(&self, page_data: PageData) -> Result<String, ServiceError>;
    fn open(&self, page_data: PageData) -> Result<(), ServiceError> {
        let page_url = self.url(page_data)?;
        open::that(&page_url).map_err(|e| {
            error!("Could not open page {page_url}; error: {e}");
            ServiceError::BrowserOpen(page_url)
        })
    }
    fn provider(&self) -> BrowserPageProvider;
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
pub enum BrowserPageProvider {
    #[default]
    #[strum(serialize = "haminfo-map")]
    HaminfoMap,
    #[strum(serialize = "k7fry-map")]
    K7Fry,
    #[strum(serialize = "zone-check")]
    ZoneCheck,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageData {
    Key(String),
    Args(HashMap<String, String>),
}
// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_provider(
    provider: BrowserPageProvider,
) -> Result<Box<dyn BrowserPageService>, ServiceError> {
    match provider {
        BrowserPageProvider::HaminfoMap => Ok(Box::new(haminfo::MapPage)),
        BrowserPageProvider::K7Fry => Ok(Box::new(k7fry::MapPage)),
        BrowserPageProvider::ZoneCheck => Ok(Box::new(zone_check::ZoneCheckPage)),
    }
}

// ------------------------------------------------------------------------------------------------
// Provider Modules
// ------------------------------------------------------------------------------------------------

mod haminfo {
    use crate::{
        browser::{BrowserPageProvider, BrowserPageService, PageData},
        error::ServiceError,
    };
    use rfham_core::error::CoreError;

    #[derive(Debug, Default)]
    pub(crate) struct MapPage;

    impl BrowserPageService for MapPage {
        fn url(&self, page_data: PageData) -> Result<String, ServiceError> {
            match page_data {
                PageData::Key(callsign) => {
                    Ok(format!("https://haminfo.tetranz.com/map/{}", callsign))
                }
                PageData::Args(args) => Err(CoreError::InvalidValueCtx(
                    format!("{:?}", args),
                    "PageData",
                    "hamnfo::MapPage",
                )
                .into()),
            }
        }

        fn provider(&self) -> BrowserPageProvider {
            BrowserPageProvider::HaminfoMap
        }
    }
}

mod k7fry {
    use crate::{
        browser::{BrowserPageProvider, BrowserPageService, PageData},
        error::ServiceError,
    };
    use rfham_core::error::CoreError;

    #[derive(Debug, Default)]
    pub(crate) struct MapPage;

    impl BrowserPageService for MapPage {
        fn url(&self, page_data: PageData) -> Result<String, ServiceError> {
            match page_data {
                PageData::Key(callsign) => Ok(format!("https://k7fry.com/grid/?qth={}", callsign)),
                PageData::Args(args) => Err(CoreError::InvalidValueCtx(
                    format!("{:?}", args),
                    "PageData",
                    "hamnfo::MapPage",
                )
                .into()),
            }
        }

        fn provider(&self) -> BrowserPageProvider {
            BrowserPageProvider::HaminfoMap
        }
    }
}

mod zone_check {
    use crate::{
        browser::{BrowserPageProvider, BrowserPageService, PageData},
        error::ServiceError,
    };

    #[derive(Debug, Default)]
    pub(crate) struct ZoneCheckPage;

    impl BrowserPageService for ZoneCheckPage {
        fn url(&self, _: PageData) -> Result<String, ServiceError> {
            Ok("https://zone-check.eu/".to_string())
        }

        fn provider(&self) -> BrowserPageProvider {
            BrowserPageProvider::ZoneCheck
        }
    }
}
