#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::net::IpAddr;

/// Classifies a host-like input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostKind {
    /// IP literal host.
    Ip,
    /// Domain or hostname-like host.
    Domain,
    /// The special `localhost` host.
    Localhost,
    /// Unknown or invalid host input.
    Unknown,
}

/// Stores a normalized host and its detected kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// Normalized host value.
    pub value: String,
    /// Detected host kind.
    pub kind: HostKind,
}

fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
}

fn looks_like_hostname(input: &str) -> bool {
    let trimmed = input.trim_end_matches('.');

    !trimmed.is_empty()
        && trimmed.len() <= 253
        && !trimmed.contains(':')
        && !trimmed.contains('/')
        && !trimmed.chars().any(char::is_whitespace)
        && trimmed.split('.').all(is_valid_label)
}

/// Removes surrounding IPv6-style brackets when present.
pub fn strip_brackets(input: &str) -> &str {
    if input.len() >= 2 && input.starts_with('[') && input.ends_with(']') {
        &input[1..input.len() - 1]
    } else {
        input
    }
}

/// Adds brackets around an IPv6 host and leaves other hosts unchanged.
pub fn bracket_ipv6_host(input: &str) -> String {
    let trimmed = input.trim();
    let stripped = strip_brackets(trimmed).trim();

    match stripped.parse::<IpAddr>() {
        Ok(IpAddr::V6(address)) => format!("[{address}]"),
        _ => trimmed.to_string(),
    }
}

/// Detects the host kind for a host-like input.
pub fn detect_host_kind(input: &str) -> HostKind {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return HostKind::Unknown;
    }

    let candidate = strip_brackets(trimmed).trim();

    if candidate.eq_ignore_ascii_case("localhost") {
        HostKind::Localhost
    } else if candidate.parse::<IpAddr>().is_ok() {
        HostKind::Ip
    } else if looks_like_hostname(&candidate.to_ascii_lowercase()) {
        HostKind::Domain
    } else {
        HostKind::Unknown
    }
}

/// Parses and normalizes a host-like input.
pub fn parse_host(input: &str) -> Option<Host> {
    let value = normalize_host(input)?;
    let kind = detect_host_kind(&value);

    Some(Host { value, kind })
}

/// Returns `true` when the input is `localhost`.
pub fn is_localhost(input: &str) -> bool {
    matches!(detect_host_kind(input), HostKind::Localhost)
}

/// Returns `true` when the input is an IP literal host.
pub fn is_ip_host(input: &str) -> bool {
    matches!(detect_host_kind(input), HostKind::Ip)
}

/// Returns `true` when the input is a domain or hostname-like host.
pub fn is_domain_host(input: &str) -> bool {
    matches!(detect_host_kind(input), HostKind::Domain)
}

/// Normalizes a host-like input.
pub fn normalize_host(input: &str) -> Option<String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    let candidate = strip_brackets(trimmed).trim();

    if candidate.eq_ignore_ascii_case("localhost") {
        return Some(String::from("localhost"));
    }

    if let Ok(address) = candidate.parse::<IpAddr>() {
        return Some(address.to_string());
    }

    let normalized = candidate.trim_end_matches('.').to_ascii_lowercase();

    if looks_like_hostname(&normalized) {
        Some(normalized)
    } else {
        None
    }
}
