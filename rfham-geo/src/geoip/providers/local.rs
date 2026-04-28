//! IPv4 CIDR-range type for local geo-IP lookup tables.
//!
//! [`IpNetwork`] represents an IPv4 network in CIDR notation (`address/prefix`).
//! It can be constructed from a CIDR string, from an `(address, prefix_length)` pair,
//! or from an `(address, netmask)` pair.
//!
//! # Examples
//!
//! ```rust
//! use rfham_geo::geoip::providers::local::IpNetwork;
//! use std::{net::Ipv4Addr, str::FromStr};
//!
//! let net: IpNetwork = "192.168.1.0/24".parse().unwrap();
//! assert_eq!(net.prefix_length(), 24);
//! assert!(net.contains(Ipv4Addr::from_str("192.168.1.42").unwrap()));
//! assert!(!net.contains(Ipv4Addr::from_str("192.168.2.1").unwrap()));
//! assert_eq!(net.to_string(), "192.168.1.0/24");
//! ```

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
            let address = Ipv4Addr::from_str(pair[0])
                .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"))?;
            let prefix_length = u8::from_str(pair[1])
                .map_err(|_| CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"))?;
            if prefix_length > 32 {
                return Err(CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"));
            }
            Ok(IpNetwork::from_cidr(address, prefix_length))
        } else {
            Err(CoreError::InvalidValueFromStr(s.to_string(), "IpNetwork"))
        }
    }
}

impl IpNetwork {
    pub fn from_cidr(address: Ipv4Addr, prefix_length: u8) -> Self {
        assert!(prefix_length <= 32);
        // Network mask: prefix_length leading 1-bits followed by trailing 0-bits.
        // Shifts by 32 overflow for u32, so /0 and /32 are special-cased.
        let mask = match prefix_length {
            0 => 0u32,
            32 => u32::MAX,
            n => !(u32::MAX >> n),
        };
        Self {
            address: address.to_bits(),
            mask,
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
mod tests {
    use super::IpNetwork;
    use pretty_assertions::assert_eq;
    use std::{net::Ipv4Addr, str::FromStr};

    #[test]
    fn test_cidr_parse_and_display() {
        let net: IpNetwork = "192.168.1.0/24".parse().unwrap();
        assert_eq!(net.to_string(), "192.168.1.0/24");
        assert_eq!(net.prefix_length(), 24);
    }

    #[test]
    fn test_cidr_contains() {
        let net: IpNetwork = "10.0.0.0/8".parse().unwrap();
        assert!(net.contains(Ipv4Addr::from_str("10.1.2.3").unwrap()));
        assert!(net.contains(Ipv4Addr::from_str("10.255.255.255").unwrap()));
        assert!(!net.contains(Ipv4Addr::from_str("11.0.0.1").unwrap()));
    }

    #[test]
    fn test_cidr_host_route() {
        let net: IpNetwork = "203.0.113.5/32".parse().unwrap();
        assert_eq!(net.prefix_length(), 32);
        assert!(net.contains(Ipv4Addr::from_str("203.0.113.5").unwrap()));
        assert!(!net.contains(Ipv4Addr::from_str("203.0.113.6").unwrap()));
    }

    #[test]
    fn test_from_mask() {
        let addr = Ipv4Addr::from_str("192.168.0.0").unwrap();
        let mask = Ipv4Addr::from_str("255.255.255.0").unwrap();
        let net = IpNetwork::from_mask(addr, mask);
        assert_eq!(net.prefix_length(), 24);
        assert!(net.contains(Ipv4Addr::from_str("192.168.0.99").unwrap()));
    }

    #[test]
    fn test_invalid_cidr_returns_error() {
        assert!("notanip/24".parse::<IpNetwork>().is_err());
        assert!("192.168.1.0/33".parse::<IpNetwork>().is_err()); // prefix > 32 panics in from_cidr
        assert!("192.168.1.0".parse::<IpNetwork>().is_err()); // no prefix
    }
}
