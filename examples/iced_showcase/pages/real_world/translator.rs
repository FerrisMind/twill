use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text, text_input};
use iced::{Alignment, Element, Length};
use twill::iced::{primary_button, styled_container, to_color, to_font_weight, to_padding};
use twill::tokens::{
    BorderRadius, BorderStyle, BorderWidth, Color, FontWeight, Scale, SemanticColor,
    SemanticThemeVars, Spacing,
};
use twill::utilities::Padding;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// A realistic application layout
let header = row![
    text("DeepL Clone").size(24),
    Space::with_width(Length::Fill),
    primary_button("Login", Message::NoOp),
];

let input_area = text_input("Type to translate...", "");
let output_area = text("Translation will appear here...");

column![header, row![input_area, output_area]]"#;

    let theme = SemanticThemeVars::shadcn_neutral();
    let border_color = theme.resolve(SemanticColor::Border, is_dark).unwrap();
    let bg_color = theme.resolve(SemanticColor::Background, is_dark).unwrap();
    let panel_bg = if is_dark {
        Color::gray(Scale::S900)
    } else {
        Color::gray(Scale::S50)
    };

    // Build the visual UI

    let header = styled_container(
        row![
            text("DeepTranslator").size(20).font(iced::Font {
                weight: to_font_weight(FontWeight::Bold),
                ..Default::default()
            }),
            Space::new().width(Length::Fill).height(Length::Shrink),
            primary_button("Pro version", Message::NoOp),
        ]
        .align_y(Alignment::Center)
        .into(),
        &twill::Style::new()
            .border(BorderWidth::S1, BorderStyle::Solid, border_color)
            .padding(Padding::all(Spacing::S4)),
    );

    let source_panel = styled_container(
        column![
            row![
                text("English").size(14).font(iced::Font {
                    weight: to_font_weight(FontWeight::Bold),
                    ..Default::default()
                }),
                Space::new().width(Length::Fill).height(Length::Shrink),
            ],
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(16.0)),
            text_input("Type to translate...", "").padding(12),
        ]
        .into(),
        &twill::Style::new()
            .bg(panel_bg)
            .border(BorderWidth::S1, BorderStyle::Solid, border_color)
            .rounded(BorderRadius::Lg)
            .padding(Padding::all(Spacing::S4)),
    )
    .width(Length::FillPortion(1))
    .height(Length::Fixed(300.0));

    let target_panel = styled_container(
        column![
            row![
                text("Russian").size(14).font(iced::Font {
                    weight: to_font_weight(FontWeight::Bold),
                    ..Default::default()
                }),
                Space::new().width(Length::Fill).height(Length::Shrink),
            ],
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(16.0)),
            text("Type something on the left to see translation...")
                .size(16)
                .color(if is_dark {
                    to_color(Color::gray(Scale::S500))
                } else {
                    to_color(Color::gray(Scale::S400))
                }),
        ]
        .into(),
        &twill::Style::new()
            .bg(panel_bg)
            .border(BorderWidth::S1, BorderStyle::Solid, border_color)
            .rounded(BorderRadius::Lg)
            .padding(Padding::all(Spacing::S4)),
    )
    .width(Length::FillPortion(1))
    .height(Length::Fixed(300.0));

    let main_content = row![source_panel, target_panel]
        .spacing(16)
        .padding(to_padding(Spacing::S4));

    let visual = styled_container(
        column![header, main_content].into(),
        &twill::Style::new()
            .bg(bg_color)
            .border(BorderWidth::S1, BorderStyle::Solid, border_color)
            .rounded(BorderRadius::Lg),
    )
    .width(Length::Fill);

    let snippet = Snippet::new("Translator Clone Layout", code, visual);

    column![
        text("Real World Layouts").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}
