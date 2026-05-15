#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::net::IpAddr;

const UDP_SERVICES: &[(&str, u16)] = &[
    ("dns", 53),
    ("ntp", 123),
    ("snmp", 161),
    ("ldap", 389),
    ("syslog", 514),
    ("radius", 1812),
];

/// Stores a normalized UDP endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpEndpoint {
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

/// Parses a UDP endpoint from a host-and-port string.
pub fn parse_udp_endpoint(input: &str) -> Option<UdpEndpoint> {
    let (host, port) = split_raw_endpoint(input)?;

    Some(UdpEndpoint { host, port })
}

/// Formats a UDP endpoint with IPv6 bracket handling.
pub fn format_udp_endpoint(endpoint: &UdpEndpoint) -> String {
    match endpoint.host.parse::<IpAddr>() {
        Ok(IpAddr::V6(address)) => format!("[{address}]:{}", endpoint.port),
        _ => format!("{}:{}", endpoint.host, endpoint.port),
    }
}

/// Looks up a default UDP port for a common service name.
pub fn default_udp_port(service: &str) -> Option<u16> {
    let normalized = service.trim().to_ascii_lowercase();

    UDP_SERVICES
        .iter()
        .find(|(candidate_service, _)| *candidate_service == normalized)
        .map(|(_, port)| *port)
}

/// Looks up a common UDP service name for a port.
pub fn udp_service_name(port: u16) -> Option<&'static str> {
    UDP_SERVICES
        .iter()
        .find(|(_, candidate_port)| *candidate_port == port)
        .map(|(service, _)| *service)
}

/// Returns `true` when the port matches one of the known UDP services.
pub fn is_common_udp_port(port: u16) -> bool {
    udp_service_name(port).is_some()
}
