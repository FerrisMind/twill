#[cfg(feature = "egui")]
mod common;

#[cfg(feature = "egui")]
use common::{interactive_style, surface_style};
#[cfg(feature = "egui")]
use twill::backends::egui::{self, ToEgui};
#[cfg(feature = "egui")]
use twill::prelude::*;
#[cfg(feature = "egui")]
use twill::tokens::Cursor;

#[cfg(feature = "egui")]
fn main() {
    let frame = surface_style().to_egui();
    let color = Color::blue(Scale::S500).to_egui();
    let semantic = egui::to_semantic_color32(SemanticColor::Primary, ThemeVariant::Dark);
    let cursor = Cursor::Pointer.to_egui();
    let shadow = interactive_style()
        .data_state_style("state=open")
        .and_then(Style::box_shadow_value)
        .and_then(|value| egui::to_shadow_with_color(value, twill::backends::ShadowColor::Default));

    println!("== egui adapter ==");
    println!("frame fill: {:?}", frame.fill);
    println!("brand color: {:?}", color);
    println!("semantic primary (dark): {:?}", semantic);
    println!("cursor: {:?}", cursor);
    println!("shadow: {:?}", shadow);
}

#[cfg(not(feature = "egui"))]
fn main() {}
