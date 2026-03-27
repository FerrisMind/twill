# iced Showcase

This is the full `iced` showcase for `main`/`0.3.x`. It mirrors the conceptual content of the `egui` showcase while keeping the backend-specific wiring in `iced`.

- File: `examples/21_showcase_iced.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example showcase_iced --features iced
```

- Expected UI: a native `iced` window with token swatches, semantic theme sections, and composed cards rendered through `twill::backends::iced`.

Why this exists:
it proves that the same Twill style story can be surfaced in a second backend without reintroducing old component abstractions into the crate.
