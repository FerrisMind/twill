//! Iced demo showcasing rustwind capabilities via tabs.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Background, Border, Element, Length, Task, Theme};
use rustwind::backends::to_color;
use rustwind::{
    AnimationToken, BorderRadius, BorderWidth, Button, ButtonSize, ButtonVariant, Color, Easing,
    FontSize, FontWeight, Scale, SemanticColor, SemanticThemeVars, Shadow, Spacing, Style, ToCss,
    TransitionDuration,
};

type PaletteFn = fn(Scale) -> Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Tokens,
    Typography,
    Components,
    Motion,
    Semantic,
}

impl Tab {
    fn all() -> &'static [(Tab, &'static str)] {
        &[
            (Tab::Tokens, "Tokens"),
            (Tab::Typography, "Typography"),
            (Tab::Components, "Components"),
            (Tab::Motion, "Motion"),
            (Tab::Semantic, "Semantic"),
        ]
    }
}

struct DemoApp {
    dark_mode: bool,
    tab: Tab,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleTheme,
    SetTab(Tab),
}

pub fn main() -> iced::Result {
    iced::application(DemoApp::new, DemoApp::update, DemoApp::view)
        .theme(DemoApp::theme)
        .run()
}

impl DemoApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                dark_mode: true,
                tab: Tab::Tokens,
            },
            Task::none(),
        )
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ToggleTheme => self.dark_mode = !self.dark_mode,
            Message::SetTab(tab) => self.tab = tab,
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
        let fg = if self.dark_mode {
            Color::gray(Scale::S50)
        } else {
            Color::gray(Scale::S900)
        };

        let mut tabs = row![];
        for (tab, label) in Tab::all() {
            let active = self.tab == *tab;
            let btn = if active {
                Button::primary()
            } else {
                Button::secondary().sm()
            };
            tabs = tabs.push(styled_button(label, btn, Message::SetTab(*tab)));
        }

        let content = column![
            row![
                styled_button(if self.dark_mode {
                    "Switch to Light (gray-50)"
                } else {
                    "Switch to Dark (gray-950)"
                }, Button::primary(), Message::ToggleTheme),
                tabs.spacing(6)
            ]
            .spacing(10),
            text("Rustwind - Iced Demo").size(30).color(to_color(fg)),
            render_tab(self.tab, fg)
        ]
        .spacing(10)
        .padding(24);

        scrollable(
            container(content).width(Length::Fill).style(move |_| container::Style {
                background: Some(Background::Color(to_color(app_bg))),
                ..Default::default()
            }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn render_tab(tab: Tab, fg: Color) -> Element<'static, Message> {
    match tab {
        Tab::Tokens => render_tokens(fg),
        Tab::Typography => render_typography(fg),
        Tab::Components => render_components(fg),
        Tab::Motion => render_motion(fg),
        Tab::Semantic => render_semantic(fg),
    }
}

fn section(title: &'static str, fg: Color) -> Element<'static, Message> {
    text(title).size(18).color(to_color(fg)).into()
}

fn render_tokens(fg: Color) -> Element<'static, Message> {
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

    let mut color_rows = column![];
    for (name, f) in palettes {
        let swatches = [
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
        .fold(row![], |r, s| r.push(swatch(f(*s))))
        .spacing(4)
        .wrap();
        color_rows = color_rows.push(column![text(name).color(to_color(fg)), swatches].spacing(4));
    }

    let spacing_list = [
        Spacing::S0,
        Spacing::S1,
        Spacing::S2,
        Spacing::S3,
        Spacing::S4,
        Spacing::S5,
        Spacing::S6,
        Spacing::S8,
        Spacing::S10,
        Spacing::S12,
        Spacing::S16,
        Spacing::S20,
        Spacing::S24,
        Spacing::S32,
        Spacing::S40,
        Spacing::S48,
        Spacing::S56,
        Spacing::S64,
        Spacing::S72,
        Spacing::S80,
        Spacing::S96,
        Spacing::Px,
        Spacing::Auto,
    ]
    .iter()
    .fold(row![], |r, s| r.push(text(s.to_css()).size(11)))
    .spacing(8)
    .wrap();

    column![
        section("Tokens: Colors", fg),
        color_rows.spacing(6),
        section("Spacing", fg),
        spacing_list,
        section("Radius + Shadow", fg),
        text(format!(
            "radius: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            BorderRadius::None,
            BorderRadius::Xs,
            BorderRadius::Sm,
            BorderRadius::Md,
            BorderRadius::Lg,
            BorderRadius::Xl,
            BorderRadius::S2xl,
            BorderRadius::S3xl,
            BorderRadius::S4xl,
            BorderRadius::Full
        )),
        text(format!(
            "shadow: {}, {}, {}, {}, {}, {}, {}, {}",
            Shadow::Xs2.to_css(),
            Shadow::Xs.to_css(),
            Shadow::Sm.to_css(),
            Shadow::Md.to_css(),
            Shadow::Lg.to_css(),
            Shadow::Xl.to_css(),
            Shadow::S2xl.to_css(),
            Shadow::None.to_css()
        )),
    ]
    .spacing(8)
    .into()
}

fn render_typography(fg: Color) -> Element<'static, Message> {
    let mut col = column![section("Font Sizes", fg)].spacing(6);
    for (name, size) in [
        ("xs", FontSize::Xs),
        ("sm", FontSize::Sm),
        ("base", FontSize::Base),
        ("lg", FontSize::Lg),
        ("xl", FontSize::Xl),
        ("2xl", FontSize::S2xl),
        ("3xl", FontSize::S3xl),
        ("4xl", FontSize::S4xl),
        ("5xl", FontSize::S5xl),
        ("6xl", FontSize::S6xl),
        ("7xl", FontSize::S7xl),
        ("8xl", FontSize::S8xl),
        ("9xl", FontSize::S9xl),
    ] {
        col = col.push(text(format!("{name}: {}", size.to_css())).color(to_color(fg)));
    }
    col = col.push(section("Font Weights", fg));
    for weight in [
        FontWeight::Thin,
        FontWeight::ExtraLight,
        FontWeight::Light,
        FontWeight::Normal,
        FontWeight::Medium,
        FontWeight::SemiBold,
        FontWeight::Bold,
        FontWeight::ExtraBold,
        FontWeight::Black,
    ] {
        col = col.push(text(format!("{weight:?}: {}", weight.value())).color(to_color(fg)));
    }
    col.into()
}

fn render_components(fg: Color) -> Element<'static, Message> {
    column![
        section("Button API", fg),
        text(Button::primary().to_css()),
        text(Button::secondary().to_css()),
        text(Button::outline().to_css()),
        text(Button::ghost().to_css()),
        text(Button::destructive().to_css()),
        text(Button::new(ButtonVariant::Link, ButtonSize::Md).to_css()),
        text(Button::primary().sm().to_css()),
        text(Button::primary().lg().to_css()),
        text(Button::primary().icon().to_css()),
        text(Button::primary().disabled().to_css()),
        text(Button::primary().full_width().to_css()),
        section("Style Builder + CSS", fg),
        text(
            Style::centered_col()
                .gap(Spacing::S4)
                .bg(Color::blue(Scale::S500))
                .rounded(BorderRadius::Md)
                .to_css()
        ),
    ]
    .spacing(6)
    .into()
}

fn render_motion(fg: Color) -> Element<'static, Message> {
    column![
        section("Durations", fg),
        text(TransitionDuration::Ms0.to_css()),
        text(TransitionDuration::Ms75.to_css()),
        text(TransitionDuration::Ms100.to_css()),
        text(TransitionDuration::Ms150.to_css()),
        text(TransitionDuration::Ms200.to_css()),
        text(TransitionDuration::Ms300.to_css()),
        text(TransitionDuration::Ms500.to_css()),
        text(TransitionDuration::Ms700.to_css()),
        text(TransitionDuration::Ms1000.to_css()),
        text(TransitionDuration::CustomMs(350).to_css()),
        section("Easing", fg),
        text(Easing::Linear.to_css()),
        text(Easing::In.to_css()),
        text(Easing::Out.to_css()),
        text(Easing::InOut.to_css()),
        section("Animation tokens", fg),
        text(AnimationToken::None.to_css()),
        text(AnimationToken::Spin.to_css()),
        text(AnimationToken::Ping.to_css()),
        text(AnimationToken::Pulse.to_css()),
        text(AnimationToken::Bounce.to_css()),
    ]
    .spacing(6)
    .into()
}

fn render_semantic(fg: Color) -> Element<'static, Message> {
    let theme = SemanticThemeVars::shadcn_neutral();
    let primary = theme
        .resolve_light(SemanticColor::Primary)
        .unwrap_or(Color::blue(Scale::S600));
    let primary_fg = theme
        .resolve_light(SemanticColor::PrimaryForeground)
        .unwrap_or(Color::white());
    let secondary = theme
        .resolve_light(SemanticColor::Secondary)
        .unwrap_or(Color::gray(Scale::S200));
    let secondary_fg = theme
        .resolve_light(SemanticColor::SecondaryForeground)
        .unwrap_or(Color::gray(Scale::S900));
    column![
        section("Semantic vars (CSS)", fg),
        text(format!("{:?} => {}", SemanticColor::Background, SemanticColor::Background.to_css())),
        text(format!("{:?} => {}", SemanticColor::Foreground, SemanticColor::Foreground.to_css())),
        text(format!("{:?} => {}", SemanticColor::Primary, SemanticColor::Primary.to_css())),
        text(format!("{:?} => {}", SemanticColor::Border, SemanticColor::Border.to_css())),
        section("Direct resolve (no CSS)", fg),
        text(format!(
            "light background: {}",
            theme.resolve_light(SemanticColor::Background).map(|c| c.to_css()).unwrap_or_default()
        )),
        text(format!(
            "dark background: {}",
            theme.resolve_dark(SemanticColor::Background).map(|c| c.to_css()).unwrap_or_default()
        )),
        section("Semantic visual demo", fg),
        row![
            semantic_chip("Background", theme.resolve_light(SemanticColor::Background).unwrap_or(Color::gray(Scale::S50)), fg),
            semantic_chip("Card", theme.resolve_light(SemanticColor::Card).unwrap_or(Color::white()), fg),
            semantic_chip("Accent", theme.resolve_light(SemanticColor::Accent).unwrap_or(Color::gray(Scale::S100)), fg),
            semantic_chip("Destructive", theme.resolve_light(SemanticColor::Destructive).unwrap_or(Color::red(Scale::S600)), primary_fg),
        ].spacing(8).wrap(),
        row![
            semantic_button("Primary", primary, primary_fg),
            semantic_button("Secondary", secondary, secondary_fg),
            semantic_button(
                "Destructive",
                theme.resolve_light(SemanticColor::Destructive).unwrap_or(Color::red(Scale::S600)),
                primary_fg,
            ),
        ].spacing(8).wrap(),
        row![
            semantic_button("Tab: Active", primary, primary_fg),
            semantic_button("Tab: Idle", secondary, secondary_fg),
        ].spacing(8),
        text("Generated variable block:"),
        container(text(theme.to_css_variables()).size(11)).padding(8).style(|_| container::Style {
            background: Some(Background::Color(to_color(Color::slate(Scale::S950)))),
            border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }),
    ]
    .spacing(6)
    .into()
}

fn swatch(color: Color) -> Element<'static, Message> {
    container(text(""))
        .width(18)
        .height(18)
        .style(move |_| container::Style {
            background: Some(Background::Color(to_color(color))),
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn styled_button<'a>(label: &'a str, spec: Button, msg: Message) -> Element<'a, Message> {
    let s = spec.style();
    let bg = s
        .background_color
        .map(to_color)
        .unwrap_or_else(|| to_color(Color::gray(Scale::S200)));
    let fg = s
        .text_color
        .map(to_color)
        .unwrap_or_else(|| to_color(Color::gray(Scale::S900)));
    let border_radius = s.border_radius.map(radius_to_px).unwrap_or(6.0);
    let border_width = s
        .border_width
        .map(|w| match w {
            BorderWidth::S0 => 0.0,
            BorderWidth::S1 => 1.0,
            BorderWidth::S2 => 2.0,
            BorderWidth::S4 => 4.0,
            BorderWidth::S8 => 8.0,
        })
        .unwrap_or(0.0);
    let border_color = s.border_color.map(to_color).unwrap_or(bg);

    button(text(label).color(fg))
        .padding(8)
        .on_press(msg)
        .style(move |_theme, _status| {
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: fg,
                border: Border {
                    radius: border_radius.into(),
                    width: border_width,
                    color: border_color,
                },
                ..button::Style::default()
            }
        })
        .into()
}

fn semantic_button<'a>(label: &'a str, bg: Color, fg: Color) -> Element<'a, Message> {
    button(text(label).color(to_color(fg)))
        .padding(8)
        .style(move |_theme, _status| {
            button::Style {
                background: Some(Background::Color(to_color(bg))),
                text_color: to_color(fg),
                border: Border {
                    radius: 6.0.into(),
                    width: 0.0,
                    color: to_color(bg),
                },
                ..button::Style::default()
            }
        })
        .into()
}

fn semantic_chip<'a>(label: &'a str, bg: Color, fg: Color) -> Element<'a, Message> {
    container(text(label).size(11).color(to_color(fg)))
        .padding([6, 10])
        .style(move |_| container::Style {
            background: Some(Background::Color(to_color(bg))),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: to_color(Color::gray(Scale::S300)),
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
