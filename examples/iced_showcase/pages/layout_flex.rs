use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};
use twill::backends::iced::{
    apply_flex_item, apply_flex_item_with_custom_properties, styled_container, to_color,
    to_font_weight,
};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{FlexDirection, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("flex").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Utilities for controlling how flex items both grow and shrink.").size(16),
        classes_section(is_dark),
        number_section(is_dark),
        fraction_section(is_dark),
        initial_section(is_dark),
        auto_section(is_dark),
        none_section(is_dark),
        custom_property_section(is_dark),
        arbitrary_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn classes_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Tailwind classes covered in Twill:
// flex-<number>            -> Style::new().flex_number(n)
// flex-<fraction>          -> Style::new().flex_fraction(num, den)
// flex-auto                -> Style::new().flex_auto()
// flex-initial             -> Style::new().flex_initial()
// flex-none                -> Style::new().flex_none()
// flex-(<custom-property>) -> Style::new().flex_custom_property("--var")
// flex-[<value>]           -> Style::new().flex_arbitrary("3_1_auto")

let parsed = twill::Flex::from_tailwind_class("flex-1/2");"#;

    let visual = text("All class forms from Tailwind `flex.mdx` are modeled explicitly.").size(13);

    Snippet::new("Flex: Class Coverage", code, visual).view(is_dark)
}

fn number_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let row = row![
    apply_flex_item(tile("01"), &Style::new().flex_none(), FlexDirection::Row),
    apply_flex_item(tile("02"), &Style::new().flex_number(1), FlexDirection::Row), // flex-1
    apply_flex_item(tile("03"), &Style::new().flex_number(2), FlexDirection::Row), // flex-2
];"#;

    let visual = preview_surface(
        row![
            apply_flex_item(
                fixed_tile("01", Color::pink(Scale::S300), 56.0, is_dark),
                &Style::new().flex_none(),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fluid_tile("02", Color::pink(Scale::S500), is_dark),
                &Style::new().flex_number(1),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fluid_tile("03", Color::pink(Scale::S500), is_dark),
                &Style::new().flex_number(2),
                FlexDirection::Row,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .width(Length::Fill),
        is_dark,
    );

    Snippet::new("Flex: Number (`flex-<number>`)", code, visual).view(is_dark)
}

fn fraction_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let row = row![
    apply_flex_item(tile("A"), &Style::new().flex_fraction(1, 2), FlexDirection::Row), // flex-1/2
    apply_flex_item(tile("B"), &Style::new().flex_fraction(1, 2), FlexDirection::Row), // flex-1/2
];"#;

    let visual = preview_surface(
        row![
            apply_flex_item(
                fluid_tile("A", Color::emerald(Scale::S500), is_dark),
                &Style::new().flex_fraction(1, 2),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fluid_tile("B", Color::emerald(Scale::S500), is_dark),
                &Style::new().flex_fraction(1, 2),
                FlexDirection::Row,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .width(Length::Fill),
        is_dark,
    );

    Snippet::new("Flex: Fraction (`flex-<fraction>`)", code, visual).view(is_dark)
}

fn initial_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let row = row![
    apply_flex_item(tile("01"), &Style::new().flex_none(), FlexDirection::Row),
    apply_flex_item(tile("02"), &Style::new().flex_initial(), FlexDirection::Row),
    apply_flex_item(tile("03"), &Style::new().flex_initial(), FlexDirection::Row),
];"#;

    let visual = preview_surface(
        row![
            apply_flex_item(
                fixed_tile("01", Color::blue(Scale::S300), 56.0, is_dark),
                &Style::new().flex_none(),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fixed_tile("02", Color::blue(Scale::S500), 192.0, is_dark),
                &Style::new().flex_initial(),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fixed_tile("03", Color::blue(Scale::S500), 128.0, is_dark),
                &Style::new().flex_initial(),
                FlexDirection::Row,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .width(Length::Fill),
        is_dark,
    );

    Snippet::new("Flex: Initial", code, visual).view(is_dark)
}

fn auto_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let row = row![
    apply_flex_item(tile("01"), &Style::new().flex_none(), FlexDirection::Row),
    apply_flex_item(tile("02"), &Style::new().flex_auto(), FlexDirection::Row),
    apply_flex_item(tile("03"), &Style::new().flex_auto(), FlexDirection::Row),
];"#;

    let visual = column![
        preview_surface(
            row![
                apply_flex_item(
                    fixed_tile("01", Color::violet(Scale::S300), 56.0, is_dark),
                    &Style::new().flex_none(),
                    FlexDirection::Row,
                ),
                apply_flex_item(
                    fluid_tile("02", Color::violet(Scale::S500), is_dark),
                    &Style::new().flex_auto(),
                    FlexDirection::Row,
                ),
                apply_flex_item(
                    fluid_tile("03", Color::violet(Scale::S500), is_dark),
                    &Style::new().flex_auto(),
                    FlexDirection::Row,
                ),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .width(Length::Fill),
            is_dark,
        ),
        text("Iced mapping: `flex-auto` -> `Length::Fill` on the main axis.").size(12)
    ]
    .spacing(8);

    Snippet::new("Flex: Auto", code, visual).view(is_dark)
}

fn none_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let row = row![
    apply_flex_item(tile("01"), &Style::new().flex_none(), FlexDirection::Row),
    apply_flex_item(tile("02"), &Style::new().flex_none(), FlexDirection::Row),
    apply_flex_item(tile("03"), &Style::new().flex_1(), FlexDirection::Row),
];"#;

    let visual = preview_surface(
        row![
            apply_flex_item(
                fixed_tile("01", Color::indigo(Scale::S500), 56.0, is_dark),
                &Style::new().flex_none(),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fixed_tile("02", Color::indigo(Scale::S500), 128.0, is_dark),
                &Style::new().flex_none(),
                FlexDirection::Row,
            ),
            apply_flex_item(
                fluid_tile(
                    "03",
                    if is_dark {
                        Color::indigo(Scale::S800)
                    } else {
                        Color::indigo(Scale::S300)
                    },
                    is_dark,
                ),
                &Style::new().flex_1(),
                FlexDirection::Row,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .width(Length::Fill),
        is_dark,
    );

    Snippet::new("Flex: None", code, visual).view(is_dark)
}

fn custom_property_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let style = Style::new().flex_custom_property("--my-flex"); // flex-(--my-flex)

let vars = [("--my-flex", "2")];
let item = apply_flex_item_with_custom_properties(content, &style, FlexDirection::Row, &vars);
"#;

    let visual = column![
        preview_surface(
            row![
                apply_flex_item_with_custom_properties(
                    fixed_tile("Var", Color::amber(Scale::S500), 96.0, is_dark),
                    &Style::new().flex_custom_property("--my-flex"),
                    FlexDirection::Row,
                    &[("--my-flex", "2")],
                ),
                apply_flex_item(
                    fluid_tile("Fill", Color::amber(Scale::S300), is_dark),
                    &Style::new().flex_1(),
                    FlexDirection::Row,
                ),
            ]
            .spacing(12)
            .width(Length::Fill),
            is_dark,
        ),
        text("`flex-(<custom-property>)` resolves through provided variables in iced.").size(12),
    ]
    .spacing(8);

    Snippet::new(
        "Flex: Custom Property (`flex-(<custom-property>)`)",
        code,
        visual,
    )
    .view(is_dark)
}

fn arbitrary_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let row = row![
    apply_flex_item(tile("N"), &Style::new().flex_arbitrary("2"), FlexDirection::Row),       // flex-[2]
    apply_flex_item(tile("F"), &Style::new().flex_arbitrary("1/2"), FlexDirection::Row),     // flex-[1/2]
    apply_flex_item(tile("A"), &Style::new().flex_arbitrary("auto"), FlexDirection::Row),    // flex-[auto]
    apply_flex_item(tile("R"), &Style::new().flex_arbitrary("3_1_auto"), FlexDirection::Row),// flex-[3_1_auto]
];
"#;

    let visual = column![
        preview_surface(
            row![
                apply_flex_item(
                    fluid_tile("N", Color::sky(Scale::S500), is_dark),
                    &Style::new().flex_arbitrary("2"),
                    FlexDirection::Row,
                ),
                apply_flex_item(
                    fluid_tile("F", Color::sky(Scale::S500), is_dark),
                    &Style::new().flex_arbitrary("1/2"),
                    FlexDirection::Row,
                ),
                apply_flex_item(
                    fluid_tile("A", Color::sky(Scale::S500), is_dark),
                    &Style::new().flex_arbitrary("auto"),
                    FlexDirection::Row,
                ),
                apply_flex_item(
                    fluid_tile("R", Color::sky(Scale::S300), is_dark),
                    &Style::new().flex_arbitrary("3_1_auto"),
                    FlexDirection::Row,
                ),
            ]
            .spacing(12)
            .width(Length::Fill),
            is_dark,
        ),
        text("Arbitrary values are parsed best-effort for iced; unsupported forms degrade safely.")
            .size(12),
    ]
    .spacing(8);

    Snippet::new("Flex: Arbitrary (`flex-[<value>]`)", code, visual).view(is_dark)
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

fn fixed_tile<'a>(label: &'a str, bg: Color, width: f32, is_dark: bool) -> Element<'a, Message> {
    container(fluid_tile(label, bg, is_dark))
        .width(Length::Fixed(width))
        .into()
}

fn fluid_tile<'a>(label: &'a str, bg: Color, is_dark: bool) -> Element<'a, Message> {
    let text_color = if is_dark {
        to_color(Color::gray(Scale::S100))
    } else {
        to_color(Color::white())
    };

    styled_container(
        container(text(label).size(14).color(text_color))
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
