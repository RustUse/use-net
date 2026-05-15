#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::net::IpAddr;

/// Classifies a parsed IP literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpKind {
    /// IPv4 address literal.
    V4,
    /// IPv6 address literal.
    V6,
    /// Unknown or invalid input.
    Unknown,
}

/// Stores an original IP-like input with its detected kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpParts {
    /// Original input value.
    pub input: String,
    /// Detected IP kind.
    pub kind: IpKind,
}

fn parse_ip(input: &str) -> Option<IpAddr> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    trimmed.parse().ok()
}

/// Returns `true` when the input is a valid IPv4 or IPv6 literal.
pub fn is_ip(input: &str) -> bool {
    parse_ip(input).is_some()
}

/// Returns `true` when the input is a valid IPv4 literal.
pub fn is_ipv4(input: &str) -> bool {
    matches!(parse_ip(input), Some(IpAddr::V4(_)))
}

/// Returns `true` when the input is a valid IPv6 literal.
pub fn is_ipv6(input: &str) -> bool {
    matches!(parse_ip(input), Some(IpAddr::V6(_)))
}

/// Detects whether the input is IPv4, IPv6, or unknown.
pub fn detect_ip_kind(input: &str) -> IpKind {
    match parse_ip(input) {
        Some(IpAddr::V4(_)) => IpKind::V4,
        Some(IpAddr::V6(_)) => IpKind::V6,
        None => IpKind::Unknown,
    }
}

/// Normalizes a valid IP literal to Rust's canonical string form.
pub fn normalize_ip(input: &str) -> Option<String> {
    parse_ip(input).map(|address| address.to_string())
}

/// Returns `true` when the input is a loopback IP literal.
pub fn is_loopback_ip(input: &str) -> bool {
    parse_ip(input).is_some_and(|address| address.is_loopback())
}

/// Returns `true` when the input is an unspecified IP literal.
pub fn is_unspecified_ip(input: &str) -> bool {
    parse_ip(input).is_some_and(|address| address.is_unspecified())
}

/// Returns `true` when the input is a private IPv4 or unique-local IPv6 literal.
pub fn is_private_ip(input: &str) -> bool {
    match parse_ip(input) {
        Some(IpAddr::V4(address)) => address.is_private(),
        Some(IpAddr::V6(address)) => address.is_unique_local(),
        None => false,
    }
}

/// Returns `true` when the input is a link-local IPv4 or IPv6 literal.
pub fn is_link_local_ip(input: &str) -> bool {
    match parse_ip(input) {
        Some(IpAddr::V4(address)) => address.is_link_local(),
        Some(IpAddr::V6(address)) => address.is_unicast_link_local(),
        None => false,
    }
}

/// Returns `true` when the input is a multicast IP literal.
pub fn is_multicast_ip(input: &str) -> bool {
    parse_ip(input).is_some_and(|address| address.is_multicast())
}
