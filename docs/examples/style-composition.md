# Style Composition

This example shows how real Twill code is usually assembled: a base style, one or more variants, interaction layers, and responsive overrides merged into one final `Style`.

- File: `examples/08_style_composition.rs`
- Run:

```bash
cargo run --example style_composition
```

- Expected output: a terminal dump of the composed style plus the resolved values at multiple breakpoints.

Why this exists:
it teaches the main ergonomic pattern of the library without turning examples into giant single-chain builders.
