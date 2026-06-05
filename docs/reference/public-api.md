# Public API

`twill` keeps its public API centered on style primitives and backend adapters.

If you only need the backend-agnostic style engine, depend on `twill-core` directly. The
top-level `twill` crate is now a facade that re-exports the same core modules and optionally
adds GUI adapters.

## Core entry points

- `Style`
- `twill-core`
- `twill::prelude::core::*`
- `twill::prelude::theme::*`
- `twill::prelude::arbitrary::*`
- `twill::prelude::traits::*`
- `twill::prelude::*`
- `Merge`
- `ComputeValue`
- `Responsive`

## Tokens

Main token families:

- `Color`, `ColorFamily`, `Scale`, `ColorValue`
- `BackgroundColor`, `TextColor`, `BorderColor`, `OutlineColor`, `RingColor`
- `ColorValueToken` and `*ColorVar` wrappers for typed arbitrary/custom-property values
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
use twill::prelude::core::*;

let card = Style::card()
    .merged(Style::interactive())
    .padding(Padding::all(Spacing::S4))
    .hover(|style| style.shadow(Shadow::Md))
    .at_lg(|style| style.padding(Padding::all(Spacing::S6)));

assert!(!card.is_empty());
```

## Composition helpers

Prefer composing `Style` values instead of creating framework-specific wrapper types:

- `Style::surface()` for structural spacing, radius, and elevation
- `Style::card()` for semantic card/background/border defaults
- `Style::interactive()` for pointer/focus/disabled affordances
- `Style::merged(...)` when you want an immutable composition step
- `Style::merge_in_place(...)` when you want to extend a mutable style value
- `Style::resolved_theme(...)` plus `resolved_light_theme(...)` / `resolved_dark_theme(...)`
  when you want semantic aliases converted into concrete color tokens
- `Style::resolve_theme_in_place(...)` when you want that resolution on a mutable style
- Verbose builder aliases like `background_color(...)`, `border_color(...)`,
  `outline_color(...)`, `ring_width(...)`, `ring_color(...)`, and `box_shadow(...)`
  when you want a more self-explanatory Rust-first call site

The wide `twill::prelude::*` remains available, but `prelude::core::*` plus opt-in
sub-preludes gives better IDE discoverability and smaller import scope.

## Backend surface

Feature-gated backend modules:

- `twill::backends::egui`
- `twill::backends::iced`
- `twill::backends::slint`
- `twill::egui`
- `twill::iced`
- `twill::slint`

These modules convert Twill values into framework-specific primitives. Twill itself does not define components like `Button` or `Card`.

If you do not want the facade crate, use the dedicated adapter crates directly:

- `twill-egui`
- `twill-iced`
- `twill-slint`

Backend-specific helper types:

- `twill::backends::ShadowColor`
- `twill::iced::TextDirection`
- `twill::slint::SlintCursor`
