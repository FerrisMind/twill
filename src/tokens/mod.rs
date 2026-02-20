//! Design tokens module.
//!
//! This module contains all design tokens following Tailwind CSS conventions:
//! - Colors (all 22 color families with 11 scale values each)
//! - Spacing (0-96 with fractional values)
//! - Typography (font sizes, weights, line heights, letter spacing)
//! - Borders (radius, width, style)
//! - Shadows (box shadow, inset shadow, drop shadow, text shadow)

pub mod aspect_ratio;
pub mod blur;
pub mod borders;
pub mod colors;
pub mod cursor;
pub mod motion;
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
pub use motion::{AnimationToken, Easing, MotionDefaults, TransitionDuration};
pub use semantic::{SemanticColor, SemanticThemeVars};
pub use shadows::{DropShadow, InsetShadow, Shadow, TextShadow};
pub use spacing::{Container, Percentage, Spacing};
pub use typography::{
    FontFamily, FontSize, FontWeight, LetterSpacing, LineHeight, TextAlign, TextDecoration,
    TextOverflow, TextTransform, WhiteSpace, WordBreak,
};
