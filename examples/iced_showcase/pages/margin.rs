use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};
use twill::Style;
use twill::iced::{
    apply_layout_with_custom_properties, styled_container, to_color, to_font_weight,
};
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{MarginValue, MarginVar, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("margin").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed margin families for iced integration.").size(16),
        variants_section(is_dark),
        all_sides_section(is_dark),
        axis_section(is_dark),
        sides_section(is_dark),
        logical_section(is_dark),
        negative_section(is_dark),
        custom_and_auto_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .m(Spacing::S4)      // m-*
    .mx(Spacing::S8)     // mx-*
    .my(Spacing::S2)     // my-*
    .ms(Spacing::S6)     // ms-*
    .me(Spacing::S3)     // me-*
    .mbs(Spacing::S4)    // mbs-*
    .mbe(Spacing::S1)    // mbe-*
    .mt(Spacing::S5)     // mt-*
    .mr(Spacing::S4)     // mr-*
    .mb(Spacing::S2)     // mb-*
    .ml(Spacing::S7)     // ml-*
    .neg_mt(Spacing::S8); // -mt-*

let custom = Style::new()
    .mx_value(MarginValue::var(MarginVar::new("--mx")))
    .mb_value(MarginValue::rem(1.25));"#;

    let visual = text("No runtime class parsing. Margin families are typed Rust methods.").size(13);

    Snippet::new("Margin: Variant Families", code, visual).view(is_dark)
}

fn all_sides_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().m(Spacing::S8); // m-8"#;
    let visual = preview_surface(
        margin_demo(
            "m-8",
            Style::new().m(Spacing::S8),
            Color::violet(Scale::S500),
            is_dark,
        ),
        is_dark,
    );

    Snippet::new("All Sides", code, visual).view(is_dark)
}

fn axis_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let horizontal = Style::new().mx(Spacing::S8); // mx-8
let vertical = Style::new().my(Spacing::S8);   // my-8"#;

    let visual = row![
        margin_demo(
            "mx-8",
            Style::new().mx(Spacing::S8),
            Color::indigo(Scale::S500),
            is_dark
        ),
        margin_demo(
            "my-8",
            Style::new().my(Spacing::S8),
            Color::pink(Scale::S500),
            is_dark
        ),
    ]
    .spacing(12);

    Snippet::new("Axis Utilities", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn sides_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let top = Style::new().mt(Spacing::S6);    // mt-6
let right = Style::new().mr(Spacing::S6);  // mr-6
let bottom = Style::new().mb(Spacing::S6); // mb-6
let left = Style::new().ml(Spacing::S6);   // ml-6"#;

    let visual = row![
        margin_demo(
            "mt-6",
            Style::new().mt(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
        margin_demo(
            "mr-6",
            Style::new().mr(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
        margin_demo(
            "mb-6",
            Style::new().mb(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
        margin_demo(
            "ml-6",
            Style::new().ml(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
    ]
    .spacing(10);

    Snippet::new("Per-Side Utilities", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn logical_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let start = Style::new().ms(Spacing::S8); // ms-8
let end = Style::new().me(Spacing::S8);   // me-8
let block_start = Style::new().mbs(Spacing::S6); // mbs-6
let block_end = Style::new().mbe(Spacing::S6);   // mbe-6"#;

    let visual = column![
        row![
            margin_demo(
                "ms-8",
                Style::new().ms(Spacing::S8),
                Color::emerald(Scale::S500),
                is_dark
            ),
            margin_demo(
                "me-8",
                Style::new().me(Spacing::S8),
                Color::emerald(Scale::S500),
                is_dark
            ),
        ]
        .spacing(12),
        row![
            margin_demo(
                "mbs-6",
                Style::new().mbs(Spacing::S6),
                Color::cyan(Scale::S500),
                is_dark
            ),
            margin_demo(
                "mbe-6",
                Style::new().mbe(Spacing::S6),
                Color::cyan(Scale::S500),
                is_dark
            ),
        ]
        .spacing(12),
        text(
            "For iced integration today: ms/me map to left/right (LTR), mbs/mbe map to top/bottom."
        )
        .size(12),
    ]
    .spacing(10);

    Snippet::new("Logical Properties", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn negative_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .mt(Spacing::S8)
    .neg_mt(Spacing::S4); // -mt-4"#;

    let visual = preview_surface(
        margin_demo(
            "-mt-4",
            Style::new().mt(Spacing::S8).neg_mt(Spacing::S4),
            Color::sky(Scale::S500),
            is_dark,
        ),
        is_dark,
    );

    Snippet::new("Negative Values", code, visual).view(is_dark)
}

fn custom_and_auto_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"const MX: MarginVar = MarginVar::new("--mx");

let custom = Style::new()
    .mx_value(MarginValue::var(MX)) // mx-(--mx)
    .mb_value(MarginValue::px(10.0)); // mb-[10px]

let centered = Style::new().mx(Spacing::Auto); // mx-auto

let preview = apply_layout_with_custom_properties(
    content,
    &custom,
    &[("--mx", 20.0)],
);"#;

    const MX: MarginVar = MarginVar::new("--mx");

    let custom_style = Style::new()
        .mx_value(MarginValue::var(MX))
        .mb_value(MarginValue::px(10.0));

    let custom = margin_demo_with_custom_properties(
        "mx-(--mx)",
        custom_style,
        &[("--mx", 20.0)],
        Color::fuchsia(Scale::S500),
        is_dark,
    );

    let auto = margin_demo(
        "mx-auto",
        Style::new().mx(Spacing::Auto),
        Color::amber(Scale::S500),
        is_dark,
    );

    let visual = row![custom, auto].spacing(12);

    Snippet::new(
        "Custom Values + Auto",
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
        content.into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S900 } else { Scale::S50 }))
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Md),
    )
    .width(Length::Fill)
    .into()
}

fn margin_demo<'a>(
    label: &'a str,
    margin_style: Style,
    accent: Color,
    is_dark: bool,
) -> Element<'a, Message> {
    margin_demo_with_custom_properties(label, margin_style, &[], accent, is_dark)
}

fn margin_demo_with_custom_properties<'a>(
    label: &'a str,
    margin_style: Style,
    custom_properties: &[(&str, f32)],
    accent: Color,
    is_dark: bool,
) -> Element<'a, Message> {
    let text_color = if is_dark {
        to_color(Color::gray(Scale::S100))
    } else {
        to_color(Color::white())
    };

    let inner: Element<'a, Message> = styled_container(
        container(text(label).size(13).color(text_color))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fixed(84.0))
            .height(Length::Fixed(46.0))
            .into(),
        &Style::new()
            .bg(accent)
            .rounded(BorderRadius::Sm)
            .padding(Padding::all(Spacing::S2)),
    )
    .into();

    let with_margin = apply_layout_with_custom_properties(inner, &margin_style, custom_properties);

    styled_container(
        container(with_margin)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fixed(140.0))
            .into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S800 } else { Scale::S100 }))
            .rounded(BorderRadius::Md)
            .padding(Padding::all(Spacing::S2)),
    )
    .width(Length::Fill)
    .into()
}
