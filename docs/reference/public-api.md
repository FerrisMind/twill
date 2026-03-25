# Public API

`twill` exposes a small set of high-value entry points and keeps most day-to-day usage centered on three layers:

1. `Style` for fluent composition.
2. typed tokens such as `Color`, `Spacing`, `FontSize`, `BorderRadius`.
3. reusable component descriptors such as `Button`.

## Core Entry Points

- `Style`
- `Button`, `ButtonVariant`, `ButtonSize`
- `ComputeValue`, `Merge`

## Tokens

Primary token families exported from the crate root:

- color and semantic tokens:
  `Color`, `ColorFamily`, `Scale`, `ColorValue`, `BackgroundColor`, `BackgroundColorVar`,
  `SemanticColor`, `SemanticThemeVars`, `DynamicSemanticTheme`
- spacing and sizing tokens:
  `Spacing`, `Percentage`, `Container`, `Padding`, `PaddingValue`, `PaddingVar`,
  `Margin`, `MarginValue`, `MarginVar`, `Width`, `WidthVar`, `Height`, `HeightVar`,
  `SizeConstraints`
- typography and motion tokens:
  `FontFamily`, `FontSize`, `FontSizeVar`, `FontWeight`, `LetterSpacing`,
  `LineHeight`, `TextAlign`, `TransitionDuration`, `TransitionProperty`, `AnimationToken`

## Recommended Usage

```rust
use twill::{
    BorderRadius, Button, Color, Padding, Scale, Spacing, Style,
};

let card = Style::new()
    .padding(Padding::all(Spacing::S4))
    .bg(Color::slate(Scale::S50))
    .rounded(BorderRadius::Lg);

let cta = Button::primary().lg().full_width();

let combined = card.merge(cta.style());
assert!(!combined.is_empty());
```

## Standard Conversions

Several public types implement standard conversion traits to keep call sites idiomatic:

- `From<Button> for Style`
- `From<(ButtonVariant, ButtonSize)> for Button`
- `From<Padding> for Style`
- `From<Margin> for Style`
- `From<Width> for Style`
- `From<Height> for Style`
- `From<FlexContainer> for Style`
- `From<GridContainer> for Style`

Custom-property wrapper types also implement `AsRef<str>`, `Display`, and `From<&'static str>`:

- `PaddingVar`
- `MarginVar`
- `WidthVar`
- `HeightVar`
- `BackgroundColorVar`
- `FontSizeVar`

## Component API Notes

`Button` is a plain value object. It is cheap to copy, derives common comparison traits, and can be configured fluently:

```rust
use twill::{Button, ButtonSize, ButtonVariant};

let button = Button::default()
    .with_variant(ButtonVariant::Outline)
    .with_size(ButtonSize::Lg)
    .disabled_if(false)
    .full_width_if(true);

assert_eq!(button.variant(), ButtonVariant::Outline);
assert_eq!(button.size(), ButtonSize::Lg);
assert!(button.is_full_width());
```

## Backend Surface

Feature-gated backend modules are re-exported from the crate root:

- `twill::egui`
- `twill::iced`
- `twill::slint`

Each backend is responsible for translating `twill` tokens and `Style` values into framework-specific primitives without changing the core API model.
