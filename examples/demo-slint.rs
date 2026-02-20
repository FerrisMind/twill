//! Slint demo showcasing twill capabilities via a visually identical component showcase.

slint::slint! {
    import { VerticalBox, HorizontalBox, ScrollView } from "std-widgets.slint";

    component StyledButton inherits Rectangle {
        in property <string> text;
        in property <color> bg_color;
        in property <color> text_color;
        in property <length> r_radius: 6px;
        in property <length> b_width: 0px;
        in property <color> b_color: transparent;
        in property <color> hover_bg;
        callback clicked();

        min-height: 36px;
        horizontal-stretch: 0;
        border-radius: root.r_radius;
        background: root.bg_color;
        border-width: root.b_width;
        border-color: root.b_color;

        t := TouchArea {
            clicked => { root.clicked(); }
        }

        hover_overlay := Rectangle {
            width: 100%;
            height: 100%;
            border-radius: root.r_radius;
            background: t.has-hover ? root.hover_bg : transparent;
        }

        HorizontalBox {
            padding-left: 16px;
            padding-right: 16px;
            alignment: center;
            Text {
                text: root.text;
                color: root.text_color;
                font-size: 14px;
                vertical-alignment: center;
                horizontal-alignment: center;
            }
        }
    }

    export component DemoWindow inherits Window {
        title: "Twill - Universal Design Showcase (Slint)";
        in-out property <bool> dark_mode: true;
        in-out property <color> color_text: black;
        in-out property <color> color_muted_text: black;
        in-out property <color> color_app_bg: black;
        in-out property <color> color_card_bg: black;
        in-out property <color> color_card_border: black;
        
        in-out property <color> primary_bg: black;
        in-out property <color> primary_bg_hover: black;
        in-out property <color> primary_fg: white;
        in-out property <color> secondary_bg: black;
        in-out property <color> secondary_bg_hover: black;
        in-out property <color> secondary_fg: white;
        in-out property <color> outline_bg: transparent;
        in-out property <color> outline_bg_hover: transparent;
        in-out property <color> outline_fg: black;
        in-out property <color> outline_border: black;
        in-out property <color> ghost_bg: transparent;
        in-out property <color> ghost_bg_hover: transparent;
        in-out property <color> ghost_fg: black;
        in-out property <color> destructive_bg: red;
        in-out property <color> destructive_bg_hover: red;
        in-out property <color> destructive_fg: white;
        
        in-out property <color> tw_blue: black;
        in-out property <color> tw_blue_hover: black;
        in-out property <color> tw_slate: black;
        in-out property <color> tw_slate_hover: black;
        in-out property <color> tw_slate_fg: black;
        in-out property <color> tw_border: black;
        in-out property <color> tw_red: black;
        in-out property <color> tw_red_hover: black;
        in-out property <color> tw_white: white;
        in-out property <color> tw_black: black;

        in-out property <bool> shadow_enabled: false;

        callback toggle_theme();

        background: root.color_app_bg;
        preferred-width: 800px;
        preferred-height: 600px;

        ScrollView {
            width: 100%;
            height: 100%;

            VerticalBox {
                padding-top: 48px;
                alignment: start;
                spacing: 0px;

                VerticalBox {
                    alignment: center;
                    spacing: 12px;

                    HorizontalBox {
                        alignment: center;
                        spacing: 16px;

                        Image {
                            source: @image-url("../assets/icon.png");
                            width: 48px;
                            height: 48px;
                            horizontal-alignment: center;
                        }

                        VerticalBox {
                            alignment: center;
                            Text {
                                text: "Twill Universal Design";
                                font-size: 32px;
                                color: root.color_text;
                                font-weight: 700;
                                horizontal-alignment: center;
                            }
                        }
                    }
                    Text {
                        text: "Rendered exactly the same across Egui, Iced, and Slint.";
                        font-size: 16px;
                        color: root.color_muted_text;
                        horizontal-alignment: center;
                    }
                }

                Rectangle { height: 32px; }

                HorizontalBox {
                    alignment: center;
                    StyledButton {
                        text: root.dark_mode ? "Switch to Light Mode" : "Switch to Dark Mode";
                        bg_color: root.secondary_bg;
                        hover_bg: root.secondary_bg_hover;
                        text_color: root.secondary_fg;
                        clicked => { root.toggle_theme(); }
                    }
                }

                Rectangle { height: 48px; }

                Text {
                    text: "Semantic Palette (shadcn-like)";
                    font-size: 14px;
                    color: root.color_muted_text;
                    horizontal-alignment: center;
                }

                Rectangle { height: 12px; }

                HorizontalBox {
                    alignment: center;
                    spacing: 12px;

                    StyledButton { text: "Primary"; bg_color: root.primary_bg; hover_bg: root.primary_bg_hover; text_color: root.primary_fg; }
                    StyledButton { text: "Secondary"; bg_color: root.secondary_bg; hover_bg: root.secondary_bg_hover; text_color: root.secondary_fg; }
                    StyledButton { text: "Outline"; bg_color: root.outline_bg; hover_bg: root.outline_bg_hover; text_color: root.outline_fg; b_width: 1px; b_color: root.outline_border; }
                    StyledButton { text: "Ghost"; bg_color: root.ghost_bg; hover_bg: root.ghost_bg_hover; text_color: root.ghost_fg; }
                    StyledButton { text: "Destructive"; bg_color: root.destructive_bg; hover_bg: root.destructive_bg_hover; text_color: root.destructive_fg; }
                }

                Rectangle { height: 32px; }

                Text {
                    text: "Tailwind Palette (direct colors)";
                    font-size: 14px;
                    color: root.color_muted_text;
                    horizontal-alignment: center;
                }

                Rectangle { height: 12px; }

                HorizontalBox {
                    alignment: center;
                    spacing: 12px;

                    StyledButton { text: "Blue"; bg_color: root.tw_blue; hover_bg: root.tw_blue_hover; text_color: root.tw_white; }
                    StyledButton { text: "Slate"; bg_color: root.tw_slate; hover_bg: root.tw_slate_hover; text_color: root.tw_slate_fg; }
                    StyledButton { text: "Outline"; bg_color: transparent; hover_bg: root.tw_slate; text_color: root.tw_black; b_width: 1px; b_color: root.tw_border; }
                    StyledButton { text: "Ghost"; bg_color: transparent; hover_bg: root.tw_slate; text_color: root.tw_black; }
                    StyledButton { text: "Rose"; bg_color: root.tw_red; hover_bg: root.tw_red_hover; text_color: root.tw_white; }
                }

                Rectangle { height: 48px; }

                HorizontalBox {
                    alignment: center;
                    
                    Rectangle {
                        width: 420px;
                        background: root.color_card_bg;
                        border-radius: 12px;
                        border-width: 1px;
                        border-color: root.color_card_border;
                        drop-shadow-blur: root.shadow_enabled ? 15px : 0px;
                        drop-shadow-offset-y: root.shadow_enabled ? 10px : 0px;
                        drop-shadow-color: root.shadow_enabled ? #00000042 : transparent;

                        VerticalBox {
                            padding-left: 24px;
                            padding-right: 24px;
                            padding-top: 24px;
                            padding-bottom: 24px;
                            spacing: 0px;
                            alignment: center;

                            Rectangle { height: 8px; }
                            
                            Text {
                                text: "Interactive Card";
                                font-size: 24px;
                                font-weight: 700;
                                color: root.color_text;
                            }

                            Rectangle { height: 12px; }

                            Text {
                                text: "This card is styled entirely with Twill tokens and its appearance is mathematically mapped to Slint elements.";
                                font-size: 15px;
                                color: root.color_muted_text;
                                horizontal-alignment: center;
                                wrap: word-wrap;
                            }

                            Rectangle { height: 32px; }

                            HorizontalBox {
                                alignment: center;
                                StyledButton { text: "Action Button"; bg_color: root.primary_bg; hover_bg: root.primary_bg_hover; text_color: root.primary_fg; }
                            }

                            Rectangle { height: 8px; }
                        }
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

    demo.run()
}

fn apply_theme_colors(demo: &DemoWindow) {
    let theme = twill::SemanticThemeVars::shadcn_neutral();
    let is_dark = demo.get_dark_mode();
    let resolve =
        |token| theme.resolve(token, is_dark).unwrap_or(twill::Color::gray(twill::Scale::S500));
    let to_sl = twill::backends::to_slint_color;

    let accent = resolve(twill::SemanticColor::Accent);
    let primary_hover = resolve(twill::SemanticColor::Primary);
    let secondary_hover = resolve(twill::SemanticColor::Secondary);
    let destructive_hover = resolve(twill::SemanticColor::Destructive);

    demo.set_color_app_bg(to_sl(resolve(twill::SemanticColor::Background)));
    demo.set_color_text(to_sl(resolve(twill::SemanticColor::Foreground)));
    demo.set_color_muted_text(to_sl(resolve(twill::SemanticColor::MutedForeground)));
    demo.set_color_card_bg(to_sl(resolve(twill::SemanticColor::Card)));
    demo.set_color_card_border(to_sl(resolve(twill::SemanticColor::Border)));

    demo.set_primary_bg(to_sl(resolve(twill::SemanticColor::Primary)));
    demo.set_primary_bg_hover(to_sl(primary_hover).with_alpha(0.85));
    demo.set_primary_fg(to_sl(resolve(twill::SemanticColor::PrimaryForeground)));
    
    demo.set_secondary_bg(to_sl(resolve(twill::SemanticColor::Secondary)));
    demo.set_secondary_bg_hover(to_sl(secondary_hover).with_alpha(0.85));
    demo.set_secondary_fg(to_sl(resolve(twill::SemanticColor::SecondaryForeground)));

    demo.set_outline_bg(slint::Color::from_argb_u8(0, 0, 0, 0));
    demo.set_outline_bg_hover(to_sl(accent));
    demo.set_outline_fg(to_sl(resolve(twill::SemanticColor::Foreground)));
    demo.set_outline_border(to_sl(resolve(twill::SemanticColor::Border)));

    demo.set_ghost_bg(slint::Color::from_argb_u8(0, 0, 0, 0));
    demo.set_ghost_bg_hover(to_sl(accent));
    demo.set_ghost_fg(to_sl(resolve(twill::SemanticColor::Foreground)));

    demo.set_destructive_bg(to_sl(resolve(twill::SemanticColor::Destructive)));
    demo.set_destructive_bg_hover(to_sl(destructive_hover).with_alpha(0.85));
    demo.set_destructive_fg(to_sl(twill::Color::slate(twill::Scale::S50)));

    demo.set_tw_blue(to_sl(twill::Color::blue(twill::Scale::S500)));
    demo.set_tw_blue_hover(to_sl(twill::Color::blue(twill::Scale::S600)));

    demo.set_tw_slate(to_sl(if is_dark { twill::Color::slate(twill::Scale::S800) } else { twill::Color::slate(twill::Scale::S100) }));
    demo.set_tw_slate_hover(to_sl(if is_dark { twill::Color::slate(twill::Scale::S700) } else { twill::Color::slate(twill::Scale::S200) }));
    demo.set_tw_slate_fg(to_sl(if is_dark { twill::Color::slate(twill::Scale::S50) } else { twill::Color::slate(twill::Scale::S900) }));

    demo.set_tw_border(to_sl(if is_dark { twill::Color::slate(twill::Scale::S700) } else { twill::Color::slate(twill::Scale::S200) }));

    demo.set_tw_red(to_sl(twill::Color::red(twill::Scale::S500)));
    demo.set_tw_red_hover(to_sl(twill::Color::red(twill::Scale::S600)));

    demo.set_tw_white(to_sl(twill::Color::white()));
    demo.set_tw_black(to_sl(if is_dark { twill::Color::slate(twill::Scale::S50) } else { twill::Color::slate(twill::Scale::S900) }));

    demo.set_shadow_enabled(!is_dark);
}
