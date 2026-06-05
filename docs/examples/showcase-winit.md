# winit Showcase

This showcase runs the same `slint`-based Twill UI on an explicitly selected `winit` backend. It exists to demonstrate a lower-level Rust-native windowing path without introducing a dedicated `twill-winit` adapter crate.

- File: `examples/23_showcase_winit.rs`
- Run:

```bash
cargo run --example showcase_winit --features slint
```

- Expected UI: a native window titled `twill showcase (winit)` with the same token, semantic theme, and composed-style sections as the `slint` showcase, plus runtime information sourced from the underlying `winit::Window`.

Why this exists:
it gives Twill users a pragmatic `winit` integration reference while keeping the crate surface small and avoiding premature backend-specific API design.
