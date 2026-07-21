use twill_backend_common::ShadowColor;
use twill_core::tokens::{
    AspectRatio, Blur, BorderRadius, Color, ColorValue, Cursor, Easing, FontSize, FontWeight,
    SemanticColor, Shadow, Spacing, TextAlign, ThemeVariant, TransitionDuration,
};
use twill_core::utilities::ObjectFit;

use super::widgets::SemanticThemeSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

pub fn to_color(color: Color) -> iced_core::Color {
    super::widgets::to_color(color)
}

pub fn to_color_value(value: ColorValue) -> iced_core::Color {
    super::widgets::to_color_value(value)
}

pub fn to_padding(spacing: Spacing) -> iced_core::Padding {
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

pub fn to_content_fit(fit: ObjectFit) -> iced_core::ContentFit {
    super::widgets::to_content_fit(fit)
}

pub fn to_shadow_with_color(shadow: Shadow, color: ShadowColor) -> iced_core::Shadow {
    super::widgets::to_shadow_with_color(shadow, color)
}

pub fn to_shadow_layers_with_color(shadow: Shadow, color: ShadowColor) -> Vec<iced_core::Shadow> {
    super::widgets::to_shadow_layers_with_color(shadow, color)
}

pub fn to_shadow(shadow: Shadow, color: Color) -> iced_core::Shadow {
    super::widgets::to_shadow(shadow, color)
}

pub fn to_font_size(size: FontSize) -> f32 {
    super::widgets::to_font_size(size)
}

pub fn resolve_font_size(size: FontSize, custom_properties: &[(&str, f32)]) -> Option<f32> {
    super::widgets::resolve_font_size(size, custom_properties)
}

pub fn to_font_weight(weight: FontWeight) -> iced_core::font::Weight {
    super::widgets::to_font_weight(weight)
}

pub fn to_text_alignment(align: TextAlign) -> iced_core::widget::text::Alignment {
    super::widgets::to_text_alignment(align)
}

pub fn to_text_alignment_with_direction(
    align: TextAlign,
    direction: TextDirection,
) -> iced_core::widget::text::Alignment {
    super::widgets::to_text_alignment_with_direction(align, direction)
}

pub fn to_semantic_color_with_theme<S: SemanticThemeSource + ?Sized>(
    semantic: SemanticColor,
    semantic_theme: &S,
    variant: ThemeVariant,
) -> iced_core::Color {
    super::widgets::to_semantic_color_with_theme(semantic, semantic_theme, variant)
}

pub fn to_semantic_color(semantic: SemanticColor, variant: ThemeVariant) -> iced_core::Color {
    super::widgets::to_semantic_color(semantic, variant)
}

pub fn to_duration(duration: TransitionDuration) -> std::time::Duration {
    super::widgets::to_duration(duration)
}

pub fn to_easing(easing: Easing) -> iced_core::animation::Easing {
    super::widgets::to_easing(easing)
}

pub fn to_interaction(cursor: Cursor) -> iced_core::mouse::Interaction {
    super::widgets::to_interaction(cursor)
}
