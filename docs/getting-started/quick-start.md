# Quick Start

```rust
use twill::prelude::core::*;

let style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .rounded(BorderRadius::Md);
```

```rust
use twill::prelude::core::*;

let style = Style::new()
    .padding(Padding::all(Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .hover(|style| style.opacity(0.9))
    .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
    .at_md(|style| style.padding(Padding::all(Spacing::S6)));
```
