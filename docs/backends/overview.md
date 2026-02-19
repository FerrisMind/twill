# Backends Overview

Twill core is backend-agnostic.  
Adapters convert tokens/styles into framework-specific types.

Supported adapters:
- `egui`
- `iced`
- `slint`

## Enabling backend features

```toml
[dependencies]
twill = { version = "0.2", features = ["egui", "iced", "slint"] }
```

## Backend modules
- `twill::backends::egui`
- `twill::backends::iced`
- `twill::backends::slint`

## Common pattern
1. Build style/tokens in Twill.
2. Convert via backend helper.
3. Apply to widgets in that GUI framework.

