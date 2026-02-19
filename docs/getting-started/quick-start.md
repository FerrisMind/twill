# Quick Start

## Basic style composition

```rust
use rustwind::{Style, Color, Scale, Padding, Spacing, BorderRadius, ToCss};

let style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .text_color(Color::slate(Scale::S50))
    .rounded(BorderRadius::Md);

println!("{}", style.to_css());
```

## Prebuilt button

```rust
use rustwind::{Button, ToCss};

let primary = Button::primary().lg().to_css();
let outline = Button::outline().sm().to_css();
```

## Layout utilities

```rust
use rustwind::{Style, Spacing, Padding};

let centered = Style::centered_col()
    .gap(Spacing::S4)
    .padding(Padding::all(Spacing::S6));
```

## Next steps
1. Explore all token families in [Design Tokens](../concepts/tokens.md)
2. Learn fluent methods in [Style Builder](../concepts/style-builder.md)
3. Map styles to GUI frameworks in [Backends Overview](../backends/overview.md)

