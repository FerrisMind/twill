use iced::widget::{column, text};
use iced::{Element, Length};
use crate::components::Snippet;
use crate::Message;
use twill::tokens::{Shadow, Color, Scale, FontWeight, BorderRadius};
use twill::iced::{to_color, to_font_weight, styled_container};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("Shadows").size(32).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        text("Box shadows from Twill, rendered with iced native shadow support.").size(16),
        box_shadow_section(is_dark),
    ]
    .spacing(32)
    .into()
}

// ── Box Shadows ─────────────────────────────────────────────────────

fn box_shadow_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let shadow = Shadow::Lg;
styled_container(text("Card").into(), &twill::Style::new()
    .shadow(shadow)
    .shadow_color(Color::black())
    .bg(iced::Color::WHITE))"#;

    let shadows = [
        (Shadow::None, "None"),
        (Shadow::Xs2, "Xs2"),
        (Shadow::Xs, "Xs"),
        (Shadow::Sm, "Sm"),
        (Shadow::Md, "Md"),
        (Shadow::Lg, "Lg"),
        (Shadow::Xl, "Xl"),
        (Shadow::S2xl, "2XL"),
    ];

    let _box_bg = if is_dark {
        to_color(Color::gray(Scale::S800))
    } else {
        to_color(Color::white())
    };
    let shadow_col = if is_dark { Color::black() } else { Color::gray(Scale::S900) };
    let text_col = if is_dark { to_color(Color::white()) } else { to_color(Color::black()) };

    let mut blocks = column![].spacing(32).padding(32);

    let mut current_row = iced::widget::row![].spacing(32);
    let mut count = 0;

    for (shadow, label) in shadows {
        let bg_color = if is_dark { Color::gray(Scale::S800) } else { Color::white() };

        let c = styled_container(
            text(label).size(14).color(text_col).into(),
            &twill::Style::new()
                .bg(bg_color)
                .rounded(BorderRadius::Lg)
                .shadow(shadow)
                .shadow_color(shadow_col)
        )
            .width(Length::Fixed(160.0))
            .height(Length::Fixed(90.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        current_row = current_row.push(c);
        count += 1;

        if count == 4 {
            blocks = blocks.push(current_row);
            current_row = iced::widget::row![].spacing(32);
            count = 0;
        }
    }

    if count > 0 {
        blocks = blocks.push(current_row);
    }

    Snippet::new("Box Shadow", code, blocks).view(is_dark)
}
