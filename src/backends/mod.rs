//! Backend adapters for different UI frameworks.

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
pub use slint::{SlintColors, SlintRadius, SlintSpacing, ToSlint};
