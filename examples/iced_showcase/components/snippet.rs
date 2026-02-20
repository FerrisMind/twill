use iced::widget::{Space, column, container, text};
use iced::{Element, Length, Padding};
use twill::iced::{to_color, to_semantic_color};
use twill::tokens::{Color, Scale, SemanticColor};

pub struct Snippet<'a, Message> {
    title: &'a str,
    code: &'a str,
    visual: Element<'a, Message>,
}

impl<'a, Message> Snippet<'a, Message>
where
    Message: Clone + 'a,
{
    pub fn new(title: &'a str, code: &'a str, visual: impl Into<Element<'a, Message>>) -> Self {
        Self {
            title,
            code,
            visual: visual.into(),
        }
    }

    pub fn view(self, is_dark: bool) -> Element<'a, Message> {
        let border_color = to_semantic_color(SemanticColor::Border, is_dark);
        let bg_color = if is_dark {
            to_color(Color::gray(Scale::S800))
        } else {
            to_color(Color::gray(Scale::S100))
        };
        let code_color = if is_dark {
            to_color(Color::gray(Scale::S300))
        } else {
            to_color(Color::gray(Scale::S700))
        };

        // Top area: the visual preview
        let preview_area = container(self.visual)
            .width(Length::Fill)
            .padding(Padding::new(24.0))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        // Bottom area: the code snippet matching the visual
        let code_area = container(
            text(self.code)
                .font(iced::Font::MONOSPACE)
                .size(14)
                .color(code_color),
        )
        .width(Length::Fill)
        .padding(Padding::new(16.0))
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(bg_color)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        let main_card = column![
            // We use a custom style for the top half to only round top corners and add a border
            container(preview_area).style(move |_theme| {
                container::Style {
                    border: iced::Border {
                        color: border_color,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }
            }),
            code_area,
        ];

        column![
            text(self.title).size(20).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(12.0)),
            main_card,
        ]
        .spacing(4)
        .into()
    }
}
