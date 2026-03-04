use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};
use twill::backends::iced::{
    gap_layout, gap_x_layout, gap_y_layout, styled_container, to_color, to_font_weight,
};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{FlexDirection, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("gap").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed spacing utilities for flex/grid gutters in Twill + iced.").size(16),
        variants_section(is_dark),
        gap_section(is_dark),
        gap_x_section(is_dark),
        gap_y_section(is_dark),
        axis_mapping_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .gap(Spacing::S4)      // gap-*
    .gap_x(Spacing::S8)    // gap-x-*
    .gap_y(Spacing::S2);   // gap-y-*"#;

    let visual =
        text("Gap families are represented by typed builder methods and spacing tokens.").size(13);

    Snippet::new("Gap: Variant Families", code, visual).view(is_dark)
}

fn gap_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let items = vec![tile("01"), tile("02"), tile("03")];
let preview = gap_layout(items, FlexDirection::Row, Spacing::S4);"#;

    let visual = preview_surface(
        gap_layout(
            vec![
                tile("01", Color::violet(Scale::S500), is_dark),
                tile("02", Color::violet(Scale::S500), is_dark),
                tile("03", Color::violet(Scale::S500), is_dark),
            ],
            FlexDirection::Row,
            Spacing::S4,
        ),
        is_dark,
    );

    Snippet::new("Gap", code, visual).view(is_dark)
}

fn gap_x_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().gap_x(Spacing::S8);
let preview = gap_x_layout(items, FlexDirection::Row, Spacing::S8);"#;

    let visual = preview_surface(
        gap_x_layout(
            vec![
                tile("01", Color::blue(Scale::S500), is_dark),
                tile("02", Color::blue(Scale::S500), is_dark),
                tile("03", Color::blue(Scale::S500), is_dark),
            ],
            FlexDirection::Row,
            Spacing::S8,
        ),
        is_dark,
    );

    Snippet::new("Gap X", code, visual).view(is_dark)
}

fn gap_y_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_col().gap_y(Spacing::S6);
let preview = gap_y_layout(items, FlexDirection::Col, Spacing::S6);"#;

    let visual = preview_surface(
        container(gap_y_layout(
            vec![
                tile("01", Color::indigo(Scale::S500), is_dark),
                tile("02", Color::indigo(Scale::S500), is_dark),
                tile("03", Color::indigo(Scale::S500), is_dark),
            ],
            FlexDirection::Col,
            Spacing::S6,
        ))
        .width(Length::Fixed(240.0)),
        is_dark,
    );

    Snippet::new("Gap Y", code, visual).view(is_dark)
}

fn axis_mapping_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Best-effort single-line flex mapping in iced:
// - Row / RowReverse -> gap-x on main axis
// - Col / ColReverse -> gap-y on main axis"#;

    let visual = row![
        column![
            text("Row + gap-x").size(12),
            preview_surface(
                gap_x_layout(
                    vec![
                        tile("01", Color::cyan(Scale::S500), is_dark),
                        tile("02", Color::cyan(Scale::S500), is_dark),
                        tile("03", Color::cyan(Scale::S500), is_dark),
                    ],
                    FlexDirection::Row,
                    Spacing::S6,
                ),
                is_dark
            ),
        ]
        .spacing(8)
        .width(Length::Fill),
        column![
            text("Col + gap-y").size(12),
            preview_surface(
                container(gap_y_layout(
                    vec![
                        tile("01", Color::emerald(Scale::S500), is_dark),
                        tile("02", Color::emerald(Scale::S500), is_dark),
                        tile("03", Color::emerald(Scale::S500), is_dark),
                    ],
                    FlexDirection::Col,
                    Spacing::S6,
                ))
                .width(Length::Fixed(180.0)),
                is_dark
            ),
        ]
        .spacing(8)
        .width(Length::Fill),
    ]
    .spacing(12);

    Snippet::new("Axis Mapping (iced)", code, visual).view(is_dark)
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
