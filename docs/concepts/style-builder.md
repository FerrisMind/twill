# Style Builder

`Style` is the central composition object. You combine typed tokens and utilities, then map the resulting structure to a native backend.

```rust
use twill::{Color, Scale, Style};

let card = Style::new()
    .bg(Color::white())
    .text_color(Color::gray(Scale::S900));
```
