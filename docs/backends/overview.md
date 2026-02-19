# Backends Overview

Rustwind core is backend-agnostic.  
Adapters convert tokens/styles into framework-specific types.

Supported adapters:
- `egui`
- `iced`
- `slint`

## Enabling backend features

```toml
[dependencies]
rustwind = { version = "0.2", features = ["egui", "iced", "slint"] }
```

## Backend modules
- `rustwind::backends::egui`
- `rustwind::backends::iced`
- `rustwind::backends::slint`

## Common pattern
1. Build style/tokens in Rustwind.
2. Convert via backend helper.
3. Apply to widgets in that GUI framework.

