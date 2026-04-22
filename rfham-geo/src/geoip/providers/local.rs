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

use rfham_core::error::CoreError;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, net::Ipv4Addr, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct IpNetwork {
    address: u32,
    mask: u32,
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

impl Display for IpNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.address(), self.prefix_length())
    }
}

impl FromStr for IpNetwork {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<&str> = s.split('/').collect::<Vec<_>>();
        if pair.len() == 2 {
            assert!(pair.len() == 2);
            let address = Ipv4Addr::from_str(pair[0])
                .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"))?;
            let prefix_length = u8::from_str(pair[1])
                .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"))?;
            Ok(IpNetwork::from_cidr(address, prefix_length))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"))
        }
    }
}

impl IpNetwork {
    pub fn from_cidr(address: Ipv4Addr, prefix_length: u8) -> Self {
        assert!(prefix_length <= 32);
        Self {
            address: address.to_bits(),
            mask: u32::MAX >> (32 - prefix_length),
        }
    }

    pub fn from_mask(address: Ipv4Addr, net_mask: Ipv4Addr) -> Self {
        Self {
            address: address.to_bits(),
            mask: net_mask.to_bits(),
        }
    }

    pub fn address(&self) -> Ipv4Addr {
        self.address.into()
    }

    pub fn address_u32(&self) -> u32 {
        self.address
    }

    pub fn mask(&self) -> Ipv4Addr {
        self.mask.into()
    }

    pub fn mask_u32(&self) -> u32 {
        self.mask
    }

    pub fn prefix_length(&self) -> u8 {
        self.mask_u32().leading_ones() as u8
    }

    pub fn contains(&self, address: Ipv4Addr) -> bool {
        (self.address & self.mask) == (address.to_bits() & self.mask)
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
