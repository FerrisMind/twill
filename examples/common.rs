#![allow(dead_code)]

use twill::prelude::*;

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
    Style::new()
        .bg(Color::white())
        .text_color(Color::slate(Scale::S900))
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Xl)
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            Color::slate(Scale::S200),
        )
        .shadow(Shadow::Sm)
}

pub fn interactive_style() -> Style {
    surface_style()
        .bg(Color::blue(Scale::S500))
        .text_color(Color::white())
        .hover(|style| style.opacity(0.92))
        .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)))
        .disabled(|style| style.opacity(0.55))
        .data_state("state=open", |style| style.shadow(Shadow::Lg))
        .aria_state("selected", |style| {
            style.border(
                BorderWidth::S1,
                BorderStyle::Solid,
                Color::amber(Scale::S400),
            )
        })
}

pub fn responsive_style() -> Style {
    Style::new()
        .padding(Padding::all(Spacing::S3))
        .w(Spacing::S24)
        .sm(|style| style.w(Spacing::S32))
        .md(|style| style.padding(Padding::all(Spacing::S4)))
        .lg(|style| style.w(Spacing::S40))
        .xl(|style| style.shadow(Shadow::Lg))
        .s2xl(|style| style.padding(Padding::all(Spacing::S6)))
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
            style: surface_style().with(Style::new().bg(Color::slate(Scale::S50)).border(
                BorderWidth::S1,
                BorderStyle::Solid,
                Color::slate(Scale::S300),
            )),
        },
        ShowcaseSection {
            title: "States",
            description: "Interactive layers live next to the base style through hover/focus/data/aria hooks.",
            style: interactive_style(),
        },
        ShowcaseSection {
            title: "Responsive",
            description: "Breakpoint layers resolve into concrete styles through the Responsive trait.",
            style: responsive_style(),
        },
    ]
}
