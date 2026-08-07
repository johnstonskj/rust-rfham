use crate::{
    OnceCommand,
    error::{CliError, config_file_missing},
};
use rfham_config::{
    Configuration,
    connections::{Connection, IpConnection, SerialConnection},
};
use std::{path::PathBuf, process::ExitCode, str::FromStr};
use strum::{EnumIs, EnumTryAs};
use tracing::info;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TestTransport {
    config_file: Option<PathBuf>,
    connection: ConnectionString,
}

#[derive(Debug, EnumIs, EnumTryAs)]
pub enum ConnectionString {
    Profile(String),
    Ip(String),
    Serial(String),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for TestTransport {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let config_file_path = if let Some(config_file) = self.config_file {
            config_file
        } else {
            Configuration::default_file_path()?
        };
        info!(
            "configuration file path is {config_file_path:?}, exists: {}",
            config_file_path.is_file()
        );
        if !config_file_path.is_file() {
            config_file_missing(config_file_path).print();
            Ok(ExitCode::FAILURE)
        } else {
            let config = Configuration::load_from(config_file_path)?;
            info!("Connection string: {:?}", self.connection);
            let connection = self.connection.to_connection(&config)?;
            info!("Parsed connection: {:?}", connection);

            Ok(ExitCode::SUCCESS)
        }
    }
}

impl TestTransport {
    pub fn new(config_file: Option<PathBuf>, connection: ConnectionString) -> Self {
        Self {
            config_file,
            connection,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl ConnectionString {
    pub fn to_connection(&self, config: &Configuration) -> Result<Connection, CliError> {
        Ok(match self {
            Self::Ip(s) => IpConnection::from_str(s)?.into(),
            Self::Profile(name) => config.connection(name).cloned().expect("oops"),
            Self::Serial(s) => SerialConnection::from_str(s)?.into(),
        })
    }
}
