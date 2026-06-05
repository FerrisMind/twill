#![forbid(unsafe_code)]

//! # Twill Core
//!
//! Backend-agnostic style engine for the Twill ecosystem.
//!
//! ## Philosophy
//!
//! Applies utility-first styling ideas:
//! - **Design Tokens** - type-safe base values (colors, spacing, sizes)
//! - **Utility-first** - composable atomic styles
//! - **State & Responsive Layers** - compose hover/focus/data/aria and breakpoint styles
//!
//! But implements them through Rust types for native GUI backends.
//!
//! ## Features
//!
//! - **Type-safe** - impossible to specify invalid colors or sizes
//! - **Autocomplete** - IDE suggests all available options
//! - **Compile-time checks** - style errors caught at compile time
//! - **Composable** - styles can be combined and reused
//! - **Backend-agnostic** - depends only on the core token and style model
//!
//! ## Quick Start
//!
//! ```rust
//! use twill_core::prelude::core::*;
//!
//! // Create a reusable card style
//! let surface_style = Style::card()
//!     .merged(Style::interactive())
//!     .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
//!     .hover(|style| style.opacity(0.9))
//!     .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
//!     .at_md(|style| style.padding(Padding::all(Spacing::S6)));
//! ```
//!
//!
//! ## API Surface
//!
//! The root namespace intentionally stays small.
//! Import day-to-day styling types from [`prelude`], and use module namespaces
//! like [`tokens`] and [`utilities`] for the rest.

pub mod style;
pub mod tokens;
pub mod traits;
pub mod utilities;

/// Canonical import surface for day-to-day twill usage.
pub mod prelude {
    /// Narrow, recommended import set for most applications.
    pub mod core {
        pub use crate::style::{AriaAttr, DataAttr, DataState, Style};
        pub use crate::tokens::{
            BorderRadius, BorderStyle, BorderWidth, Breakpoint, Color, Container, DropShadow,
            Easing, FontFamily, FontSize, FontWeight, InsetShadow, LetterSpacing, LineHeight,
            OutlineStyle, Percentage, Perspective, RingWidth, Scale, Shadow, Spacing, TextAlign,
            TextDecoration, TextTransform, TransitionDuration, TransitionProperty,
        };
        pub use crate::utilities::{
            AlignItems, Columns, Display, FlexContainer, FlexDirection, GridContainer,
            GridTemplate, Height, JustifyContent, Margin, Overflow, Padding, Position,
            SizeConstraints, Width, ZIndex,
        };
    }

    /// Semantic theme tokens and aliases.
    pub mod theme {
        pub use crate::tokens::{
            DynamicSemanticTheme, SemanticColor, SemanticThemeSource, SemanticThemeVars,
            ThemeVariant,
        };
    }

    /// Typed escape hatches for arbitrary values and custom properties.
    pub mod arbitrary {
        pub use crate::tokens::{
            BackgroundColor, BackgroundColorVar, BorderColor, BorderColorVar, ColorFamily,
            ColorValue, ColorValueToken, FontSizeVar, LetterSpacingVar, LineHeightVar,
            OutlineColor, OutlineColorVar, RingColor, RingColorVar, ShadowColorToken,
            ShadowColorVar, SpecialColor, TextColor, TextColorVar,
        };
        pub use crate::utilities::{
            HeightSize, HeightVar, MarginValue, MarginVar, PaddingValue, PaddingVar, Size,
            WidthSize, WidthVar,
        };
    }

    /// Public traits for advanced integration and generic programming.
    pub mod traits {
        pub use crate::traits::{ComputeValue, IntoStyle, Merge, Responsive, ThemedStyle};
    }

    pub use crate::tokens::{
        AnimationToken, DivideWidth, MotionDefaults, TextOverflow, WhiteSpace, WordBreak,
    };
    pub use crate::utilities::{AlignSelf, Flex, FlexWrap};
    pub use arbitrary::*;
    pub use core::*;
    pub use theme::*;
    pub use traits::*;
}

// Style
pub use style::{AriaAttr, DataAttr, DataState, Style};

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
