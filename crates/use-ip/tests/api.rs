use use_ip::{IpKind, detect_ip_kind, is_ip, is_ipv4, is_ipv6, is_loopback_ip, is_private_ip};

#[test]
fn detects_ipv4_addresses() {
    assert!(is_ip("127.0.0.1"));
    assert!(is_ipv4("127.0.0.1"));
    assert_eq!(detect_ip_kind("127.0.0.1"), IpKind::V4);
}

#[test]
fn detects_ipv6_addresses() {
    assert!(is_ip("2001:db8::1"));
    assert!(is_ipv6("2001:db8::1"));
    assert_eq!(detect_ip_kind("2001:db8::1"), IpKind::V6);
}

#[test]
fn rejects_invalid_ip_input() {
    assert!(!is_ip("not-an-ip"));
    assert_eq!(detect_ip_kind("not-an-ip"), IpKind::Unknown);
}

#[test]
fn detects_loopback_addresses() {
    assert!(is_loopback_ip("127.0.0.1"));
    assert!(is_loopback_ip("::1"));
}

#[test]
fn detects_private_addresses() {
    assert!(is_private_ip("192.168.0.10"));
    assert!(is_private_ip("fd00::1"));
}

#[test]
fn handles_empty_input() {
    assert!(!is_ip(""));
    assert_eq!(detect_ip_kind(""), IpKind::Unknown);
}
