use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, text};
use iced::{Element, Length};
use twill::backends::iced::{apply_layout, styled_container, to_font_weight};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::Padding;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("overflow").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for controlling how an element handles overflow content.").size(16),
        overflow_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn overflow_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().overflow_auto();
// applies iced::widget::scrollable() when overflowing
apply_layout(content, &style)
"#;
    let text_content: Element<'_, Message> = column![
        text("Line 1").size(16),
        text("Line 2").size(16),
        text("Line 3").size(16),
        text("Line 4").size(16),
        text("Line 5").size(16),
        text("Line 6").size(16),
    ]
    .spacing(8)
    .into();

    let scrollable_container = apply_layout(text_content, &Style::new().overflow_auto());

    let blocks = styled_container(
        scrollable_container,
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S800 } else { Scale::S100 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    )
    .height(Length::Fixed(100.0));

    Snippet::new("Overflow: Auto", code, blocks).view(is_dark)
}
