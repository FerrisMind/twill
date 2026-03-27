# egui Showcase

This is the full `egui` showcase for `main`/`0.3.x`. It combines tokens, composed styles, state layers, responsive resolution, and semantic theme switching in one window.

- File: `examples/20_showcase_egui.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example showcase_egui --features egui
```

- Expected UI: a native `egui` window with token swatches, semantic theme labels, and composed cards built from shared Twill styles.

Why this exists:
it is the visual "map" of the system for `egui`, while the smaller examples stay optimized for one concept at a time.
