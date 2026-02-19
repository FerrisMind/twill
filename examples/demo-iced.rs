//! Iced demo showcasing rustwind styling.

use iced::widget::{button, column, row, text, container};
use iced::{Element, Theme, Task, Background, Border};
use rustwind::{Color, Scale};
use rustwind::backends::to_color;

pub fn main() -> iced::Result {
    iced::application(DemoApp::new, DemoApp::update, DemoApp::view)
        .theme(DemoApp::theme)
        .centered()
        .run()
}

#[derive(Default)]
struct DemoApp {
    counter: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

impl DemoApp {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
            Message::Reset => self.counter = 0,
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Rustwind + Iced Demo")
            .size(28)
            .color(to_color(Color::white()));

        let description = text("Type-safe styling for Rust GUI applications")
            .size(16)
            .color(to_color(Color::slate(Scale::S400)));

        let counter_text = text(format!("Counter: {}", self.counter))
            .size(48)
            .color(to_color(Color::white()));

        // Card container
        let card = container(counter_text)
            .padding(24)
            .style(|_theme| {
                container::Style {
                    background: Some(Background::Color(to_color(Color::slate(Scale::S800)))),
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            });

        // Buttons using Iced's built-in styles
        let buttons = row![
            button("Increment")
                .on_press(Message::Increment)
                .style(button::primary)
                .padding(10),
            button("Decrement")
                .on_press(Message::Decrement)
                .style(button::secondary)
                .padding(10),
            button("Reset")
                .on_press(Message::Reset)
                .style(button::danger)
                .padding(10),
        ]
        .spacing(16);

        // Color palette
        let palette_title = text("Color Palette")
            .size(20)
            .color(to_color(Color::white()));

        let color_row1 = row![
            color_swatch(Color::blue(Scale::S100)),
            color_swatch(Color::blue(Scale::S300)),
            color_swatch(Color::blue(Scale::S500)),
            color_swatch(Color::blue(Scale::S700)),
            color_swatch(Color::blue(Scale::S900)),
        ]
        .spacing(8);

        let color_row2 = row![
            color_swatch(Color::red(Scale::S100)),
            color_swatch(Color::red(Scale::S300)),
            color_swatch(Color::red(Scale::S500)),
            color_swatch(Color::red(Scale::S700)),
            color_swatch(Color::red(Scale::S900)),
        ]
        .spacing(8);

        let color_row3 = row![
            color_swatch(Color::green(Scale::S100)),
            color_swatch(Color::green(Scale::S300)),
            color_swatch(Color::green(Scale::S500)),
            color_swatch(Color::green(Scale::S700)),
            color_swatch(Color::green(Scale::S900)),
        ]
        .spacing(8);

        let content = column![
            title,
            description,
            card,
            buttons,
            palette_title,
            color_row1,
            color_row2,
            color_row3,
        ]
        .spacing(24)
        .align_x(iced::Alignment::Center);

        container(content)
            .padding(32)
            .style(|_theme| {
                container::Style {
                    background: Some(Background::Color(to_color(Color::slate(Scale::S900)))),
                    ..Default::default()
                }
            })
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn color_swatch(color: Color) -> Element<'static, Message> {
    container(text("").size(20))
        .width(40)
        .height(40)
        .style(move |_theme| {
            container::Style {
                background: Some(Background::Color(to_color(color))),
                border: Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}