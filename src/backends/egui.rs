//! Egui backend for twill.

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, Spacing};
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

    // Shadow (simplified for newer egui API)
    if let Some(_s) = &style.box_shadow {
        frame = frame.shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 8,
            spread: 0,
            color: egui::Color32::from_black_alpha(30),
        });
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
