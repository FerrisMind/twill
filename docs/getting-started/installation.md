# Installation

## Minimum setup
Choose the smallest crate that fits your use case.

Backend-agnostic core only:

```toml
[dependencies]
twill-core = "0.3"
```

Facade crate with the same core API:

```toml
[dependencies]
twill = "0.3"
```

MSRV: Rust `1.93`.

## Enable GUI backends
Enable only the backends you use:

```toml
[dependencies]
twill = { version = "0.3", features = ["egui"] }
# or
twill = { version = "0.3", features = ["iced"] }
# or
twill = { version = "0.3", features = ["slint"] }
```

You can combine features:

```toml
[dependencies]
twill = { version = "0.3", features = ["egui", "iced", "slint"] }
```

## Feature notes

- `twill-core` stays backend-agnostic and does not require a GUI runtime.
- Base `twill` simply re-exports `twill-core`; it does not require a GUI runtime until you enable a backend feature.
- `egui` enables egui conversion helpers only.
- `iced` enables the Iced adapter and the Linux windowing/runtime feature set used by this crate configuration.
- `slint` enables Slint conversion helpers only.

## Verify installation
Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" check
```

If you enabled backend features, you can verify the crate builds with:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build --features egui
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build --features iced
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build --features slint
```
