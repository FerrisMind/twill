//! Backend adapters for different UI frameworks.

use crate::tokens::Color;

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

#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "iced")]
pub mod iced;

#[cfg(feature = "slint")]
pub mod slint;

#[cfg(feature = "egui")]
pub use egui::ToEgui;

#[cfg(feature = "iced")]
pub use iced::ToIced;

#[cfg(feature = "slint")]
pub use slint::{SlintColors, SlintCursor, SlintRadius, SlintSpacing, ToSlint};
