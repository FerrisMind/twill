//! Iced backend for twill.
//!
//! Converts twill styles to iced types.

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, Scale, Spacing, Cursor, Blur, AspectRatio, Shadow, FontSize, FontWeight, TransitionDuration, SemanticColor, SemanticThemeVars};
use crate::traits::ToCss;

/// Convert twill Color to iced Color.
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

/// Convert twill ColorValue to iced Color.
pub fn to_color_value(value: crate::tokens::ColorValue) -> iced::Color {
    iced::Color::from_rgba(
        value.r as f32 / 255.0,
        value.g as f32 / 255.0,
        value.b as f32 / 255.0,
        value.a,
    )
}

/// Convert twill Spacing to iced Padding.
pub fn to_padding(spacing: Spacing) -> iced::Padding {
    let rem = spacing.to_rem().unwrap_or(0.0);
    let px = rem * 16.0;
    iced::Padding::new(px)
}

/// Convert twill BorderRadius to iced border radius.
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

/// Convert twill Blur to iced blur radius (f32).
pub fn to_blur_radius(blur: Blur) -> f32 {
    match blur {
        Blur::None => 0.0,
        Blur::Sm => 4.0,
        Blur::Base => 8.0,
        Blur::Md => 12.0,
        Blur::Lg => 16.0,
        Blur::Xl => 24.0,
        Blur::S2xl => 40.0,
        Blur::S3xl => 64.0,
        Blur::Custom(px) => px as f32,
    }
}

/// Convert twill AspectRatio to iced f32
pub fn to_aspect_ratio(ratio: AspectRatio) -> Option<f32> {
    match ratio {
        AspectRatio::Auto => None,
        AspectRatio::Square => Some(1.0),
        AspectRatio::Video => Some(16.0 / 9.0),
        AspectRatio::Custom(w, h) => Some(w as f32 / h as f32),
    }
}

/// Convert twill Shadow to iced Shadow
pub fn to_shadow(shadow: Shadow, color: Color) -> iced::Shadow {
    let (offset_y, blur, alpha) = match shadow {
        Shadow::None => return iced::Shadow::default(),
        Shadow::Xs2 => (1.0, 0.0, 0.05),
        Shadow::Xs => (1.0, 2.0, 0.05),
        Shadow::Sm => (1.0, 3.0, 0.1),
        Shadow::Md => (4.0, 6.0, 0.1),
        Shadow::Lg => (10.0, 15.0, 0.26),
        Shadow::Xl => (20.0, 25.0, 0.26),
        Shadow::S2xl => (25.0, 50.0, 0.63),
    };

    let mut c = to_color(color);
    c.a = alpha;

    iced::Shadow {
        color: c,
        offset: iced::Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

/// Convert twill FontSize to f32 for iced
pub fn to_font_size(size: FontSize) -> f32 {
    size.size_rem() * 16.0
}

/// Convert twill FontWeight to iced font Weight
pub fn to_font_weight(weight: FontWeight) -> iced::font::Weight {
    match weight {
        FontWeight::Thin => iced::font::Weight::Thin,
        FontWeight::ExtraLight => iced::font::Weight::ExtraLight,
        FontWeight::Light => iced::font::Weight::Light,
        FontWeight::Normal => iced::font::Weight::Normal,
        FontWeight::Medium => iced::font::Weight::Medium,
        FontWeight::SemiBold => iced::font::Weight::Semibold,
        FontWeight::Bold => iced::font::Weight::Bold,
        FontWeight::ExtraBold => iced::font::Weight::ExtraBold,
        FontWeight::Black => iced::font::Weight::Black,
    }
}

/// Convert twill SemanticColor to iced Color based on the theme variant
pub fn to_semantic_color(semantic: SemanticColor, is_dark: bool) -> iced::Color {
    let color = SemanticThemeVars::shadcn_neutral().resolve(semantic, is_dark).unwrap_or(Color::black());
    to_color(color)
}

/// Convert twill TransitionDuration to std::time::Duration for iced
pub fn to_duration(duration: TransitionDuration) -> std::time::Duration {
    std::time::Duration::from_millis(duration.as_millis() as u64)
}

/// Convert twill Cursor to iced mouse Interaction.
pub fn to_interaction(cursor: Cursor) -> iced::mouse::Interaction {
    match cursor {
        Cursor::Auto | Cursor::Default => iced::mouse::Interaction::Idle,
        Cursor::Pointer => iced::mouse::Interaction::Pointer,
        Cursor::Wait => iced::mouse::Interaction::Wait,
        Cursor::Progress => iced::mouse::Interaction::Progress,
        Cursor::Text | Cursor::VerticalText => iced::mouse::Interaction::Text,
        Cursor::Move => iced::mouse::Interaction::Move,
        Cursor::Help => iced::mouse::Interaction::Help,
        Cursor::NotAllowed => iced::mouse::Interaction::NotAllowed,
        Cursor::NoDrop => iced::mouse::Interaction::NoDrop,
        Cursor::None => iced::mouse::Interaction::None,
        Cursor::ContextMenu => iced::mouse::Interaction::ContextMenu,
        Cursor::Cell => iced::mouse::Interaction::Cell,
        Cursor::Crosshair => iced::mouse::Interaction::Crosshair,
        Cursor::Alias => iced::mouse::Interaction::Alias,
        Cursor::Copy => iced::mouse::Interaction::Copy,
        Cursor::Grab => iced::mouse::Interaction::Grab,
        Cursor::Grabbing => iced::mouse::Interaction::Grabbing,
        Cursor::AllScroll => iced::mouse::Interaction::AllScroll,
        Cursor::ColResize => iced::mouse::Interaction::ResizingColumn,
        Cursor::RowResize => iced::mouse::Interaction::ResizingRow,
        Cursor::NResize | Cursor::SResize | Cursor::NsResize => iced::mouse::Interaction::ResizingVertically,
        Cursor::EResize | Cursor::WResize | Cursor::EwResize => iced::mouse::Interaction::ResizingHorizontally,
        Cursor::NeResize | Cursor::SwResize | Cursor::NeswResize => iced::mouse::Interaction::ResizingDiagonallyUp,
        Cursor::NwResize | Cursor::SeResize | Cursor::NwseResize => iced::mouse::Interaction::ResizingDiagonallyDown,
        Cursor::ZoomIn => iced::mouse::Interaction::ZoomIn,
        Cursor::ZoomOut => iced::mouse::Interaction::ZoomOut,
    }
}

/// Create a styled button using twill colors.
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

/// Create a styled container with twill Style.
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
