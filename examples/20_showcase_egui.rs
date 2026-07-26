#[cfg(feature = "egui")]
mod common;

#[cfg(feature = "egui")]
	use common::{
    composition_style_for, interactive_style, responsive_style_for, semantic_demo_chip,
    semantic_summary, themed_surface_style, token_palette,
};
#[cfg(feature = "egui")]
use eframe::egui::{self, RichText};
#[cfg(feature = "egui")]
use twill::backends::egui::{self as twill_egui, ToEgui};
#[cfg(feature = "egui")]
use twill::prelude::{core::*, theme::*};

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
                let variant = if self.dark_mode {
                    ThemeVariant::Dark
                } else {
                    ThemeVariant::Light
                };

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
                let surface = themed_surface_style(variant);
                let surface_ink = surface
                    .text_color_value()
                    .map(|c| c.to_egui())
                    .unwrap_or(ui.visuals().text_color());
                ui.horizontal_wrapped(|ui| {
                    surface.to_egui().show(ui, |ui| {
                        ui.label(RichText::new("Radius").strong().color(surface_ink));
                        ui.label(RichText::new("Rounded XL surface").color(surface_ink));
                    });
                    surface.clone().shadow(Shadow::Lg).to_egui().show(ui, |ui| {
                        ui.label(RichText::new("Shadow").strong().color(surface_ink));
                        ui.label(RichText::new("Large shadow token").color(surface_ink));
                    });
                    surface
                        .clone()
                        .padding(Padding::all(Spacing::S6))
                        .to_egui()
                        .show(ui, |ui| {
                            ui.label(RichText::new("Spacing").strong().color(surface_ink));
                            ui.label(RichText::new("Padding S6 preview").color(surface_ink));
                        });
                });

                ui.add_space(12.0);
                ui.heading("Semantic theme");
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
                        let (surface_token, ink_token) = semantic_demo_chip(token, variant);
                        let surface = SemanticThemeVars::shadcn_neutral()
                            .resolve(surface_token, variant)
                            .unwrap_or(Color::white());
                        let ink = twill_egui::to_semantic_color32(ink_token, variant);
                        let chip = Style::new()
                            .bg(surface)
                            .text_color(
                                SemanticThemeVars::shadcn_neutral()
                                    .resolve(ink_token, variant)
                                    .unwrap_or(Color::slate(Scale::S900)),
                            )
                            .padding(Padding::symmetric(Spacing::S2, Spacing::S3))
                            .rounded(BorderRadius::Md)
                            .border(
                                BorderWidth::S1,
                                BorderStyle::Solid,
                                SemanticThemeVars::shadcn_neutral()
                                    .resolve(SemanticColor::Border, variant)
                                    .unwrap_or(Color::slate(Scale::S300)),
                            );
                        chip.to_egui().show(ui, |ui| {
                            ui.label(RichText::new(label).color(ink));
                        });
                        ui.add_space(4.0);
                    }
                });

                ui.add_space(12.0);
                ui.heading("Composed sections");
                let composition = composition_style_for(variant);
                let composition_ink = composition
                    .text_color_value()
                    .map(|c| c.to_egui())
                    .unwrap_or(ui.visuals().text_color());
                composition.to_egui().show(ui, |ui| {
                    ui.label(RichText::new("Base style").strong().color(composition_ink));
                    ui.label(
                        RichText::new("Reusable surface built from one Style.").color(composition_ink),
                    );
                    ui.label(
                        RichText::new(
                            "Padding, border, text color, radius, and shadow come from core API.",
                        )
                        .color(composition_ink),
                    );
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
                let responsive = responsive_style_for(variant);
                let responsive_ink = responsive
                    .text_color_value()
                    .map(|c| c.to_egui())
                    .unwrap_or(ui.visuals().text_color());
                responsive.to_egui().show(ui, |ui| {
                    ui.label(
                        RichText::new("Responsive resolved preview")
                            .strong()
                            .color(responsive_ink),
                    );
                    ui.label(
                        RichText::new(
                            "The same Style resolves into distinct cards at each breakpoint.",
                        )
                        .color(responsive_ink),
                    );
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        for breakpoint in [
                            Breakpoint::Sm,
                            Breakpoint::Md,
                            Breakpoint::Lg,
                            Breakpoint::S2xl,
                        ] {
                            let resolved = responsive.at_breakpoint(breakpoint);
                            let resolved_ink = resolved
                                .text_color_value()
                                .map(|c| c.to_egui())
                                .unwrap_or(responsive_ink);
                            resolved.to_egui().show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("{breakpoint:?}"))
                                        .strong()
                                        .color(resolved_ink),
                                );
                                ui.label(RichText::new("Resolved card preview").color(resolved_ink));
                                ui.label(
                                    RichText::new(format!(
                                        "width={:?}, padding={:?}, shadow={:?}",
                                        resolved.width_value(),
                                        resolved.padding_value(),
                                        resolved.box_shadow_value()
                                    ))
                                    .small()
                                    .color(resolved_ink),
                                );
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
