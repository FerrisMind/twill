//! Egui backend for twill.

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, Spacing, Cursor, Blur, AspectRatio, Shadow, FontSize, FontWeight, TransitionDuration, SemanticColor, SemanticThemeVars};
use crate::traits::ToCss;

/// Convert twill Color to egui Color32.
pub fn to_color32(color: Color) -> egui::Color32 {
    let css = color.to_css();
    let hex = css.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        egui::Color32::from_rgb(r, g, b)
    } else {
        egui::Color32::WHITE
    }
}

/// Convert twill ColorValue to egui Color32.
pub fn to_color32_value(value: crate::tokens::ColorValue) -> egui::Color32 {
    egui::Color32::from_rgba_premultiplied(
        (value.r as f32 * value.a) as u8,
        (value.g as f32 * value.a) as u8,
        (value.b as f32 * value.a) as u8,
        (255.0 * value.a) as u8,
    )
}

/// Convert twill Spacing to egui Vec2 (in points).
pub fn to_vec2(spacing: Spacing) -> egui::Vec2 {
    let rem = spacing.to_rem().unwrap_or(0.0);
    let px = rem * 16.0;
    egui::Vec2::splat(px)
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
    match blur {
        Blur::None => 0.0,
        Blur::Sm => 4.0,
        Blur::Base => 8.0,
        Blur::Md => 12.0,
        Blur::Lg => 16.0,
        Blur::Xl => 24.0,
        Blur::S2xl => 40.0,
        Blur::S3xl => 64.0,
        Blur::Custom(px) => px as f32,
    }
}

/// Convert twill AspectRatio to Option<f32> for egui.
pub fn to_aspect_ratio(ratio: AspectRatio) -> Option<f32> {
    match ratio {
        AspectRatio::Auto => None,
        AspectRatio::Square => Some(1.0),
        AspectRatio::Video => Some(16.0 / 9.0),
        AspectRatio::Custom(w, h) => Some(w as f32 / h as f32),
    }
}

/// Convert twill Shadow to egui Shadow.
pub fn to_shadow(shadow: Shadow) -> Option<egui::epaint::Shadow> {
    match shadow {
        Shadow::None => None,
        Shadow::Xs2 => Some(egui::epaint::Shadow { offset: [0, 1], blur: 0, spread: 0, color: egui::Color32::from_black_alpha(13) }),
        Shadow::Xs => Some(egui::epaint::Shadow { offset: [0, 1], blur: 2, spread: 0, color: egui::Color32::from_black_alpha(13) }),
        Shadow::Sm => Some(egui::epaint::Shadow { offset: [0, 1], blur: 3, spread: 0, color: egui::Color32::from_black_alpha(26) }),
        Shadow::Md => Some(egui::epaint::Shadow { offset: [0, 4], blur: 6, spread: 0, color: egui::Color32::from_black_alpha(26) }),
        Shadow::Lg => Some(egui::epaint::Shadow { offset: [0, 10], blur: 15, spread: 0, color: egui::Color32::from_black_alpha(26) }),
        Shadow::Xl => Some(egui::epaint::Shadow { offset: [0, 20], blur: 25, spread: 0, color: egui::Color32::from_black_alpha(26) }),
        Shadow::S2xl => Some(egui::epaint::Shadow { offset: [0, 25], blur: 50, spread: 0, color: egui::Color32::from_black_alpha(63) }),
    }
}

/// Convert twill FontSize to f32.
pub fn to_font_size(size: FontSize) -> f32 {
    size.size_rem() * 16.0
}

/// Convert twill FontWeight to egui FontFamily (fallback).
pub fn to_font_weight(_weight: FontWeight) -> egui::FontFamily {
    egui::FontFamily::Proportional
}

/// Convert twill SemanticColor to egui Color32 based on the theme variant.
pub fn to_semantic_color32(semantic: SemanticColor, is_dark: bool) -> egui::Color32 {
    let color = SemanticThemeVars::shadcn_neutral().resolve(semantic, is_dark).unwrap_or(Color::black());
    to_color32(color)
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
pub fn to_frame(style: &Style) -> egui::Frame {
    let mut frame = egui::Frame::default();

    // Padding
    if let Some(p) = &style.padding {
        let top = p
            .top
            .map(|s| s.to_rem().unwrap_or(0.0) * 16.0)
            .unwrap_or(0.0) as i8;
        let right = p
            .right
            .map(|s| s.to_rem().unwrap_or(0.0) * 16.0)
            .unwrap_or(0.0) as i8;
        let bottom = p
            .bottom
            .map(|s| s.to_rem().unwrap_or(0.0) * 16.0)
            .unwrap_or(0.0) as i8;
        let left = p
            .left
            .map(|s| s.to_rem().unwrap_or(0.0) * 16.0)
            .unwrap_or(0.0) as i8;
        frame = frame.inner_margin(egui::Margin {
            top,
            left,
            right,
            bottom,
        });
    }

    // Background
    if let Some(bg) = &style.background_color {
        frame = frame.fill(to_color32(*bg));
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
        frame = frame.stroke(egui::Stroke::new(w, to_color32(*color)));
    }

    // Shadow
    if let Some(s) = &style.box_shadow
        && let Some(egui_shadow) = to_shadow(*s)
    {
        frame = frame.shadow(egui_shadow);
    }

    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::Scale;

    #[test]
    fn test_color_conversion() {
        let blue = Color::blue(Scale::S500);
        let c32 = to_color32(blue);
        assert_eq!(c32.r(), 59);
        assert_eq!(c32.g(), 130);
        assert_eq!(c32.b(), 246);
    }

    #[test]
    fn test_spacing_conversion() {
        let s4 = to_vec2(Spacing::S4);
        assert!(s4.x > 0.0);
    }
}
