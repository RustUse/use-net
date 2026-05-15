# use-cidr

`use-cidr` provides small helpers for parsing CIDR notation and validating prefix lengths.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_cidr::{format_cidr, parse_cidr};

let block = parse_cidr("192.168.0.0/24").unwrap();

assert_eq!(block.prefix, 24);
assert_eq!(format_cidr(&block), "192.168.0.0/24");
```

## Scope

- CIDR parsing for IPv4 and IPv6 blocks.
- Prefix-length validation helpers.

## Non-goals

- Full subnet math.
- Route table implementation.
- IP range expansion.
- Firewall logic.

## License

Licensed under `MIT OR Apache-2.0`.
