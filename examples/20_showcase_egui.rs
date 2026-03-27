#[cfg(feature = "egui")]
mod common;

#[cfg(feature = "egui")]
use common::{semantic_summary, showcase_sections, token_palette};
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
            ui.heading("Tokens");
            ui.horizontal_wrapped(|ui| {
                for (color, label) in token_palette() {
                    let swatch = RichText::new(format!("  {label}  "))
                        .background_color(color.to_egui())
                        .color(egui::Color32::WHITE);
                    ui.label(swatch);
                }
            });

            ui.add_space(12.0);
            ui.heading("Semantic theme");
            let variant = if self.dark_mode {
                ThemeVariant::Dark
            } else {
                ThemeVariant::Light
            };
            for (label, token) in semantic_summary() {
                let color = twill_egui::to_semantic_color32(token, variant);
                ui.label(RichText::new(label).color(color));
            }

            ui.add_space(12.0);
            ui.heading("Composed sections");
            for section in showcase_sections() {
                section.style.to_egui().show(ui, |ui| {
                    ui.label(RichText::new(section.title).strong());
                    ui.label(section.description);
                    if section.title == "Responsive" {
                        let resolved = section.style.at_breakpoint(Breakpoint::Lg);
                        ui.label(format!(
                            "Lg -> width={:?}, padding={:?}",
                            resolved.width_value(),
                            resolved.padding_value()
                        ));
                    }
                });
                ui.add_space(8.0);
            }
        });
    }
}

#[cfg(not(feature = "egui"))]
fn main() {}
