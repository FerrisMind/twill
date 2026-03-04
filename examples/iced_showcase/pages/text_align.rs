use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, text};
use iced::{Element, Length};
use twill::Style;
use twill::iced::{styled_container, to_text_alignment, to_text_alignment_with_direction};
use twill::tokens::{BorderRadius, Color, Scale, Spacing, TextAlign};
use twill::utilities::Padding;

const SAMPLE: &str =
    "So I started to walk into the water. I won't lie to you boys, I was terrified.";

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("text-align").size(32),
        text("Typed Rust utilities for controlling text alignment in iced.").size(16),
        variants_section(is_dark),
        basic_section(is_dark),
        logical_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .text_left()     // text-left
    .text_center()   // text-center
    .text_right()    // text-right
    .text_justify()  // text-justify
    .text_start()    // text-start
    .text_end();     // text-end"#;

    let visual =
        text("Text alignment classes are exposed as typed builder methods on Style.").size(13);

    Snippet::new("Text Align: Variant Families", code, visual).view(is_dark)
}

fn basic_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().text_center(); // text-center
let align = style.text_align.unwrap_or(TextAlign::Left);

let paragraph = text("...")
    .width(Length::Fill)
    .align_x(to_text_alignment(align));"#;

    let samples = [
        ("text-left", Style::new().text_left()),
        ("text-center", Style::new().text_center()),
        ("text-right", Style::new().text_right()),
        ("text-justify", Style::new().text_justify()),
    ];

    let mut visual = column![].spacing(10);
    for (class_name, style) in samples {
        let align = style.text_align.unwrap_or(TextAlign::Left);
        visual = visual.push(
            column![
                text(class_name).size(12),
                text(SAMPLE)
                    .width(Length::Fill)
                    .align_x(to_text_alignment(align)),
            ]
            .spacing(4),
        );
    }

    Snippet::new("Basic Example", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn logical_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let start = Style::new().text_start(); // text-start
let end = Style::new().text_end();     // text-end

let start_align_ltr = to_text_alignment_with_direction(TextAlign::Start, false);
let end_align_ltr = to_text_alignment_with_direction(TextAlign::End, false);

let start_align_rtl = to_text_alignment_with_direction(TextAlign::Start, true);
let end_align_rtl = to_text_alignment_with_direction(TextAlign::End, true);"#;

    let visual = column![
        text("LTR mapping").size(12),
        text(SAMPLE)
            .width(Length::Fill)
            .align_x(to_text_alignment_with_direction(TextAlign::Start, false)),
        text(SAMPLE)
            .width(Length::Fill)
            .align_x(to_text_alignment_with_direction(TextAlign::End, false)),
        text("RTL mapping").size(12),
        text("بدأتُ أسير نحو الماء. لن أكذب عليكم يا رفاق، كنتُ مرعوبًا.")
            .width(Length::Fill)
            .align_x(to_text_alignment_with_direction(TextAlign::Start, true)),
        text("بدأتُ أسير نحو الماء. لن أكذب عليكم يا رفاق، كنتُ مرعوبًا.")
            .width(Length::Fill)
            .align_x(to_text_alignment_with_direction(TextAlign::End, true)),
    ]
    .spacing(8);

    Snippet::new(
        "Using Logical Properties",
        code,
        preview_surface(visual, is_dark),
    )
    .view(is_dark)
}

fn preview_surface<'a>(
    content: impl Into<Element<'a, Message>>,
    is_dark: bool,
) -> Element<'a, Message> {
    styled_container(
        container(content.into())
            .width(Length::Fill)
            .max_width(560.0)
            .center_x(Length::Fill)
            .into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    )
    .width(Length::Fill)
    .into()
}
