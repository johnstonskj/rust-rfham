use crate::{OnceCommand, error::CliError};
use colored::Colorize;
use rfham_core::Name;
use rfham_core::callsign::CallSign;
use rfham_itu::callsigns::ItuSeriesAllocation;
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
    service: Option<Name>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for ValidateCallSign {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        if let Ok(callsign) = CallSign::from_str(&self.callsign) {
            println!(
                "Callsign {:?} {}:",
                self.callsign,
                "is valid".bold().green()
            );
            if let Some(a_prefix) = callsign.ancillary_prefix() {
                println!("└── ancillary prefix: {}", a_prefix);
            }
            println!("└── prefix: {}", callsign.prefix());
            if let Some(allocation) = ItuSeriesAllocation::from_callsign(&callsign) {
                println!("    └── ITU allocation; {allocation:#}");
            } else {
                println!("    └── ITU allocation; Unknown/unallocated");
                println!(
                    "    └── is non-standard: {}",
                    callsign.is_prefix_non_standard()
                );
            }
            println!("└── separator numeral: {}", callsign.separator_numeral());
            println!("└── suffix: {}", callsign.suffix());
            println!("    └── is special: {}", callsign.is_special());
            if let Some(a_suffix) = callsign.ancillary_suffix() {
                println!("└── ancillary suffix: {}", a_suffix);
                println!("    └── is portable: {}", callsign.is_portable());
                println!("    └── is mobile: {}", callsign.is_mobile());
                println!(
                    "    └── is aeronautical mobile: {}",
                    callsign.is_aeronautical_mobile()
                );
                println!(
                    "    └── is maritime mobile: {}",
                    callsign.is_maritime_mobile()
                );
                println!(
                    "    └── is at alternate location: {}",
                    callsign.is_at_alternate_location()
                );
                println!("    └── is operating QRP: {}", callsign.is_operating_qrp());
                println!(
                    "    └── FCC license pending: {}",
                    callsign.is_fcc_license_pending()
                );
            }
        } else {
            println!(
                "Callsign {:?} {}",
                self.callsign,
                "is not valid".bold().red()
            );
        }
        Ok(ExitCode::SUCCESS)
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
        Ok(ExitCode::SUCCESS)
    }
}

impl LookupCallSign {
    pub fn new(callsign: String, service: Option<Name>) -> Self {
        Self { callsign, service }
    }
}
