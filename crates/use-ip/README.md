# use-ip

`use-ip` provides small helpers for parsing, normalizing, and classifying IP address literals.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_ip::{detect_ip_kind, is_private_ip, normalize_ip, IpKind};

assert_eq!(detect_ip_kind("192.168.1.20"), IpKind::V4);
assert!(is_private_ip("fd00::1"));
assert_eq!(normalize_ip("2001:0db8::0001"), Some("2001:db8::1".to_string()));
```

## Scope

- IP address validation and normalization.
- Basic IP classifications such as loopback, private, link-local, and multicast.

## Non-goals

- Geolocation.
- Network interface inspection.
- Packet parsing.

## License

Licensed under `MIT OR Apache-2.0`.