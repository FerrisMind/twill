use iced::widget::{column, row, text};
use iced::{Element, Length};
use crate::components::Snippet;
use crate::Message;
use twill::tokens::{Spacing, Color, Scale, FontWeight};
use twill::iced::{to_padding, to_color, to_font_weight, styled_container};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Convert twill Spacing into iced Padding
let custom_padding = to_padding(Spacing::S8); // 8 * 0.25rem * 16px = 32px

container(text("Hello"))
    .padding(custom_padding)
"#;

    let spacings = [
        (Spacing::S0, "S0"),
        (Spacing::Px, "Px"),
        (Spacing::S1, "S1"),
        (Spacing::S2, "S2"),
        (Spacing::S4, "S4"),
        (Spacing::S8, "S8"),
        (Spacing::S12, "S12"),
        (Spacing::S16, "S16"),
        (Spacing::S24, "S24"),
        (Spacing::S32, "S32"),
        (Spacing::S48, "S48"),
        (Spacing::S64, "S64"),
        (Spacing::S96, "S96"),
    ];

    let mut blocks = column![].spacing(16);

    for (spacing, label) in spacings {
        let padding_val = to_padding(spacing);
        
        // Inner blue box 
        let inner = styled_container(text(label).size(12).color(to_color(Color::white())).into(), &twill::Style::new().bg(Color::blue(Scale::S500)))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // Outer red box showing the padding area
        let outer = styled_container(inner.into(), &twill::Style::new().bg(Color::red(Scale::S200)))
            .padding(padding_val)
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(120.0));
        
        let info = column![
            text(format!("{} ({}px)", label, padding_val.top)).size(14).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        ];

        blocks = blocks.push(row![outer, info].spacing(16).align_y(iced::Alignment::Center));
    }

    let snippet = Snippet::new("Spacing / Padding", code, blocks);

    column![
        text("Spacing").size(32).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        text("Twill spacing variables map perfectly directly into standard Iced padding.").size(16),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}
