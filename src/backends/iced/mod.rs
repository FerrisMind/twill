mod convert;
mod widgets;

use crate::tokens::{
    AspectRatio, Blur, BorderRadius, Color, ColorValue, Cursor, FontSize, FontWeight, Shadow,
    Spacing, TextAlign, TransitionDuration,
};
use crate::utilities::ObjectFit;
use iced::ContentFit;

pub use convert::{
    TextDirection, resolve_font_size, to_aspect_ratio, to_blur_radius, to_border_radius, to_color,
    to_color_value, to_content_fit, to_duration, to_font_size, to_font_weight, to_interaction,
    to_padding, to_semantic_color, to_shadow, to_shadow_layers_with_color, to_shadow_with_color,
    to_text_alignment, to_text_alignment_with_direction,
};
pub use widgets::{
    align_items_layout, apply_flex_item, apply_flex_item_with_custom_properties, apply_layout,
    apply_layout_with_custom_properties, columns_layout, flex_direction_layout, gap_layout,
    gap_x_layout, gap_y_layout, grid_template_columns_layout,
    grid_template_columns_layout_with_context, justify_content_layout, styled_container,
    styled_container_with_custom_properties,
};

mod private {
    pub trait Sealed {}
}

/// Canonical iced conversion trait for typed twill values.
pub trait ToIced: private::Sealed {
    type Output;

    fn to_iced(self) -> Self::Output;
}

impl private::Sealed for Color {}
impl ToIced for Color {
    type Output = iced::Color;

    fn to_iced(self) -> Self::Output {
        to_color(self)
    }
}

impl private::Sealed for ColorValue {}
impl ToIced for ColorValue {
    type Output = iced::Color;

    fn to_iced(self) -> Self::Output {
        to_color_value(self)
    }
}

impl private::Sealed for Spacing {}
impl ToIced for Spacing {
    type Output = iced::Padding;

    fn to_iced(self) -> Self::Output {
        to_padding(self)
    }
}

impl private::Sealed for BorderRadius {}
impl ToIced for BorderRadius {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_border_radius(self)
    }
}

impl private::Sealed for Blur {}
impl ToIced for Blur {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_blur_radius(self)
    }
}

impl private::Sealed for AspectRatio {}
impl ToIced for AspectRatio {
    type Output = Option<f32>;

    fn to_iced(self) -> Self::Output {
        to_aspect_ratio(self)
    }
}

impl private::Sealed for ObjectFit {}
impl ToIced for ObjectFit {
    type Output = ContentFit;

    fn to_iced(self) -> Self::Output {
        to_content_fit(self)
    }
}

impl private::Sealed for Shadow {}
impl ToIced for Shadow {
    type Output = iced::Shadow;

    fn to_iced(self) -> Self::Output {
        to_shadow_with_color(self, super::ShadowColor::Default)
    }
}

impl private::Sealed for FontSize {}
impl ToIced for FontSize {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_font_size(self)
    }
}

impl private::Sealed for FontWeight {}
impl ToIced for FontWeight {
    type Output = iced::font::Weight;

    fn to_iced(self) -> Self::Output {
        to_font_weight(self)
    }
}

impl private::Sealed for TextAlign {}
impl ToIced for TextAlign {
    type Output = iced::widget::text::Alignment;

    fn to_iced(self) -> Self::Output {
        to_text_alignment(self)
    }
}

impl private::Sealed for TransitionDuration {}
impl ToIced for TransitionDuration {
    type Output = std::time::Duration;

    fn to_iced(self) -> Self::Output {
        to_duration(self)
    }
}

impl private::Sealed for Cursor {}
impl ToIced for Cursor {
    type Output = iced::mouse::Interaction;

    fn to_iced(self) -> Self::Output {
        to_interaction(self)
    }
}
