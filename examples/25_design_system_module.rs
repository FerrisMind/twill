use twill::prelude::core::*;

mod theme {
    use twill::prelude::core::*;

    pub fn surface_bg() -> Color {
        Color::slate(Scale::S50)
    }

    pub fn surface_fg() -> Color {
        Color::slate(Scale::S900)
    }

    pub fn accent() -> Color {
        Color::blue(Scale::S600)
    }

    pub fn accent_hover() -> Color {
        Color::blue(Scale::S700)
    }

    pub fn danger() -> Color {
        Color::rose(Scale::S600)
    }
}

mod styles {
    use super::theme;
    use twill::prelude::core::*;

    pub fn surface() -> Style {
        Style::surface()
            .bg(theme::surface_bg())
            .text_color(theme::surface_fg())
            .rounded(BorderRadius::Lg)
    }

    pub fn button_primary() -> Style {
        Style::interactive()
            .bg(theme::accent())
            .text_color(Color::white())
            .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
            .rounded(BorderRadius::Md)
            .hover(|style| style.bg(theme::accent_hover()))
    }

    pub fn danger_text() -> Style {
        Style::new().text_color(theme::danger())
    }

    pub fn settings_section() -> Style {
        surface()
            .padding(Padding::all(Spacing::S4))
            .at_lg(|style| style.padding(Padding::all(Spacing::S6)))
    }
}

fn main() {
    let panel = styles::settings_section();
    let cta = styles::button_primary();
    let warning = styles::danger_text();

    println!("== design system module ==");
    println!("panel bg: {:?}", panel.background_color_value());
    println!(
        "panel responsive lg: {:?}",
        panel.at_breakpoint(Breakpoint::Lg).padding_value()
    );
    println!(
        "button hover bg: {:?}",
        cta.hover_style().and_then(Style::background_color_value)
    );
    println!("button cursor: {:?}", cta.cursor_value());
    println!("warning text: {:?}", warning.text_color_value());
}
