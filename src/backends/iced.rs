//! Iced backend for twill.
//!
//! Converts twill styles to iced types.

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, Scale, Spacing, Cursor, Blur, AspectRatio, Shadow, FontSize, FontWeight, TransitionDuration, SemanticColor, SemanticThemeVars};
use crate::traits::ComputeValue;

fn spacing_to_px(spacing: Spacing) -> f32 {
    match spacing.to_px() {
        Some(px) => px as f32,
        None => 0.0,
    }
}

/// Convert twill Color to iced Color.
pub fn to_color(color: Color) -> iced::Color {
    to_color_value(color.compute())
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
    iced::Padding::new(spacing_to_px(spacing))
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
        AspectRatio::Custom(_, 0) => None,
        AspectRatio::Custom(w, h) => Some(w as f32 / h as f32),
    }
}

/// Convert twill Shadow to iced Shadow with an optional color override.
pub fn to_shadow_with_color(shadow: Shadow, color: Option<Color>) -> iced::Shadow {
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

    let mut c = to_color(color.unwrap_or(Color::black()));
    c.a = alpha;

    iced::Shadow {
        color: c,
        offset: iced::Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

/// Convert twill Shadow to iced Shadow.
pub fn to_shadow(shadow: Shadow, color: Color) -> iced::Shadow {
    to_shadow_with_color(shadow, Some(color))
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
) -> iced::widget::Container<'a, Message> {
    let mut container = iced::widget::container(content);

    // Padding
    if let Some(p) = &style.padding
        && let (Some(t), Some(r), Some(b), Some(l)) = (p.top, p.right, p.bottom, p.left)
    {
        let top = spacing_to_px(t);
        let right = spacing_to_px(r);
        let bottom = spacing_to_px(b);
        let left = spacing_to_px(l);
        container = container.padding(iced::Padding {
            top,
            right,
            bottom,
            left,
        });
    }

    let bg_color = style.background_color.map(to_color);
    let border_width = style.border_width.map_or(0.0, |w| match w {
        crate::tokens::BorderWidth::S0 => 0.0,
        crate::tokens::BorderWidth::S1 => 1.0,
        crate::tokens::BorderWidth::S2 => 2.0,
        crate::tokens::BorderWidth::S4 => 4.0,
        crate::tokens::BorderWidth::S8 => 8.0,
    });
    let border_radius = style.border_radius.map_or(0.0, to_border_radius);
    let border_color = style
        .border_color
        .map(to_color)
        .unwrap_or(iced::Color::TRANSPARENT);
    let shadow = style
        .box_shadow
        .map(|s| to_shadow_with_color(s, style.shadow_color))
        .unwrap_or_default();

    container = container.style(move |_| iced::widget::container::Style {
        background: bg_color.map(iced::Background::Color),
        border: iced::Border {
            radius: border_radius.into(),
            width: border_width,
            color: border_color,
        },
        shadow,
        ..Default::default()
    });

    container
}

/// Create an `iced` button directly from `twill::Button`.
pub fn twill_button<'a, Message: Clone + 'a>(
    button_cfg: &crate::components::Button,
    label: &'a str,
    on_press: Message,
) -> iced::Element<'a, Message> {
    let style = button_cfg.style();
    let bg = style.background_color.unwrap_or(Color::blue(Scale::S500));
    let text_color = style.text_color.unwrap_or(Color::white());
    let border_color = style
        .border_color
        .map(to_color)
        .unwrap_or(iced::Color::TRANSPARENT);
    let border_width = style.border_width.map_or(0.0, |w| match w {
        crate::tokens::BorderWidth::S0 => 0.0,
        crate::tokens::BorderWidth::S1 => 1.0,
        crate::tokens::BorderWidth::S2 => 2.0,
        crate::tokens::BorderWidth::S4 => 4.0,
        crate::tokens::BorderWidth::S8 => 8.0,
    });
    let border_radius = style.border_radius.map_or(6.0, to_border_radius);
    let padding = style
        .padding
        .and_then(|p| match (p.top, p.right, p.bottom, p.left) {
            (Some(top), Some(right), Some(bottom), Some(left)) => Some(iced::Padding {
                top: spacing_to_px(top),
                right: spacing_to_px(right),
                bottom: spacing_to_px(bottom),
                left: spacing_to_px(left),
            }),
            _ => None,
        })
        .unwrap_or(iced::Padding {
            top: 8.0,
            right: 16.0,
            bottom: 8.0,
            left: 16.0,
        });

    let mut widget = iced::widget::button(
        iced::widget::text(label).color(to_color(text_color)),
    )
    .padding(padding)
    .style(move |_theme, status| {
        let mut background = to_color(bg);
        match status {
            iced::widget::button::Status::Hovered => {
                background.a *= 0.9;
            }
            iced::widget::button::Status::Pressed => {
                background.a *= 0.8;
            }
            _ => {}
        }

        iced::widget::button::Style {
            background: Some(iced::Background::Color(background)),
            text_color: to_color(text_color),
            border: iced::Border {
                radius: border_radius.into(),
                width: border_width,
                color: border_color,
            },
            ..Default::default()
        }
    });

    if !button_cfg.disabled {
        widget = widget.on_press(on_press);
    }

    widget.into()
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

    #[test]
    fn test_color_conversion_uses_raw_values() {
        let color = Color::blue(Scale::S500);
        let converted = to_color(color);
        let raw = color.compute();
        assert!((converted.r - raw.r as f32 / 255.0).abs() < 0.001);
        assert!((converted.g - raw.g as f32 / 255.0).abs() < 0.001);
        assert!((converted.b - raw.b as f32 / 255.0).abs() < 0.001);
    }

    #[test]
    fn test_spacing_px_padding() {
        let p = to_padding(Spacing::Px);
        assert!((p.top - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aspect_ratio_zero_denominator() {
        assert_eq!(to_aspect_ratio(AspectRatio::Custom(16, 0)), None);
    }

    #[test]
    fn test_shadow_uses_custom_color() {
        let shadow = to_shadow_with_color(Shadow::Sm, Some(Color::red(Scale::S500)));
        assert!(shadow.color.r > shadow.color.g);
    }
}
