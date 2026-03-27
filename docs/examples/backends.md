# Backend Adapter Examples

These examples keep backend coverage focused on adapter deltas instead of repeating the full product story.

## egui

- File: `examples/10_backend_egui.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example backend_egui --features egui
```

- Expected output: converted `egui` primitives such as `Color32`, `Frame`, cursor, semantic colors, and shadow values.

## iced

- File: `examples/11_backend_iced.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example backend_iced --features iced
```

- Expected output: converted `iced` primitives such as `Color`, `Padding`, cursor interaction, semantic colors, and shadows.

## slint

- File: `examples/12_backend_slint.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example backend_slint --features slint
```

- Expected output: converted `slint` primitives such as `slint::Color`, lengths, cursor wrapper, semantic colors, and shadow tuples.

Why these exist:
each adapter example answers "how do I bridge Twill values into this runtime?" without pretending Twill ships a full widget kit.
