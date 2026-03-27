#[cfg(feature = "iced")]
mod common;

#[cfg(feature = "iced")]
use common::{semantic_summary, showcase_sections, token_palette};
#[cfg(feature = "iced")]
use iced::widget::{button, column, container, row, text};
#[cfg(feature = "iced")]
use iced::{Element, Fill, Theme};
#[cfg(feature = "iced")]
use twill::backends::iced::{self as twill_iced, styled_container};
#[cfg(feature = "iced")]
use twill::prelude::*;

#[cfg(feature = "iced")]
fn main() -> iced::Result {
    iced::application(ShowcaseApp::default, ShowcaseApp::update, ShowcaseApp::view)
        .title(ShowcaseApp::title)
        .theme(ShowcaseApp::theme)
        .run()
}

#[cfg(feature = "iced")]
#[derive(Default)]
struct ShowcaseApp {
    dark_mode: bool,
}

#[cfg(feature = "iced")]
#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleTheme,
}

#[cfg(feature = "iced")]
impl ShowcaseApp {
    fn title(&self) -> String {
        String::from("twill showcase (iced)")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
            }
        }
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::TokyoNight
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let topbar = row![
            text("Twill 0.3.x layered examples").size(28),
            button("Toggle semantic theme").on_press(Message::ToggleTheme),
        ]
        .spacing(16);

        let tokens = row(token_palette()
            .into_iter()
            .map(|(color, label)| {
                let style = Style::new()
                    .bg(color)
                    .text_color(Color::white())
                    .padding(Padding::all(Spacing::S3))
                    .rounded(BorderRadius::Lg);
                styled_container(text(label).into(), &style).into()
            })
            .collect::<Vec<Element<'_, Message>>>())
        .spacing(12);

        let variant = if self.dark_mode {
            ThemeVariant::Dark
        } else {
            ThemeVariant::Light
        };
        let semantic_column = column(
            semantic_summary()
                .into_iter()
                .map(|(label, token)| {
                    let color = twill_iced::to_semantic_color(token, variant);
                    let style = Style::new()
                        .bg(Color::slate(Scale::S50))
                        .text_color(Color::slate(Scale::S900))
                        .padding(Padding::all(Spacing::S2))
                        .rounded(BorderRadius::Md)
                        .border(
                            BorderWidth::S1,
                            BorderStyle::Solid,
                            Color::slate(Scale::S200),
                        );
                    styled_container(
                        row![text(label), text(format!("{color:?}"))]
                            .spacing(8)
                            .into(),
                        &style,
                    )
                    .into()
                })
                .collect::<Vec<Element<'_, Message>>>(),
        )
        .spacing(8);

        let sections = column(
            showcase_sections()
                .into_iter()
                .map(|section| {
                    let mut body =
                        column![text(section.title).size(22), text(section.description)].spacing(8);
                    if section.title == "Responsive" {
                        let resolved = section.style.at_breakpoint(Breakpoint::Lg);
                        body = body.push(text(format!(
                            "Lg -> width={:?}, padding={:?}",
                            resolved.width_value(),
                            resolved.padding_value()
                        )));
                    }
                    styled_container(body.into(), &section.style).into()
                })
                .collect::<Vec<Element<'_, Message>>>(),
        )
        .spacing(12);

        container(
            column![
                topbar,
                text("Tokens"),
                tokens,
                text("Semantic theme"),
                semantic_column,
                text("Composed sections"),
                sections
            ]
            .spacing(16),
        )
        .padding(24)
        .width(Fill)
        .into()
    }
}

#[cfg(not(feature = "iced"))]
fn main() {}
