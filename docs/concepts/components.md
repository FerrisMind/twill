# Components

Rustwind currently provides a `Button` component abstraction.

## Button variants
- `Primary`
- `Secondary`
- `Outline`
- `Ghost`
- `Destructive`
- `Link`

## Button sizes
- `Sm`
- `Md`
- `Lg`
- `Icon`

## Usage

```rust
use rustwind::{Button, ToCss};

let primary = Button::primary().to_css();
let large = Button::primary().lg().to_css();
let outline = Button::outline().sm().to_css();
let danger = Button::destructive().full_width().to_css();
let disabled = Button::secondary().disabled().to_css();
```

## Extracting `Style`
If you need to adapt style per backend:

```rust
use rustwind::Button;

let style = Button::primary().lg().style();
```

