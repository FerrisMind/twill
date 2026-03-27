#[cfg(feature = "slint")]
mod common;

#[cfg(feature = "slint")]
use common::interactive_style;
#[cfg(feature = "slint")]
use twill::backends::slint::{self, ToSlint};
#[cfg(feature = "slint")]
use twill::prelude::*;
#[cfg(feature = "slint")]
use twill::tokens::Cursor;

#[cfg(feature = "slint")]
fn main() {
    let color = Color::emerald(Scale::S500).to_slint();
    let arbitrary =
        slint::to_slint_color_value(ColorValueToken::from_rgba8(56, 189, 248, 180).into());
    let spacing = Spacing::S4.to_slint();
    let radius = BorderRadius::Lg.to_slint();
    let semantic = slint::to_semantic_color(SemanticColor::Primary, ThemeVariant::Dark);
    let cursor = Cursor::Pointer.to_slint();
    let shadow = interactive_style()
        .data_state_style("state=open")
        .and_then(Style::box_shadow_value)
        .map(slint::to_shadow);

    println!("== slint adapter ==");
    println!("brand color: {:?}", color);
    println!("arbitrary color: {:?}", arbitrary);
    println!("spacing: {:?}", spacing);
    println!("radius: {:?}", radius);
    println!("semantic primary (dark): {:?}", semantic);
    println!("cursor: {:?}", cursor.twill_cursor());
    println!("shadow: {:?}", shadow);
}

#[cfg(not(feature = "slint"))]
fn main() {}
