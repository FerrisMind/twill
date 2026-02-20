use iced::widget::{button, column, container, row, text, Space, scrollable};
use iced::{Alignment, Element, Length, Subscription, Theme};
use std::time::Instant;
use twill::tokens::{Color, Scale, Spacing, FontWeight};
use twill::iced::{to_color, to_font_weight, to_padding};

mod components;
mod pages;
mod sidebar;

use pages::Page;
use sidebar::Sidebar;

pub fn main() -> iced::Result {
    iced::application(Showcase::default, Showcase::update, Showcase::view)
        .title(Showcase::title)
        .theme(Showcase::theme)
        .subscription(Showcase::subscription)
        .run()
}

pub struct Showcase {
    sidebar: Sidebar,
    current_page: Page,
    is_dark: bool,
    start_time: Instant,
    elapsed: f64,
}

impl Default for Showcase {
    fn default() -> Self {
        Self {
            sidebar: Sidebar,
            current_page: Page::default(),
            is_dark: true,
            start_time: Instant::now(),
            elapsed: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PageSelected(Page),
    ToggleTheme,
    NoOp,
    Tick(Instant),
}

impl Showcase {
    fn title(&self) -> String {
        String::from("Twill Iced Showcase")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PageSelected(page) => {
                self.current_page = page;
            }
            Message::ToggleTheme => {
                self.is_dark = !self.is_dark;
            }
            Message::NoOp => {}
            Message::Tick(now) => {
                self.elapsed = now.duration_since(self.start_time).as_secs_f64();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let is_dark = self.is_dark;

        // Header
        let theme_text = if is_dark { "🌙 Dark Mode" } else { "☀️ Light Mode" };
        let header = container(
            row![
                text("Twill Iced Showcase").size(24).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
                Space::new().width(Length::Fill).height(Length::Shrink),
                button(theme_text).on_press(Message::ToggleTheme),
            ]
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(to_padding(Spacing::S4))
        .style(move |_theme| {
            container::Style {
                border: iced::Border {
                    color: if is_dark { to_color(Color::gray(Scale::S800)) } else { to_color(Color::gray(Scale::S300)) },
                    width: 1.0,
                    ..Default::default()
                },
                ..container::Style::default()
            }
        });

        // Main content area
        let content = row![
            self.sidebar.view(self.current_page.clone()),
            container(scrollable(self.current_page.view(is_dark, self.elapsed)))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(to_padding(Spacing::S8)),
        ];

        column![
            header,
            content,
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        // Only tick when on Motion page to save resources
        if self.current_page == Page::Motion {
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Tick(Instant::now()))
        } else {
            Subscription::none()
        }
    }
}
