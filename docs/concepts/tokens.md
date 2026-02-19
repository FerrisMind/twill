# Design Tokens

Twill tokens are Rust enums and value types representing styling primitives.

## Color tokens
Use `Color` + `Scale`:

```rust
use twill::{Color, Scale};

let primary = Color::blue(Scale::S500);
let danger = Color::red(Scale::S600);
let bg = Color::gray(Scale::S50);
```

Special colors:
- `Color::white()`
- `Color::black()`
- `Color::transparent()`

## Spacing tokens
Use `Spacing` for paddings, margins, gaps:

```rust
use twill::Spacing;

let p = Spacing::S4;   // 1rem
let gap = Spacing::S2; // 0.5rem
```

## Border and radius tokens
- `BorderWidth`
- `BorderStyle`
- `BorderRadius`

## Typography tokens
- `FontFamily`
- `FontSize`
- `FontWeight`
- `LineHeight`
- `LetterSpacing`
- `TextAlign`

## Shadow tokens
- `Shadow`
- `InsetShadow`
- `DropShadow`
- `TextShadow`

## Motion tokens
- `TransitionDuration`
- `Easing`
- `AnimationToken`

Motion is applied through `Style` methods:
- `transition_property(...)`
- `transition_duration(...)`
- `transition_ease(...)`
- `animate(...)`

