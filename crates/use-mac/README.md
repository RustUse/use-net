# use-mac

`use-mac` provides small helpers for parsing, formatting, and normalizing MAC addresses.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_mac::{normalize_mac, parse_mac_address};

let mac = parse_mac_address("00-1A-2B-3C-4D-5E").unwrap();

assert_eq!(normalize_mac("001A2B3C4D5E"), Some("00:1A:2B:3C:4D:5E".to_string()));
assert_eq!(mac.octets, [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
```

## Scope

- Parsing common MAC address formats.
- Formatting helpers for colon and hyphen output.
- Small helpers for broadcast and zero-address checks.

## Non-goals

- Vendor lookup.
- Network adapter inspection.
- Packet parsing.

## License

Licensed under `MIT OR Apache-2.0`.