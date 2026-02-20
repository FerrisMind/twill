use crate::Message;
use iced::widget::{Space, column, container, row, text};
use iced::{Element, Length};
use twill::ComputeValue;
use twill::tokens::{Color as TwillColor, ColorValue, Scale};

pub fn view<'a>(_is_dark: bool) -> Element<'a, Message> {
    // 1. Oklch "Dead zone" gradient demonstration
    // Let's go from Blue to Yellow.
    let start_rgb = ColorValue::from_rgb(0, 0, 255); // Pure Blue
    let end_rgb = ColorValue::from_rgb(255, 255, 0); // Pure Yellow

    // Let's generate 9 steps
    let steps = 9;

    // RGB Linear Interpretation (bad)
    let mut rgb_gradient_row = row![].spacing(0);
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let r = ((1.0 - t) * start_rgb.r as f32 + t * end_rgb.r as f32) as u8;
        let g = ((1.0 - t) * start_rgb.g as f32 + t * end_rgb.g as f32) as u8;
        let b = ((1.0 - t) * start_rgb.b as f32 + t * end_rgb.b as f32) as u8;

        rgb_gradient_row = rgb_gradient_row.push(
            container(Space::new().width(Length::Fill).height(Length::Fixed(40.0))).style(
                move |_| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb8(r, g, b))),
                    ..Default::default()
                },
            ),
        );
    }

    // OKLCH Perceptual Interpretation (good)
    let mut oklch_gradient_row = row![].spacing(0);
    let srgb_start = palette::Srgb::new(0.0, 0.0, 1.0);
    let srgb_end = palette::Srgb::new(1.0, 1.0, 0.0);

    use palette::FromColor;
    let oklch_start: palette::Oklch = palette::Oklch::from_color(srgb_start);
    let oklch_end: palette::Oklch = palette::Oklch::from_color(srgb_end);

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let l = (1.0 - t) * oklch_start.l + t * oklch_end.l;
        let c = (1.0 - t) * oklch_start.chroma + t * oklch_end.chroma;
        // Simple Hue interpolation (shortest path)
        let h1 = oklch_start.hue.into_inner();
        let h2 = oklch_end.hue.into_inner();
        let h = (1.0 - t) * h1 + t * h2;

        let (r, g, b) = twill::tokens::oklch::OklchConverter::to_rgb(l, c, h);

        oklch_gradient_row = oklch_gradient_row.push(
            container(Space::new().width(Length::Fill).height(Length::Fixed(40.0))).style(
                move |_| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb8(r, g, b))),
                    ..Default::default()
                },
            ),
        );
    }

    // 2. Darkening a color: RGB vs OKLCH
    let brand_red = TwillColor::red(Scale::S500).compute(); // Tailwind Red-500

    let bad_hover_rgb = ColorValue::new(
        (brand_red.r as f32 * 0.7) as u8,
        (brand_red.g as f32 * 0.7) as u8,
        (brand_red.b as f32 * 0.7) as u8,
        1.0,
    );
    let good_hover_oklch = brand_red.darken_oklch(0.15); // Preserves chroma

    let comparison_row = row![
        column![
            text("Raw RGB Darkness (-30%)").size(14),
            container(
                Space::new()
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(60.0)),
            )
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    bad_hover_rgb.r,
                    bad_hover_rgb.g,
                    bad_hover_rgb.b,
                ))),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
        ]
        .spacing(8),
        column![
            text("OKLCH Darkness (L - 0.15)").size(14),
            container(
                Space::new()
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(60.0)),
            )
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    good_hover_oklch.r,
                    good_hover_oklch.g,
                    good_hover_oklch.b,
                ))),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
        ]
        .spacing(8),
    ]
    .spacing(32);

    column![
        text("RGB vs OKLCH Demonstration").size(24),
        text("1. The Dead Zone: Gradient from Blue to Yellow").size(18),
        text("Linear RGB Interpolation (Notice the muddy/grey center)").size(14),
        rgb_gradient_row,
        text("Perceptual OKLCH Interpolation (Notice the vibrant transition)").size(14),
        oklch_gradient_row,
        text("-----------------------------------").size(24),
        text("2. Programmatic Darkening (e.g. for Hover states)").size(18),
        text("Notice how linear RGB darkening loses saturation/vibrancy, while OKLCH just gets 'darker' but stays rich red.").size(14),
        comparison_row,
    ]
    .spacing(24)
    .padding(32)
    .into()
}
