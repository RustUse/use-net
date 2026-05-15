# use-udp

`use-udp` provides small helpers for parsing UDP-style endpoints and looking up a few common UDP service ports.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_udp::{default_udp_port, format_udp_endpoint, parse_udp_endpoint};

let endpoint = parse_udp_endpoint("localhost:53").unwrap();

assert_eq!(endpoint.port, 53);
assert_eq!(default_udp_port("dns"), Some(53));
assert_eq!(format_udp_endpoint(&endpoint), "localhost:53");
```

## Scope

- Parsing and formatting UDP-style host-and-port endpoints.
- Small common UDP service mappings.

## Non-goals

- UDP sockets.
- Datagram sending.
- Datagram receiving.
- Broadcast or multicast I/O.

## License

Licensed under `MIT OR Apache-2.0`.
