# Quick Start

```rust
use twill::prelude::core::*;

let style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .background_color(Color::blue(Scale::S500))
    .rounded(BorderRadius::Md);
```

```rust
use twill::prelude::core::*;

let style = Style::new()
    .padding(Padding::all(Spacing::S4))
    .background_color(Color::blue(Scale::S500))
    .hover(|style| style.opacity(0.9))
    .at_md(|style| style.padding(Padding::all(Spacing::S6)));
```

Use `twill::prelude::*` when you need advanced arbitrary/custom-property wrappers in the same import.
