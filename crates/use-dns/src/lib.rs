#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Supported DNS record types for small helper routines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsRecordType {
    /// IPv4 address record.
    A,
    /// IPv6 address record.
    AAAA,
    /// Canonical name record.
    CNAME,
    /// Mail exchanger record.
    MX,
    /// Text record.
    TXT,
    /// Name server record.
    NS,
    /// Start of authority record.
    SOA,
    /// Service locator record.
    SRV,
    /// Pointer record.
    PTR,
    /// Certification authority authorization record.
    CAA,
    /// Unknown or unsupported record type.
    Unknown,
}

/// Stores a simple DNS record value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord {
    /// Record name.
    pub name: String,
    /// Record type.
    pub record_type: DnsRecordType,
    /// Record payload string.
    pub value: String,
    /// Optional TTL value.
    pub ttl: Option<u32>,
}

fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

fn normalize_candidate(input: &str) -> Option<String> {
    let trimmed = input.trim().trim_end_matches('.');

    if trimmed.is_empty()
        || trimmed.len() > 253
        || trimmed.contains(':')
        || trimmed.contains('/')
        || trimmed.chars().any(char::is_whitespace)
    {
        return None;
    }

    let normalized = trimmed.to_ascii_lowercase();

    if normalized.split('.').all(is_valid_label) {
        Some(normalized)
    } else {
        None
    }
}

/// Parses a DNS record type from text.
pub fn parse_record_type(input: &str) -> DnsRecordType {
    match input.trim().to_ascii_uppercase().as_str() {
        "A" => DnsRecordType::A,
        "AAAA" => DnsRecordType::AAAA,
        "CNAME" => DnsRecordType::CNAME,
        "MX" => DnsRecordType::MX,
        "TXT" => DnsRecordType::TXT,
        "NS" => DnsRecordType::NS,
        "SOA" => DnsRecordType::SOA,
        "SRV" => DnsRecordType::SRV,
        "PTR" => DnsRecordType::PTR,
        "CAA" => DnsRecordType::CAA,
        _ => DnsRecordType::Unknown,
    }
}

/// Formats a DNS record type as uppercase text.
pub fn format_record_type(record_type: DnsRecordType) -> &'static str {
    match record_type {
        DnsRecordType::A => "A",
        DnsRecordType::AAAA => "AAAA",
        DnsRecordType::CNAME => "CNAME",
        DnsRecordType::MX => "MX",
        DnsRecordType::TXT => "TXT",
        DnsRecordType::NS => "NS",
        DnsRecordType::SOA => "SOA",
        DnsRecordType::SRV => "SRV",
        DnsRecordType::PTR => "PTR",
        DnsRecordType::CAA => "CAA",
        DnsRecordType::Unknown => "UNKNOWN",
    }
}

/// Returns `true` when the record type stores an IP address.
pub fn is_address_record(record_type: DnsRecordType) -> bool {
    matches!(record_type, DnsRecordType::A | DnsRecordType::AAAA)
}

/// Returns `true` when the record type stores an alias target.
pub fn is_alias_record(record_type: DnsRecordType) -> bool {
    matches!(record_type, DnsRecordType::CNAME)
}

/// Returns `true` when the record type is mail-related.
pub fn is_mail_record(record_type: DnsRecordType) -> bool {
    matches!(record_type, DnsRecordType::MX)
}

/// Returns `true` when the record type stores free-form text.
pub fn is_text_record(record_type: DnsRecordType) -> bool {
    matches!(record_type, DnsRecordType::TXT)
}

/// Returns `true` when the input looks like a DNS-style name.
pub fn looks_like_dns_name(input: &str) -> bool {
    normalize_candidate(input).is_some()
}

/// Normalizes a DNS-style name.
pub fn normalize_dns_name(input: &str) -> Option<String> {
    normalize_candidate(input)
}
