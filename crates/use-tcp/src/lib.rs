#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::net::IpAddr;

const TCP_SERVICES: &[(&str, u16)] = &[
    ("ftp", 21),
    ("ssh", 22),
    ("smtp", 25),
    ("dns", 53),
    ("http", 80),
    ("pop3", 110),
    ("imap", 143),
    ("ldap", 389),
    ("https", 443),
    ("mysql", 3306),
    ("postgres", 5432),
    ("redis", 6379),
];

/// Stores a normalized TCP endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpEndpoint {
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

/// Parses a TCP endpoint from a host-and-port string.
pub fn parse_tcp_endpoint(input: &str) -> Option<TcpEndpoint> {
    let (host, port) = split_raw_endpoint(input)?;

    Some(TcpEndpoint { host, port })
}

/// Formats a TCP endpoint with IPv6 bracket handling.
pub fn format_tcp_endpoint(endpoint: &TcpEndpoint) -> String {
    match endpoint.host.parse::<IpAddr>() {
        Ok(IpAddr::V6(address)) => format!("[{address}]:{}", endpoint.port),
        _ => format!("{}:{}", endpoint.host, endpoint.port),
    }
}

/// Looks up a default TCP port for a common service name.
pub fn default_tcp_port(service: &str) -> Option<u16> {
    let normalized = service.trim().to_ascii_lowercase();

    TCP_SERVICES
        .iter()
        .find(|(candidate_service, _)| *candidate_service == normalized)
        .map(|(_, port)| *port)
}

/// Looks up a common TCP service name for a port.
pub fn tcp_service_name(port: u16) -> Option<&'static str> {
    TCP_SERVICES
        .iter()
        .find(|(_, candidate_port)| *candidate_port == port)
        .map(|(service, _)| *service)
}

/// Returns `true` when the port matches one of the known TCP services.
pub fn is_common_tcp_port(port: u16) -> bool {
    tcp_service_name(port).is_some()
}
