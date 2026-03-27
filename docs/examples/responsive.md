# Responsive Example

This example shows breakpoint layers and how they resolve through `Responsive::at_breakpoint(...)`.

- File: `examples/04_responsive.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example responsive
```

- Expected output: one line per breakpoint showing the resolved width, height, padding, and shadow values.

Why this exists:
responsive behavior should be inspectable without launching a GUI backend.
