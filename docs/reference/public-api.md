# Public API

`twill` keeps its public API centered on style primitives and backend adapters.

## Core entry points

- `Style`
- `twill::prelude::*`
- `Merge`
- `ComputeValue`
- `Responsive`

## Tokens

Main token families:

- `Color`, `ColorFamily`, `Scale`, `ColorValue`
- `SemanticColor`, `SemanticThemeVars`, `DynamicSemanticTheme`
- `ThemeVariant`
- `Spacing`, `Percentage`, `Container`, `Breakpoint`
- `FontFamily`, `FontSize`, `FontWeight`, `LetterSpacing`, `LineHeight`
- `BorderRadius`, `BorderWidth`, `BorderStyle`, `OutlineStyle`, `RingWidth`
- `Shadow`, `InsetShadow`, `DropShadow`, `TextShadow`
- `AnimationToken`, `TransitionDuration`, `TransitionProperty`, `Easing`

## Utilities

Common utility-style value types:

- `Padding`, `PaddingValue`, `PaddingVar`
- `Margin`, `MarginValue`, `MarginVar`
- `Width`, `WidthVar`
- `Height`, `HeightVar`
- `SizeConstraints`
- `Display`, `Position`, `Overflow`, `ZIndex`
- `FlexContainer`, `FlexDirection`, `Flex`
- `GridContainer`, `GridTemplate`

## Recommended usage

```rust
use twill::prelude::*;

let card = Style::new()
    .padding(Padding::all(Spacing::S4))
    .bg(Color::slate(Scale::S50))
    .rounded(BorderRadius::Lg)
    .hover(|style| style.shadow(Shadow::Md))
    .lg(|style| style.padding(Padding::all(Spacing::S6)));

assert!(!card.is_empty());
```

## Backend surface

Feature-gated backend modules:

- `twill::egui`
- `twill::iced`
- `twill::slint`

These modules convert Twill values into framework-specific primitives. Twill itself does not define components like `Button` or `Card`.

Backend-specific helper types:

- `twill::backends::ShadowColor`
- `twill::iced::TextDirection`
- `twill::slint::SlintCursor`
