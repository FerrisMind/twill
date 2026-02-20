//! Iced backend for twill.
//!
//! Converts twill styles to iced types.

use crate::style::Style;
use crate::tokens::{
    AspectRatio, Blur, BorderRadius, BorderStyle, Color, Container, Cursor, FontSize, FontWeight,
    Scale, SemanticColor, SemanticThemeVars, Shadow, Spacing, TransitionDuration,
};
use crate::traits::ComputeValue;
use crate::utilities::{Columns, Flex, FlexDirection, ObjectFit};
use iced::advanced::layout::{Layout as AdvancedLayout, Limits, Node};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Shell, Widget as AdvancedWidget};
use iced::widget::{canvas, stack};
use iced::{ContentFit, Length, Point, Rectangle, Renderer, Size, Theme, Vector, border, mouse};

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
    let color = SemanticThemeVars::shadcn_neutral()
        .resolve(semantic, is_dark)
        .unwrap_or(Color::black());
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
        });

    let bg_color = style.background_color.map(to_color);
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
        .unwrap_or(iced::Color::TRANSPARENT);
    let border_style = style.border_style.unwrap_or(BorderStyle::Solid);
    let border_width = base_border_width;
    let shadow = style
        .box_shadow
        .map(|s| to_shadow_with_color(s, style.shadow_color))
        .unwrap_or_default();

    match border_style {
        BorderStyle::Solid => {
            let mut container = iced::widget::container(content);
            if let Some(p) = padding {
                container = container.padding(p);
            }
            container.style(move |_| iced::widget::container::Style {
                background: bg_color.map(iced::Background::Color),
                border: iced::Border {
                    radius: border_radius.into(),
                    width: border_width,
                    color: border_color,
                },
                shadow,
                ..Default::default()
            })
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

            iced::widget::container(stack![border_layer, content_layer]).style(move |_| {
                iced::widget::container::Style {
                    shadow,
                    ..Default::default()
                }
            })
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
                let child = children
                    .next()
                    .expect("columns flow missing expected child state");
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
                let child = children
                    .next()
                    .expect("columns flow missing expected child state");
                let mut node = element
                    .as_widget_mut()
                    .layout(child, renderer, &child_limits);

                let (column_index, column_height) = heights
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, height)| (index, *height))
                    .expect("columns flow has at least one column");

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

        let child_state = tree
            .children
            .first_mut()
            .expect("aspect ratio box missing child state");
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

/// Apply high-level layout wrappers like Display::Hidden, Overflow, MaxWidth
pub fn apply_layout<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
) -> iced::Element<'a, Message> {
    if matches!(style.display, Some(crate::utilities::Display::Hidden)) {
        return iced::widget::Space::new().into();
    }

    let mut container = styled_container(content, style);

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

    if let Some(ratio) = style.aspect_ratio.and_then(to_aspect_ratio) {
        element = AspectRatioBox::new(element, ratio)
            .width(Length::Fill)
            .into();
    }

    element
}

fn is_reverse_direction(direction: FlexDirection) -> bool {
    matches!(
        direction,
        FlexDirection::RowReverse | FlexDirection::ColReverse
    )
}

/// Create an iced flex layout for `flex-direction` classes.
///
/// Tailwind mapping:
/// - `flex-row`
/// - `flex-row-reverse`
/// - `flex-col`
/// - `flex-col-reverse`
pub fn flex_direction_layout<'a, Message: Clone + 'a>(
    mut items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    if is_reverse_direction(direction) {
        items.reverse();
    }

    let gap = spacing_to_px(gap);

    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            let mut row = iced::widget::Row::new().spacing(gap).width(Length::Fill);
            for item in items {
                row = row.push(item);
            }
            row.into()
        }
        FlexDirection::Col | FlexDirection::ColReverse => {
            let mut col = iced::widget::Column::new().spacing(gap).width(Length::Fill);
            for item in items {
                col = col.push(item);
            }
            col.into()
        }
    }
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
    let element = apply_layout(content, style);
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

    let variant = button_cfg.variant;
    let base_bg_token = style.background_color;

    let mut widget = iced::widget::button(iced::widget::text(label).color(to_color(text_color)))
        .padding(padding)
        .style(move |_theme, status| {
            let mut background_value = base_bg_token.map(|c| c.compute());
            let mut resolved_text = to_color(text_color);

            let is_dark_theme = matches!(_theme, iced::Theme::Dark);
            if matches!(variant, crate::components::ButtonVariant::Ghost) {
                resolved_text = if is_dark_theme {
                    to_color(Color::gray(Scale::S100))
                } else {
                    to_color(Color::gray(Scale::S900))
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
                    to_color(Color::gray(Scale::S100))
                } else {
                    to_color(Color::gray(Scale::S900))
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
                background: background_value.map(|v| iced::Background::Color(to_color_value(v))),
                text_color: resolved_text,
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
}
