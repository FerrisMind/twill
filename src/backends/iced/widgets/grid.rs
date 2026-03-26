use crate::style::Style;
use crate::tokens::Spacing;
use crate::utilities::{Columns, GridTemplate};
use iced::advanced::layout::{Layout as AdvancedLayout, Limits, Node};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Shell, Widget as AdvancedWidget};
use iced::{Length, Point, Rectangle, Size, Vector, mouse};

use super::common::{
    ABSOLUTE_MAX_COLUMNS, DEFAULT_MAX_COLUMNS, normalize_max_columns, resolve_column_width,
    resolve_columns_count, sanitize_gap, spacing_to_px,
};

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

pub(crate) fn track_count_from_template_value(value: &str) -> Option<usize> {
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

pub(crate) fn resolve_grid_template_track_count(
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
