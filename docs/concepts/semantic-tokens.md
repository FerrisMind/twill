# Semantic Tokens

Semantic tokens map role-based names (background, foreground, primary, etc.) to concrete colors in light and dark palettes.

```rust
use twill::prelude::theme::*;

let theme = SemanticThemeVars::shadcn_neutral();
let light_bg = theme.resolve_light(SemanticColor::Background);
let dark_bg = theme.resolve_dark(SemanticColor::Background);
```

If you want to resolve semantic aliases before handing a style to a backend, do it on `Style`
directly:

```rust
use twill::prelude::{arbitrary::*, core::*, theme::*};

let theme = SemanticThemeVars::shadcn_neutral();
let style = Style::card()
    .text_color_token(TextColor::semantic(SemanticColor::Primary))
    .ring_token(RingWidth::S2, RingColor::semantic(SemanticColor::Ring));

let resolved = style.resolved_dark_theme(theme);

assert!(matches!(
    resolved.text_color_token_value(),
    Some(TextColor::Arbitrary(_))
));
```
