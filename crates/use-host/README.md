# use-host

`use-host` provides lightweight helpers for parsing hosts such as localhost, IP literals, and domain-style host names.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_host::{bracket_ipv6_host, parse_host, HostKind};

let host = parse_host("[::1]").unwrap();

assert_eq!(host.kind, HostKind::Ip);
assert_eq!(host.value, "::1");
assert_eq!(bracket_ipv6_host(&host.value), "[::1]");
```

## Scope

- Localhost, IP-literal, and hostname-style host detection.
- Simple host normalization and IPv6 bracket helpers.

## Non-goals

- DNS lookup.
- Public suffix validation.
- Host reachability testing.

## License

Licensed under `MIT OR Apache-2.0`.
