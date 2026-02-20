use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text};
use iced::{Element, Length};
use twill::iced::{styled_container, to_color, to_font_weight};
use twill::tokens::{Color, FontWeight, Scale, Spacing};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Inspect scale values directly in pixels:
let px = Spacing::S24.to_px().unwrap(); // 96
"#;

    let spacings = [
        (Spacing::S0, "S0"),
        (Spacing::Px, "Px"),
        (Spacing::S1, "S1"),
        (Spacing::S1_5, "S1.5"),
        (Spacing::S2, "S2"),
        (Spacing::S2_5, "S2.5"),
        (Spacing::S3, "S3"),
        (Spacing::S3_5, "S3.5"),
        (Spacing::S4, "S4"),
        (Spacing::S6, "S6"),
        (Spacing::S8, "S8"),
        (Spacing::S10, "S10"),
        (Spacing::S12, "S12"),
        (Spacing::S14, "S14"),
        (Spacing::S16, "S16"),
        (Spacing::S20, "S20"),
        (Spacing::S24, "S24"),
        (Spacing::S32, "S32"),
        (Spacing::S48, "S48"),
        (Spacing::S64, "S64"),
        (Spacing::S96, "S96"),
    ];

    let mut blocks = column![].spacing(16);

    for (spacing, label) in spacings {
        let px = spacing.to_px().unwrap_or_default() as f32;
        let bar_width = (px * 1.5).clamp(1.0, 320.0);

        let sample = row![
            styled_container(
                Space::new()
                    .width(Length::Fixed(bar_width))
                    .height(Length::Fixed(12.0))
                    .into(),
                &twill::Style::new().bg(Color::blue(Scale::S500))
            )
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
            text(format!("{} px", px as i32))
                .size(13)
                .color(if is_dark {
                    to_color(Color::gray(Scale::S300))
                } else {
                    to_color(Color::gray(Scale::S700))
                })
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center);

        let card = styled_container(
            column![
                text(label).size(14).font(iced::Font {
                    weight: to_font_weight(FontWeight::Bold),
                    ..Default::default()
                }),
                sample
            ]
            .spacing(8)
            .into(),
            &twill::Style::new()
                .bg(if is_dark {
                    Color::gray(Scale::S900)
                } else {
                    Color::gray(Scale::S100)
                })
                .rounded(twill::tokens::BorderRadius::Md)
                .padding(twill::utilities::Padding::all(Spacing::S3)),
        )
        .width(Length::Fill);

        blocks = blocks.push(card);
    }

    let snippet = Snippet::new("Spacing / Padding", code, blocks);

    column![
        text("Spacing").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Twill spacing variables map perfectly directly into standard Iced padding.").size(16),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}
