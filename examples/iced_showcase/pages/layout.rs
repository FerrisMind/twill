use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, slider, text};
use iced::{Alignment, Element, Length};
use twill::backends::iced::{columns_layout, styled_container, to_font_weight};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, Container, FontWeight, Scale, Spacing};
use twill::utilities::Padding;

pub fn view<'a>(is_dark: bool, preview_width: f32) -> Element<'a, Message> {
    column![
        text("columns").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for controlling the number of columns within an element.").size(16),
        text("Use the width slider in interactive examples to test responsive column behavior.")
            .size(13),
        columns_count_section(is_dark),
        columns_width_section(is_dark, preview_width),
        columns_max_count_section(is_dark, preview_width),
        columns_gap_section(is_dark),
        columns_auto_section(is_dark),
        columns_custom_value_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn columns_count_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .columns_count(3)
    .column_gap(Spacing::S8);

let preview = columns_layout(items, &style);"#;

    let style = Style::new().columns_count(3).column_gap(Spacing::S8);

    let blocks = styled_container(
        columns_layout(sample_cards(is_dark), &style),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    Snippet::new("Columns: Count", code, blocks).view(is_dark)
}

fn columns_width_section<'a>(is_dark: bool, preview_width: f32) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .columns_width(Container::S3xs)
    .column_gap(Spacing::S4);

let preview = columns_layout(items, &style);"#;

    let style = Style::new()
        .columns_width(Container::S3xs)
        .column_gap(Spacing::S4);

    let blocks = styled_container(
        columns_layout(sample_cards(is_dark), &style),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    let visual = column![
        columns_width_control(preview_width),
        container(blocks).width(Length::Fixed(preview_width.clamp(280.0, 920.0))),
    ]
    .spacing(12);

    Snippet::new("Columns: Width (Responsive)", code, visual).view(is_dark)
}

fn columns_gap_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .columns_count(3)
    .column_gap(Spacing::S10);

let preview = columns_layout(items, &style);"#;

    let style = Style::new().columns_count(3).column_gap(Spacing::S10);

    let blocks = styled_container(
        columns_layout(sample_cards(is_dark), &style),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    Snippet::new("Columns: Gap", code, blocks).view(is_dark)
}

fn columns_max_count_section<'a>(is_dark: bool, preview_width: f32) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .columns_width(Container::S3xs)
    .columns_max_count(2)
    .column_gap(Spacing::S4);

let preview = columns_layout(items, &style);"#;

    let style = Style::new()
        .columns_width(Container::S3xs)
        .columns_max_count(2)
        .column_gap(Spacing::S4);

    let blocks = styled_container(
        columns_layout(sample_cards(is_dark), &style),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    let visual = column![
        columns_width_control(preview_width),
        container(blocks).width(Length::Fixed(preview_width.clamp(280.0, 920.0))),
    ]
    .spacing(12);

    Snippet::new("Columns: Max Count", code, visual).view(is_dark)
}

fn columns_auto_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .columns_auto()
    .column_gap(Spacing::S4);

let preview = columns_layout(items, &style);"#;

    let style = Style::new().columns_auto().column_gap(Spacing::S4);

    let blocks = styled_container(
        columns_layout(sample_cards(is_dark), &style),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    Snippet::new("Columns: Auto", code, blocks).view(is_dark)
}

fn columns_custom_value_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .columns_width_px(280.0)
    .column_gap(Spacing::S4);

let preview = columns_layout(items, &style);"#;

    let style = Style::new().columns_width_px(280.0).column_gap(Spacing::S4);

    let blocks = styled_container(
        columns_layout(sample_cards(is_dark), &style),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    );

    Snippet::new("Columns: Custom Value", code, blocks).view(is_dark)
}

fn columns_width_control<'a>(preview_width: f32) -> Element<'a, Message> {
    row![
        text(format!("Preview width: {:.0}px", preview_width)).size(13),
        slider(
            280.0..=920.0,
            preview_width.clamp(280.0, 920.0),
            Message::ColumnsPreviewWidthChanged,
        )
        .step(10.0)
        .width(Length::Fill),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn sample_cards<'a>(is_dark: bool) -> Vec<Element<'a, Message>> {
    let entries = [
        ("Card 01", 1_u8),
        ("Card 02", 3_u8),
        ("Card 03", 2_u8),
        ("Card 04", 5_u8),
        ("Card 05", 2_u8),
        ("Card 06", 4_u8),
        ("Card 07", 1_u8),
        ("Card 08", 3_u8),
        ("Card 09", 6_u8),
    ];

    entries
        .into_iter()
        .map(|(title, lines)| sample_card(title, lines, is_dark))
        .collect()
}

fn sample_card<'a>(title: &'a str, lines: u8, is_dark: bool) -> Element<'a, Message> {
    let mut body = column![text(title).size(14).font(iced::Font {
        weight: to_font_weight(FontWeight::SemiBold),
        ..Default::default()
    })]
    .spacing(6);

    for idx in 0..lines {
        body = body.push(
            text(format!("Content line {}", idx + 1))
                .size(12)
                .color(if is_dark {
                    twill::backends::iced::to_color(Color::gray(Scale::S300))
                } else {
                    twill::backends::iced::to_color(Color::gray(Scale::S700))
                }),
        );
    }

    styled_container(
        body.into(),
        &Style::new()
            .bg(if is_dark {
                Color::gray(Scale::S800)
            } else {
                Color::white()
            })
            .padding(Padding::all(Spacing::S3))
            .rounded(BorderRadius::Md)
            .border(
                twill::tokens::BorderWidth::S1,
                twill::tokens::BorderStyle::Solid,
                if is_dark {
                    Color::gray(Scale::S700)
                } else {
                    Color::gray(Scale::S300)
                },
            ),
    )
    .width(Length::Fill)
    .into()
}
