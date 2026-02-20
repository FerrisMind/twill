use crate::Message;
use crate::components::Snippet;
use iced::Element;
use iced::widget::{column, text};
use twill::backends::iced::{apply_layout, to_font_weight};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::Padding;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("max-width").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for setting the maximum width of an element.").size(16),
        max_width_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn max_width_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().max_w_prose();
// maps to max_width(520.0) constraint
apply_layout(content, &style)
"#;
    let content: Element<'_, Message> = text("This text container is constrained by max-width: prose (65ch). It won't grow infinitely wide, which is excellent for readability of long blocks of text in your application. Like this one, which goes on and on and on and on and on.")
        .size(16).into();

    let blocks: Element<'_, Message> = apply_layout(
        content,
        &Style::new()
            .max_w_prose()
            .bg(Color::gray(if is_dark { Scale::S800 } else { Scale::S100 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    Snippet::new("Max Width: Prose", code, blocks).view(is_dark)
}
