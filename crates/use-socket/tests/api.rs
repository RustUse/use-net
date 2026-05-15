use use_socket::{
    is_host_port, is_ipv4_socket_endpoint, is_ipv6_socket_endpoint, parse_socket_endpoint,
    split_host_port,
};

#[test]
fn parses_ipv4_endpoints() {
    let endpoint = parse_socket_endpoint("127.0.0.1:8080").unwrap();

    assert_eq!(endpoint.host, "127.0.0.1");
    assert_eq!(endpoint.port, 8080);
    assert!(is_ipv4_socket_endpoint("127.0.0.1:8080"));
}

#[test]
fn parses_ipv6_endpoints() {
    let endpoint = parse_socket_endpoint("[::1]:8080").unwrap();

    assert_eq!(endpoint.host, "::1");
    assert!(is_ipv6_socket_endpoint("[::1]:8080"));
}

#[test]
fn parses_localhost_endpoints() {
    let endpoint = parse_socket_endpoint("localhost:3000").unwrap();

    assert_eq!(endpoint.host, "localhost");
    assert_eq!(endpoint.port, 3000);
}

#[test]
fn parses_domain_endpoints() {
    let endpoint = parse_socket_endpoint("Example.com:443").unwrap();

    assert_eq!(endpoint.host, "example.com");
    assert_eq!(endpoint.port, 443);
}

#[test]
fn splits_host_and_port() {
    assert_eq!(
        split_host_port("example.com:443"),
        Some((String::from("example.com"), 443))
    );
    assert!(is_host_port("example.com:443"));
}

#[test]
fn rejects_malformed_input() {
    assert!(parse_socket_endpoint("::1:8080").is_none());
    assert!(parse_socket_endpoint("example.com").is_none());
}

#[test]
fn handles_empty_input() {
    assert!(parse_socket_endpoint("").is_none());
}
