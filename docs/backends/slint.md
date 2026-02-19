# slint

Enable feature:

```toml
rustwind = { version = "0.2", features = ["slint"] }
```

## What you get
- direct `Color -> slint::Color` conversion helpers,
- integration with Slint property-based styling,
- demo example: `examples/demo-slint.rs`.

## Run demo

```bash
cargo run --example demo-slint --features slint
```

In the demo, theme-dependent colors are resolved from Rustwind and pushed to Slint properties.

