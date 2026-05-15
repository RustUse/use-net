use use_port::{
    PortRange, common_port_name, default_port_for_service, is_dynamic_port, is_registered_port,
    is_system_port, parse_port, port_range,
};

#[test]
fn parses_valid_ports() {
    let port = parse_port("8080").unwrap();

    assert_eq!(port.value, 8080);
}

#[test]
fn rejects_invalid_ports() {
    assert!(parse_port("70000").is_none());
    assert!(parse_port("abc").is_none());
}

#[test]
fn classifies_system_ports() {
    assert_eq!(port_range(80), PortRange::System);
    assert!(is_system_port(80));
}

#[test]
fn classifies_registered_ports() {
    assert_eq!(port_range(8080), PortRange::Registered);
    assert!(is_registered_port(8080));
}

#[test]
fn classifies_dynamic_ports() {
    assert_eq!(port_range(55_000), PortRange::Dynamic);
    assert!(is_dynamic_port(55_000));
}

#[test]
fn looks_up_common_services() {
    assert_eq!(common_port_name(443), Some("https"));
    assert_eq!(default_port_for_service("redis"), Some(6379));
}

#[test]
fn handles_empty_input() {
    assert!(parse_port("").is_none());
}
