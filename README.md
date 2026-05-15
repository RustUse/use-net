# RustUse use-net

`use-net` is a RustUse workspace for small, focused Rust 2024 crates around low-level networking primitives such as IP addresses, ports, hosts, domains, DNS record types, socket endpoints, CIDR blocks, and MAC addresses.

> Experimental: every crate in this workspace is below 0.3.0. Expect API changes while the surfaces settle.

## Crate List

- `use-net`: feature-gated umbrella crate that re-exports the focused networking crates.
- `use-ip`: IP address detection, normalization, and classification helpers.
- `use-port`: port parsing, service lookup, and port-range classification helpers.
- `use-host`: host parsing helpers for localhost, IP literals, and domain-style host names.
- `use-domain`: domain and hostname validation, normalization, and label helpers.
- `use-dns`: DNS record-type helpers and DNS name normalization helpers.
- `use-socket`: host-and-port parsing and formatting helpers.
- `use-tcp`: TCP endpoint helpers and common TCP service mappings.
- `use-udp`: UDP endpoint helpers and common UDP service mappings.
- `use-cidr`: CIDR parsing and prefix validation helpers.
- `use-mac`: MAC address parsing, normalization, and formatting helpers.

## Scope

- Lightweight parsing, normalization, validation, and classification helpers.
- Predictable formatting helpers for diagnostics, config, fixtures, CLIs, and web tooling.
- Simple value types and enums that compose well with `std::net` values.

## Non-goals

- Network I/O, socket creation, or runtime networking behavior.
- DNS resolution, resolver implementations, or zone file parsing.
- TCP or UDP protocol implementations.
- Async runtime integration.

## Example Usage

Use a focused crate when you only need one small primitive surface:

```rust
use use_ip::is_ipv6;
use use_socket::parse_socket_endpoint;

assert!(is_ipv6("::1"));

let endpoint = parse_socket_endpoint("[::1]:8080").unwrap();

assert_eq!(endpoint.host, "::1");
assert_eq!(endpoint.port, 8080);
```

Use the umbrella crate with the features you need enabled:

```rust
use use_net::{ip, socket};

assert!(ip::is_ipv4("127.0.0.1"));

let endpoint = socket::parse_socket_endpoint("localhost:3000").unwrap();

assert_eq!(endpoint.port, 3000);
```

## License

Licensed under either `MIT` or `Apache-2.0`.
