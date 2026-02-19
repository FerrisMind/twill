//! Iced backend for rustwind.
//!
//! Converts rustwind styles to iced types.

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, Scale, Spacing};
use crate::traits::ToCss;

/// Convert rustwind Color to iced Color.
pub fn to_color(color: Color) -> iced::Color {
    let css = color.to_css();
    let hex = css.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        iced::Color::from_rgb(r, g, b)
    } else {
        iced::Color::WHITE
    }
}

/// Convert rustwind Spacing to iced Padding.
pub fn to_padding(spacing: Spacing) -> iced::Padding {
    let rem = spacing.to_rem().unwrap_or(0.0);
    let px = rem * 16.0;
    iced::Padding::new(px)
}

/// Convert rustwind BorderRadius to iced border radius.
pub fn to_border_radius(radius: BorderRadius) -> f32 {
    match radius {
        BorderRadius::None => 0.0,
        BorderRadius::Xs => 2.0,
        BorderRadius::Sm => 4.0,
        BorderRadius::Md => 6.0,
        BorderRadius::Lg => 8.0,
        BorderRadius::Xl => 12.0,
        BorderRadius::S2xl => 16.0,
        BorderRadius::S3xl => 24.0,
        BorderRadius::S4xl => 32.0,
        BorderRadius::Full => 9999.0,
    }
}

/// Create a styled button using rustwind colors.
pub fn styled_button<'a, Message: Clone + 'a>(
    label: &'a str,
    bg_color: Color,
    text_color: Color,
    on_press: Message,
) -> iced::Element<'a, Message> {
    iced::widget::button(iced::widget::text(label).color(to_color(text_color)))
        .style(
            move |_theme: &iced::Theme, status: iced::widget::button::Status| {
                let bg = to_color(bg_color);
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(match status {
                        iced::widget::button::Status::Hovered => iced::Color {
                            a: bg.a * 0.9,
                            ..bg
                        },
                        iced::widget::button::Status::Pressed => iced::Color {
                            a: bg.a * 0.8,
                            ..bg
                        },
                        _ => bg,
                    })),
                    text_color: to_color(text_color),
                    border: iced::Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
        )
        .on_press(on_press)
        .into()
}

/// Create a primary button (blue).
pub fn primary_button<'a, Message: Clone + 'a>(
    label: &'a str,
    on_press: Message,
) -> iced::Element<'a, Message> {
    styled_button(label, Color::blue(Scale::S500), Color::white(), on_press)
}

/// Create a secondary button (gray).
pub fn secondary_button<'a, Message: Clone + 'a>(
    label: &'a str,
    on_press: Message,
) -> iced::Element<'a, Message> {
    styled_button(
        label,
        Color::gray(Scale::S100),
        Color::gray(Scale::S900),
        on_press,
    )
}

/// Create a danger button (red).
pub fn danger_button<'a, Message: Clone + 'a>(
    label: &'a str,
    on_press: Message,
) -> iced::Element<'a, Message> {
    styled_button(label, Color::red(Scale::S500), Color::white(), on_press)
}

/// Create a styled container with rustwind Style.
pub fn styled_container<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
) -> iced::Element<'a, Message> {
    let mut container = iced::widget::container(content);

    // Padding
    if let Some(p) = &style.padding
        && let (Some(t), Some(r), Some(b), Some(l)) = (p.top, p.right, p.bottom, p.left)
    {
        let top = t.to_rem().unwrap_or(0.0) * 16.0;
        let right = r.to_rem().unwrap_or(0.0) * 16.0;
        let bottom = b.to_rem().unwrap_or(0.0) * 16.0;
        let left = l.to_rem().unwrap_or(0.0) * 16.0;
        // Use uniform padding (average)
        container = container.padding(iced::Padding::new((top + right + bottom + left) / 4.0));
    }

    // Background
    if let Some(bg) = &style.background_color {
        let bg_color = to_color(*bg);
        container = container.style(move |_| {
            iced::widget::container::Style::default().background(iced::Background::Color(bg_color))
        });
    }

    container.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let blue = Color::blue(Scale::S500);
        let c = to_color(blue);
        assert!((c.r - 59.0 / 255.0).abs() < 0.01);
        assert!((c.g - 130.0 / 255.0).abs() < 0.01);
        assert!((c.b - 246.0 / 255.0).abs() < 0.01);
    }
}
