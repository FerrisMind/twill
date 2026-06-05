# twill-egui

`twill-egui` is the direct `egui` adapter crate for the Twill styling ecosystem.

Use it when you want:

- `twill-core` for the backend-agnostic style model
- `twill-egui` for converting Twill tokens and `Style` values into `egui` primitives
- no facade crate in your dependency graph

## Installation

```toml
[dependencies]
twill-core = "0.3"
twill-egui = "0.3"
```

## What it provides

- `ToEgui` for canonical typed conversions
- helpers like `to_color32`, `to_frame`, `to_cursor_icon`, and semantic color resolution
- no component abstractions and no CSS layer

If you prefer one facade crate, use `twill = { version = "0.3", features = ["egui"] }` instead.
