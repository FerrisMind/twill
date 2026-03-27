# Semantic Theme Example

This example shows both the built-in `SemanticThemeVars::shadcn_neutral()` palette and a runtime-generated `DynamicSemanticTheme`.

- File: `examples/05_semantic_theme.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example semantic_theme
```

- Expected output: light and dark semantic resolutions for key aliases like `Background`, `Primary`, `Border`, and `Ring`.

Why this exists:
semantic aliases are the bridge between raw tokens and app-level themes.
