//! Egui demo showcasing ALL rustwind capabilities.

use eframe::egui;
use rustwind::backends::{to_color32, to_corner_radius, to_frame};
use rustwind::{
    BorderRadius, Color, FontSize, FontWeight, Padding, Scale, Shadow, Spacing, Style, ToCss,
};

type PaletteFn = fn(Scale) -> Color;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Rustwind - Egui Demo (Full Features)"),
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
}

impl Default for DemoApp {
    fn default() -> Self {
        Self { dark_mode: true }
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
        style.visuals.window_fill = to_color32(bg);
        style.visuals.panel_fill = to_color32(bg);
        style.visuals.override_text_color = Some(to_color32(if self.dark_mode {
            Color::slate(Scale::S50)
        } else {
            Color::slate(Scale::S900)
        }));
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let label = if self.dark_mode {
                        "Switch to Light (gray-50)"
                    } else {
                        "Switch to Dark (gray-950)"
                    };
                    if ui.button(label).clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
                ui.add_space(8.0);
                ui.heading(
                    egui::RichText::new("Rustwind - Complete Feature Demo")
                        .size(32.0)
                        .color(to_color32(if self.dark_mode {
                            Color::white()
                        } else {
                            Color::slate(Scale::S900)
                        }))
                        .strong(),
                );
                ui.add_space(24.0);

                // === COLOR PALETTES ===
                section_title(ui, "All Color Palettes (22 palettes × 11 shades)");

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
                    ui.label(
                        egui::RichText::new(name)
                            .size(12.0)
                            .color(to_color32(Color::slate(Scale::S400))),
                    );
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
                            let rect = egui::Frame::default()
                                .fill(to_color32(color_fn(scale)))
                                .corner_radius(4.0)
                                .inner_margin(egui::Vec2::splat(12.0));
                            rect.show(ui, |_ui| {});
                        }
                    });
                    ui.add_space(4.0);
                }

                ui.add_space(24.0);

                // === SPACING SCALE ===
                section_title(ui, "Spacing Scale (0-96, px)");
                ui.horizontal_wrapped(|ui| {
                    let item_width = 180.0;
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
                    for (spacing, label) in spacings {
                        let px = spacing.to_rem().unwrap_or(0.0) * 16.0;
                        let bar_width = px.clamp(1.0, item_width - 70.0);
                        ui.allocate_ui_with_layout(
                            egui::vec2(item_width, 28.0),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(label)
                                        .size(10.0)
                                        .color(to_color32(Color::slate(Scale::S200))),
                                );
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(bar_width, 12.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(
                                    rect,
                                    2.0,
                                    to_color32(Color::blue(Scale::S500)),
                                );
                            },
                        );
                    }
                });

                ui.add_space(24.0);

                // === BORDER RADIUS ===
                section_title(ui, "Border Radius");
                ui.horizontal_wrapped(|ui| {
                    for (radius, name) in [
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
                    ] {
                        let frame = egui::Frame::default()
                            .fill(to_color32(Color::violet(Scale::S500)))
                            .corner_radius(to_corner_radius(radius))
                            .inner_margin(egui::Vec2::splat(16.0));
                        frame.show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(name)
                                    .size(11.0)
                                    .color(to_color32(Color::white())),
                            );
                        });
                    }
                });

                ui.add_space(24.0);

                // === TYPOGRAPHY ===
                section_title(ui, "Typography - Font Sizes");
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
                ] {
                    let rem = size.size_rem();
                    let px = rem * 16.0;
                    ui.label(
                        egui::RichText::new(format!("{} ({:.0}px)", name, px))
                            .size(px)
                            .color(to_color32(Color::slate(Scale::S200))),
                    );
                }

                ui.add_space(16.0);
                section_title(ui, "Typography - Font Weights");
                ui.horizontal_wrapped(|ui| {
                    for (idx, (_weight, name)) in [
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
                        let btn = egui::Button::new(
                            egui::RichText::new(name)
                                .color(if idx <= 4 {
                                    to_color32(Color::white())
                                } else {
                                    to_color32(Color::slate(Scale::S900))
                                })
                                .size(12.0),
                        )
                        .fill(to_color32(Color::slate(tone)))
                        .corner_radius(4.0);
                        ui.add(btn);
                    }
                });

                ui.add_space(24.0);

                // === SHADOWS ===
                section_title(ui, "Box Shadows");
                ui.horizontal_wrapped(|ui| {
                    for (shadow_token, name, spread, alpha_step, offset_y) in [
                        (Shadow::Sm, "sm", 4.0, 16u8, 2.0),
                        (Shadow::Md, "md", 8.0, 22u8, 4.0),
                        (Shadow::Lg, "lg", 12.0, 30u8, 6.0),
                        (Shadow::Xl, "xl", 18.0, 38u8, 8.0),
                        (Shadow::S2xl, "2xl", 24.0, 46u8, 10.0),
                    ] {
                        ui.allocate_ui(egui::vec2(140.0, 96.0), |ui| {
                            let (rect, _) = ui
                                .allocate_exact_size(egui::vec2(124.0, 72.0), egui::Sense::hover());
                            let card = egui::Rect::from_min_size(
                                rect.min + egui::vec2(8.0, 8.0),
                                egui::vec2(108.0, 56.0),
                            );

                            // Manual multi-layer shadow so the difference is visually obvious.
                            for layer in 1..=5 {
                                let grow = spread * (layer as f32) / 5.0;
                                let alpha =
                                    alpha_step.saturating_sub((layer - 1) as u8 * (alpha_step / 6));
                                let shadow_rect = card
                                    .translate(egui::vec2(0.0, offset_y + layer as f32))
                                    .expand(grow);
                                ui.painter().rect_filled(
                                    shadow_rect,
                                    10.0 + grow * 0.25,
                                    egui::Color32::from_black_alpha(alpha),
                                );
                            }

                            ui.painter()
                                .rect_filled(card, 10.0, to_color32(Color::white()));
                            ui.painter().text(
                                card.center(),
                                egui::Align2::CENTER_CENTER,
                                name,
                                egui::FontId::proportional(14.0),
                                to_color32(Color::slate(Scale::S700)),
                            );
                            ui.painter().text(
                                egui::pos2(card.center().x, card.center().y + 14.0),
                                egui::Align2::CENTER_CENTER,
                                format!("{shadow_token:?}"),
                                egui::FontId::proportional(10.0),
                                to_color32(Color::slate(Scale::S500)),
                            );
                        });
                    }
                });

                ui.add_space(24.0);

                // === BUTTON VARIANTS ===
                section_title(ui, "Button Variants");
                ui.horizontal(|ui| {
                    // Primary button
                    let btn = egui::Button::new(
                        egui::RichText::new("Primary").color(to_color32(Color::white())),
                    )
                    .fill(to_color32(Color::blue(Scale::S500)))
                    .corner_radius(6.0);
                    ui.add(btn);

                    // Secondary button
                    let btn = egui::Button::new(
                        egui::RichText::new("Secondary")
                            .color(to_color32(Color::slate(Scale::S900))),
                    )
                    .fill(to_color32(Color::gray(Scale::S200)))
                    .corner_radius(6.0);
                    ui.add(btn);

                    // Outline button
                    let btn = egui::Button::new(
                        egui::RichText::new("Outline").color(to_color32(Color::blue(Scale::S500))),
                    )
                    .fill(to_color32(Color::slate(Scale::S800)))
                    .corner_radius(6.0)
                    .stroke(egui::Stroke::new(1.0, to_color32(Color::blue(Scale::S500))));
                    ui.add(btn);

                    // Ghost button
                    let btn = egui::Button::new(
                        egui::RichText::new("Ghost").color(to_color32(Color::slate(Scale::S300))),
                    )
                    .fill(to_color32(Color::slate(Scale::S800)))
                    .corner_radius(6.0);
                    ui.add(btn);

                    // Destructive button
                    let btn = egui::Button::new(
                        egui::RichText::new("Destructive").color(to_color32(Color::white())),
                    )
                    .fill(to_color32(Color::red(Scale::S500)))
                    .corner_radius(6.0);
                    ui.add(btn);
                });

                ui.add_space(16.0);
                section_title(ui, "Button Sizes");
                ui.horizontal(|ui| {
                    for (name, padding) in [("Small", 6.0), ("Medium", 10.0), ("Large", 14.0)] {
                        let btn = egui::Button::new(
                            egui::RichText::new(name).color(to_color32(Color::white())),
                        )
                        .fill(to_color32(Color::blue(Scale::S500)))
                        .corner_radius(6.0);
                        ui.add_sized([80.0 + padding * 2.0, 24.0 + padding], btn);
                    }
                });

                ui.add_space(24.0);

                // === STYLE BUILDER ===
                section_title(ui, "Style Builder Examples");

                let card_style = Style::new()
                    .padding(Padding::all(Spacing::S6))
                    .bg(Color::slate(Scale::S800))
                    .rounded(BorderRadius::Lg);

                to_frame(&card_style).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Card Component")
                            .size(18.0)
                            .color(to_color32(Color::white()))
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new("Style::new().padding().bg().rounded()")
                            .size(12.0)
                            .color(to_color32(Color::slate(Scale::S400))),
                    );
                });

                ui.add_space(12.0);

                let flex_style = Style::centered_col().gap(Spacing::S4);
                to_frame(&flex_style).show(ui, |ui| {
                    ui.set_min_width(300.0);
                    ui.label(
                        egui::RichText::new("Centered Column").color(to_color32(Color::white())),
                    );
                    ui.label(
                        egui::RichText::new("Style::centered_col().gap(S4)")
                            .size(11.0)
                            .color(to_color32(Color::slate(Scale::S400))),
                    );
                });

                ui.add_space(24.0);

                // === CSS OUTPUT ===
                section_title(ui, "CSS Output Example");
                let style = Style::new()
                    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
                    .bg(Color::blue(Scale::S500))
                    .rounded(BorderRadius::Md);

                let frame = egui::Frame::default()
                    .fill(to_color32(Color::slate(Scale::S950)))
                    .corner_radius(8.0)
                    .inner_margin(egui::Vec2::splat(12.0));
                frame.show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(style.to_css())
                            .size(12.0)
                            .color(to_color32(Color::green(Scale::S400))),
                    );
                });
            });
        });
    }
}

fn section_title(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .size(18.0)
            .color(to_color32(Color::white()))
            .strong(),
    );
    ui.add_space(8.0);
}
