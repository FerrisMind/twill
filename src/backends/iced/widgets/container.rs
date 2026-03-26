use crate::style::Style;
use crate::tokens::BorderStyle;
use iced::widget::{canvas, stack};
use iced::{Point, Rectangle, Renderer, Size, Theme, border, mouse};

use super::common::{apply_opacity_to_color, apply_opacity_to_color_value, resolved_opacity};
use super::conversions::{
    resolve_background_color_token, shadow_layers_with_opacity, to_border_radius, to_color,
    to_color_value, to_style_padding, wrap_with_shadow_layers,
};

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
