# Contributing To Twill

Twill is a Rust-first styling crate. Keep changes additive, type-safe, and easy to discover.

## Local workflow

Run the core checks before opening a PR:

```bash
cargo fmt --all --check
cargo test
cargo clippy --all-features --all-targets -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
```

## API principles

- Prefer additive APIs over breaking renames in `0.3.x`.
- Keep `Style` as the main composition surface instead of introducing component abstractions.
- Prefer typed entry points such as `prelude::core::*`, `data_attr(...)`, and `aria_attr(...)`.
- Leave raw escape hatches available when they remove friction for advanced users.
- Avoid backend-specific behavior in the core crate unless it is fully feature-gated.

## Docs and examples

- Update README, rustdoc, and mdBook pages when the recommended API changes.
- Prefer short examples that compile and demonstrate one idea clearly.
- Keep onboarding examples on the narrow `prelude::core::*` path unless they need advanced wrappers.
