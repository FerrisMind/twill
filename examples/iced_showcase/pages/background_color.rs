use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, row, text};
use iced::{Element, Length};
use twill::iced::{styled_container, to_color, to_font_weight};
use twill::style::Style;
use twill::tokens::{BackgroundColorVar, BorderRadius, Color, ColorValueToken, FontWeight, Scale};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("Background Color").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed background tokens for iced integration.").size(16),
        palette_bg_snippet().view(is_dark),
        special_bg_snippet().view(is_dark),
        custom_bg_snippet().view(is_dark),
    ]
    .spacing(32)
    .into()
}

fn palette_bg_snippet<'a>() -> Snippet<'a, Message> {
    let code = r#"// bg-{color}
let style = Style::new()
    .bg(Color::blue(Scale::S500))
    .rounded(BorderRadius::Md);"#;

    let preview = row![
        bg_chip("blue-500", Style::new().bg(Color::blue(Scale::S500))),
        bg_chip("cyan-500", Style::new().bg(Color::cyan(Scale::S500))),
        bg_chip("pink-500", Style::new().bg(Color::pink(Scale::S500))),
    ]
    .spacing(16);

    Snippet::new("Palette backgrounds", code, preview)
}

fn special_bg_snippet<'a>() -> Snippet<'a, Message> {
    let code = r#"// bg-inherit / bg-current / bg-transparent
let inherit = Style::new().bg_inherit();
let current = Style::new().text_color(Color::emerald(Scale::S600)).bg_current();
let transparent = Style::new().bg_transparent();"#;

    let preview = row![
        bg_chip("inherit", Style::new().bg_inherit()),
        bg_chip(
            "current",
            Style::new()
                .text_color(Color::emerald(Scale::S600))
                .bg_current()
        ),
        bg_chip("transparent", Style::new().bg_transparent()),
    ]
    .spacing(16);

    Snippet::new("Special background tokens", code, preview)
}

fn custom_bg_snippet<'a>() -> Snippet<'a, Message> {
    let code = r#"// bg-(<custom-property>) / bg-[<value>]
const SURFACE_BG: BackgroundColorVar = BackgroundColorVar::new("--surface-bg");
let custom_property = Style::new().bg_var(SURFACE_BG);

let arbitrary = Style::new().bg_arbitrary(
    ColorValueToken::from_rgba8(80, 215, 30, 255)
);"#;

    let preview = row![
        bg_chip(
            "var(--surface-bg)",
            Style::new().bg_var(BackgroundColorVar::new("--surface-bg"))
        ),
        bg_chip(
            "rgba(80,215,30,1)",
            Style::new().bg_arbitrary(ColorValueToken::from_rgba8(80, 215, 30, 255))
        ),
    ]
    .spacing(16);

    Snippet::new("Custom value backgrounds", code, preview)
}

fn bg_chip<'a>(label: &'a str, style: Style) -> Element<'a, Message> {
    styled_container(
        text(label).size(12).color(to_color(Color::white())).into(),
        &style.rounded(BorderRadius::Md),
    )
    .width(Length::Fixed(160.0))
    .height(Length::Fixed(72.0))
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}
