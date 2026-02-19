//! Optional semantic tokens inspired by shadcn variables.
//!
//! These tokens are opt-in: you can ignore them and continue using raw color tokens.

use crate::tokens::{Color, Scale};
use crate::traits::ToCss;

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
    pub fn var_name(&self) -> &'static str {
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

    pub fn as_css_var(&self) -> String {
        format!("var(--{})", self.var_name())
    }
}

impl ToCss for SemanticColor {
    fn to_css(&self) -> String {
        self.as_css_var()
    }
}

/// Semantic token set with light and dark variable definitions.
#[derive(Debug, Clone)]
pub struct SemanticThemeVars {
    pub radius: &'static str,
    pub light: &'static [(SemanticColor, Color)],
    pub dark: &'static [(SemanticColor, Color)],
}

impl SemanticThemeVars {
    /// Default neutral semantic theme mapped to rustwind RGB palette.
    pub fn shadcn_neutral() -> Self {
        const LIGHT: &[(SemanticColor, Color)] = &[
            (SemanticColor::Background, Color::gray(Scale::S50)),
            (SemanticColor::Foreground, Color::gray(Scale::S900)),
            (SemanticColor::Card, Color::white()),
            (SemanticColor::CardForeground, Color::gray(Scale::S900)),
            (SemanticColor::Popover, Color::white()),
            (SemanticColor::PopoverForeground, Color::gray(Scale::S900)),
            (SemanticColor::Primary, Color::gray(Scale::S900)),
            (SemanticColor::PrimaryForeground, Color::gray(Scale::S50)),
            (SemanticColor::Secondary, Color::gray(Scale::S100)),
            (SemanticColor::SecondaryForeground, Color::gray(Scale::S900)),
            (SemanticColor::Muted, Color::gray(Scale::S100)),
            (SemanticColor::MutedForeground, Color::gray(Scale::S500)),
            (SemanticColor::Accent, Color::gray(Scale::S100)),
            (SemanticColor::AccentForeground, Color::gray(Scale::S900)),
            (SemanticColor::Destructive, Color::red(Scale::S600)),
            (SemanticColor::Border, Color::gray(Scale::S200)),
            (SemanticColor::Input, Color::gray(Scale::S200)),
            (SemanticColor::Ring, Color::gray(Scale::S500)),
            (SemanticColor::Chart1, Color::orange(Scale::S500)),
            (SemanticColor::Chart2, Color::teal(Scale::S500)),
            (SemanticColor::Chart3, Color::sky(Scale::S700)),
            (SemanticColor::Chart4, Color::amber(Scale::S400)),
            (SemanticColor::Chart5, Color::amber(Scale::S500)),
            (SemanticColor::Sidebar, Color::gray(Scale::S50)),
            (SemanticColor::SidebarForeground, Color::gray(Scale::S900)),
            (SemanticColor::SidebarPrimary, Color::gray(Scale::S900)),
            (
                SemanticColor::SidebarPrimaryForeground,
                Color::gray(Scale::S50),
            ),
            (SemanticColor::SidebarAccent, Color::gray(Scale::S100)),
            (
                SemanticColor::SidebarAccentForeground,
                Color::gray(Scale::S900),
            ),
            (SemanticColor::SidebarBorder, Color::gray(Scale::S200)),
            (SemanticColor::SidebarRing, Color::gray(Scale::S500)),
        ];
        const DARK: &[(SemanticColor, Color)] = &[
            (SemanticColor::Background, Color::gray(Scale::S950)),
            (SemanticColor::Foreground, Color::gray(Scale::S50)),
            (SemanticColor::Card, Color::gray(Scale::S900)),
            (SemanticColor::CardForeground, Color::gray(Scale::S50)),
            (SemanticColor::Popover, Color::gray(Scale::S800)),
            (SemanticColor::PopoverForeground, Color::gray(Scale::S50)),
            (SemanticColor::Primary, Color::gray(Scale::S200)),
            (SemanticColor::PrimaryForeground, Color::gray(Scale::S900)),
            (SemanticColor::Secondary, Color::gray(Scale::S800)),
            (SemanticColor::SecondaryForeground, Color::gray(Scale::S50)),
            (SemanticColor::Muted, Color::gray(Scale::S800)),
            (SemanticColor::MutedForeground, Color::gray(Scale::S400)),
            (SemanticColor::Accent, Color::gray(Scale::S700)),
            (SemanticColor::AccentForeground, Color::gray(Scale::S50)),
            (SemanticColor::Destructive, Color::red(Scale::S500)),
            (SemanticColor::Border, Color::gray(Scale::S700)),
            (SemanticColor::Input, Color::gray(Scale::S700)),
            (SemanticColor::Ring, Color::gray(Scale::S500)),
            (SemanticColor::Chart1, Color::indigo(Scale::S500)),
            (SemanticColor::Chart2, Color::emerald(Scale::S400)),
            (SemanticColor::Chart3, Color::amber(Scale::S500)),
            (SemanticColor::Chart4, Color::purple(Scale::S500)),
            (SemanticColor::Chart5, Color::rose(Scale::S500)),
            (SemanticColor::Sidebar, Color::gray(Scale::S900)),
            (SemanticColor::SidebarForeground, Color::gray(Scale::S50)),
            (SemanticColor::SidebarPrimary, Color::indigo(Scale::S500)),
            (
                SemanticColor::SidebarPrimaryForeground,
                Color::gray(Scale::S50),
            ),
            (SemanticColor::SidebarAccent, Color::gray(Scale::S800)),
            (
                SemanticColor::SidebarAccentForeground,
                Color::gray(Scale::S50),
            ),
            (SemanticColor::SidebarBorder, Color::gray(Scale::S700)),
            (SemanticColor::SidebarRing, Color::gray(Scale::S600)),
        ];

        Self {
            radius: "0.625rem",
            light: LIGHT,
            dark: DARK,
        }
    }

    pub fn to_css_variables(&self) -> String {
        let mut out = String::new();
        out.push_str(":root {\n");
        out.push_str(&format!("  --radius: {};\n", self.radius));
        for (token, color) in self.light {
            out.push_str(&format!("  --{}: {};\n", token.var_name(), color.to_css()));
        }
        out.push_str("}\n\n.dark {\n");
        for (token, color) in self.dark {
            out.push_str(&format!("  --{}: {};\n", token.var_name(), color.to_css()));
        }
        out.push_str("}\n");
        out
    }

    /// Resolve a semantic token to a concrete `Color`.
    ///
    /// `is_dark = false` resolves from light values, `true` from dark values.
    pub fn resolve(&self, token: SemanticColor, is_dark: bool) -> Option<Color> {
        let source = if is_dark { self.dark } else { self.light };
        source
            .iter()
            .find(|(t, _)| *t == token)
            .map(|(_, color)| *color)
    }

    /// Resolve a semantic token from the light palette.
    pub fn resolve_light(&self, token: SemanticColor) -> Option<Color> {
        self.resolve(token, false)
    }

    /// Resolve a semantic token from the dark palette.
    pub fn resolve_dark(&self, token: SemanticColor) -> Option<Color> {
        self.resolve(token, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_var_css() {
        assert_eq!(SemanticColor::Primary.to_css(), "var(--primary)");
    }

    #[test]
    fn test_semantic_theme_generation() {
        let css = SemanticThemeVars::shadcn_neutral().to_css_variables();
        assert!(css.contains("--background: #"));
        assert!(css.contains(".dark"));
    }

    #[test]
    fn test_semantic_resolve_color() {
        let theme = SemanticThemeVars::shadcn_neutral();
        let light_bg = theme.resolve_light(SemanticColor::Background).unwrap();
        let dark_bg = theme.resolve_dark(SemanticColor::Background).unwrap();
        assert_eq!(light_bg, Color::gray(Scale::S50));
        assert_eq!(dark_bg, Color::gray(Scale::S950));
    }
}
