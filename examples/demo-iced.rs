//! Iced demo showcasing ALL rustwind capabilities.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Background, Border, Element, Length, Shadow as IcedShadow, Task, Theme, Vector};
use rustwind::backends::to_color;
use rustwind::{
    BorderRadius, Color, FontSize, FontWeight, Padding, Scale, Shadow, Spacing, Style, ToCss,
};

type PaletteFn = fn(Scale) -> Color;

pub fn main() -> iced::Result {
    iced::application(DemoApp::new, DemoApp::update, DemoApp::view)
        .theme(DemoApp::theme)
        .centered()
        .run()
}

struct DemoApp {
    dark_mode: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleTheme,
}

impl DemoApp {
    fn new() -> (Self, Task<Message>) {
        (Self { dark_mode: true }, Task::none())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ToggleTheme => self.dark_mode = !self.dark_mode,
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
        let app_bg = if self.dark_mode {
            Color::gray(Scale::S950)
        } else {
            Color::gray(Scale::S50)
        };
        let heading_color = if self.dark_mode {
            Color::white()
        } else {
            Color::slate(Scale::S900)
        };
        let palettes: [(&str, PaletteFn); 22] = [
            ("Slate", Color::slate),
            ("Gray", Color::gray),
            ("Zinc", Color::zinc),
            ("Neutral", Color::neutral),
            ("Stone", Color::stone),
            ("Red", Color::red),
            ("Orange", Color::orange),
            ("Amber", Color::amber),
            ("Yellow", Color::yellow),
            ("Lime", Color::lime),
            ("Green", Color::green),
            ("Emerald", Color::emerald),
            ("Teal", Color::teal),
            ("Cyan", Color::cyan),
            ("Sky", Color::sky),
            ("Blue", Color::blue),
            ("Indigo", Color::indigo),
            ("Violet", Color::violet),
            ("Purple", Color::purple),
            ("Fuchsia", Color::fuchsia),
            ("Pink", Color::pink),
            ("Rose", Color::rose),
        ];

        let mut color_sections = Vec::new();
        for (name, color_fn) in palettes {
            let label = text(name)
                .size(12)
                .color(to_color(Color::slate(Scale::S400)));
            let swatches: Vec<Element<Message>> = [
                Scale::S50,
                Scale::S100,
                Scale::S200,
                Scale::S300,
                Scale::S400,
                Scale::S500,
                Scale::S600,
                Scale::S700,
                Scale::S800,
                Scale::S900,
                Scale::S950,
            ]
            .iter()
            .map(|&s| color_swatch(color_fn(s)))
            .collect();
            color_sections.push(column![label, row(swatches).spacing(4).wrap()].spacing(4));
        }

        let spacings = [
            (Spacing::S0, "0"),
            (Spacing::S1, "1"),
            (Spacing::S2, "2"),
            (Spacing::S3, "3"),
            (Spacing::S4, "4"),
            (Spacing::S5, "5"),
            (Spacing::S6, "6"),
            (Spacing::S8, "8"),
            (Spacing::S10, "10"),
            (Spacing::S12, "12"),
            (Spacing::S16, "16"),
            (Spacing::S20, "20"),
            (Spacing::S24, "24"),
            (Spacing::S32, "32"),
            (Spacing::S40, "40"),
            (Spacing::S48, "48"),
            (Spacing::S56, "56"),
            (Spacing::S64, "64"),
            (Spacing::S72, "72"),
            (Spacing::S80, "80"),
            (Spacing::S96, "96"),
            (Spacing::Px, "px"),
            (Spacing::Auto, "auto"),
        ];
        let spacing_cards: Vec<Element<Message>> = spacings
            .iter()
            .map(|(spacing, label)| spacing_card(*spacing, label))
            .collect();

        let radii = [
            (BorderRadius::None, "none"),
            (BorderRadius::Xs, "xs"),
            (BorderRadius::Sm, "sm"),
            (BorderRadius::Md, "md"),
            (BorderRadius::Lg, "lg"),
            (BorderRadius::Xl, "xl"),
            (BorderRadius::S2xl, "2xl"),
            (BorderRadius::S3xl, "3xl"),
            (BorderRadius::S4xl, "4xl"),
            (BorderRadius::Full, "full"),
        ];
        let radius_cards: Vec<Element<Message>> = radii
            .iter()
            .map(|(radius, name)| radius_box(radius_to_px(*radius), name))
            .collect();

        let font_items: Vec<Element<Message>> = [
            (FontSize::Xs, "xs"),
            (FontSize::Sm, "sm"),
            (FontSize::Base, "base"),
            (FontSize::Lg, "lg"),
            (FontSize::Xl, "xl"),
            (FontSize::S2xl, "2xl"),
            (FontSize::S3xl, "3xl"),
            (FontSize::S4xl, "4xl"),
            (FontSize::S5xl, "5xl"),
        ]
        .iter()
        .map(|(s, n)| {
            let px = s.size_rem() * 16.0;
            text(format!("{n} ({px:.0}px)"))
                .size(px.min(42.0))
                .color(to_color(Color::slate(Scale::S200)))
                .into()
        })
        .collect();

        let weight_cards: Vec<Element<Message>> = [
            (FontWeight::Thin, "thin"),
            (FontWeight::ExtraLight, "extralight"),
            (FontWeight::Light, "light"),
            (FontWeight::Normal, "normal"),
            (FontWeight::Medium, "medium"),
            (FontWeight::SemiBold, "semibold"),
            (FontWeight::Bold, "bold"),
            (FontWeight::ExtraBold, "extrabold"),
            (FontWeight::Black, "black"),
        ]
        .iter()
        .enumerate()
        .map(|(idx, (_weight, name))| weight_chip(name, idx))
        .collect();

        let shadow_cards: Vec<Element<Message>> = [
            (Shadow::Sm, "sm", 2.0, 8.0, 0.20),
            (Shadow::Md, "md", 4.0, 12.0, 0.26),
            (Shadow::Lg, "lg", 6.0, 16.0, 0.30),
            (Shadow::Xl, "xl", 8.0, 22.0, 0.34),
            (Shadow::S2xl, "2xl", 10.0, 28.0, 0.40),
        ]
        .iter()
        .map(|(token, name, y, blur, alpha)| shadow_card(token, name, *y, *blur, *alpha))
        .collect();

        let buttons = row![
            button("Primary").style(button::primary).padding(8),
            button("Secondary").style(button::secondary).padding(8),
            button("Destructive").style(button::danger).padding(8),
        ]
        .spacing(12)
        .wrap();
        let button_sizes = row![
            button("Small").style(button::primary).padding([6, 10]),
            button("Medium").style(button::primary).padding([10, 14]),
            button("Large").style(button::primary).padding([14, 18]),
        ]
        .spacing(10)
        .wrap();

        let card_style = Style::new()
            .padding(Padding::all(Spacing::S6))
            .bg(Color::slate(Scale::S800))
            .rounded(BorderRadius::Lg);
        let flex_style = Style::centered_col().gap(Spacing::S4);

        let style_builder = column![
            container(
                column![
                    text("Card Component")
                        .size(18)
                        .color(to_color(Color::white())),
                    text("Style::new().padding().bg().rounded()")
                        .size(12)
                        .color(to_color(Color::slate(Scale::S400))),
                    text(card_style.to_css())
                        .size(11)
                        .color(to_color(Color::green(Scale::S400))),
                ]
                .spacing(4)
            )
            .padding(16)
            .style(|_| container::Style {
                background: Some(Background::Color(to_color(Color::slate(Scale::S800)))),
                border: Border {
                    radius: 10.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            container(
                column![
                    text("Centered Column")
                        .size(16)
                        .color(to_color(Color::white())),
                    text("Style::centered_col().gap(S4)")
                        .size(11)
                        .color(to_color(Color::slate(Scale::S400))),
                    text(flex_style.to_css())
                        .size(11)
                        .color(to_color(Color::green(Scale::S400))),
                ]
                .spacing(4)
            )
            .padding(16)
            .style(|_| container::Style {
                background: Some(Background::Color(to_color(Color::slate(Scale::S800)))),
                border: Border {
                    radius: 10.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        ]
        .spacing(10);

        let css_style = Style::new()
            .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
            .bg(Color::blue(Scale::S500))
            .rounded(BorderRadius::Md);
        let css_output = container(
            text(css_style.to_css())
                .size(12)
                .color(to_color(Color::green(Scale::S400))),
        )
        .padding(12)
        .style(|_| container::Style {
            background: Some(Background::Color(to_color(Color::slate(Scale::S950)))),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

        let mut content = column![
            button(if self.dark_mode {
                "Switch to Light (gray-50)"
            } else {
                "Switch to Dark (gray-950)"
            })
            .on_press(Message::ToggleTheme)
            .padding(8),
            text("Rustwind - Complete Feature Demo")
                .size(32)
                .color(to_color(heading_color)),
            text("Iced showcase aligned with egui demo")
                .size(14)
                .color(to_color(Color::slate(Scale::S300))),
            text(""),
            section_title(
                "All Color Palettes (22 palettes × 11 shades)",
                heading_color
            ),
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center);

        for section in color_sections {
            content = content.push(section);
        }

        content = content
            .push(text("").height(24))
            .push(section_title("Spacing Scale (0-96, px)", heading_color))
            .push(row(spacing_cards).spacing(8).wrap())
            .push(text("").height(24))
            .push(section_title("Border Radius", heading_color))
            .push(row(radius_cards).spacing(8).wrap())
            .push(text("").height(24))
            .push(section_title("Typography - Font Sizes", heading_color))
            .push(column(font_items).spacing(4))
            .push(text("").height(16))
            .push(section_title("Typography - Font Weights", heading_color))
            .push(row(weight_cards).spacing(8).wrap())
            .push(text("").height(24))
            .push(section_title("Box Shadows", heading_color))
            .push(row(shadow_cards).spacing(12).wrap())
            .push(text("").height(24))
            .push(section_title("Button Variants", heading_color))
            .push(buttons)
            .push(text("").height(16))
            .push(section_title("Button Sizes", heading_color))
            .push(button_sizes)
            .push(text("").height(24))
            .push(section_title("Style Builder Examples", heading_color))
            .push(style_builder)
            .push(text("").height(24))
            .push(section_title("CSS Output Example", heading_color))
            .push(css_output);

        scrollable(
            container(content.padding(32))
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(to_color(app_bg))),
                    ..Default::default()
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn section_title(label: &'static str, color: Color) -> Element<'static, Message> {
    text(label).size(18).color(to_color(color)).into()
}

fn color_swatch(color: Color) -> Element<'static, Message> {
    container(text("").size(10))
        .width(24)
        .height(24)
        .style(move |_| container::Style {
            background: Some(Background::Color(to_color(color))),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn spacing_card(spacing: Spacing, label: &'static str) -> Element<'static, Message> {
    let px = spacing.to_rem().unwrap_or(0.0) * 16.0;
    let bar_width = px.clamp(2.0, 92.0);
    container(
        row![
            text(label)
                .size(10)
                .color(to_color(Color::slate(Scale::S300))),
            container(text(""))
                .width(bar_width)
                .height(10)
                .style(|_| container::Style {
                    background: Some(Background::Color(to_color(Color::blue(Scale::S500)))),
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
    )
    .padding([6, 8])
    .width(Length::Shrink)
    .style(|_| container::Style {
        background: Some(Background::Color(to_color(Color::slate(Scale::S800)))),
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn radius_to_px(radius: BorderRadius) -> f32 {
    match radius {
        BorderRadius::None => 0.0,
        BorderRadius::Xs => 2.0,
        BorderRadius::Sm => 4.0,
        BorderRadius::Md => 6.0,
        BorderRadius::Lg => 8.0,
        BorderRadius::Xl => 12.0,
        BorderRadius::S2xl => 16.0,
        BorderRadius::S3xl => 24.0,
        BorderRadius::S4xl => 32.0,
        BorderRadius::Full => 9999.0,
    }
}

fn radius_box(radius: f32, name: &'static str) -> Element<'static, Message> {
    container(text(name).size(10).color(to_color(Color::white())))
        .padding(12)
        .style(move |_| container::Style {
            background: Some(Background::Color(to_color(Color::violet(Scale::S500)))),
            border: Border {
                radius: radius.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn weight_chip(name: &'static str, idx: usize) -> Element<'static, Message> {
    let tone = match idx {
        0 => Scale::S900,
        1 => Scale::S800,
        2 => Scale::S700,
        3 => Scale::S600,
        4 => Scale::S500,
        5 => Scale::S400,
        6 => Scale::S300,
        7 => Scale::S200,
        _ => Scale::S100,
    };
    let text_color = if idx <= 4 {
        Color::white()
    } else {
        Color::slate(Scale::S900)
    };
    container(text(name).size(12).color(to_color(text_color)))
        .padding([6, 10])
        .style(move |_| container::Style {
            background: Some(Background::Color(to_color(Color::slate(tone)))),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn shadow_card(
    token: &Shadow,
    label: &'static str,
    offset_y: f32,
    blur: f32,
    alpha: f32,
) -> Element<'static, Message> {
    let token_label = format!("{token:?}");
    container(
        column![
            text(label)
                .size(14)
                .color(to_color(Color::slate(Scale::S700))),
            text(token_label)
                .size(10)
                .color(to_color(Color::slate(Scale::S500))),
        ]
        .spacing(2)
        .align_x(iced::Alignment::Center),
    )
    .padding(18)
    .style(move |_| container::Style {
        background: Some(Background::Color(to_color(Color::white()))),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        shadow: IcedShadow {
            color: iced::Color {
                a: alpha,
                ..iced::Color::BLACK
            },
            offset: Vector::new(0.0, offset_y),
            blur_radius: blur,
        },
        ..Default::default()
    })
    .into()
}
