use twill::prelude::core::*;

fn panel_base() -> Style {
    Style::card()
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Xl)
}

fn panel_brand_variant() -> Style {
    Style::new()
        .bg(Color::blue(Scale::S600))
        .text_color(Color::white())
        .shadow(Shadow::Md)
}

fn panel_interactions() -> Style {
    Style::interactive()
        .hover(|style| style.opacity(0.95))
        .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
}

fn panel_responsive() -> Style {
    Style::new()
        .w(Spacing::S24)
        .at_md(|style| style.w(Spacing::S32))
        .at_lg(|style| style.padding(Padding::all(Spacing::S6)))
}

fn main() {
    let panel = panel_base()
        .merged(panel_brand_variant())
        .merged(panel_interactions())
        .merged(panel_responsive());

    println!("== style composition ==");
    println!("base bg: {:?}", panel.background_color_value());
    println!("base text: {:?}", panel.text_color_value());
    println!(
        "open shadow: {:?}",
        panel
            .data_attr_style(DataState::Open)
            .and_then(Style::box_shadow_value)
    );

    for breakpoint in [Breakpoint::Sm, Breakpoint::Md, Breakpoint::Lg] {
        let resolved = panel.at_breakpoint(breakpoint);
        println!(
            "{breakpoint:?}: width={:?}, padding={:?}, shadow={:?}",
            resolved.width_value(),
            resolved.padding_value(),
            resolved.box_shadow_value()
        );
    }
}
