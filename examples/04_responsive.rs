use twill::prelude::*;

fn main() {
    let style = Style::new()
        .w(Spacing::S12)
        .sm(|style| style.w(Spacing::S24))
        .md(|style| style.padding(Padding::all(Spacing::S4)))
        .lg(|style| style.h(Spacing::S32))
        .xl(|style| style.shadow(Shadow::Lg))
        .s2xl(|style| style.padding(Padding::all(Spacing::S6)));

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
