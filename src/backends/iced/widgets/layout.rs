use crate::style::Style;
use iced::{Length, Point, Rectangle, Size, Vector, mouse};

use super::container::styled_container_with_custom_properties;
use super::conversions::{
    MarginOffsets, ResolvedHeight, ResolvedMarginValue, ResolvedWidth, resolve_height,
    resolve_width, to_aspect_ratio, to_style_margin,
};
use super::ratio_boxes::{AspectRatioBox, HeightRatioBox, WidthRatioBox};
use iced::advanced::layout::{Layout as AdvancedLayout, Limits, Node};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Shell, Widget as AdvancedWidget};

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
