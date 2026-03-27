# Design Tokens

Twill tokens are Rust enums and value types representing styling primitives.

## Color tokens
Use `Color` + `Scale`:

```rust
use twill::prelude::*;

let primary = Color::blue(Scale::S500);
let danger = Color::red(Scale::S600);
let bg = Color::gray(Scale::S50);
```

Special colors:
- `Color::white()`
- `Color::black()`
- `Color::transparent()`

Typed arbitrary/custom-property escape hatches:
- `ColorValueToken`
- `BackgroundColorVar`
- `TextColorVar`
- `BorderColorVar`
- `OutlineColorVar`
- `RingColorVar`
- `ShadowColorVar`

## Spacing tokens
Use `Spacing` for paddings, margins, gaps:

```rust
use twill::prelude::*;

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

Typography also supports typed custom values:
- `FontSizeVar`
- `LetterSpacingVar`
- `LineHeightVar`
- `LetterSpacing::Em(...)`
- `LineHeight::Number(...)`

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

For edge cases there are typed arbitrary/custom paths such as:
- `transition_custom(...)`
- `transition_duration_ms(...)`
- `blur_px(...)`
- `perspective_px(...)`
