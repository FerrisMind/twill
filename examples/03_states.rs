use twill::prelude::core::*;

fn main() {
    let style = Style::new()
        .background_color(Color::slate(Scale::S100))
        .text_color(Color::slate(Scale::S900))
        .hover(|style| style.opacity(0.9))
        .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)))
        .disabled(|style| style.opacity(0.5))
        .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
        .aria_attr(AriaAttr::selected(true), |style| {
            style.font_weight(FontWeight::Bold)
        });

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
            .data_attr_style(DataState::Open)
            .and_then(Style::box_shadow_value)
    );
    println!(
        "aria[selected=true] weight: {:?}",
        style
            .aria_attr_style(AriaAttr::selected(true))
            .and_then(Style::font_weight_value)
    );
}
