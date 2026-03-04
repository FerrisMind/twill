use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, text};
use iced::{Element, Length};
use twill::backends::iced::{align_items_layout, styled_container, to_color, to_font_weight};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{AlignItems, FlexDirection, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("align-items").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed flex cross-axis alignment utilities for Twill + iced.").size(16),
        variants_section(is_dark),
        stretch_section(is_dark),
        start_section(is_dark),
        center_section(is_dark),
        end_section(is_dark),
        baseline_section(is_dark),
        safe_variants_section(is_dark),
        baseline_last_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let variants = [
    AlignItems::Start,
    AlignItems::End,
    AlignItems::EndSafe,
    AlignItems::Center,
    AlignItems::CenterSafe,
    AlignItems::Baseline,
    AlignItems::BaselineLast,
    AlignItems::Stretch,
];

let style = Style::flex_row().items_center();"#;

    let visual =
        text("`align-items` is modeled as enum variants, not runtime class strings.").size(13);

    Snippet::new("Align Items: Variant Coverage", code, visual).view(is_dark)
}

fn stretch_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().items_stretch();
let preview = align_items_layout(items, FlexDirection::Row, Spacing::S4, AlignItems::Stretch);"#;

    align_section(
        "Stretch",
        code,
        AlignItems::Stretch,
        Color::cyan(Scale::S500),
        is_dark,
        Some("Best-effort iced mapping: stretch uses Fill wrappers on the cross axis."),
    )
}

fn start_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().items_start();
let preview = align_items_layout(items, FlexDirection::Row, Spacing::S4, AlignItems::Start);"#;

    align_section(
        "Start",
        code,
        AlignItems::Start,
        Color::pink(Scale::S500),
        is_dark,
        None,
    )
}

fn center_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().items_center();
let preview = align_items_layout(items, FlexDirection::Row, Spacing::S4, AlignItems::Center);"#;

    align_section(
        "Center",
        code,
        AlignItems::Center,
        Color::violet(Scale::S500),
        is_dark,
        None,
    )
}

fn end_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().items_end();
let preview = align_items_layout(items, FlexDirection::Row, Spacing::S4, AlignItems::End);"#;

    align_section(
        "End",
        code,
        AlignItems::End,
        Color::sky(Scale::S500),
        is_dark,
        None,
    )
}

fn baseline_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().items_baseline();
let preview = align_items_layout(items, FlexDirection::Row, Spacing::S4, AlignItems::Baseline);"#;

    align_section(
        "Baseline",
        code,
        AlignItems::Baseline,
        Color::blue(Scale::S500),
        is_dark,
        Some("Iced has no baseline layout primitive; baseline is approximated on the cross axis."),
    )
}

fn safe_variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let end_safe = Style::flex_row().items_end_safe();
let center_safe = Style::flex_row().items_center_safe();"#;

    let visual = column![
        aligned_preview(AlignItems::EndSafe, Color::emerald(Scale::S500), is_dark),
        aligned_preview(AlignItems::CenterSafe, Color::emerald(Scale::S500), is_dark),
        text("`safe` variants are preserved in the type system and degrade safely in iced.")
            .size(12),
    ]
    .spacing(10);

    Snippet::new("Safe Variants", code, visual).view(is_dark)
}

fn baseline_last_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::flex_row().items_baseline_last();
let preview = align_items_layout(
    items,
    FlexDirection::Row,
    Spacing::S4,
    AlignItems::BaselineLast,
);"#;

    align_section(
        "Last Baseline",
        code,
        AlignItems::BaselineLast,
        Color::indigo(Scale::S500),
        is_dark,
        Some("`BaselineLast` is represented explicitly and uses a best-effort iced fallback."),
    )
}

fn align_section<'a>(
    title: &'static str,
    code: &'static str,
    align_items: AlignItems,
    bg: Color,
    is_dark: bool,
    note: Option<&'static str>,
) -> Element<'a, Message> {
    let mut visual = column![aligned_preview(align_items, bg, is_dark)].spacing(10);
    if let Some(note) = note {
        visual = visual.push(text(note).size(12));
    }

    Snippet::new(title, code, visual).view(is_dark)
}

fn aligned_preview<'a>(align_items: AlignItems, bg: Color, is_dark: bool) -> Element<'a, Message> {
    let items = if matches!(align_items, AlignItems::Stretch) {
        fill_tiles(bg, is_dark)
    } else {
        varying_tiles(bg, is_dark)
    };

    preview_surface(
        container(align_items_layout(
            items,
            FlexDirection::Row,
            Spacing::S4,
            align_items,
        ))
        .width(Length::Fill)
        .height(Length::Fixed(180.0)),
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

fn varying_tiles<'a>(bg: Color, is_dark: bool) -> Vec<Element<'a, Message>> {
    vec![
        fixed_tile("01", bg, 56.0, is_dark),
        fixed_tile("02", bg, 112.0, is_dark),
        fixed_tile("03", bg, 80.0, is_dark),
    ]
}

fn fill_tiles<'a>(bg: Color, is_dark: bool) -> Vec<Element<'a, Message>> {
    vec![
        fill_tile("01", bg, is_dark),
        fill_tile("02", bg, is_dark),
        fill_tile("03", bg, is_dark),
    ]
}

fn fixed_tile<'a>(label: &'a str, bg: Color, height: f32, is_dark: bool) -> Element<'a, Message> {
    tile_base(label, bg, Length::Fixed(height), is_dark)
}

fn fill_tile<'a>(label: &'a str, bg: Color, is_dark: bool) -> Element<'a, Message> {
    tile_base(label, bg, Length::Fill, is_dark)
}

fn tile_base<'a>(label: &'a str, bg: Color, height: Length, is_dark: bool) -> Element<'a, Message> {
    let text_color = if is_dark {
        to_color(Color::gray(Scale::S100))
    } else {
        to_color(Color::white())
    };

    styled_container(
        container(text(label).size(14).color(text_color))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fixed(90.0))
            .height(height)
            .into(),
        &Style::new()
            .bg(bg)
            .rounded(BorderRadius::Md)
            .padding(Padding::all(Spacing::S2)),
    )
    .into()
}
