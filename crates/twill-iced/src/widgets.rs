mod common;
mod container;
mod conversions;
mod flex;
mod grid;
mod layout;
mod ratio_boxes;

pub use container::{
    styled_container, styled_container_with_custom_properties, styled_container_with_semantic_theme,
};
pub use conversions::{
    SemanticThemeSource, resolve_font_size, to_aspect_ratio, to_blur_radius, to_border_radius,
    to_color, to_color_value, to_content_fit, to_duration, to_easing, to_font_size, to_font_weight,
    to_interaction, to_padding, to_semantic_color, to_semantic_color_with_theme, to_shadow,
    to_shadow_layers_with_color, to_shadow_with_color, to_text_alignment,
    to_text_alignment_with_direction,
};
pub use flex::{
    align_items_layout, apply_flex_item, apply_flex_item_with_custom_properties,
    flex_direction_layout, gap_layout, gap_x_layout, gap_y_layout, justify_content_layout,
};
pub use grid::{
    columns_layout, grid_template_columns_layout, grid_template_columns_layout_with_context,
};
pub use layout::{
    apply_layout, apply_layout_with_custom_properties, apply_layout_with_semantic_theme,
};

#[cfg(test)]
pub(crate) use common::{
    ABSOLUTE_MAX_COLUMNS, DEFAULT_MAX_COLUMNS, resolve_aspect_size, resolve_column_width,
    resolve_columns_count,
};
#[cfg(test)]
pub(crate) use conversions::{
    ResolvedHeight, ResolvedWidth, resolve_background_color_token_with_semantic_theme,
    resolve_border_color_token_with_semantic_theme, resolve_height,
    resolve_text_color_token_with_semantic_theme, resolve_width, to_style_padding,
};
#[cfg(test)]
pub(crate) use flex::{
    column_alignment_for_items, gap_on_main_axis, is_reverse_direction, normalize_justify_content,
    row_alignment_for_items,
};
#[cfg(test)]
pub(crate) use grid::{resolve_grid_template_track_count, track_count_from_template_value};

#[cfg(test)]
mod tests;
