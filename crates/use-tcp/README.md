# use-tcp

`use-tcp` provides small helpers for parsing TCP-style endpoints and looking up a few common TCP service ports.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_tcp::{default_tcp_port, format_tcp_endpoint, parse_tcp_endpoint};

let endpoint = parse_tcp_endpoint("example.com:443").unwrap();

assert_eq!(endpoint.port, 443);
assert_eq!(default_tcp_port("https"), Some(443));
assert_eq!(format_tcp_endpoint(&endpoint), "example.com:443");
```

## Scope

- Parsing and formatting TCP-style host-and-port endpoints.
- Small common TCP service mappings.

## Non-goals

- TCP connections.
- TCP listeners.
- Stream handling.
- TLS.

## License

Licensed under `MIT OR Apache-2.0`.
