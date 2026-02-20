//! Egui demo showcasing twill capabilities via a visually identical component showcase.

use eframe::egui;
use twill::backends::{to_color32, to_color32_value, to_frame};
use twill::{
    BorderRadius, BorderWidth, Color, Padding, Scale, SemanticColor, SemanticThemeVars,
    Shadow, Spacing, Style,
};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Twill - Universal Design Showcase (Egui)"),
        ..Default::default()
    };
    eframe::run_native(
        "Twill Egui",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp { dark_mode: true, icon: None }))),
    )
}

struct DemoApp {
    dark_mode: bool,
    icon: Option<egui::TextureHandle>,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = SemanticThemeVars::shadcn_neutral();
        let bg = theme
            .resolve(SemanticColor::Background, self.dark_mode)
            .unwrap_or(Color::gray(Scale::S50));
        let fg = theme
            .resolve(SemanticColor::Foreground, self.dark_mode)
            .unwrap_or(Color::gray(Scale::S900));

        let mut visual_style = (*ctx.style()).clone();
        visual_style.visuals.window_fill = to_color32(bg);
        visual_style.visuals.panel_fill = to_color32(bg);
        visual_style.visuals.override_text_color = Some(to_color32(fg));
        ctx.set_style(visual_style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(48.0);

            ui.vertical_centered(|ui| {
                #[allow(clippy::collapsible_if)]
                if self.icon.is_none() {
                    if let Ok(img) = image::open("assets/icon.png") {
                        let size = [img.width() as _, img.height() as _];
                        let image_buffer = img.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            pixels.as_slice(),
                        );
                        self.icon = Some(ctx.load_texture("twill_icon", color_image, Default::default()));
                    }
                }
                
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 410.0) / 2.0); // Center the horizontal content
                    if let Some(texture) = &self.icon {
                        ui.add(egui::Image::new(texture).max_width(48.0).max_height(48.0));
                        ui.add_space(16.0);
                    }
    
                    ui.heading(
                        egui::RichText::new("Twill Universal Design")
                            .size(32.0)
                            .color(to_color32(fg))
                            .strong(),
                    );
                });

                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Rendered exactly the same across Egui, Iced, and Slint.")
                        .size(16.0)
                        .color(to_color32(
                            theme
                                .resolve(SemanticColor::MutedForeground, self.dark_mode)
                                .unwrap(),
                        )),
                );

                let primary_bg = to_color32(theme.resolve(SemanticColor::Primary, self.dark_mode).unwrap());
                let primary_fg = to_color32(theme.resolve(SemanticColor::PrimaryForeground, self.dark_mode).unwrap());
                let secondary_bg = to_color32(theme.resolve(SemanticColor::Secondary, self.dark_mode).unwrap());
                let secondary_fg = to_color32(theme.resolve(SemanticColor::SecondaryForeground, self.dark_mode).unwrap());
                let destructive_bg = to_color32(theme.resolve(SemanticColor::Destructive, self.dark_mode).unwrap());
                let destructive_fg = to_color32(Color::slate(Scale::S50));
                let fg_color = to_color32(fg);
                let border_color = to_color32(theme.resolve(SemanticColor::Border, self.dark_mode).unwrap());
                let accent_bg = to_color32(theme.resolve(SemanticColor::Accent, self.dark_mode).unwrap());
                let accent_fg = to_color32(theme.resolve(SemanticColor::AccentForeground, self.dark_mode).unwrap());

                let muted_fg = theme.resolve(SemanticColor::MutedForeground, self.dark_mode).unwrap();

                let primary_hover = theme.resolve(SemanticColor::Primary, self.dark_mode).unwrap();
                let secondary_hover = theme.resolve(SemanticColor::Secondary, self.dark_mode).unwrap();
                let destructive_hover = theme.resolve(SemanticColor::Destructive, self.dark_mode).unwrap();
                
                let tw_blue = to_color32(Color::blue(Scale::S500));
                let tw_blue_hover = to_color32(Color::blue(Scale::S600));
                
                let tw_slate = to_color32(if self.dark_mode { Color::slate(Scale::S800) } else { Color::slate(Scale::S100) });
                let tw_slate_hover = to_color32(if self.dark_mode { Color::slate(Scale::S700) } else { Color::slate(Scale::S200) });
                let tw_slate_fg = to_color32(if self.dark_mode { Color::slate(Scale::S50) } else { Color::slate(Scale::S900) });

                let tw_border = to_color32(if self.dark_mode { Color::slate(Scale::S700) } else { Color::slate(Scale::S200) });

                let tw_red = to_color32(Color::red(Scale::S500));
                let tw_red_hover = to_color32(Color::red(Scale::S600));
                
                let tw_white = to_color32(Color::white());
                let tw_black = to_color32(if self.dark_mode { Color::slate(Scale::S50) } else { Color::slate(Scale::S900) });

                let transparent = to_color32_value(twill::tokens::ColorValue::TRANSPARENT);

                if styled_button(
                    ui,
                    if self.dark_mode {
                        "Switch to Light Mode"
                    } else {
                        "Switch to Dark Mode"
                    },
                    secondary_bg, secondary_fg, 0.0, transparent, Some(to_color32(secondary_hover)), Some(secondary_fg)
                )
                .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                }

                ui.add_space(48.0);

                ui.label(egui::RichText::new("Semantic Palette (shadcn-like)").size(14.0).color(to_color32(muted_fg)));
                ui.add_space(12.0);

                let total_w = 460.0;
                ui.allocate_ui_with_layout(
                    egui::vec2(total_w, 40.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        styled_button(ui, "Primary", primary_bg, primary_fg, 0.0, transparent, Some(to_color32(primary_hover)), Some(primary_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Secondary", secondary_bg, secondary_fg, 0.0, transparent, Some(to_color32(secondary_hover)), Some(secondary_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Outline", transparent, fg_color, 1.0, border_color, Some(accent_bg), Some(accent_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Ghost", transparent, fg_color, 0.0, transparent, Some(accent_bg), Some(accent_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Destructive", destructive_bg, destructive_fg, 0.0, transparent, Some(to_color32(destructive_hover)), Some(destructive_fg));
                    },
                );
                
                ui.add_space(32.0);

                ui.label(egui::RichText::new("Tailwind Palette (direct colors)").size(14.0).color(to_color32(muted_fg)));
                ui.add_space(12.0);

                ui.allocate_ui_with_layout(
                    egui::vec2(total_w, 40.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        styled_button(ui, "Blue", tw_blue, tw_white, 0.0, transparent, Some(tw_blue_hover), Some(tw_white));
                        ui.add_space(12.0);
                        styled_button(ui, "Slate", tw_slate, tw_slate_fg, 0.0, transparent, Some(tw_slate_hover), Some(tw_slate_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Outline", transparent, tw_black, 1.0, tw_border, Some(tw_slate), Some(tw_slate_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Ghost", transparent, tw_black, 0.0, transparent, Some(tw_slate), Some(tw_slate_fg));
                        ui.add_space(12.0);
                        styled_button(ui, "Rose", tw_red, tw_white, 0.0, transparent, Some(tw_red_hover), Some(tw_white));
                    },
                );

                ui.add_space(48.0);

                let card_style = Style::new()
                    .bg(theme.resolve(SemanticColor::Card, self.dark_mode).unwrap())
                    .border(
                        BorderWidth::S1,
                        twill::BorderStyle::Solid,
                        theme
                            .resolve(SemanticColor::Border, self.dark_mode)
                            .unwrap(),
                    )
                    .rounded(BorderRadius::Xl)
                    .shadow(if self.dark_mode { Shadow::None } else { Shadow::Lg })
                    .padding(Padding::all(Spacing::S6));

                ui.allocate_ui_with_layout(
                    egui::vec2(420.0, 200.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        to_frame(&card_style).show(ui, |ui| {
                            ui.set_width(420.0);
                            ui.vertical_centered(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new("Interactive Card")
                                        .size(24.0)
                                        .strong()
                                        .color(to_color32(
                                            theme
                                                .resolve(SemanticColor::Foreground, self.dark_mode)
                                                .unwrap(),
                                        )),
                                );
                                
                                ui.add_space(12.0);
                                ui.label(
                                    egui::RichText::new(
                                        "This card is styled entirely with Twill tokens and its appearance is mathematically mapped to Egui elements.",
                                    )
                                    .size(15.0)
                                    .color(to_color32(
                                        theme
                                            .resolve(SemanticColor::MutedForeground, self.dark_mode)
                                            .unwrap(),
                                    )),
                                );

                                ui.add_space(32.0);
                                styled_button(ui, "Action Button", primary_bg, primary_fg, 0.0, transparent, Some(to_color32(primary_hover)), Some(primary_fg));
                                ui.add_space(8.0);
                            });
                        });
                    },
                );
            });
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn styled_button(ui: &mut egui::Ui, label: &str, bg: egui::Color32, fg: egui::Color32, stroke_width: f32, stroke_color: egui::Color32, hover_bg: Option<egui::Color32>, hover_fg: Option<egui::Color32>) -> egui::Response {
    let px = 16.0;
    let py = 8.0;

    let prev_padding = ui.spacing().button_padding;
    ui.spacing_mut().button_padding = egui::vec2(px, py);

    let stroke = if stroke_width > 0.0 {
        egui::Stroke::new(stroke_width, stroke_color)
    } else {
        egui::Stroke::NONE
    };

    let res = ui.scope(|ui| {
        let is_trans = bg == egui::Color32::TRANSPARENT;
        
        let mut visual_hover_bg = hover_bg.unwrap_or(bg);
        if hover_bg.is_some() && !is_trans {
            visual_hover_bg = egui::Color32::from_rgba_premultiplied(
                (visual_hover_bg.r() as f32 * 0.9) as u8, 
                (visual_hover_bg.g() as f32 * 0.9) as u8, 
                (visual_hover_bg.b() as f32 * 0.9) as u8, 
                visual_hover_bg.a()
            );
        } else if hover_bg.is_none() {
            if is_trans {
                visual_hover_bg = egui::Color32::from_rgba_unmultiplied(128, 128, 128, 25);
            } else {
                visual_hover_bg = egui::Color32::from_rgba_premultiplied((bg.r() as f32 * 0.9) as u8, (bg.g() as f32 * 0.9) as u8, (bg.b() as f32 * 0.9) as u8, bg.a());
            }
        }

        let mut visual_active_bg = hover_bg.unwrap_or(bg);
        if hover_bg.is_some() {
            if !is_trans {
                visual_active_bg = egui::Color32::from_rgba_premultiplied(
                    (visual_active_bg.r() as f32 * 0.8) as u8, 
                    (visual_active_bg.g() as f32 * 0.8) as u8, 
                    (visual_active_bg.b() as f32 * 0.8) as u8, 
                    visual_active_bg.a()
                );
            }
        } else if is_trans {
            visual_active_bg = egui::Color32::from_rgba_unmultiplied(128, 128, 128, 50);
        } else {
            visual_active_bg = egui::Color32::from_rgba_premultiplied((bg.r() as f32 * 0.8) as u8, (bg.g() as f32 * 0.8) as u8, (bg.b() as f32 * 0.8) as u8, bg.a());
        }

        ui.visuals_mut().widgets.inactive.weak_bg_fill = bg;
        ui.visuals_mut().widgets.hovered.weak_bg_fill = visual_hover_bg;
        ui.visuals_mut().widgets.active.weak_bg_fill = visual_active_bg;
        
        let fg_color = hover_fg.unwrap_or(fg);

        let btn = egui::Button::new(egui::RichText::new(label).color(fg_color).size(14.0))
            .stroke(stroke)
            .corner_radius(6.0)
            .min_size(egui::vec2(0.0, 36.0));

        ui.add(btn)
    }).inner;

    ui.spacing_mut().button_padding = prev_padding;
    res
}
