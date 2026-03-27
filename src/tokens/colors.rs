//! Color design tokens following curated palette families.
//!
//! Palette resolution is performed in OKLCH and converted for iced-compatible RGBA output.

use std::fmt;

use crate::traits::ComputeValue;

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
    pub const ALL: [Scale; 11] = [
        Scale::S50,
        Scale::S100,
        Scale::S200,
        Scale::S300,
        Scale::S400,
        Scale::S500,
        Scale::S600,
        Scale::S700,
        Scale::S800,
        Scale::S900,
        Scale::S950,
    ];

    pub const fn value(&self) -> u16 {
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

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Color palette families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorFamily {
    Black,
    White,
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
    Mauve,
    Olive,
    Mist,
    Taupe,
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

impl ColorFamily {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ColorFamily::Black => "black",
            ColorFamily::White => "white",
            ColorFamily::Slate => "slate",
            ColorFamily::Gray => "gray",
            ColorFamily::Zinc => "zinc",
            ColorFamily::Neutral => "neutral",
            ColorFamily::Stone => "stone",
            ColorFamily::Mauve => "mauve",
            ColorFamily::Olive => "olive",
            ColorFamily::Mist => "mist",
            ColorFamily::Taupe => "taupe",
            ColorFamily::Red => "red",
            ColorFamily::Orange => "orange",
            ColorFamily::Amber => "amber",
            ColorFamily::Yellow => "yellow",
            ColorFamily::Lime => "lime",
            ColorFamily::Green => "green",
            ColorFamily::Emerald => "emerald",
            ColorFamily::Teal => "teal",
            ColorFamily::Cyan => "cyan",
            ColorFamily::Sky => "sky",
            ColorFamily::Blue => "blue",
            ColorFamily::Indigo => "indigo",
            ColorFamily::Violet => "violet",
            ColorFamily::Purple => "purple",
            ColorFamily::Fuchsia => "fuchsia",
            ColorFamily::Pink => "pink",
            ColorFamily::Rose => "rose",
        }
    }
}

impl fmt::Display for ColorFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A color with its family and scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub(crate) family: ColorFamily,
    pub(crate) scale: Scale,
}

impl Color {
    pub const fn new(family: ColorFamily, scale: Scale) -> Self {
        Self { family, scale }
    }

    pub const fn family(&self) -> ColorFamily {
        self.family
    }

    pub const fn scale(&self) -> Scale {
        self.scale
    }

    /// White color (`--color-white`).
    pub const fn white() -> Self {
        Self::new(ColorFamily::White, Scale::S500)
    }

    /// Black color (`--color-black`).
    pub const fn black() -> Self {
        Self::new(ColorFamily::Black, Scale::S500)
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
    pub const fn mauve(scale: Scale) -> Self {
        Self::new(ColorFamily::Mauve, scale)
    }
    pub const fn olive(scale: Scale) -> Self {
        Self::new(ColorFamily::Olive, scale)
    }
    pub const fn mist(scale: Scale) -> Self {
        Self::new(ColorFamily::Mist, scale)
    }
    pub const fn taupe(scale: Scale) -> Self {
        Self::new(ColorFamily::Taupe, scale)
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

/// OKLCH color value with alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorValue {
    pub(crate) l: f32,
    pub(crate) c: f32,
    pub(crate) h: f32,
    pub(crate) a: f32,
}

impl ColorValue {
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(l: f32, c: f32, h: f32, a: f32) -> Self {
        Self { l, c, h, a }
    }

    pub const fn lightness(&self) -> f32 {
        self.l
    }

    pub const fn chroma(&self) -> f32 {
        self.c
    }

    pub const fn hue(&self) -> f32 {
        self.h
    }

    pub const fn alpha(&self) -> f32 {
        self.a
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let (l, c, h) = crate::tokens::oklch::OklchConverter::from_rgb(r, g, b);
        Self::new(l, c, h, 1.0)
    }

    /// Creates a ColorValue from OKLCH parameters
    pub const fn from_oklch(l: f32, c: f32, h: f32) -> Self {
        Self::new(l, c, h, 1.0)
    }

    fn oklch_components(&self) -> (f32, f32, f32) {
        (self.l, self.c, self.h)
    }

    /// Converts the stored OKLCH value into displayable sRGB bytes.
    pub fn to_rgb8(&self) -> (u8, u8, u8) {
        crate::tokens::oklch::OklchConverter::to_rgb(self.l, self.c, self.h)
    }

    /// Converts the stored OKLCH value into displayable RGBA bytes.
    pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
        let (r, g, b) = self.to_rgb8();
        let alpha = (self.a.clamp(0.0, 1.0) * 255.0).round() as u8;
        (r, g, b, alpha)
    }

    /// Darkens the current color based on an OKLCH lightness transformation
    pub fn darken_oklch(&self, amount: f32) -> Self {
        let (l, c, h) = self.oklch_components();
        let new_l = (l - amount).max(0.0);
        Self::new(new_l, c, h, self.a)
    }

    /// Lightens the current color based on an OKLCH lightness transformation
    pub fn lighten_oklch(&self, amount: f32) -> Self {
        let (l, c, h) = self.oklch_components();
        let new_l = (l + amount).min(1.0);
        Self::new(new_l, c, h, self.a)
    }

    /// Rebuilds the color with the same chroma/hue and a new OKLCH lightness.
    pub fn with_oklch_lightness(&self, lightness: f32) -> Self {
        let (_, c, h) = self.oklch_components();
        Self::new(lightness, c, h, self.a)
    }

    /// OKLCH perceptual lightness (`L`, 0.0..1.0).
    pub fn perceived_lightness_oklch(&self) -> f32 {
        let (l, _, _) = self.oklch_components();
        l
    }

    /// Fast black/white contrast pick based on perceptual OKLCH lightness.
    pub fn preferred_text_color(&self) -> SpecialColor {
        if self.perceived_lightness_oklch() > 0.6 {
            SpecialColor::Black
        } else {
            SpecialColor::White
        }
    }

    /// Generates an 11-step scale (50..950) preserving hue/chroma.
    pub fn generate_scale_oklch(&self) -> [ColorValue; 11] {
        // Ordered for Scale::ALL = [50, 100, ..., 950]
        let lightness_steps = [
            0.985, 0.955, 0.91, 0.84, 0.74, 0.64, 0.54, 0.44, 0.34, 0.26, 0.18,
        ];
        let (_, c, h) = self.oklch_components();

        let mut out = [ColorValue::TRANSPARENT; 11];
        for (idx, l) in lightness_steps.iter().enumerate() {
            out[idx] = ColorValue::new(*l, c, h, self.a);
        }
        out
    }

    /// Generates a scale paired with semantic `Scale` keys.
    pub fn generate_scale_map_oklch(&self) -> [(Scale, ColorValue); 11] {
        let values = self.generate_scale_oklch();
        [
            (Scale::S50, values[0]),
            (Scale::S100, values[1]),
            (Scale::S200, values[2]),
            (Scale::S300, values[3]),
            (Scale::S400, values[4]),
            (Scale::S500, values[5]),
            (Scale::S600, values[6]),
            (Scale::S700, values[7]),
            (Scale::S800, values[8]),
            (Scale::S900, values[9]),
            (Scale::S950, values[10]),
        ]
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// Returns this color in typed OKLCH components for palette/iced workflows.
    pub fn to_oklch(&self) -> (f32, f32, f32) {
        self.oklch_components()
    }

    pub fn from_color(color: Color) -> Self {
        let (l, c, h) = get_palette_oklch(color.family, color.scale);
        Self::from_oklch(l, c, h)
    }
}

/// Typed background-color token family (`bg-*` in Tailwind reference semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackgroundColor {
    Inherit,
    Current,
    Transparent,
    Palette(Color),
    CustomProperty(BackgroundColorVar),
    Arbitrary(ColorValueToken),
}

impl BackgroundColor {
    pub const fn inherit() -> Self {
        Self::Inherit
    }

    pub const fn current() -> Self {
        Self::Current
    }

    pub const fn transparent() -> Self {
        Self::Transparent
    }

    pub const fn palette(color: Color) -> Self {
        Self::Palette(color)
    }

    pub const fn custom_property(var: BackgroundColorVar) -> Self {
        Self::CustomProperty(var)
    }

    pub const fn arbitrary(value: ColorValueToken) -> Self {
        Self::Arbitrary(value)
    }
}

impl From<Color> for BackgroundColor {
    fn from(value: Color) -> Self {
        Self::Palette(value)
    }
}

/// Typed CSS custom-property reference for background-color tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackgroundColorVar(&'static str);

impl BackgroundColorVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for BackgroundColorVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for BackgroundColorVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for BackgroundColorVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Typed arbitrary literal value for background-color (`bg-[<value>]` equivalent).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorValueToken(u8, u8, u8, u8);

impl ColorValueToken {
    pub const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b, 255)
    }

    pub const fn red(self) -> u8 {
        self.0
    }

    pub const fn green(self) -> u8 {
        self.1
    }

    pub const fn blue(self) -> u8 {
        self.2
    }

    pub const fn alpha(self) -> u8 {
        self.3
    }

    pub const fn to_rgba8(self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }
}

impl From<ColorValue> for ColorValueToken {
    fn from(value: ColorValue) -> Self {
        let (r, g, b, a) = value.to_rgba8();
        Self(r, g, b, a)
    }
}

impl From<ColorValueToken> for ColorValue {
    fn from(value: ColorValueToken) -> Self {
        ColorValue::from_rgb(value.0, value.1, value.2).with_alpha(value.3 as f32 / 255.0)
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

impl SpecialColor {
    pub fn value(&self) -> &'static str {
        match self {
            SpecialColor::Transparent => "transparent",
            SpecialColor::Current => "currentColor",
            SpecialColor::Black => "oklch(0.000 0.000 0.000)",
            SpecialColor::White => "oklch(1.000 0.000 0.000)",
        }
    }
}

/// Resolve an OKLCH palette value for a family and scale.
fn get_palette_oklch(family: ColorFamily, scale: Scale) -> (f32, f32, f32) {
    match family {
        ColorFamily::Black => (0.0, 0.0, 0.0),
        ColorFamily::White => (1.0, 0.0, 0.0),
        ColorFamily::Slate => match scale {
            Scale::S50 => (0.984, 0.003, 247.858),
            Scale::S100 => (0.968, 0.007, 247.896),
            Scale::S200 => (0.929, 0.013, 255.508),
            Scale::S300 => (0.869, 0.022, 252.894),
            Scale::S400 => (0.704, 0.04, 256.788),
            Scale::S500 => (0.554, 0.046, 257.417),
            Scale::S600 => (0.446, 0.043, 257.281),
            Scale::S700 => (0.372, 0.044, 257.287),
            Scale::S800 => (0.279, 0.041, 260.031),
            Scale::S900 => (0.208, 0.042, 265.755),
            Scale::S950 => (0.129, 0.042, 264.695),
        },
        ColorFamily::Gray => match scale {
            Scale::S50 => (0.985, 0.002, 247.839),
            Scale::S100 => (0.967, 0.003, 264.542),
            Scale::S200 => (0.928, 0.006, 264.531),
            Scale::S300 => (0.872, 0.01, 258.338),
            Scale::S400 => (0.707, 0.022, 261.325),
            Scale::S500 => (0.551, 0.027, 264.364),
            Scale::S600 => (0.446, 0.03, 256.802),
            Scale::S700 => (0.373, 0.034, 259.733),
            Scale::S800 => (0.278, 0.033, 256.848),
            Scale::S900 => (0.21, 0.034, 264.665),
            Scale::S950 => (0.13, 0.028, 261.692),
        },
        ColorFamily::Zinc => match scale {
            Scale::S50 => (0.985, 0.0, 0.0),
            Scale::S100 => (0.967, 0.001, 286.375),
            Scale::S200 => (0.92, 0.004, 286.32),
            Scale::S300 => (0.871, 0.006, 286.286),
            Scale::S400 => (0.705, 0.015, 286.067),
            Scale::S500 => (0.552, 0.016, 285.938),
            Scale::S600 => (0.442, 0.017, 285.786),
            Scale::S700 => (0.37, 0.013, 285.805),
            Scale::S800 => (0.274, 0.006, 286.033),
            Scale::S900 => (0.21, 0.006, 285.885),
            Scale::S950 => (0.141, 0.005, 285.823),
        },
        ColorFamily::Neutral => match scale {
            Scale::S50 => (0.985, 0.0, 0.0),
            Scale::S100 => (0.97, 0.0, 0.0),
            Scale::S200 => (0.922, 0.0, 0.0),
            Scale::S300 => (0.87, 0.0, 0.0),
            Scale::S400 => (0.708, 0.0, 0.0),
            Scale::S500 => (0.556, 0.0, 0.0),
            Scale::S600 => (0.439, 0.0, 0.0),
            Scale::S700 => (0.371, 0.0, 0.0),
            Scale::S800 => (0.269, 0.0, 0.0),
            Scale::S900 => (0.205, 0.0, 0.0),
            Scale::S950 => (0.145, 0.0, 0.0),
        },
        ColorFamily::Stone => match scale {
            Scale::S50 => (0.985, 0.001, 106.423),
            Scale::S100 => (0.97, 0.001, 106.424),
            Scale::S200 => (0.923, 0.003, 48.717),
            Scale::S300 => (0.869, 0.005, 56.366),
            Scale::S400 => (0.709, 0.01, 56.259),
            Scale::S500 => (0.553, 0.013, 58.071),
            Scale::S600 => (0.444, 0.011, 73.639),
            Scale::S700 => (0.374, 0.01, 67.558),
            Scale::S800 => (0.268, 0.007, 34.298),
            Scale::S900 => (0.216, 0.006, 56.043),
            Scale::S950 => (0.147, 0.004, 49.25),
        },
        ColorFamily::Red => match scale {
            Scale::S50 => (0.971, 0.013, 17.38),
            Scale::S100 => (0.936, 0.032, 17.717),
            Scale::S200 => (0.885, 0.062, 18.334),
            Scale::S300 => (0.808, 0.114, 19.571),
            Scale::S400 => (0.704, 0.191, 22.216),
            Scale::S500 => (0.637, 0.237, 25.331),
            Scale::S600 => (0.577, 0.245, 27.325),
            Scale::S700 => (0.505, 0.213, 27.518),
            Scale::S800 => (0.444, 0.177, 26.899),
            Scale::S900 => (0.396, 0.141, 25.723),
            Scale::S950 => (0.258, 0.092, 26.042),
        },
        ColorFamily::Orange => match scale {
            Scale::S50 => (0.98, 0.016, 73.684),
            Scale::S100 => (0.954, 0.038, 75.164),
            Scale::S200 => (0.901, 0.076, 70.697),
            Scale::S300 => (0.837, 0.128, 66.29),
            Scale::S400 => (0.75, 0.183, 55.934),
            Scale::S500 => (0.705, 0.213, 47.604),
            Scale::S600 => (0.646, 0.222, 41.116),
            Scale::S700 => (0.553, 0.195, 38.402),
            Scale::S800 => (0.47, 0.157, 37.304),
            Scale::S900 => (0.408, 0.123, 38.172),
            Scale::S950 => (0.266, 0.079, 36.259),
        },
        ColorFamily::Amber => match scale {
            Scale::S50 => (0.987, 0.022, 95.277),
            Scale::S100 => (0.962, 0.059, 95.617),
            Scale::S200 => (0.924, 0.12, 95.746),
            Scale::S300 => (0.879, 0.169, 91.605),
            Scale::S400 => (0.828, 0.189, 84.429),
            Scale::S500 => (0.769, 0.188, 70.08),
            Scale::S600 => (0.666, 0.179, 58.318),
            Scale::S700 => (0.555, 0.163, 48.998),
            Scale::S800 => (0.473, 0.137, 46.201),
            Scale::S900 => (0.414, 0.112, 45.904),
            Scale::S950 => (0.279, 0.077, 45.635),
        },
        ColorFamily::Yellow => match scale {
            Scale::S50 => (0.987, 0.026, 102.212),
            Scale::S100 => (0.973, 0.071, 103.193),
            Scale::S200 => (0.945, 0.129, 101.54),
            Scale::S300 => (0.905, 0.182, 98.111),
            Scale::S400 => (0.852, 0.199, 91.936),
            Scale::S500 => (0.795, 0.184, 86.047),
            Scale::S600 => (0.681, 0.162, 75.834),
            Scale::S700 => (0.554, 0.135, 66.442),
            Scale::S800 => (0.476, 0.114, 61.907),
            Scale::S900 => (0.421, 0.095, 57.708),
            Scale::S950 => (0.286, 0.066, 53.813),
        },
        ColorFamily::Lime => match scale {
            Scale::S50 => (0.986, 0.031, 120.757),
            Scale::S100 => (0.967, 0.067, 122.328),
            Scale::S200 => (0.938, 0.127, 124.321),
            Scale::S300 => (0.897, 0.196, 126.665),
            Scale::S400 => (0.841, 0.238, 128.85),
            Scale::S500 => (0.768, 0.233, 130.85),
            Scale::S600 => (0.648, 0.2, 131.684),
            Scale::S700 => (0.532, 0.157, 131.589),
            Scale::S800 => (0.453, 0.124, 130.933),
            Scale::S900 => (0.405, 0.101, 131.063),
            Scale::S950 => (0.274, 0.072, 132.109),
        },
        ColorFamily::Green => match scale {
            Scale::S50 => (0.982, 0.018, 155.826),
            Scale::S100 => (0.962, 0.044, 156.743),
            Scale::S200 => (0.925, 0.084, 155.995),
            Scale::S300 => (0.871, 0.15, 154.449),
            Scale::S400 => (0.792, 0.209, 151.711),
            Scale::S500 => (0.723, 0.219, 149.579),
            Scale::S600 => (0.627, 0.194, 149.214),
            Scale::S700 => (0.527, 0.154, 150.069),
            Scale::S800 => (0.448, 0.119, 151.328),
            Scale::S900 => (0.393, 0.095, 152.535),
            Scale::S950 => (0.266, 0.065, 152.934),
        },
        ColorFamily::Emerald => match scale {
            Scale::S50 => (0.979, 0.021, 166.113),
            Scale::S100 => (0.95, 0.052, 163.051),
            Scale::S200 => (0.905, 0.093, 164.15),
            Scale::S300 => (0.845, 0.143, 164.978),
            Scale::S400 => (0.765, 0.177, 163.223),
            Scale::S500 => (0.696, 0.17, 162.48),
            Scale::S600 => (0.596, 0.145, 163.225),
            Scale::S700 => (0.508, 0.118, 165.612),
            Scale::S800 => (0.432, 0.095, 166.913),
            Scale::S900 => (0.378, 0.077, 168.94),
            Scale::S950 => (0.262, 0.051, 172.552),
        },
        ColorFamily::Teal => match scale {
            Scale::S50 => (0.984, 0.014, 180.72),
            Scale::S100 => (0.953, 0.051, 180.801),
            Scale::S200 => (0.91, 0.096, 180.426),
            Scale::S300 => (0.855, 0.138, 181.071),
            Scale::S400 => (0.777, 0.152, 181.912),
            Scale::S500 => (0.704, 0.14, 182.503),
            Scale::S600 => (0.6, 0.118, 184.704),
            Scale::S700 => (0.511, 0.096, 186.391),
            Scale::S800 => (0.437, 0.078, 188.216),
            Scale::S900 => (0.386, 0.063, 188.416),
            Scale::S950 => (0.277, 0.046, 192.524),
        },
        ColorFamily::Cyan => match scale {
            Scale::S50 => (0.984, 0.019, 200.873),
            Scale::S100 => (0.956, 0.045, 203.388),
            Scale::S200 => (0.917, 0.08, 205.041),
            Scale::S300 => (0.865, 0.127, 207.078),
            Scale::S400 => (0.789, 0.154, 211.53),
            Scale::S500 => (0.715, 0.143, 215.221),
            Scale::S600 => (0.609, 0.126, 221.723),
            Scale::S700 => (0.52, 0.105, 223.128),
            Scale::S800 => (0.45, 0.085, 224.283),
            Scale::S900 => (0.398, 0.07, 227.392),
            Scale::S950 => (0.302, 0.056, 229.695),
        },
        ColorFamily::Sky => match scale {
            Scale::S50 => (0.977, 0.013, 236.62),
            Scale::S100 => (0.951, 0.026, 236.824),
            Scale::S200 => (0.901, 0.058, 230.902),
            Scale::S300 => (0.828, 0.111, 230.318),
            Scale::S400 => (0.746, 0.16, 232.661),
            Scale::S500 => (0.685, 0.169, 237.323),
            Scale::S600 => (0.588, 0.158, 241.966),
            Scale::S700 => (0.5, 0.134, 242.749),
            Scale::S800 => (0.443, 0.11, 240.79),
            Scale::S900 => (0.391, 0.09, 240.876),
            Scale::S950 => (0.293, 0.066, 243.157),
        },
        ColorFamily::Blue => match scale {
            Scale::S50 => (0.97, 0.014, 254.604),
            Scale::S100 => (0.932, 0.032, 255.585),
            Scale::S200 => (0.882, 0.059, 254.128),
            Scale::S300 => (0.809, 0.105, 251.813),
            Scale::S400 => (0.707, 0.165, 254.624),
            Scale::S500 => (0.623, 0.214, 259.815),
            Scale::S600 => (0.546, 0.245, 262.881),
            Scale::S700 => (0.488, 0.243, 264.376),
            Scale::S800 => (0.424, 0.199, 265.638),
            Scale::S900 => (0.379, 0.146, 265.522),
            Scale::S950 => (0.282, 0.091, 267.935),
        },
        ColorFamily::Indigo => match scale {
            Scale::S50 => (0.962, 0.018, 272.314),
            Scale::S100 => (0.93, 0.034, 272.788),
            Scale::S200 => (0.87, 0.065, 274.039),
            Scale::S300 => (0.785, 0.115, 274.713),
            Scale::S400 => (0.673, 0.182, 276.935),
            Scale::S500 => (0.585, 0.233, 277.117),
            Scale::S600 => (0.511, 0.262, 276.966),
            Scale::S700 => (0.457, 0.24, 277.023),
            Scale::S800 => (0.398, 0.195, 277.366),
            Scale::S900 => (0.359, 0.144, 278.697),
            Scale::S950 => (0.257, 0.09, 281.288),
        },
        ColorFamily::Violet => match scale {
            Scale::S50 => (0.969, 0.016, 293.756),
            Scale::S100 => (0.943, 0.029, 294.588),
            Scale::S200 => (0.894, 0.057, 293.283),
            Scale::S300 => (0.811, 0.111, 293.571),
            Scale::S400 => (0.702, 0.183, 293.541),
            Scale::S500 => (0.606, 0.25, 292.717),
            Scale::S600 => (0.541, 0.281, 293.009),
            Scale::S700 => (0.491, 0.27, 292.581),
            Scale::S800 => (0.432, 0.232, 292.759),
            Scale::S900 => (0.38, 0.189, 293.745),
            Scale::S950 => (0.283, 0.141, 291.089),
        },
        ColorFamily::Purple => match scale {
            Scale::S50 => (0.977, 0.014, 308.299),
            Scale::S100 => (0.946, 0.033, 307.174),
            Scale::S200 => (0.902, 0.063, 306.703),
            Scale::S300 => (0.827, 0.119, 306.383),
            Scale::S400 => (0.714, 0.203, 305.504),
            Scale::S500 => (0.627, 0.265, 303.9),
            Scale::S600 => (0.558, 0.288, 302.321),
            Scale::S700 => (0.496, 0.265, 301.924),
            Scale::S800 => (0.438, 0.218, 303.724),
            Scale::S900 => (0.381, 0.176, 304.987),
            Scale::S950 => (0.291, 0.149, 302.717),
        },
        ColorFamily::Fuchsia => match scale {
            Scale::S50 => (0.977, 0.017, 320.058),
            Scale::S100 => (0.952, 0.037, 318.852),
            Scale::S200 => (0.903, 0.076, 319.62),
            Scale::S300 => (0.833, 0.145, 321.434),
            Scale::S400 => (0.74, 0.238, 322.16),
            Scale::S500 => (0.667, 0.295, 322.15),
            Scale::S600 => (0.591, 0.293, 322.896),
            Scale::S700 => (0.518, 0.253, 323.949),
            Scale::S800 => (0.452, 0.211, 324.591),
            Scale::S900 => (0.401, 0.17, 325.612),
            Scale::S950 => (0.293, 0.136, 325.661),
        },
        ColorFamily::Pink => match scale {
            Scale::S50 => (0.971, 0.014, 343.198),
            Scale::S100 => (0.948, 0.028, 342.258),
            Scale::S200 => (0.899, 0.061, 343.231),
            Scale::S300 => (0.823, 0.12, 346.018),
            Scale::S400 => (0.718, 0.202, 349.761),
            Scale::S500 => (0.656, 0.241, 354.308),
            Scale::S600 => (0.592, 0.249, 0.584),
            Scale::S700 => (0.525, 0.223, 3.958),
            Scale::S800 => (0.459, 0.187, 3.815),
            Scale::S900 => (0.408, 0.153, 2.432),
            Scale::S950 => (0.284, 0.109, 3.907),
        },
        ColorFamily::Rose => match scale {
            Scale::S50 => (0.969, 0.015, 12.422),
            Scale::S100 => (0.941, 0.03, 12.58),
            Scale::S200 => (0.892, 0.058, 10.001),
            Scale::S300 => (0.81, 0.117, 11.638),
            Scale::S400 => (0.712, 0.194, 13.428),
            Scale::S500 => (0.645, 0.246, 16.439),
            Scale::S600 => (0.586, 0.253, 17.585),
            Scale::S700 => (0.514, 0.222, 16.935),
            Scale::S800 => (0.455, 0.188, 13.697),
            Scale::S900 => (0.41, 0.159, 10.272),
            Scale::S950 => (0.271, 0.105, 12.094),
        },
        // Real Tailwind v4 custom palettes from
        // references/tailwindcss/packages/tailwindcss/theme.css.
        ColorFamily::Mauve => match scale {
            Scale::S50 => (0.985, 0.0, 0.0),
            Scale::S100 => (0.96, 0.003, 325.6),
            Scale::S200 => (0.922, 0.005, 325.62),
            Scale::S300 => (0.865, 0.012, 325.68),
            Scale::S400 => (0.711, 0.019, 323.02),
            Scale::S500 => (0.542, 0.034, 322.5),
            Scale::S600 => (0.435, 0.029, 321.78),
            Scale::S700 => (0.364, 0.029, 323.89),
            Scale::S800 => (0.263, 0.024, 320.12),
            Scale::S900 => (0.212, 0.019, 322.12),
            Scale::S950 => (0.145, 0.008, 326.0),
        },
        ColorFamily::Olive => match scale {
            Scale::S50 => (0.988, 0.003, 106.5),
            Scale::S100 => (0.966, 0.005, 106.5),
            Scale::S200 => (0.93, 0.007, 106.5),
            Scale::S300 => (0.88, 0.011, 106.6),
            Scale::S400 => (0.737, 0.021, 106.9),
            Scale::S500 => (0.58, 0.031, 107.3),
            Scale::S600 => (0.466, 0.025, 107.3),
            Scale::S700 => (0.394, 0.023, 107.4),
            Scale::S800 => (0.286, 0.016, 107.4),
            Scale::S900 => (0.228, 0.013, 107.4),
            Scale::S950 => (0.153, 0.006, 107.1),
        },
        ColorFamily::Mist => match scale {
            Scale::S50 => (0.987, 0.002, 197.1),
            Scale::S100 => (0.963, 0.002, 197.1),
            Scale::S200 => (0.925, 0.005, 214.3),
            Scale::S300 => (0.872, 0.007, 219.6),
            Scale::S400 => (0.723, 0.014, 214.4),
            Scale::S500 => (0.56, 0.021, 213.5),
            Scale::S600 => (0.45, 0.017, 213.2),
            Scale::S700 => (0.378, 0.015, 216.0),
            Scale::S800 => (0.275, 0.011, 216.9),
            Scale::S900 => (0.218, 0.008, 223.9),
            Scale::S950 => (0.148, 0.004, 228.8),
        },
        ColorFamily::Taupe => match scale {
            Scale::S50 => (0.986, 0.002, 67.8),
            Scale::S100 => (0.96, 0.002, 17.2),
            Scale::S200 => (0.922, 0.005, 34.3),
            Scale::S300 => (0.868, 0.007, 39.5),
            Scale::S400 => (0.714, 0.014, 41.2),
            Scale::S500 => (0.547, 0.021, 43.1),
            Scale::S600 => (0.438, 0.017, 39.3),
            Scale::S700 => (0.367, 0.016, 35.7),
            Scale::S800 => (0.268, 0.011, 36.5),
            Scale::S900 => (0.214, 0.009, 43.1),
            Scale::S950 => (0.147, 0.004, 49.3),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_oklch_close(
        actual: (f32, f32, f32),
        expected: (f32, f32, f32),
        lightness_tolerance: f32,
        chroma_tolerance: f32,
        hue_tolerance: f32,
    ) {
        let lightness_delta = (actual.0 - expected.0).abs();
        let chroma_delta = (actual.1 - expected.1).abs();
        let hue_delta = (actual.2 - expected.2)
            .abs()
            .min(360.0 - (actual.2 - expected.2).abs());

        assert!(
            lightness_delta <= lightness_tolerance,
            "lightness mismatch: actual={}, expected={}, delta={}",
            actual.0,
            expected.0,
            lightness_delta
        );
        assert!(
            chroma_delta <= chroma_tolerance,
            "chroma mismatch: actual={}, expected={}, delta={}",
            actual.1,
            expected.1,
            chroma_delta
        );
        if expected.1 > 0.01 {
            assert!(
                hue_delta <= hue_tolerance,
                "hue mismatch: actual={}, expected={}, delta={}",
                actual.2,
                expected.2,
                hue_delta
            );
        }
    }

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
        let (l, c, h) = value.to_oklch();
        assert!((l - 0.623).abs() < 0.02);
        assert!((c - 0.214).abs() < 0.03);
        assert!((h - 259.815).abs() < 3.0);
    }

    #[test]
    fn test_oklch_roundtrip_conversion() {
        let value = ColorValue::from_rgb(255, 0, 0);
        let (l, c, h) = value.to_oklch();
        let roundtrip = ColorValue::from_oklch(l, c, h);
        assert_eq!(roundtrip.to_rgb8(), value.to_rgb8());
    }

    #[test]
    fn test_black_white_are_exact() {
        assert_eq!(Color::black().compute().to_rgb8(), (0, 0, 0));
        assert_eq!(Color::white().compute().to_rgb8(), (255, 255, 255));
    }

    #[test]
    fn test_oklch_contrast_pick() {
        assert_eq!(
            ColorValue::from_rgb(255, 255, 255).preferred_text_color(),
            SpecialColor::Black
        );
        assert_eq!(
            ColorValue::from_rgb(0, 0, 0).preferred_text_color(),
            SpecialColor::White
        );
    }

    #[test]
    fn test_oklch_scale_generation_order() {
        let brand = ColorValue::from_oklch(0.623, 0.188, 259.815);
        let scale = brand.generate_scale_oklch();
        assert_eq!(scale.len(), 11);
        assert!(
            scale[0].perceived_lightness_oklch() > scale[10].perceived_lightness_oklch(),
            "50 should be lighter than 950"
        );
    }

    #[test]
    fn test_background_color_var_standard_string_traits() {
        let var = BackgroundColorVar::from("--surface");
        assert_eq!(var.as_ref(), "--surface");
        assert_eq!(var.to_string(), "--surface");
    }

    #[test]
    fn test_tailwind_palette_reference_entries_match_exact_oklch_values() {
        let cases = [
            (ColorFamily::Slate, Scale::S50, (0.984, 0.003, 247.858)),
            (ColorFamily::Gray, Scale::S500, (0.551, 0.027, 264.364)),
            (ColorFamily::Blue, Scale::S500, (0.623, 0.214, 259.815)),
            (ColorFamily::Red, Scale::S950, (0.258, 0.092, 26.042)),
            (ColorFamily::Emerald, Scale::S400, (0.765, 0.177, 163.223)),
            (ColorFamily::Mauve, Scale::S500, (0.542, 0.034, 322.5)),
            (ColorFamily::Olive, Scale::S950, (0.153, 0.006, 107.1)),
            (ColorFamily::Mist, Scale::S300, (0.872, 0.007, 219.6)),
            (ColorFamily::Taupe, Scale::S100, (0.96, 0.002, 17.2)),
        ];

        for (family, scale, expected) in cases {
            assert_eq!(get_palette_oklch(family, scale), expected);
        }
    }

    #[test]
    fn test_all_tailwind_palette_entries_roundtrip_through_oklch() {
        let families = [
            ColorFamily::Slate,
            ColorFamily::Gray,
            ColorFamily::Zinc,
            ColorFamily::Neutral,
            ColorFamily::Stone,
            ColorFamily::Red,
            ColorFamily::Orange,
            ColorFamily::Amber,
            ColorFamily::Yellow,
            ColorFamily::Lime,
            ColorFamily::Green,
            ColorFamily::Emerald,
            ColorFamily::Teal,
            ColorFamily::Cyan,
            ColorFamily::Sky,
            ColorFamily::Blue,
            ColorFamily::Indigo,
            ColorFamily::Violet,
            ColorFamily::Purple,
            ColorFamily::Fuchsia,
            ColorFamily::Pink,
            ColorFamily::Rose,
            ColorFamily::Mauve,
            ColorFamily::Olive,
            ColorFamily::Mist,
            ColorFamily::Taupe,
        ];

        for family in families {
            for scale in Scale::ALL {
                let expected = get_palette_oklch(family, scale);
                let actual = Color::new(family, scale).compute().to_oklch();
                assert_oklch_close(actual, expected, 0.02, 0.035, 6.0);
            }
        }
    }
}
