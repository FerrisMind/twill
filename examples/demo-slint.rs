//! Slint demo showcasing rustwind capabilities via tabs.

slint::slint! {
    import { VerticalBox, HorizontalBox, ScrollView } from "std-widgets.slint";

    export component DemoWindow inherits Window {
        title: "Rustwind - Slint Demo";
        in-out property <bool> dark_mode: true;
        in-out property <int> active_tab: 0;
        callback toggle_theme();
        callback set_tab(int);

        background: root.dark_mode ? #030712 : #F9FAFB;
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
                        background: rgb(59, 130, 246);
                        t := TouchArea { clicked => { root.toggle_theme(); } }
                        Text {
                            text: root.dark_mode ? "Switch to Light (gray-50)" : "Switch to Dark (gray-950)";
                            color: white;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                            font-size: 13px;
                        }
                    }

                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 0 ? rgb(30, 64, 175) : rgb(71, 85, 105);
                        TouchArea { clicked => { root.set_tab(0); } }
                        Text { text: "Tokens"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 1 ? rgb(30, 64, 175) : rgb(71, 85, 105);
                        TouchArea { clicked => { root.set_tab(1); } }
                        Text { text: "Typography"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 2 ? rgb(30, 64, 175) : rgb(71, 85, 105);
                        TouchArea { clicked => { root.set_tab(2); } }
                        Text { text: "Components"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 3 ? rgb(30, 64, 175) : rgb(71, 85, 105);
                        TouchArea { clicked => { root.set_tab(3); } }
                        Text { text: "Motion"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                    Rectangle {
                        min-width: 110px; min-height: 36px; border-radius: 8px;
                        background: root.active_tab == 4 ? rgb(30, 64, 175) : rgb(71, 85, 105);
                        TouchArea { clicked => { root.set_tab(4); } }
                        Text { text: "Semantic"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 12px; }
                    }
                }

                Text {
                    text: "Rustwind - Slint Demo";
                    font-size: 28px;
                    color: root.dark_mode ? rgb(248, 250, 252) : rgb(15, 23, 42);
                    font-weight: 700;
                    horizontal-alignment: center;
                }

                // Tokens tab
                VerticalBox {
                    visible: root.active_tab == 0;
                    spacing: 8px;
                    Text { text: "Tokens"; font-size: 20px; color: root.dark_mode ? rgb(248, 250, 252) : rgb(15, 23, 42); }
                    Text { text: "Color palettes, spacing, radius, shadows"; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                    HorizontalBox {
                        spacing: 6px;
                        for c in [rgb(248,250,252), rgb(15,23,42), rgb(239,68,68), rgb(249,115,22), rgb(234,179,8), rgb(34,197,94), rgb(6,182,212), rgb(59,130,246), rgb(139,92,246), rgb(217,70,239), rgb(244,63,94)] : Rectangle {
                            min-width: 22px; min-height: 22px; border-radius: 4px; background: c;
                        }
                    }
                    HorizontalBox {
                        spacing: 8px;
                        for w in [6px, 10px, 16px, 24px, 32px, 48px, 64px, 96px] : Rectangle {
                            min-width: w; min-height: 10px; border-radius: 3px; background: rgb(59,130,246);
                        }
                    }
                    HorizontalBox {
                        spacing: 8px;
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 0px; background: rgb(139,92,246); }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 4px; background: rgb(139,92,246); }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 8px; background: rgb(139,92,246); }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 12px; background: rgb(139,92,246); }
                        Rectangle { min-width: 52px; min-height: 30px; border-radius: 999px; background: rgb(139,92,246); }
                    }
                }

                // Typography tab
                VerticalBox {
                    visible: root.active_tab == 1;
                    spacing: 6px;
                    Text { text: "Typography"; font-size: 20px; color: root.dark_mode ? rgb(248, 250, 252) : rgb(15, 23, 42); }
                    Text { text: "xs (12px)"; font-size: 12px; color: root.dark_mode ? rgb(226,232,240) : rgb(30,41,59); }
                    Text { text: "sm (14px)"; font-size: 14px; color: root.dark_mode ? rgb(226,232,240) : rgb(30,41,59); }
                    Text { text: "base (16px)"; font-size: 16px; color: root.dark_mode ? rgb(226,232,240) : rgb(30,41,59); }
                    Text { text: "lg (18px)"; font-size: 18px; color: root.dark_mode ? rgb(226,232,240) : rgb(30,41,59); }
                    Text { text: "xl (20px)"; font-size: 20px; color: root.dark_mode ? rgb(226,232,240) : rgb(30,41,59); }
                    Text { text: "2xl (24px)"; font-size: 24px; color: root.dark_mode ? rgb(226,232,240) : rgb(30,41,59); }
                    HorizontalBox {
                        spacing: 6px;
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: rgb(15,23,42); Text { text: "Thin 100"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: rgb(30,41,59); Text { text: "Normal 400"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: rgb(51,65,85); Text { text: "Medium 500"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: rgb(100,116,139); Text { text: "Bold 700"; color: white; horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                        Rectangle { min-width: 88px; min-height: 26px; border-radius: 4px; background: rgb(203,213,225); Text { text: "Black 900"; color: rgb(15,23,42); horizontal-alignment: center; vertical-alignment: center; font-size: 11px; } }
                    }
                }

                // Components tab
                VerticalBox {
                    visible: root.active_tab == 2;
                    spacing: 8px;
                    Text { text: "Components"; font-size: 20px; color: root.dark_mode ? rgb(248, 250, 252) : rgb(15, 23, 42); }
                    Text { text: "Button variants and builder chains (see demo.rs / README for full CSS output)."; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                    HorizontalBox {
                        spacing: 10px;
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: rgb(59,130,246); Text { text: "Primary"; color: white; horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: rgb(243,244,246); Text { text: "Secondary"; color: rgb(17,24,39); horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: root.dark_mode ? rgb(15,23,42) : rgb(255,255,255); border-width: 1px; border-color: rgb(59,130,246); Text { text: "Outline"; color: rgb(59,130,246); horizontal-alignment: center; vertical-alignment: center; } }
                        Rectangle { min-width: 110px; min-height: 36px; border-radius: 6px; background: rgb(239,68,68); Text { text: "Destructive"; color: white; horizontal-alignment: center; vertical-alignment: center; } }
                    }
                }

                // Motion tab
                VerticalBox {
                    visible: root.active_tab == 3;
                    spacing: 6px;
                    Text { text: "Motion"; font-size: 20px; color: root.dark_mode ? rgb(248, 250, 252) : rgb(15, 23, 42); }
                    Text { text: "Durations: 0, 75, 100, 150, 200, 300, 500, 700, 1000ms"; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                    Text { text: "Easing: linear, ease-in, ease-out, ease-in-out"; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                    Text { text: "Animations: none, spin, ping, pulse, bounce"; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                }

                // Semantic tab
                VerticalBox {
                    visible: root.active_tab == 4;
                    spacing: 6px;
                    Text { text: "Semantic"; font-size: 20px; color: root.dark_mode ? rgb(248, 250, 252) : rgb(15, 23, 42); }
                    Text { text: "Supports both CSS vars (var(--primary)) and direct Color resolve (light/dark)."; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                    Text { text: "Intended for CSS-capable frameworks (future dioxus support) and non-CSS GUIs."; font-size: 13px; color: root.dark_mode ? rgb(148, 163, 184) : rgb(71, 85, 105); }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let demo = DemoWindow::new()?;

    {
        let demo_weak = demo.as_weak();
        demo.on_toggle_theme(move || {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_dark_mode(!demo.get_dark_mode());
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
