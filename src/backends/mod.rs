//! Backend adapters for different UI frameworks.

#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "iced")]
pub mod iced;

#[cfg(feature = "slint")]
pub mod slint;

// Re-export common types
#[cfg(feature = "egui")]
pub use egui::{to_color32, to_color32_value, to_corner_radius, to_frame, to_vec2, twill_button as egui_twill_button};

#[cfg(feature = "iced")]
pub use iced::{to_color, to_color_value, twill_button as iced_twill_button};

#[cfg(feature = "slint")]
pub use slint::{SlintColors, SlintRadius, SlintSpacing, to_length, to_radius, to_shadow_with_color as to_slint_shadow_with_color, to_slint_color};
