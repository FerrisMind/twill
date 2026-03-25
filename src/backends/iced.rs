//! Iced backend for twill.
//!
//! Converts twill styles to iced types.

use crate::style::Style;
use crate::tokens::{
    AspectRatio, BackgroundColor, Blur, BorderRadius, BorderStyle, Color, ColorValue, Container,
    Cursor, FontSize, FontWeight, Percentage, Scale, SemanticColor, SemanticThemeVars, Shadow,
    Spacing, TextAlign, TransitionDuration,
};
use crate::traits::ComputeValue;
use crate::utilities::{
    AlignItems, Columns, Flex, FlexDirection, GridTemplate, JustifyContent, MarginValue, ObjectFit,
    PaddingValue,
};
use iced::advanced::layout::{Layout as AdvancedLayout, Limits, Node};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Shell, Widget as AdvancedWidget};
use iced::widget::{canvas, stack};
use iced::{ContentFit, Length, Point, Rectangle, Renderer, Size, Theme, Vector, border, mouse};

/// Canonical iced conversion trait for typed twill values.
pub trait ToIced {
    type Output;

    fn to_iced(self) -> Self::Output;
}

/// Direct iced rendering helpers for high-level twill components.
pub trait IcedButtonExt {
    fn render_iced<'a, Message: Clone + 'a>(
        &self,
        label: &'a str,
        on_press: Message,
    ) -> iced::Element<'a, Message>;
}

impl ToIced for Color {
    type Output = iced::Color;

    fn to_iced(self) -> Self::Output {
        to_color(self)
    }
}

impl ToIced for ColorValue {
    type Output = iced::Color;

    fn to_iced(self) -> Self::Output {
        to_color_value(self)
    }
}

impl ToIced for Spacing {
    type Output = iced::Padding;

    fn to_iced(self) -> Self::Output {
        to_padding(self)
    }
}

impl ToIced for BorderRadius {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_border_radius(self)
    }
}

impl ToIced for Blur {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_blur_radius(self)
    }
}

impl ToIced for AspectRatio {
    type Output = Option<f32>;

    fn to_iced(self) -> Self::Output {
        to_aspect_ratio(self)
    }
}

impl ToIced for ObjectFit {
    type Output = ContentFit;

    fn to_iced(self) -> Self::Output {
        to_content_fit(self)
    }
}

impl ToIced for Shadow {
    type Output = iced::Shadow;

    fn to_iced(self) -> Self::Output {
        to_shadow_with_color(self, None)
    }
}

impl ToIced for FontSize {
    type Output = f32;

    fn to_iced(self) -> Self::Output {
        to_font_size(self)
    }
}

impl ToIced for FontWeight {
    type Output = iced::font::Weight;

    fn to_iced(self) -> Self::Output {
        to_font_weight(self)
    }
}

impl ToIced for TextAlign {
    type Output = iced::widget::text::Alignment;

    fn to_iced(self) -> Self::Output {
        to_text_alignment(self)
    }
}

impl ToIced for TransitionDuration {
    type Output = std::time::Duration;

    fn to_iced(self) -> Self::Output {
        to_duration(self)
    }
}

impl ToIced for Cursor {
    type Output = iced::mouse::Interaction;

    fn to_iced(self) -> Self::Output {
        to_interaction(self)
    }
}

impl IcedButtonExt for crate::components::Button {
    fn render_iced<'a, Message: Clone + 'a>(
        &self,
        label: &'a str,
        on_press: Message,
    ) -> iced::Element<'a, Message> {
        twill_button(self, label, on_press)
    }
}

fn apply_opacity_to_color(mut color: iced::Color, opacity: f32) -> iced::Color {
    if opacity.is_finite() {
        color.a *= opacity.clamp(0.0, 1.0);
    }
    color
}

fn apply_opacity_to_color_value(mut color: ColorValue, opacity: f32) -> ColorValue {
    if opacity.is_finite() {
        color.a *= opacity.clamp(0.0, 1.0);
    }
    color
}

fn resolved_opacity(style: &Style) -> f32 {
    style.opacity.unwrap_or(1.0).clamp(0.0, 1.0)
}

fn spacing_to_px(spacing: Spacing) -> f32 {
    match spacing.to_px() {
        Some(px) => px as f32,
        None => 0.0,
    }
}

fn container_to_px(container: Container) -> f32 {
    match container {
        Container::S3xs => 256.0,
        Container::S2xs => 288.0,
        Container::Xs => 320.0,
        Container::Sm => 384.0,
        Container::Md => 448.0,
        Container::Lg => 512.0,
        Container::Xl => 576.0,
        Container::S2xl => 672.0,
        Container::S3xl => 768.0,
        Container::S4xl => 896.0,
        Container::S5xl => 1024.0,
        Container::S6xl => 1152.0,
        Container::S7xl => 1280.0,
    }
}

fn sanitize_gap(gap: f32) -> f32 {
    if gap.is_finite() { gap.max(0.0) } else { 0.0 }
}

const DEFAULT_MAX_COLUMNS: usize = 16;
const ABSOLUTE_MAX_COLUMNS: usize = 64;

fn normalize_max_columns(max_columns: usize) -> usize {
    max_columns.clamp(1, ABSOLUTE_MAX_COLUMNS)
}

fn resolve_columns_count(
    columns: Columns,
    available_width: f32,
    gap: f32,
    max_columns: usize,
) -> usize {
    let safe_width = if available_width.is_finite() {
        available_width.clamp(0.0, 20_000.0)
    } else {
        0.0
    };
    let safe_gap = sanitize_gap(gap);

    let max_columns = normalize_max_columns(max_columns);

    match columns {
        Columns::Count(count) => usize::from(count.get()),
        Columns::Width(width) => {
            let target = container_to_px(width);
            if target <= 0.0 {
                1
            } else {
                ((safe_width + safe_gap) / (target + safe_gap))
                    .floor()
                    .max(1.0) as usize
            }
        }
        Columns::WidthPx(width) => {
            let target = f32::from(width).max(1.0);
            ((safe_width + safe_gap) / (target + safe_gap))
                .floor()
                .max(1.0) as usize
        }
        Columns::Auto => 1,
    }
    .clamp(1, max_columns)
}

fn resolve_column_width(available_width: f32, count: usize, gap: f32) -> f32 {
    let safe_width = if available_width.is_finite() {
        available_width.max(0.0)
    } else {
        0.0
    };
    let safe_gap = sanitize_gap(gap);
    let count = count.max(1) as f32;
    let total_gap = safe_gap * (count - 1.0);

    ((safe_width - total_gap) / count).max(0.0)
}

fn resolve_aspect_size(max_width: f32, max_height: f32, ratio: f32) -> Size {
    let safe_ratio = if ratio.is_finite() && ratio > 0.0 {
        ratio
    } else {
        1.0
    };
    let mut width = if max_width.is_finite() {
        max_width.max(0.0)
    } else {
        0.0
    };
    let mut height = width / safe_ratio;
    let safe_max_height = if max_height.is_finite() {
        max_height.max(0.0)
    } else {
        f32::INFINITY
    };

    if height > safe_max_height {
        height = safe_max_height;
        width = height * safe_ratio;
    }

    Size::new(width.max(0.0), height.max(0.0))
}

/// Convert twill Color to iced Color.
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

fn resolve_background_color_token(
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

fn to_style_padding(
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
enum ResolvedMarginValue {
    Px(f32),
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct ResolvedMargin {
    top: Option<ResolvedMarginValue>,
    right: Option<ResolvedMarginValue>,
    bottom: Option<ResolvedMarginValue>,
    left: Option<ResolvedMarginValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct MarginOffsets {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl MarginOffsets {
    fn is_zero(self) -> bool {
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

fn to_style_margin(
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
enum ResolvedWidth {
    Length(Length),
    Ratio(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ResolvedHeight {
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

fn resolve_width(
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

fn resolve_height(
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
pub fn to_shadow_with_color(shadow: Shadow, color: Option<Color>) -> iced::Shadow {
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
pub fn to_shadow_layers_with_color(shadow: Shadow, color: Option<Color>) -> Vec<iced::Shadow> {
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

    let base = color.unwrap_or(Color::black());
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

fn shadow_layers_with_opacity(
    shadow: Shadow,
    color: Option<Color>,
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

fn wrap_with_shadow_layers<'a, Message: 'a>(
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
    to_shadow_with_color(shadow, Some(color))
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
    to_text_alignment_with_direction(align, false)
}

/// Convert twill TextAlign to iced text alignment with explicit text direction.
///
/// - `is_rtl = false` for left-to-right content
/// - `is_rtl = true` for right-to-left content
pub fn to_text_alignment_with_direction(
    align: TextAlign,
    is_rtl: bool,
) -> iced::widget::text::Alignment {
    match align {
        TextAlign::Left => iced::widget::text::Alignment::Left,
        TextAlign::Center => iced::widget::text::Alignment::Center,
        TextAlign::Right => iced::widget::text::Alignment::Right,
        TextAlign::Justify => iced::widget::text::Alignment::Justified,
        TextAlign::Start => {
            if is_rtl {
                iced::widget::text::Alignment::Right
            } else {
                iced::widget::text::Alignment::Left
            }
        }
        TextAlign::End => {
            if is_rtl {
                iced::widget::text::Alignment::Left
            } else {
                iced::widget::text::Alignment::Right
            }
        }
    }
}

/// Convert twill SemanticColor to iced Color based on the theme variant
pub fn to_semantic_color(semantic: SemanticColor, is_dark: bool) -> iced::Color {
    let color = SemanticThemeVars::shadcn_neutral()
        .resolve_value(semantic, is_dark)
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

/// Create a styled button using twill colors.
pub fn styled_button<'a, Message: Clone + 'a>(
    label: &'a str,
    bg_color: Color,
    text_color: Color,
    on_press: Message,
) -> iced::Element<'a, Message> {
    let base_bg = bg_color.compute();
    iced::widget::button(iced::widget::text(label).color(to_color(text_color)))
        .style(
            move |_theme: &iced::Theme, status: iced::widget::button::Status| {
                let bg_value = match status {
                    iced::widget::button::Status::Hovered => base_bg.darken_oklch(0.05),
                    iced::widget::button::Status::Pressed => base_bg.darken_oklch(0.10),
                    _ => base_bg,
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(to_color_value(bg_value))),
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
    styled_container_with_custom_properties(content, style, &[])
}

/// Create a styled container with twill Style and explicit custom-property values.
pub fn styled_container_with_custom_properties<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
    custom_properties: &[(&str, f32)],
) -> iced::widget::Container<'a, Message> {
    let opacity = resolved_opacity(style);
    let padding = style
        .padding
        .map(|padding| to_style_padding(padding, custom_properties));

    let bg_color = style
        .background_color
        .and_then(|bg| resolve_background_color_token(bg, style.text_color))
        .map(|bg| apply_opacity_to_color_value(bg, opacity))
        .map(to_color_value);
    let base_border_width: f32 = style.border_width.map_or(0.0, |w| match w {
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
        .map(|color| apply_opacity_to_color(color, opacity))
        .unwrap_or(iced::Color::TRANSPARENT);
    let border_style = style.border_style.unwrap_or(BorderStyle::Solid);
    let border_width = base_border_width;
    let shadow_layers = style
        .box_shadow
        .map(|s| shadow_layers_with_opacity(s, style.shadow_color, opacity))
        .unwrap_or_default();

    match border_style {
        BorderStyle::Solid => {
            let mut container = iced::widget::container(content);
            if let Some(p) = padding {
                container = container.padding(p);
            }
            let base = container.style(move |_| iced::widget::container::Style {
                background: bg_color.map(iced::Background::Color),
                border: iced::Border {
                    radius: border_radius.into(),
                    width: border_width,
                    color: border_color,
                },
                ..Default::default()
            });

            wrap_with_shadow_layers(base.into(), &shadow_layers, border_radius)
        }
        _ => {
            let mut content_layer = iced::widget::container(content);
            if let Some(p) = padding {
                content_layer = content_layer.padding(p);
            }

            let border_layer = canvas(BorderCanvas {
                border_style,
                border_width,
                border_radius,
                border_color,
                background: bg_color,
            })
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);

            let base = iced::widget::container(stack![border_layer, content_layer]);
            wrap_with_shadow_layers(base.into(), &shadow_layers, border_radius)
        }
    }
}

/// Layout child elements into CSS-like columns for iced.
pub fn columns_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    style: &Style,
) -> iced::Element<'a, Message> {
    let gap = style.column_gap.map_or(0.0, spacing_to_px);
    let max_columns = style
        .columns_max_count
        .map(|max| usize::from(max.get()))
        .unwrap_or(DEFAULT_MAX_COLUMNS);

    match style.columns {
        Some(columns) => ColumnsFlow::with_elements(items, columns)
            .gap(gap)
            .max_columns(max_columns)
            .width(Length::Fill)
            .into(),
        None => {
            let mut fallback = iced::widget::Column::new().spacing(gap);
            for item in items {
                fallback = fallback.push(item);
            }
            fallback.into()
        }
    }
}

const MAX_GRID_TEMPLATE_TRACKS: usize = 64;

fn clamp_track_count(count: usize) -> usize {
    count.clamp(1, MAX_GRID_TEMPLATE_TRACKS)
}

fn parse_repeat_track_count(value: &str) -> Option<usize> {
    let raw = value.trim();
    let repeat_start = raw.find("repeat(")?;
    let after_repeat = &raw[repeat_start + "repeat(".len()..];
    let comma_index = after_repeat.find(',')?;
    let count_part = after_repeat[..comma_index].trim();
    count_part.parse::<usize>().ok().map(clamp_track_count)
}

fn count_top_level_tracks(value: &str) -> Option<usize> {
    let mut depth = 0_i32;
    let mut in_token = false;
    let mut count = 0_usize;

    for ch in value.chars() {
        match ch {
            '(' | '[' | '{' => {
                depth += 1;
                in_token = true;
            }
            ')' | ']' | '}' => {
                depth = (depth - 1).max(0);
            }
            _ if ch.is_whitespace() && depth == 0 => {
                if in_token {
                    count += 1;
                    in_token = false;
                }
            }
            _ => {
                if !ch.is_whitespace() {
                    in_token = true;
                }
            }
        }
    }

    if in_token {
        count += 1;
    }

    if count == 0 {
        None
    } else {
        Some(clamp_track_count(count))
    }
}

fn track_count_from_template_value(value: &str) -> Option<usize> {
    let raw = value.trim();
    if raw.is_empty() {
        return None;
    }

    if raw.eq_ignore_ascii_case("none") {
        return Some(1);
    }

    if let Ok(count) = raw.parse::<usize>() {
        return Some(clamp_track_count(count));
    }

    parse_repeat_track_count(raw).or_else(|| count_top_level_tracks(raw))
}

fn lookup_custom_property_value<'a>(name: &str, vars: &'a [(&'a str, &'a str)]) -> Option<&'a str> {
    vars.iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| *value)
}

fn resolve_grid_template_track_count(
    template: &GridTemplate,
    inherited_track_count: Option<usize>,
    custom_properties: &[(&str, &str)],
) -> usize {
    let resolved = match template {
        GridTemplate::Count(count) => Some(count.get() as usize),
        GridTemplate::None => Some(1),
        GridTemplate::Subgrid => inherited_track_count,
        GridTemplate::CustomProperty(name) => lookup_custom_property_value(name, custom_properties)
            .and_then(track_count_from_template_value)
            .or(inherited_track_count),
        GridTemplate::Arbitrary(value) => {
            if value.trim().eq_ignore_ascii_case("subgrid") {
                inherited_track_count
            } else {
                track_count_from_template_value(value)
            }
        }
    };

    clamp_track_count(resolved.unwrap_or(1))
}

fn build_grid_template_columns_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    track_count: usize,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    let track_count = clamp_track_count(track_count);
    let gap_px = spacing_to_px(gap);

    if track_count <= 1 {
        let mut col = iced::widget::Column::new()
            .spacing(gap_px)
            .width(Length::Fill);
        for item in items {
            col = col.push(item);
        }
        return col.into();
    }

    let mut grid_rows = iced::widget::Column::new()
        .spacing(gap_px)
        .width(Length::Fill);
    let mut current_row = iced::widget::Row::new().spacing(gap_px).width(Length::Fill);
    let mut items_in_row = 0_usize;

    for item in items {
        current_row = current_row.push(iced::widget::container(item).width(Length::FillPortion(1)));
        items_in_row += 1;

        if items_in_row == track_count {
            grid_rows = grid_rows.push(current_row);
            current_row = iced::widget::Row::new().spacing(gap_px).width(Length::Fill);
            items_in_row = 0;
        }
    }

    if items_in_row > 0 {
        for _ in items_in_row..track_count {
            current_row = current_row.push(
                iced::widget::Space::new()
                    .width(Length::FillPortion(1))
                    .height(Length::Shrink),
            );
        }
        grid_rows = grid_rows.push(current_row);
    }

    grid_rows.into()
}

/// Create an iced layout for a typed `grid-template-columns` value.
pub fn grid_template_columns_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    template: GridTemplate,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    grid_template_columns_layout_with_context(items, template, gap, None, &[])
}

/// Create an iced layout for typed `grid-template-columns` with optional context.
///
/// - `inherited_track_count` is used for `Subgrid`.
/// - `custom_properties` is used for `CustomProperty`.
pub fn grid_template_columns_layout_with_context<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    template: GridTemplate,
    gap: Spacing,
    inherited_track_count: Option<usize>,
    custom_properties: &[(&str, &str)],
) -> iced::Element<'a, Message> {
    let track_count =
        resolve_grid_template_track_count(&template, inherited_track_count, custom_properties);
    build_grid_template_columns_layout(items, track_count, gap)
}

struct ColumnsFlow<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    elements: Vec<iced::Element<'a, Message, Theme, Renderer>>,
    columns: Columns,
    max_columns: usize,
    gap: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> ColumnsFlow<'a, Message, Theme, Renderer> {
    fn with_elements(
        elements: Vec<iced::Element<'a, Message, Theme, Renderer>>,
        columns: Columns,
    ) -> Self {
        Self {
            elements,
            columns,
            max_columns: DEFAULT_MAX_COLUMNS,
            gap: 0.0,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn gap(mut self, gap: f32) -> Self {
        self.gap = sanitize_gap(gap);
        self
    }

    fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    fn max_columns(mut self, max_columns: usize) -> Self {
        self.max_columns = normalize_max_columns(max_columns);
        self
    }
}

impl<Message, Theme, Renderer> AdvancedWidget<Message, Theme, Renderer>
    for ColumnsFlow<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        self.elements.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        let available_width = limits.max().width.max(0.0);
        let gap = sanitize_gap(self.gap);
        let count = resolve_columns_count(self.columns, available_width, gap, self.max_columns);
        let column_width = resolve_column_width(available_width, count, gap);

        let max_height = limits.max().height.max(0.0);
        let child_limits = Limits::new(
            Size::new(column_width, 0.0),
            Size::new(column_width, max_height),
        );

        let mut nodes = Vec::with_capacity(self.elements.len());
        let mut column_heights = [0.0_f32; ABSOLUTE_MAX_COLUMNS];
        let heights = &mut column_heights[..count];
        let mut children = tree.children.iter_mut();

        if count == 1 {
            let mut y = 0.0_f32;
            for element in &mut self.elements {
                let Some(child) = children.next() else {
                    break;
                };
                let mut node = element
                    .as_widget_mut()
                    .layout(child, renderer, &child_limits);
                node.move_to_mut(Point::new(0.0, y));
                y += node.size().height + gap;
                nodes.push(node);
            }
            heights[0] = y;
        } else {
            for element in &mut self.elements {
                let Some(child) = children.next() else {
                    break;
                };
                let mut node = element
                    .as_widget_mut()
                    .layout(child, renderer, &child_limits);

                let (column_index, column_height) = heights
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, height)| (index, *height))
                    .unwrap_or((0, 0.0));

                let x = column_index as f32 * (column_width + gap);
                node.move_to_mut(Point::new(x, column_height));

                let next_height = column_height + node.size().height + gap;
                heights[column_index] = next_height;
                nodes.push(node);
            }
        }

        let content_height = if nodes.is_empty() {
            0.0
        } else {
            (heights.iter().copied().fold(0.0_f32, f32::max) - gap).max(0.0)
        };
        let content_width = (count as f32 * column_width) + (count.saturating_sub(1) as f32 * gap);
        let size = limits.resolve(
            self.width,
            self.height,
            Size::new(content_width, content_height),
        );

        Node::with_children(size, nodes)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .elements
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((state, layout), child) in tree
            .children
            .iter_mut()
            .zip(layout.children())
            .zip(self.elements.iter_mut())
        {
            child.as_widget_mut().update(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: AdvancedLayout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        for ((state, layout), child) in tree
            .children
            .iter_mut()
            .zip(layout.children())
            .zip(self.elements.iter_mut())
        {
            operation.container(None, layout.bounds());
            operation.traverse(&mut |operation| {
                child
                    .as_widget_mut()
                    .operate(state, layout, renderer, operation);
            });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.elements
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: AdvancedLayout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.elements,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<ColumnsFlow<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    fn from(flow: ColumnsFlow<'a, Message, Theme, Renderer>) -> Self {
        iced::Element::new(flow)
    }
}

struct AspectRatioBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> AspectRatioBox<'a, Message, Theme, Renderer> {
    fn new(child: iced::Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
        Self {
            child,
            ratio: ratio.max(0.0001),
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<Message, Theme, Renderer> AdvancedWidget<Message, Theme, Renderer>
    for AspectRatioBox<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child));
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        let max = limits.max();
        let aspect = resolve_aspect_size(max.width, max.height, self.ratio);
        let width = aspect.width;
        let height = aspect.height;

        let child_limits = Limits::new(Size::new(width, height), Size::new(width, height))
            .width(Length::Fixed(width))
            .height(Length::Fixed(height));

        let Some(child_state) = tree.children.first_mut() else {
            return Node::new(limits.resolve(self.width, self.height, Size::new(width, height)));
        };
        let child_node = self
            .child
            .as_widget_mut()
            .layout(child_state, renderer, &child_limits);

        let size = limits.resolve(self.width, self.height, Size::new(width, height));
        Node::with_children(size, vec![child_node])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let (Some(child_state), Some(child_layout)) =
            (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().draw(
                child_state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            self.child.as_widget_mut().update(
                state,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: AdvancedLayout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            operation.container(None, child_layout.bounds());
            operation.traverse(&mut |operation| {
                self.child
                    .as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
            });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if let (Some(state), Some(child_layout)) = (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().mouse_interaction(
                state,
                child_layout,
                cursor,
                viewport,
                renderer,
            )
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: AdvancedLayout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        else {
            return None;
        };

        self.child
            .as_widget_mut()
            .overlay(state, child_layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<AspectRatioBox<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    fn from(boxed: AspectRatioBox<'a, Message, Theme, Renderer>) -> Self {
        iced::Element::new(boxed)
    }
}

struct WidthRatioBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> WidthRatioBox<'a, Message, Theme, Renderer> {
    fn new(child: iced::Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
        Self {
            child,
            ratio: ratio.clamp(0.0, 1.0),
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
}

impl<Message, Theme, Renderer> AdvancedWidget<Message, Theme, Renderer>
    for WidthRatioBox<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child));
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        let max = limits.max();

        let Some(child_state) = tree.children.first_mut() else {
            return Node::new(limits.resolve(self.width, self.height, Size::ZERO));
        };

        if !max.width.is_finite() {
            let child_node = self
                .child
                .as_widget_mut()
                .layout(child_state, renderer, &limits);
            let size = limits.resolve(self.width, self.height, child_node.size());
            return Node::with_children(size, vec![child_node]);
        }

        let width = (max.width.max(0.0) * self.ratio).max(0.0);
        let height = if max.height.is_finite() {
            max.height.max(0.0)
        } else {
            f32::INFINITY
        };

        let child_limits = Limits::new(Size::new(width, 0.0), Size::new(width, height))
            .width(Length::Fixed(width));

        let child_node = self
            .child
            .as_widget_mut()
            .layout(child_state, renderer, &child_limits);
        let size = limits.resolve(self.width, self.height, child_node.size());

        Node::with_children(size, vec![child_node])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let (Some(child_state), Some(child_layout)) =
            (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().draw(
                child_state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            self.child.as_widget_mut().update(
                state,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: AdvancedLayout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            operation.container(None, child_layout.bounds());
            operation.traverse(&mut |operation| {
                self.child
                    .as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
            });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if let (Some(state), Some(child_layout)) = (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().mouse_interaction(
                state,
                child_layout,
                cursor,
                viewport,
                renderer,
            )
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: AdvancedLayout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        else {
            return None;
        };

        self.child
            .as_widget_mut()
            .overlay(state, child_layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<WidthRatioBox<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    fn from(boxed: WidthRatioBox<'a, Message, Theme, Renderer>) -> Self {
        iced::Element::new(boxed)
    }
}

struct HeightRatioBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> HeightRatioBox<'a, Message, Theme, Renderer> {
    fn new(child: iced::Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
        Self {
            child,
            ratio: ratio.clamp(0.0, 1.0),
            width: Length::Fill,
            height: Length::Fill,
        }
    }
}

impl<Message, Theme, Renderer> AdvancedWidget<Message, Theme, Renderer>
    for HeightRatioBox<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child));
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        let max = limits.max();

        let Some(child_state) = tree.children.first_mut() else {
            return Node::new(limits.resolve(self.width, self.height, Size::ZERO));
        };

        if !max.height.is_finite() {
            let child_node = self
                .child
                .as_widget_mut()
                .layout(child_state, renderer, &limits);
            let size = limits.resolve(self.width, self.height, child_node.size());
            return Node::with_children(size, vec![child_node]);
        }

        let height = (max.height.max(0.0) * self.ratio).max(0.0);
        let width = if max.width.is_finite() {
            max.width.max(0.0)
        } else {
            f32::INFINITY
        };

        let child_limits = Limits::new(Size::new(0.0, height), Size::new(width, height))
            .height(Length::Fixed(height));

        let child_node = self
            .child
            .as_widget_mut()
            .layout(child_state, renderer, &child_limits);
        let size = limits.resolve(self.width, self.height, child_node.size());

        Node::with_children(size, vec![child_node])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let (Some(child_state), Some(child_layout)) =
            (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().draw(
                child_state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            self.child.as_widget_mut().update(
                state,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: AdvancedLayout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            operation.container(None, child_layout.bounds());
            operation.traverse(&mut |operation| {
                self.child
                    .as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
            });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if let (Some(state), Some(child_layout)) = (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().mouse_interaction(
                state,
                child_layout,
                cursor,
                viewport,
                renderer,
            )
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: AdvancedLayout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        else {
            return None;
        };

        self.child
            .as_widget_mut()
            .overlay(state, child_layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<HeightRatioBox<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    fn from(boxed: HeightRatioBox<'a, Message, Theme, Renderer>) -> Self {
        iced::Element::new(boxed)
    }
}

struct MarginBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    margin: MarginOffsets,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> MarginBox<'a, Message, Theme, Renderer> {
    fn new(child: iced::Element<'a, Message, Theme, Renderer>, margin: MarginOffsets) -> Self {
        Self {
            child,
            margin,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
}

impl<Message, Theme, Renderer> AdvancedWidget<Message, Theme, Renderer>
    for MarginBox<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.child)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.child));
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        let shrink_width = self.margin.left.max(0.0) + self.margin.right.max(0.0);
        let shrink_height = self.margin.top.max(0.0) + self.margin.bottom.max(0.0);
        let child_limits = limits.shrink(Size::new(shrink_width, shrink_height));

        let Some(child_state) = tree.children.first_mut() else {
            return Node::new(limits.resolve(self.width, self.height, Size::ZERO));
        };
        let mut child_node =
            self.child
                .as_widget_mut()
                .layout(child_state, renderer, &child_limits);
        child_node.move_to_mut(Point::new(self.margin.left, self.margin.top));

        let child_size = child_node.size();
        let width = (child_size.width + self.margin.left + self.margin.right).max(0.0);
        let height = (child_size.height + self.margin.top + self.margin.bottom).max(0.0);
        let size = limits.resolve(self.width, self.height, Size::new(width, height));

        Node::with_children(size, vec![child_node])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let (Some(child_state), Some(child_layout)) =
            (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().draw(
                child_state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            self.child.as_widget_mut().update(
                state,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: AdvancedLayout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        {
            operation.container(None, child_layout.bounds());
            operation.traverse(&mut |operation| {
                self.child
                    .as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
            });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: AdvancedLayout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if let (Some(state), Some(child_layout)) = (tree.children.first(), layout.children().next())
        {
            self.child.as_widget().mouse_interaction(
                state,
                child_layout,
                cursor,
                viewport,
                renderer,
            )
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: AdvancedLayout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let (Some(state), Some(child_layout)) =
            (tree.children.first_mut(), layout.children().next())
        else {
            return None;
        };

        self.child
            .as_widget_mut()
            .overlay(state, child_layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<MarginBox<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    fn from(boxed: MarginBox<'a, Message, Theme, Renderer>) -> Self {
        iced::Element::new(boxed)
    }
}

fn apply_margin<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    margin: crate::utilities::Margin,
    custom_properties: &[(&str, f32)],
) -> iced::Element<'a, Message> {
    let resolved = to_style_margin(margin, custom_properties);
    let mut element = content;

    let mut horizontal_auto = None;
    let mut vertical_auto = None;

    let offsets = MarginOffsets {
        top: match resolved.top {
            Some(ResolvedMarginValue::Px(px)) => px,
            Some(ResolvedMarginValue::Auto) => {
                vertical_auto = Some(iced::alignment::Vertical::Bottom);
                0.0
            }
            None => 0.0,
        },
        right: match resolved.right {
            Some(ResolvedMarginValue::Px(px)) => px,
            Some(ResolvedMarginValue::Auto) => {
                horizontal_auto = Some(iced::alignment::Horizontal::Left);
                0.0
            }
            None => 0.0,
        },
        bottom: match resolved.bottom {
            Some(ResolvedMarginValue::Px(px)) => px,
            Some(ResolvedMarginValue::Auto) => {
                vertical_auto = Some(iced::alignment::Vertical::Top);
                0.0
            }
            None => 0.0,
        },
        left: match resolved.left {
            Some(ResolvedMarginValue::Px(px)) => px,
            Some(ResolvedMarginValue::Auto) => {
                horizontal_auto = Some(iced::alignment::Horizontal::Right);
                0.0
            }
            None => 0.0,
        },
    };

    if matches!(resolved.left, Some(ResolvedMarginValue::Auto))
        && matches!(resolved.right, Some(ResolvedMarginValue::Auto))
    {
        horizontal_auto = Some(iced::alignment::Horizontal::Center);
    }

    if matches!(resolved.top, Some(ResolvedMarginValue::Auto))
        && matches!(resolved.bottom, Some(ResolvedMarginValue::Auto))
    {
        vertical_auto = Some(iced::alignment::Vertical::Center);
    }

    if !offsets.is_zero() {
        element = MarginBox::new(element, offsets).into();
    }

    if horizontal_auto.is_some() || vertical_auto.is_some() {
        let mut wrapper = iced::widget::container(element);

        if let Some(horizontal) = horizontal_auto {
            wrapper = wrapper.width(Length::Fill).align_x(horizontal);
        }

        if let Some(vertical) = vertical_auto {
            wrapper = wrapper.height(Length::Fill).align_y(vertical);
        }

        element = wrapper.into();
    }

    element
}

/// Apply high-level layout wrappers like Display::Hidden, Overflow, MaxWidth
pub fn apply_layout<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
) -> iced::Element<'a, Message> {
    apply_layout_with_custom_properties(content, style, &[])
}

/// Apply high-level layout wrappers with explicit custom-property values.
pub fn apply_layout_with_custom_properties<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
    custom_properties: &[(&str, f32)],
) -> iced::Element<'a, Message> {
    if matches!(style.display, Some(crate::utilities::Display::Hidden)) {
        return iced::widget::Space::new().into();
    }

    let mut container = styled_container_with_custom_properties(content, style, custom_properties);
    let mut width_ratio = None;
    let mut height_ratio = None;

    if let Some(width) = style
        .width
        .and_then(|width| resolve_width(width, custom_properties))
    {
        match width {
            ResolvedWidth::Length(length) => {
                container = container.width(length);
            }
            ResolvedWidth::Ratio(ratio) => {
                width_ratio = Some(ratio);
                container = container.width(Length::Fill);
            }
        }
    }

    if let Some(height) = style
        .height
        .and_then(|height| resolve_height(height, custom_properties))
    {
        match height {
            ResolvedHeight::Length(length) => {
                container = container.height(length);
            }
            ResolvedHeight::Ratio(ratio) => {
                height_ratio = Some(ratio);
                container = container.height(Length::Fill);
            }
        }
    }

    if let Some(constraints) = style.constraints {
        match constraints.max_width {
            Some(crate::utilities::Size::Spacing(ref s)) => {
                if let Some(px) = s.to_px() {
                    container = container.max_width(px as f32);
                }
            }
            Some(crate::utilities::Size::Prose) => {
                // A rough heuristic for 65ch
                container = container.max_width(520.0);
            }
            _ => {}
        }

        match constraints.max_height {
            Some(crate::utilities::Size::Spacing(ref s)) => {
                if let Some(px) = s.to_px() {
                    container = container.max_height(px as f32);
                }
            }
            Some(crate::utilities::Size::Prose) => {
                container = container.max_height(520.0);
            }
            _ => {}
        }
    }

    if matches!(style.overflow, Some(crate::utilities::Overflow::Hidden)) {
        container = container.clip(true);
    }

    let mut element: iced::Element<'a, Message> = if matches!(
        style.overflow,
        Some(crate::utilities::Overflow::Auto) | Some(crate::utilities::Overflow::Scroll)
    ) {
        iced::widget::scrollable(container).into()
    } else {
        container.into()
    };

    if let Some(ratio) = width_ratio {
        element = WidthRatioBox::new(element, ratio).into();
    }

    if let Some(ratio) = height_ratio {
        element = HeightRatioBox::new(element, ratio).into();
    }

    if let Some(ratio) = style.aspect_ratio.and_then(to_aspect_ratio) {
        element = AspectRatioBox::new(element, ratio)
            .width(Length::Fill)
            .into();
    }

    if let Some(margin) = style.margin {
        element = apply_margin(element, margin, custom_properties);
    }

    element
}

fn is_reverse_direction(direction: FlexDirection) -> bool {
    matches!(
        direction,
        FlexDirection::RowReverse | FlexDirection::ColReverse
    )
}

fn row_alignment_for_items(items: AlignItems) -> iced::alignment::Vertical {
    match items {
        AlignItems::Start | AlignItems::Stretch => iced::alignment::Vertical::Top,
        AlignItems::End | AlignItems::EndSafe => iced::alignment::Vertical::Bottom,
        AlignItems::Center | AlignItems::CenterSafe => iced::alignment::Vertical::Center,
        AlignItems::Baseline | AlignItems::BaselineLast => iced::alignment::Vertical::Bottom,
    }
}

fn column_alignment_for_items(items: AlignItems) -> iced::alignment::Horizontal {
    match items {
        AlignItems::Start
        | AlignItems::Stretch
        | AlignItems::Baseline
        | AlignItems::BaselineLast => iced::alignment::Horizontal::Left,
        AlignItems::End | AlignItems::EndSafe => iced::alignment::Horizontal::Right,
        AlignItems::Center | AlignItems::CenterSafe => iced::alignment::Horizontal::Center,
    }
}

fn normalize_justify_content(justify: JustifyContent) -> JustifyContent {
    match justify {
        JustifyContent::EndSafe => JustifyContent::End,
        JustifyContent::CenterSafe => JustifyContent::Center,
        _ => justify,
    }
}

fn main_axis_spacer<'a, Message: Clone + 'a>(
    direction: FlexDirection,
    portion: u16,
) -> iced::Element<'a, Message> {
    let portion = portion.max(1);
    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => iced::widget::Space::new()
            .width(Length::FillPortion(portion))
            .height(Length::Shrink)
            .into(),
        FlexDirection::Col | FlexDirection::ColReverse => iced::widget::Space::new()
            .width(Length::Shrink)
            .height(Length::FillPortion(portion))
            .into(),
    }
}

fn distribute_items_for_justify<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    justify_content: Option<JustifyContent>,
) -> (Vec<iced::Element<'a, Message>>, bool) {
    let justify = justify_content
        .map(normalize_justify_content)
        .unwrap_or(JustifyContent::Start);
    let start_at_end = is_reverse_direction(direction);
    let item_count = items.len();

    let mut distributed = Vec::new();
    let mut needs_main_axis_fill = false;

    match justify {
        JustifyContent::Start | JustifyContent::Normal | JustifyContent::Baseline => {
            if start_at_end && item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
            distributed.extend(items);
        }
        JustifyContent::End => {
            if !start_at_end && item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
            distributed.extend(items);
        }
        JustifyContent::Center => {
            if item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
            distributed.extend(items);
            if item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
        }
        JustifyContent::Between => {
            if item_count <= 1 {
                if start_at_end && item_count > 0 {
                    distributed.push(main_axis_spacer(direction, 1));
                    needs_main_axis_fill = true;
                }
                distributed.extend(items);
            } else {
                for (index, item) in items.into_iter().enumerate() {
                    if index > 0 {
                        distributed.push(main_axis_spacer(direction, 1));
                        needs_main_axis_fill = true;
                    }
                    distributed.push(item);
                }
            }
        }
        JustifyContent::Around => {
            if item_count == 0 {
                return (distributed, false);
            }
            distributed.push(main_axis_spacer(direction, 1));
            needs_main_axis_fill = true;

            for (index, item) in items.into_iter().enumerate() {
                distributed.push(item);
                if index + 1 < item_count {
                    distributed.push(main_axis_spacer(direction, 2));
                }
            }

            distributed.push(main_axis_spacer(direction, 1));
        }
        JustifyContent::Evenly => {
            if item_count == 0 {
                return (distributed, false);
            }
            distributed.push(main_axis_spacer(direction, 1));
            needs_main_axis_fill = true;

            for (index, item) in items.into_iter().enumerate() {
                distributed.push(item);
                if index + 1 < item_count {
                    distributed.push(main_axis_spacer(direction, 1));
                }
            }

            distributed.push(main_axis_spacer(direction, 1));
        }
        JustifyContent::Stretch => {
            distributed.extend(items);
            needs_main_axis_fill = true;
        }
        JustifyContent::EndSafe | JustifyContent::CenterSafe => {
            unreachable!("safe variants are normalized before distribution")
        }
    }

    (distributed, needs_main_axis_fill)
}

fn flex_layout<'a, Message: Clone + 'a>(
    mut items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
    align_items: Option<AlignItems>,
    justify_content: Option<JustifyContent>,
) -> iced::Element<'a, Message> {
    if is_reverse_direction(direction) {
        items.reverse();
    }

    if matches!(align_items, Some(AlignItems::Stretch)) {
        items = items
            .into_iter()
            .map(|item| match direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    iced::widget::container(item).height(Length::Fill).into()
                }
                FlexDirection::Col | FlexDirection::ColReverse => {
                    iced::widget::container(item).width(Length::Fill).into()
                }
            })
            .collect();
    }

    let justify_content = justify_content.map(normalize_justify_content);
    if matches!(justify_content, Some(JustifyContent::Stretch)) {
        items = items
            .into_iter()
            .map(|item| match direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    iced::widget::container(item).width(Length::Fill).into()
                }
                FlexDirection::Col | FlexDirection::ColReverse => {
                    iced::widget::container(item).height(Length::Fill).into()
                }
            })
            .collect();
    }

    let (items, needs_main_axis_fill) =
        distribute_items_for_justify(items, direction, justify_content);

    let gap = spacing_to_px(gap);

    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            let mut row = iced::widget::Row::new().spacing(gap).width(Length::Fill);
            if let Some(items) = align_items {
                row = row
                    .align_y(row_alignment_for_items(items))
                    .height(Length::Fill);
            }
            for item in items {
                row = row.push(item);
            }
            row.into()
        }
        FlexDirection::Col | FlexDirection::ColReverse => {
            let mut col = iced::widget::Column::new().spacing(gap).width(Length::Fill);
            if let Some(items) = align_items {
                col = col.align_x(column_alignment_for_items(items));
            }
            col = if needs_main_axis_fill {
                col.height(Length::Fill)
            } else {
                col.height(Length::Shrink)
            };
            for item in items {
                col = col.push(item);
            }
            col.into()
        }
    }
}

fn gap_on_main_axis(
    direction: FlexDirection,
    gap: Option<Spacing>,
    row_gap: Option<Spacing>,
    col_gap: Option<Spacing>,
) -> Spacing {
    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => col_gap.or(gap).unwrap_or(Spacing::S0),
        FlexDirection::Col | FlexDirection::ColReverse => row_gap.or(gap).unwrap_or(Spacing::S0),
    }
}

/// Create an iced flex layout for a given direction.
pub fn flex_direction_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced layout for `gap-*` utilities.
pub fn gap_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced layout for `gap-x-*` utilities.
///
/// In single-line flex layouts, this maps to the main axis for row directions.
pub fn gap_x_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap_x: Spacing,
) -> iced::Element<'a, Message> {
    let gap = gap_on_main_axis(direction, None, None, Some(gap_x));
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced layout for `gap-y-*` utilities.
///
/// In single-line flex layouts, this maps to the main axis for column directions.
pub fn gap_y_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap_y: Spacing,
) -> iced::Element<'a, Message> {
    let gap = gap_on_main_axis(direction, None, Some(gap_y), None);
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced flex layout for a typed align-items value.
pub fn align_items_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
    align_items: AlignItems,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, Some(align_items), None)
}

/// Create an iced flex layout for a typed justify-content value.
pub fn justify_content_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
    justify_content: JustifyContent,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, None, Some(justify_content))
}

fn fraction_to_portion(numerator: u16, denominator: u16) -> u16 {
    if denominator == 0 || numerator == 0 {
        return 0;
    }

    let scaled = (u32::from(numerator) * 100) / u32::from(denominator.max(1));
    scaled.clamp(1, u32::from(u16::MAX)) as u16
}

fn arbitrary_to_length(value: &str) -> Option<Length> {
    let raw = value.trim();
    if raw.eq_ignore_ascii_case("auto") {
        return Some(Length::Fill);
    }
    if raw.eq_ignore_ascii_case("none")
        || raw.eq_ignore_ascii_case("initial")
        || raw == "0 auto"
        || raw == "0 1 auto"
        || raw == "0"
    {
        return Some(Length::Shrink);
    }

    if let Ok(number) = raw.parse::<u16>() {
        return if number == 0 {
            Some(Length::Shrink)
        } else {
            Some(Length::FillPortion(number))
        };
    }

    if let Some((num, den)) = raw.split_once('/') {
        let numerator = num.trim().parse::<u16>().ok()?;
        let denominator = den.trim().parse::<u16>().ok()?;
        let portion = fraction_to_portion(numerator, denominator);
        return if portion == 0 {
            Some(Length::Shrink)
        } else {
            Some(Length::FillPortion(portion))
        };
    }

    // Handle common arbitrary flex shorthands like "3 1 auto" or "2 0 10rem"
    // by mapping the first grow factor to iced FillPortion.
    let tokens = raw.split_whitespace().collect::<Vec<_>>();
    if let Some(first) = tokens.first()
        && let Ok(grow) = first.parse::<u16>()
    {
        return if grow == 0 {
            Some(Length::Shrink)
        } else {
            Some(Length::FillPortion(grow))
        };
    }

    None
}

fn flex_item_length(flex: &Flex) -> Length {
    match flex {
        Flex::Number(0) => Length::Shrink,
        Flex::Number(number) => Length::FillPortion(*number),
        Flex::Fraction {
            numerator,
            denominator,
        } => {
            let portion = fraction_to_portion(*numerator, denominator.get());
            if portion == 0 {
                Length::Shrink
            } else {
                Length::FillPortion(portion)
            }
        }
        Flex::Auto => Length::Fill,
        Flex::Initial | Flex::None => Length::Shrink,
        Flex::CustomProperty(_) => Length::Shrink,
        Flex::Arbitrary(value) => arbitrary_to_length(value).unwrap_or(Length::Shrink),
    }
}

fn custom_property_length(name: &str, vars: &[(&str, &str)]) -> Option<Length> {
    vars.iter()
        .find(|(key, _)| *key == name)
        .and_then(|(_, value)| arbitrary_to_length(value))
}

/// Apply flex-item shorthand (`flex-*`) for a child element in an iced flex container.
///
/// `direction` is the parent container direction and determines which axis receives
/// the fill/shrink strategy.
pub fn apply_flex_item<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
    direction: FlexDirection,
) -> iced::Element<'a, Message> {
    apply_flex_item_with_custom_properties(content, style, direction, &[])
}

/// Apply flex-item shorthand with custom property values resolver for
/// `flex-(<custom-property>)` use cases.
pub fn apply_flex_item_with_custom_properties<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
    direction: FlexDirection,
    custom_properties: &[(&str, &str)],
) -> iced::Element<'a, Message> {
    let numeric_custom_properties = if style_uses_numeric_custom_properties(style) {
        custom_properties
            .iter()
            .filter_map(|(name, value)| value.parse::<f32>().ok().map(|parsed| (*name, parsed)))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let element = apply_layout_with_custom_properties(content, style, &numeric_custom_properties);
    let Some(flex) = style.flex_item.as_ref() else {
        return element;
    };

    let length = match flex {
        Flex::CustomProperty(name) => {
            custom_property_length(name, custom_properties).unwrap_or(Length::Shrink)
        }
        _ => flex_item_length(flex),
    };
    let wrapper = match direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            iced::widget::container(element).width(length)
        }
        FlexDirection::Col | FlexDirection::ColReverse => {
            iced::widget::container(element).height(length)
        }
    };

    wrapper.into()
}

fn style_uses_numeric_custom_properties(style: &Style) -> bool {
    let padding_uses_var = style.padding.is_some_and(|padding| {
        [padding.top, padding.right, padding.bottom, padding.left]
            .into_iter()
            .flatten()
            .any(|value| matches!(value, PaddingValue::Var(_)))
    });

    let margin_uses_var = style.margin.is_some_and(|margin| {
        [margin.top, margin.right, margin.bottom, margin.left]
            .into_iter()
            .flatten()
            .any(|value| matches!(value, MarginValue::Var(_)))
    });

    let width_uses_var = style.width.is_some_and(|width| {
        matches!(
            width.0,
            Some(crate::utilities::Size::Var(_)) | Some(crate::utilities::Size::HeightVar(_))
        )
    });
    let height_uses_var = style.height.is_some_and(|height| {
        matches!(
            height.0,
            Some(crate::utilities::Size::Var(_)) | Some(crate::utilities::Size::HeightVar(_))
        )
    });

    padding_uses_var || margin_uses_var || width_uses_var || height_uses_var
}

#[derive(Debug, Clone, Copy)]
struct BorderCanvas {
    border_style: BorderStyle,
    border_width: f32,
    border_radius: f32,
    border_color: iced::Color,
    background: Option<iced::Color>,
}

impl<Message> canvas::Program<Message> for BorderCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let fill_path = canvas::Path::rounded_rectangle(
            Point::ORIGIN,
            bounds.size(),
            border::Radius::from(self.border_radius.max(0.0)),
        );
        if let Some(bg) = self.background {
            frame.fill(&fill_path, bg);
        }

        match self.border_style {
            BorderStyle::None | BorderStyle::Hidden => {}
            BorderStyle::Double => {
                let outer = border_path(bounds.size(), self.border_width * 0.5, self.border_radius);
                let inner = border_path(
                    bounds.size(),
                    self.border_width * 2.0,
                    (self.border_radius - self.border_width).max(0.0),
                );
                frame.stroke(
                    &outer,
                    canvas::Stroke::default()
                        .with_width(self.border_width.max(1.0))
                        .with_color(self.border_color),
                );
                frame.stroke(
                    &inner,
                    canvas::Stroke::default()
                        .with_width(self.border_width.max(1.0))
                        .with_color(self.border_color),
                );
            }
            BorderStyle::Solid => {
                let path = border_path(bounds.size(), self.border_width * 0.5, self.border_radius);
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_width(self.border_width.max(1.0))
                        .with_color(self.border_color),
                );
            }
            BorderStyle::Dashed => {
                let path = border_path(bounds.size(), self.border_width * 0.5, self.border_radius);
                frame.stroke(
                    &path,
                    canvas::Stroke {
                        line_dash: canvas::LineDash {
                            segments: &[8.0, 5.0],
                            offset: 0,
                        },
                        ..canvas::Stroke::default()
                    }
                    .with_width(self.border_width.max(1.0))
                    .with_color(self.border_color),
                );
            }
            BorderStyle::Dotted => {
                let path = border_path(bounds.size(), self.border_width * 0.5, self.border_radius);
                frame.stroke(
                    &path,
                    canvas::Stroke {
                        line_cap: canvas::LineCap::Round,
                        line_dash: canvas::LineDash {
                            segments: &[1.0, 5.0],
                            offset: 0,
                        },
                        ..canvas::Stroke::default()
                    }
                    .with_width(self.border_width.max(1.0))
                    .with_color(self.border_color),
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

fn border_path(size: Size, inset: f32, radius: f32) -> canvas::Path {
    let width = (size.width - inset * 2.0).max(1.0);
    let height = (size.height - inset * 2.0).max(1.0);
    canvas::Path::rounded_rectangle(
        Point::new(inset, inset),
        Size::new(width, height),
        border::Radius::from(radius.max(0.0)),
    )
}

/// Create an `iced` button directly from `twill::Button`.
pub fn twill_button<'a, Message: Clone + 'a>(
    button_cfg: &crate::components::Button,
    label: &'a str,
    on_press: Message,
) -> iced::Element<'a, Message> {
    let style = button_cfg.style();
    let opacity = resolved_opacity(&style);
    let text_color = style.text_color.unwrap_or(Color::white());
    let border_color = style
        .border_color
        .map(to_color)
        .map(|color| apply_opacity_to_color(color, opacity))
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
        .map(|padding| to_style_padding(padding, &[]))
        .unwrap_or(iced::Padding {
            top: 8.0,
            right: 16.0,
            bottom: 8.0,
            left: 16.0,
        });

    let variant = button_cfg.variant();
    let base_bg_token = style.background_color;

    let mut widget = iced::widget::button(iced::widget::text(label).color(to_color(text_color)))
        .padding(padding)
        .style(move |_theme, status| {
            let mut background_value =
                base_bg_token.and_then(|bg| resolve_background_color_token(bg, Some(text_color)));
            let mut resolved_text = apply_opacity_to_color(to_color(text_color), opacity);

            let is_dark_theme = matches!(_theme, iced::Theme::Dark);
            if matches!(variant, crate::components::ButtonVariant::Ghost) {
                resolved_text = if is_dark_theme {
                    apply_opacity_to_color(to_color(Color::gray(Scale::S100)), opacity)
                } else {
                    apply_opacity_to_color(to_color(Color::gray(Scale::S900)), opacity)
                };
                if matches!(
                    status,
                    iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed
                ) {
                    background_value = Some(if is_dark_theme {
                        Color::gray(Scale::S800).compute()
                    } else {
                        Color::gray(Scale::S100).compute()
                    });
                } else {
                    background_value = None;
                }
            }

            if matches!(variant, crate::components::ButtonVariant::Outline) {
                resolved_text = if is_dark_theme {
                    apply_opacity_to_color(to_color(Color::gray(Scale::S100)), opacity)
                } else {
                    apply_opacity_to_color(to_color(Color::gray(Scale::S900)), opacity)
                };
            }

            if let Some(value) = background_value {
                background_value = Some(match status {
                    iced::widget::button::Status::Hovered => value.darken_oklch(0.05),
                    iced::widget::button::Status::Pressed => value.darken_oklch(0.10),
                    _ => value,
                });
            }

            iced::widget::button::Style {
                background: background_value.map(|v| {
                    iced::Background::Color(to_color_value(apply_opacity_to_color_value(
                        v, opacity,
                    )))
                }),
                text_color: resolved_text,
                border: iced::Border {
                    radius: border_radius.into(),
                    width: border_width,
                    color: border_color,
                },
                ..Default::default()
            }
        });

    if !button_cfg.is_disabled() {
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
        let raw = blue.compute();
        let (r, g, b) = raw.to_rgb8();
        assert!((c.r - r as f32 / 255.0).abs() < 0.01);
        assert!((c.g - g as f32 / 255.0).abs() < 0.01);
        assert!((c.b - b as f32 / 255.0).abs() < 0.01);
    }

    #[test]
    fn test_color_conversion_uses_raw_values() {
        let color = Color::blue(Scale::S500);
        let converted = to_color(color);
        let raw = color.compute();
        let (r, g, b) = raw.to_rgb8();
        assert!((converted.r - r as f32 / 255.0).abs() < 0.001);
        assert!((converted.g - g as f32 / 255.0).abs() < 0.001);
        assert!((converted.b - b as f32 / 255.0).abs() < 0.001);
    }

    #[test]
    fn test_font_size_resolution_variants() {
        const TITLE_SIZE: crate::tokens::FontSizeVar = crate::tokens::FontSizeVar::new("--title");
        assert!((to_font_size(FontSize::Base) - 16.0).abs() < f32::EPSILON);
        assert_eq!(resolve_font_size(FontSize::var(TITLE_SIZE), &[]), None);
        assert_eq!(
            resolve_font_size(FontSize::var(TITLE_SIZE), &[("--title", 28.0)]),
            Some(28.0)
        );
        assert_eq!(resolve_font_size(FontSize::px(22), &[]), Some(22.0));
    }

    #[test]
    fn test_font_weight_mapping_variants() {
        assert_eq!(to_font_weight(FontWeight::Thin), iced::font::Weight::Thin);
        assert_eq!(
            to_font_weight(FontWeight::ExtraLight),
            iced::font::Weight::ExtraLight
        );
        assert_eq!(to_font_weight(FontWeight::Light), iced::font::Weight::Light);
        assert_eq!(
            to_font_weight(FontWeight::Normal),
            iced::font::Weight::Normal
        );
        assert_eq!(
            to_font_weight(FontWeight::Medium),
            iced::font::Weight::Medium
        );
        assert_eq!(
            to_font_weight(FontWeight::SemiBold),
            iced::font::Weight::Semibold
        );
        assert_eq!(to_font_weight(FontWeight::Bold), iced::font::Weight::Bold);
        assert_eq!(
            to_font_weight(FontWeight::ExtraBold),
            iced::font::Weight::ExtraBold
        );
        assert_eq!(to_font_weight(FontWeight::Black), iced::font::Weight::Black);
    }

    #[test]
    fn test_text_alignment_mapping_variants() {
        assert_eq!(
            to_text_alignment(TextAlign::Left),
            iced::widget::text::Alignment::Left
        );
        assert_eq!(
            to_text_alignment(TextAlign::Center),
            iced::widget::text::Alignment::Center
        );
        assert_eq!(
            to_text_alignment(TextAlign::Right),
            iced::widget::text::Alignment::Right
        );
        assert_eq!(
            to_text_alignment(TextAlign::Justify),
            iced::widget::text::Alignment::Justified
        );
        assert_eq!(
            to_text_alignment_with_direction(TextAlign::Start, false),
            iced::widget::text::Alignment::Left
        );
        assert_eq!(
            to_text_alignment_with_direction(TextAlign::Start, true),
            iced::widget::text::Alignment::Right
        );
        assert_eq!(
            to_text_alignment_with_direction(TextAlign::End, false),
            iced::widget::text::Alignment::Right
        );
        assert_eq!(
            to_text_alignment_with_direction(TextAlign::End, true),
            iced::widget::text::Alignment::Left
        );
    }

    #[test]
    fn test_spacing_px_padding() {
        let p = to_padding(Spacing::Px);
        assert!((p.top - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_style_padding_allows_partial_sides() {
        let px = to_style_padding(crate::utilities::Padding::x(Spacing::S4), &[]);
        assert!((px.left - 16.0).abs() < f32::EPSILON);
        assert!((px.right - 16.0).abs() < f32::EPSILON);
        assert!((px.top - 0.0).abs() < f32::EPSILON);
        assert!((px.bottom - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_style_padding_logical_aliases_map_to_physical_sides() {
        let ps = to_style_padding(crate::utilities::Padding::ps(Spacing::S6), &[]);
        let pe = to_style_padding(crate::utilities::Padding::pe(Spacing::S6), &[]);
        let pbs = to_style_padding(crate::utilities::Padding::pbs(Spacing::S2), &[]);
        let pbe = to_style_padding(crate::utilities::Padding::pbe(Spacing::S2), &[]);

        assert!((ps.left - 24.0).abs() < f32::EPSILON);
        assert!((ps.right - 0.0).abs() < f32::EPSILON);
        assert!((pe.left - 0.0).abs() < f32::EPSILON);
        assert!((pe.right - 24.0).abs() < f32::EPSILON);
        assert!((pbs.top - 8.0).abs() < f32::EPSILON);
        assert!((pbe.bottom - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_style_padding_px_token_families() {
        let p = to_style_padding(crate::utilities::Padding::p(Spacing::Px), &[]);
        let px = to_style_padding(crate::utilities::Padding::px(Spacing::Px), &[]);
        let py = to_style_padding(crate::utilities::Padding::py(Spacing::Px), &[]);
        let pt = to_style_padding(crate::utilities::Padding::pt(Spacing::Px), &[]);
        let pr = to_style_padding(crate::utilities::Padding::pr(Spacing::Px), &[]);
        let pb = to_style_padding(crate::utilities::Padding::pb(Spacing::Px), &[]);
        let pl = to_style_padding(crate::utilities::Padding::pl(Spacing::Px), &[]);
        let ps = to_style_padding(crate::utilities::Padding::ps(Spacing::Px), &[]);
        let pe = to_style_padding(crate::utilities::Padding::pe(Spacing::Px), &[]);
        let pbs = to_style_padding(crate::utilities::Padding::pbs(Spacing::Px), &[]);
        let pbe = to_style_padding(crate::utilities::Padding::pbe(Spacing::Px), &[]);

        assert!((p.top - 1.0).abs() < f32::EPSILON);
        assert!((p.right - 1.0).abs() < f32::EPSILON);
        assert!((p.bottom - 1.0).abs() < f32::EPSILON);
        assert!((p.left - 1.0).abs() < f32::EPSILON);
        assert!((px.left - 1.0).abs() < f32::EPSILON);
        assert!((px.right - 1.0).abs() < f32::EPSILON);
        assert!((py.top - 1.0).abs() < f32::EPSILON);
        assert!((py.bottom - 1.0).abs() < f32::EPSILON);
        assert!((pt.top - 1.0).abs() < f32::EPSILON);
        assert!((pr.right - 1.0).abs() < f32::EPSILON);
        assert!((pb.bottom - 1.0).abs() < f32::EPSILON);
        assert!((pl.left - 1.0).abs() < f32::EPSILON);
        assert!((ps.left - 1.0).abs() < f32::EPSILON);
        assert!((pe.right - 1.0).abs() < f32::EPSILON);
        assert!((pbs.top - 1.0).abs() < f32::EPSILON);
        assert!((pbe.bottom - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_style_padding_custom_property_and_arbitrary_values() {
        let var = crate::utilities::PaddingVar::new("--pad");
        let p = crate::utilities::Padding::individual_value(
            PaddingValue::var(var),
            PaddingValue::px(5.0),
            PaddingValue::rem(1.0),
            PaddingValue::Scale(Spacing::S2),
        );
        let resolved = to_style_padding(p, &[("--pad", 12.0)]);
        assert!((resolved.top - 12.0).abs() < f32::EPSILON);
        assert!((resolved.right - 5.0).abs() < f32::EPSILON);
        assert!((resolved.bottom - 16.0).abs() < f32::EPSILON);
        assert!((resolved.left - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resolve_width_fixed_and_container_variants() {
        assert_eq!(
            resolve_width(crate::utilities::Width::w(Spacing::S24), &[]),
            Some(ResolvedWidth::Length(Length::Fixed(96.0)))
        );
        assert_eq!(
            resolve_width(
                crate::utilities::Width::w_container(crate::tokens::Container::Md),
                &[]
            ),
            Some(ResolvedWidth::Length(Length::Fixed(448.0)))
        );
        assert_eq!(
            resolve_width(crate::utilities::Width::w_px(), &[]),
            Some(ResolvedWidth::Length(Length::Fixed(1.0)))
        );
        assert_eq!(
            resolve_width(crate::utilities::Width::w_full(), &[]),
            Some(ResolvedWidth::Length(Length::Fill))
        );
        assert_eq!(
            resolve_width(crate::utilities::Width::w_auto(), &[]),
            Some(ResolvedWidth::Length(Length::Shrink))
        );
    }

    #[test]
    fn test_resolve_width_fraction_variants() {
        let half = resolve_width(
            crate::utilities::Width::w_fraction(crate::tokens::Percentage::S1_2),
            &[],
        );
        match half {
            Some(ResolvedWidth::Ratio(ratio)) => assert!((ratio - 0.5).abs() < f32::EPSILON),
            _ => panic!("expected ratio width"),
        }

        let two_fifths = resolve_width(
            crate::utilities::Width::w_fraction(crate::tokens::Percentage::S2_5),
            &[],
        );
        match two_fifths {
            Some(ResolvedWidth::Ratio(ratio)) => assert!((ratio - 0.4).abs() < 0.0001),
            _ => panic!("expected ratio width"),
        }
    }

    #[test]
    fn test_resolve_width_custom_property_and_px_value() {
        let var = crate::utilities::WidthVar::new("--panel-w");
        assert_eq!(
            resolve_width(crate::utilities::Width::w_var(var), &[("--panel-w", 280.0)]),
            Some(ResolvedWidth::Length(Length::Fixed(280.0)))
        );
        assert_eq!(
            resolve_width(crate::utilities::Width::w_px_value(320), &[]),
            Some(ResolvedWidth::Length(Length::Fixed(320.0)))
        );
    }

    #[test]
    fn test_resolve_height_fixed_and_variant_families() {
        assert_eq!(
            resolve_height(crate::utilities::Height::h(Spacing::S24), &[]),
            Some(ResolvedHeight::Length(Length::Fixed(96.0)))
        );
        assert_eq!(
            resolve_height(crate::utilities::Height::h_px(), &[]),
            Some(ResolvedHeight::Length(Length::Fixed(1.0)))
        );
        assert_eq!(
            resolve_height(crate::utilities::Height::h_full(), &[]),
            Some(ResolvedHeight::Length(Length::Fill))
        );
        assert_eq!(
            resolve_height(crate::utilities::Height::h_auto(), &[]),
            Some(ResolvedHeight::Length(Length::Shrink))
        );
        assert_eq!(
            resolve_height(crate::utilities::Height::h_lh(), &[]),
            Some(ResolvedHeight::Length(Length::Shrink))
        );
    }

    #[test]
    fn test_resolve_height_fraction_variants() {
        let half = resolve_height(
            crate::utilities::Height::h_fraction(crate::tokens::Percentage::S1_2),
            &[],
        );
        match half {
            Some(ResolvedHeight::Ratio(ratio)) => assert!((ratio - 0.5).abs() < f32::EPSILON),
            _ => panic!("expected ratio height"),
        }

        let two_fifths = resolve_height(
            crate::utilities::Height::h_fraction(crate::tokens::Percentage::S2_5),
            &[],
        );
        match two_fifths {
            Some(ResolvedHeight::Ratio(ratio)) => assert!((ratio - 0.4).abs() < 0.0001),
            _ => panic!("expected ratio height"),
        }
    }

    #[test]
    fn test_resolve_height_custom_property_and_px_value() {
        let var = crate::utilities::HeightVar::new("--panel-h");
        assert_eq!(
            resolve_height(
                crate::utilities::Height::h_var(var),
                &[("--panel-h", 240.0)]
            ),
            Some(ResolvedHeight::Length(Length::Fixed(240.0)))
        );
        assert_eq!(
            resolve_height(crate::utilities::Height::h_px_value(360), &[]),
            Some(ResolvedHeight::Length(Length::Fixed(360.0)))
        );
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

    #[test]
    fn test_shadow_layers_for_sm_are_two() {
        let layers = to_shadow_layers_with_color(Shadow::Sm, None);
        assert_eq!(layers.len(), 2);
        assert!((layers[0].offset.y - 1.0).abs() < f32::EPSILON);
        assert!((layers[1].offset.y - 1.0).abs() < f32::EPSILON);
        assert!((layers[0].blur_radius - 3.0).abs() < f32::EPSILON);
        assert!((layers[1].blur_radius - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_shadow_layers_for_2xl_are_single() {
        let layers = to_shadow_layers_with_color(Shadow::S2xl, None);
        assert_eq!(layers.len(), 1);
        assert!((layers[0].offset.y - 25.0).abs() < f32::EPSILON);
        assert!((layers[0].blur_radius - 50.0).abs() < f32::EPSILON);
        assert!((layers[0].color.a - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resolve_columns_count_from_count() {
        let count = resolve_columns_count(Columns::count(3), 900.0, 16.0, DEFAULT_MAX_COLUMNS);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_resolve_columns_count_from_width() {
        let count = resolve_columns_count(
            Columns::width(Container::S3xs),
            820.0,
            16.0,
            DEFAULT_MAX_COLUMNS,
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn test_resolve_columns_count_from_custom_width() {
        let count =
            resolve_columns_count(Columns::width_px(280.0), 900.0, 20.0, DEFAULT_MAX_COLUMNS);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_resolve_columns_count_auto_is_one() {
        let count = resolve_columns_count(Columns::auto(), 1200.0, 16.0, DEFAULT_MAX_COLUMNS);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_resolve_columns_count_small_width_is_one() {
        let count = resolve_columns_count(
            Columns::width(Container::S3xs),
            120.0,
            16.0,
            DEFAULT_MAX_COLUMNS,
        );
        assert_eq!(count, 1);
    }

    #[test]
    fn test_resolve_columns_count_is_capped() {
        let by_count = resolve_columns_count(Columns::count(255), 5000.0, 0.0, usize::MAX);
        assert_eq!(by_count, ABSOLUTE_MAX_COLUMNS);

        let by_width = resolve_columns_count(Columns::width_px(1.0), 20_000.0, 0.0, usize::MAX);
        assert_eq!(by_width, ABSOLUTE_MAX_COLUMNS);
    }

    #[test]
    fn test_resolve_columns_count_respects_style_max() {
        let count = resolve_columns_count(Columns::width(Container::S3xs), 1200.0, 16.0, 2);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_track_count_from_repeat_value() {
        assert_eq!(
            track_count_from_template_value("repeat(4, minmax(0, 1fr))"),
            Some(4)
        );
    }

    #[test]
    fn test_track_count_from_explicit_tracks() {
        assert_eq!(
            track_count_from_template_value("200px minmax(0, 1fr) 100px"),
            Some(3)
        );
    }

    #[test]
    fn test_resolve_grid_template_track_count_variants() {
        assert_eq!(
            resolve_grid_template_track_count(&GridTemplate::count(3), None, &[]),
            3
        );
        assert_eq!(
            resolve_grid_template_track_count(&GridTemplate::none(), None, &[]),
            1
        );
        assert_eq!(
            resolve_grid_template_track_count(&GridTemplate::subgrid(), Some(5), &[]),
            5
        );
        assert_eq!(
            resolve_grid_template_track_count(
                &GridTemplate::custom_property("--layout-cols"),
                None,
                &[("--layout-cols", "repeat(6, minmax(0, 1fr))")]
            ),
            6
        );
        assert_eq!(
            resolve_grid_template_track_count(
                &GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px"),
                None,
                &[]
            ),
            3
        );
    }

    #[test]
    fn test_grid_template_columns_layout_builds() {
        let _: iced::Element<'_, ()> =
            grid_template_columns_layout(vec![], GridTemplate::count(4), Spacing::S4);
        let _: iced::Element<'_, ()> = grid_template_columns_layout_with_context(
            vec![],
            GridTemplate::subgrid(),
            Spacing::S2,
            Some(3),
            &[],
        );
    }

    #[test]
    fn test_resolve_column_width_handles_tight_space() {
        let width = resolve_column_width(100.0, 3, 80.0);
        assert_eq!(width, 0.0);
    }

    #[test]
    fn test_resolve_column_width_handles_invalid_gap() {
        let width = resolve_column_width(600.0, 3, f32::NAN);
        assert!((width - 200.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resolve_aspect_size_by_width() {
        let size = resolve_aspect_size(240.0, 600.0, 1.0);
        assert!((size.width - 240.0).abs() < f32::EPSILON);
        assert!((size.height - 240.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resolve_aspect_size_height_capped() {
        let size = resolve_aspect_size(400.0, 100.0, 16.0 / 9.0);
        assert!((size.height - 100.0).abs() < f32::EPSILON);
        assert!(size.width > 170.0 && size.width < 180.0);
    }

    #[test]
    fn test_resolve_aspect_size_invalid_ratio_defaults() {
        let size = resolve_aspect_size(120.0, 80.0, 0.0);
        assert!((size.width - 80.0).abs() < f32::EPSILON);
        assert!((size.height - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reverse_directions_detection() {
        assert!(is_reverse_direction(FlexDirection::RowReverse));
        assert!(is_reverse_direction(FlexDirection::ColReverse));
        assert!(!is_reverse_direction(FlexDirection::Row));
        assert!(!is_reverse_direction(FlexDirection::Col));
    }

    #[test]
    fn test_flex_direction_layout_reverse_builds() {
        let _: iced::Element<'_, ()> =
            flex_direction_layout(vec![], FlexDirection::RowReverse, Spacing::S4);
        let _: iced::Element<'_, ()> =
            flex_direction_layout(vec![], FlexDirection::ColReverse, Spacing::S4);
    }

    #[test]
    fn test_align_items_safe_and_baseline_mapping() {
        assert_eq!(
            row_alignment_for_items(AlignItems::EndSafe),
            iced::alignment::Vertical::Bottom
        );
        assert_eq!(
            row_alignment_for_items(AlignItems::BaselineLast),
            iced::alignment::Vertical::Bottom
        );
        assert_eq!(
            column_alignment_for_items(AlignItems::CenterSafe),
            iced::alignment::Horizontal::Center
        );
    }

    #[test]
    fn test_align_items_layout_builds() {
        let _: iced::Element<'_, ()> = align_items_layout(
            vec![],
            FlexDirection::Row,
            Spacing::S4,
            AlignItems::CenterSafe,
        );
        let _: iced::Element<'_, ()> = align_items_layout(
            vec![],
            FlexDirection::ColReverse,
            Spacing::S2,
            AlignItems::Stretch,
        );
    }

    #[test]
    fn test_gap_main_axis_mapping() {
        assert_eq!(
            gap_on_main_axis(
                FlexDirection::Row,
                Some(Spacing::S4),
                Some(Spacing::S2),
                Some(Spacing::S8)
            ),
            Spacing::S8
        );
        assert_eq!(
            gap_on_main_axis(
                FlexDirection::RowReverse,
                Some(Spacing::S4),
                Some(Spacing::S2),
                None
            ),
            Spacing::S4
        );
        assert_eq!(
            gap_on_main_axis(
                FlexDirection::Col,
                Some(Spacing::S4),
                Some(Spacing::S6),
                Some(Spacing::S8)
            ),
            Spacing::S6
        );
        assert_eq!(
            gap_on_main_axis(FlexDirection::ColReverse, None, None, Some(Spacing::S8)),
            Spacing::S0
        );
    }

    #[test]
    fn test_gap_layout_builds() {
        let _: iced::Element<'_, ()> = gap_layout(vec![], FlexDirection::Row, Spacing::S4);
        let _: iced::Element<'_, ()> = gap_x_layout(vec![], FlexDirection::Row, Spacing::S6);
        let _: iced::Element<'_, ()> = gap_y_layout(vec![], FlexDirection::Col, Spacing::S3);
    }

    #[test]
    fn test_justify_content_safe_mapping() {
        assert_eq!(
            normalize_justify_content(JustifyContent::EndSafe),
            JustifyContent::End
        );
        assert_eq!(
            normalize_justify_content(JustifyContent::CenterSafe),
            JustifyContent::Center
        );
        assert_eq!(
            normalize_justify_content(JustifyContent::Between),
            JustifyContent::Between
        );
    }

    #[test]
    fn test_justify_content_layout_builds() {
        let _: iced::Element<'_, ()> = justify_content_layout(
            vec![],
            FlexDirection::Row,
            Spacing::S4,
            JustifyContent::Between,
        );
        let _: iced::Element<'_, ()> = justify_content_layout(
            vec![],
            FlexDirection::RowReverse,
            Spacing::S2,
            JustifyContent::CenterSafe,
        );
        let _: iced::Element<'_, ()> = justify_content_layout(
            vec![],
            FlexDirection::ColReverse,
            Spacing::S2,
            JustifyContent::Evenly,
        );
    }
}
