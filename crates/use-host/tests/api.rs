use use_host::{
    HostKind, bracket_ipv6_host, is_domain_host, is_ip_host, is_localhost, parse_host,
    strip_brackets,
};

#[test]
fn detects_localhost() {
    assert!(is_localhost("localhost"));
    assert_eq!(parse_host("localhost").unwrap().kind, HostKind::Localhost);
}

#[test]
fn detects_ip_hosts() {
    assert!(is_ip_host("127.0.0.1"));
    assert_eq!(parse_host("[::1]").unwrap().kind, HostKind::Ip);
}

#[test]
fn detects_domain_hosts() {
    assert!(is_domain_host("example.com"));
    assert_eq!(parse_host("EXAMPLE.com").unwrap().value, "example.com");
}

#[test]
fn strips_ipv6_brackets() {
    assert_eq!(strip_brackets("[::1]"), "::1");
}

#[test]
fn formats_ipv6_brackets() {
    assert_eq!(bracket_ipv6_host("::1"), "[::1]");
}

#[test]
fn rejects_malformed_input() {
    assert!(parse_host("bad host!").is_none());
}

#[test]
fn handles_empty_input() {
    assert!(parse_host("").is_none());
}
