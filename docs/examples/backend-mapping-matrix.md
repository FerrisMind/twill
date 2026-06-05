# Backend Mapping Matrix

This example demonstrates the backend-agnostic promise of Twill: the same token and style inputs can be mapped into `egui`, `iced`, and `slint`.

- File: `examples/13_backend_mapping_matrix.rs`
- Run:

```bash
cargo run --example backend_mapping_matrix --features egui,iced,slint
```

- Expected output: a terminal dump showing parallel conversions for colors, spacing, radius, shadows, and semantic tokens across the three supported adapters.

Why this exists:
it gives downstream users one place to validate backend parity without opening three separate examples side by side.
