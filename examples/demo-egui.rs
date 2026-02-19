//! Egui demo showcasing rustwind capabilities via tabs.

use eframe::egui;
use rustwind::backends::{to_color32, to_corner_radius, to_frame};
use rustwind::{
    AnimationToken, BorderRadius, Button, ButtonSize, ButtonVariant, Color, Easing, FontSize,
    FontWeight, Padding, Scale, SemanticColor, SemanticThemeVars, Shadow, Spacing, Style, ToCss,
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

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Rustwind - Egui Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Rustwind Egui",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}

struct DemoApp {
    dark_mode: bool,
    tab: Tab,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            dark_mode: true,
            tab: Tab::Tokens,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        let bg = if self.dark_mode {
            Color::gray(Scale::S950)
        } else {
            Color::gray(Scale::S50)
        };
        let fg = if self.dark_mode {
            Color::slate(Scale::S50)
        } else {
            Color::slate(Scale::S900)
        };
        style.visuals.window_fill = to_color32(bg);
        style.visuals.panel_fill = to_color32(bg);
        style.visuals.override_text_color = Some(to_color32(fg));
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if styled_button(
                    ui,
                    if self.dark_mode {
                        "Switch to Light (gray-50)"
                    } else {
                        "Switch to Dark (gray-950)"
                    },
                    Button::primary(),
                    false,
                )
                .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                }
                ui.separator();
                for (tab, label) in Tab::all() {
                    let active = self.tab == *tab;
                    let tab_btn = if active {
                        Button::primary()
                    } else {
                        Button::secondary().sm()
                    };
                    if styled_button(ui, label, tab_btn, active).clicked() {
                        self.tab = *tab;
                    }
                }
            });
            ui.add_space(8.0);

            egui::ScrollArea::vertical().show(ui, |ui| match self.tab {
                Tab::Tokens => render_tokens_tab(ui),
                Tab::Typography => render_typography_tab(ui),
                Tab::Components => render_components_tab(ui),
                Tab::Motion => render_motion_tab(ui),
                Tab::Semantic => render_semantic_tab(ui),
            });
        });
    }
}

fn render_tokens_tab(ui: &mut egui::Ui) {
    ui.heading("Tokens: Colors / Spacing / Radius / Shadows");
    ui.add_space(12.0);

    section_title(ui, "All Color Palettes (22 × 11)");
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
    for (name, color_fn) in palettes {
        ui.label(name);
        ui.horizontal_wrapped(|ui| {
            for scale in [
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
            ] {
                egui::Frame::default()
                    .fill(to_color32(color_fn(scale)))
                    .corner_radius(4.0)
                    .inner_margin(egui::Vec2::splat(10.0))
                    .show(ui, |_ui| {});
            }
        });
        ui.add_space(4.0);
    }

    ui.add_space(16.0);
    section_title(ui, "Spacing Scale");
    ui.horizontal_wrapped(|ui| {
        for (spacing, label) in [
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
        ] {
            let px = spacing.to_rem().unwrap_or(0.0) * 16.0;
            let width = px.clamp(2.0, 80.0);
            ui.group(|ui| {
                ui.label(label);
                let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 10.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 2.0, to_color32(Color::blue(Scale::S500)));
            });
        }
    });

    ui.add_space(16.0);
    section_title(ui, "Border Radius + Shadow Tokens");
    ui.horizontal_wrapped(|ui| {
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
        for (radius, name) in radii {
            egui::Frame::default()
                .fill(to_color32(Color::violet(Scale::S500)))
                .corner_radius(to_corner_radius(radius))
                .inner_margin(egui::Vec2::splat(14.0))
                .show(ui, |ui| {
                    ui.label(name);
                });
        }
    });
    ui.horizontal_wrapped(|ui| {
        for shadow in [Shadow::Xs2, Shadow::Xs, Shadow::Sm, Shadow::Md, Shadow::Lg, Shadow::Xl, Shadow::S2xl, Shadow::None] {
            ui.label(format!("{shadow:?}: {}", shadow.to_css()));
        }
    });
}

fn render_typography_tab(ui: &mut egui::Ui) {
    ui.heading("Typography Tokens");
    ui.add_space(12.0);
    section_title(ui, "Font Sizes");
    for (size, name) in [
        (FontSize::Xs, "xs"),
        (FontSize::Sm, "sm"),
        (FontSize::Base, "base"),
        (FontSize::Lg, "lg"),
        (FontSize::Xl, "xl"),
        (FontSize::S2xl, "2xl"),
        (FontSize::S3xl, "3xl"),
        (FontSize::S4xl, "4xl"),
        (FontSize::S5xl, "5xl"),
        (FontSize::S6xl, "6xl"),
        (FontSize::S7xl, "7xl"),
        (FontSize::S8xl, "8xl"),
        (FontSize::S9xl, "9xl"),
    ] {
        let px = (size.size_rem() * 16.0).min(56.0);
        ui.label(egui::RichText::new(format!("{name} ({px:.0}px)")).size(px));
    }

    ui.add_space(12.0);
    section_title(ui, "Font Weights");
    ui.horizontal_wrapped(|ui| {
        for (idx, weight) in [
            FontWeight::Thin,
            FontWeight::ExtraLight,
            FontWeight::Light,
            FontWeight::Normal,
            FontWeight::Medium,
            FontWeight::SemiBold,
            FontWeight::Bold,
            FontWeight::ExtraBold,
            FontWeight::Black,
        ]
        .into_iter()
        .enumerate()
        {
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
            ui.add(
                egui::Button::new(format!("{weight:?} ({})", weight.value()))
                    .fill(to_color32(Color::slate(tone))),
            );
        }
    });
}

fn render_components_tab(ui: &mut egui::Ui) {
    ui.heading("Components + Style Builder");
    ui.add_space(12.0);

    section_title(ui, "Button Variants");
    for css in [
        Button::primary().to_css(),
        Button::secondary().to_css(),
        Button::outline().to_css(),
        Button::ghost().to_css(),
        Button::destructive().to_css(),
        Button::new(ButtonVariant::Link, ButtonSize::Md).to_css(),
        Button::primary().sm().to_css(),
        Button::primary().lg().to_css(),
        Button::primary().icon().to_css(),
        Button::primary().disabled().to_css(),
        Button::primary().full_width().to_css(),
    ] {
        ui.label(css);
    }

    ui.add_space(12.0);
    section_title(ui, "Style Builder Chains");
    let card = Style::new()
        .padding(Padding::all(Spacing::S6))
        .bg(Color::slate(Scale::S800))
        .rounded(BorderRadius::Lg)
        .shadow(Shadow::Md);
    let interactive = Style::centered_col()
        .gap(Spacing::S4)
        .bg(Color::blue(Scale::S500))
        .rounded(BorderRadius::Md)
        .transition_property("all")
        .transition_duration(TransitionDuration::Ms300)
        .transition_ease(Easing::InOut)
        .animate(AnimationToken::Pulse);

    to_frame(&card).show(ui, |ui| {
        ui.label("Card");
        ui.label(card.to_css());
    });
    ui.add_space(8.0);
    to_frame(&interactive).show(ui, |ui| {
        ui.label("Interactive");
        ui.label(interactive.to_css());
    });
}

fn render_motion_tab(ui: &mut egui::Ui) {
    ui.heading("Motion Tokens");
    ui.add_space(12.0);
    section_title(ui, "Durations");
    ui.horizontal_wrapped(|ui| {
        for d in [
            TransitionDuration::Ms0,
            TransitionDuration::Ms75,
            TransitionDuration::Ms100,
            TransitionDuration::Ms150,
            TransitionDuration::Ms200,
            TransitionDuration::Ms300,
            TransitionDuration::Ms500,
            TransitionDuration::Ms700,
            TransitionDuration::Ms1000,
            TransitionDuration::CustomMs(350),
        ] {
            ui.label(d.to_css());
        }
    });
    section_title(ui, "Easing + Animation");
    for e in [Easing::Linear, Easing::In, Easing::Out, Easing::InOut] {
        ui.label(format!("{e:?}: {}", e.to_css()));
    }
    for a in [
        AnimationToken::None,
        AnimationToken::Spin,
        AnimationToken::Ping,
        AnimationToken::Pulse,
        AnimationToken::Bounce,
    ] {
        ui.label(format!("{a:?}: {}", a.to_css()));
    }
}

fn render_semantic_tab(ui: &mut egui::Ui) {
    ui.heading("Semantic Tokens (Optional Layer)");
    ui.add_space(12.0);
    let theme = SemanticThemeVars::shadcn_neutral();
    section_title(ui, "Variable-based usage");
    for token in [
        SemanticColor::Background,
        SemanticColor::Foreground,
        SemanticColor::Primary,
        SemanticColor::PrimaryForeground,
        SemanticColor::Border,
        SemanticColor::Ring,
    ] {
        ui.label(format!("{token:?} -> {}", token.to_css()));
    }
    section_title(ui, "Direct Color resolve (no CSS)");
    let semantic_samples = [
        SemanticColor::Background,
        SemanticColor::Foreground,
        SemanticColor::Primary,
        SemanticColor::Secondary,
        SemanticColor::Destructive,
        SemanticColor::Card,
        SemanticColor::Accent,
        SemanticColor::Border,
    ];
    ui.horizontal_wrapped(|ui| {
        for token in semantic_samples {
            let light = theme.resolve_light(token).unwrap_or(Color::gray(Scale::S50));
            let dark = theme.resolve_dark(token).unwrap_or(Color::gray(Scale::S950));
            egui::Frame::default()
                .fill(to_color32(light))
                .corner_radius(8.0)
                .inner_margin(egui::Vec2::new(10.0, 8.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(format!("{token:?}")).color(to_color32(dark)));
                });
        }
    });
    for token in semantic_samples {
        let light = theme.resolve_light(token).map(|c| c.to_css()).unwrap_or_default();
        let dark = theme.resolve_dark(token).map(|c| c.to_css()).unwrap_or_default();
        ui.label(format!("{token:?}: light={light}, dark={dark}"));
    }

    section_title(ui, "Semantic buttons/tabs visual");
    let primary = theme.resolve_light(SemanticColor::Primary).unwrap_or(Color::blue(Scale::S600));
    let primary_fg = theme
        .resolve_light(SemanticColor::PrimaryForeground)
        .unwrap_or(Color::white());
    let secondary = theme.resolve_light(SemanticColor::Secondary).unwrap_or(Color::gray(Scale::S200));
    let secondary_fg = theme
        .resolve_light(SemanticColor::SecondaryForeground)
        .unwrap_or(Color::gray(Scale::S900));
    ui.horizontal(|ui| {
        semantic_button(ui, "Semantic Primary", primary, primary_fg);
        semantic_button(ui, "Semantic Secondary", secondary, secondary_fg);
        semantic_button(
            ui,
            "Semantic Destructive",
            theme.resolve_light(SemanticColor::Destructive)
                .unwrap_or(Color::red(Scale::S600)),
            primary_fg,
        );
    });
    ui.horizontal(|ui| {
        semantic_tab(ui, "Overview", true, primary, primary_fg, secondary, secondary_fg);
        semantic_tab(ui, "Settings", false, primary, primary_fg, secondary, secondary_fg);
        semantic_tab(ui, "Danger Zone", false, primary, primary_fg, secondary, secondary_fg);
    });
    section_title(ui, "Generated CSS vars");
    egui::Frame::default()
        .fill(to_color32(Color::slate(Scale::S950)))
        .corner_radius(8.0)
        .inner_margin(egui::Vec2::splat(12.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(theme.to_css_variables())
                    .size(11.0)
                    .color(to_color32(Color::green(Scale::S400))),
            );
        });
}

fn section_title(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).size(18.0).strong());
    ui.add_space(6.0);
}

fn styled_button(
    ui: &mut egui::Ui,
    label: &str,
    button: Button,
    active: bool,
) -> egui::Response {
    let s = button.style();
    let fill = s
        .background_color
        .map(to_color32)
        .unwrap_or_else(|| to_color32(Color::gray(Scale::S200)));
    let text_color = s
        .text_color
        .map(to_color32)
        .unwrap_or_else(|| to_color32(Color::gray(Scale::S900)));
    let radius = s.border_radius.map(to_corner_radius).unwrap_or(4.0);
    let stroke = if let (Some(w), Some(c)) = (s.border_width, s.border_color) {
        let width = match w {
            rustwind::BorderWidth::S0 => 0.0,
            rustwind::BorderWidth::S1 => 1.0,
            rustwind::BorderWidth::S2 => 2.0,
            rustwind::BorderWidth::S4 => 4.0,
            rustwind::BorderWidth::S8 => 8.0,
        };
        egui::Stroke::new(width, to_color32(c))
    } else if active {
        egui::Stroke::new(1.0, to_color32(Color::blue(Scale::S700)))
    } else {
        egui::Stroke::NONE
    };
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(text_color))
            .fill(fill)
            .stroke(stroke)
            .corner_radius(radius),
    )
}

fn semantic_button(ui: &mut egui::Ui, label: &str, bg: Color, fg: Color) {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(to_color32(fg)))
            .fill(to_color32(bg))
            .corner_radius(6.0),
    );
}

fn semantic_tab(
    ui: &mut egui::Ui,
    label: &str,
    active: bool,
    active_bg: Color,
    active_fg: Color,
    idle_bg: Color,
    idle_fg: Color,
) {
    let (bg, fg) = if active {
        (active_bg, active_fg)
    } else {
        (idle_bg, idle_fg)
    };
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(to_color32(fg)))
            .fill(to_color32(bg))
            .corner_radius(6.0),
    );
}
