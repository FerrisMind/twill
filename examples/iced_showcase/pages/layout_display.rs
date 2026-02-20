use crate::Message;
use crate::components::Snippet;
use iced::Element;
use iced::widget::{column, text};
use twill::backends::iced::{apply_layout, to_font_weight};
use twill::style::Style;
use twill::tokens::FontWeight;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("display").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for controlling the display box type of an element.").size(16),
        display_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn display_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style_hidden = Style::hidden();
// Returns iced::widget::Space::new()
apply_layout(content, &style_hidden)
"#;
    let blocks: Element<'_, Message> = column![
        Element::from(
            text("The next element is hidden using twill Style Display::Hidden:").size(16)
        ),
        apply_layout(
            Element::from(text("You shouldn't see me!")),
            &Style::hidden(),
        ),
        Element::from(text("Above is the hidden element ^").size(16)),
    ]
    .spacing(16)
    .into();

    Snippet::new("Display: Hidden", code, blocks).view(is_dark)
}
