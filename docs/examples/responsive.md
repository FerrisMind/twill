# Responsive Example

This example shows breakpoint layers and how they resolve through `Style::at_breakpoint(...)`.

- File: `examples/04_responsive.rs`
- Run:

```bash
cargo run --example responsive
```

- Expected output: one line per breakpoint showing the resolved width, height, padding, and shadow values.

Why this exists:
responsive behavior should be inspectable without launching a GUI backend.
The example also uses the more discoverable `at_md`/`at_2xl` builder aliases.
