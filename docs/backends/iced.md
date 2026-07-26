# iced

Enable feature:

```toml
twill = { version = "0.3", features = ["iced"] }
```

The `iced` feature enables both WGPU and Tiny Skia. To select only one renderer, use:

```toml
twill = { version = "0.3", features = ["iwgpu"] }  # WGPU only
twill = { version = "0.3", features = ["itskia"] } # Tiny Skia only
```

## What you get
- color/style mapping helpers for `iced`,
- compatibility with `iced` application architecture,
- typed translation from Twill values into `iced` primitives.

Use `twill::backends::iced::ToIced` together with the helper functions in this module to feed `Style` data into your own `iced` widgets, layouts, and themes.
