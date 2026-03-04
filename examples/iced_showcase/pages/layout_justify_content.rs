use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, text};
use iced::{Element, Length};
use twill::backends::iced::{justify_content_layout, styled_container, to_color, to_font_weight};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{FlexDirection, JustifyContent, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("justify-content").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed flex main-axis alignment utilities for Twill + iced.").size(16),
        variants_section(is_dark),
        start_section(is_dark),
        center_section(is_dark),
        end_section(is_dark),
        between_section(is_dark),
        around_section(is_dark),
        evenly_section(is_dark),
        safe_variants_section(is_dark),
        stretch_section(is_dark),
        normal_and_baseline_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let variants = [
    JustifyContent::Start,
    JustifyContent::End,
    JustifyContent::EndSafe,
    JustifyContent::Center,
    JustifyContent::CenterSafe,
    JustifyContent::Between,
    JustifyContent::Around,
    JustifyContent::Evenly,
    JustifyContent::Stretch,
    JustifyContent::Baseline,
    JustifyContent::Normal,
];"#;

    let visual = text("Main-axis distribution is modeled as enum variants.").size(13);

    Snippet::new("Justify Content: Variant Coverage", code, visual).view(is_dark)
}

fn start_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::Start);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::Start);"#;

    justify_section(
        "Start",
        code,
        JustifyContent::Start,
        Color::pink(Scale::S500),
        is_dark,
        false,
        None,
    )
}

fn center_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::Center);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::Center);"#;

    justify_section(
        "Center",
        code,
        JustifyContent::Center,
        Color::violet(Scale::S500),
        is_dark,
        false,
        None,
    )
}

fn end_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::End);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::End);"#;

    justify_section(
        "End",
        code,
        JustifyContent::End,
        Color::sky(Scale::S500),
        is_dark,
        false,
        None,
    )
}

fn between_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::Between);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::Between);"#;

    justify_section(
        "Between",
        code,
        JustifyContent::Between,
        Color::cyan(Scale::S500),
        is_dark,
        false,
        None,
    )
}

fn around_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::Around);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::Around);"#;

    justify_section(
        "Around",
        code,
        JustifyContent::Around,
        Color::purple(Scale::S500),
        is_dark,
        false,
        None,
    )
}

fn evenly_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::Evenly);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::Evenly);"#;

    justify_section(
        "Evenly",
        code,
        JustifyContent::Evenly,
        Color::indigo(Scale::S500),
        is_dark,
        false,
        None,
    )
}

fn safe_variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let end_safe = Style::flex_row().justify_content(JustifyContent::EndSafe);
let center_safe = Style::flex_row().justify_content(JustifyContent::CenterSafe);"#;

    let visual = column![
        justified_preview(
            JustifyContent::EndSafe,
            Color::emerald(Scale::S500),
            is_dark,
            false
        ),
        justified_preview(
            JustifyContent::CenterSafe,
            Color::emerald(Scale::S500),
            is_dark,
            false
        ),
        text("`safe` variants are preserved in the type system and mapped as best-effort in iced.")
            .size(12),
    ]
    .spacing(10);

    Snippet::new("Safe Variants", code, visual).view(is_dark)
}

fn stretch_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().justify_content(JustifyContent::Stretch);
let preview = justify_content_layout(items, FlexDirection::Row, Spacing::S4, JustifyContent::Stretch);"#;

    justify_section(
        "Stretch",
        code,
        JustifyContent::Stretch,
        Color::fuchsia(Scale::S500),
        is_dark,
        true,
        Some("Best-effort iced mapping: stretch uses Fill wrappers on the main axis."),
    )
}

fn normal_and_baseline_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let normal = Style::flex_row().justify_content(JustifyContent::Normal);
let baseline = Style::flex_row().justify_content(JustifyContent::Baseline);"#;

    let visual = column![
        justified_preview(
            JustifyContent::Normal,
            Color::blue(Scale::S500),
            is_dark,
            false
        ),
        justified_preview(
            JustifyContent::Baseline,
            Color::blue(Scale::S300),
            is_dark,
            false
        ),
        text("`Normal` and `Baseline` are explicit variants with safe fallback behavior in iced.")
            .size(12),
    ]
    .spacing(10);

    Snippet::new("Normal / Baseline", code, visual).view(is_dark)
}

fn justify_section<'a>(
    title: &'static str,
    code: &'static str,
    justify_content: JustifyContent,
    bg: Color,
    is_dark: bool,
    stretch_items: bool,
    note: Option<&'static str>,
) -> Element<'a, Message> {
    let mut visual = column![justified_preview_with_mode(
        justify_content,
        bg,
        is_dark,
        stretch_items
    )]
    .spacing(10);

    if let Some(note) = note {
        visual = visual.push(text(note).size(12));
    }

    Snippet::new(title, code, visual).view(is_dark)
}

fn justified_preview<'a>(
    justify_content: JustifyContent,
    bg: Color,
    is_dark: bool,
    stretch_items: bool,
) -> Element<'a, Message> {
    justified_preview_with_mode(justify_content, bg, is_dark, stretch_items)
}

fn justified_preview_with_mode<'a>(
    justify_content: JustifyContent,
    bg: Color,
    is_dark: bool,
    stretch_items: bool,
) -> Element<'a, Message> {
    let items = if stretch_items {
        fill_tiles(bg, is_dark)
    } else {
        fixed_tiles(bg, is_dark)
    };

    preview_surface(
        container(justify_content_layout(
            items,
            FlexDirection::Row,
            Spacing::S4,
            justify_content,
        ))
        .width(Length::Fill)
        .height(Length::Fixed(92.0)),
        is_dark,
    )
}

fn preview_surface<'a>(
    content: impl Into<Element<'a, Message>>,
    is_dark: bool,
) -> Element<'a, Message> {
    styled_container(
        content.into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    )
    .width(Length::Fill)
    .into()
}

fn fixed_tiles<'a>(bg: Color, is_dark: bool) -> Vec<Element<'a, Message>> {
    vec![
        fixed_tile("01", bg, 64.0, is_dark),
        fixed_tile("02", bg, 64.0, is_dark),
        fixed_tile("03", bg, 64.0, is_dark),
    ]
}

fn fill_tiles<'a>(bg: Color, is_dark: bool) -> Vec<Element<'a, Message>> {
    vec![
        fill_tile("01", bg, is_dark),
        fill_tile("02", bg, is_dark),
        fill_tile("03", bg, is_dark),
    ]
}

fn fixed_tile<'a>(label: &'a str, bg: Color, width: f32, is_dark: bool) -> Element<'a, Message> {
    container(tile_base(label, bg, is_dark))
        .width(Length::Fixed(width))
        .into()
}

fn fill_tile<'a>(label: &'a str, bg: Color, is_dark: bool) -> Element<'a, Message> {
    styled_container(
        container(text(label).size(14).color(tile_text_color(is_dark)))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fixed(56.0))
            .into(),
        &Style::new()
            .bg(bg)
            .rounded(BorderRadius::Md)
            .padding(Padding::all(Spacing::S2)),
    )
    .width(Length::Fill)
    .into()
}

fn tile_base<'a>(label: &'a str, bg: Color, is_dark: bool) -> Element<'a, Message> {
    styled_container(
        container(text(label).size(14).color(tile_text_color(is_dark)))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fixed(64.0))
            .height(Length::Fixed(56.0))
            .into(),
        &Style::new()
            .bg(bg)
            .rounded(BorderRadius::Md)
            .padding(Padding::all(Spacing::S2)),
    )
    .into()
}

fn tile_text_color(is_dark: bool) -> iced::Color {
    if is_dark {
        to_color(Color::gray(Scale::S100))
    } else {
        to_color(Color::white())
    }
}
