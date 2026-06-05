use iced::advanced::layout::{Layout as AdvancedLayout, Limits, Node};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Shell, Widget as AdvancedWidget};
use iced::{Length, Rectangle, Size, Vector, mouse};

use super::common::resolve_aspect_size;

pub(super) struct AspectRatioBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> AspectRatioBox<'a, Message, Theme, Renderer> {
    pub(super) fn new(child: iced::Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
        Self {
            child,
            ratio: ratio.max(0.0001),
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    pub(super) fn width(mut self, width: Length) -> Self {
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

pub(super) struct WidthRatioBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> WidthRatioBox<'a, Message, Theme, Renderer> {
    pub(super) fn new(child: iced::Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
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

pub(super) struct HeightRatioBox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    child: iced::Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> HeightRatioBox<'a, Message, Theme, Renderer> {
    pub(super) fn new(child: iced::Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
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
