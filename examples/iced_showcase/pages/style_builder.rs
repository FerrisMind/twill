use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text};
use iced::{Element, Length};
use twill::components::Button as TwillButton;
use twill::iced::{styled_container, to_color, to_font_weight, twill_button};
use twill::style::Style;
use twill::tokens::{
    BorderRadius, BorderStyle, BorderWidth, Color, FontWeight, Scale, Shadow, Spacing,
};
use twill::utilities::Padding;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("Style Builder & Components")
            .size(32)
            .font(iced::Font {
                weight: to_font_weight(FontWeight::Bold),
                ..Default::default()
            }),
        text("Twill's fluent Style API and iced backend integration.").size(16),
        styled_container_section(is_dark),
        twill_button_section(is_dark),
    ]
    .spacing(32)
    .into()
}

// ── Styled Container ────────────────────────────────────────────────

fn styled_container_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .bg(Color::blue(Scale::S500))
    .rounded(BorderRadius::Lg)
    .shadow(Shadow::Md)
    .padding(Padding::all(Spacing::S4));

styled_container(content, &style)"#;

    let text_col = if is_dark {
        to_color(Color::white())
    } else {
        to_color(Color::black())
    };

    // Card 1: Primary card
    let style1 = Style::new()
        .bg(Color::blue(Scale::S500))
        .rounded(BorderRadius::Lg)
        .shadow(Shadow::Md)
        .padding(Padding::all(Spacing::S4));

    let card1 = styled_container(
        text("Primary Card")
            .size(16)
            .color(to_color(Color::white()))
            .into(),
        &style1,
    );

    // Card 2: Subtle card
    let bg2 = if is_dark {
        Color::gray(Scale::S800)
    } else {
        Color::gray(Scale::S100)
    };
    let style2 = Style::new()
        .bg(bg2)
        .rounded(BorderRadius::Xl)
        .shadow(Shadow::Sm)
        .padding(Padding::all(Spacing::S6))
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            if is_dark {
                Color::gray(Scale::S700)
            } else {
                Color::gray(Scale::S300)
            },
        );

    let card2 = styled_container(
        column![
            text("Subtle Card").size(16).color(text_col),
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(4.0)),
            text("With border and shadow")
                .size(12)
                .color(to_color(Color::gray(Scale::S400))),
        ]
        .into(),
        &style2,
    );

    // Card 3: Danger card
    let style3 = Style::new()
        .bg(Color::red(Scale::S500))
        .rounded(BorderRadius::Md)
        .shadow_with_color(Shadow::Lg, Color::red(Scale::S900))
        .padding(Padding::symmetric(Spacing::S3, Spacing::S6));

    let card3 = styled_container(
        text("Danger Card")
            .size(16)
            .color(to_color(Color::white()))
            .into(),
        &style3,
    );

    // Card 4: Gradient-like card (emerald)
    let style4 = Style::new()
        .bg(Color::emerald(Scale::S600))
        .rounded(BorderRadius::S2xl)
        .shadow(Shadow::Xl)
        .padding(Padding::all(Spacing::S8));

    let card4 = styled_container(
        column![
            text("Success Card")
                .size(18)
                .color(to_color(Color::white()))
                .font(iced::Font {
                    weight: to_font_weight(FontWeight::Bold),
                    ..Default::default()
                }),
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(4.0)),
            text("Elevated with large shadow")
                .size(13)
                .color(to_color(Color::gray(Scale::S200))),
        ]
        .into(),
        &style4,
    );

    let cards = row![card1, card2, card3, card4].spacing(24);

    Snippet::new("styled_container()", code, cards).view(is_dark)
}

// ── Twill Button ────────────────────────────────────────────────────

fn twill_button_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Using twill::components::Button with twill_button()
let btn = TwillButton::primary().lg();
twill_button(&btn, "Primary", Message::NoOp)

let outline = TwillButton::outline().md();
twill_button(&outline, "Outline", Message::NoOp)"#;

    let btn_primary = TwillButton::primary().lg();
    let btn_secondary = TwillButton::secondary();
    let btn_outline = TwillButton::outline();
    let btn_ghost = TwillButton::ghost();
    let btn_danger = TwillButton::destructive();
    let btn_sm = TwillButton::primary().sm();

    let buttons = column![
        row![
            twill_button(&btn_primary, "Primary (lg)", Message::NoOp),
            twill_button(&btn_secondary, "Secondary", Message::NoOp),
            twill_button(&btn_outline, "Outline", Message::NoOp),
        ]
        .spacing(16),
        row![
            twill_button(&btn_ghost, "Ghost", Message::NoOp),
            twill_button(&btn_danger, "Danger", Message::NoOp),
            twill_button(&btn_sm, "Small Primary", Message::NoOp),
        ]
        .spacing(16),
    ]
    .spacing(16);

    Snippet::new("twill_button()", code, buttons).view(is_dark)
}
