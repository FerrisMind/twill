# Tokens Example

This example shows the lowest layer of the API: typed tokens for color, spacing, typography, shadow, and their custom-value escape hatches.

- File: `examples/01_tokens.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example tokens
```

- Expected output: a small terminal dump of palette values, spacing in pixels, representative typography/shadow tokens, and typed custom token wrappers.

Why this exists:
use it when you need to inspect or explain Twill's primitive values before composing full `Style` objects.
