use crate::{AfGainControl, AntennaSwitching, VfoAandB, error::RigError, protocol::Frequency};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Base Traits
// ------------------------------------------------------------------------------------------------

pub trait Entity {}

pub trait Configurable: Entity {
    type Config;

    fn config(&self) -> &Self::Config;
    fn config_mut(&mut self) -> &mut Self::Config;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Rig Traits
// ------------------------------------------------------------------------------------------------

pub trait Rig: Configurable<Config = Self::RigConfig> {
    type RigConfig: RigConfig;

    fn has_af_gain_control(&self) -> bool;
    fn af_gain_control(&self) -> impl AfGainControl;

    fn has_vfo_a_and_b(&self) -> bool;
    fn vfo_a_and_b(&self) -> impl VfoAandB;
    fn current_vfo(&self) -> Result<impl Vfo, RigError>;

    fn has_antenna_switching(&self) -> bool;
    fn antenna_switching(&self) -> impl AntennaSwitching;
}

pub trait RigConfig {
    fn brand_name(&self) -> &str;
    fn model_name(&self) -> &str;
    fn model_version(&self) -> Option<&str>;
    fn serial_number(&self) -> Option<&str>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ VFO Traits
// ------------------------------------------------------------------------------------------------

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    AsRefStr,
    Display,
    EnumIs,
    EnumString,
    EnumIter,
)]
pub enum VfoLabel {
    VfoA,
    VfoB,
    VfoSub,
}

pub trait Vfo: Configurable<Config = Self::VfoConfig> {
    type VfoConfig: VfoConfig;

    fn label(&self) -> VfoLabel;

    fn frequency(&self) -> Result<(), RigError>;
    fn set_frequency(&mut self, frequency: Frequency) -> Result<(), RigError>;

    fn frequency_up(&mut self) -> Result<(), RigError>;
    fn frequency_down(&mut self) -> Result<(), RigError>;
}

pub trait VfoConfig {
    fn frequency_increment(&self) -> Result<(), RigError>;
    fn set_frequency_increment(&self, increment: Frequency) -> Result<(), RigError>;
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
