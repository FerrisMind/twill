use crate::Message;
use crate::components::Snippet;
use iced::Element;
use iced::widget::{column, text};
use twill::Style;
use twill::iced::to_font_weight;
use twill::tokens::FontWeight;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("font-weight").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed Rust utilities for controlling font weight in iced.").size(16),
        variants_section(is_dark),
        basic_section(is_dark),
        iced_integration_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .font_thin()        // font-thin
    .font_extralight()  // font-extralight
    .font_light()       // font-light
    .font_normal()      // font-normal
    .font_medium()      // font-medium
    .font_semibold()    // font-semibold
    .font_bold()        // font-bold
    .font_extrabold()   // font-extrabold
    .font_black();      // font-black"#;

    let visual =
        text("Font weight classes are exposed as typed builder methods on Style.").size(13);

    Snippet::new("Font Weight: Variant Families", code, visual).view(is_dark)
}

fn basic_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().font_bold(); // font-bold
let weight = style.font_weight.unwrap_or(FontWeight::Normal);

let label = text("The quick brown fox jumps over the lazy dog.")
    .font(iced::Font {
        weight: to_font_weight(weight),
        ..Default::default()
    });"#;

    let samples = [
        ("font-thin", Style::new().font_thin()),
        ("font-extralight", Style::new().font_extralight()),
        ("font-light", Style::new().font_light()),
        ("font-normal", Style::new().font_normal()),
        ("font-medium", Style::new().font_medium()),
        ("font-semibold", Style::new().font_semibold()),
        ("font-bold", Style::new().font_bold()),
        ("font-extrabold", Style::new().font_extrabold()),
        ("font-black", Style::new().font_black()),
    ];

    let mut visual = column![].spacing(8);
    for (class_name, style) in samples {
        let weight = style.font_weight.unwrap_or(FontWeight::Normal);
        visual = visual.push(
            text(format!("{class_name} ({})", weight.value())).font(iced::Font {
                weight: to_font_weight(weight),
                ..Default::default()
            }),
        );
    }

    Snippet::new("Basic Example", code, visual).view(is_dark)
}

fn iced_integration_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let label_style = Style::new().font_semibold(); // font-semibold
let heading_style = Style::new().font_black();   // font-black

let label_weight = label_style.font_weight.unwrap_or(FontWeight::Normal);
let heading_weight = heading_style.font_weight.unwrap_or(FontWeight::Normal);

let label = text("Label").font(iced::Font {
    weight: to_font_weight(label_weight),
    ..Default::default()
});

let heading = text("Heading").font(iced::Font {
    weight: to_font_weight(heading_weight),
    ..Default::default()
});"#;

    let label_style = Style::new().font_semibold();
    let heading_style = Style::new().font_black();

    let label_weight = label_style.font_weight.unwrap_or(FontWeight::Normal);
    let heading_weight = heading_style.font_weight.unwrap_or(FontWeight::Normal);

    let visual = column![
        text("Label").font(iced::Font {
            weight: to_font_weight(label_weight),
            ..Default::default()
        }),
        text("Heading").size(28).font(iced::Font {
            weight: to_font_weight(heading_weight),
            ..Default::default()
        })
    ]
    .spacing(8);

    Snippet::new("Iced Integration", code, visual).view(is_dark)
}
