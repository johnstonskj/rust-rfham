use crate::{OnceCommand, error::CliError};
use colored::Colorize;
use rfham_core::callsigns::CallSign;
use rfham_geo::grid::GridIdentifier;
use rfham_itu::callsigns::ItuSeriesAllocation;
use rfham_markdown::{blank_line, bulleted_list_item, header, link_to_string, plain_text};
use rfham_services::callsign::{CallSignInfoService, GeoSource, get_default_provider};
use std::process::ExitCode;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ValidateCallSign {
    callsign: String,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO: implement in services
pub struct LookupCallSign {
    callsign: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for ValidateCallSign {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        fn color_true(value: bool) -> String {
            if value {
                format!("**{}**", "true".bold())
            } else {
                "false".to_string()
            }
        }
        let mut writer = std::io::stdout();
        if let Ok(callsign) = CallSign::from_str(&self.callsign) {
            blank_line(&mut writer)?;
            plain_text(
                &mut writer,
                format!(
                    "Callsign {:?} **{}** valid:",
                    self.callsign,
                    "is".bold().green()
                ),
            )?;
            blank_line(&mut writer)?;
            if let Some(a_prefix) = callsign.ancillary_prefix() {
                bulleted_list_item(&mut writer, 1, format!("Ancillary Prefix: {}", a_prefix))?;
            }
            bulleted_list_item(&mut writer, 1, format!("Prefix: {}", callsign.prefix()))?;
            if let Some(allocation) = ItuSeriesAllocation::from_callsign(&callsign) {
                bulleted_list_item(&mut writer, 2, format!("ITU allocation; {allocation:#}"))?;
            } else {
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!("ITU allocation; Unknown/unallocated"),
                )?;
                bulleted_list_item(
                    &mut writer,
                    3,
                    format!(
                        "is non-standard: {}",
                        color_true(callsign.is_prefix_non_standard())
                    ),
                )?;
            }
            bulleted_list_item(
                &mut writer,
                1,
                format!("Separator Numeral: {}", callsign.separator_numeral()),
            )?;
            bulleted_list_item(&mut writer, 1, format!("Suffix: {}", callsign.suffix()))?;
            bulleted_list_item(
                &mut writer,
                2,
                format!("is special: {}", callsign.is_special()),
            )?;
            if let Some(a_suffix) = callsign.ancillary_suffix() {
                bulleted_list_item(&mut writer, 1, format!("Ancillary Suffix: {}", a_suffix))?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!("is portable: {}", color_true(callsign.is_portable())),
                )?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!("is mobile: {}", color_true(callsign.is_mobile())),
                )?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!(
                        "is aeronautical mobile: {}",
                        color_true(callsign.is_aeronautical_mobile())
                    ),
                )?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!(
                        "is maritime mobile: {}",
                        color_true(callsign.is_maritime_mobile())
                    ),
                )?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!(
                        "is at alternate location: {}",
                        color_true(callsign.is_at_alternate_location())
                    ),
                )?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!(
                        "is operating QRP: {}",
                        color_true(callsign.is_operating_qrp())
                    ),
                )?;
                bulleted_list_item(
                    &mut writer,
                    2,
                    format!(
                        "FCC license pending: {}",
                        color_true(callsign.is_fcc_license_pending())
                    ),
                )?;
            }
            Ok(ExitCode::SUCCESS)
        } else {
            plain_text(
                &mut writer,
                format!(
                    "Callsign {:?} is **{}** valid.",
                    self.callsign,
                    "not".bold().red()
                ),
            )?;
            Ok(ExitCode::FAILURE)
        }
    }
}

impl ValidateCallSign {
    pub fn new(callsign: String) -> Self {
        Self { callsign }
    }
}
// ------------------------------------------------------------------------------------------------

impl OnceCommand for LookupCallSign {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let mut writer = std::io::stdout();
        if let Ok(callsign) = CallSign::from_str(&self.callsign) {
            let provider = get_default_provider()?;
            let response = provider.lookup(&callsign)?;
            blank_line(&mut writer)?;
            if let Some(queried) = response.sign().queried() {
                header(
                    &mut writer,
                    1,
                    format!("Callsign {} (from {queried})", response.sign().current()),
                )?;
            } else {
                header(
                    &mut writer,
                    1,
                    format!("Callsign {}", response.sign().current()),
                )?;
            }
            blank_line(&mut writer)?;

            if let Some(previous) = response.sign().previous() {
                bulleted_list_item(&mut writer, 1, format!("Previoous callsign: {previous}"))?;
                blank_line(&mut writer)?;
            }

            if !response.sign().aliases().is_empty() {
                for alias in response.sign().aliases() {
                    bulleted_list_item(&mut writer, 1, format!("Alias: {alias}"))?;
                }
                blank_line(&mut writer)?;
            }

            let email = if let Some(email) = response.email() {
                format!(" {}", link_to_string(email, format!("mailto:{}", email)))
            } else {
                String::new()
            };

            plain_text(
                &mut writer,
                format!(
                    "Registered to {} {}{}{}.",
                    response.name().given_names().join(" "),
                    if let Some(nick_name) = response.name().nick_name() {
                        format!("({}) ", nick_name.italic())
                    } else {
                        String::new()
                    },
                    response.name().family_name(),
                    email,
                ),
            )?;
            blank_line(&mut writer)?;

            if let Some(license) = response.license() {
                if license.codes().is_some()
                    || license.effective_date().is_some()
                    || license.expiration_date().is_some()
                {
                    header(&mut writer, 2, "License")?;
                    blank_line(&mut writer)?;
                    bulleted_list_item(
                        &mut writer,
                        1,
                        format!("Class **{}**", license.class().to_string().bold()),
                    )?;
                    if let Some(codes) = license.codes() {
                        bulleted_list_item(&mut writer, 1, format!("Codes **{}**", codes.bold()))?;
                    }
                    if let Some(effective_date) = license.effective_date() {
                        bulleted_list_item(
                            &mut writer,
                            1,
                            format!("Effective date: **{}**", effective_date.bold()),
                        )?;
                    }
                    if let Some(expiration_date) = license.expiration_date() {
                        bulleted_list_item(
                            &mut writer,
                            1,
                            format!("Expiration date: **{}**", expiration_date.bold()),
                        )?;
                    }
                } else {
                    plain_text(
                        &mut writer,
                        format!("License Class **{}**", license.class().to_string().bold()),
                    )?;
                }
                blank_line(&mut writer)?;
            }

            header(&mut writer, 2, "Mailing Address")?;
            blank_line(&mut writer)?;
            if let Some(attn) = response.address().for_attention() {
                plain_text(&mut writer, format!("> Attn; {attn} \\"))?;
            }
            if let Some(street_1) = response.address().street_line_1() {
                plain_text(&mut writer, format!("> {street_1} \\"))?;
            }
            if let Some(street_2) = response.address().street_line_2() {
                plain_text(&mut writer, format!("> {street_2} \\"))?;
            }
            if let Some(county) = response.address().county() {
                plain_text(&mut writer, format!("> {county} \\"))?;
            }
            if let Some(region) = response.address().region() {
                plain_text(&mut writer, format!("> {region} \\"))?;
            }
            if let Some(postal_code) = response.address().postal_code() {
                plain_text(&mut writer, format!("> {postal_code} \\"))?;
            }
            plain_text(
                &mut writer,
                format!("> {}", response.address().country_name()),
            )?;
            blank_line(&mut writer)?;

            header(&mut writer, 2, "Other Location Information")?;
            blank_line(&mut writer)?;

            bulleted_list_item(
                &mut writer,
                1,
                format!(
                    "DXCC Entity ID: {} ({})",
                    response.dx().country_code(),
                    response.dx().country_name()
                ),
            )?;
            bulleted_list_item(
                &mut writer,
                1,
                format!(
                    "DXCC Mailing Entity ID: {}",
                    response.dx().mailing_country_code()
                ),
            )?;

            if let Some(locator) = response.locator() {
                bulleted_list_item(
                    &mut writer,
                    1,
                    format!(
                        "Grid locator {}",
                        link_to_string(
                            locator.as_str(),
                            format!("https://k7fry.com/grid/?qth={locator}")
                        )
                    ),
                )?;
            }

            if let Some(location) = response.location() {
                bulleted_list_item(
                    &mut writer,
                    1,
                    format!(
                        "Geographic Location {}",
                        link_to_string(
                            format!("{location:#}"),
                            format!(
                                "https://www.google.com/maps?q={},{}",
                                location.latitude(),
                                location.longitude()
                            ),
                        )
                    ),
                )?;
            }

            blank_line(&mut writer)?;
            plain_text(
                &mut writer,
                format!(
                    "Data sourced from the qrz.com XML API. Location source was {}.",
                    match response.location_source() {
                        GeoSource::User => "provided by the user",
                        GeoSource::GeoCode => "geo-coded from the address",
                        GeoSource::Grid => "calculated from the grid locator",
                        GeoSource::Zip => "calculated from the postal code",
                        GeoSource::State => "calculated from the state or region",
                        GeoSource::DxCc => "calculated from the DXCC entity ID",
                        GeoSource::None => "not computed, or not available",
                    }
                ),
            )?;
            blank_line(&mut writer)?;

            Ok(ExitCode::SUCCESS)
        } else {
            plain_text(
                &mut writer,
                format!(
                    "Callsign {:?} is **{}** valid.",
                    self.callsign,
                    "not".bold().red()
                ),
            )?;
            Ok(ExitCode::FAILURE)
        }
    }
}

impl LookupCallSign {
    pub fn new(callsign: String) -> Self {
        Self { callsign }
    }
}
