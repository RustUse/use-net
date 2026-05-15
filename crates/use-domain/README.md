# use-domain

`use-domain` provides lightweight helpers for validating, normalizing, and inspecting domain names and hostnames.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

```rust
use use_domain::{normalize_domain, root_domain_guess, split_domain_labels};

assert_eq!(normalize_domain("WWW.Example.com."), Some("www.example.com".to_string()));
assert_eq!(split_domain_labels("api.example.com"), vec!["api", "example", "com"]);
assert_eq!(root_domain_guess("api.example.com"), Some("example.com".to_string()));
```

## Scope

- Domain and hostname validation.
- Domain normalization and label splitting.
- Small inspection helpers such as depth and subdomain checks.

## Non-goals

- Complete public suffix list support.
- IDNA conversion.
- DNS lookup.
- Certificate validation.

## License

Licensed under `MIT OR Apache-2.0`.
