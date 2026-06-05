//! Backend adapters for different UI frameworks.

pub use twill_backend_common::ShadowColor;

#[cfg(feature = "egui")]
pub use twill_egui as egui;

#[cfg(feature = "iced")]
pub use twill_iced as iced;

#[cfg(feature = "slint")]
pub use twill_slint as slint;

#[cfg(feature = "egui")]
pub use egui::ToEgui;

#[cfg(feature = "iced")]
pub use iced::ToIced;

#[cfg(feature = "slint")]
pub use slint::{SlintColors, SlintCursor, SlintRadius, SlintSpacing, ToSlint};
