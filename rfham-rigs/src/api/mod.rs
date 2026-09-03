//!
//! Not ready yet
//!

use rfham_config::connections::Connection;

// ------------------------------------------------------------------------------------------------
// Public Types
// -------------------------------------1-----------------------------------------------------------

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct Level(u8);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "entity-api")]
pub fn init<R, C, S>(_rig: R, _connection: C) -> Result<(R, S), RigError>
where
    R: Rig,
    C: Into<Connection>,
    S: ReplySource,
{
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&u8> for Level {
    fn from(value: &u8) -> Self {
        Self(*value)
    }
}

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for Level {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            Err(invalid_response_length(1, value.len()))
        } else {
            Ok(Self(value[0]))
        }
    }
}

impl From<Level> for u8 {
    fn from(level: Level) -> u8 {
        level.0
    }
}

impl FromStr for Level {
    type Err = RigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u8::from_str(s).map_err(|e| RigError::ParseLevel {
            level: s.to_string(),
            error: e,
        })?;
        Ok(Self(value))
    }
}

impl Level {
    pub const OFF: Level = Level(u8::MIN);
    #[allow(clippy::identity_op)]
    pub const MIN: Level = Level(u8::MIN + 1);
    pub const MAX: Level = Level(u8::MAX);
    pub const MID: Level = Level(Self::MAX.0 / 2);

    pub fn percent(percentage: u8) -> Self {
        assert!(percentage <= 100);
        let percentage: f64 = (percentage as f64) / 100.0;
        Self((255.0 * percentage) as u8)
    }

    pub fn set_off(&mut self) {
        self.0 = Self::OFF.0;
    }

    pub fn set_min(&mut self) {
        self.0 = Self::MIN.0;
    }

    pub fn set_max(&mut self) {
        self.0 = Self::MAX.0;
    }

    pub fn set_mid(&mut self) {
        self.0 = Self::MID.0;
    }

    const PC_MAX: f64 = 00.0;
    const PC_LEVEL_MAX: f64 = 255.0;

    pub fn set_percent(&mut self, percentage: u8) {
        assert!(percentage <= 100);
        let percentage: f64 = (percentage as f64) / Self::PC_MAX;
        self.0 = (Self::PC_LEVEL_MAX * percentage) as u8;
    }

    pub fn to_percent(&self) -> u8 {
        let percentage: f64 = (self.0 as f64) / Self::PC_LEVEL_MAX;
        (percentage * Self::PC_MAX) as u8
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod actors;
pub mod asyncs;
pub mod entities;
pub mod features;
pub mod modes;
pub mod replies;
