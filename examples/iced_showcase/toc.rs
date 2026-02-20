use iced::widget::{button, column, container, text};
use iced::{Background, Element, Length};

use crate::Message;
use crate::docs_index::toc_for;
use crate::pages::Page;
use twill::iced::{to_color, to_font_weight};
use twill::tokens::{Color, FontWeight, Scale};

pub struct TableOfContents;

impl TableOfContents {
    pub fn view(&self, page: Page, compact: bool) -> Element<'_, Message> {
        let mut content = column![
            text("ON THIS PAGE").size(12).font(iced::Font {
                weight: to_font_weight(FontWeight::SemiBold),
                ..Default::default()
            }),
            text("Jump between key sections").size(12)
        ]
        .spacing(8);

        for (index, entry) in toc_for(page).iter().enumerate() {
            let button = button(text(entry.title))
                .width(Length::Fill)
                .padding([6, 8])
                .style(iced::widget::button::text)
                .on_press(Message::TocJump(index));

            content = content.push(button);
        }

        container(content.padding(12))
            .width(if compact {
                Length::Fill
            } else {
                Length::Fixed(256.0)
            })
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
