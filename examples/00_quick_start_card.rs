use twill::prelude::core::*;

fn main() {
    let card = Style::card()
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Lg);

    let button = Style::interactive()
        .bg(Color::blue(Scale::S600))
        .text_color(Color::white())
        .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
        .rounded(BorderRadius::Md)
        .hover(|style| style.bg(Color::blue(Scale::S700)))
        .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)));

    println!("== quick start card ==");
    println!("card bg: {:?}", card.background_color_value());
    println!("card radius: {:?}", card.border_radius_value());
    println!("button bg: {:?}", button.background_color_value());
    println!("button text: {:?}", button.text_color_value());
    println!(
        "button focus ring: {:?}",
        button
            .focus_visible_style()
            .and_then(Style::ring_color_value)
    );
}
