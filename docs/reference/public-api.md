# Public API

This page summarizes the primary exports from `rustwind`.

## Core traits
- `ToCss`
- `Merge`
- `ComputeValue`

## Core builder
- `Style`

## Tokens (selected)
- Colors: `Color`, `ColorFamily`, `Scale`, `SpecialColor`
- Spacing and size: `Spacing`
- Borders: `BorderWidth`, `BorderStyle`, `BorderRadius`
- Typography: `FontFamily`, `FontSize`, `FontWeight`, `LineHeight`, `LetterSpacing`
- Shadows: `Shadow`, `DropShadow`, `InsetShadow`, `TextShadow`
- Motion: `TransitionDuration`, `Easing`, `AnimationToken`, `MotionDefaults`
- Semantic: `SemanticColor`, `SemanticThemeVars`

## Utilities
- Layout/display: `Display`, `Position`, `Overflow`, `ZIndex`
- Flex/grid: `FlexContainer`, `FlexDirection`, `FlexWrap`, `GridContainer`, `GridTemplate`
- Spacing wrappers: `Padding`, `Margin`
- Size wrappers: `Width`, `Height`, `SizeConstraints`
- Alignment: `AlignItems`, `AlignSelf`, `JustifyContent`

## Components
- `Button`
- `ButtonVariant`
- `ButtonSize`

## Feature-gated backend exports
- `rustwind::egui` with `egui` feature
- `rustwind::iced` with `iced` feature
- `rustwind::slint` with `slint` feature

## Shortcuts
`rustwind` also exports:
- `colors::*` helper module
- `spacing::*` helper module

