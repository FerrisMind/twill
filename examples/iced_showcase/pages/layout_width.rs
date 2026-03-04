use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, row, text};
use iced::{Element, Length};
use twill::backends::iced::{
    apply_layout, apply_layout_with_custom_properties, to_color, to_font_weight,
};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, Container, FontWeight, Percentage, Scale, Spacing};
use twill::utilities::{Padding, Width, WidthVar};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("width").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed width utilities for iced integration.").size(16),
        variants_section(is_dark),
        spacing_scale_section(is_dark),
        fraction_section(is_dark),
        container_scale_section(is_dark),
        viewport_and_intrinsic_section(is_dark),
        custom_property_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .w(Spacing::S24)                         // w-24
    .w_fraction(Percentage::S1_2)           // w-1/2
    .w_container(Container::Md)             // w-md
    .w_px()                                 // w-px
    .w_auto()                               // w-auto
    .w_screen()                             // w-screen
    .w_dvw()                                // w-dvw
    .w_min()                                // w-min
    .w_fit()                                // w-fit
    .width(Width::w_full());                // w-full

let custom = Style::new().w_var(WidthVar::new("--panel-w"));"#;

    let visual = text("Width families are represented as Rust types and builder methods.").size(13);
    Snippet::new("Width: Variant Families", code, visual).view(is_dark)
}

fn spacing_scale_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let a = Style::new().w(Spacing::S24); // w-24
let b = Style::new().w(Spacing::S40); // w-40
let c = Style::new().w(Spacing::S64); // w-64"#;

    let visual = preview_surface(
        column![
            width_demo(
                "w-24",
                Width::w(Spacing::S24),
                Color::blue(Scale::S500),
                is_dark
            ),
            width_demo(
                "w-40",
                Width::w(Spacing::S40),
                Color::blue(Scale::S500),
                is_dark
            ),
            width_demo(
                "w-64",
                Width::w(Spacing::S64),
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
    let code = r#"let half = Style::new().w_fraction(Percentage::S1_2); // w-1/2
let two_fifths = Style::new().w_fraction(Percentage::S2_5); // w-2/5
let three_fifths = Style::new().w_fraction(Percentage::S3_5); // w-3/5
let full = Style::new().width(Width::w_full()); // w-full"#;

    let visual = preview_surface(
        column![
            row![
                width_demo(
                    "w-1/2",
                    Width::w_fraction(Percentage::S1_2),
                    Color::violet(Scale::S500),
                    is_dark
                ),
                width_demo(
                    "w-1/2",
                    Width::w_fraction(Percentage::S1_2),
                    Color::violet(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(8)
            .width(Length::Fill),
            row![
                width_demo(
                    "w-2/5",
                    Width::w_fraction(Percentage::S2_5),
                    Color::violet(Scale::S500),
                    is_dark
                ),
                width_demo(
                    "w-3/5",
                    Width::w_fraction(Percentage::S3_5),
                    Color::violet(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(8)
            .width(Length::Fill),
            width_demo(
                "w-full",
                Width::w_full(),
                Color::violet(Scale::S500),
                is_dark
            ),
        ]
        .spacing(10),
        is_dark,
    );

    Snippet::new("Fraction + Full Width", code, visual).view(is_dark)
}

fn container_scale_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let sm = Style::new().w_container(Container::Sm); // w-sm
let md = Style::new().w_container(Container::Md); // w-md
let xl = Style::new().w_container(Container::Xl); // w-xl"#;

    let visual = preview_surface(
        column![
            width_demo(
                "w-sm",
                Width::w_container(Container::Sm),
                Color::sky(Scale::S500),
                is_dark
            ),
            width_demo(
                "w-md",
                Width::w_container(Container::Md),
                Color::sky(Scale::S500),
                is_dark
            ),
            width_demo(
                "w-xl",
                Width::w_container(Container::Xl),
                Color::sky(Scale::S500),
                is_dark
            ),
        ]
        .spacing(10),
        is_dark,
    );

    Snippet::new("Container Scale", code, visual).view(is_dark)
}

fn viewport_and_intrinsic_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let viewport = Style::new().w_screen();           // w-screen
let dynamic = Style::new().w_dvw();             // w-dvw
let intrinsic_min = Style::new().w_min();       // w-min
let intrinsic_max = Style::new().w_max();       // w-max
let intrinsic_fit = Style::new().w_fit();       // w-fit
let one_pixel = Style::new().w_px();            // w-px
let auto = Style::new().w_auto();               // w-auto"#;

    let visual = preview_surface(
        column![
            width_demo(
                "w-screen",
                Width::w_screen(),
                Color::emerald(Scale::S500),
                is_dark
            ),
            width_demo(
                "w-dvw",
                Width::w_dvw(),
                Color::emerald(Scale::S500),
                is_dark
            ),
            row![
                width_demo(
                    "w-min",
                    Width::w_min(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                width_demo(
                    "w-max",
                    Width::w_max(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
                width_demo(
                    "w-fit",
                    Width::w_fit(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(8),
            row![
                width_demo("w-px", Width::w_px(), Color::emerald(Scale::S500), is_dark),
                width_demo(
                    "w-auto",
                    Width::w_auto(),
                    Color::emerald(Scale::S500),
                    is_dark
                ),
            ]
            .spacing(8),
        ]
        .spacing(10),
        is_dark,
    );

    Snippet::new("Viewport + Intrinsic", code, visual).view(is_dark)
}

fn custom_property_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"const PANEL_W: WidthVar = WidthVar::new("--panel-w");

let custom = Style::new().w_var(PANEL_W); // w-(--panel-w)
let px_value = Style::new().w_px_value(280); // w-[280px]

let a = apply_layout_with_custom_properties(content, &custom, &[("--panel-w", 220.0)]);
let b = apply_layout(content, &px_value);"#;

    const PANEL_W: WidthVar = WidthVar::new("--panel-w");
    let custom = Style::new().w_var(PANEL_W);
    let px_value = Style::new().w_px_value(280);

    let custom_panel = apply_layout_with_custom_properties(
        text("w-(--panel-w)").size(13).into(),
        &custom
            .bg(Color::indigo(Scale::S500))
            .rounded(BorderRadius::Sm)
            .padding(Padding::all(Spacing::S2)),
        &[("--panel-w", 220.0)],
    );

    let px_panel = apply_layout(
        text("w-[280px]").size(13).into(),
        &px_value
            .bg(Color::indigo(Scale::S500))
            .rounded(BorderRadius::Sm)
            .padding(Padding::all(Spacing::S2)),
    );

    let visual = preview_surface(column![custom_panel, px_panel].spacing(10), is_dark);
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

fn width_demo<'a>(
    label: &'a str,
    width: Width,
    accent: Color,
    is_dark: bool,
) -> Element<'a, Message> {
    let text_color = if is_dark {
        to_color(Color::gray(Scale::S100))
    } else {
        to_color(Color::white())
    };

    apply_layout(
        text(label).size(13).color(text_color).into(),
        &Style::new()
            .width(width)
            .bg(accent)
            .rounded(BorderRadius::Sm)
            .padding(Padding::symmetric(Spacing::S2, Spacing::S3)),
    )
}
