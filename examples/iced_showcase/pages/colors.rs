use iced::widget::{column, row, text, Space};
use iced::{Element, Length};
use crate::components::Snippet;
use crate::Message;
use twill::tokens::{Color, ColorFamily, Scale, FontWeight, BorderRadius};
use twill::traits::ComputeValue;
use twill::iced::{to_color, to_font_weight, styled_container};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Using Twill tokens to generate all palettes
let blue_500 = Color::blue(Scale::S500);
let bg_color = to_color(blue_500);
"#;

    let families = [
        (ColorFamily::Slate, "Slate"),
        (ColorFamily::Gray, "Gray"),
        (ColorFamily::Zinc, "Zinc"),
        (ColorFamily::Neutral, "Neutral"),
        (ColorFamily::Stone, "Stone"),
        (ColorFamily::Red, "Red"),
        (ColorFamily::Orange, "Orange"),
        (ColorFamily::Amber, "Amber"),
        (ColorFamily::Yellow, "Yellow"),
        (ColorFamily::Lime, "Lime"),
        (ColorFamily::Green, "Green"),
        (ColorFamily::Emerald, "Emerald"),
        (ColorFamily::Teal, "Teal"),
        (ColorFamily::Cyan, "Cyan"),
        (ColorFamily::Sky, "Sky"),
        (ColorFamily::Blue, "Blue"),
        (ColorFamily::Indigo, "Indigo"),
        (ColorFamily::Violet, "Violet"),
        (ColorFamily::Purple, "Purple"),
        (ColorFamily::Fuchsia, "Fuchsia"),
        (ColorFamily::Pink, "Pink"),
        (ColorFamily::Rose, "Rose"),
    ];

    let scales = [
        (Scale::S50, "50"),
        (Scale::S100, "100"),
        (Scale::S200, "200"),
        (Scale::S300, "300"),
        (Scale::S400, "400"),
        (Scale::S500, "500"),
        (Scale::S600, "600"),
        (Scale::S700, "700"),
        (Scale::S800, "800"),
        (Scale::S900, "900"),
        (Scale::S950, "950"),
    ];

    let mut all_palettes = column![].spacing(24);

    for (family, name) in families.iter() {
        let mut row_content = row![].spacing(8);

        for (scale, label) in scales.iter() {
            let col = Color::new(*family, *scale);
            row_content = row_content.push(color_box(col, label));
        }

        all_palettes = all_palettes.push(
            column![
                text(*name).size(20).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
                Space::new().width(Length::Shrink).height(Length::Fixed(8.0)),
                row_content
            ]
        );
    }

    let snippet = Snippet::new("Tailwind Color Palettes (All)", code, all_palettes);

    column![
        text("Colors").size(32).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}

fn color_box<'a>(color: Color, label: &'a str) -> Element<'a, crate::Message> {
    let text_col = if (color.compute().r as u16 + color.compute().g as u16 + (color.compute().b as u16)) < 382 {
        to_color(Color::white())
    } else {
        to_color(Color::black())
    };

    column![
        styled_container(Space::new().width(Length::Fill).height(Length::Fill).into(), &twill::Style::new().bg(color).rounded(BorderRadius::Md))
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0)),
        Space::new().width(Length::Shrink).height(Length::Fixed(4.0)),
        text(label).size(12).color(text_col), // Fallback text col, but label is outside standard visual layout?
    ]
    .align_x(iced::Alignment::Center)
    .into()
}
