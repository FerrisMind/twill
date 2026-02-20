use palette::{FromColor, IntoColor, Oklch, Srgb};

/// A utility struct to convert between OKLCH and RGB colors.
pub struct OklchConverter;

impl OklchConverter {
    /// Creates a new OKLCH color and converts it directly to `(u8, u8, u8)`.
    ///
    /// - `l`: Lightness (0.0 to 1.0)
    /// - `c`: Chroma (typically 0.0 to 0.4)
    /// - `h`: Hue (0.0 to 360.0)
    pub fn to_rgb(l: f32, c: f32, h: f32) -> (u8, u8, u8) {
        let oklch = Oklch::new(l, c, h);
        let srgb: Srgb = oklch.into_color();

        // Clamp out-of-gamut channels safely into displayable sRGB.
        let to_u8 = |channel: f32| (channel.clamp(0.0, 1.0) * 255.0).round() as u8;
        (to_u8(srgb.red), to_u8(srgb.green), to_u8(srgb.blue))
    }

    /// Converts RGB bytes into OKLCH `(l, c, h)`.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let oklch: Oklch = Oklch::from_color(srgb);
        let hue = oklch.hue.into_inner();
        let safe_hue = if hue.is_finite() { hue } else { 0.0 };
        (oklch.l, oklch.chroma, safe_hue)
    }

    /// Darkens an OKLCH color by reducing its lightness `l`.
    /// Returns the resulting `(u8, u8, u8)` for direct use in the UI.
    pub fn darken(l: f32, c: f32, h: f32, amount: f32) -> (u8, u8, u8) {
        // Ensure we don't drop below 0.0 lightness
        let new_l = (l - amount).max(0.0);
        Self::to_rgb(new_l, c, h)
    }

    /// Lightens an OKLCH color by increasing its lightness `l`.
    /// Returns the resulting `(u8, u8, u8)` for direct use in the UI.
    pub fn lighten(l: f32, c: f32, h: f32, amount: f32) -> (u8, u8, u8) {
        // Ensure we don't exceed 1.0 lightness
        let new_l = (l + amount).min(1.0);
        Self::to_rgb(new_l, c, h)
    }

    /// Helper to convert a CSS-style OKLCH string if needed (optional)
    /// Example: `oklch(0.6 0.1 250)`
    pub fn from_css_string(css: &str) -> Option<(u8, u8, u8)> {
        // Simplistic parser for demonstration, could be expanded
        let content = css.strip_prefix("oklch(")?.strip_suffix(")")?;
        let parts: Vec<&str> = content.split_whitespace().collect();

        if parts.len() == 3 {
            let l = parts[0].parse::<f32>().ok()?;
            let c = parts[1].parse::<f32>().ok()?;
            let h = parts[2].parse::<f32>().ok()?;
            Some(Self::to_rgb(l, c, h))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oklch_to_rgb() {
        // A standard blue in OKLCH
        let (r, g, b) = OklchConverter::to_rgb(0.5, 0.2, 250.0);
        // Basic sanity: conversion should produce a non-black color here.
        assert_ne!((r, g, b), (0, 0, 0));
    }
}
