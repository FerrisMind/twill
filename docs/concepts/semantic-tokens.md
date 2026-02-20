# Semantic Tokens

Semantic tokens map role-based names (background, foreground, primary, etc.) to concrete colors in light and dark palettes.

```rust
use twill::{SemanticColor, SemanticThemeVars};

let theme = SemanticThemeVars::shadcn_neutral();
let light_bg = theme.resolve_light(SemanticColor::Background);
let dark_bg = theme.resolve_dark(SemanticColor::Background);
```
