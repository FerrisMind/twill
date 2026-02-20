//! Design tokens module.
//!
//! This module contains all design tokens following Tailwind conventions:
//! - Colors (all core families with 11 scale values each)
//! - Spacing (0-96 with fractional values)
//! - Breakpoints and container scales
//! - Typography (font sizes, weights, line heights, letter spacing)
//! - Borders (radius, width, style)
//! - Shadows (box shadow, inset shadow, drop shadow, text shadow)
//! - Perspective and motion primitives

pub mod aspect_ratio;
pub mod blur;
pub mod borders;
pub mod colors;
pub mod cursor;
pub mod motion;
pub mod oklch;
pub mod perspective;
pub mod semantic;
pub mod shadows;
pub mod spacing;
pub mod typography;

// Re-export commonly used types
pub use aspect_ratio::AspectRatio;
pub use blur::Blur;
pub use borders::{BorderRadius, BorderStyle, BorderWidth, DivideWidth, OutlineStyle, RingWidth};
pub use colors::{Color, ColorFamily, ColorValue, Scale, SpecialColor};
pub use cursor::Cursor;
pub use motion::{AnimationToken, Easing, MotionDefaults, TransitionDuration, TransitionProperty};
pub use oklch::OklchConverter;
pub use perspective::Perspective;
pub use semantic::{DynamicSemanticTheme, SemanticColor, SemanticThemeVars};
pub use shadows::{DropShadow, InsetShadow, Shadow, TextShadow};
pub use spacing::{Breakpoint, Container, Percentage, Spacing};
pub use typography::{
    FontFamily, FontSize, FontWeight, LetterSpacing, LineHeight, TextAlign, TextDecoration,
    TextOverflow, TextTransform, WhiteSpace, WordBreak,
};
