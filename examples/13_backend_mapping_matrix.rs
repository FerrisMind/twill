#[cfg(all(feature = "egui", feature = "iced", feature = "slint"))]
use twill::backends::{
    egui::ToEgui,
    iced::{self as twill_iced, ToIced},
    slint::{self as twill_slint, ToSlint},
};
#[cfg(all(feature = "egui", feature = "iced", feature = "slint"))]
use twill::prelude::{core::*, theme::*};

#[cfg(all(feature = "egui", feature = "iced", feature = "slint"))]
fn main() {
    let style = Style::card()
        .merged(Style::interactive())
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Lg);
    let color = Color::blue(Scale::S500);
    let spacing = Spacing::S4;
    let radius = BorderRadius::Lg;
    let shadow = Shadow::Md;
    let semantic = SemanticColor::Primary;

    let egui_frame = style.to_egui();

    println!("== backend mapping matrix ==");
    println!("source style bg: {:?}", style.background_color_value());
    println!("source style text: {:?}", style.text_color_value());
    println!("source style shadow: {:?}", style.box_shadow_value());

    println!(
        "egui: color={:?}, padding={:?}, radius={:?}, shadow={:?}",
        color.to_egui(),
        egui_frame.inner_margin,
        radius.to_egui(),
        shadow.to_egui()
    );
    println!(
        "iced: color={:?}, padding={:?}, radius={:?}, shadow={:?}, semantic={:?}",
        color.to_iced(),
        spacing.to_iced(),
        radius.to_iced(),
        shadow.to_iced(),
        twill_iced::to_semantic_color(semantic, ThemeVariant::Light)
    );
    println!(
        "slint: color={:?}, spacing={:?}, radius={:?}, shadow={:?}, semantic={:?}",
        color.to_slint(),
        spacing.to_slint(),
        radius.to_slint(),
        twill_slint::to_shadow(shadow),
        twill_slint::to_semantic_color(semantic, ThemeVariant::Light)
    );
}

#[cfg(not(all(feature = "egui", feature = "iced", feature = "slint")))]
fn main() {}
