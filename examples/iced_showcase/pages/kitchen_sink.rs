use iced::widget::{column, row, text, Space};
use iced::{Element, Length};
use crate::Message;
use twill::tokens::{BorderRadius, Color, FontSize, FontWeight, Scale, Shadow, Spacing};
use twill::utilities::Padding;
use twill::iced::{styled_container, 
    primary_button, secondary_button, danger_button, styled_button, to_color, to_font_size, to_font_weight,
};

/// Kitchen Sink: combines representative elements from every token category
/// into a single scrollable view for quick visual regression testing.
pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    let text_col = if is_dark { to_color(Color::white()) } else { to_color(Color::black()) };
    let subtle_text = to_color(Color::gray(Scale::S400));

    // ── Color palette row ───────────────────────────────────────────
    let color_families = [
        Color::red(Scale::S500),
        Color::orange(Scale::S500),
        Color::amber(Scale::S500),
        Color::green(Scale::S500),
        Color::teal(Scale::S500),
        Color::blue(Scale::S500),
        Color::indigo(Scale::S500),
        Color::violet(Scale::S500),
        Color::pink(Scale::S500),
        Color::rose(Scale::S500),
    ];

    let mut color_row = row![].spacing(6);
    for color in color_families {
        color_row = color_row.push(
            styled_container(
                Space::new().width(Length::Fixed(40.0)).height(Length::Fixed(40.0)).into(),
                &twill::Style::new().bg(color).rounded(BorderRadius::Md)
            )
        );
    }

    // ── Typography samples ──────────────────────────────────────────
    let typo = column![
        text("Heading 1").size(to_font_size(FontSize::S3xl)).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        text("Heading 2").size(to_font_size(FontSize::S2xl)).font(iced::Font { weight: to_font_weight(FontWeight::SemiBold), ..Default::default() }),
        text("Body text rendered at base size.").size(to_font_size(FontSize::Base)),
        text("Small caption text").size(to_font_size(FontSize::Sm)).color(subtle_text),
    ].spacing(6);

    // ── Border radius samples ───────────────────────────────────────
    let radii = [BorderRadius::Sm, BorderRadius::Md, BorderRadius::Lg, BorderRadius::Xl, BorderRadius::Full];
    let mut radius_row = row![].spacing(12);
    for r in radii {
        radius_row = radius_row.push(
            styled_container(
                Space::new().width(Length::Fixed(50.0)).height(Length::Fixed(50.0)).into(),
                &twill::Style::new().bg(Color::sky(Scale::S500)).rounded(r)
            )
        );
    }

    // ── Shadow samples ──────────────────────────────────────────────
    let box_bg_twill = if is_dark { Color::gray(Scale::S800) } else { Color::white() };
    let shadow_col_twill = if is_dark { Color::black() } else { Color::gray(Scale::S900) };

    let shadow_variants = [Shadow::Sm, Shadow::Md, Shadow::Lg, Shadow::Xl];
    let mut shadow_row = row![].spacing(24).padding(16);
    for shadow in shadow_variants {
        shadow_row = shadow_row.push(
            styled_container(
                Space::new().width(Length::Fixed(80.0)).height(Length::Fixed(50.0)).into(),
                &twill::Style::new()
                    .bg(box_bg_twill)
                    .rounded(BorderRadius::Lg)
                    .shadow(shadow)
                    .shadow_color(shadow_col_twill)
            )
        );
    }

    // ── Button samples ──────────────────────────────────────────────
    let button_row = row![
        primary_button("Primary", Message::NoOp),
        secondary_button("Secondary", Message::NoOp),
        danger_button("Danger", Message::NoOp),
        styled_button("Custom", Color::purple(Scale::S500), Color::white(), Message::NoOp),
    ].spacing(12);

    // ── Spacing sample ──────────────────────────────────────────────
    let spacing_vals = [Spacing::S1, Spacing::S2, Spacing::S4, Spacing::S8, Spacing::S16];
    let mut spacing_row = row![].spacing(8);
    for sp in spacing_vals {
        spacing_row = spacing_row.push(
            styled_container(
                styled_container(
                    Space::new().width(Length::Fill).height(Length::Fill).into(),
                    &twill::Style::new().bg(Color::blue(Scale::S500))
                ).into(),
                &twill::Style::new()
                    .bg(Color::red(Scale::S200))
                    .padding(Padding::all(sp))
            )
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(60.0))
        );
    }

    // ── Assemble kitchen sink ───────────────────────────────────────
    column![
        text("Kitchen Sink").size(32).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        text("All token categories at a glance for visual regression testing.").size(16).color(subtle_text),

        section("Color Palette", color_row, text_col),
        section("Typography", typo, text_col),
        section("Border Radius", radius_row, text_col),
        section("Shadows", shadow_row, text_col),
        section("Buttons", button_row, text_col),
        section("Spacing (Padding)", spacing_row, text_col),
    ]
    .spacing(24)
    .into()
}

fn section<'a>(
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
    text_col: iced::Color,
) -> Element<'a, Message> {
    column![
        text(title).size(18).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }).color(text_col),
        Space::new().width(Length::Shrink).height(Length::Fixed(8.0)),
        content.into(),
    ]
    .spacing(4)
    .into()
}
