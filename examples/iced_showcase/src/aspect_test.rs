use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::{Element, Length, Size, Rectangle, Vector, mouse, Event};

pub struct AspectRatioNode<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    ratio: f32,
}

impl<'a, Message, Theme, Renderer> AspectRatioNode<'a, Message, Theme, Renderer> {
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>, ratio: f32) -> Self {
        Self {
            content: content.into(),
            ratio: ratio.max(0.0001),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for AspectRatioNode<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max_size = limits.max();
        let target_width = max_size.width;
        let target_height = target_width / self.ratio;

        let final_size = if target_height <= max_size.height {
            Size::new(target_width, target_height)
        } else {
            Size::new(max_size.height * self.ratio, max_size.height)
        };

        let new_limits = layout::Limits::new(Size::ZERO, final_size)
            .width(Length::Fixed(final_size.width))
            .height(Length::Fixed(final_size.height));

        let content_layout = self.content.as_widget().layout(&mut tree.children[0], renderer, &new_limits);
        let size = content_layout.size();
        
        layout::Node::with_children(size, vec![content_layout])
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        )
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.content.as_widget().operate(
            &mut state.children[0],
            layout.children().next().unwrap(),
            renderer,
            operation,
        )
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> iced::event::Status {
        self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut state.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> Into<Element<'a, Message, Theme, Renderer>>
    for AspectRatioNode<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn into(self) -> Element<'a, Message, Theme, Renderer> {
        Element::new(self)
    }
}
