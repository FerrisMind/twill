use crate::backends::ShadowColor;
use crate::tokens::{
    AspectRatio, BackgroundColor, Blur, BorderRadius, Color, ColorValue, Cursor, FontSize,
    FontWeight, Percentage, SemanticColor, SemanticThemeVars, Shadow, Spacing, TextAlign,
    ThemeVariant, TransitionDuration,
};
use crate::traits::ComputeValue;
use crate::utilities::{MarginValue, ObjectFit, PaddingValue};
use iced::{ContentFit, Length};

use super::common::{apply_opacity_to_color, container_to_px, spacing_to_px};
pub use crate::backends::iced::convert::TextDirection;

pub fn to_color(color: Color) -> iced::Color {
    to_color_value(color.compute())
}

/// Convert twill ColorValue to iced Color.
pub fn to_color_value(value: crate::tokens::ColorValue) -> iced::Color {
    let (r, g, b) = value.to_rgb8();
    iced::Color::from_rgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        value.a,
    )
}

pub(super) fn resolve_background_color_token(
    token: BackgroundColor,
    fallback_text: Option<Color>,
) -> Option<ColorValue> {
    match token {
        BackgroundColor::Inherit => None,
        BackgroundColor::Current => fallback_text.map(|text| text.compute()),
        BackgroundColor::Transparent => Some(ColorValue::TRANSPARENT),
        BackgroundColor::Palette(color) => Some(color.compute()),
        BackgroundColor::CustomProperty(_) => None,
        BackgroundColor::Arbitrary(value) => Some(value.into()),
    }
}

/// Convert twill Spacing to iced Padding.
pub fn to_padding(spacing: Spacing) -> iced::Padding {
    iced::Padding::new(spacing_to_px(spacing))
}

fn resolve_padding_value_px(value: PaddingValue, custom_properties: &[(&str, f32)]) -> f32 {
    match value {
        PaddingValue::Scale(spacing) => spacing_to_px(spacing),
        PaddingValue::Px(px) => {
            if px.is_finite() {
                px.max(0.0)
            } else {
                0.0
            }
        }
        PaddingValue::Rem(rem) => {
            if rem.is_finite() {
                (rem * 16.0).max(0.0)
            } else {
                0.0
            }
        }
        PaddingValue::Var(var) => custom_properties
            .iter()
            .find(|(name, _)| *name == var.as_str())
            .map(|(_, value)| value.max(0.0))
            .unwrap_or(0.0),
    }
}

pub(crate) fn to_style_padding(
    padding: crate::utilities::Padding,
    custom_properties: &[(&str, f32)],
) -> iced::Padding {
    iced::Padding {
        top: padding
            .top
            .map(|value| resolve_padding_value_px(value, custom_properties))
            .unwrap_or(0.0),
        right: padding
            .right
            .map(|value| resolve_padding_value_px(value, custom_properties))
            .unwrap_or(0.0),
        bottom: padding
            .bottom
            .map(|value| resolve_padding_value_px(value, custom_properties))
            .unwrap_or(0.0),
        left: padding
            .left
            .map(|value| resolve_padding_value_px(value, custom_properties))
            .unwrap_or(0.0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ResolvedMarginValue {
    Px(f32),
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(super) struct ResolvedMargin {
    pub(super) top: Option<ResolvedMarginValue>,
    pub(super) right: Option<ResolvedMarginValue>,
    pub(super) bottom: Option<ResolvedMarginValue>,
    pub(super) left: Option<ResolvedMarginValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(super) struct MarginOffsets {
    pub(super) top: f32,
    pub(super) right: f32,
    pub(super) bottom: f32,
    pub(super) left: f32,
}

impl MarginOffsets {
    pub(super) fn is_zero(self) -> bool {
        (self.top.abs() < f32::EPSILON)
            && (self.right.abs() < f32::EPSILON)
            && (self.bottom.abs() < f32::EPSILON)
            && (self.left.abs() < f32::EPSILON)
    }
}

fn resolve_margin_value_px(
    value: MarginValue,
    custom_properties: &[(&str, f32)],
) -> ResolvedMarginValue {
    match value {
        MarginValue::Scale(spacing) => ResolvedMarginValue::Px(spacing_to_px(spacing)),
        MarginValue::NegativeScale(spacing) => ResolvedMarginValue::Px(-spacing_to_px(spacing)),
        MarginValue::Px(px) => {
            if px.is_finite() {
                ResolvedMarginValue::Px(px)
            } else {
                ResolvedMarginValue::Px(0.0)
            }
        }
        MarginValue::Rem(rem) => {
            if rem.is_finite() {
                ResolvedMarginValue::Px(rem * 16.0)
            } else {
                ResolvedMarginValue::Px(0.0)
            }
        }
        MarginValue::Var(var) => ResolvedMarginValue::Px(
            custom_properties
                .iter()
                .find(|(name, _)| *name == var.as_str())
                .map(|(_, value)| *value)
                .unwrap_or(0.0),
        ),
        MarginValue::Auto => ResolvedMarginValue::Auto,
    }
}

pub(super) fn to_style_margin(
    margin: crate::utilities::Margin,
    custom_properties: &[(&str, f32)],
) -> ResolvedMargin {
    ResolvedMargin {
        top: margin
            .top
            .map(|value| resolve_margin_value_px(value, custom_properties)),
        right: margin
            .right
            .map(|value| resolve_margin_value_px(value, custom_properties)),
        bottom: margin
            .bottom
            .map(|value| resolve_margin_value_px(value, custom_properties)),
        left: margin
            .left
            .map(|value| resolve_margin_value_px(value, custom_properties)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ResolvedWidth {
    Length(Length),
    Ratio(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ResolvedHeight {
    Length(Length),
    Ratio(f32),
}

fn percentage_to_ratio(value: Percentage) -> Option<f32> {
    match value {
        Percentage::S0 => Some(0.0),
        Percentage::S1_2 | Percentage::S2_4 | Percentage::S3_6 => Some(0.5),
        Percentage::S1_3 | Percentage::S2_6 => Some(1.0 / 3.0),
        Percentage::S2_3 | Percentage::S4_6 => Some(2.0 / 3.0),
        Percentage::S1_4 => Some(0.25),
        Percentage::S3_4 => Some(0.75),
        Percentage::S1_5 => Some(0.2),
        Percentage::S2_5 => Some(0.4),
        Percentage::S3_5 => Some(0.6),
        Percentage::S4_5 => Some(0.8),
        Percentage::S1_6 => Some(1.0 / 6.0),
        Percentage::S5_6 => Some(5.0 / 6.0),
        Percentage::Full => Some(1.0),
        Percentage::Min | Percentage::Max | Percentage::Fit => None,
    }
}

pub(crate) fn resolve_width(
    width: crate::utilities::Width,
    custom_properties: &[(&str, f32)],
) -> Option<ResolvedWidth> {
    match width.size()? {
        crate::utilities::Size::Spacing(spacing) => {
            Some(ResolvedWidth::Length(Length::Fixed(spacing_to_px(spacing))))
        }
        crate::utilities::Size::Percentage(fraction) => {
            if matches!(fraction, Percentage::Full) {
                Some(ResolvedWidth::Length(Length::Fill))
            } else if matches!(
                fraction,
                Percentage::Min | Percentage::Max | Percentage::Fit
            ) {
                Some(ResolvedWidth::Length(Length::Shrink))
            } else {
                percentage_to_ratio(fraction)
                    .map(|ratio| ResolvedWidth::Ratio(ratio.clamp(0.0, 1.0)))
            }
        }
        crate::utilities::Size::Container(container) => Some(ResolvedWidth::Length(Length::Fixed(
            container_to_px(container),
        ))),
        crate::utilities::Size::Auto => Some(ResolvedWidth::Length(Length::Shrink)),
        crate::utilities::Size::Full
        | crate::utilities::Size::ScreenWidth
        | crate::utilities::Size::ScreenHeight
        | crate::utilities::Size::Dvw
        | crate::utilities::Size::Dvh
        | crate::utilities::Size::Lvw
        | crate::utilities::Size::Lvh
        | crate::utilities::Size::Svw
        | crate::utilities::Size::Svh => Some(ResolvedWidth::Length(Length::Fill)),
        crate::utilities::Size::Prose => Some(ResolvedWidth::Length(Length::Fixed(520.0))),
        crate::utilities::Size::MinContent
        | crate::utilities::Size::MaxContent
        | crate::utilities::Size::Fit
        | crate::utilities::Size::Lh => Some(ResolvedWidth::Length(Length::Shrink)),
        crate::utilities::Size::Var(var) => custom_properties
            .iter()
            .find(|(name, _)| *name == var.as_str())
            .map(|(_, px)| ResolvedWidth::Length(Length::Fixed(px.max(0.0)))),
        crate::utilities::Size::HeightVar(var) => custom_properties
            .iter()
            .find(|(name, _)| *name == var.as_str())
            .map(|(_, px)| ResolvedWidth::Length(Length::Fixed(px.max(0.0)))),
        crate::utilities::Size::Px(px) => Some(ResolvedWidth::Length(Length::Fixed(f32::from(px)))),
    }
}

pub(crate) fn resolve_height(
    height: crate::utilities::Height,
    custom_properties: &[(&str, f32)],
) -> Option<ResolvedHeight> {
    match height.size()? {
        crate::utilities::Size::Spacing(spacing) => Some(ResolvedHeight::Length(Length::Fixed(
            spacing_to_px(spacing),
        ))),
        crate::utilities::Size::Percentage(fraction) => {
            if matches!(fraction, Percentage::Full) {
                Some(ResolvedHeight::Length(Length::Fill))
            } else if matches!(
                fraction,
                Percentage::Min | Percentage::Max | Percentage::Fit
            ) {
                Some(ResolvedHeight::Length(Length::Shrink))
            } else {
                percentage_to_ratio(fraction)
                    .map(|ratio| ResolvedHeight::Ratio(ratio.clamp(0.0, 1.0)))
            }
        }
        crate::utilities::Size::Container(container) => Some(ResolvedHeight::Length(
            Length::Fixed(container_to_px(container)),
        )),
        crate::utilities::Size::Auto => Some(ResolvedHeight::Length(Length::Shrink)),
        crate::utilities::Size::Full
        | crate::utilities::Size::ScreenWidth
        | crate::utilities::Size::ScreenHeight
        | crate::utilities::Size::Dvw
        | crate::utilities::Size::Dvh
        | crate::utilities::Size::Lvw
        | crate::utilities::Size::Lvh
        | crate::utilities::Size::Svw
        | crate::utilities::Size::Svh => Some(ResolvedHeight::Length(Length::Fill)),
        crate::utilities::Size::Prose => Some(ResolvedHeight::Length(Length::Fixed(520.0))),
        crate::utilities::Size::MinContent
        | crate::utilities::Size::MaxContent
        | crate::utilities::Size::Fit
        | crate::utilities::Size::Lh => Some(ResolvedHeight::Length(Length::Shrink)),
        crate::utilities::Size::Var(var) => custom_properties
            .iter()
            .find(|(name, _)| *name == var.as_str())
            .map(|(_, px)| ResolvedHeight::Length(Length::Fixed(px.max(0.0)))),
        crate::utilities::Size::HeightVar(var) => custom_properties
            .iter()
            .find(|(name, _)| *name == var.as_str())
            .map(|(_, px)| ResolvedHeight::Length(Length::Fixed(px.max(0.0)))),
        crate::utilities::Size::Px(px) => {
            Some(ResolvedHeight::Length(Length::Fixed(f32::from(px))))
        }
    }
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
        Blur::Xs => 4.0,
        Blur::Sm => 8.0,
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

/// Convert twill ObjectFit to iced ContentFit
pub fn to_content_fit(fit: ObjectFit) -> ContentFit {
    match fit {
        ObjectFit::Contain => ContentFit::Contain,
        ObjectFit::Cover => ContentFit::Cover,
        ObjectFit::Fill => ContentFit::Fill,
        ObjectFit::ScaleDown => ContentFit::ScaleDown,
        ObjectFit::None => ContentFit::None,
    }
}

/// Convert twill Shadow to iced Shadow with an optional color override.
pub fn to_shadow_with_color(shadow: Shadow, color: ShadowColor) -> iced::Shadow {
    to_shadow_layers_with_color(shadow, color)
        .into_iter()
        .next()
        .unwrap_or_default()
}

/// Convert twill Shadow to one or more iced shadows.
///
/// Tailwind defines multiple box-shadow layers for some tokens (`sm`, `md`, `lg`, `xl`).
/// Iced supports only one shadow per container, so layered fidelity is achieved
/// by wrapping content with multiple shadow containers.
pub fn to_shadow_layers_with_color(shadow: Shadow, color: ShadowColor) -> Vec<iced::Shadow> {
    let layers: &[(f32, f32, f32)] = match shadow {
        Shadow::None => &[],
        Shadow::Xs2 => &[(1.0, 0.0, 0.05)],
        Shadow::Xs => &[(1.0, 2.0, 0.05)],
        Shadow::Sm => &[(1.0, 3.0, 0.1), (1.0, 2.0, 0.1)],
        Shadow::Md => &[(4.0, 6.0, 0.1), (2.0, 4.0, 0.1)],
        Shadow::Lg => &[(10.0, 15.0, 0.1), (4.0, 6.0, 0.1)],
        Shadow::Xl => &[(20.0, 25.0, 0.1), (8.0, 10.0, 0.1)],
        Shadow::S2xl => &[(25.0, 50.0, 0.25)],
    };

    let base = match color {
        ShadowColor::Default => Color::black(),
        ShadowColor::Explicit(color) => color,
    };
    layers
        .iter()
        .map(|(offset_y, blur, alpha)| {
            let mut c = to_color(base);
            c.a *= *alpha;
            iced::Shadow {
                color: c,
                offset: iced::Vector::new(0.0, *offset_y),
                blur_radius: *blur,
            }
        })
        .collect()
}

pub(super) fn shadow_layers_with_opacity(
    shadow: Shadow,
    color: ShadowColor,
    opacity: f32,
) -> Vec<iced::Shadow> {
    to_shadow_layers_with_color(shadow, color)
        .into_iter()
        .map(|mut layer| {
            layer.color = apply_opacity_to_color(layer.color, opacity);
            layer
        })
        .collect()
}

pub(super) fn wrap_with_shadow_layers<'a, Message: 'a>(
    content: iced::Element<'a, Message>,
    layers: &[iced::Shadow],
    border_radius: f32,
) -> iced::widget::Container<'a, Message> {
    let mut current = content;

    for shadow in layers.iter().copied().rev() {
        current = iced::widget::container(current)
            .style(move |_| iced::widget::container::Style {
                border: iced::Border {
                    radius: border_radius.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow,
                ..Default::default()
            })
            .into();
    }

    iced::widget::container(current)
}

/// Convert twill Shadow to iced Shadow.
pub fn to_shadow(shadow: Shadow, color: Color) -> iced::Shadow {
    to_shadow_with_color(shadow, ShadowColor::Explicit(color))
}

/// Convert twill FontSize to f32 for iced
pub fn to_font_size(size: FontSize) -> f32 {
    size.resolve_px(&[]).unwrap_or(16.0)
}

/// Resolve twill FontSize to pixels for iced with optional custom properties.
pub fn resolve_font_size(size: FontSize, custom_properties: &[(&str, f32)]) -> Option<f32> {
    size.resolve_px(custom_properties)
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

/// Convert twill TextAlign to iced text alignment using LTR logical fallback.
///
/// For direction-aware logical alignment (`text-start` / `text-end`),
/// use [`to_text_alignment_with_direction`].
pub fn to_text_alignment(align: TextAlign) -> iced::widget::text::Alignment {
    to_text_alignment_with_direction(align, TextDirection::LeftToRight)
}

/// Convert twill TextAlign to iced text alignment with explicit text direction.
///
/// - [`TextDirection::LeftToRight`] for left-to-right content
/// - [`TextDirection::RightToLeft`] for right-to-left content
pub fn to_text_alignment_with_direction(
    align: TextAlign,
    direction: TextDirection,
) -> iced::widget::text::Alignment {
    match align {
        TextAlign::Left => iced::widget::text::Alignment::Left,
        TextAlign::Center => iced::widget::text::Alignment::Center,
        TextAlign::Right => iced::widget::text::Alignment::Right,
        TextAlign::Justify => iced::widget::text::Alignment::Justified,
        TextAlign::Start => {
            if matches!(direction, TextDirection::RightToLeft) {
                iced::widget::text::Alignment::Right
            } else {
                iced::widget::text::Alignment::Left
            }
        }
        TextAlign::End => {
            if matches!(direction, TextDirection::RightToLeft) {
                iced::widget::text::Alignment::Left
            } else {
                iced::widget::text::Alignment::Right
            }
        }
    }
}

/// Convert twill SemanticColor to iced Color based on the theme variant
pub fn to_semantic_color(semantic: SemanticColor, variant: ThemeVariant) -> iced::Color {
    let color = SemanticThemeVars::shadcn_neutral()
        .resolve_value(semantic, variant)
        .unwrap_or_else(|| Color::black().compute());
    to_color_value(color)
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
        Cursor::NResize | Cursor::SResize | Cursor::NsResize => {
            iced::mouse::Interaction::ResizingVertically
        }
        Cursor::EResize | Cursor::WResize | Cursor::EwResize => {
            iced::mouse::Interaction::ResizingHorizontally
        }
        Cursor::NeResize | Cursor::SwResize | Cursor::NeswResize => {
            iced::mouse::Interaction::ResizingDiagonallyUp
        }
        Cursor::NwResize | Cursor::SeResize | Cursor::NwseResize => {
            iced::mouse::Interaction::ResizingDiagonallyDown
        }
        Cursor::ZoomIn => iced::mouse::Interaction::ZoomIn,
        Cursor::ZoomOut => iced::mouse::Interaction::ZoomOut,
    }
}
