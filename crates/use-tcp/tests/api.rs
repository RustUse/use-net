use use_tcp::{
    default_tcp_port, format_tcp_endpoint, is_common_tcp_port, parse_tcp_endpoint, tcp_service_name,
};

#[test]
fn parses_tcp_endpoints() {
    let endpoint = parse_tcp_endpoint("example.com:443").unwrap();

    assert_eq!(endpoint.host, "example.com");
    assert_eq!(endpoint.port, 443);
}

#[test]
fn formats_tcp_endpoints() {
    let endpoint = parse_tcp_endpoint("[::1]:8080").unwrap();

    assert_eq!(format_tcp_endpoint(&endpoint), "[::1]:8080");
}

#[test]
fn looks_up_default_tcp_ports() {
    assert_eq!(default_tcp_port("https"), Some(443));
}

#[test]
fn looks_up_common_tcp_services() {
    assert_eq!(tcp_service_name(22), Some("ssh"));
    assert!(is_common_tcp_port(22));
}

#[test]
fn rejects_malformed_input() {
    assert!(parse_tcp_endpoint("::1:80").is_none());
    assert!(parse_tcp_endpoint("bad host!:80").is_none());
}

#[test]
fn handles_empty_input() {
    assert!(parse_tcp_endpoint("").is_none());
}
