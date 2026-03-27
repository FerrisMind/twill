use twill::prelude::*;

fn main() {
    let base = Style::new()
        .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
        .rounded(BorderRadius::Lg)
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            Color::slate(Scale::S300),
        );

    let emphasis = Style::new()
        .bg(Color::blue(Scale::S500))
        .text_color(Color::white())
        .shadow(Shadow::Md);

    let composed = base.with(emphasis);

    println!("== style builder ==");
    println!("padding: {:?}", composed.padding_value());
    println!("background: {:?}", composed.background_color_value());
    println!("text color: {:?}", composed.text_color_value());
    println!("radius: {:?}", composed.border_radius_value());
    println!("shadow: {:?}", composed.box_shadow_value());
}
