use use_udp::{
    default_udp_port, format_udp_endpoint, is_common_udp_port, parse_udp_endpoint, udp_service_name,
};

#[test]
fn parses_udp_endpoints() {
    let endpoint = parse_udp_endpoint("localhost:53").unwrap();

    assert_eq!(endpoint.host, "localhost");
    assert_eq!(endpoint.port, 53);
}

#[test]
fn formats_udp_endpoints() {
    let endpoint = parse_udp_endpoint("[::1]:123").unwrap();

    assert_eq!(format_udp_endpoint(&endpoint), "[::1]:123");
}

#[test]
fn looks_up_default_udp_ports() {
    assert_eq!(default_udp_port("dns"), Some(53));
}

#[test]
fn looks_up_common_udp_services() {
    assert_eq!(udp_service_name(123), Some("ntp"));
    assert!(is_common_udp_port(123));
}

#[test]
fn rejects_malformed_input() {
    assert!(parse_udp_endpoint("::1:53").is_none());
    assert!(parse_udp_endpoint("bad host!:53").is_none());
}

#[test]
fn handles_empty_input() {
    assert!(parse_udp_endpoint("").is_none());
}
