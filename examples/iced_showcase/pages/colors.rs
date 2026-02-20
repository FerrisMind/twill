use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text};
use iced::{Element, Length};
use twill::iced::{styled_container, to_color, to_font_weight};
use twill::tokens::{BorderRadius, Color, ColorFamily, FontWeight, Scale};
use twill::traits::ComputeValue;

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
        (ColorFamily::Mauve, "Mauve"),
        (ColorFamily::Olive, "Olive"),
        (ColorFamily::Mist, "Mist"),
        (ColorFamily::Taupe, "Taupe"),
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
            row_content = row_content.push(color_box(col, label, is_dark));
        }

        all_palettes = all_palettes.push(column![
            text(*name).size(20).font(iced::Font {
                weight: to_font_weight(FontWeight::Bold),
                ..Default::default()
            }),
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(8.0)),
            row_content
        ]);
    }

    let snippet = Snippet::new("Tailwind Color Palettes (All)", code, all_palettes);

    column![
        text("Colors").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}

fn color_box<'a>(color: Color, label: &'a str, is_dark: bool) -> Element<'a, crate::Message> {
    let chip_text_color = if is_light_color(color) {
        to_color(Color::slate(Scale::S900))
    } else {
        to_color(Color::white())
    };
    let label_text_color = if is_dark {
        to_color(Color::gray(Scale::S200))
    } else {
        to_color(Color::gray(Scale::S700))
    };

    column![
        styled_container(
            text(label).size(11).color(chip_text_color).into(),
            &twill::Style::new().bg(color).rounded(BorderRadius::Md)
        )
        .width(Length::Fixed(48.0))
        .height(Length::Fixed(48.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center),
        Space::new()
            .width(Length::Shrink)
            .height(Length::Fixed(4.0)),
        text(label).size(12).color(label_text_color),
    ]
    .align_x(iced::Alignment::Center)
    .into()
}

fn is_light_color(color: Color) -> bool {
    let c = color.compute();
    let luminance = (0.299 * c.r as f32) + (0.587 * c.g as f32) + (0.114 * c.b as f32);
    luminance > 150.0
}
