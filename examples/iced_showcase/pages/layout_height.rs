use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};
use twill::backends::iced::{
    apply_layout, apply_layout_with_custom_properties, to_color, to_font_weight,
};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Percentage, Scale, Spacing};
use twill::utilities::{Height, HeightVar, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("height").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed height utilities for iced integration.").size(16),
        variants_section(is_dark),
        spacing_scale_section(is_dark),
        fraction_section(is_dark),
        viewport_and_intrinsic_section(is_dark),
        custom_property_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .h(Spacing::S24)                         // h-24
    .h_fraction(Percentage::S1_2)            // h-1/2
    .h_px()                                  // h-px
    .h_auto()                                // h-auto
    .h_screen()                              // h-screen
    .h_dvh()                                 // h-dvh
    .h_dvw()                                 // h-dvw
    .h_lvh()                                 // h-lvh
    .h_lvw()                                 // h-lvw
    .h_svh()                                 // h-svh
    .h_svw()                                 // h-svw
    .h_min()                                 // h-min
    .h_max()                                 // h-max
    .h_fit()                                 // h-fit
    .h_lh()                                  // h-lh
    .height(Height::h_full());               // h-full

let custom = Style::new().h_var(HeightVar::new("--panel-h"));"#;

    let visual =
        text("Height families are represented as Rust types and builder methods.").size(13);
    Snippet::new("Height: Variant Families", code, visual).view(is_dark)
}

fn spacing_scale_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let a = Style::new().h(Spacing::S24); // h-24
let b = Style::new().h(Spacing::S40); // h-40
let c = Style::new().h(Spacing::S64); // h-64"#;

    let visual = preview_surface(
        row![
            height_demo(
                "h-24",
                Height::h(Spacing::S24),
                Color::blue(Scale::S500),
                is_dark
            ),
            height_demo(
                "h-40",
                Height::h(Spacing::S40),
                Color::blue(Scale::S500),
                is_dark
            ),
            height_demo(
                "h-64",
                Height::h(Spacing::S64),
                Color::blue(Scale::S500),
                is_dark
            ),
        ]
        .spacing(10),
        is_dark,
    );

    Snippet::new("Spacing Scale", code, visual).view(is_dark)
}

fn fraction_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let half = Style::new().h_fraction(Percentage::S1_2); // h-1/2
let two_fifths = Style::new().h_fraction(Percentage::S2_5); // h-2/5
let three_fifths = Style::new().h_fraction(Percentage::S3_5); // h-3/5
let full = Style::new().height(Height::h_full()); // h-full"#;

    let visual = preview_surface(
        row![
            height_demo(
                "h-1/2",
                Height::h_fraction(Percentage::S1_2),
                Color::sky(Scale::S500),
                is_dark
            ),
            height_demo(
                "h-2/5",
                Height::h_fraction(Percentage::S2_5),
                Color::sky(Scale::S500),
                is_dark
            ),
            height_demo(
                "h-3/5",
                Height::h_fraction(Percentage::S3_5),
                Color::sky(Scale::S500),
                is_dark
            ),
            height_demo("h-full", Height::h_full(), Color::sky(Scale::S500), is_dark),
        ]
        .spacing(10),
        is_dark,
    );

    Snippet::new("Fraction + Full Height", code, visual).view(is_dark)
}

fn viewport_and_intrinsic_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let viewport = Style::new().h_screen();        // h-screen
let dynamic = Style::new().h_dvh();             // h-dvh
let dynamic_w = Style::new().h_dvw();           // h-dvw
let large = Style::new().h_lvh();               // h-lvh
let large_w = Style::new().h_lvw();             // h-lvw
let small = Style::new().h_svh();               // h-svh
let small_w = Style::new().h_svw();             // h-svw
let intrinsic_min = Style::new().h_min();       // h-min
let intrinsic_max = Style::new().h_max();       // h-max
let intrinsic_fit = Style::new().h_fit();       // h-fit
let line_height = Style::new().h_lh();          // h-lh
let one_pixel = Style::new().h_px();            // h-px
let auto = Style::new().h_auto();               // h-auto"#;

    let visual = preview_surface(
        column![
            row![
                height_demo(
                    "h-screen",
                    Height::h_screen(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-dvh",
                    Height::h_dvh(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-dvw",
                    Height::h_dvw(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(10),
            row![
                height_demo(
                    "h-lvh",
                    Height::h_lvh(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-lvw",
                    Height::h_lvw(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-svh",
                    Height::h_svh(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-svw",
                    Height::h_svw(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(10),
            row![
                height_demo(
                    "h-min",
                    Height::h_min(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-max",
                    Height::h_max(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo(
                    "h-fit",
                    Height::h_fit(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                height_demo("h-lh", Height::h_lh(), Color::emerald(Scale::S500), is_dark),
            ]
            .spacing(10),
            row![
                height_demo("h-px", Height::h_px(), Color::emerald(Scale::S500), is_dark),
                height_demo(
                    "h-auto",
                    Height::h_auto(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(10),
        ]
        .spacing(10),
        is_dark,
    );

    Snippet::new("Viewport + Intrinsic", code, visual).view(is_dark)
}

fn custom_property_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"const PANEL_H: HeightVar = HeightVar::new("--panel-h");

let custom = Style::new().h_var(PANEL_H); // h-(--panel-h)
let px_value = Style::new().h_px_value(140); // h-[140px]

let a = apply_layout_with_custom_properties(content, &custom, &[("--panel-h", 180.0)]);
let b = apply_layout(content, &px_value);"#;

    const PANEL_H: HeightVar = HeightVar::new("--panel-h");
    let custom = Style::new().h_var(PANEL_H);
    let px_value = Style::new().h_px_value(140);

    let custom_panel = height_demo_with_style(
        "h-(--panel-h)",
        &custom
            .bg(Color::indigo(Scale::S500))
            .rounded(BorderRadius::Sm)
            .padding(Padding::symmetric(Spacing::S2, Spacing::S3)),
        &[("--panel-h", 180.0)],
    );

    let px_panel = height_demo_with_style(
        "h-[140px]",
        &px_value
            .bg(Color::indigo(Scale::S500))
            .rounded(BorderRadius::Sm)
            .padding(Padding::symmetric(Spacing::S2, Spacing::S3)),
        &[],
    );

    let visual = preview_surface(row![custom_panel, px_panel].spacing(10), is_dark);
    Snippet::new("Custom Property + Typed Arbitrary Px", code, visual).view(is_dark)
}

fn preview_surface<'a>(
    content: impl Into<Element<'a, Message>>,
    is_dark: bool,
) -> Element<'a, Message> {
    apply_layout(
        content.into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    )
}

fn height_demo<'a>(
    label: &'a str,
    height: Height,
    accent: Color,
    _is_dark: bool,
) -> Element<'a, Message> {
    height_demo_with_style(
        label,
        &Style::new()
            .height(height)
            .bg(accent)
            .rounded(BorderRadius::Sm)
            .padding(Padding::symmetric(Spacing::S2, Spacing::S3)),
        &[],
    )
}

fn height_demo_with_style<'a>(
    label: &'a str,
    style: &Style,
    custom_properties: &[(&str, f32)],
) -> Element<'a, Message> {
    let panel = apply_layout_with_custom_properties(
        text(label).size(13).color(to_color(Color::white())).into(),
        style,
        custom_properties,
    );

    container(panel)
        .height(Length::Fixed(220.0))
        .width(Length::Fixed(88.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Bottom)
        .into()
}
