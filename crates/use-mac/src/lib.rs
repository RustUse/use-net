#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::str;

/// Stores a parsed MAC address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddress {
    /// Raw MAC address octets.
    pub octets: [u8; 6],
}

fn normalized_hex(input: &str) -> Option<String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    if trimmed.contains('.') {
        let parts: Vec<_> = trimmed.split('.').collect();

        if parts.len() != 3
            || parts.iter().any(|part| {
                part.len() != 4 || !part.chars().all(|character| character.is_ascii_hexdigit())
            })
        {
            return None;
        }

        return Some(parts.concat().to_ascii_uppercase());
    }

    if trimmed.contains(':') || trimmed.contains('-') {
        let separator = if trimmed.contains(':') { ':' } else { '-' };
        let parts: Vec<_> = trimmed.split(separator).collect();

        if parts.len() != 6
            || parts.iter().any(|part| {
                part.len() != 2 || !part.chars().all(|character| character.is_ascii_hexdigit())
            })
        {
            return None;
        }

        return Some(parts.concat().to_ascii_uppercase());
    }

    if trimmed.len() == 12
        && trimmed
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        Some(trimmed.to_ascii_uppercase())
    } else {
        None
    }
}

/// Returns `true` when the input is a valid MAC address in a supported format.
pub fn is_mac_address(input: &str) -> bool {
    parse_mac_address(input).is_some()
}

/// Parses a MAC address from a common textual format.
pub fn parse_mac_address(input: &str) -> Option<MacAddress> {
    let hex = normalized_hex(input)?;
    let mut octets = [0_u8; 6];

    for (index, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let chunk = str::from_utf8(chunk).ok()?;
        octets[index] = u8::from_str_radix(chunk, 16).ok()?;
    }

    Some(MacAddress { octets })
}

/// Formats a MAC address using colon separators.
pub fn format_mac_colon(mac: &MacAddress) -> String {
    mac.octets
        .iter()
        .map(|octet| format!("{octet:02X}"))
        .collect::<Vec<_>>()
        .join(":")
}

/// Formats a MAC address using hyphen separators.
pub fn format_mac_hyphen(mac: &MacAddress) -> String {
    mac.octets
        .iter()
        .map(|octet| format!("{octet:02X}"))
        .collect::<Vec<_>>()
        .join("-")
}

/// Normalizes a MAC address to uppercase colon-separated form.
pub fn normalize_mac(input: &str) -> Option<String> {
    parse_mac_address(input).map(|mac| format_mac_colon(&mac))
}

/// Returns `true` when the MAC address is the all-ones broadcast address.
pub fn is_broadcast_mac(input: &str) -> bool {
    parse_mac_address(input).is_some_and(|mac| mac.octets.iter().all(|octet| *octet == 0xFF))
}

/// Returns `true` when the MAC address is the all-zero address.
pub fn is_zero_mac(input: &str) -> bool {
    parse_mac_address(input).is_some_and(|mac| mac.octets.iter().all(|octet| *octet == 0x00))
}
