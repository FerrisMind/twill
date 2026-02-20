use iced::widget::{Space, button, column, container, image, row, scrollable, text};
use iced::{Background, Element, Length};

use crate::Message;
use crate::docs_index::{DocsNavEntry, docs_categories};
use crate::pages::Page;
use twill::iced::{to_color, to_font_weight};
use twill::tokens::{Color, FontWeight, Scale};

pub struct Sidebar;

impl Default for Sidebar {
    fn default() -> Self {
        Self
    }
}

impl Sidebar {
    fn twill_icon<'a>() -> Element<'a, Message> {
        let handle = image::Handle::from_bytes(include_bytes!("../../assets/icon.png").as_slice());

        image(handle)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .into()
    }

    fn nav_entry<'a>(
        entry: &'a DocsNavEntry,
        current_page: Page,
        nested: bool,
    ) -> Element<'a, Message> {
        let is_selected = entry.page == Some(current_page);
        let is_clickable = entry.page.is_some();

        let mut item = button(text(entry.title).size(if nested { 12 } else { 13 }).style(
            move |theme| {
                let is_dark = matches!(theme, iced::Theme::Dark);
                let color = if is_selected {
                    to_color(Color::blue(Scale::S500))
                } else if is_clickable {
                    if is_dark {
                        to_color(Color::gray(Scale::S200))
                    } else {
                        to_color(Color::gray(Scale::S800))
                    }
                } else {
                    to_color(Color::gray(Scale::S500))
                };

                iced::widget::text::Style { color: Some(color) }
            },
        ))
        .width(Length::Fill)
        .padding(if nested { [4, 10] } else { [6, 10] });

        item = if let Some(page) = entry.page {
            item.on_press(Message::PageSelected(page))
                .style(if is_selected {
                    iced::widget::button::secondary
                } else {
                    iced::widget::button::text
                })
        } else {
            item.style(iced::widget::button::text)
        };

        let mut content = column![item].spacing(2);

        if !entry.children.is_empty() {
            let mut children = column![].spacing(2);
            for child in entry.children {
                children = children.push(Self::nav_entry(child, current_page, true));
            }
            content = content.push(
                row![
                    Space::new()
                        .width(Length::Fixed(14.0))
                        .height(Length::Shrink),
                    children,
                ]
                .spacing(0),
            );
        }

        content.into()
    }

    pub fn view(&self, current_page: Page, is_mobile: bool) -> Element<'_, Message> {
        let mut content = column![].spacing(12);

        for category in docs_categories() {
            let mut entries = column![].spacing(3);
            for entry in category.entries {
                entries = entries.push(Self::nav_entry(entry, current_page, false));
            }

            content = content.push(
                column![
                    text(category.title.to_uppercase())
                        .size(11)
                        .font(iced::Font {
                            weight: to_font_weight(FontWeight::SemiBold),
                            ..Default::default()
                        }),
                    entries
                ]
                .spacing(4),
            );
        }

        let scroll = scrollable(
            column![
                row![
                    Self::twill_icon(),
                    text("Twill Showcase").size(18).font(iced::Font {
                        weight: to_font_weight(FontWeight::Bold),
                        ..Default::default()
                    }),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
                content
            ]
            .spacing(14)
            .padding(16),
        );

        container(scroll)
            .width(if is_mobile {
                Length::Fill
            } else {
                Length::Fixed(288.0)
            })
            .height(Length::Fill)
            .style(move |theme| {
                let is_dark = matches!(theme, iced::Theme::Dark);
                container::Style {
                    border: iced::Border {
                        color: if is_dark {
                            to_color(Color::gray(Scale::S800))
                        } else {
                            to_color(Color::gray(Scale::S300))
                        },
                        width: 1.0,
                        ..Default::default()
                    },
                    background: Some(if is_dark {
                        Background::Color(to_color(Color::gray(Scale::S950)))
                    } else {
                        Background::Color(to_color(Color::gray(Scale::S50)))
                    }),
                    ..container::Style::default()
                }
            })
            .into()
    }
}
