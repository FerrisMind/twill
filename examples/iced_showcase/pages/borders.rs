use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text};
use iced::{Element, Length};
use twill::iced::{styled_container, to_color, to_font_weight};
use twill::tokens::{BorderRadius, BorderStyle, BorderWidth, Color, FontWeight, Scale};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("Borders").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Standard web token borders map cleanly to iced elements.").size(16),
        border_radius_section(is_dark),
        border_width_section(is_dark),
        border_style_section(is_dark),
    ]
    .spacing(32)
    .into()
}

// ── Border Radius ───────────────────────────────────────────────────

fn border_radius_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let radius = BorderRadius::Lg;
styled_container(content, &twill::Style::new()
    .bg(Color::blue(Scale::S500))
    .rounded(radius))"#;

    let radii = [
        (BorderRadius::None, "None"),
        (BorderRadius::Xs, "Xs"),
        (BorderRadius::Sm, "Sm"),
        (BorderRadius::Md, "Md"),
        (BorderRadius::Lg, "Lg"),
        (BorderRadius::Xl, "Xl"),
        (BorderRadius::S2xl, "2XL"),
        (BorderRadius::S3xl, "3XL"),
        (BorderRadius::S4xl, "4XL"),
        (BorderRadius::Full, "Full"),
    ];

    let mut blocks = column![].spacing(24);
    let mut current_row = row![].spacing(24);
    let mut count = 0;

    for (radius, label) in radii {
        let r_val = twill::iced::to_border_radius(radius);

        let c = styled_container(
            text(label).size(14).color(to_color(Color::white())).into(),
            &twill::Style::new()
                .bg(Color::blue(Scale::S500))
                .rounded(radius),
        )
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(100.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center);

        let info = column![
            c,
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(8.0)),
            text(format!("{} ({}px)", label, r_val)).size(12)
        ]
        .align_x(iced::Alignment::Center);

        current_row = current_row.push(info);
        count += 1;

        if count == 5 {
            blocks = blocks.push(current_row);
            current_row = row![].spacing(24);
            count = 0;
        }
    }

    if count > 0 {
        blocks = blocks.push(current_row);
    }

    Snippet::new("Border Radius", code, blocks).view(is_dark)
}

// ── Border Width ────────────────────────────────────────────────────

fn border_width_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// BorderWidth tokens: S0..S8
styled_container(content, &twill::Style::new()
    .border(BorderWidth::S4, BorderStyle::Solid, Color::blue(Scale::S500))
    .rounded(BorderRadius::Lg))"#;

    let widths = [
        (BorderWidth::S0, "S0 (0px)"),
        (BorderWidth::S1, "S1 (1px)"),
        (BorderWidth::S2, "S2 (2px)"),
        (BorderWidth::S4, "S4 (4px)"),
        (BorderWidth::S8, "S8 (8px)"),
    ];

    let _border_col = if is_dark {
        to_color(Color::blue(Scale::S400))
    } else {
        to_color(Color::blue(Scale::S500))
    };

    let text_col = if is_dark {
        to_color(Color::white())
    } else {
        to_color(Color::black())
    };

    let mut blocks = column![].spacing(16);
    let mut row_content = row![].spacing(16);
    let mut count = 0;
    for (width, label) in widths {
        let _w_px: f32 = match width {
            BorderWidth::S0 => 0.0,
            BorderWidth::S1 => 1.0,
            BorderWidth::S2 => 2.0,
            BorderWidth::S4 => 4.0,
            BorderWidth::S8 => 8.0,
        };

        let border_col_token = if is_dark {
            Color::blue(Scale::S400)
        } else {
            Color::blue(Scale::S500)
        };
        let c = styled_container(
            text(label).size(14).color(text_col).into(),
            &twill::Style::new()
                .bg(if is_dark {
                    Color::gray(Scale::S900)
                } else {
                    Color::white()
                })
                .border(width, BorderStyle::Solid, border_col_token)
                .rounded(BorderRadius::Lg),
        )
        .width(Length::Fixed(120.0))
        .height(Length::Fixed(80.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center);

        row_content = row_content.push(c);
        count += 1;
        if count == 3 {
            blocks = blocks.push(row_content);
            row_content = row![].spacing(16);
            count = 0;
        }
    }

    if count > 0 {
        blocks = blocks.push(row_content);
    }

    Snippet::new("Border Width", code, blocks).view(is_dark)
}

// ── Border Style ────────────────────────────────────────────────────

fn border_style_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// BorderStyle now rendered by twill::iced::styled_container
// using custom backend drawing when iced lacks native support."#;

    let styles = [
        (BorderStyle::Solid, "Solid"),
        (BorderStyle::Dashed, "Dashed"),
        (BorderStyle::Dotted, "Dotted"),
        (BorderStyle::Double, "Double"),
        (BorderStyle::Hidden, "Hidden"),
        (BorderStyle::None, "None"),
    ];

    let text_col = if is_dark {
        to_color(Color::white())
    } else {
        to_color(Color::black())
    };

    let mut blocks = column![].spacing(16);
    let mut row_content = row![].spacing(16);
    let mut count = 0;
    for (style, label) in styles {
        let border_col = if is_dark {
            Color::blue(Scale::S400)
        } else {
            Color::blue(Scale::S500)
        };

        let content = styled_container(
            text(label).size(13).color(text_col).into(),
            &twill::Style::new()
                .bg(if is_dark {
                    Color::gray(Scale::S900)
                } else {
                    Color::white()
                })
                .border(BorderWidth::S2, style, border_col)
                .rounded(BorderRadius::Md),
        )
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(70.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center);

        let info = column![
            content,
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(4.0)),
            text(format!("{} — {}", label, style.keyword())).size(11),
        ];

        row_content = row_content.push(info);
        count += 1;
        if count == 3 {
            blocks = blocks.push(row_content);
            row_content = row![].spacing(16);
            count = 0;
        }
    }

    if count > 0 {
        blocks = blocks.push(row_content);
    }

    Snippet::new("Border Style", code, blocks).view(is_dark)
}
