# Installation

## Minimum setup
Add `twill` to your `Cargo.toml`:

```toml
[dependencies]
twill = "0.2"
```

## Enable GUI backends
Enable only the backends you use:

```toml
[dependencies]
twill = { version = "0.2", features = ["egui"] }
# or
twill = { version = "0.2", features = ["iced"] }
# or
twill = { version = "0.2", features = ["slint"] }
```

You can combine features:

```toml
[dependencies]
twill = { version = "0.2", features = ["egui", "iced", "slint"] }
```

## Verify installation
Run:

```bash
cargo check
```

If you enabled example backends, you can run:

```bash
cargo run --example demo-egui --features egui
cargo run --example demo-iced --features iced
cargo run --example demo-slint --features slint
```

