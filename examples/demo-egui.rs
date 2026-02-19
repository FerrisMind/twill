//! Egui demo showcasing rustwind styling.

use eframe::egui;
use rustwind::{Color, Scale, Spacing, Padding, BorderRadius, Style};
use rustwind::backends::{to_color32, to_corner_radius, to_frame};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Rustwind - Egui Demo"),
        ..Default::default()
    };
    
    eframe::run_native("Rustwind Egui", options, Box::new(|cc| {
        // Set up dark theme with rustwind colors
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.window_fill = to_color32(Color::slate(Scale::S900));
        style.visuals.panel_fill = to_color32(Color::slate(Scale::S900));
        style.visuals.extreme_bg_color = to_color32(Color::slate(Scale::S950));
        style.visuals.override_text_color = Some(to_color32(Color::slate(Scale::S50)));
        cc.egui_ctx.set_style(style);
        
        Ok(Box::new(DemoApp::default()))
    }))
}

struct DemoApp {
    counter: i32,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title
            ui.heading(
                egui::RichText::new("Rustwind + Egui Demo")
                    .size(28.0)
                    .color(to_color32(Color::white()))
                    .strong()
            );
            
            ui.add_space(16.0);
            
            // Description
            ui.label(
                egui::RichText::new("Type-safe styling for Rust GUI applications")
                    .size(16.0)
                    .color(to_color32(Color::slate(Scale::S400)))
            );
            
            ui.add_space(24.0);
            
            // Button showcase
            ui.label(egui::RichText::new("Button Variants")
                .size(20.0)
                .color(to_color32(Color::white()))
                .strong());
            
            ui.add_space(12.0);
            
            ui.horizontal(|ui| {
                // Primary button
                let primary_btn = egui::Button::new(
                    egui::RichText::new("Primary")
                        .color(to_color32(Color::white()))
                )
                .fill(to_color32(Color::blue(Scale::S500)))
                .corner_radius(to_corner_radius(BorderRadius::Md));
                
                if ui.add(primary_btn).clicked() {
                    self.counter += 1;
                }
                
                // Secondary button
                let secondary_btn = egui::Button::new(
                    egui::RichText::new("Secondary")
                        .color(to_color32(Color::gray(Scale::S900)))
                )
                .fill(to_color32(Color::gray(Scale::S100)))
                .corner_radius(to_corner_radius(BorderRadius::Md));
                
                if ui.add(secondary_btn).clicked() {
                    self.counter += 1;
                }
                
                // Danger button
                let danger_btn = egui::Button::new(
                    egui::RichText::new("Danger")
                        .color(to_color32(Color::white()))
                )
                .fill(to_color32(Color::red(Scale::S500)))
                .corner_radius(to_corner_radius(BorderRadius::Md));
                
                if ui.add(danger_btn).clicked() {
                    self.counter -= 1;
                }
            });
            
            ui.add_space(24.0);
            
            // Counter display in a styled frame
            let card_style = Style::new()
                .padding(Padding::all(Spacing::S6))
                .bg(Color::slate(Scale::S800))
                .rounded(BorderRadius::Lg);
            
            to_frame(&card_style).show(ui, |ui| {
                ui.label(egui::RichText::new("Counter")
                    .size(14.0)
                    .color(to_color32(Color::slate(Scale::S400))));
                ui.label(egui::RichText::new(format!("{}", self.counter))
                    .size(48.0)
                    .color(to_color32(Color::white()))
                    .strong());
            });
            
            ui.add_space(24.0);
            
            // Color palette showcase
            ui.label(egui::RichText::new("Color Palette")
                .size(20.0)
                .color(to_color32(Color::white()))
                .strong());
            
            ui.add_space(12.0);
            
            ui.horizontal(|ui| {
                for scale in [Scale::S100, Scale::S300, Scale::S500, Scale::S700, Scale::S900] {
                    let color_rect = egui::Frame::default()
                        .fill(to_color32(Color::blue(scale)))
                        .corner_radius(to_corner_radius(BorderRadius::Md))
                        .inner_margin(egui::Vec2::splat(20.0));
                    color_rect.show(ui, |_ui| {});
                }
            });
            
            ui.horizontal(|ui| {
                for scale in [Scale::S100, Scale::S300, Scale::S500, Scale::S700, Scale::S900] {
                    let color_rect = egui::Frame::default()
                        .fill(to_color32(Color::red(scale)))
                        .corner_radius(to_corner_radius(BorderRadius::Md))
                        .inner_margin(egui::Vec2::splat(20.0));
                    color_rect.show(ui, |_ui| {});
                }
            });
            
            ui.horizontal(|ui| {
                for scale in [Scale::S100, Scale::S300, Scale::S500, Scale::S700, Scale::S900] {
                    let color_rect = egui::Frame::default()
                        .fill(to_color32(Color::green(scale)))
                        .corner_radius(to_corner_radius(BorderRadius::Md))
                        .inner_margin(egui::Vec2::splat(20.0));
                    color_rect.show(ui, |_ui| {});
                }
            });
        });
    }
}