# Installation

## Minimum setup
Add `twill` to your `Cargo.toml`:

```toml
[dependencies]
twill = "0.3"
```

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
