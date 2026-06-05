use twill::prelude::core::*;

fn button_primary() -> Style {
    Style::interactive()
        .bg(Color::blue(Scale::S600))
        .text_color(Color::white())
        .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
        .rounded(BorderRadius::Md)
        .hover(|style| style.bg(Color::blue(Scale::S700)))
        .disabled(|style| style.opacity(0.5))
}

fn card_surface() -> Style {
    Style::card()
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Xl)
}

fn input_field() -> Style {
    Style::surface()
        .bg(Color::white())
        .text_color(Color::slate(Scale::S900))
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            Color::slate(Scale::S300),
        )
        .rounded(BorderRadius::Md)
        .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)))
        .disabled(|style| style.opacity(0.55))
}

fn main() {
    let button = button_primary();
    let card = card_surface();
    let input = input_field();

    println!("== component recipes ==");
    println!("button padding: {:?}", button.padding_value());
    println!(
        "button hover bg: {:?}",
        button.hover_style().and_then(Style::background_color_value)
    );
    println!("card bg: {:?}", card.background_color_value());
    println!("card shadow: {:?}", card.box_shadow_value());
    println!("input border: {:?}", input.border_color_value());
    println!(
        "input focus ring: {:?}",
        input
            .focus_visible_style()
            .and_then(Style::ring_color_value)
    );
}
