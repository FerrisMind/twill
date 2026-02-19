# Semantic Tokens

Semantic tokens are optional.  
You can keep using raw palette tokens (`Color::blue(Scale::S500)`) without semantic layer.

## Why use semantics
Use semantic names like `Primary`, `Background`, `MutedForeground` instead of direct colors for:
- theme switching,
- consistent design systems,
- easier UI refactoring.

## API
- `SemanticColor` enum
- `SemanticThemeVars::shadcn_neutral()`
- `resolve(token, is_dark)` / `resolve_light` / `resolve_dark`
- `to_css_variables()` for CSS-variable output

## Example: resolve into concrete color

```rust
use twill::{SemanticThemeVars, SemanticColor};

let theme = SemanticThemeVars::shadcn_neutral();
let light_bg = theme.resolve_light(SemanticColor::Background).unwrap();
let dark_bg = theme.resolve_dark(SemanticColor::Background).unwrap();
```

## Example: export CSS variables

```rust
use twill::SemanticThemeVars;

let css_vars = SemanticThemeVars::shadcn_neutral().to_css_variables();
println!("{css_vars}");
```

This emits `:root` and `.dark` variable blocks for CSS-capable frontends.

