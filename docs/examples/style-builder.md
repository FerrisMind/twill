# Style Builder Example

This example shows how a reusable base `Style` is composed with an additional layer through `with(...)`, including layout and spacing escape hatches.

- File: `examples/02_style_builder.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example style_builder
```

- Expected output: the merged padding, background, text color, width, constraints, layout tokens, radius, and shadow values.

Why this exists:
it demonstrates that Twill's core abstraction is style composition, not framework-specific components, and that arbitrary/custom values still live inside the same typed builder surface.
