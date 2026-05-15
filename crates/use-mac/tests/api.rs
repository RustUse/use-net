use use_mac::{format_mac_hyphen, is_broadcast_mac, is_zero_mac, normalize_mac, parse_mac_address};

#[test]
fn parses_colon_mac_addresses() {
    let mac = parse_mac_address("00:1A:2B:3C:4D:5E").unwrap();

    assert_eq!(mac.octets, [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
}

#[test]
fn parses_hyphen_mac_addresses() {
    let mac = parse_mac_address("00-1A-2B-3C-4D-5E").unwrap();

    assert_eq!(format_mac_hyphen(&mac), "00-1A-2B-3C-4D-5E");
}

#[test]
fn parses_dot_mac_addresses() {
    assert!(parse_mac_address("001A.2B3C.4D5E").is_some());
}

#[test]
fn parses_compact_mac_addresses() {
    assert!(parse_mac_address("001A2B3C4D5E").is_some());
}

#[test]
fn normalizes_mac_addresses() {
    assert_eq!(
        normalize_mac("00-1A-2B-3C-4D-5E"),
        Some("00:1A:2B:3C:4D:5E".to_string())
    );
}

#[test]
fn detects_broadcast_mac_addresses() {
    assert!(is_broadcast_mac("FF:FF:FF:FF:FF:FF"));
}

#[test]
fn detects_zero_mac_addresses() {
    assert!(is_zero_mac("00:00:00:00:00:00"));
}

#[test]
fn rejects_malformed_input() {
    assert!(parse_mac_address("00:00:00:00:00").is_none());
}

#[test]
fn handles_empty_input() {
    assert!(parse_mac_address("").is_none());
}
