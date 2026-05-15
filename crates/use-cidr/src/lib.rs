#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::net::IpAddr;

/// Stores a parsed CIDR block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CidrBlock {
    /// Normalized IP address component.
    pub address: String,
    /// Prefix length component.
    pub prefix: u8,
}

/// Parses a CIDR block from text.
pub fn parse_cidr(input: &str) -> Option<CidrBlock> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    let (address, prefix) = trimmed.split_once('/')?;
    let address = address.parse::<IpAddr>().ok()?;
    let prefix = prefix.parse::<u8>().ok()?;

    let valid_prefix = match address {
        IpAddr::V4(_) => is_valid_ipv4_prefix(prefix),
        IpAddr::V6(_) => is_valid_ipv6_prefix(prefix),
    };

    if !valid_prefix {
        return None;
    }

    Some(CidrBlock {
        address: address.to_string(),
        prefix,
    })
}

/// Formats a CIDR block.
pub fn format_cidr(block: &CidrBlock) -> String {
    format!("{}/{}", block.address, block.prefix)
}

/// Returns `true` when the input is a valid IPv4 or IPv6 CIDR block.
pub fn is_cidr(input: &str) -> bool {
    parse_cidr(input).is_some()
}

/// Returns `true` when the input is a valid IPv4 CIDR block.
pub fn is_ipv4_cidr(input: &str) -> bool {
    parse_cidr(input).is_some_and(|block| {
        block
            .address
            .parse::<IpAddr>()
            .is_ok_and(|address| matches!(address, IpAddr::V4(_)))
    })
}

/// Returns `true` when the input is a valid IPv6 CIDR block.
pub fn is_ipv6_cidr(input: &str) -> bool {
    parse_cidr(input).is_some_and(|block| {
        block
            .address
            .parse::<IpAddr>()
            .is_ok_and(|address| matches!(address, IpAddr::V6(_)))
    })
}

/// Returns `true` when the prefix is valid for IPv4.
pub fn is_valid_ipv4_prefix(prefix: u8) -> bool {
    prefix <= 32
}

/// Returns `true` when the prefix is valid for IPv6.
pub fn is_valid_ipv6_prefix(prefix: u8) -> bool {
    prefix <= 128
}
