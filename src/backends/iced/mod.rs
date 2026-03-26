mod convert;
mod widgets;

use crate::tokens::{
    AspectRatio, Blur, BorderRadius, Color, ColorValue, Cursor, FontSize, FontWeight, Shadow,
    Spacing, TextAlign, TransitionDuration,
};
use crate::utilities::ObjectFit;
use iced::ContentFit;

pub use convert::{
    resolve_font_size, to_aspect_ratio, to_blur_radius, to_border_radius, to_color, to_color_value,
    to_content_fit, to_duration, to_font_size, to_font_weight, to_interaction, to_padding,
    to_semantic_color, to_shadow, to_shadow_layers_with_color, to_shadow_with_color,
    to_text_alignment, to_text_alignment_with_direction,
};
pub use widgets::{
    align_items_layout, apply_flex_item, apply_flex_item_with_custom_properties, apply_layout,
    apply_layout_with_custom_properties, columns_layout, flex_direction_layout, gap_layout,
    gap_x_layout, gap_y_layout, grid_template_columns_layout,
    grid_template_columns_layout_with_context, justify_content_layout, styled_container,
    styled_container_with_custom_properties,
};

/// Canonical iced conversion trait for typed twill values.
pub trait ToIced {
    type Output;

    fn to_iced(self) -> Self::Output;
}

impl ToIced for Color {
    type Output = iced::Color;

    fn to_iced(self) -> Self::Output {
        to_color(self)
    }
}

impl ToIced for ColorValue {
    type Output = iced::Color;

    fn to_iced(self) -> Self::Output {
        to_color_value(self)
    }
}

impl ToIced for Spacing {
    type Output = iced::Padding;

    fn to_iced(self) -> Self::Output {
        to_padding(self)
    }
}

impl ToIced for BorderRadius {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_border_radius(self)
    }
}

impl ToIced for Blur {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_blur_radius(self)
    }
}

impl ToIced for AspectRatio {
    type Output = Option<f32>;

    fn to_iced(self) -> Self::Output {
        to_aspect_ratio(self)
    }
}

impl ToIced for ObjectFit {
    type Output = ContentFit;

    fn to_iced(self) -> Self::Output {
        to_content_fit(self)
    }
}

impl ToIced for Shadow {
    type Output = iced::Shadow;

    fn to_iced(self) -> Self::Output {
        to_shadow_with_color(self, None)
    }
}

impl ToIced for FontSize {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_font_size(self)
    }
}

impl ToIced for FontWeight {
    type Output = iced::font::Weight;

    fn to_iced(self) -> Self::Output {
        to_font_weight(self)
    }
}

impl ToIced for TextAlign {
    type Output = iced::widget::text::Alignment;

    fn to_iced(self) -> Self::Output {
        to_text_alignment(self)
    }
}

impl ToIced for TransitionDuration {
    type Output = std::time::Duration;

    fn to_iced(self) -> Self::Output {
        to_duration(self)
    }
}

impl ToIced for Cursor {
    type Output = iced::mouse::Interaction;

    fn to_iced(self) -> Self::Output {
        to_interaction(self)
    }
}
