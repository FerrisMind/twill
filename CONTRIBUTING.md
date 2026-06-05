# Contributing To Twill

Twill is a Rust-first styling crate. Keep changes additive, type-safe, and easy to discover.

## Local workflow

Run the core checks before opening a PR:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-features --examples
cargo doc --workspace --all-features --no-deps
```

## API principles

- Prefer additive APIs over breaking renames in `0.3.x`.
- Keep `Style` as the main composition surface instead of introducing component abstractions.
- Prefer typed entry points such as `prelude::core::*`, `data_attr(...)`, and `aria_attr(...)`.
- Leave raw escape hatches available when they remove friction for advanced users.
- Keep backend-specific behavior in dedicated adapter crates unless it genuinely belongs in `twill-core`.
- Preserve the facade ergonomics of `twill`, but keep adapter logic publishable as standalone crates.

## Docs and examples

- Update README, rustdoc, and mdBook pages when the recommended API changes.
- Prefer short examples that compile and demonstrate one idea clearly.
- Keep onboarding examples on the narrow `prelude::core::*` path unless they need advanced wrappers.
