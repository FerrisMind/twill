#![allow(dead_code)]

use eframe::egui::{self, RichText};
use twill::backends::egui::ToEgui;
use twill::prelude::{core::*, theme::*};

pub fn run_demo<T>(title: &'static str) -> eframe::Result<()>
where
    T: eframe::App + Default + 'static,
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 780.0]),
        ..Default::default()
    };

    eframe::run_native(title, options, Box::new(|_cc| Ok(Box::<T>::default())))
}

pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
    ctx.set_visuals(if dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    });
}

pub fn example_shell(
    ctx: &egui::Context,
    title: &str,
    subtitle: &str,
    dark_mode: Option<&mut bool>,
    add_body: impl FnOnce(&mut egui::Ui),
) {
    egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(title);
                ui.label(subtitle);
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(dark_mode) = dark_mode {
                    let label = if *dark_mode {
                        "Light theme"
                    } else {
                        "Dark theme"
                    };
                    if ui.button(label).clicked() {
                        *dark_mode = !*dark_mode;
                    }
                }
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| add_body(ui));
    });
}

fn text_color(style: &Style, ui: &egui::Ui) -> egui::Color32 {
    style
        .text_color_value()
        .map(|color| color.to_egui())
        .unwrap_or_else(|| ui.visuals().text_color())
}

pub fn style_card(ui: &mut egui::Ui, style: &Style, title: &str, body: &str, facts: &[String]) {
    style.to_egui().show(ui, |ui| {
        let text = text_color(style, ui);

        ui.label(RichText::new(title).strong().size(20.0).color(text));
        ui.add_space(4.0);
        ui.label(RichText::new(body).color(text));
        if !facts.is_empty() {
            ui.add_space(8.0);
            for fact in facts {
                ui.label(RichText::new(fact).small().color(text));
            }
        }
    });
}

pub fn merge_state(base: &Style, nested: Option<&Style>) -> Style {
    base.clone().merged(nested.cloned().unwrap_or_default())
}

pub fn semantic_surface(variant: ThemeVariant) -> Style {
    let theme = SemanticThemeVars::shadcn_neutral();
    Style::new()
        .bg(theme
            .resolve(SemanticColor::Background, variant)
            .unwrap_or(Color::white()))
        .text_color(
            theme
                .resolve(SemanticColor::Foreground, variant)
                .unwrap_or(Color::slate(Scale::S900)),
        )
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Xl)
        .border(
            BorderWidth::S1,
            BorderStyle::Solid,
            theme
                .resolve(SemanticColor::Border, variant)
                .unwrap_or(Color::slate(Scale::S300)),
        )
}
