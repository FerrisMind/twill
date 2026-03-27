#[cfg(feature = "egui")]
mod common;

#[cfg(feature = "egui")]
use common::{
    composition_style, interactive_style, responsive_style, semantic_summary, surface_style,
    token_palette,
};
#[cfg(feature = "egui")]
use eframe::egui::{self, RichText};
#[cfg(feature = "egui")]
use twill::backends::egui::{self as twill_egui, ToEgui};
#[cfg(feature = "egui")]
use twill::prelude::*;

#[cfg(feature = "egui")]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "twill showcase (egui)",
        options,
        Box::new(|_cc| Ok(Box::<ShowcaseApp>::default())),
    )
}

#[cfg(feature = "egui")]
#[derive(Default)]
struct ShowcaseApp {
    dark_mode: bool,
}

#[cfg(feature = "egui")]
impl eframe::App for ShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Twill 0.3.x layered examples");
                if ui.button("Toggle semantic theme").clicked() {
                    self.dark_mode = !self.dark_mode;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Tokens");
                ui.horizontal_wrapped(|ui| {
                    for (color, label) in token_palette() {
                        let swatch = RichText::new(format!("  {label}  "))
                            .background_color(color.to_egui())
                            .color(egui::Color32::WHITE);
                        ui.label(swatch);
                    }
                });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    surface_style().to_egui().show(ui, |ui| {
                        ui.label(RichText::new("Radius").strong());
                        ui.label("Rounded XL surface");
                    });
                    surface_style().shadow(Shadow::Lg).to_egui().show(ui, |ui| {
                        ui.label(RichText::new("Shadow").strong());
                        ui.label("Large shadow token");
                    });
                    surface_style().padding(Padding::all(Spacing::S6)).to_egui().show(ui, |ui| {
                        ui.label(RichText::new("Spacing").strong());
                        ui.label("Padding S6 preview");
                    });
                });

                ui.add_space(12.0);
                ui.heading("Semantic theme");
                let variant = if self.dark_mode {
                    ThemeVariant::Dark
                } else {
                    ThemeVariant::Light
                };
                let semantic_surface = Style::new()
                    .bg(
                        SemanticThemeVars::shadcn_neutral()
                            .resolve(SemanticColor::Background, variant)
                            .unwrap_or(Color::white()),
                    )
                    .text_color(
                        SemanticThemeVars::shadcn_neutral()
                            .resolve(SemanticColor::Foreground, variant)
                            .unwrap_or(Color::slate(Scale::S900)),
                    )
                    .padding(Padding::all(Spacing::S4))
                    .rounded(BorderRadius::Xl)
                    .border(
                        BorderWidth::S1,
                        BorderStyle::Solid,
                        SemanticThemeVars::shadcn_neutral()
                            .resolve(SemanticColor::Border, variant)
                            .unwrap_or(Color::slate(Scale::S300)),
                    );
                semantic_surface.to_egui().show(ui, |ui| {
                    ui.label(RichText::new("Light/dark semantic aliases").strong());
                    ui.label("These values switch with the semantic theme toggle.");
                    ui.add_space(8.0);
                    for (label, token) in semantic_summary() {
                        let color = twill_egui::to_semantic_color32(token, variant);
                        ui.label(RichText::new(label).color(color));
                    }
                });

                ui.add_space(12.0);
                ui.heading("Composed sections");
                composition_style().to_egui().show(ui, |ui| {
                    ui.label(RichText::new("Base style").strong());
                    ui.label("Reusable surface built from one Style.");
                    ui.label("Padding, border, text color, radius, and shadow come from core API.");
                });
                ui.add_space(8.0);
                interactive_style().to_egui().show(ui, |ui| {
                    ui.label(RichText::new("Interactive states").strong());
                    ui.label("Hover, focus-visible, disabled, data-state, and aria-state live next to the base style.");
                    ui.label("Open state adds a larger shadow; focus-visible adds a ring.");
                });
                ui.add_space(8.0);
                semantic_surface.to_egui().show(ui, |ui| {
                    ui.label(RichText::new("Semantic aliases").strong());
                    ui.label("This card itself uses semantic Background / Foreground / Border colors.");
                    ui.label("The toggle at the top switches the entire semantic surface.");
                });
                ui.add_space(8.0);
                let responsive = responsive_style();
                responsive.to_egui().show(ui, |ui| {
                    ui.label(RichText::new("Responsive resolved preview").strong());
                    ui.label("The same Style resolves into distinct cards at each breakpoint.");
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        for breakpoint in [
                            Breakpoint::Sm,
                            Breakpoint::Md,
                            Breakpoint::Lg,
                            Breakpoint::S2xl,
                        ] {
                            let resolved = responsive.at_breakpoint(breakpoint);
                            resolved.to_egui().show(ui, |ui| {
                                ui.label(RichText::new(format!("{breakpoint:?}")).strong());
                                ui.label("Resolved card preview");
                                ui.small(format!(
                                    "width={:?}, padding={:?}, shadow={:?}",
                                    resolved.width_value(),
                                    resolved.padding_value(),
                                    resolved.box_shadow_value()
                                ));
                            });
                            ui.add_space(8.0);
                        }
                    });
                });
            });
        });
    }
}

#[cfg(not(feature = "egui"))]
fn main() {}
