# twill-slint

`twill-slint` is the direct `slint` adapter crate for the Twill styling ecosystem.

Use it when you want:

- `twill-core` for the backend-agnostic style model
- `twill-slint` for converting Twill tokens and `Style` values into `slint` primitives
- no facade crate in your dependency graph

## Installation

```toml
[dependencies]
twill-core = "0.3"
twill-slint = "0.3"
```

## What it provides

- `ToSlint` for canonical typed conversions
- helpers like `to_slint_color`, `to_length`, `to_radius`, and semantic color conversion
- no component abstractions and no CSS layer

If you prefer one facade crate, use `twill = { version = "0.3", features = ["slint"] }` instead.
