use twill::prelude::*;

fn main() {
    let style = Style::new()
        .bg(Color::slate(Scale::S100))
        .text_color(Color::slate(Scale::S900))
        .hover(|style| style.opacity(0.9))
        .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)))
        .disabled(|style| style.opacity(0.5))
        .data_state("state=open", |style| style.shadow(Shadow::Lg))
        .aria_state("selected", |style| style.font_weight(FontWeight::Bold));

    println!("== state layers ==");
    println!("hover: {:?}", style.hover_style().map(Style::opacity_value));
    println!(
        "focus-visible ring: {:?}",
        style
            .focus_visible_style()
            .and_then(Style::ring_color_value)
    );
    println!(
        "disabled opacity: {:?}",
        style.disabled_style().and_then(Style::opacity_value)
    );
    println!(
        "data[state=open] shadow: {:?}",
        style
            .data_state_style("state=open")
            .and_then(Style::box_shadow_value)
    );
    println!(
        "aria[selected] weight: {:?}",
        style
            .aria_state_style("selected")
            .and_then(Style::font_weight_value)
    );
}
