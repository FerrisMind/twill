//! Slint demo showcasing rustwind capabilities via tabs.

slint::slint! {
    import { VerticalBox, HorizontalBox, ScrollView } from "std-widgets.slint";

    export component DemoWindow inherits Window {
        title: "Rustwind - Slint Demo";
        in-out property <bool> dark_mode: true;
        in-out property <int> active_tab: 0;
        in-out property <color> color_text: black;
        in-out property <color> color_muted_text: black;
        in-out property <color> color_btn_primary: black;
        in-out property <color> color_btn_primary_fg: white;
        in-out property <color> color_btn_secondary: black;
        in-out property <color> color_btn_secondary_fg: white;
        in-out property <color> color_tab_active: black;
        in-out property <color> color_tab_idle: black;
        in-out property <color> color_app_bg: black;
        in-out property <color> color_semantic_bg: black;
        in-out property <color> color_semantic_card: black;
        in-out property <color> color_semantic_accent: black;
        in-out property <color> color_semantic_destructive: black;
        in-out property <color> color_token_bar: black;
        in-out property <color> color_token_radius: black;
        in-out property <color> color_token_1: black;
        in-out property <color> color_token_2: black;
        in-out property <color> color_token_3: black;
        in-out property <color> color_token_4: black;
        in-out property <color> color_token_5: black;
        in-out property <color> color_token_6: black;
        in-out property <color> color_token_7: black;
        in-out property <color> color_token_8: black;
        in-out property <color> color_token_9: black;
        in-out property <color> color_token_10: black;
        in-out property <color> color_token_11: black;
        in-out property <color> color_weight_1: black;
        in-out property <color> color_weight_2: black;
        in-out property <color> color_weight_3: black;
        in-out property <color> color_weight_4: black;
        in-out property <color> color_weight_5: black;
        in-out property <color> color_weight_5_fg: white;
        callback toggle_theme();
        callback set_tab(int);

        background: root.color_app_bg;
        preferred-width: 1200px;
        preferred-height: 900px;

        ScrollView {
            width: 100%;
            height: 100%;

            VerticalBox {
                padding: 24px;
                spacing: 10px;

                HorizontalBox {
                    spacing: 8px;
                    Rectangle {
                        min-width: 240px;
                        min-height: 36px;
                        border-radius: 8px;
                        background: root.color_btn_primary;
                        t := TouchArea { clicked => { root.toggle_theme(); } }
                        Text {
                            text: root.dark_mode ? "Switch to Light (gray-50)" : "Switch to Dark (gray-950)";
                            color: root.color_btn_primary_fg;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                            font-size: 13px;
                        }
                    }

                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 0 ? root.color_tab_active : root.color_tab_idle;
                        TouchArea { clicked => { root.set_tab(0); } }
                        Text { text: "Tokens"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 1 ? root.color_tab_active : root.color_tab_idle;
                        TouchArea { clicked => { root.set_tab(1); } }
                        Text { text: "Typography"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 2 ? root.color_tab_active : root.color_tab_idle;
                        TouchArea { clicked => { root.set_tab(2); } }
                        Text { text: "Components"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 3 ? root.color_tab_active : root.color_tab_idle;
                        TouchArea { clicked => { root.set_tab(3); } }
                        Text { text: "Motion"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 4 ? root.color_tab_active : root.color_tab_idle;
                        TouchArea { clicked => { root.set_tab(4); } }
                        Text { text: "Semantic"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                }

                Text {
                    text: "Rustwind - Slint Demo";
                    font-size: 28px;
                    color: root.color_text;
                    font-weight: 700;
                    horizontal-alignment: center;
                }

                // Tokens tab
                VerticalBox {
                    visible: root.active_tab == 0;
                    spacing: 8px;
                    Text { text: "Tokens"; font-size: 20px; color: root.color_text; }
                    Text { text: "Color palettes, spacing, radius, shadows"; font-size: 13px; color: root.color_muted_text; }
                    HorizontalBox {
                        spacing: 6px;
                        for c in [root.color_token_1, root.color_token_2, root.color_token_3, root.color_token_4, root.color_token_5, root.color_token_6, root.color_token_7, root.color_token_8, root.color_token_9, root.color_token_10, root.color_token_11] : Rectangle {
                            min-width: 22px; min-height: 22px; border-radius: 4px; background: c;
                        }
                    }
                    HorizontalBox {
                        spacing: 8px;
                        for w in [6px, 10px, 16px, 24px, 32px, 48px, 64px, 96px] : Rectangle {
                            min-width: w; min-height: 10px; border-radius: 3px; background: root.color_token_bar;
                        }
                    }
                    HorizontalBox {
                        spacing: 8px;
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 0px; background: root.color_token_radius; }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 4px; background: root.color_token_radius; }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 8px; background: root.color_token_radius; }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 12px; background: root.color_token_radius; }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 999px; background: root.color_token_radius; }
                    }
                }

                // Typography tab
                VerticalBox {
                    visible: root.active_tab == 1;
                    spacing: 6px;
                    Text { text: "Typography"; font-size: 20px; color: root.color_text; }
                    Text { text: "xs (12px)"; font-size: 12px; color: root.color_text; }
                    Text { text: "sm (14px)"; font-size: 14px; color: root.color_text; }
                    Text { text: "base (16px)"; font-size: 16px; color: root.color_text; }
                    Text { text: "lg (18px)"; font-size: 18px; color: root.color_text; }
                    Text { text: "xl (20px)"; font-size: 20px; color: root.color_text; }
                    Text { text: "2xl (24px)"; font-size: 24px; color: root.color_text; }
                    HorizontalBox {
                        spacing: 6px;
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: root.color_weight_1; Text { text: "Thin 100"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: root.color_weight_2; Text { text: "Normal 400"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: root.color_weight_3; Text { text: "Medium 500"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: root.color_weight_4; Text { text: "Bold 700"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: root.color_weight_5; Text { text: "Black 900"; color: root.color_weight_5_fg; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                    }
                }

                // Components tab
                VerticalBox {
                    visible: root.active_tab == 2;
                    spacing: 8px;
                    Text { text: "Components"; font-size: 20px; color: root.color_text; }
                    Text { text: "Button variants and builder chains (see demo.rs / README for full CSS output)."; font-size: 13px; color: root.color_muted_text; }
                    HorizontalBox {
                        spacing: 10px;
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: root.color_btn_primary; Text { text: "Primary"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: root.color_btn_secondary; Text { text: "Secondary"; color: root.color_btn_secondary_fg; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: root.color_semantic_bg; border-width: 1px; border-color: root.color_btn_primary; Text { text: "Outline"; color: root.color_btn_primary; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: root.color_semantic_destructive; Text { text: "Destructive"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; } }
                    }
                }

                // Motion tab
                VerticalBox {
                    visible: root.active_tab == 3;
                    spacing: 6px;
                    Text { text: "Motion"; font-size: 20px; color: root.color_text; }
                    Text { text: "Durations: 0, 75, 100, 150, 200, 300, 500, 700, 1000ms"; font-size: 13px; color: root.color_muted_text; }
                    Text { text: "Easing: linear, ease-in, ease-out, ease-in-out"; font-size: 13px; color: root.color_muted_text; }
                    Text { text: "Animations: none, spin, ping, pulse, bounce"; font-size: 13px; color: root.color_muted_text; }
                }

                // Semantic tab
                VerticalBox {
                    visible: root.active_tab == 4;
                    spacing: 6px;
                    Text { text: "Semantic"; font-size: 20px; color: root.color_text; }
                    Text { text: "Supports both CSS vars and direct Color resolve."; font-size: 13px; color: root.color_muted_text; }
                    HorizontalBox {
                        spacing: 8px;
                        Rectangle { min-width: 120px; min-height: 40px; border-radius: 8px; background: root.color_semantic_bg; Text { text: "Background"; color: root.color_text; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 120px; min-height: 40px; border-radius: 8px; background: root.color_semantic_card; Text { text: "Card"; color: root.color_text; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 120px; min-height: 40px; border-radius: 8px; background: root.color_semantic_accent; Text { text: "Accent"; color: root.color_text; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 120px; min-height: 40px; border-radius: 8px; background: root.color_semantic_destructive; Text { text: "Destructive"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; } }
                    }
                    HorizontalBox {
                        spacing: 8px;
                        Rectangle { min-width: 120px; min-height: 36px; border-radius: 6px; background: root.color_btn_primary; Text { text: "Semantic Primary"; color: root.color_btn_primary_fg; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 120px; min-height: 36px; border-radius: 6px; background: root.color_btn_secondary; Text { text: "Semantic Secondary"; color: root.color_btn_secondary_fg; horizontal-alignment: center; vertical-alignment: center; } }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let demo = DemoWindow::new()?;

    apply_theme_colors(&demo);

    {
        let demo_weak = demo.as_weak();
        demo.on_toggle_theme(move || {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_dark_mode(!demo.get_dark_mode());
            apply_theme_colors(&demo);
        });
    }

    {
        let demo_weak = demo.as_weak();
        demo.on_set_tab(move |tab| {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_active_tab(tab);
        });
    }

    demo.run()
}

fn apply_theme_colors(demo: &DemoWindow) {
    let theme = rustwind::SemanticThemeVars::shadcn_neutral();
    let is_dark = demo.get_dark_mode();
    let resolve = |token| theme.resolve(token, is_dark).unwrap_or(rustwind::Color::gray(rustwind::Scale::S500));
    let to_sl = rustwind::backends::to_slint_color;

    demo.set_color_text(to_sl(resolve(rustwind::SemanticColor::Foreground)));
    demo.set_color_muted_text(to_sl(resolve(rustwind::SemanticColor::MutedForeground)));
    demo.set_color_btn_primary(to_sl(resolve(rustwind::SemanticColor::Primary)));
    demo.set_color_btn_primary_fg(to_sl(resolve(rustwind::SemanticColor::PrimaryForeground)));
    demo.set_color_btn_secondary(to_sl(resolve(rustwind::SemanticColor::Secondary)));
    demo.set_color_btn_secondary_fg(to_sl(resolve(rustwind::SemanticColor::SecondaryForeground)));
    demo.set_color_tab_active(to_sl(resolve(rustwind::SemanticColor::Primary)));
    demo.set_color_tab_idle(to_sl(resolve(rustwind::SemanticColor::Muted)));
    demo.set_color_app_bg(to_sl(resolve(rustwind::SemanticColor::Background)));
    demo.set_color_semantic_bg(to_sl(resolve(rustwind::SemanticColor::Background)));
    demo.set_color_semantic_card(to_sl(resolve(rustwind::SemanticColor::Card)));
    demo.set_color_semantic_accent(to_sl(resolve(rustwind::SemanticColor::Accent)));
    demo.set_color_semantic_destructive(to_sl(resolve(rustwind::SemanticColor::Destructive)));
    demo.set_color_token_bar(to_sl(rustwind::Color::blue(rustwind::Scale::S500)));
    demo.set_color_token_radius(to_sl(rustwind::Color::violet(rustwind::Scale::S500)));
    demo.set_color_token_1(to_sl(rustwind::Color::slate(rustwind::Scale::S50)));
    demo.set_color_token_2(to_sl(rustwind::Color::slate(rustwind::Scale::S900)));
    demo.set_color_token_3(to_sl(rustwind::Color::red(rustwind::Scale::S500)));
    demo.set_color_token_4(to_sl(rustwind::Color::orange(rustwind::Scale::S500)));
    demo.set_color_token_5(to_sl(rustwind::Color::amber(rustwind::Scale::S500)));
    demo.set_color_token_6(to_sl(rustwind::Color::green(rustwind::Scale::S500)));
    demo.set_color_token_7(to_sl(rustwind::Color::cyan(rustwind::Scale::S500)));
    demo.set_color_token_8(to_sl(rustwind::Color::blue(rustwind::Scale::S500)));
    demo.set_color_token_9(to_sl(rustwind::Color::violet(rustwind::Scale::S500)));
    demo.set_color_token_10(to_sl(rustwind::Color::fuchsia(rustwind::Scale::S500)));
    demo.set_color_token_11(to_sl(rustwind::Color::rose(rustwind::Scale::S500)));
    demo.set_color_weight_1(to_sl(rustwind::Color::slate(rustwind::Scale::S900)));
    demo.set_color_weight_2(to_sl(rustwind::Color::slate(rustwind::Scale::S800)));
    demo.set_color_weight_3(to_sl(rustwind::Color::slate(rustwind::Scale::S700)));
    demo.set_color_weight_4(to_sl(rustwind::Color::slate(rustwind::Scale::S500)));
    demo.set_color_weight_5(to_sl(rustwind::Color::slate(rustwind::Scale::S200)));
    demo.set_color_weight_5_fg(to_sl(rustwind::Color::slate(rustwind::Scale::S900)));
}
