# Migrating From 0.2.x

This branch documents the `0.3.x` API only.
If you are coming from `0.2.x`, the important change is conceptual:
Twill is now a style-composition crate, not a component crate and not a CSS serialization layer.

## What changed

- `Style` is the center of the public API.
- Backend adapters stay feature-gated and convert Twill values into `egui`, `iced`, and `slint` types.
- Legacy component demos and CSS-oriented APIs belong to the `0.2.x` line.

## Recommended 0.3.x path

Start with:

```rust
use twill::prelude::core::*;
```

Use the full prelude only when you need advanced arbitrary/custom-property wrappers:

```rust
use twill::prelude::{arbitrary::*, core::*};
```

## Practical API updates

- Prefer `Style::background_color(...)` when discoverability matters; `bg(...)` still exists.
- Prefer `data_attr(DataState::..., ...)` and `aria_attr(AriaAttr::..., ...)` over raw strings.
- Prefer `Style::at_breakpoint(...)` in normal application code.
- Use `at_sm` / `at_md` / `at_lg` / `at_xl` / `at_2xl` if the short breakpoint builders feel too terse.
- Prefer `Style::surface()`, `Style::card()`, and `Style::interactive()` as reusable starting points.
- Prefer `merged(...)` or `merge_in_place(...)` for explicit style composition.

## What did not change

- The short DSL remains available.
- Raw escape hatches like `data_state("...")` and `aria_state("...")` still exist.
- Backend support is still opt-in through Cargo features.

## Validation checklist

- Replace broad starter imports with `prelude::core::*` where possible.
- Update raw `data-state` / `aria-*` selectors to typed helpers when the built-in enums cover the case.
- Re-run:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-features --examples
```
