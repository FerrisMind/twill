# Design System Module

This example shows how to organize Twill in a real application: small `theme` helpers and a `styles` module that exports reusable style recipes.

- File: `examples/25_design_system_module.rs`
- Run:

```bash
cargo run --example design_system_module
```

- Expected output: a terminal dump showing how app-level `surface`, `button_primary`, and `danger_text` recipes resolve into concrete Twill values.

Why this exists:
it is the most copyable example for serious adoption because it mirrors how downstream applications typically structure style code.
