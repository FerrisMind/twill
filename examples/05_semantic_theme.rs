use twill::prelude::{arbitrary::*, core::*, theme::*};

fn main() {
    let theme = SemanticThemeVars::shadcn_neutral();
    let brand = DynamicSemanticTheme::from_brand_oklch(0.628, 0.258, 29.234);
    let semantic_style = Style::card()
        .text_color_token(TextColor::semantic(SemanticColor::Primary))
        .ring_token(RingWidth::S2, RingColor::semantic(SemanticColor::Ring));
    let resolved_style = semantic_style.resolved_dark_theme(theme);

    println!("== semantic theme ==");
    for token in [
        SemanticColor::Background,
        SemanticColor::Foreground,
        SemanticColor::Primary,
        SemanticColor::PrimaryForeground,
        SemanticColor::Border,
        SemanticColor::Ring,
    ] {
        println!(
            "{token}: light={:?}, dark={:?}",
            theme.resolve_light(token),
            theme.resolve_dark(token)
        );
    }

    println!(
        "dynamic brand primary: light={:?}, dark={:?}",
        brand.resolve_light(SemanticColor::Primary),
        brand.resolve_dark(SemanticColor::Primary)
    );
    println!(
        "resolved style: text={:?}, ring={:?}",
        resolved_style.text_color_token_value(),
        resolved_style.ring_color_token_value()
    );
}
