use twill::prelude::*;

fn main() {
    let palette = [
        ("brand", Color::blue(Scale::S500)),
        ("danger", Color::red(Scale::S600)),
        ("surface", Color::slate(Scale::S50)),
    ];
    let spacing = [Spacing::S2, Spacing::S4, Spacing::S8];
    let typography = (
        FontSize::Lg,
        FontWeight::SemiBold,
        LetterSpacing::em(0.035),
        LineHeight::number(1.75),
    );
    let shadow = Shadow::Lg;
    let custom_spacing = PaddingValue::var(PaddingVar::new("--panel-pad"));

    println!("== tokens ==");
    for (label, color) in palette {
        let value = color.compute();
        let (r, g, b, a) = value.to_rgba8();
        println!("{label}: rgba({r}, {g}, {b}, {a})");
    }

    for token in spacing {
        println!("{token:?}: {}px", token.to_px().unwrap_or_default());
    }

    println!(
        "font size: {:?}, weight: {:?}, tracking: {:?}, leading: {:?}",
        typography.0, typography.1, typography.2, typography.3
    );
    println!("custom spacing token: {:?}", custom_spacing);
    println!("shadow: {:?}", shadow);
}
