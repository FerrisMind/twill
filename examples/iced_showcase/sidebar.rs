use iced::widget::{button, column, container, text, scrollable, Space};
use iced::{Element, Length};

use crate::pages::Page;
use crate::Message;
use twill::tokens::{Color, Scale, FontWeight};
use twill::iced::{to_color, to_font_weight};

pub struct Sidebar;

impl Default for Sidebar {
    fn default() -> Self {
        Self
    }
}

impl Sidebar {
    pub fn view(&self, current_page: Page) -> Element<'_, Message> {
        let pages = vec![
            Page::Typography,
            Page::Colors,
            Page::SemanticColors,
            Page::Spacing,
            Page::Borders,
            Page::Shadows,
            Page::Buttons,
            Page::Motion,
            Page::StyleBuilder,
            Page::KitchenSink,
            Page::Translator,
        ];

        let mut menu = column![].spacing(8);

        for page in pages {
            let is_selected = page == current_page;

            let label = if is_selected {
                text(page.title()).style(move |_| iced::widget::text::Style { color: Some(to_color(Color::blue(Scale::S500))) })
            } else {
                text(page.title())
            };

            let mut btn = button(label)
                .width(Length::Fill)
                .padding(12);

            if !is_selected {
                btn = btn.on_press(Message::PageSelected(page));
                btn = btn.style(iced::widget::button::text);
            } else {
                btn = btn.style(iced::widget::button::secondary);
            }

            menu = menu.push(btn);
        }

        container(scrollable(
            column![
                text("Components").size(18).font(iced::Font { weight: to_font_weight(FontWeight::SemiBold), ..Default::default() }),
                Space::new().width(Length::Shrink).height(Length::Fixed(16.0)),
                menu,
            ]
            .padding(24)
        ))
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(move |theme| {
            let is_dark = matches!(theme, iced::Theme::Dark);
            container::Style {
                border: iced::Border {
                    color: if is_dark { to_color(Color::gray(Scale::S800)) } else { to_color(Color::gray(Scale::S300)) },
                    width: 1.0,
                    ..Default::default()
                },
                ..container::Style::default()
            }
        })
        .into()
    }
}
