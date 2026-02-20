//! Iced demo showcasing twill capabilities via a visually identical component showcase.

use iced::widget::{button, column, container, row, text, Space, image};
use iced::{alignment, Background, Border, Element, Length, Task, Theme};
use twill::backends::iced::{to_color, to_color_value, to_shadow};
use twill::{Color, Scale, SemanticColor, SemanticThemeVars, Shadow};

pub fn main() -> iced::Result {
    iced::application(DemoApp::default, DemoApp::update, DemoApp::view)
        .title("Twill - Universal Design Showcase (Iced)")
        .theme(DemoApp::theme)
        .window_size(iced::Size::new(800.0, 600.0))
        .run()
}

struct DemoApp {
    dark_mode: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleTheme,
    ActionPressed,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

impl DemoApp {

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ToggleTheme => self.dark_mode = !self.dark_mode,
            Message::ActionPressed => {}
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = SemanticThemeVars::shadcn_neutral();
        let bg = theme
            .resolve(SemanticColor::Background, self.dark_mode)
            .unwrap_or(Color::gray(Scale::S50));
        let fg = theme
            .resolve(SemanticColor::Foreground, self.dark_mode)
            .unwrap_or(Color::gray(Scale::S900));
        let muted_fg = theme
            .resolve(SemanticColor::MutedForeground, self.dark_mode)
            .unwrap();

        let header = column![
            row![
                image("assets/icon.png").width(Length::Fixed(48.0)).height(Length::Fixed(48.0)),
                text("Twill Universal Design")
                    .size(32)
                    .color(to_color(fg))
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
            ]
            .spacing(16)
            .align_y(alignment::Vertical::Center),
            text("Rendered exactly the same across Egui, Iced, and Slint.")
                .size(16)
                .color(to_color(muted_fg))
        ]
        .spacing(12)
        .align_x(alignment::Horizontal::Center);

        let primary_bg = to_color(theme.resolve(SemanticColor::Primary, self.dark_mode).unwrap());
        let primary_fg = to_color(theme.resolve(SemanticColor::PrimaryForeground, self.dark_mode).unwrap());
        let secondary_bg = to_color(theme.resolve(SemanticColor::Secondary, self.dark_mode).unwrap());
        let secondary_fg = to_color(theme.resolve(SemanticColor::SecondaryForeground, self.dark_mode).unwrap());
        let destructive_bg = to_color(theme.resolve(SemanticColor::Destructive, self.dark_mode).unwrap());
        let destructive_fg = to_color(Color::slate(Scale::S50));
        let fg_color = to_color(fg);
        let border_c = to_color(theme.resolve(SemanticColor::Border, self.dark_mode).unwrap());
        let transparent = to_color_value(twill::tokens::ColorValue::TRANSPARENT);

        let mut primary_hover = primary_bg;
        primary_hover.a = 0.9;
        let mut secondary_hover = secondary_bg;
        secondary_hover.a = 0.8;
        let mut destructive_hover = destructive_bg;
        destructive_hover.a = 0.9;

        let toggle_btn = styled_button(
            if self.dark_mode { "Switch to Light Mode" } else { "Switch to Dark Mode" },
            secondary_bg, secondary_fg, 0.0, transparent, 
            Some(secondary_hover), Some(secondary_fg),
            Message::ToggleTheme
        );

        let semantic_label = text("Semantic Palette (shadcn-like)")
            .size(14)
            .color(to_color(muted_fg));

        let buttons_row = row![
            styled_button("Primary", primary_bg, primary_fg, 0.0, transparent, Some(primary_hover), Some(primary_fg), Message::ActionPressed),
            styled_button("Secondary", secondary_bg, secondary_fg, 0.0, transparent, Some(secondary_hover), Some(secondary_fg), Message::ActionPressed),
            styled_button("Outline", transparent, fg_color, 1.0, border_c, Some(to_color(theme.resolve(SemanticColor::Accent, self.dark_mode).unwrap())), Some(to_color(theme.resolve(SemanticColor::AccentForeground, self.dark_mode).unwrap())), Message::ActionPressed),
            styled_button("Ghost", transparent, fg_color, 0.0, transparent, Some(to_color(theme.resolve(SemanticColor::Accent, self.dark_mode).unwrap())), Some(to_color(theme.resolve(SemanticColor::AccentForeground, self.dark_mode).unwrap())), Message::ActionPressed),
            styled_button("Destructive", destructive_bg, destructive_fg, 0.0, transparent, Some(destructive_hover), Some(destructive_fg), Message::ActionPressed),
        ]
        .spacing(12)
        .align_y(alignment::Vertical::Center);

        let tailwind_label = text("Tailwind Palette (direct colors)")
            .size(14)
            .color(to_color(muted_fg));

        let tw_blue = to_color(Color::blue(Scale::S500));
        let tw_blue_hover = to_color(Color::blue(Scale::S600));
        
        let tw_slate = to_color(if self.dark_mode { Color::slate(Scale::S800) } else { Color::slate(Scale::S100) });
        let tw_slate_hover = to_color(if self.dark_mode { Color::slate(Scale::S700) } else { Color::slate(Scale::S200) });
        let tw_slate_fg = to_color(if self.dark_mode { Color::slate(Scale::S50) } else { Color::slate(Scale::S900) });

        let tw_border = to_color(if self.dark_mode { Color::slate(Scale::S700) } else { Color::slate(Scale::S200) });

        let tw_red = to_color(Color::red(Scale::S500));
        let tw_red_hover = to_color(Color::red(Scale::S600));
        
        let tw_white = to_color(Color::white());
        let tw_black = to_color(if self.dark_mode { Color::slate(Scale::S50) } else { Color::slate(Scale::S900) });

        let tailwind_row = row![
            styled_button("Blue", tw_blue, tw_white, 0.0, transparent, Some(tw_blue_hover), Some(tw_white), Message::ActionPressed),
            styled_button("Slate", tw_slate, tw_slate_fg, 0.0, transparent, Some(tw_slate_hover), Some(tw_slate_fg), Message::ActionPressed),
            styled_button("Outline", transparent, tw_black, 1.0, tw_border, Some(tw_slate), Some(tw_slate_fg), Message::ActionPressed),
            styled_button("Ghost", transparent, tw_black, 0.0, transparent, Some(tw_slate), Some(tw_slate_fg), Message::ActionPressed),
            styled_button("Rose", tw_red, tw_white, 0.0, transparent, Some(tw_red_hover), Some(tw_white), Message::ActionPressed),
        ]
        .spacing(12)
        .align_y(alignment::Vertical::Center);

        let card_bg = theme.resolve(SemanticColor::Card, self.dark_mode).unwrap();
        let card_border = theme
            .resolve(SemanticColor::Border, self.dark_mode)
            .unwrap();
        
        let card_content = column![
            Space::new().height(Length::Fixed(8.0)),
            text("Interactive Card")
                .size(24)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(to_color(fg)),
            Space::new().height(Length::Fixed(12.0)),
            text("This card is styled entirely with Twill tokens and its appearance is mathematically mapped to Iced elements.")
                .size(15)
                .color(to_color(muted_fg))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
            Space::new().height(Length::Fixed(32.0)),
            styled_button("Action Button", primary_bg, primary_fg, 0.0, transparent, Some(primary_hover), Some(primary_fg), Message::ActionPressed),
            Space::new().height(Length::Fixed(8.0)),
        ]
        .spacing(0)
        .align_x(alignment::Horizontal::Center);

        let card = container(card_content)
            .width(Length::Fixed(420.0))
            .padding(24)
            .style(move |_| container::Style {
                background: Some(Background::Color(to_color(card_bg))),
                border: Border {
                    radius: 12.0.into(),
                    width: 1.0,
                    color: to_color(card_border),
                },
                shadow: if self.dark_mode { iced::Shadow::default() } else { to_shadow(Shadow::Lg, Color::black()) },
                ..Default::default()
            });

        let content = column![
            Space::new().height(Length::Fixed(48.0)),
            header,
            Space::new().height(Length::Fixed(32.0)),
            toggle_btn,
            Space::new().height(Length::Fixed(48.0)),
            semantic_label,
            Space::new().height(Length::Fixed(12.0)),
            buttons_row,
            Space::new().height(Length::Fixed(32.0)),
            tailwind_label,
            Space::new().height(Length::Fixed(12.0)),
            tailwind_row,
            Space::new().height(Length::Fixed(48.0)),
            card
        ]
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(to_color(bg))),
                ..Default::default()
            })
            .into()
    }
}

#[allow(clippy::too_many_arguments)]
fn styled_button<'a>(
    label: &'a str,
    bg: iced::Color,
    fg: iced::Color,
    border_width: f32,
    border_color: iced::Color,
    hover_bg: Option<iced::Color>,
    hover_fg: Option<iced::Color>,
    msg: Message,
) -> Element<'a, Message> {
    button(
        text(label)
            .color(fg)
            .size(14)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center),
    )
    .padding([8, 16])
    .height(Length::Fixed(36.0))
    .on_press(msg)
    .style(move |_theme, status| {
        let is_ghost_or_outline = bg == iced::Color::TRANSPARENT;
        let mut current_bg = bg;
        let mut current_fg = fg;

        match status {
            button::Status::Hovered => {
                if let Some(hbg) = hover_bg {
                    current_bg = hbg;
                }
                if let Some(hfg) = hover_fg {
                    current_fg = hfg;
                }
            }
            button::Status::Pressed => {
                if let Some(mut hbg) = hover_bg {
                    if !is_ghost_or_outline {
                        hbg.a *= 0.8;
                    } else {
                        hbg.a *= 1.2;
                    }
                    current_bg = hbg;
                }
                if let Some(hfg) = hover_fg {
                    current_fg = hfg;
                }
            }
            _ => {}
        }

        button::Style {
            background: Some(Background::Color(current_bg)),
            text_color: current_fg,
            border: Border {
                radius: 6.0.into(),
                width: border_width,
                color: border_color,
            },
            ..button::Style::default()
        }
    })
    .into()
}

