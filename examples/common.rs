#![allow(dead_code)]

use twill::prelude::{core::*, theme::*};

#[derive(Debug, Clone)]
pub struct ShowcaseSection {
    pub title: &'static str,
    pub description: &'static str,
    pub style: Style,
}

pub fn token_palette() -> [(Color, &'static str); 4] {
    [
        (Color::blue(Scale::S500), "Brand"),
        (Color::emerald(Scale::S500), "Success"),
        (Color::amber(Scale::S500), "Warning"),
        (Color::rose(Scale::S500), "Accent"),
    ]
}

pub fn surface_style() -> Style {
    Style::card().rounded(BorderRadius::Xl)
}

/// Card surface with concrete colors resolved for `variant`.
///
/// Prefer this in theme-toggling showcases: backend helpers that still default to
/// `ThemeVariant::Light` will otherwise keep cards white while label fallbacks go light.
pub fn themed_surface_style(variant: ThemeVariant) -> Style {
    let theme = SemanticThemeVars::shadcn_neutral();
    Style::new()
        .bg(theme
            .resolve(SemanticColor::Card, variant)
            .unwrap_or(Color::white()))
        .text_color(
            theme
                .resolve(SemanticColor::CardForeground, variant)
                .unwrap_or(Color::neutral(Scale::S950)),
        )
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Xl)
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            theme
                .resolve(SemanticColor::Border, variant)
                .unwrap_or(Color::neutral(Scale::S200)),
        )
}

pub fn composition_style() -> Style {
    composition_style_for(ThemeVariant::Light)
}

pub fn composition_style_for(variant: ThemeVariant) -> Style {
    themed_surface_style(variant)
}

pub fn interactive_style() -> Style {
    surface_style()
        .merged(Style::interactive())
        .bg(Color::blue(Scale::S500))
        .text_color(Color::white())
        .hover(|style| style.opacity(0.92))
        .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
        .aria_attr(AriaAttr::Selected, |style| {
            style.border(
                BorderWidth::S1,
                BorderStyle::Solid,
                Color::amber(Scale::S400),
            )
        })
}

pub fn responsive_style() -> Style {
    responsive_style_for(ThemeVariant::Light)
}

pub fn responsive_style_for(variant: ThemeVariant) -> Style {
    composition_style_for(variant)
        .padding(Padding::all(Spacing::S3))
        .w(Spacing::S24)
        .at_sm(|style| style.w(Spacing::S32))
        .at_md(|style| style.padding(Padding::all(Spacing::S4)))
        .at_lg(|style| style.w(Spacing::S40))
        .at_xl(|style| style.shadow(Shadow::Lg))
        .at_2xl(|style| style.padding(Padding::all(Spacing::S6)))
}

pub fn semantic_summary() -> Vec<(&'static str, SemanticColor)> {
    vec![
        ("Background", SemanticColor::Background),
        ("Foreground", SemanticColor::Foreground),
        ("Primary", SemanticColor::Primary),
        ("Primary Foreground", SemanticColor::PrimaryForeground),
        ("Border", SemanticColor::Border),
        ("Ring", SemanticColor::Ring),
    ]
}

/// How to paint a semantic token so showcase labels stay readable.
///
/// Returns `(chip_surface, label_ink)`.
///
/// `*Foreground` tokens are ink meant for a paired surface (Background / Primary).
/// Showing them as plain text on the page background makes them blend with the theme;
/// demos instead place them on that paired surface so they read as the theme opposite.
pub fn semantic_demo_chip(
    token: SemanticColor,
    variant: ThemeVariant,
) -> (SemanticColor, SemanticColor) {
    match token {
        SemanticColor::Foreground => (SemanticColor::Background, SemanticColor::Foreground),
        SemanticColor::PrimaryForeground => {
            (SemanticColor::Primary, SemanticColor::PrimaryForeground)
        }
        SemanticColor::Primary => (SemanticColor::Primary, SemanticColor::PrimaryForeground),
        SemanticColor::Background => (SemanticColor::Background, SemanticColor::Foreground),
        // Dark shadcn Border is near-white; use Background ink so the label stays opposite.
        SemanticColor::Border | SemanticColor::Ring => {
            let ink = match variant {
                ThemeVariant::Dark => SemanticColor::Background,
                ThemeVariant::Light => SemanticColor::Foreground,
            };
            (token, ink)
        }
        other => (other, SemanticColor::Foreground),
    }
}

pub fn showcase_sections() -> Vec<ShowcaseSection> {
    vec![
        ShowcaseSection {
            title: "Tokens",
            description: "Typed color, spacing, radius, and shadow values are the base layer.",
            style: surface_style(),
        },
        ShowcaseSection {
            title: "Style Composition",
            description: "Reusable surfaces stay in Style, not in framework-specific components.",
            style: composition_style(),
        },
        ShowcaseSection {
            title: "States",
            description: "Interactive layers live next to the base style through hover/focus/data/aria hooks.",
            style: interactive_style(),
        },
        ShowcaseSection {
            title: "Responsive",
            description: "Breakpoint layers resolve into concrete styles through Style::at_breakpoint.",
            style: responsive_style(),
        },
    ]
}
