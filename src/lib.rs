//! # Twill
//!
//! Idiomatic Rust styling library with utility-first design tokens for native GUI.
//!
//! ## Philosophy
//!
//! Applies utility-first styling ideas:
//! - **Design Tokens** - type-safe base values (colors, spacing, sizes)
//! - **Utility-first** - composable atomic styles
//! - **Component Variants** - pre-built component variants
//!
//! But implements them through Rust types for native GUI backends.
//!
//! ## Features
//!
//! - **Type-safe** - impossible to specify invalid colors or sizes
//! - **Autocomplete** - IDE suggests all available options
//! - **Compile-time checks** - style errors caught at compile time
//! - **Composable** - styles can be combined and reused
//! - **Multi-backend** - egui, iced, slint support
//!
//! ## Quick Start
//!
//! ```rust
//! use twill::prelude::*;
//!
//! // Create a button style
//! let button_style = Style::new()
//!     .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
//!     .bg(Color::blue(Scale::S500))
//!     .text_color(Color::slate(Scale::S50))
//!     .rounded(BorderRadius::Md);
//! ```
//!
//!
//! ## API Surface
//!
//! The root namespace intentionally stays small.
//! Import day-to-day styling types from [`prelude`], and use module namespaces
//! like [`tokens`], [`utilities`], and [`backends`] for the rest.

pub mod backends;

#[cfg(feature = "egui")]
pub use backends::egui;

#[cfg(feature = "iced")]
pub use backends::iced;

#[cfg(feature = "slint")]
pub use backends::slint;
pub mod style;
pub mod tokens;
pub mod traits;
pub mod utilities;

/// Canonical import surface for day-to-day twill usage.
pub mod prelude {
    pub use crate::style::Style;
    pub use crate::tokens::{
        AnimationToken, BackgroundColor, BackgroundColorVar, BorderRadius, BorderStyle,
        BorderWidth, Breakpoint, Color, ColorFamily, ColorValue, ColorValueToken, Container,
        DivideWidth, DropShadow, DynamicSemanticTheme, Easing, FontFamily, FontSize, FontSizeVar,
        FontWeight, InsetShadow, LetterSpacing, LineHeight, MotionDefaults, OutlineStyle,
        Percentage, Perspective, RingWidth, Scale, SemanticColor, SemanticThemeVars, Shadow,
        Spacing, SpecialColor, TextAlign, TextDecoration, TextOverflow, TextTransform,
        TransitionDuration, TransitionProperty, WhiteSpace, WordBreak,
    };
    pub use crate::traits::{ComputeValue, IntoStyle, Merge, Responsive, ThemedStyle};
    pub use crate::utilities::{
        AlignItems, AlignSelf, Columns, Display, Flex, FlexContainer, FlexDirection, FlexWrap,
        GridContainer, GridTemplate, Height, HeightVar, JustifyContent, Margin, MarginValue,
        MarginVar, Overflow, Padding, PaddingValue, PaddingVar, Position, Size, SizeConstraints,
        Width, WidthVar, ZIndex,
    };
}

// Style
pub use style::Style;

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_basic_style() {
        let style = Style::new()
            .padding(Padding::all(Spacing::S4))
            .bg(Color::blue(Scale::S500))
            .rounded(BorderRadius::Md);

        assert_eq!(style.padding, Some(Padding::all(Spacing::S4)));
        assert_eq!(
            style.background_color,
            Some(BackgroundColor::palette(Color::blue(Scale::S500)))
        );
        assert_eq!(style.border_radius, Some(BorderRadius::Md));
    }

    #[test]
    fn test_flex_layout() {
        let style = Style::centered_col().gap(Spacing::S4);

        let flex = style.flex.expect("flex container should be present");
        assert_eq!(flex.direction, Some(FlexDirection::Col));
        assert_eq!(flex.justify, Some(JustifyContent::Center));
        assert_eq!(flex.align, Some(AlignItems::Center));
        assert_eq!(flex.gap, Some(Spacing::S4));
    }
}
