use crate::Message;
use crate::components::Snippet;
use iced::Element;
use iced::widget::{column, text};
use twill::backends::iced::to_font_weight;
use twill::tokens::FontWeight;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("object-fit").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text(
            "Utilities for controlling how replaced content should be resized to fit its container."
        )
        .size(16),
        object_fit_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn object_fit_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Mapping ObjectFit bounds to iced ContentFit
to_content_fit(ObjectFit::Contain)"#;

    let blocks: iced::Element<'_, Message> =
        text("ObjectFit maps directly to `iced::ContentFit` for images.")
            .size(16)
            .into();

    Snippet::new("Object Fit", code, blocks).view(is_dark)
}
