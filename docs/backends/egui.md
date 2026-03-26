# egui

Enable feature:

```toml
twill = { version = "0.3", features = ["egui"] }
```

## What you get
- color conversion helpers,
- style integration points for `egui` widgets,
- typed translation from Twill tokens into `egui` primitives.

Use `twill::backends::egui::ToEgui` and the frame/color helpers to bridge `Style` into `egui`.
