use crate::Message;
use crate::components::Snippet;
use iced::widget::{column, container, text};
use iced::{Element, Length};
use twill::backends::iced::{
    grid_template_columns_layout, grid_template_columns_layout_with_context, styled_container,
    to_color, to_font_weight,
};
use twill::style::Style;
use twill::tokens::{BorderRadius, Color, FontWeight, Scale, Spacing};
use twill::utilities::{GridTemplate, Padding};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("grid-template-columns").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Typed utilities for specifying grid column tracks.").size(16),
        variants_section(is_dark),
        count_section(is_dark),
        none_section(is_dark),
        subgrid_section(is_dark),
        custom_property_section(is_dark),
        arbitrary_value_section(is_dark),
    ]
    .spacing(32)
    .into()
}

fn variants_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let count = GridTemplate::count(4);
let none = GridTemplate::none();
let subgrid = GridTemplate::subgrid();
let custom = GridTemplate::custom_property("--layout-cols");
let arbitrary = GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px");"#;

    let visual =
        text("Template columns are modeled as enum variants and tokenized builders.").size(13);

    Snippet::new("Grid Template Columns: Variant Families", code, visual).view(is_dark)
}

fn count_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let template = GridTemplate::count(4);
let preview = grid_template_columns_layout(items, template, Spacing::S4);"#;

    let visual = preview_surface(
        grid_template_columns_layout(
            sample_tiles(Color::fuchsia(Scale::S500), is_dark),
            GridTemplate::count(4),
            Spacing::S4,
        ),
        is_dark,
    );

    Snippet::new("Count", code, visual).view(is_dark)
}

fn none_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let template = GridTemplate::none();
let preview = grid_template_columns_layout(items, template, Spacing::S4);"#;

    let visual = preview_surface(
        container(grid_template_columns_layout(
            sample_tiles(Color::indigo(Scale::S500), is_dark),
            GridTemplate::none(),
            Spacing::S4,
        ))
        .width(Length::Fixed(220.0)),
        is_dark,
    );

    Snippet::new("None", code, visual).view(is_dark)
}

fn subgrid_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let template = GridTemplate::subgrid();
let inherited_tracks = Some(3);
let preview = grid_template_columns_layout_with_context(
    items,
    template,
    Spacing::S4,
    inherited_tracks,
    &[],
);"#;

    let visual = preview_surface(
        grid_template_columns_layout_with_context(
            sample_tiles(Color::emerald(Scale::S500), is_dark),
            GridTemplate::subgrid(),
            Spacing::S4,
            Some(3),
            &[],
        ),
        is_dark,
    );

    Snippet::new("Subgrid", code, visual).view(is_dark)
}

fn custom_property_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let template = GridTemplate::custom_property("--layout-cols");
let preview = grid_template_columns_layout_with_context(
    items,
    template,
    Spacing::S4,
    None,
    &[("--layout-cols", "repeat(5, minmax(0, 1fr))")],
);"#;

    let visual = preview_surface(
        grid_template_columns_layout_with_context(
            sample_tiles(Color::blue(Scale::S500), is_dark),
            GridTemplate::custom_property("--layout-cols"),
            Spacing::S4,
            None,
            &[("--layout-cols", "repeat(5, minmax(0, 1fr))")],
        ),
        is_dark,
    );

    Snippet::new("Custom Property", code, visual).view(is_dark)
}

fn arbitrary_value_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let template = GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px");
let preview = grid_template_columns_layout(items, template, Spacing::S4);"#;

    let visual = preview_surface(
        grid_template_columns_layout(
            sample_tiles(Color::violet(Scale::S500), is_dark),
            GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px"),
            Spacing::S4,
        ),
        is_dark,
    );

    Snippet::new("Arbitrary Value", code, visual).view(is_dark)
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

fn sample_tiles<'a>(bg: Color, is_dark: bool) -> Vec<Element<'a, Message>> {
    vec![
        tile("01", bg, is_dark),
        tile("02", bg, is_dark),
        tile("03", bg, is_dark),
        tile("04", bg, is_dark),
        tile("05", bg, is_dark),
        tile("06", bg, is_dark),
        tile("07", bg, is_dark),
        tile("08", bg, is_dark),
        tile("09", bg, is_dark),
    ]
}

fn tile<'a>(label: &'a str, bg: Color, is_dark: bool) -> Element<'a, Message> {
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
    .into()
}
