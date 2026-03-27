# Arbitrary Values Example

This example shows the controlled escape-hatch layer of `twill`: arbitrary and custom-property values for key utility families without falling back to a class-string parser.

- File: `examples/06_arbitrary_values.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example arbitrary_values
```

- Expected output: a terminal dump showing arbitrary/custom tokens for background, text, spacing, constraints, border, ring, shadow color, typography overrides, and custom motion/effect values.

Why this exists:
use it when preset tokens cover most of the design, but a backend-specific edge case still needs a typed arbitrary value or a custom-property hook for color, spacing, size, typography, or motion.
