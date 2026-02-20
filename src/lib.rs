//! # Twill
//!
//! Idiomatic Rust styling library inspired by Tailwind concepts.
//!
//! ## Philosophy
//!
//! Takes the best ideas from Tailwind:
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
//! use twill::{Style, Color, Scale, Spacing, Padding, BorderRadius};
//!
//! // Create a button style
//! let button_style = Style::new()
//!     .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
//!     .bg(Color::blue(Scale::S500))
//!     .text_color(Color::slate(Scale::S50))
//!     .rounded(BorderRadius::Md);
//! ```
//!
//! ## Components
//!
//! ```rust
//! use twill::Button;
//!
//! // Create a primary button
//! let btn = Button::primary().lg();
//!
//! // Create an outline button
//! let btn = Button::outline().sm();
//! ```

pub mod backends;
pub mod components;
pub mod style;
pub mod tokens;
pub mod traits;
pub mod utilities;

// Re-export commonly used types
pub use traits::{ComputeValue, Merge};

// Tokens
pub use tokens::{
    AnimationToken, BorderRadius, BorderStyle, BorderWidth, Breakpoint, Color, ColorFamily,
    ColorValue, Container, DivideWidth, DropShadow, DynamicSemanticTheme, Easing, FontFamily,
    FontSize, FontWeight, InsetShadow, LetterSpacing, LineHeight, MotionDefaults, OutlineStyle,
    Percentage, Perspective, RingWidth, Scale, SemanticColor, SemanticThemeVars, Shadow, Spacing,
    SpecialColor, TextAlign, TextDecoration, TextOverflow, TextShadow as TextShadowToken,
    TextTransform, TransitionDuration, TransitionProperty, WhiteSpace, WordBreak,
};

// Utilities
pub use utilities::{
    AlignItems, AlignSelf, Columns, Display, Flex, FlexContainer, FlexDirection, FlexWrap,
    GridContainer, GridTemplate, Height, JustifyContent, Margin, Overflow, Padding, Position,
    SizeConstraints, Width, ZIndex,
};

// Style
pub use style::Style;

// Components
pub use components::{Button, ButtonSize, ButtonVariant};

// Backends (feature-gated)
#[cfg(feature = "egui")]
pub use backends::egui;
#[cfg(feature = "iced")]
pub use backends::iced;
#[cfg(feature = "slint")]
pub use backends::slint;

// Color shortcuts
pub mod colors {
    pub use crate::tokens::{Color, ColorFamily, Scale, SpecialColor};

    // Common color shortcuts
    pub fn slate(s: Scale) -> Color {
        Color::slate(s)
    }
    pub fn gray(s: Scale) -> Color {
        Color::gray(s)
    }
    pub fn zinc(s: Scale) -> Color {
        Color::zinc(s)
    }
    pub fn neutral(s: Scale) -> Color {
        Color::neutral(s)
    }
    pub fn stone(s: Scale) -> Color {
        Color::stone(s)
    }
    pub fn mauve(s: Scale) -> Color {
        Color::mauve(s)
    }
    pub fn olive(s: Scale) -> Color {
        Color::olive(s)
    }
    pub fn mist(s: Scale) -> Color {
        Color::mist(s)
    }
    pub fn taupe(s: Scale) -> Color {
        Color::taupe(s)
    }
    pub fn red(s: Scale) -> Color {
        Color::red(s)
    }
    pub fn orange(s: Scale) -> Color {
        Color::orange(s)
    }
    pub fn amber(s: Scale) -> Color {
        Color::amber(s)
    }
    pub fn yellow(s: Scale) -> Color {
        Color::yellow(s)
    }
    pub fn lime(s: Scale) -> Color {
        Color::lime(s)
    }
    pub fn green(s: Scale) -> Color {
        Color::green(s)
    }
    pub fn emerald(s: Scale) -> Color {
        Color::emerald(s)
    }
    pub fn teal(s: Scale) -> Color {
        Color::teal(s)
    }
    pub fn cyan(s: Scale) -> Color {
        Color::cyan(s)
    }
    pub fn sky(s: Scale) -> Color {
        Color::sky(s)
    }
    pub fn blue(s: Scale) -> Color {
        Color::blue(s)
    }
    pub fn indigo(s: Scale) -> Color {
        Color::indigo(s)
    }
    pub fn violet(s: Scale) -> Color {
        Color::violet(s)
    }
    pub fn purple(s: Scale) -> Color {
        Color::purple(s)
    }
    pub fn fuchsia(s: Scale) -> Color {
        Color::fuchsia(s)
    }
    pub fn pink(s: Scale) -> Color {
        Color::pink(s)
    }
    pub fn rose(s: Scale) -> Color {
        Color::rose(s)
    }

    pub const TRANSPARENT: SpecialColor = SpecialColor::Transparent;
    pub const CURRENT: SpecialColor = SpecialColor::Current;
    pub const BLACK: SpecialColor = SpecialColor::Black;
    pub const WHITE: SpecialColor = SpecialColor::White;
}

// Spacing shortcuts
pub mod spacing {
    pub use crate::tokens::Spacing;

    pub const PX: Spacing = Spacing::Px;
    pub const S0: Spacing = Spacing::S0;
    pub const S1: Spacing = Spacing::S1;
    pub const S2: Spacing = Spacing::S2;
    pub const S3: Spacing = Spacing::S3;
    pub const S4: Spacing = Spacing::S4;
    pub const S5: Spacing = Spacing::S5;
    pub const S6: Spacing = Spacing::S6;
    pub const S8: Spacing = Spacing::S8;
    pub const S10: Spacing = Spacing::S10;
    pub const S12: Spacing = Spacing::S12;
    pub const S16: Spacing = Spacing::S16;
    pub const S20: Spacing = Spacing::S20;
    pub const S24: Spacing = Spacing::S24;
    pub const S32: Spacing = Spacing::S32;
    pub const AUTO: Spacing = Spacing::Auto;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_style() {
        let style = Style::new()
            .padding(Padding::all(Spacing::S4))
            .bg(Color::blue(Scale::S500))
            .rounded(BorderRadius::Md);

        assert_eq!(style.padding, Some(Padding::all(Spacing::S4)));
        assert_eq!(style.background_color, Some(Color::blue(Scale::S500)));
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

    #[test]
    fn test_button_component() {
        let btn = Button::primary();
        let style = btn.style();
        assert_eq!(style.background_color, Some(Color::blue(Scale::S500)));
    }
}
