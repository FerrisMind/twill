# Quick Start

```rust
use twill::{BorderRadius, Color, Padding, Scale, Spacing, Style};

let style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .rounded(BorderRadius::Md);
```

```rust
use twill::prelude::*;

let style = Style::new()
    .padding(Padding::all(Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .hover(|style| style.opacity(0.9))
    .md(|style| style.padding(Padding::all(Spacing::S6)));
```
