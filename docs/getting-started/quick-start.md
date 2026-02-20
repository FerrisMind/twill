# Quick Start

```rust
use twill::{BorderRadius, Color, Padding, Scale, Spacing, Style};

let style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .rounded(BorderRadius::Md);
```

```rust
use twill::Button;

let primary = Button::primary().lg();
let outline = Button::outline().sm();
```
