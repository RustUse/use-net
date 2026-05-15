# use-port

`use-port` provides small helpers for parsing port numbers, classifying port ranges, and mapping a few common services to default ports.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_port::{common_port_name, parse_port, port_range, PortRange};

let port = parse_port("443").unwrap();

assert_eq!(port.value, 443);
assert_eq!(port_range(port.value), PortRange::System);
assert_eq!(common_port_name(port.value), Some("https"));
```

## Scope

- Port parsing from strings.
- Port range classification.
- Small common service-to-port lookup tables.

## Non-goals

- Port scanning.
- Socket binding.
- Service discovery.

## License

Licensed under `MIT OR Apache-2.0`.
