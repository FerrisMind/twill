# Quick Start

```rust
use twill::prelude::core::*;

let style = Style::card()
    .merged(Style::interactive())
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .hover(|style| style.opacity(0.9))
    .at_md(|style| style.padding(Padding::all(Spacing::S6)));
```

Recommended imports:

- `twill::prelude::core::*` for normal application code.
- `twill::prelude::theme::*` when you need semantic tokens and theme resolvers.
- `twill::prelude::arbitrary::*` when you need typed arbitrary/custom-property values.
- `twill::prelude::traits::*` for `Merge`, `Responsive`, and other integration traits.

```rust
use twill::prelude::{arbitrary::*, core::*};

let style = Style::new()
    .bg_arbitrary(ColorValueToken::from_rgb8(15, 23, 42))
    .text_color_arbitrary(ColorValueToken::from_rgb8(248, 250, 252))
    .px_var(PaddingVar::new("--panel-pad-x"))
    .min_w_var(WidthVar::new("--panel-min-w"))
    .tracking_em(0.035)
    .transition_custom("filter, transform");
```
