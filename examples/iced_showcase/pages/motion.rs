use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text};
use iced::{Element, Length};
use twill::iced::{styled_container, to_color, to_font_weight};
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, TransitionDuration};

pub fn view<'a>(is_dark: bool, elapsed: f64) -> Element<'a, Message> {
    let code = r#"// Example: drive animation state via elapsed time
let t = (elapsed as f32 * 2.0).sin();
let width = 24.0 + ((t + 1.0) * 0.5) * 180.0;"#;

    let t = (elapsed as f32 * 2.0).sin();
    let progress_width = 24.0 + ((t + 1.0) * 0.5) * 180.0;

    let durations = [
        TransitionDuration::Ms75,
        TransitionDuration::Ms150,
        TransitionDuration::Ms300,
        TransitionDuration::Ms500,
        TransitionDuration::Ms700,
        TransitionDuration::Ms1000,
    ];

    let mut token_column = column![].spacing(8);
    for duration in durations {
        token_column = token_column.push(
            text(format!("{} ({})", duration.value(), duration.as_millis()))
                .size(13)
                .color(if is_dark {
                    to_color(Color::gray(Scale::S300))
                } else {
                    to_color(Color::gray(Scale::S700))
                }),
        );
    }

    let demo = column![
        row![
            text("Animated Width").size(13),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text(format!("{:.0}px", progress_width)).size(13),
        ],
        styled_container(
            Space::new()
                .width(Length::Fixed(progress_width))
                .height(Length::Fixed(14.0))
                .into(),
            &twill::Style::new()
                .bg(Color::blue(Scale::S500))
                .rounded(BorderRadius::Md)
        ),
        text("Token Durations").size(14).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        token_column,
    ]
    .spacing(10);

    column![
        text("Motion").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Runtime animation preview + motion tokens.").size(16),
        Snippet::new("Motion Tokens", code, demo).view(is_dark),
    ]
    .spacing(32)
    .into()
}
