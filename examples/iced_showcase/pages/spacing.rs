use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};
use twill::iced::{
    styled_container, styled_container_with_custom_properties, to_color, to_font_weight,
};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{Padding, PaddingValue, PaddingVar};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("padding").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed padding families for iced integration.").size(16),
        variants_section(is_dark),
        all_sides_section(is_dark),
        pixel_token_section(is_dark),
        axis_section(is_dark),
        sides_section(is_dark),
        logical_section(is_dark),
        custom_value_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new()
    .p(Spacing::S4)     // p-*
    .px(Spacing::S8)    // px-*
    .py(Spacing::S2)    // py-*
    .ps(Spacing::S6)    // ps-*
    .pe(Spacing::S3)    // pe-*
    .pbs(Spacing::S4)   // pbs-*
    .pbe(Spacing::S1)   // pbe-*
    .pt(Spacing::S5)    // pt-*
    .pr(Spacing::S4)    // pr-*
    .pb(Spacing::S2)    // pb-*
    .pl(Spacing::S7);   // pl-*

let custom = Style::new()
    .p_value(PaddingValue::px(5.0))
    .px_value(PaddingValue::var(PaddingVar::new("--pad-x")));"#;

    let visual =
        text("No runtime class parsing. Families are typed Rust builder methods.").size(13);

    Snippet::new("Padding: Variant Families", code, visual).view(is_dark)
}

fn all_sides_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().p(Spacing::S8); // p-8"#;
    let visual = preview_surface(
        padded_demo(
            "p-8",
            Padding::p(Spacing::S8),
            Color::violet(Scale::S500),
            is_dark,
        ),
        is_dark,
    );

    Snippet::new("All Sides", code, visual).view(is_dark)
}

fn pixel_token_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let all = Style::new().p(Spacing::Px);    // p-px
let x = Style::new().px(Spacing::Px);      // px-px
let y = Style::new().py(Spacing::Px);      // py-px
let top = Style::new().pt(Spacing::Px);    // pt-px"#;

    let visual = row![
        padded_demo(
            "p-px",
            Padding::p(Spacing::Px),
            Color::amber(Scale::S500),
            is_dark
        ),
        padded_demo(
            "px-px",
            Padding::px(Spacing::Px),
            Color::amber(Scale::S500),
            is_dark
        ),
        padded_demo(
            "py-px",
            Padding::py(Spacing::Px),
            Color::amber(Scale::S500),
            is_dark
        ),
        padded_demo(
            "pt-px",
            Padding::pt(Spacing::Px),
            Color::amber(Scale::S500),
            is_dark
        ),
    ]
    .spacing(10);

    Snippet::new(
        "Pixel Token (`*-px`)",
        code,
        preview_surface(visual, is_dark),
    )
    .view(is_dark)
}

fn axis_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let horizontal = Style::new().px(Spacing::S8); // px-8
let vertical = Style::new().py(Spacing::S8);   // py-8"#;

    let visual = row![
        padded_demo(
            "px-8",
            Padding::px(Spacing::S8),
            Color::indigo(Scale::S500),
            is_dark
        ),
        padded_demo(
            "py-8",
            Padding::py(Spacing::S8),
            Color::pink(Scale::S500),
            is_dark
        ),
    ]
    .spacing(12);

    Snippet::new("Axis Utilities", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn sides_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let top = Style::new().pt(Spacing::S6);    // pt-6
let right = Style::new().pr(Spacing::S6);  // pr-6
let bottom = Style::new().pb(Spacing::S6); // pb-6
let left = Style::new().pl(Spacing::S6);   // pl-6"#;

    let visual = row![
        padded_demo(
            "pt-6",
            Padding::pt(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
        padded_demo(
            "pr-6",
            Padding::pr(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
        padded_demo(
            "pb-6",
            Padding::pb(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
        padded_demo(
            "pl-6",
            Padding::pl(Spacing::S6),
            Color::blue(Scale::S500),
            is_dark
        ),
    ]
    .spacing(10);

    Snippet::new("Per-Side Utilities", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn logical_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let start = Style::new().ps(Spacing::S8); // ps-8
let end = Style::new().pe(Spacing::S8);   // pe-8
let block_start = Style::new().pbs(Spacing::S6); // pbs-6
let block_end = Style::new().pbe(Spacing::S6);   // pbe-6"#;

    let visual = column![
        row![
            padded_demo(
                "ps-8",
                Padding::ps(Spacing::S8),
                Color::emerald(Scale::S500),
                is_dark
            ),
            padded_demo(
                "pe-8",
                Padding::pe(Spacing::S8),
                Color::emerald(Scale::S500),
                is_dark
            ),
        ]
        .spacing(12),
        row![
            padded_demo(
                "pbs-6",
                Padding::pbs(Spacing::S6),
                Color::cyan(Scale::S500),
                is_dark
            ),
            padded_demo(
                "pbe-6",
                Padding::pbe(Spacing::S6),
                Color::cyan(Scale::S500),
                is_dark
            ),
        ]
        .spacing(12),
        text(
            "For iced integration today: ps/pe map to left/right (LTR), pbs/pbe map to top/bottom."
        )
        .size(12),
    ]
    .spacing(10);

    Snippet::new("Logical Properties", code, preview_surface(visual, is_dark)).view(is_dark)
}

fn custom_value_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"const PAD_X: PaddingVar = PaddingVar::new("--pad-x");

let style = Style::new()
    .p_value(PaddingValue::px(5.0))   // p-[5px]
    .px_value(PaddingValue::var(PAD_X)); // px-(--pad-x)

let preview = styled_container_with_custom_properties(
    content,
    &style,
    &[("--pad-x", 18.0)],
);"#;

    const PAD_X: PaddingVar = PaddingVar::new("--pad-x");

    let custom = Style::new()
        .p_value(PaddingValue::px(5.0))
        .px_value(PaddingValue::var(PAD_X));

    let visual = preview_surface(
        styled_container_with_custom_properties(
            container(text("p-[5px] + px-(--pad-x)").size(13).color(if is_dark {
                to_color(Color::gray(Scale::S100))
            } else {
                to_color(Color::white())
            }))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fixed(96.0))
            .into(),
            &custom
                .bg(Color::violet(Scale::S500))
                .rounded(BorderRadius::Md),
            &[("--pad-x", 18.0)],
        ),
        is_dark,
    );

    Snippet::new("Custom Property + Arbitrary Value", code, visual).view(is_dark)
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

fn padded_demo<'a>(
    label: &'a str,
    padding: Padding,
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

    styled_container(
        container(inner)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fixed(136.0))
            .into(),
        &Style::new()
            .bg(Color::gray(if is_dark { Scale::S800 } else { Scale::S100 }))
            .rounded(BorderRadius::Md)
            .padding(padding),
    )
    .width(Length::Fill)
    .into()
}
