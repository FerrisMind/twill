#[cfg(feature = "iced")]
mod common;

#[cfg(feature = "iced")]
use common::{interactive_style, surface_style};
#[cfg(feature = "iced")]
use twill::backends::iced::{self, ToIced};
#[cfg(feature = "iced")]
use twill::prelude::*;
#[cfg(feature = "iced")]
use twill::tokens::Cursor;

#[cfg(feature = "iced")]
fn main() {
    let color = Color::blue(Scale::S500).to_iced();
    let padding = Spacing::S4.to_iced();
    let semantic = iced::to_semantic_color(SemanticColor::Primary, ThemeVariant::Dark);
    let cursor = Cursor::Pointer.to_iced();
    let shadow = interactive_style()
        .data_state_style("state=open")
        .and_then(Style::box_shadow_value)
        .map(|value| iced::to_shadow_with_color(value, twill::backends::ShadowColor::Default));

    println!("== iced adapter ==");
    println!("surface bg: {:?}", surface_style().background_color_value());
    println!("brand color: {:?}", color);
    println!("padding: {:?}", padding);
    println!("semantic primary (dark): {:?}", semantic);
    println!("cursor: {:?}", cursor);
    println!("shadow: {:?}", shadow);
}

#[cfg(not(feature = "iced"))]
fn main() {}
