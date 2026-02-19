//! Iced demo showcasing rustwind styling.

use iced::widget::{button, column, row, text, container};
use iced::{Element, Length, Theme};
use rustwind::{Color, Scale, Spacing, Padding, BorderRadius, Style};
use rustwind::backends::{to_color, primary_button, secondary_button, danger_button, styled_container};

fn main() -> iced::Result {
    iced::run("Rustwind - Iced Demo", DemoApp::update, DemoApp::view)
        .theme(DemoApp::theme)
}

#[derive(Default)]
struct DemoApp {
    counter: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

impl DemoApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
            Message::Reset => self.counter = 0,
        }
    }
    
    fn view(&self) -> Element<Message> {
        // Title
        let title = text("Rustwind + Iced Demo")
            .size(28)
            .color(to_color(Color::white()));
        
        // Description
        let description = text("Type-safe styling for Rust GUI applications")
            .size(16)
            .color(to_color(Color::slate(Scale::S400)));
        
        // Counter display
        let counter_text = text(format!("Counter: {}", self.counter))
            .size(48)
            .color(to_color(Color::white()));
        
        // Buttons row
        let buttons = row![
            primary_button("Increment", Message::Increment),
            secondary_button("Decrement", Message::Decrement),
            danger_button("Reset", Message::Reset),
        ]
        .spacing(16);
        
        // Color palette section
        let palette_title = text("Color Palette")
            .size(20)
            .color(to_color(Color::white()));
        
        let color_row = row![
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
        
        // Card container for counter
        let card_style = Style::new()
            .padding(Padding::all(Spacing::S6))
            .bg(Color::slate(Scale::S800));
        
        let card = container(counter_text)
            .padding(24)
            .style(move |_| {
                container::Style::default()
                    .background(iced::Background::Color(to_color(Color::slate(Scale::S800))))
                    .border_radius(8.0)
            });
        
        // Main content
        let content = column![
            title,
            description,
            card,
            buttons,
            palette_title,
            color_row,
            color_row2,
            color_row3,
        ]
        .spacing(24)
        .align_x(iced::Alignment::Center);
        
        // Main container with dark background
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(32)
            .style(|_| {
                container::Style::default()
                    .background(iced::Background::Color(to_color(Color::slate(Scale::S900))))
            })
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
    
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

/// Create a color swatch widget.
fn color_swatch(color: Color) -> Element<'static, Message> {
    container(text("  ").size(20))
        .width(40)
        .height(40)
        .style(move |_| {
            container::Style::default()
                .background(iced::Background::Color(to_color(color)))
                .border_radius(6.0)
        })
        .into()
}