use crate::style::Style;
use crate::tokens::{ColorValue, Container, Spacing};
use crate::utilities::Columns;
use iced::Size;

pub(super) fn apply_opacity_to_color(mut color: iced::Color, opacity: f32) -> iced::Color {
    if opacity.is_finite() {
        color.a *= opacity.clamp(0.0, 1.0);
    }
    color
}

pub(super) fn apply_opacity_to_color_value(mut color: ColorValue, opacity: f32) -> ColorValue {
    if opacity.is_finite() {
        color.a *= opacity.clamp(0.0, 1.0);
    }
    color
}

pub(super) fn resolved_opacity(style: &Style) -> f32 {
    style.opacity.unwrap_or(1.0).clamp(0.0, 1.0)
}

pub(super) fn spacing_to_px(spacing: Spacing) -> f32 {
    match spacing.to_px() {
        Some(px) => px as f32,
        None => 0.0,
    }
}

pub(super) fn container_to_px(container: Container) -> f32 {
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

pub(super) fn sanitize_gap(gap: f32) -> f32 {
    if gap.is_finite() { gap.max(0.0) } else { 0.0 }
}

pub(crate) const DEFAULT_MAX_COLUMNS: usize = 16;
pub(crate) const ABSOLUTE_MAX_COLUMNS: usize = 64;

pub(super) fn normalize_max_columns(max_columns: usize) -> usize {
    max_columns.clamp(1, ABSOLUTE_MAX_COLUMNS)
}

pub(crate) fn resolve_columns_count(
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

pub(crate) fn resolve_column_width(available_width: f32, count: usize, gap: f32) -> f32 {
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

pub(crate) fn resolve_aspect_size(max_width: f32, max_height: f32, ratio: f32) -> Size {
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
