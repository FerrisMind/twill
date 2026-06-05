# State Variants

This example gathers the most important state hooks into one scenario.

- File: `examples/09_state_variants.rs`
- Run:

```bash
cargo run --example state_variants
```

- Expected output: a terminal dump of `hover`, `focus-visible`, `disabled`, `active`, `selected`, `checked`, `open`, `closed`, `data-*`, and `aria-*` overrides resolved from one style value.

Why this exists:
state layers are one of Twill's core features, so users need one focused example that demonstrates the whole state vocabulary in one place.
