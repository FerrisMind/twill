//! Backend adapters for different UI frameworks.

#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "iced")]
pub mod iced;

#[cfg(feature = "slint")]
pub mod slint;

// Re-export common types
#[cfg(feature = "egui")]
pub use egui::{to_color32, to_corner_radius, to_frame, to_vec2};

#[cfg(feature = "iced")]
pub use iced::to_color;

#[cfg(feature = "slint")]
pub use slint::{SlintColors, SlintRadius, SlintSpacing, to_length, to_radius, to_slint_color};
