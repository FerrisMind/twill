use crate::backends::ShadowColor;
use crate::tokens::{
    AspectRatio, Blur, BorderRadius, Color, ColorValue, Cursor, FontSize, FontWeight,
    SemanticColor, Shadow, Spacing, TextAlign, ThemeVariant, TransitionDuration,
};
use crate::utilities::ObjectFit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

pub fn to_color(color: Color) -> iced::Color {
    super::widgets::to_color(color)
}

pub fn to_color_value(value: ColorValue) -> iced::Color {
    super::widgets::to_color_value(value)
}

pub fn to_padding(spacing: Spacing) -> iced::Padding {
    super::widgets::to_padding(spacing)
}

pub fn to_border_radius(radius: BorderRadius) -> f32 {
    super::widgets::to_border_radius(radius)
}

pub fn to_blur_radius(blur: Blur) -> f32 {
    super::widgets::to_blur_radius(blur)
}

pub fn to_aspect_ratio(ratio: AspectRatio) -> Option<f32> {
    super::widgets::to_aspect_ratio(ratio)
}

pub fn to_content_fit(fit: ObjectFit) -> iced::ContentFit {
    super::widgets::to_content_fit(fit)
}

pub fn to_shadow_with_color(shadow: Shadow, color: ShadowColor) -> iced::Shadow {
    super::widgets::to_shadow_with_color(shadow, color)
}

pub fn to_shadow_layers_with_color(shadow: Shadow, color: ShadowColor) -> Vec<iced::Shadow> {
    super::widgets::to_shadow_layers_with_color(shadow, color)
}

pub fn to_shadow(shadow: Shadow, color: Color) -> iced::Shadow {
    super::widgets::to_shadow(shadow, color)
}

pub fn to_font_size(size: FontSize) -> f32 {
    super::widgets::to_font_size(size)
}

pub fn resolve_font_size(size: FontSize, custom_properties: &[(&str, f32)]) -> Option<f32> {
    super::widgets::resolve_font_size(size, custom_properties)
}

pub fn to_font_weight(weight: FontWeight) -> iced::font::Weight {
    super::widgets::to_font_weight(weight)
}

pub fn to_text_alignment(align: TextAlign) -> iced::widget::text::Alignment {
    super::widgets::to_text_alignment(align)
}

pub fn to_text_alignment_with_direction(
    align: TextAlign,
    direction: TextDirection,
) -> iced::widget::text::Alignment {
    super::widgets::to_text_alignment_with_direction(align, direction)
}

pub fn to_semantic_color(semantic: SemanticColor, variant: ThemeVariant) -> iced::Color {
    super::widgets::to_semantic_color(semantic, variant)
}

pub fn to_duration(duration: TransitionDuration) -> std::time::Duration {
    super::widgets::to_duration(duration)
}

pub fn to_interaction(cursor: Cursor) -> iced::mouse::Interaction {
    super::widgets::to_interaction(cursor)
}
