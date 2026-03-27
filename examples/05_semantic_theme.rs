use twill::prelude::*;

fn main() {
    let theme = SemanticThemeVars::shadcn_neutral();
    let brand = DynamicSemanticTheme::from_brand_oklch(0.628, 0.258, 29.234);

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
}
