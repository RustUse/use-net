# use-socket

`use-socket` provides small helpers for parsing and formatting host-and-port endpoints.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_socket::{format_socket_endpoint, parse_socket_endpoint};

let endpoint = parse_socket_endpoint("[::1]:8080").unwrap();

assert_eq!(endpoint.host, "::1");
assert_eq!(format_socket_endpoint(&endpoint), "[::1]:8080");
```

## Scope

- Parsing host-and-port endpoints.
- Formatting endpoints with proper IPv6 bracket handling.
- Simple host and port splitting helpers.

## Non-goals

- Opening sockets.
- Async socket handling.
- Unix sockets.
- Socket options.

## License

Licensed under `MIT OR Apache-2.0`.
