# slint Showcase

This is the full `slint` showcase for `main`/`0.3.x`. It reuses the same Twill token and style story as the `egui` and `iced` showcases, but wires the visuals through `slint` primitives.

- File: `examples/22_showcase_slint.rs`
- Run:

```bash
cargo run --example showcase_slint --features slint
```

- Expected UI: a native `slint` window with token swatches, semantic theme rows, and composed cards built from shared Twill styles.

Why this exists:
it proves that Twill's layered API can stay useful in declarative UI runtimes without introducing a new component abstraction layer.
