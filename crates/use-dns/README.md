# use-dns

`use-dns` provides lightweight helpers for DNS record types and DNS-style names.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_dns::{format_record_type, normalize_dns_name, parse_record_type, DnsRecordType};

assert_eq!(parse_record_type("aaaa"), DnsRecordType::AAAA);
assert_eq!(format_record_type(DnsRecordType::MX), "MX");
assert_eq!(normalize_dns_name("_sip._tcp.EXAMPLE.com."), Some("_sip._tcp.example.com".to_string()));
```

## Scope

- DNS record type parsing and formatting.
- Small DNS record classification helpers.
- DNS-style name normalization.

## Non-goals

- DNS resolution.
- Zone file parsing.
- Resolver implementation.
- DNSSEC validation.

## License

Licensed under `MIT OR Apache-2.0`.
