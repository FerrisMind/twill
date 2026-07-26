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

Both WGPU and Tiny Skia are enabled by default. Iced uses WGPU as the primary
renderer and Tiny Skia as a fallback. To build with only Tiny Skia, disable
default features and enable `tiny-skia`:

```toml
[dependencies]
twill-iced = { version = "0.3", default-features = false, features = ["tiny-skia"] }
```

To build with only WGPU, use `default-features = false` and enable `wgpu`:

```toml
[dependencies]
twill-iced = { version = "0.3", default-features = false, features = ["wgpu"] }
```

## What it provides

- `ToIced` for canonical typed conversions
- helpers like `to_color`, `to_padding`, `styled_container`, `apply_layout`, and semantic theme-aware conversions
- no component abstractions and no CSS layer

If you prefer one facade crate, use `twill = { version = "0.3", features = ["iced"] }` for
both renderers, `twill = { version = "0.3", features = ["iwgpu"] }` for WGPU only,
or `twill = { version = "0.3", features = ["itskia"] }` for Tiny Skia only.
