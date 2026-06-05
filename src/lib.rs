//! # Twill
//!
//! Ergonomic facade crate for the Twill styling ecosystem.
//!
//! Reach for this crate when you want the backend-agnostic Twill core API together with
//! optional adapters for `egui`, `iced`, or `slint`.
//!
//! - Use [`prelude::core`] for the small day-to-day import surface.
//! - Add [`prelude::theme`], [`prelude::arbitrary`], and [`prelude::traits`] only as needed.
//! - Use [`prelude`] when you explicitly want the full power-user surface.
//! - Depend on `twill-core` directly when you only need the backend-agnostic style engine.
//! - Depend on `twill-egui`, `twill-iced`, or `twill-slint` directly when you want one adapter
//!   without the facade crate.

#![forbid(unsafe_code)]

pub use twill_core::{Style, prelude, style, tokens, traits, utilities};

#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "egui", feature = "iced", feature = "slint")))
)]
pub mod backends;

#[cfg(feature = "egui")]
#[cfg_attr(docsrs, doc(cfg(feature = "egui")))]
pub use backends::egui;

#[cfg(feature = "iced")]
#[cfg_attr(docsrs, doc(cfg(feature = "iced")))]
pub use backends::iced;

#[cfg(feature = "slint")]
#[cfg_attr(docsrs, doc(cfg(feature = "slint")))]
pub use backends::slint;

#[cfg(test)]
mod tests {
    use crate::prelude::core::*;
    use crate::tokens::BackgroundColor;

    #[test]
    fn test_basic_style() {
        let style = Style::new()
            .padding(Padding::all(Spacing::S4))
            .bg(Color::blue(Scale::S500))
            .rounded(BorderRadius::Md);

        assert_eq!(style.padding_value(), Some(&Padding::all(Spacing::S4)));
        assert_eq!(
            style.background_color_value(),
            Some(BackgroundColor::palette(Color::blue(Scale::S500)))
        );
        assert_eq!(style.border_radius_value(), Some(BorderRadius::Md));
    }

    #[test]
    fn test_flex_layout() {
        let style = Style::centered_col().gap(Spacing::S4);

        let flex = style
            .flex_container()
            .expect("flex container should be present");
        assert_eq!(flex.direction_value(), Some(FlexDirection::Col));
        assert_eq!(flex.justify_value(), Some(JustifyContent::Center));
        assert_eq!(flex.align_value(), Some(AlignItems::Center));
        assert_eq!(flex.gap_value(), Some(Spacing::S4));
    }
}
