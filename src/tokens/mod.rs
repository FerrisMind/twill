//! Design tokens module.
//!
//! This module contains all design tokens following Tailwind CSS conventions:
//! - Colors (all 22 color families with 11 scale values each)
//! - Spacing (0-96 with fractional values)
//! - Typography (font sizes, weights, line heights, letter spacing)
//! - Borders (radius, width, style)
//! - Shadows (box shadow, inset shadow, drop shadow, text shadow)

pub mod colors;
pub mod spacing;
pub mod typography;
pub mod borders;
pub mod shadows;

// Re-export commonly used types
pub use colors::{Color, ColorFamily, ColorValue, Scale, SpecialColor};
pub use spacing::{Container, Percentage, Spacing};
pub use typography::{
    FontFamily, FontSize, FontWeight, LetterSpacing, LineHeight,
    TextAlign, TextDecoration, TextOverflow, TextTransform, WhiteSpace, WordBreak,
};
pub use borders::{BorderRadius, BorderStyle, BorderWidth, DivideWidth, OutlineStyle, RingWidth};
pub use shadows::{DropShadow, InsetShadow, Shadow, TextShadow};