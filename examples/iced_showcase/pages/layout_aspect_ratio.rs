use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, slider, text};
use iced::{Alignment, Element, Length};
use twill::backends::iced::{apply_layout, to_color, to_font_weight};
use twill::style::Style;
use twill::tokens::{
    AspectRatio, BorderRadius, BorderStyle, BorderWidth, Color, FontWeight, Scale, Spacing,
};
use twill::utilities::Padding;

pub fn view<'a>(is_dark: bool, preview_width: f32) -> Element<'a, Message> {
    column![
        text("aspect-ratio").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for controlling the aspect ratio of an element.").size(16),
        interactive_width_section(is_dark, preview_width),
        square_section(is_dark),
        video_section(is_dark),
        custom_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn interactive_width_section<'a>(is_dark: bool, preview_width: f32) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .aspect_ratio(AspectRatio::Video);

// Change container width to see dynamic ratio-preserving height
let preview = apply_layout(content, &style);"#;

    let width = preview_width.clamp(140.0, 520.0);

    let visual = column![
        aspect_width_control(preview_width),
        fixed_width_tile("1:1", AspectRatio::Square, width, is_dark),
        fixed_width_tile("16:9", AspectRatio::Video, width, is_dark),
        fixed_width_tile("4:3", AspectRatio::Custom(4, 3), width, is_dark),
    ]
    .spacing(12);

    Snippet::new("Aspect Ratio: Interactive Width", code, visual).view(is_dark)
}

fn square_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .aspect_ratio(AspectRatio::Square);

let preview = apply_layout(content, &style);"#;

    let preview = row![
        fixed_width_tile("Square A", AspectRatio::Square, 180.0, is_dark),
        fixed_width_tile("Square B", AspectRatio::Square, 180.0, is_dark),
    ]
    .spacing(16);

    Snippet::new("Aspect Ratio: Square", code, preview).view(is_dark)
}

fn video_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .aspect_ratio(AspectRatio::Video);

let preview = apply_layout(content, &style);"#;

    let preview = row![
        fixed_width_tile("16:9", AspectRatio::Video, 280.0, is_dark),
        fixed_width_tile("16:9", AspectRatio::Video, 220.0, is_dark),
    ]
    .spacing(16);

    Snippet::new("Aspect Ratio: Video (16/9)", code, preview).view(is_dark)
}

fn custom_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .aspect_ratio(AspectRatio::Custom(4, 3));

let preview = apply_layout(content, &style);"#;

    let preview = row![
        fixed_width_tile("4:3", AspectRatio::Custom(4, 3), 220.0, is_dark),
        fixed_width_tile("3:2", AspectRatio::Custom(3, 2), 220.0, is_dark),
        fixed_width_tile("1:1", AspectRatio::Custom(1, 1), 220.0, is_dark),
    ]
    .spacing(16);

    Snippet::new("Aspect Ratio: Custom", code, preview).view(is_dark)
}

fn fixed_width_tile<'a>(
    label: &'a str,
    ratio: AspectRatio,
    width: f32,
    is_dark: bool,
) -> Element<'a, Message> {
    let content_color = if is_dark {
        Color::gray(Scale::S200)
    } else {
        Color::gray(Scale::S700)
    };
    let bg = if is_dark {
        Color::gray(Scale::S800)
    } else {
        Color::gray(Scale::S100)
    };
    let border = if is_dark {
        Color::gray(Scale::S700)
    } else {
        Color::gray(Scale::S300)
    };

    let content: Element<'_, Message> =
        container(text(label).size(14).color(to_color(content_color)))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

    let style = Style::new()
        .aspect_ratio(ratio)
        .bg(bg)
        .padding(Padding::all(Spacing::S3))
        .rounded(BorderRadius::Md)
        .border(BorderWidth::S1, BorderStyle::Solid, border);

    container(apply_layout(content, &style))
        .width(Length::Fixed(width))
        .into()
}

fn aspect_width_control<'a>(preview_width: f32) -> Element<'a, Message> {
    row![
        text(format!("Preview width: {:.0}px", preview_width)).size(13),
        slider(
            140.0..=520.0,
            preview_width.clamp(140.0, 520.0),
            Message::AspectPreviewWidthChanged,
        )
        .step(10.0)
        .width(Length::Fill),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}
