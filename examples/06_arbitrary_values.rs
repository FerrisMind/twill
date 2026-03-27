use twill::prelude::*;

fn main() {
    let style = Style::new()
        .bg_arbitrary(ColorValueToken::from_rgb8(15, 23, 42))
        .text_color_arbitrary(ColorValueToken::from_rgb8(248, 250, 252))
        .px_var(PaddingVar::new("--panel-pad-x"))
        .pt_px(12.0)
        .pb_rem(1.25)
        .mx_auto()
        .min_w_var(WidthVar::new("--panel-min-w"))
        .max_h_px(640)
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            Color::slate(Scale::S300),
        )
        .border_color_var(BorderColorVar::new("--panel-border"))
        .ring(RingWidth::S2, Color::blue(Scale::S400))
        .ring_color_arbitrary(ColorValueToken::from_rgba8(56, 189, 248, 180))
        .shadow(Shadow::Lg)
        .shadow_color_arbitrary(ColorValueToken::from_rgba8(15, 23, 42, 220))
        .text_size_px(18)
        .tracking_em(0.035)
        .leading_number(1.75)
        .blur_px(18)
        .perspective_px(960)
        .transition_custom("filter, transform")
        .transition_duration_ms(240)
        .transition_delay_ms(60);

    println!("== arbitrary/custom values ==");
    println!("background token: {:?}", style.background_color_value());
    println!("text token: {:?}", style.text_color_token_value());
    println!("padding: {:?}", style.padding_value());
    println!("margin: {:?}", style.margin_value());
    println!("constraints: {:?}", style.constraints_value());
    println!("border token: {:?}", style.border_color_token_value());
    println!("ring token: {:?}", style.ring_color_token_value());
    println!("shadow token: {:?}", style.shadow_color_token_value());
    println!("font size: {:?}", style.font_size_value());
    println!("tracking: {:?}", style.letter_spacing_value());
    println!("leading: {:?}", style.line_height_value());
    println!("blur: {:?}", style.blur_value());
    println!("perspective: {:?}", style.perspective_value());
    println!(
        "transition property: {:?}",
        style.transition_property_value()
    );
    println!(
        "transition duration: {:?}",
        style.transition_duration_value()
    );
    println!("transition delay: {:?}", style.transition_delay_value());
}
