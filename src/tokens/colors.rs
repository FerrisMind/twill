//! Color design tokens following Tailwind CSS / shadcn color palette.
//!
//! Provides type-safe color values with exact RGB from shadcn-svelte.

use crate::traits::{ComputeValue, ToCss};

/// Color scale values (50-950).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scale {
    S50,
    S100,
    S200,
    S300,
    S400,
    S500,
    S600,
    S700,
    S800,
    S900,
    S950,
}

impl Scale {
    pub fn value(&self) -> u16 {
        match self {
            Scale::S50 => 50,
            Scale::S100 => 100,
            Scale::S200 => 200,
            Scale::S300 => 300,
            Scale::S400 => 400,
            Scale::S500 => 500,
            Scale::S600 => 600,
            Scale::S700 => 700,
            Scale::S800 => 800,
            Scale::S900 => 900,
            Scale::S950 => 950,
        }
    }
}

/// Tailwind color palette families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorFamily {
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
}

/// A color with its family and scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub family: ColorFamily,
    pub scale: Scale,
}

impl Color {
    pub const fn new(family: ColorFamily, scale: Scale) -> Self {
        Self { family, scale }
    }

    /// White color (alias for slate-50 which is near-white)
    pub const fn white() -> Self {
        Self::slate(Scale::S50)
    }

    /// Black color (alias for slate-950 which is near-black)
    pub const fn black() -> Self {
        Self::slate(Scale::S950)
    }

    pub const fn slate(scale: Scale) -> Self {
        Self::new(ColorFamily::Slate, scale)
    }
    pub const fn gray(scale: Scale) -> Self {
        Self::new(ColorFamily::Gray, scale)
    }
    pub const fn zinc(scale: Scale) -> Self {
        Self::new(ColorFamily::Zinc, scale)
    }
    pub const fn neutral(scale: Scale) -> Self {
        Self::new(ColorFamily::Neutral, scale)
    }
    pub const fn stone(scale: Scale) -> Self {
        Self::new(ColorFamily::Stone, scale)
    }
    pub const fn red(scale: Scale) -> Self {
        Self::new(ColorFamily::Red, scale)
    }
    pub const fn orange(scale: Scale) -> Self {
        Self::new(ColorFamily::Orange, scale)
    }
    pub const fn amber(scale: Scale) -> Self {
        Self::new(ColorFamily::Amber, scale)
    }
    pub const fn yellow(scale: Scale) -> Self {
        Self::new(ColorFamily::Yellow, scale)
    }
    pub const fn lime(scale: Scale) -> Self {
        Self::new(ColorFamily::Lime, scale)
    }
    pub const fn green(scale: Scale) -> Self {
        Self::new(ColorFamily::Green, scale)
    }
    pub const fn emerald(scale: Scale) -> Self {
        Self::new(ColorFamily::Emerald, scale)
    }
    pub const fn teal(scale: Scale) -> Self {
        Self::new(ColorFamily::Teal, scale)
    }
    pub const fn cyan(scale: Scale) -> Self {
        Self::new(ColorFamily::Cyan, scale)
    }
    pub const fn sky(scale: Scale) -> Self {
        Self::new(ColorFamily::Sky, scale)
    }
    pub const fn blue(scale: Scale) -> Self {
        Self::new(ColorFamily::Blue, scale)
    }
    pub const fn indigo(scale: Scale) -> Self {
        Self::new(ColorFamily::Indigo, scale)
    }
    pub const fn violet(scale: Scale) -> Self {
        Self::new(ColorFamily::Violet, scale)
    }
    pub const fn purple(scale: Scale) -> Self {
        Self::new(ColorFamily::Purple, scale)
    }
    pub const fn fuchsia(scale: Scale) -> Self {
        Self::new(ColorFamily::Fuchsia, scale)
    }
    pub const fn pink(scale: Scale) -> Self {
        Self::new(ColorFamily::Pink, scale)
    }
    pub const fn rose(scale: Scale) -> Self {
        Self::new(ColorFamily::Rose, scale)
    }
}

impl ComputeValue for Color {
    type Output = ColorValue;
    fn compute(&self) -> Self::Output {
        ColorValue::from_color(*self)
    }
}

impl ToCss for Color {
    fn to_css(&self) -> String {
        self.compute().to_hex()
    }
}

/// RGBA color value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorValue {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl ColorValue {
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0.0);

    pub const fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::from_rgb(r, g, b))
            }
            _ => None,
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_css_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn from_color(color: Color) -> Self {
        let (r, g, b) = get_palette_rgb(color.family, color.scale);
        Self::from_rgb(r, g, b)
    }
}

impl ComputeValue for ColorValue {
    type Output = ColorValue;
    fn compute(&self) -> Self::Output {
        *self
    }
}

/// Special colors (transparent, current, inherit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialColor {
    Transparent,
    Current,
    Black,
    White,
}

impl ToCss for SpecialColor {
    fn to_css(&self) -> String {
        match self {
            SpecialColor::Transparent => "transparent".to_string(),
            SpecialColor::Current => "currentColor".to_string(),
            SpecialColor::Black => "#000000".to_string(),
            SpecialColor::White => "#ffffff".to_string(),
        }
    }
}

/// Get RGB values for a color family and scale (from shadcn-svelte colors.ts)
fn get_palette_rgb(family: ColorFamily, scale: Scale) -> (u8, u8, u8) {
    match family {
        ColorFamily::Slate => match scale {
            Scale::S50 => (248, 250, 252),
            Scale::S100 => (241, 245, 249),
            Scale::S200 => (226, 232, 240),
            Scale::S300 => (203, 213, 225),
            Scale::S400 => (148, 163, 184),
            Scale::S500 => (100, 116, 139),
            Scale::S600 => (71, 85, 105),
            Scale::S700 => (51, 65, 85),
            Scale::S800 => (30, 41, 59),
            Scale::S900 => (15, 23, 42),
            Scale::S950 => (2, 6, 23),
        },
        ColorFamily::Gray => match scale {
            Scale::S50 => (249, 250, 251),
            Scale::S100 => (243, 244, 246),
            Scale::S200 => (229, 231, 235),
            Scale::S300 => (209, 213, 219),
            Scale::S400 => (156, 163, 175),
            Scale::S500 => (107, 114, 128),
            Scale::S600 => (75, 85, 99),
            Scale::S700 => (55, 65, 81),
            Scale::S800 => (31, 41, 55),
            Scale::S900 => (17, 24, 39),
            Scale::S950 => (3, 7, 18),
        },
        ColorFamily::Zinc => match scale {
            Scale::S50 => (250, 250, 250),
            Scale::S100 => (244, 244, 245),
            Scale::S200 => (228, 228, 231),
            Scale::S300 => (212, 212, 216),
            Scale::S400 => (161, 161, 170),
            Scale::S500 => (113, 113, 122),
            Scale::S600 => (82, 82, 91),
            Scale::S700 => (63, 63, 70),
            Scale::S800 => (39, 39, 42),
            Scale::S900 => (24, 24, 27),
            Scale::S950 => (9, 9, 11),
        },
        ColorFamily::Neutral => match scale {
            Scale::S50 => (250, 250, 250),
            Scale::S100 => (245, 245, 245),
            Scale::S200 => (229, 229, 229),
            Scale::S300 => (212, 212, 212),
            Scale::S400 => (163, 163, 163),
            Scale::S500 => (115, 115, 115),
            Scale::S600 => (82, 82, 82),
            Scale::S700 => (64, 64, 64),
            Scale::S800 => (38, 38, 38),
            Scale::S900 => (23, 23, 23),
            Scale::S950 => (10, 10, 10),
        },
        ColorFamily::Stone => match scale {
            Scale::S50 => (250, 250, 249),
            Scale::S100 => (245, 245, 244),
            Scale::S200 => (231, 229, 228),
            Scale::S300 => (214, 211, 209),
            Scale::S400 => (168, 162, 158),
            Scale::S500 => (120, 113, 108),
            Scale::S600 => (87, 83, 78),
            Scale::S700 => (68, 64, 60),
            Scale::S800 => (41, 37, 36),
            Scale::S900 => (28, 25, 23),
            Scale::S950 => (12, 10, 9),
        },
        ColorFamily::Red => match scale {
            Scale::S50 => (254, 242, 242),
            Scale::S100 => (254, 226, 226),
            Scale::S200 => (254, 202, 202),
            Scale::S300 => (252, 165, 165),
            Scale::S400 => (248, 113, 113),
            Scale::S500 => (239, 68, 68),
            Scale::S600 => (220, 38, 38),
            Scale::S700 => (185, 28, 28),
            Scale::S800 => (153, 27, 27),
            Scale::S900 => (127, 29, 29),
            Scale::S950 => (69, 10, 10),
        },
        ColorFamily::Orange => match scale {
            Scale::S50 => (255, 247, 237),
            Scale::S100 => (255, 237, 213),
            Scale::S200 => (254, 215, 170),
            Scale::S300 => (253, 186, 116),
            Scale::S400 => (251, 146, 60),
            Scale::S500 => (249, 115, 22),
            Scale::S600 => (234, 88, 12),
            Scale::S700 => (194, 65, 12),
            Scale::S800 => (154, 52, 18),
            Scale::S900 => (124, 45, 18),
            Scale::S950 => (67, 20, 7),
        },
        ColorFamily::Amber => match scale {
            Scale::S50 => (255, 251, 235),
            Scale::S100 => (254, 243, 199),
            Scale::S200 => (253, 230, 138),
            Scale::S300 => (252, 211, 77),
            Scale::S400 => (251, 191, 36),
            Scale::S500 => (245, 158, 11),
            Scale::S600 => (217, 119, 6),
            Scale::S700 => (180, 83, 9),
            Scale::S800 => (146, 64, 14),
            Scale::S900 => (120, 53, 15),
            Scale::S950 => (69, 26, 3),
        },
        ColorFamily::Yellow => match scale {
            Scale::S50 => (254, 252, 232),
            Scale::S100 => (254, 249, 195),
            Scale::S200 => (254, 240, 138),
            Scale::S300 => (253, 224, 71),
            Scale::S400 => (250, 204, 21),
            Scale::S500 => (234, 179, 8),
            Scale::S600 => (202, 138, 4),
            Scale::S700 => (161, 98, 7),
            Scale::S800 => (133, 77, 14),
            Scale::S900 => (113, 63, 18),
            Scale::S950 => (66, 32, 6),
        },
        ColorFamily::Lime => match scale {
            Scale::S50 => (247, 254, 231),
            Scale::S100 => (236, 252, 203),
            Scale::S200 => (217, 249, 157),
            Scale::S300 => (190, 242, 100),
            Scale::S400 => (163, 230, 53),
            Scale::S500 => (132, 204, 22),
            Scale::S600 => (101, 163, 13),
            Scale::S700 => (77, 124, 15),
            Scale::S800 => (63, 98, 18),
            Scale::S900 => (54, 83, 20),
            Scale::S950 => (26, 46, 5),
        },
        ColorFamily::Green => match scale {
            Scale::S50 => (240, 253, 244),
            Scale::S100 => (220, 252, 231),
            Scale::S200 => (187, 247, 208),
            Scale::S300 => (134, 239, 172),
            Scale::S400 => (74, 222, 128),
            Scale::S500 => (34, 197, 94),
            Scale::S600 => (22, 163, 74),
            Scale::S700 => (21, 128, 61),
            Scale::S800 => (22, 101, 52),
            Scale::S900 => (20, 83, 45),
            Scale::S950 => (5, 46, 22),
        },
        ColorFamily::Emerald => match scale {
            Scale::S50 => (236, 253, 245),
            Scale::S100 => (209, 250, 229),
            Scale::S200 => (167, 243, 208),
            Scale::S300 => (110, 231, 183),
            Scale::S400 => (52, 211, 153),
            Scale::S500 => (16, 185, 129),
            Scale::S600 => (5, 150, 105),
            Scale::S700 => (4, 120, 87),
            Scale::S800 => (6, 95, 70),
            Scale::S900 => (6, 78, 59),
            Scale::S950 => (2, 44, 34),
        },
        ColorFamily::Teal => match scale {
            Scale::S50 => (240, 253, 250),
            Scale::S100 => (204, 251, 241),
            Scale::S200 => (153, 246, 228),
            Scale::S300 => (94, 234, 212),
            Scale::S400 => (45, 212, 191),
            Scale::S500 => (20, 184, 166),
            Scale::S600 => (13, 148, 136),
            Scale::S700 => (15, 118, 110),
            Scale::S800 => (17, 94, 89),
            Scale::S900 => (19, 78, 74),
            Scale::S950 => (4, 47, 46),
        },
        ColorFamily::Cyan => match scale {
            Scale::S50 => (236, 254, 255),
            Scale::S100 => (207, 250, 254),
            Scale::S200 => (165, 243, 252),
            Scale::S300 => (103, 232, 249),
            Scale::S400 => (34, 211, 238),
            Scale::S500 => (6, 182, 212),
            Scale::S600 => (8, 145, 178),
            Scale::S700 => (14, 116, 144),
            Scale::S800 => (21, 94, 117),
            Scale::S900 => (22, 78, 99),
            Scale::S950 => (8, 51, 68),
        },
        ColorFamily::Sky => match scale {
            Scale::S50 => (240, 249, 255),
            Scale::S100 => (224, 242, 254),
            Scale::S200 => (186, 230, 253),
            Scale::S300 => (125, 211, 252),
            Scale::S400 => (56, 189, 248),
            Scale::S500 => (14, 165, 233),
            Scale::S600 => (2, 132, 199),
            Scale::S700 => (3, 105, 161),
            Scale::S800 => (7, 89, 133),
            Scale::S900 => (12, 74, 110),
            Scale::S950 => (8, 47, 73),
        },
        ColorFamily::Blue => match scale {
            Scale::S50 => (239, 246, 255),
            Scale::S100 => (219, 234, 254),
            Scale::S200 => (191, 219, 254),
            Scale::S300 => (147, 197, 253),
            Scale::S400 => (96, 165, 250),
            Scale::S500 => (59, 130, 246),
            Scale::S600 => (37, 99, 235),
            Scale::S700 => (29, 78, 216),
            Scale::S800 => (30, 64, 175),
            Scale::S900 => (30, 58, 138),
            Scale::S950 => (23, 37, 84),
        },
        ColorFamily::Indigo => match scale {
            Scale::S50 => (238, 242, 255),
            Scale::S100 => (224, 231, 255),
            Scale::S200 => (199, 210, 254),
            Scale::S300 => (165, 180, 252),
            Scale::S400 => (129, 140, 248),
            Scale::S500 => (99, 102, 241),
            Scale::S600 => (79, 70, 229),
            Scale::S700 => (67, 56, 202),
            Scale::S800 => (55, 48, 163),
            Scale::S900 => (49, 46, 129),
            Scale::S950 => (30, 27, 75),
        },
        ColorFamily::Violet => match scale {
            Scale::S50 => (245, 243, 255),
            Scale::S100 => (237, 233, 254),
            Scale::S200 => (221, 214, 254),
            Scale::S300 => (196, 181, 253),
            Scale::S400 => (167, 139, 250),
            Scale::S500 => (139, 92, 246),
            Scale::S600 => (124, 58, 237),
            Scale::S700 => (109, 40, 217),
            Scale::S800 => (91, 33, 182),
            Scale::S900 => (76, 29, 149),
            Scale::S950 => (46, 16, 101),
        },
        ColorFamily::Purple => match scale {
            Scale::S50 => (250, 245, 255),
            Scale::S100 => (243, 232, 255),
            Scale::S200 => (233, 213, 255),
            Scale::S300 => (216, 180, 254),
            Scale::S400 => (192, 132, 252),
            Scale::S500 => (168, 85, 247),
            Scale::S600 => (147, 51, 234),
            Scale::S700 => (126, 34, 206),
            Scale::S800 => (107, 33, 168),
            Scale::S900 => (88, 28, 135),
            Scale::S950 => (59, 7, 100),
        },
        ColorFamily::Fuchsia => match scale {
            Scale::S50 => (253, 244, 255),
            Scale::S100 => (250, 232, 255),
            Scale::S200 => (245, 208, 254),
            Scale::S300 => (240, 171, 252),
            Scale::S400 => (232, 121, 249),
            Scale::S500 => (217, 70, 239),
            Scale::S600 => (192, 38, 211),
            Scale::S700 => (162, 28, 175),
            Scale::S800 => (134, 25, 143),
            Scale::S900 => (112, 26, 117),
            Scale::S950 => (74, 4, 78),
        },
        ColorFamily::Pink => match scale {
            Scale::S50 => (253, 242, 248),
            Scale::S100 => (252, 231, 243),
            Scale::S200 => (251, 207, 232),
            Scale::S300 => (249, 168, 212),
            Scale::S400 => (244, 114, 182),
            Scale::S500 => (236, 72, 153),
            Scale::S600 => (219, 39, 119),
            Scale::S700 => (190, 24, 93),
            Scale::S800 => (157, 23, 77),
            Scale::S900 => (131, 24, 67),
            Scale::S950 => (80, 7, 36),
        },
        ColorFamily::Rose => match scale {
            Scale::S50 => (255, 241, 242),
            Scale::S100 => (255, 228, 230),
            Scale::S200 => (254, 205, 211),
            Scale::S300 => (253, 164, 175),
            Scale::S400 => (251, 113, 133),
            Scale::S500 => (244, 63, 94),
            Scale::S600 => (225, 29, 72),
            Scale::S700 => (190, 18, 60),
            Scale::S800 => (159, 18, 57),
            Scale::S900 => (136, 19, 55),
            Scale::S950 => (76, 5, 25),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let blue = Color::blue(Scale::S500);
        assert_eq!(blue.family, ColorFamily::Blue);
        assert_eq!(blue.scale, Scale::S500);
    }

    #[test]
    fn test_color_value() {
        let blue = Color::blue(Scale::S500);
        let value = blue.compute();
        assert_eq!(value.r, 59);
        assert_eq!(value.g, 130);
        assert_eq!(value.b, 246);
    }

    #[test]
    fn test_hex_conversion() {
        let value = ColorValue::from_rgb(255, 0, 0);
        assert_eq!(value.to_hex(), "#ff0000");
    }
}
