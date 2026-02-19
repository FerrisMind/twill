# Style Builder

`Style` is the core fluent API for composing utility-like style blocks.

## Typical chain

```rust
use rustwind::{
    Style, Color, Scale, Padding, Spacing, BorderRadius, Shadow,
    Display, FlexContainer, JustifyContent, AlignItems
};

let card = Style::new()
    .display(Display::Flex)
    .flex(FlexContainer::row()
        .justify(JustifyContent::Center)
        .align(AlignItems::Center))
    .padding(Padding::all(Spacing::S4))
    .bg(Color::white())
    .rounded(BorderRadius::Lg)
    .shadow(Shadow::Md);
```

## Main groups of methods
- Layout: `display`, `position`, `z_index`, `overflow`
- Flex/Grid: `flex`, `grid`, `gap`
- Spacing: `padding`, `margin`
- Size: `width`, `height`, `constraints`
- Visuals: `bg`, `opacity`, `border`, `rounded`, `shadow`
- Typography: `font`, `text_size`, `font_weight`, `text_color`, `text_align`
- Motion: transition and animation methods

## Merging styles
Use `Merge` to compose reusable snippets:

```rust
use rustwind::{Style, Merge, Padding, Spacing};

let base = Style::new().padding(Padding::all(Spacing::S3));
let elevated = Style::new();
let composed = base.merge(elevated);
```

## CSS output
Use `ToCss`:

```rust
use rustwind::ToCss;

let css = card.to_css();
println!("{css}");
```

