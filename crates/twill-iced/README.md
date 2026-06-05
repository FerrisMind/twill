# twill-iced

`twill-iced` is the direct `iced` adapter crate for the Twill styling ecosystem.

Use it when you want:

- `twill-core` for the backend-agnostic style model
- `twill-iced` for converting Twill tokens and `Style` values into `iced` primitives
- no facade crate in your dependency graph

## Installation

```toml
[dependencies]
twill-core = "0.3"
twill-iced = "0.3"
```

## What it provides

- `ToIced` for canonical typed conversions
- helpers like `to_color`, `to_padding`, `styled_container`, `apply_layout`, and semantic theme-aware conversions
- no component abstractions and no CSS layer

If you prefer one facade crate, use `twill = { version = "0.3", features = ["iced"] }` instead.
