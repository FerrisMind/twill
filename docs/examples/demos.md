# Demo Applications

Twill ships three GUI demos plus a generic CSS-style demo:

- `examples/demo-egui.rs`
- `examples/demo-iced.rs`
- `examples/demo-slint.rs`
- `examples/demo.rs`

## Run all GUI demos

```bash
cargo run --example demo-egui --features egui
cargo run --example demo-iced --features iced
cargo run --example demo-slint --features slint
```

## What each demo shows
- Tabs for different documentation areas
- Color tokens and spacing/radius visual blocks
- Typography and component variants
- Motion token references
- Semantic token behavior in light/dark mode

