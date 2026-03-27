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
        .shadow(Shadow::Md)
        .px_px(18.0)
        .w_var(WidthVar::new("--card-width"))
        .min_h_var(HeightVar::new("--card-min-h"))
        .flex_arbitrary("2_1_auto")
        .grid_cols_arbitrary("200px_minmax(0,_1fr)");

    let composed = base.with(emphasis);

    println!("== style builder ==");
    println!("padding: {:?}", composed.padding_value());
    println!("background: {:?}", composed.background_color_value());
    println!("text color: {:?}", composed.text_color_value());
    println!("width: {:?}", composed.width_value());
    println!("constraints: {:?}", composed.constraints_value());
    println!("flex item: {:?}", composed.flex_item_value());
    println!(
        "grid columns: {:?}",
        composed
            .grid_container()
            .and_then(|grid| grid.columns_value())
    );
    println!("radius: {:?}", composed.border_radius_value());
    println!("shadow: {:?}", composed.box_shadow_value());
}
