use crate::{Frequency, Level};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::Duration};
use strum::{AsRefStr, EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, EnumIs, EnumTryAs, AsRefStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Function {
    VfoGetFrequency(Option<usize>),
    VfoSetFrequency(Option<usize>, Frequency),
    VfoSelect(usize),
    GetAfGain,
    SetAfGain(Level),
    Wait(Duration),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, EnumIs, EnumTryAs)]
pub enum Value {
    Char(char),
    Index(usize),
    Frequency(Frequency),
    Level(Level),
    Duration(Duration),
    Symbol(String, String),
}

pub trait ToFunction {
    fn to_function(&self) -> Function;
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

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}{}{})",
            self.as_ref(),
            if self.has_args() { " " } else { "" },
            self.args()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Function {
    pub fn has_args(&self) -> bool {
        match self {
            Self::VfoGetFrequency(vfo) => vfo.is_some(),
            Self::VfoSetFrequency(_, _) => true,
            Self::VfoSelect(_) => true,
            Self::SetAfGain(_) => true,
            Self::Wait(_) => true,
            _ => false,
        }
    }
    pub fn args(&self) -> Vec<Value> {
        match self {
            Self::VfoGetFrequency(vfo) => {
                if let Some(vfo) = vfo {
                    vec![Value::Index(*vfo)]
                } else {
                    vec![]
                }
            }
            Self::VfoSetFrequency(vfo, frequency) => {
                if let Some(vfo) = vfo {
                    vec![Value::Index(*vfo), Value::Frequency(*frequency)]
                } else {
                    vec![Value::Frequency(*frequency)]
                }
            }
            Self::VfoSelect(vfo) => vec![Value::Index(*vfo)],
            Self::GetAfGain => vec![],
            Self::SetAfGain(gain) => vec![Value::Level(*gain)],
            Self::Wait(duration) => vec![Value::Duration(*duration)],
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(v) => format!("#\\{v}").fmt(f),
            Self::Index(v) => v.fmt(f),
            Self::Frequency(v) => format!("(hertz {})", v.0).fmt(f),
            Self::Level(v) => format!("(level {v})").fmt(f),
            Self::Duration(v) => write!(f, "(seconds {})", v.as_secs_f64()),
            Self::Symbol(namespace, name) => if f.alternate() {
                format!("'{}:{}", namespace, name)
            } else {
                format!("'{}", name)
            }
            .fmt(f),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------
