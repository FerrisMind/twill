//! Optional semantic tokens inspired by shadcn variables.
//!
//! These tokens are opt-in: you can ignore them and continue using raw color tokens.

use std::fmt;
use std::sync::LazyLock;

use crate::tokens::{Color, ColorValue, Scale, SpecialColor};
use crate::traits::ComputeValue;

/// shadcn-style semantic color variable names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticColor {
    Background,
    Foreground,
    Card,
    CardForeground,
    Popover,
    PopoverForeground,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Muted,
    MutedForeground,
    Accent,
    AccentForeground,
    Destructive,
    Border,
    Input,
    Ring,
    Chart1,
    Chart2,
    Chart3,
    Chart4,
    Chart5,
    Sidebar,
    SidebarForeground,
    SidebarPrimary,
    SidebarPrimaryForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarRing,
}

impl SemanticColor {
    pub const fn var_name(&self) -> &'static str {
        match self {
            Self::Background => "background",
            Self::Foreground => "foreground",
            Self::Card => "card",
            Self::CardForeground => "card-foreground",
            Self::Popover => "popover",
            Self::PopoverForeground => "popover-foreground",
            Self::Primary => "primary",
            Self::PrimaryForeground => "primary-foreground",
            Self::Secondary => "secondary",
            Self::SecondaryForeground => "secondary-foreground",
            Self::Muted => "muted",
            Self::MutedForeground => "muted-foreground",
            Self::Accent => "accent",
            Self::AccentForeground => "accent-foreground",
            Self::Destructive => "destructive",
            Self::Border => "border",
            Self::Input => "input",
            Self::Ring => "ring",
            Self::Chart1 => "chart-1",
            Self::Chart2 => "chart-2",
            Self::Chart3 => "chart-3",
            Self::Chart4 => "chart-4",
            Self::Chart5 => "chart-5",
            Self::Sidebar => "sidebar",
            Self::SidebarForeground => "sidebar-foreground",
            Self::SidebarPrimary => "sidebar-primary",
            Self::SidebarPrimaryForeground => "sidebar-primary-foreground",
            Self::SidebarAccent => "sidebar-accent",
            Self::SidebarAccentForeground => "sidebar-accent-foreground",
            Self::SidebarBorder => "sidebar-border",
            Self::SidebarRing => "sidebar-ring",
        }
    }
}

impl fmt::Display for SemanticColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.var_name())
    }
}

/// Theme variant used when resolving semantic tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeVariant {
    Light,
    Dark,
}

impl ThemeVariant {
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

/// Semantic token set with light and dark variable definitions.
#[derive(Debug, Clone)]
pub struct SemanticThemeVars {
    pub(crate) radius: &'static str,
    pub(crate) light: Vec<(SemanticColor, Color)>,
    pub(crate) dark: Vec<(SemanticColor, Color)>,
    pub(crate) light_values: Vec<(SemanticColor, ColorValue)>,
    pub(crate) dark_values: Vec<(SemanticColor, ColorValue)>,
}

const SEMANTIC_COLOR_COUNT: usize = 31;

const fn semantic_color_index(token: SemanticColor) -> usize {
    match token {
        SemanticColor::Background => 0,
        SemanticColor::Foreground => 1,
        SemanticColor::Card => 2,
        SemanticColor::CardForeground => 3,
        SemanticColor::Popover => 4,
        SemanticColor::PopoverForeground => 5,
        SemanticColor::Primary => 6,
        SemanticColor::PrimaryForeground => 7,
        SemanticColor::Secondary => 8,
        SemanticColor::SecondaryForeground => 9,
        SemanticColor::Muted => 10,
        SemanticColor::MutedForeground => 11,
        SemanticColor::Accent => 12,
        SemanticColor::AccentForeground => 13,
        SemanticColor::Destructive => 14,
        SemanticColor::Border => 15,
        SemanticColor::Input => 16,
        SemanticColor::Ring => 17,
        SemanticColor::Chart1 => 18,
        SemanticColor::Chart2 => 19,
        SemanticColor::Chart3 => 20,
        SemanticColor::Chart4 => 21,
        SemanticColor::Chart5 => 22,
        SemanticColor::Sidebar => 23,
        SemanticColor::SidebarForeground => 24,
        SemanticColor::SidebarPrimary => 25,
        SemanticColor::SidebarPrimaryForeground => 26,
        SemanticColor::SidebarAccent => 27,
        SemanticColor::SidebarAccentForeground => 28,
        SemanticColor::SidebarBorder => 29,
        SemanticColor::SidebarRing => 30,
    }
}

fn resolve_fixed_entry<T: Copy>(entries: &[(SemanticColor, T)], token: SemanticColor) -> Option<T> {
    entries
        .get(semantic_color_index(token))
        .map(|(_, value)| *value)
}

impl SemanticThemeVars {
    pub const fn radius(&self) -> &'static str {
        self.radius
    }

    pub fn light_tokens(&self) -> &[(SemanticColor, Color)] {
        &self.light
    }

    pub fn dark_tokens(&self) -> &[(SemanticColor, Color)] {
        &self.dark
    }

    pub fn light_values(&self) -> &[(SemanticColor, ColorValue)] {
        &self.light_values
    }

    pub fn dark_values(&self) -> &[(SemanticColor, ColorValue)] {
        &self.dark_values
    }

    /// Default neutral semantic theme matching shadcn-ui / shadcn-svelte values.
    pub fn shadcn_neutral() -> &'static Self {
        static SHADCN_NEUTRAL: LazyLock<SemanticThemeVars> =
            LazyLock::new(SemanticThemeVars::build_shadcn_neutral);
        &SHADCN_NEUTRAL
    }

    fn build_shadcn_neutral() -> Self {
        let light = vec![
            (SemanticColor::Background, Color::white()),
            (SemanticColor::Foreground, Color::neutral(Scale::S950)),
            (SemanticColor::Card, Color::white()),
            (SemanticColor::CardForeground, Color::neutral(Scale::S950)),
            (SemanticColor::Popover, Color::white()),
            (
                SemanticColor::PopoverForeground,
                Color::neutral(Scale::S950),
            ),
            (SemanticColor::Primary, Color::neutral(Scale::S900)),
            (SemanticColor::PrimaryForeground, Color::neutral(Scale::S50)),
            (SemanticColor::Secondary, Color::neutral(Scale::S100)),
            (
                SemanticColor::SecondaryForeground,
                Color::neutral(Scale::S900),
            ),
            (SemanticColor::Muted, Color::neutral(Scale::S100)),
            (SemanticColor::MutedForeground, Color::neutral(Scale::S500)),
            (SemanticColor::Accent, Color::neutral(Scale::S100)),
            (SemanticColor::AccentForeground, Color::neutral(Scale::S900)),
            (SemanticColor::Destructive, Color::red(Scale::S600)),
            (SemanticColor::Border, Color::neutral(Scale::S200)),
            (SemanticColor::Input, Color::neutral(Scale::S200)),
            (SemanticColor::Ring, Color::neutral(Scale::S400)),
            (SemanticColor::Chart1, Color::orange(Scale::S600)),
            (SemanticColor::Chart2, Color::teal(Scale::S600)),
            (SemanticColor::Chart3, Color::cyan(Scale::S900)),
            (SemanticColor::Chart4, Color::amber(Scale::S400)),
            (SemanticColor::Chart5, Color::amber(Scale::S500)),
            (SemanticColor::Sidebar, Color::neutral(Scale::S50)),
            (
                SemanticColor::SidebarForeground,
                Color::neutral(Scale::S950),
            ),
            (SemanticColor::SidebarPrimary, Color::neutral(Scale::S900)),
            (
                SemanticColor::SidebarPrimaryForeground,
                Color::neutral(Scale::S50),
            ),
            (SemanticColor::SidebarAccent, Color::neutral(Scale::S100)),
            (
                SemanticColor::SidebarAccentForeground,
                Color::neutral(Scale::S900),
            ),
            (SemanticColor::SidebarBorder, Color::neutral(Scale::S200)),
            (SemanticColor::SidebarRing, Color::neutral(Scale::S400)),
        ];
        let dark = vec![
            (SemanticColor::Background, Color::neutral(Scale::S950)),
            (SemanticColor::Foreground, Color::neutral(Scale::S50)),
            (SemanticColor::Card, Color::neutral(Scale::S900)),
            (SemanticColor::CardForeground, Color::neutral(Scale::S50)),
            (SemanticColor::Popover, Color::neutral(Scale::S900)),
            (SemanticColor::PopoverForeground, Color::neutral(Scale::S50)),
            (SemanticColor::Primary, Color::neutral(Scale::S200)),
            (
                SemanticColor::PrimaryForeground,
                Color::neutral(Scale::S900),
            ),
            (SemanticColor::Secondary, Color::neutral(Scale::S800)),
            (
                SemanticColor::SecondaryForeground,
                Color::neutral(Scale::S50),
            ),
            (SemanticColor::Muted, Color::neutral(Scale::S800)),
            (SemanticColor::MutedForeground, Color::neutral(Scale::S400)),
            (SemanticColor::Accent, Color::neutral(Scale::S800)),
            (SemanticColor::AccentForeground, Color::neutral(Scale::S50)),
            (SemanticColor::Destructive, Color::red(Scale::S400)),
            (SemanticColor::Border, Color::white()),
            (SemanticColor::Input, Color::white()),
            (SemanticColor::Ring, Color::neutral(Scale::S500)),
            (SemanticColor::Chart1, Color::blue(Scale::S700)),
            (SemanticColor::Chart2, Color::emerald(Scale::S500)),
            (SemanticColor::Chart3, Color::amber(Scale::S500)),
            (SemanticColor::Chart4, Color::purple(Scale::S500)),
            (SemanticColor::Chart5, Color::rose(Scale::S500)),
            (SemanticColor::Sidebar, Color::neutral(Scale::S900)),
            (SemanticColor::SidebarForeground, Color::neutral(Scale::S50)),
            (SemanticColor::SidebarPrimary, Color::blue(Scale::S700)),
            (
                SemanticColor::SidebarPrimaryForeground,
                Color::neutral(Scale::S50),
            ),
            (SemanticColor::SidebarAccent, Color::neutral(Scale::S800)),
            (
                SemanticColor::SidebarAccentForeground,
                Color::neutral(Scale::S50),
            ),
            (SemanticColor::SidebarBorder, Color::white()),
            (SemanticColor::SidebarRing, Color::neutral(Scale::S500)),
        ];

        // Keep semantic values aligned to core palette tokens; only alpha-specific
        // shadcn dark values are explicit overrides.
        let light_values = light
            .iter()
            .map(|(token, color)| (*token, color.compute()))
            .collect::<Vec<_>>();
        let mut dark_values = dark
            .iter()
            .map(|(token, color)| (*token, color.compute()))
            .collect::<Vec<_>>();

        for (token, alpha) in [
            (SemanticColor::Border, 0.10f32),
            (SemanticColor::Input, 0.15f32),
            (SemanticColor::SidebarBorder, 0.10f32),
        ] {
            if let Some((_, value)) = dark_values.iter_mut().find(|(t, _)| *t == token) {
                *value = value.with_alpha(alpha);
            }
        }

        Self {
            radius: "0.625rem",
            light,
            dark,
            light_values,
            dark_values,
        }
    }

    /// Resolve a semantic token to a concrete `Color`.
    ///
    /// [`ThemeVariant::Light`] resolves from light values and [`ThemeVariant::Dark`]
    /// resolves from dark values.
    pub fn resolve(&self, token: SemanticColor, variant: ThemeVariant) -> Option<Color> {
        let source = if variant.is_dark() {
            &self.dark
        } else {
            &self.light
        };
        resolve_fixed_entry(source, token)
    }

    pub fn resolve_value(&self, token: SemanticColor, variant: ThemeVariant) -> Option<ColorValue> {
        let source = if variant.is_dark() {
            &self.dark_values
        } else {
            &self.light_values
        };
        resolve_fixed_entry(source, token)
    }

    /// Resolve a semantic token from the light palette.
    pub fn resolve_light(&self, token: SemanticColor) -> Option<Color> {
        self.resolve(token, ThemeVariant::Light)
    }

    /// Resolve a semantic token from the dark palette.
    pub fn resolve_dark(&self, token: SemanticColor) -> Option<Color> {
        self.resolve(token, ThemeVariant::Dark)
    }

    pub fn resolve_light_value(&self, token: SemanticColor) -> Option<ColorValue> {
        self.resolve_value(token, ThemeVariant::Light)
    }

    pub fn resolve_dark_value(&self, token: SemanticColor) -> Option<ColorValue> {
        self.resolve_value(token, ThemeVariant::Dark)
    }
}

/// Dynamic semantic theme generated from an arbitrary brand color using OKLCH.
#[derive(Debug, Clone)]
pub struct DynamicSemanticTheme {
    pub(crate) light: Vec<(SemanticColor, ColorValue)>,
    pub(crate) dark: Vec<(SemanticColor, ColorValue)>,
}

impl DynamicSemanticTheme {
    pub fn light_values(&self) -> &[(SemanticColor, ColorValue)] {
        &self.light
    }

    pub fn dark_values(&self) -> &[(SemanticColor, ColorValue)] {
        &self.dark
    }

    fn scale_value(scale: &[(Scale, ColorValue); 11], target: Scale) -> ColorValue {
        scale
            .iter()
            .find(|(s, _)| *s == target)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| unreachable!("Scale::ALL must contain all 11 scale entries"))
    }

    fn readable_text_for(bg: ColorValue) -> ColorValue {
        match bg.preferred_text_color() {
            SpecialColor::Black => Color::black().compute(),
            SpecialColor::White => Color::white().compute(),
            SpecialColor::Transparent | SpecialColor::Current => Color::white().compute(),
        }
    }

    /// Builds light/dark semantic palettes from any brand OKLCH color.
    pub fn from_brand_oklch(l: f32, c: f32, h: f32) -> Self {
        let brand = ColorValue::from_oklch(l, c, h);
        let scale = brand.generate_scale_map_oklch();

        let light_primary = Self::scale_value(&scale, Scale::S500);
        let dark_primary = Self::scale_value(&scale, Scale::S400);
        let light_primary_fg = Self::readable_text_for(light_primary);
        let dark_primary_fg = Self::readable_text_for(dark_primary);

        let light = vec![
            (SemanticColor::Background, Color::white().compute()),
            (
                SemanticColor::Foreground,
                Color::gray(Scale::S900).compute(),
            ),
            (SemanticColor::Card, Color::white().compute()),
            (
                SemanticColor::CardForeground,
                Color::gray(Scale::S900).compute(),
            ),
            (SemanticColor::Popover, Color::white().compute()),
            (
                SemanticColor::PopoverForeground,
                Color::gray(Scale::S900).compute(),
            ),
            (SemanticColor::Primary, light_primary),
            (SemanticColor::PrimaryForeground, light_primary_fg),
            (SemanticColor::Secondary, Color::gray(Scale::S100).compute()),
            (
                SemanticColor::SecondaryForeground,
                Color::gray(Scale::S900).compute(),
            ),
            (SemanticColor::Muted, Color::gray(Scale::S100).compute()),
            (
                SemanticColor::MutedForeground,
                Color::gray(Scale::S500).compute(),
            ),
            (
                SemanticColor::Accent,
                Self::scale_value(&scale, Scale::S100),
            ),
            (
                SemanticColor::AccentForeground,
                Self::readable_text_for(Self::scale_value(&scale, Scale::S100)),
            ),
            (
                SemanticColor::Destructive,
                Color::red(Scale::S600).compute(),
            ),
            (SemanticColor::Border, Color::gray(Scale::S200).compute()),
            (SemanticColor::Input, Color::gray(Scale::S200).compute()),
            (SemanticColor::Ring, Self::scale_value(&scale, Scale::S500)),
            (
                SemanticColor::Chart1,
                Self::scale_value(&scale, Scale::S500),
            ),
            (
                SemanticColor::Chart2,
                Self::scale_value(&scale, Scale::S600),
            ),
            (
                SemanticColor::Chart3,
                Self::scale_value(&scale, Scale::S700),
            ),
            (
                SemanticColor::Chart4,
                Self::scale_value(&scale, Scale::S400),
            ),
            (
                SemanticColor::Chart5,
                Self::scale_value(&scale, Scale::S300),
            ),
            (SemanticColor::Sidebar, Color::white().compute()),
            (
                SemanticColor::SidebarForeground,
                Color::gray(Scale::S900).compute(),
            ),
            (
                SemanticColor::SidebarPrimary,
                Self::scale_value(&scale, Scale::S600),
            ),
            (
                SemanticColor::SidebarPrimaryForeground,
                Self::readable_text_for(Self::scale_value(&scale, Scale::S600)),
            ),
            (
                SemanticColor::SidebarAccent,
                Self::scale_value(&scale, Scale::S100),
            ),
            (
                SemanticColor::SidebarAccentForeground,
                Self::readable_text_for(Self::scale_value(&scale, Scale::S100)),
            ),
            (
                SemanticColor::SidebarBorder,
                Color::gray(Scale::S200).compute(),
            ),
            (
                SemanticColor::SidebarRing,
                Self::scale_value(&scale, Scale::S500),
            ),
        ];

        let dark = vec![
            (
                SemanticColor::Background,
                Color::gray(Scale::S950).compute(),
            ),
            (SemanticColor::Foreground, Color::gray(Scale::S50).compute()),
            (SemanticColor::Card, Color::gray(Scale::S900).compute()),
            (
                SemanticColor::CardForeground,
                Color::gray(Scale::S50).compute(),
            ),
            (SemanticColor::Popover, Color::gray(Scale::S800).compute()),
            (
                SemanticColor::PopoverForeground,
                Color::gray(Scale::S50).compute(),
            ),
            (SemanticColor::Primary, dark_primary),
            (SemanticColor::PrimaryForeground, dark_primary_fg),
            (SemanticColor::Secondary, Color::gray(Scale::S800).compute()),
            (
                SemanticColor::SecondaryForeground,
                Color::gray(Scale::S50).compute(),
            ),
            (SemanticColor::Muted, Color::gray(Scale::S800).compute()),
            (
                SemanticColor::MutedForeground,
                Color::gray(Scale::S400).compute(),
            ),
            (
                SemanticColor::Accent,
                Self::scale_value(&scale, Scale::S700),
            ),
            (
                SemanticColor::AccentForeground,
                Self::readable_text_for(Self::scale_value(&scale, Scale::S700)),
            ),
            (
                SemanticColor::Destructive,
                Color::red(Scale::S500).compute(),
            ),
            (SemanticColor::Border, Color::gray(Scale::S700).compute()),
            (SemanticColor::Input, Color::gray(Scale::S700).compute()),
            (SemanticColor::Ring, Self::scale_value(&scale, Scale::S400)),
            (
                SemanticColor::Chart1,
                Self::scale_value(&scale, Scale::S400),
            ),
            (
                SemanticColor::Chart2,
                Self::scale_value(&scale, Scale::S500),
            ),
            (
                SemanticColor::Chart3,
                Self::scale_value(&scale, Scale::S600),
            ),
            (
                SemanticColor::Chart4,
                Self::scale_value(&scale, Scale::S300),
            ),
            (
                SemanticColor::Chart5,
                Self::scale_value(&scale, Scale::S200),
            ),
            (SemanticColor::Sidebar, Color::gray(Scale::S900).compute()),
            (
                SemanticColor::SidebarForeground,
                Color::gray(Scale::S50).compute(),
            ),
            (
                SemanticColor::SidebarPrimary,
                Self::scale_value(&scale, Scale::S500),
            ),
            (
                SemanticColor::SidebarPrimaryForeground,
                Self::readable_text_for(Self::scale_value(&scale, Scale::S500)),
            ),
            (
                SemanticColor::SidebarAccent,
                Color::gray(Scale::S800).compute(),
            ),
            (
                SemanticColor::SidebarAccentForeground,
                Color::gray(Scale::S50).compute(),
            ),
            (
                SemanticColor::SidebarBorder,
                Color::gray(Scale::S700).compute(),
            ),
            (
                SemanticColor::SidebarRing,
                Self::scale_value(&scale, Scale::S400),
            ),
        ];

        Self { light, dark }
    }

    /// Resolve a semantic token to concrete RGBA color.
    pub fn resolve(&self, token: SemanticColor, variant: ThemeVariant) -> Option<ColorValue> {
        let source = if variant.is_dark() {
            &self.dark
        } else {
            &self.light
        };
        if source.len() == SEMANTIC_COLOR_COUNT {
            resolve_fixed_entry(source, token)
        } else {
            source
                .iter()
                .find(|(t, _)| *t == token)
                .map(|(_, color)| *color)
        }
    }

    pub fn resolve_light(&self, token: SemanticColor) -> Option<ColorValue> {
        self.resolve(token, ThemeVariant::Light)
    }

    pub fn resolve_dark(&self, token: SemanticColor) -> Option<ColorValue> {
        self.resolve(token, ThemeVariant::Dark)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_var_name() {
        assert_eq!(SemanticColor::Primary.var_name(), "primary");
    }

    #[test]
    fn test_semantic_theme_has_palettes() {
        let theme = SemanticThemeVars::shadcn_neutral();
        assert!(!theme.light.is_empty());
        assert!(!theme.dark.is_empty());
    }

    #[test]
    fn test_semantic_resolve_color() {
        let theme = SemanticThemeVars::shadcn_neutral();
        let light_bg = theme.resolve_light(SemanticColor::Background).unwrap();
        let dark_bg = theme.resolve_dark(SemanticColor::Background).unwrap();
        assert_eq!(light_bg, Color::white());
        assert_eq!(dark_bg, Color::neutral(Scale::S950));
    }

    #[test]
    fn test_shadcn_neutral_values_match_reference() {
        let theme = SemanticThemeVars::shadcn_neutral();

        let light_expectations = [
            (
                SemanticColor::Background,
                ColorValue::from_oklch(1.0, 0.0, 0.0),
            ),
            (
                SemanticColor::Foreground,
                ColorValue::from_oklch(0.145, 0.0, 0.0),
            ),
            (
                SemanticColor::Primary,
                ColorValue::from_oklch(0.205, 0.0, 0.0),
            ),
            (
                SemanticColor::PrimaryForeground,
                ColorValue::from_oklch(0.985, 0.0, 0.0),
            ),
            (
                SemanticColor::Secondary,
                ColorValue::from_oklch(0.97, 0.0, 0.0),
            ),
            (
                SemanticColor::MutedForeground,
                ColorValue::from_oklch(0.556, 0.0, 0.0),
            ),
            (
                SemanticColor::Destructive,
                ColorValue::from_oklch(0.577, 0.245, 27.325),
            ),
            (
                SemanticColor::Border,
                ColorValue::from_oklch(0.922, 0.0, 0.0),
            ),
            (SemanticColor::Ring, ColorValue::from_oklch(0.708, 0.0, 0.0)),
            (
                SemanticColor::Chart1,
                ColorValue::from_oklch(0.646, 0.222, 41.116),
            ),
            (
                SemanticColor::Chart2,
                ColorValue::from_oklch(0.6, 0.118, 184.704),
            ),
            (
                SemanticColor::Chart3,
                ColorValue::from_oklch(0.398, 0.07, 227.392),
            ),
            (
                SemanticColor::Sidebar,
                ColorValue::from_oklch(0.985, 0.0, 0.0),
            ),
            (
                SemanticColor::SidebarRing,
                ColorValue::from_oklch(0.708, 0.0, 0.0),
            ),
        ];

        for (token, expected) in light_expectations {
            assert_eq!(theme.resolve_light_value(token), Some(expected));
        }

        let dark_expectations = [
            (
                SemanticColor::Background,
                ColorValue::from_oklch(0.145, 0.0, 0.0),
            ),
            (
                SemanticColor::Foreground,
                ColorValue::from_oklch(0.985, 0.0, 0.0),
            ),
            (
                SemanticColor::Primary,
                ColorValue::from_oklch(0.922, 0.0, 0.0),
            ),
            (
                SemanticColor::PrimaryForeground,
                ColorValue::from_oklch(0.205, 0.0, 0.0),
            ),
            (
                SemanticColor::Secondary,
                ColorValue::from_oklch(0.269, 0.0, 0.0),
            ),
            (
                SemanticColor::MutedForeground,
                ColorValue::from_oklch(0.708, 0.0, 0.0),
            ),
            (
                SemanticColor::Destructive,
                ColorValue::from_oklch(0.704, 0.191, 22.216),
            ),
            (
                SemanticColor::Border,
                ColorValue::from_oklch(1.0, 0.0, 0.0).with_alpha(0.10),
            ),
            (
                SemanticColor::Input,
                ColorValue::from_oklch(1.0, 0.0, 0.0).with_alpha(0.15),
            ),
            (SemanticColor::Ring, ColorValue::from_oklch(0.556, 0.0, 0.0)),
            (
                SemanticColor::Chart1,
                ColorValue::from_oklch(0.488, 0.243, 264.376),
            ),
            (
                SemanticColor::Chart2,
                ColorValue::from_oklch(0.696, 0.17, 162.48),
            ),
            (
                SemanticColor::Chart3,
                ColorValue::from_oklch(0.769, 0.188, 70.08),
            ),
            (
                SemanticColor::Sidebar,
                ColorValue::from_oklch(0.205, 0.0, 0.0),
            ),
            (
                SemanticColor::SidebarBorder,
                ColorValue::from_oklch(1.0, 0.0, 0.0).with_alpha(0.10),
            ),
            (
                SemanticColor::SidebarRing,
                ColorValue::from_oklch(0.556, 0.0, 0.0),
            ),
        ];

        for (token, expected) in dark_expectations {
            assert_eq!(theme.resolve_dark_value(token), Some(expected));
        }
    }

    #[test]
    fn test_shadcn_values_derive_from_color_tokens() {
        let theme = SemanticThemeVars::shadcn_neutral();

        for (token, color) in &theme.light {
            assert_eq!(theme.resolve_light_value(*token), Some(color.compute()));
        }

        for (token, color) in &theme.dark {
            let expected = match token {
                SemanticColor::Border => color.compute().with_alpha(0.10),
                SemanticColor::Input => color.compute().with_alpha(0.15),
                SemanticColor::SidebarBorder => color.compute().with_alpha(0.10),
                _ => color.compute(),
            };
            assert_eq!(theme.resolve_dark_value(*token), Some(expected));
        }
    }

    #[test]
    fn test_dynamic_theme_from_brand_oklch() {
        let theme = DynamicSemanticTheme::from_brand_oklch(0.628, 0.258, 29.234);
        let light_primary = theme
            .resolve_light(SemanticColor::Primary)
            .expect("primary in light theme");
        let dark_primary = theme
            .resolve_dark(SemanticColor::Primary)
            .expect("primary in dark theme");
        assert_ne!(light_primary, dark_primary);
    }
}
