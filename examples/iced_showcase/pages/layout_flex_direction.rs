use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, text};
use iced::{Element, Length};
use twill::backends::iced::{flex_direction_layout, styled_container, to_color, to_font_weight};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{FlexDirection, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("flex-direction").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for controlling the direction of flex items.").size(16),
        classes_section(is_dark),
        row_section(is_dark),
        row_reverse_section(is_dark),
        col_section(is_dark),
        col_reverse_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn classes_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Tailwind classes covered in Twill:
// flex-row
// flex-row-reverse
// flex-col
// flex-col-reverse

let dir = twill::FlexDirection::from_tailwind_class("flex-col-reverse");"#;

    let visual =
        text("All classes from Tailwind `flex-direction.mdx` are modeled explicitly.").size(13);

    Snippet::new("Flex Direction: Class Coverage", code, visual).view(is_dark)
}

fn row_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let items = vec![tile("01"), tile("02"), tile("03")];
let layout = flex_direction_layout(items, FlexDirection::Row, Spacing::S4); // flex-row"#;

    let visual = preview_surface(
        flex_direction_layout(
            vec![
                tile("01", Color::fuchsia(Scale::S500), is_dark),
                tile("02", Color::fuchsia(Scale::S500), is_dark),
                tile("03", Color::fuchsia(Scale::S500), is_dark),
            ],
            FlexDirection::Row,
            Spacing::S4,
        ),
        is_dark,
    );

    Snippet::new("Row (`flex-row`)", code, visual).view(is_dark)
}

fn row_reverse_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let items = vec![tile("01"), tile("02"), tile("03")];
let layout = flex_direction_layout(items, FlexDirection::RowReverse, Spacing::S4); // flex-row-reverse"#;

    let visual = preview_surface(
        flex_direction_layout(
            vec![
                tile("01", Color::blue(Scale::S500), is_dark),
                tile("02", Color::blue(Scale::S500), is_dark),
                tile("03", Color::blue(Scale::S500), is_dark),
            ],
            FlexDirection::RowReverse,
            Spacing::S4,
        ),
        is_dark,
    );

    Snippet::new("Row Reverse (`flex-row-reverse`)", code, visual).view(is_dark)
}

fn col_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let items = vec![tile("01"), tile("02"), tile("03")];
let layout = flex_direction_layout(items, FlexDirection::Col, Spacing::S4); // flex-col"#;

    let visual = preview_surface(
        container(flex_direction_layout(
            vec![
                tile("01", Color::indigo(Scale::S500), is_dark),
                tile("02", Color::indigo(Scale::S500), is_dark),
                tile("03", Color::indigo(Scale::S500), is_dark),
            ],
            FlexDirection::Col,
            Spacing::S4,
        ))
        .width(Length::Fixed(240.0)),
        is_dark,
    );

    Snippet::new("Column (`flex-col`)", code, visual).view(is_dark)
}

fn col_reverse_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let items = vec![tile("01"), tile("02"), tile("03")];
let layout = flex_direction_layout(items, FlexDirection::ColReverse, Spacing::S4); // flex-col-reverse"#;

    let visual = preview_surface(
        container(flex_direction_layout(
            vec![
                tile("01", Color::purple(Scale::S500), is_dark),
                tile("02", Color::purple(Scale::S500), is_dark),
                tile("03", Color::purple(Scale::S500), is_dark),
            ],
            FlexDirection::ColReverse,
            Spacing::S4,
        ))
        .width(Length::Fixed(240.0)),
        is_dark,
    );

    Snippet::new("Column Reverse (`flex-col-reverse`)", code, visual).view(is_dark)
}

fn preview_surface<'a>(
    content: impl Into<Element<'a, Message>>,
    is_dark: bool,
) -> Element<'a, Message> {
    styled_container(
        content.into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    )
    .width(Length::Fill)
    .into()
}

fn tile<'a>(label: &'a str, bg: Color, is_dark: bool) -> Element<'a, Message> {
    let text_color = if is_dark {
        to_color(Color::gray(Scale::S100))
    } else {
        to_color(Color::white())
    };

    styled_container(
        container(text(label).size(14).color(text_color))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fixed(56.0))
            .height(Length::Fixed(56.0))
            .into(),
        &Style::new()
            .bg(bg)
            .rounded(BorderRadius::Md)
            .padding(Padding::all(Spacing::S2)),
    )
    .into()
}
