#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::net::IpAddr;

/// Stores a normalized host-and-port endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketEndpoint {
    /// Endpoint host component.
    pub host: String,
    /// Endpoint port component.
    pub port: u16,
}

fn parse_port(input: &str) -> Option<u16> {
    if input.is_empty() {
        None
    } else {
        input.parse::<u16>().ok()
    }
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

fn normalize_host(input: &str) -> Option<String> {
    if input.eq_ignore_ascii_case("localhost") {
        return Some(String::from("localhost"));
    }

    if let Ok(address) = input.parse::<IpAddr>() {
        return Some(address.to_string());
    }

    let normalized = input.trim_end_matches('.').to_ascii_lowercase();

    if normalized.is_empty()
        || normalized.len() > 253
        || normalized.contains(':')
        || normalized.contains('/')
        || normalized.chars().any(char::is_whitespace)
    {
        return None;
    }

    if normalized.split('.').all(is_valid_label) {
        Some(normalized)
    } else {
        None
    }
}

fn split_raw_endpoint(input: &str) -> Option<(String, u16)> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    if trimmed.starts_with('[') {
        let closing_index = trimmed.find(']')?;
        let host = &trimmed[1..closing_index];
        let port = trimmed.get(closing_index + 2..)?;

        if !trimmed[closing_index + 1..].starts_with(':') {
            return None;
        }

        match host.parse::<IpAddr>() {
            Ok(IpAddr::V6(address)) => Some((address.to_string(), parse_port(port)?)),
            _ => None,
        }
    } else {
        let (host, port) = trimmed.rsplit_once(':')?;

        if host.is_empty() || host.contains(':') {
            return None;
        }

        Some((normalize_host(host)?, parse_port(port)?))
    }
}

/// Parses a socket endpoint from common host-and-port forms.
pub fn parse_socket_endpoint(input: &str) -> Option<SocketEndpoint> {
    let (host, port) = split_raw_endpoint(input)?;

    Some(SocketEndpoint { host, port })
}

/// Formats a socket endpoint, adding IPv6 brackets when needed.
pub fn format_socket_endpoint(endpoint: &SocketEndpoint) -> String {
    match endpoint.host.parse::<IpAddr>() {
        Ok(IpAddr::V6(address)) => format!("[{address}]:{}", endpoint.port),
        _ => format!("{}:{}", endpoint.host, endpoint.port),
    }
}

/// Returns `true` when the endpoint uses an IPv4 host.
pub fn is_ipv4_socket_endpoint(input: &str) -> bool {
    parse_socket_endpoint(input)
        .is_some_and(|endpoint| matches!(endpoint.host.parse::<IpAddr>(), Ok(IpAddr::V4(_))))
}

/// Returns `true` when the endpoint uses an IPv6 host.
pub fn is_ipv6_socket_endpoint(input: &str) -> bool {
    parse_socket_endpoint(input)
        .is_some_and(|endpoint| matches!(endpoint.host.parse::<IpAddr>(), Ok(IpAddr::V6(_))))
}

/// Returns `true` when the input is a valid host-and-port endpoint.
pub fn is_host_port(input: &str) -> bool {
    parse_socket_endpoint(input).is_some()
}

/// Splits a valid endpoint into host and port components.
pub fn split_host_port(input: &str) -> Option<(String, u16)> {
    split_raw_endpoint(input)
}
