use use_cidr::{
    is_ipv4_cidr, is_ipv6_cidr, is_valid_ipv4_prefix, is_valid_ipv6_prefix, parse_cidr,
};

#[test]
fn parses_ipv4_cidr_blocks() {
    let block = parse_cidr("192.168.0.0/24").unwrap();

    assert_eq!(block.address, "192.168.0.0");
    assert_eq!(block.prefix, 24);
    assert!(is_ipv4_cidr("192.168.0.0/24"));
}

#[test]
fn parses_ipv6_cidr_blocks() {
    let block = parse_cidr("2001:db8::/32").unwrap();

    assert_eq!(block.address, "2001:db8::");
    assert_eq!(block.prefix, 32);
    assert!(is_ipv6_cidr("2001:db8::/32"));
}

#[test]
fn rejects_invalid_cidr_blocks() {
    assert!(parse_cidr("192.168.0.0/40").is_none());
    assert!(parse_cidr("not-a-cidr").is_none());
}

#[test]
fn validates_ipv4_prefixes() {
    assert!(is_valid_ipv4_prefix(24));
    assert!(!is_valid_ipv4_prefix(40));
}

#[test]
fn validates_ipv6_prefixes() {
    assert!(is_valid_ipv6_prefix(64));
    assert!(!is_valid_ipv6_prefix(129));
}

#[test]
fn handles_empty_input() {
    assert!(parse_cidr("").is_none());
}
