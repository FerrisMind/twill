//! Egui backend for twill.

use crate::style::Style;
use crate::tokens::{
    AspectRatio, BackgroundColor, Blur, BorderRadius, Color, ColorValue, Cursor, FontSize,
    FontWeight, SemanticColor, SemanticThemeVars, Shadow, Spacing, TransitionDuration,
};
use crate::traits::ComputeValue;
use crate::utilities::PaddingValue;

fn apply_opacity_to_color32(color: egui::Color32, opacity: f32) -> egui::Color32 {
    let alpha = if opacity.is_finite() {
        ((color.a() as f32) * opacity.clamp(0.0, 1.0)).round() as u8
    } else {
        color.a()
    };
    egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha)
}

fn apply_opacity_to_color_value(mut color: ColorValue, opacity: f32) -> ColorValue {
    if opacity.is_finite() {
        color.a *= opacity.clamp(0.0, 1.0);
    }
    color
}

fn resolved_opacity(style: &Style) -> f32 {
    style.opacity.unwrap_or(1.0).clamp(0.0, 1.0)
}

fn spacing_to_px(spacing: Spacing) -> f32 {
    match spacing.to_px() {
        Some(px) => px as f32,
        None => 0.0,
    }
}

fn padding_value_to_px(value: PaddingValue) -> f32 {
    match value {
        PaddingValue::Scale(spacing) => spacing_to_px(spacing),
        PaddingValue::Px(px) => {
            if px.is_finite() {
                px.max(0.0)
            } else {
                0.0
            }
        }
        PaddingValue::Rem(rem) => {
            if rem.is_finite() {
                (rem * 16.0).max(0.0)
            } else {
                0.0
            }
        }
        PaddingValue::Var(_) => 0.0,
    }
}

/// Convert twill Color to egui Color32.
pub fn to_color32(color: Color) -> egui::Color32 {
    to_color32_value(color.compute())
}

/// Convert twill ColorValue to egui Color32.
pub fn to_color32_value(value: crate::tokens::ColorValue) -> egui::Color32 {
    let (r, g, b) = value.to_rgb8();
    egui::Color32::from_rgba_premultiplied(
        (r as f32 * value.a) as u8,
        (g as f32 * value.a) as u8,
        (b as f32 * value.a) as u8,
        (255.0 * value.a) as u8,
    )
}

fn resolve_background_color_token(
    token: BackgroundColor,
    fallback_text: Option<Color>,
) -> Option<ColorValue> {
    match token {
        BackgroundColor::Inherit => None,
        BackgroundColor::Current => fallback_text.map(|text| text.compute()),
        BackgroundColor::Transparent => Some(ColorValue::TRANSPARENT),
        BackgroundColor::Palette(color) => Some(color.compute()),
        BackgroundColor::CustomProperty(_) => None,
        BackgroundColor::Arbitrary(value) => Some(value.into()),
    }
}

/// Convert twill Spacing to egui Vec2 (in points).
pub fn to_vec2(spacing: Spacing) -> egui::Vec2 {
    egui::Vec2::splat(spacing_to_px(spacing))
}

/// Convert twill BorderRadius to egui f32.
pub fn to_corner_radius(radius: BorderRadius) -> f32 {
    match radius {
        BorderRadius::None => 0.0,
        BorderRadius::Xs => 2.0,
        BorderRadius::Sm => 4.0,
        BorderRadius::Md => 6.0,
        BorderRadius::Lg => 8.0,
        BorderRadius::Xl => 12.0,
        BorderRadius::S2xl => 16.0,
        BorderRadius::S3xl => 24.0,
        BorderRadius::S4xl => 32.0,
        BorderRadius::Full => 9999.0,
    }
}

/// Convert twill Blur to egui blur radius (f32).
pub fn to_blur_radius(blur: Blur) -> f32 {
    blur.radius_px() as f32
}

/// Convert twill AspectRatio to Option<f32> for egui.
pub fn to_aspect_ratio(ratio: AspectRatio) -> Option<f32> {
    match ratio {
        AspectRatio::Auto => None,
        AspectRatio::Square => Some(1.0),
        AspectRatio::Video => Some(16.0 / 9.0),
        AspectRatio::Custom(_, 0) => None,
        AspectRatio::Custom(w, h) => Some(w as f32 / h as f32),
    }
}

/// Convert twill Shadow to egui Shadow with an optional color override.
pub fn to_shadow_with_color(
    shadow: Shadow,
    shadow_color: Option<Color>,
) -> Option<egui::epaint::Shadow> {
    let (offset, blur, alpha) = match shadow {
        Shadow::None => return None,
        Shadow::Xs2 => ([0, 1], 0, 0.05),
        Shadow::Xs => ([0, 1], 2, 0.05),
        Shadow::Sm => ([0, 1], 3, 0.1),
        Shadow::Md => ([0, 4], 6, 0.1),
        Shadow::Lg => ([0, 10], 15, 0.1),
        Shadow::Xl => ([0, 20], 25, 0.1),
        Shadow::S2xl => ([0, 25], 50, 0.25),
    };

    let mut value = shadow_color.unwrap_or(Color::black()).compute();
    value.a *= alpha;

    Some(egui::epaint::Shadow {
        offset,
        blur,
        spread: 0,
        color: to_color32_value(value),
    })
}

/// Convert twill Shadow to egui Shadow.
pub fn to_shadow(shadow: Shadow) -> Option<egui::epaint::Shadow> {
    to_shadow_with_color(shadow, None)
}

/// Convert twill FontSize to f32.
pub fn to_font_size(size: FontSize) -> f32 {
    size.size_rem() * 16.0
}

/// Resolve twill FontSize to f32 with explicit custom properties.
pub fn resolve_font_size(size: FontSize, custom_properties: &[(&str, f32)]) -> Option<f32> {
    size.resolve_px(custom_properties)
}

/// Convert twill FontWeight to egui FontFamily (fallback).
pub fn to_font_weight(_weight: FontWeight) -> egui::FontFamily {
    egui::FontFamily::Proportional
}

/// Convert twill SemanticColor to egui Color32 based on the theme variant.
pub fn to_semantic_color32(semantic: SemanticColor, is_dark: bool) -> egui::Color32 {
    let color = SemanticThemeVars::shadcn_neutral()
        .resolve_value(semantic, is_dark)
        .unwrap_or_else(|| Color::black().compute());
    to_color32_value(color)
}

/// Convert twill TransitionDuration to std::time::Duration.
pub fn to_duration(duration: TransitionDuration) -> std::time::Duration {
    std::time::Duration::from_millis(duration.as_millis() as u64)
}

/// Convert twill Cursor to egui CursorIcon.
pub fn to_cursor_icon(cursor: Cursor) -> egui::CursorIcon {
    match cursor {
        Cursor::Auto | Cursor::Default => egui::CursorIcon::Default,
        Cursor::Pointer => egui::CursorIcon::PointingHand,
        Cursor::Wait | Cursor::Progress => egui::CursorIcon::Progress,
        Cursor::Text | Cursor::VerticalText => egui::CursorIcon::Text,
        Cursor::Move => egui::CursorIcon::AllScroll,
        Cursor::Help => egui::CursorIcon::Help,
        Cursor::NotAllowed | Cursor::NoDrop => egui::CursorIcon::NotAllowed,
        Cursor::None => egui::CursorIcon::None,
        Cursor::ContextMenu => egui::CursorIcon::ContextMenu,
        Cursor::Cell => egui::CursorIcon::Cell,
        Cursor::Crosshair => egui::CursorIcon::Crosshair,
        Cursor::Alias => egui::CursorIcon::Alias,
        Cursor::Copy => egui::CursorIcon::Copy,
        Cursor::Grab => egui::CursorIcon::Grab,
        Cursor::Grabbing => egui::CursorIcon::Grabbing,
        Cursor::AllScroll => egui::CursorIcon::AllScroll,
        Cursor::ColResize | Cursor::EwResize => egui::CursorIcon::ResizeColumn,
        Cursor::RowResize | Cursor::NsResize => egui::CursorIcon::ResizeRow,
        Cursor::NResize => egui::CursorIcon::ResizeNorth,
        Cursor::EResize => egui::CursorIcon::ResizeEast,
        Cursor::SResize => egui::CursorIcon::ResizeSouth,
        Cursor::WResize => egui::CursorIcon::ResizeWest,
        Cursor::NeResize => egui::CursorIcon::ResizeNorthEast,
        Cursor::NwResize => egui::CursorIcon::ResizeNorthWest,
        Cursor::SeResize => egui::CursorIcon::ResizeSouthEast,
        Cursor::SwResize => egui::CursorIcon::ResizeSouthWest,
        Cursor::NeswResize => egui::CursorIcon::ResizeNeSw,
        Cursor::NwseResize => egui::CursorIcon::ResizeNwSe,
        Cursor::ZoomIn => egui::CursorIcon::ZoomIn,
        Cursor::ZoomOut => egui::CursorIcon::ZoomOut,
    }
}

/// Create an egui Frame from a twill Style.
///
/// Note: `Style::margin` is intentionally not mapped here because `egui::Frame`
/// only owns inner spacing; outer spacing is controlled by parent layout code.
pub fn to_frame(style: &Style) -> egui::Frame {
    let mut frame = egui::Frame::default();
    let opacity = resolved_opacity(style);

    // Padding
    if let Some(p) = &style.padding {
        let clamp_i8 = |v: f32| v.clamp(i8::MIN as f32, i8::MAX as f32) as i8;
        let top = clamp_i8(p.top.map(padding_value_to_px).unwrap_or(0.0));
        let right = clamp_i8(p.right.map(padding_value_to_px).unwrap_or(0.0));
        let bottom = clamp_i8(p.bottom.map(padding_value_to_px).unwrap_or(0.0));
        let left = clamp_i8(p.left.map(padding_value_to_px).unwrap_or(0.0));
        frame = frame.inner_margin(egui::Margin {
            top,
            left,
            right,
            bottom,
        });
    }

    // Background
    if let Some(bg) = style
        .background_color
        .and_then(|bg| resolve_background_color_token(bg, style.text_color))
    {
        frame = frame.fill(to_color32_value(apply_opacity_to_color_value(bg, opacity)));
    }

    // Border radius
    if let Some(r) = &style.border_radius {
        frame = frame.corner_radius(to_corner_radius(*r));
    }

    // Border
    if let (Some(width), Some(color)) = (&style.border_width, &style.border_color) {
        let w = match width {
            crate::tokens::BorderWidth::S0 => 0.0,
            crate::tokens::BorderWidth::S1 => 1.0,
            crate::tokens::BorderWidth::S2 => 2.0,
            crate::tokens::BorderWidth::S4 => 4.0,
            crate::tokens::BorderWidth::S8 => 8.0,
        };
        frame = frame.stroke(egui::Stroke::new(
            w,
            apply_opacity_to_color32(to_color32(*color), opacity),
        ));
    }

    // Shadow
    if let Some(s) = &style.box_shadow
        && let Some(mut egui_shadow) = to_shadow_with_color(*s, style.shadow_color)
    {
        egui_shadow.color = apply_opacity_to_color32(egui_shadow.color, opacity);
        frame = frame.shadow(egui_shadow);
    }

    frame
}

/// Render an `egui` button directly from `twill::Button`.
pub fn twill_button(
    ui: &mut egui::Ui,
    button: &crate::components::Button,
    label: &str,
) -> egui::Response {
    let style = button.style();
    let opacity = resolved_opacity(&style);

    let text_color = style
        .text_color
        .map(to_color32)
        .map(|color| apply_opacity_to_color32(color, opacity))
        .unwrap_or(egui::Color32::WHITE);
    let mut text = egui::RichText::new(label).color(text_color);
    if let Some(size) = style.font_size
        && let Some(px) = resolve_font_size(size, &[])
    {
        text = text.size(px);
    }
    if let Some(weight) = style.font_weight
        && weight.value() >= 700
    {
        text = text.strong();
    }

    let mut btn = egui::Button::new(text);
    if let Some(bg) = style
        .background_color
        .and_then(|bg| resolve_background_color_token(bg, style.text_color))
    {
        btn = btn.fill(to_color32_value(apply_opacity_to_color_value(bg, opacity)));
    }
    if let Some(radius) = style.border_radius {
        btn = btn.corner_radius(to_corner_radius(radius));
    }

    if let (Some(width), Some(color)) = (style.border_width, style.border_color) {
        let stroke_width = match width {
            crate::tokens::BorderWidth::S0 => 0.0,
            crate::tokens::BorderWidth::S1 => 1.0,
            crate::tokens::BorderWidth::S2 => 2.0,
            crate::tokens::BorderWidth::S4 => 4.0,
            crate::tokens::BorderWidth::S8 => 8.0,
        };
        btn = btn.stroke(egui::Stroke::new(
            stroke_width,
            apply_opacity_to_color32(to_color32(color), opacity),
        ));
    }

    if button.disabled {
        ui.add_enabled(false, btn)
    } else {
        ui.add(btn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::Scale;

    #[test]
    fn test_color_conversion() {
        let blue = Color::blue(Scale::S500);
        let c32 = to_color32(blue);
        let raw = blue.compute();
        let (r, g, b) = raw.to_rgb8();
        assert_eq!(c32.r(), r);
        assert_eq!(c32.g(), g);
        assert_eq!(c32.b(), b);
    }

    #[test]
    fn test_spacing_conversion() {
        let s4 = to_vec2(Spacing::S4);
        assert!(s4.x > 0.0);
    }

    #[test]
    fn test_color_conversion_uses_raw_values() {
        let color = Color::blue(Scale::S500);
        let converted = to_color32(color);
        let raw = color.compute();
        let (r, g, b) = raw.to_rgb8();
        assert_eq!(converted.r(), r);
        assert_eq!(converted.g(), g);
        assert_eq!(converted.b(), b);
    }

    #[test]
    fn test_spacing_px_conversion() {
        let px = to_vec2(Spacing::Px);
        assert!((px.x - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aspect_ratio_zero_denominator() {
        assert_eq!(to_aspect_ratio(AspectRatio::Custom(16, 0)), None);
    }

    #[test]
    fn test_shadow_uses_custom_color() {
        let shadow = to_shadow_with_color(Shadow::Sm, Some(Color::red(Scale::S500))).unwrap();
        assert!(shadow.color.r() > shadow.color.g());
    }
}
