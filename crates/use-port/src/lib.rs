#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

const SERVICE_PORTS: &[(&str, u16)] = &[
    ("ftp", 21),
    ("ssh", 22),
    ("smtp", 25),
    ("dns", 53),
    ("http", 80),
    ("pop3", 110),
    ("ntp", 123),
    ("imap", 143),
    ("ldap", 389),
    ("https", 443),
    ("mysql", 3306),
    ("postgres", 5432),
    ("redis", 6379),
];

/// Stores a validated port number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port {
    /// Port value.
    pub value: u16,
}

/// Classifies a port into the standard IANA port ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortRange {
    /// Ports 0 through 1023.
    System,
    /// Ports 1024 through 49151.
    Registered,
    /// Ports 49152 through 65535.
    Dynamic,
}

/// Returns `true` when the input fits into a valid `u16` port number.
pub fn is_valid_port(input: u32) -> bool {
    input <= u16::MAX as u32
}

/// Parses a port number from text.
pub fn parse_port(input: &str) -> Option<Port> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    trimmed.parse::<u16>().ok().map(|value| Port { value })
}

/// Returns the IANA port range for the provided port.
pub fn port_range(port: u16) -> PortRange {
    match port {
        0..=1023 => PortRange::System,
        1024..=49151 => PortRange::Registered,
        _ => PortRange::Dynamic,
    }
}

/// Returns `true` when the port is in the system range.
pub fn is_system_port(port: u16) -> bool {
    matches!(port_range(port), PortRange::System)
}

/// Returns `true` when the port is in the registered range.
pub fn is_registered_port(port: u16) -> bool {
    matches!(port_range(port), PortRange::Registered)
}

/// Returns `true` when the port is in the dynamic range.
pub fn is_dynamic_port(port: u16) -> bool {
    matches!(port_range(port), PortRange::Dynamic)
}

/// Looks up a common service name for a port.
pub fn common_port_name(port: u16) -> Option<&'static str> {
    SERVICE_PORTS
        .iter()
        .find(|(_, candidate_port)| *candidate_port == port)
        .map(|(service, _)| *service)
}

/// Looks up a default port for a common service name.
pub fn default_port_for_service(service: &str) -> Option<u16> {
    let normalized = service.trim().to_ascii_lowercase();

    SERVICE_PORTS
        .iter()
        .find(|(candidate_service, _)| *candidate_service == normalized)
        .map(|(_, port)| *port)
}
