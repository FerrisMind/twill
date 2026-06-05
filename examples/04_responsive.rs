use twill::prelude::core::*;

fn main() {
    let style = Style::new()
        .w(Spacing::S12)
        .at_sm(|style| style.w(Spacing::S24))
        .at_md(|style| style.padding(Padding::all(Spacing::S4)))
        .at_lg(|style| style.h(Spacing::S32))
        .at_xl(|style| style.shadow(Shadow::Lg))
        .at_2xl(|style| style.padding(Padding::all(Spacing::S6)));

    println!("== responsive layers ==");
    for breakpoint in [
        Breakpoint::Sm,
        Breakpoint::Md,
        Breakpoint::Lg,
        Breakpoint::Xl,
        Breakpoint::S2xl,
    ] {
        let resolved = style.at_breakpoint(breakpoint);
        println!(
            "{breakpoint:?}: width={:?}, height={:?}, padding={:?}, shadow={:?}",
            resolved.width_value(),
            resolved.height_value(),
            resolved.padding_value(),
            resolved.box_shadow_value()
        );
    }
}
