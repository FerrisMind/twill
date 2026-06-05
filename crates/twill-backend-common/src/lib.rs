#![forbid(unsafe_code)]

//! Shared backend-adapter helper types for the Twill ecosystem.

use twill_core::tokens::Color;

/// Explicit shadow color selection for backend conversion helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ShadowColor {
    #[default]
    Default,
    Explicit(Color),
}

impl From<Color> for ShadowColor {
    fn from(value: Color) -> Self {
        Self::Explicit(value)
    }
}

impl From<Option<Color>> for ShadowColor {
    fn from(value: Option<Color>) -> Self {
        match value {
            Some(color) => Self::Explicit(color),
            None => Self::Default,
        }
    }
}
