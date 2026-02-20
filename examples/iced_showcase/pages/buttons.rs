use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};
use twill::components::Button as TwillButton;
use twill::iced::{to_font_weight, twill_button};
use twill::tokens::FontWeight;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("Buttons").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Twill buttons support 6 variants, 4 sizes, and full-width/disabled states.").size(16),
        variants_section(is_dark),
        sizes_section(is_dark),
        states_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let btn = TwillButton::primary();
twill_button(&btn, "Primary", Message::NoOp);"#;

    let btn_primary = TwillButton::primary();
    let btn_secondary = TwillButton::secondary();
    let btn_danger = TwillButton::destructive();
    let btn_outline = TwillButton::outline();
    let btn_ghost = TwillButton::ghost();
    let btn_link = TwillButton::link();

    let content = column![
        row![
            twill_button(&btn_primary, "Primary", Message::NoOp),
            twill_button(&btn_secondary, "Secondary", Message::NoOp),
            twill_button(&btn_danger, "Destructive", Message::NoOp),
        ]
        .spacing(12),
        row![
            twill_button(&btn_outline, "Outline", Message::NoOp),
            twill_button(&btn_ghost, "Ghost", Message::NoOp),
            twill_button(&btn_link, "Link", Message::NoOp),
        ]
        .spacing(12),
        text(if is_dark {
            "Ghost in dark mode should keep visible text and subtle hover background."
        } else {
            "Ghost in light mode should look minimal, but text must stay readable."
        })
        .size(12),
    ]
    .spacing(12);

    Snippet::new("Button Variants", code, content).view(is_dark)
}

fn sizes_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let sm = TwillButton::primary().sm();
let lg = TwillButton::primary().lg();
let icon = TwillButton::outline().icon();"#;

    let btn_sm = TwillButton::primary().sm();
    let btn_md = TwillButton::primary().md();
    let btn_lg = TwillButton::primary().lg();
    let btn_icon = TwillButton::outline().icon();

    let content = row![
        column![
            text("Small").size(12),
            twill_button(&btn_sm, "Button", Message::NoOp)
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center),
        column![
            text("Medium").size(12),
            twill_button(&btn_md, "Button", Message::NoOp)
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center),
        column![
            text("Large").size(12),
            twill_button(&btn_lg, "Button", Message::NoOp)
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center),
        column![
            text("Icon").size(12),
            twill_button(&btn_icon, "⌘", Message::NoOp)
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center),
    ]
    .spacing(20)
    .align_y(iced::Alignment::End);

    Snippet::new("Button Sizes", code, content).view(is_dark)
}

fn states_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let disabled = TwillButton::primary().disabled(true);
let full = TwillButton::secondary().full_width(true);"#;

    let mut btn_disabled = TwillButton::primary();
    btn_disabled.disabled = true;

    let mut btn_full = TwillButton::secondary();
    btn_full.full_width = true;

    // For full width, wrap in a fixed-width container for demo context
    let full_width_demo = container(twill_button(&btn_full, "Full Width Button", Message::NoOp))
        .width(Length::Fixed(300.0));

    let content = column![
        twill_button(&btn_disabled, "Disabled", Message::NoOp), // Actually making iced ignore presses requires dropping the message. We just demo styling.
        full_width_demo,
    ]
    .spacing(16);

    Snippet::new("States", code, content).view(is_dark)
}
