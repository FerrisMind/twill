#[cfg(feature = "egui")]
mod egui_demo_common;

#[cfg(feature = "egui")]
use eframe::egui;
#[cfg(feature = "egui")]
use egui_demo_common::{apply_theme, example_shell, merge_state, run_demo, style_card};
#[cfg(feature = "egui")]
use twill::prelude::core::*;

#[cfg(feature = "egui")]
fn main() -> eframe::Result<()> {
    run_demo::<StateVariantsApp>("twill state variants")
}

#[cfg(feature = "egui")]
#[derive(Default)]
struct StateVariantsApp {
    dark_mode: bool,
}

#[cfg(feature = "egui")]
fn state_style() -> Style {
    Style::interactive()
        .bg(Color::slate(Scale::S100))
        .text_color(Color::slate(Scale::S900))
        .hover(|style| style.bg(Color::slate(Scale::S200)))
        .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)))
        .disabled(|style| style.opacity(0.48))
        .active(|style| style.opacity(0.88))
        .selected(|style| {
            style.border(
                BorderWidth::S1,
                BorderStyle::Solid,
                Color::blue(Scale::S500),
            )
        })
        .checked(|style| {
            style
                .bg(Color::emerald(Scale::S500))
                .text_color(Color::white())
        })
        .open(|style| style.shadow(Shadow::Lg))
        .closed(|style| style.opacity(0.7))
        .data_attr(DataAttr::pair("side", "left"), |style| {
            style.rounded(BorderRadius::Lg)
        })
        .aria_attr(AriaAttr::Invalid, |style| {
            style.outline(
                BorderWidth::S2,
                OutlineStyle::Solid,
                Color::rose(Scale::S500),
            )
        })
}

#[cfg(feature = "egui")]
impl eframe::App for StateVariantsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx, self.dark_mode);

        example_shell(
            ctx,
            "State Variants",
            "One Style carries hover, focus-visible, disabled, selected, checked, open/closed, data-* and aria-* overrides.",
            Some(&mut self.dark_mode),
            |ui| {
                let base = state_style();

                let cards = [
                    (
                        "Base",
                        base.clone(),
                        "Interactive surface before any state is applied.",
                        vec![
                            format!("bg: {:?}", base.background_color_value()),
                            format!("text: {:?}", base.text_color_value()),
                        ],
                    ),
                    (
                        "Hover",
                        merge_state(&base, base.hover_style()),
                        "Preview of a hover override layered on top of the base style.",
                        vec![format!(
                            "hover bg: {:?}",
                            base.hover_style().and_then(Style::background_color_value)
                        )],
                    ),
                    (
                        "Focus Visible",
                        merge_state(&base, base.focus_visible_style()),
                        "Focus ring stays local to the state layer instead of leaking into the base surface.",
                        vec![format!(
                            "ring: {:?}",
                            base.focus_visible_style().and_then(Style::ring_color_value)
                        )],
                    ),
                    (
                        "Disabled",
                        merge_state(&base, base.disabled_style()),
                        "Disabled keeps the same structure, only muting the resolved opacity.",
                        vec![format!(
                            "opacity: {:?}",
                            base.disabled_style().and_then(Style::opacity_value)
                        )],
                    ),
                    (
                        "Selected",
                        merge_state(&base, base.selected_style()),
                        "Selection is expressed as a local border variant.",
                        vec![format!(
                            "border: {:?}",
                            base.selected_style().and_then(Style::border_color_value)
                        )],
                    ),
                    (
                        "Checked",
                        merge_state(&base, base.checked_style()),
                        "Checked changes both fill and foreground in one typed override.",
                        vec![format!(
                            "checked bg: {:?}",
                            base.checked_style().and_then(Style::background_color_value)
                        )],
                    ),
                    (
                        "Open",
                        merge_state(&base, base.open_style()),
                        "Open state makes the card feel lifted without changing its structural tokens.",
                        vec![format!(
                            "shadow: {:?}",
                            base.open_style().and_then(Style::box_shadow_value)
                        )],
                    ),
                    (
                        "Closed",
                        merge_state(&base, base.closed_style()),
                        "Closed can dim the same surface while preserving layout and spacing.",
                        vec![format!(
                            "opacity: {:?}",
                            base.closed_style().and_then(Style::opacity_value)
                        )],
                    ),
                    (
                        "data-side=left",
                        merge_state(&base, base.data_attr_style(DataAttr::pair("side", "left"))),
                        "Typed data hooks let downstream widgets keep their own semantic state names.",
                        vec![format!(
                            "radius: {:?}",
                            base.data_attr_style(DataAttr::pair("side", "left"))
                                .and_then(Style::border_radius_value)
                        )],
                    ),
                    (
                        "aria-invalid",
                        merge_state(&base, base.aria_attr_style(AriaAttr::Invalid)),
                        "ARIA hooks make error and selection states share the same layered model.",
                        vec![format!(
                            "outline: {:?}",
                            base.aria_attr_style(AriaAttr::Invalid)
                                .and_then(Style::outline_color_value)
                        )],
                    ),
                ];

                ui.columns(2, |columns| {
                    for (index, (title, style, body, facts)) in cards.into_iter().enumerate() {
                        let column = &mut columns[index % 2];
                        style_card(column, &style, title, body, &facts);
                        column.add_space(12.0);
                    }
                });
            },
        );
    }
}

#[cfg(not(feature = "egui"))]
fn main() {}
