# use-net

`use-net` is the umbrella crate for the RustUse networking utility workspace.

> Experimental: this crate is below 0.3.0 and the API may change.

## Example Usage

Enable the features you want and import the re-exported child crates:

```rust,ignore
use use_net::{ip, socket};

assert!(ip::is_ipv4("127.0.0.1"));

let endpoint = socket::parse_socket_endpoint("localhost:8080").unwrap();

assert_eq!(endpoint.port, 8080);
```

## Scope

- Feature-gated re-exports for the focused networking crates in this workspace.
- A small facade for consumers that want one dependency entry point.

## Non-goals

- New networking behavior beyond re-exporting the focused crates.
- Hiding crate boundaries or replacing focused crate APIs.

## License

Licensed under `MIT OR Apache-2.0`.
