# Releasing

This workspace uses a safety-first release flow. The first public crates.io wave should be
published manually in dependency order, and follow-up releases can use the automated
`release-plz` workflows after every crate in the workspace exists on crates.io.

## crates.io token setup

1. Create or reuse a crates.io API token with publish access for the intended networking crates.
2. Add the token to the GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Do not print the token in logs or local shell history.

## GitHub Actions secret

- Secret name: `CARGO_REGISTRY_TOKEN`

## Initial publish order

Publish the focused crates first, then publish the umbrella crate last:

1. `use-ip`
2. `use-port`
3. `use-host`
4. `use-domain`
5. `use-dns`
6. `use-socket`
7. `use-tcp`
8. `use-udp`
9. `use-cidr`
10. `use-mac`
11. `use-net`

The umbrella crate `use-net` should come last after all focused crates are visible on crates.io.

## Local validation

Validate the workspace before any release work:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Dry-run a focused crate locally:

```sh
cargo publish --dry-run --allow-dirty -p use-ip
```

Dry-run the umbrella crate only after the focused crates are live on crates.io:

```sh
cargo publish --dry-run --allow-dirty -p use-net
```

## Post-initial-release automation

After the first manual crates.io publish wave completes, the repository can use the release
automation below.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request from `release-plz.toml`

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: pushes to `main` when `CRATES_IO_AUTOPUBLISH_ENABLED` is `true`, or manual dispatch
- Required manual input: `post-initial-release = true`
- Purpose: confirms every published crate already exists on crates.io, then runs `release-plz`

## Permanent version warning

Published crates.io versions are permanent. Verify metadata, crate ordering, and changelog inputs
before any real publish.
