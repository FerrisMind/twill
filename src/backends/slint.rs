//! Slint backend for twill.
//!
//! Provides color constants and utilities for Slint UI.

use crate::tokens::{BorderRadius, Color, Scale, Spacing};
use crate::traits::ToCss;

/// Convert twill Color to Slint-compatible hex string.
pub fn to_slint_color(color: Color) -> slint::Color {
    let css = color.to_css();
    let hex = css.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        slint::Color::from_rgb_u8(r, g, b)
    } else {
        slint::Color::from_rgb_u8(255, 255, 255)
    }
}

/// Convert twill Spacing to Slint length (logical pixels).
pub fn to_length(spacing: Spacing) -> f32 {
    spacing.to_rem().unwrap_or(0.0) * 16.0
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
    fn test_spacing() {
        let s4 = to_length(Spacing::S4);
        assert!((s4 - 16.0).abs() < 0.1); // 1rem = 16px
    }
}
