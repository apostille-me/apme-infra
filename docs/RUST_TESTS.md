# Rust infrastructure source-contract tests

The canonical offline validator is `tools/infra-contract-check`. It checks
the tracked service catalog and every Kubernetes `.yaml` file without
contacting Supabase, Neon, Cloudflare, Kubernetes, or a customer database.

```sh
cargo fmt --manifest-path tools/infra-contract-check/Cargo.toml --all --check
cargo clippy --manifest-path tools/infra-contract-check/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path tools/infra-contract-check/Cargo.toml --all-targets
cargo run --manifest-path tools/infra-contract-check/Cargo.toml -- --root .
```

The Rust unit tests include malformed JSON, wrong organization, duplicate
service names, and missing or commented-out Kubernetes metadata. The CI gate
must run these Rust tests against the exact PR head. Production provider
checks are separate; none of these tests establishes live readiness.

`scripts/check-db-providers.py` and its Supabase workflow are **legacy
migration debt**, not a model to copy. Before moving that check, preserve
JSON Schema Draft 2020-12 validation, TOML parsing, migration naming and
namespace checks, secret-shaped literal rejection, and offline negative
fixtures in a reviewed Rust replacement.
