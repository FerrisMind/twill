//! Slint backend for twill.
//!
//! Provides color constants and utilities for Slint UI.

use crate::tokens::{
    AspectRatio, Blur, BorderRadius, Color, Cursor, FontSize, FontWeight, Scale, SemanticColor,
    SemanticThemeVars, Shadow, Spacing, TransitionDuration,
};
use crate::traits::ComputeValue;

fn spacing_to_px(spacing: Spacing) -> f32 {
    match spacing.to_px() {
        Some(px) => px as f32,
        None => 0.0,
    }
}

/// Convert twill Color to Slint-compatible hex string.
pub fn to_slint_color(color: Color) -> slint::Color {
    let value = color.compute();
    slint::Color::from_argb_u8((value.a * 255.0) as u8, value.r, value.g, value.b)
}

/// Convert twill Spacing to Slint length (logical pixels).
pub fn to_length(spacing: Spacing) -> f32 {
    spacing_to_px(spacing)
}

/// Convert twill BorderRadius to Slint border radius.
pub fn to_radius(radius: BorderRadius) -> f32 {
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

/// Convert twill Blur to Slint blur radius.
pub fn to_blur_radius(blur: Blur) -> f32 {
    blur.radius_px() as f32
}

/// Convert twill Cursor to Slint MouseCursor.
pub fn to_cursor_icon(cursor: Cursor) -> slint::private_unstable_api::re_exports::MouseCursor {
    use slint::private_unstable_api::re_exports::MouseCursor;
    match cursor {
        Cursor::Auto | Cursor::Default => MouseCursor::Default,
        Cursor::Pointer => MouseCursor::Pointer,
        Cursor::Wait => MouseCursor::Wait,
        Cursor::Progress => MouseCursor::Progress,
        Cursor::Text | Cursor::VerticalText => MouseCursor::Text,
        Cursor::Move | Cursor::AllScroll => MouseCursor::Move,
        Cursor::Help => MouseCursor::Help,
        Cursor::NotAllowed => MouseCursor::NotAllowed,
        Cursor::NoDrop => MouseCursor::NoDrop,
        Cursor::None => MouseCursor::None,
        Cursor::ContextMenu => MouseCursor::Default,
        Cursor::Cell => MouseCursor::Default,
        Cursor::Crosshair => MouseCursor::Crosshair,
        Cursor::Alias => MouseCursor::Alias,
        Cursor::Copy => MouseCursor::Copy,
        Cursor::Grab => MouseCursor::Grab,
        Cursor::Grabbing => MouseCursor::Grabbing,
        Cursor::ColResize => MouseCursor::ColResize,
        Cursor::RowResize => MouseCursor::RowResize,
        Cursor::NResize => MouseCursor::NResize,
        Cursor::EResize => MouseCursor::EResize,
        Cursor::SResize => MouseCursor::SResize,
        Cursor::WResize => MouseCursor::WResize,
        Cursor::NeResize => MouseCursor::NeResize,
        Cursor::NwResize => MouseCursor::NwResize,
        Cursor::SeResize => MouseCursor::SeResize,
        Cursor::SwResize => MouseCursor::SwResize,
        Cursor::EwResize => MouseCursor::EwResize,
        Cursor::NsResize => MouseCursor::NsResize,
        Cursor::NeswResize => MouseCursor::NeswResize,
        Cursor::NwseResize => MouseCursor::NwseResize,
        Cursor::ZoomIn => MouseCursor::Default,
        Cursor::ZoomOut => MouseCursor::Default,
    }
}

/// Convert twill AspectRatio to Option<f32>.
pub fn to_aspect_ratio(ratio: AspectRatio) -> Option<f32> {
    match ratio {
        AspectRatio::Auto => None,
        AspectRatio::Square => Some(1.0),
        AspectRatio::Video => Some(16.0 / 9.0),
        AspectRatio::Custom(_, 0) => None,
        AspectRatio::Custom(w, h) => Some(w as f32 / h as f32),
    }
}

/// Convert twill Shadow to Slint shadow values (offset_y, blur_radius).
pub fn to_shadow(shadow: Shadow) -> (f32, f32) {
    match shadow {
        Shadow::None => (0.0, 0.0),
        Shadow::Xs2 => (1.0, 0.0),
        Shadow::Xs => (1.0, 2.0),
        Shadow::Sm => (1.0, 3.0),
        Shadow::Md => (4.0, 6.0),
        Shadow::Lg => (10.0, 15.0),
        Shadow::Xl => (20.0, 25.0),
        Shadow::S2xl => (25.0, 50.0),
    }
}

/// Convert twill Shadow to Slint values with color.
pub fn to_shadow_with_color(shadow: Shadow, color: Option<Color>) -> (f32, f32, slint::Color) {
    let (offset, blur) = to_shadow(shadow);
    let mut value = color.unwrap_or(Color::black()).compute();
    value.a *= match shadow {
        Shadow::None => 0.0,
        Shadow::Xs2 | Shadow::Xs => 0.05,
        Shadow::Sm | Shadow::Md => 0.1,
        Shadow::Lg | Shadow::Xl => 0.26,
        Shadow::S2xl => 0.63,
    };
    (
        offset,
        blur,
        slint::Color::from_argb_u8((value.a * 255.0) as u8, value.r, value.g, value.b),
    )
}

/// Convert twill FontSize to f32.
pub fn to_font_size(size: FontSize) -> f32 {
    size.size_rem() * 16.0
}

/// Convert twill FontWeight to Slint weight integer via Tailwind scale.
pub fn to_font_weight(weight: FontWeight) -> i32 {
    match weight {
        FontWeight::Thin => 100,
        FontWeight::ExtraLight => 200,
        FontWeight::Light => 300,
        FontWeight::Normal => 400,
        FontWeight::Medium => 500,
        FontWeight::SemiBold => 600,
        FontWeight::Bold => 700,
        FontWeight::ExtraBold => 800,
        FontWeight::Black => 900,
    }
}

/// Convert twill SemanticColor to Slint Color based on the theme variant.
pub fn to_semantic_color(semantic: SemanticColor, is_dark: bool) -> slint::Color {
    let color = SemanticThemeVars::shadcn_neutral()
        .resolve(semantic, is_dark)
        .unwrap_or(Color::black());
    to_slint_color(color)
}

/// Convert twill TransitionDuration to i32 milliseconds for Slint.
pub fn to_duration(duration: TransitionDuration) -> i32 {
    duration.as_millis() as i32
}

/// Pre-defined Slint color palette from twill.
pub struct SlintColors;

impl SlintColors {
    pub fn blue_500() -> slint::Color {
        to_slint_color(Color::blue(Scale::S500))
    }
    pub fn blue_600() -> slint::Color {
        to_slint_color(Color::blue(Scale::S600))
    }
    pub fn blue_700() -> slint::Color {
        to_slint_color(Color::blue(Scale::S700))
    }

    pub fn red_500() -> slint::Color {
        to_slint_color(Color::red(Scale::S500))
    }
    pub fn red_600() -> slint::Color {
        to_slint_color(Color::red(Scale::S600))
    }

    pub fn green_500() -> slint::Color {
        to_slint_color(Color::green(Scale::S500))
    }

    pub fn gray_100() -> slint::Color {
        to_slint_color(Color::gray(Scale::S100))
    }
    pub fn gray_200() -> slint::Color {
        to_slint_color(Color::gray(Scale::S200))
    }
    pub fn gray_500() -> slint::Color {
        to_slint_color(Color::gray(Scale::S500))
    }
    pub fn gray_700() -> slint::Color {
        to_slint_color(Color::gray(Scale::S700))
    }
    pub fn gray_900() -> slint::Color {
        to_slint_color(Color::gray(Scale::S900))
    }

    pub fn slate_50() -> slint::Color {
        to_slint_color(Color::slate(Scale::S50))
    }
    pub fn slate_900() -> slint::Color {
        to_slint_color(Color::slate(Scale::S900))
    }

    pub fn white() -> slint::Color {
        slint::Color::from_rgb_u8(255, 255, 255)
    }
    pub fn black() -> slint::Color {
        slint::Color::from_rgb_u8(0, 0, 0)
    }
    pub fn transparent() -> slint::Color {
        slint::Color::from_argb_u8(0, 0, 0, 0)
    }
}

/// Slint spacing values in logical pixels.
pub struct SlintSpacing;

impl SlintSpacing {
    pub fn s1() -> f32 {
        to_length(Spacing::S1)
    }
    pub fn s2() -> f32 {
        to_length(Spacing::S2)
    }
    pub fn s3() -> f32 {
        to_length(Spacing::S3)
    }
    pub fn s4() -> f32 {
        to_length(Spacing::S4)
    }
    pub fn s6() -> f32 {
        to_length(Spacing::S6)
    }
    pub fn s8() -> f32 {
        to_length(Spacing::S8)
    }
    pub fn s12() -> f32 {
        to_length(Spacing::S12)
    }
    pub fn s16() -> f32 {
        to_length(Spacing::S16)
    }
}

/// Slint border radius values.
pub struct SlintRadius;

impl SlintRadius {
    pub fn none() -> f32 {
        0.0
    }
    pub fn sm() -> f32 {
        to_radius(BorderRadius::Sm)
    }
    pub fn md() -> f32 {
        to_radius(BorderRadius::Md)
    }
    pub fn lg() -> f32 {
        to_radius(BorderRadius::Lg)
    }
    pub fn xl() -> f32 {
        to_radius(BorderRadius::Xl)
    }
    pub fn full() -> f32 {
        9999.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let blue = Color::blue(Scale::S500);
        let slint_blue = to_slint_color(blue);
        // Slint Color stores as ARGB
        assert_eq!(slint_blue.red(), 59);
        assert_eq!(slint_blue.green(), 130);
        assert_eq!(slint_blue.blue(), 246);
    }

    #[test]
    fn test_color_conversion_uses_raw_values() {
        let color = Color::blue(Scale::S500);
        let converted = to_slint_color(color);
        let raw = color.compute();
        assert_eq!(converted.red(), raw.r);
        assert_eq!(converted.green(), raw.g);
        assert_eq!(converted.blue(), raw.b);
    }

    #[test]
    fn test_spacing() {
        let s4 = to_length(Spacing::S4);
        assert!((s4 - 16.0).abs() < 0.1); // 1rem = 16px
    }

    #[test]
    fn test_px_spacing() {
        let s = to_length(Spacing::Px);
        assert!((s - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aspect_ratio_zero_denominator() {
        assert_eq!(to_aspect_ratio(AspectRatio::Custom(16, 0)), None);
    }

    #[test]
    fn test_shadow_uses_custom_color() {
        let (_, _, c) = to_shadow_with_color(Shadow::Sm, Some(Color::red(Scale::S500)));
        assert!(c.red() > c.green());
    }
}
