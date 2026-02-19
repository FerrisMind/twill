//! Slint demo showcasing rustwind styling.

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, ScrollView } from "std-widgets.slint";

    export component DemoWindow inherits Window {
        title: "Rustwind - Slint Demo";
        in-out property <bool> dark_mode: true;
        background: root.dark_mode ? #030712 : #F9FAFB;
        preferred-width: 1200px;
        preferred-height: 980px;

        in-out property <int> counter: 0;
        callback increment();
        callback decrement();
        callback reset();
        callback toggle_theme();

        ScrollView {
            width: 100%;
            height: 100%;

            VerticalBox {
                padding: 32px;
                alignment: start;

            Rectangle {
                min-width: 240px;
                min-height: 38px;
                border-radius: 8px;
                background: root.dark_mode ? rgb(59, 130, 246) : rgb(37, 99, 235);
                theme_touch := TouchArea { clicked => { root.toggle_theme(); } }
                Text {
                    text: root.dark_mode ? "Switch to Light (gray-50)" : "Switch to Dark (gray-950)";
                    color: white;
                    font-size: 13px;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }

            Rectangle { height: 10px; }

            Text {
                text: "Rustwind + Slint Demo";
                font-size: 28px;
                color: rgb(248, 250, 252);
                font-weight: 700;
                horizontal-alignment: center;
            }

            Text {
                text: "Type-safe styling for Rust GUI applications";
                font-size: 16px;
                color: rgb(148, 163, 184);
                horizontal-alignment: center;
            }

            Rectangle { height: 24px; }

            Rectangle {
                background: rgb(30, 41, 59);
                border-radius: 8px;
                min-height: 120px;

                VerticalBox {
                    padding: 24px;
                    alignment: center;

                    Text {
                        text: "Counter";
                        font-size: 14px;
                        color: rgb(148, 163, 184);
                        horizontal-alignment: center;
                    }

                    Text {
                        text: root.counter;
                        font-size: 48px;
                        color: rgb(248, 250, 252);
                        font-weight: 700;
                        horizontal-alignment: center;
                    }
                }
            }

            Rectangle { height: 24px; }

            HorizontalBox {
                spacing: 16px;
                alignment: center;

                // Primary button - Increment
                Rectangle {
                    background: touch1.pressed ? rgb(37, 99, 235) : touch1.has-hover ? rgb(59, 130, 246) : rgb(59, 130, 246);
                    border-radius: 6px;
                    min-width: 120px;
                    min-height: 44px;

                    touch1 := TouchArea {
                        clicked => { root.increment(); }
                    }
                    Text {
                        text: "Increment";
                        color: rgb(255, 255, 255);
                        font-size: 14px;
                        font-weight: 500;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }

                // Secondary button - Decrement
                Rectangle {
                    background: touch2.pressed ? rgb(229, 231, 235) : touch2.has-hover ? rgb(243, 244, 246) : rgb(243, 244, 246);
                    border-radius: 6px;
                    min-width: 120px;
                    min-height: 44px;

                    touch2 := TouchArea {
                        clicked => { root.decrement(); }
                    }
                    Text {
                        text: "Decrement";
                        color: rgb(17, 24, 39);
                        font-size: 14px;
                        font-weight: 500;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }

                // Danger button - Reset
                Rectangle {
                    background: touch3.pressed ? rgb(220, 38, 38) : touch3.has-hover ? rgb(239, 68, 68) : rgb(239, 68, 68);
                    border-radius: 6px;
                    min-width: 120px;
                    min-height: 44px;

                    touch3 := TouchArea {
                        clicked => { root.reset(); }
                    }
                    Text {
                        text: "Reset";
                        color: rgb(255, 255, 255);
                        font-size: 14px;
                        font-weight: 500;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
            }

            Rectangle { height: 32px; }

            Text {
                text: "Color Palette";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }

            Rectangle { height: 12px; }

            Text {
                text: "All Palette Families (500)";
                font-size: 14px;
                color: rgb(148, 163, 184);
                horizontal-alignment: center;
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(100, 116, 139); border-radius: 6px; min-width: 28px; min-height: 28px; } // slate
                Rectangle { background: rgb(107, 114, 128); border-radius: 6px; min-width: 28px; min-height: 28px; } // gray
                Rectangle { background: rgb(113, 113, 122); border-radius: 6px; min-width: 28px; min-height: 28px; } // zinc
                Rectangle { background: rgb(115, 115, 115); border-radius: 6px; min-width: 28px; min-height: 28px; } // neutral
                Rectangle { background: rgb(120, 113, 108); border-radius: 6px; min-width: 28px; min-height: 28px; } // stone
                Rectangle { background: rgb(239, 68, 68); border-radius: 6px; min-width: 28px; min-height: 28px; } // red
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(249, 115, 22); border-radius: 6px; min-width: 28px; min-height: 28px; } // orange
                Rectangle { background: rgb(245, 158, 11); border-radius: 6px; min-width: 28px; min-height: 28px; } // amber
                Rectangle { background: rgb(234, 179, 8); border-radius: 6px; min-width: 28px; min-height: 28px; } // yellow
                Rectangle { background: rgb(132, 204, 22); border-radius: 6px; min-width: 28px; min-height: 28px; } // lime
                Rectangle { background: rgb(34, 197, 94); border-radius: 6px; min-width: 28px; min-height: 28px; } // green
                Rectangle { background: rgb(16, 185, 129); border-radius: 6px; min-width: 28px; min-height: 28px; } // emerald
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(20, 184, 166); border-radius: 6px; min-width: 28px; min-height: 28px; } // teal
                Rectangle { background: rgb(6, 182, 212); border-radius: 6px; min-width: 28px; min-height: 28px; } // cyan
                Rectangle { background: rgb(14, 165, 233); border-radius: 6px; min-width: 28px; min-height: 28px; } // sky
                Rectangle { background: rgb(59, 130, 246); border-radius: 6px; min-width: 28px; min-height: 28px; } // blue
                Rectangle { background: rgb(99, 102, 241); border-radius: 6px; min-width: 28px; min-height: 28px; } // indigo
                Rectangle { background: rgb(139, 92, 246); border-radius: 6px; min-width: 28px; min-height: 28px; } // violet
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(168, 85, 247); border-radius: 6px; min-width: 28px; min-height: 28px; } // purple
                Rectangle { background: rgb(217, 70, 239); border-radius: 6px; min-width: 28px; min-height: 28px; } // fuchsia
                Rectangle { background: rgb(236, 72, 153); border-radius: 6px; min-width: 28px; min-height: 28px; } // pink
                Rectangle { background: rgb(244, 63, 94); border-radius: 6px; min-width: 28px; min-height: 28px; } // rose
            }

            Rectangle { height: 28px; }

            Text {
                text: "Spacing Scale (0-96, px)";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 6px;
                alignment: center;
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 6px; min-height: 10px; }   // s0/s1
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 10px; min-height: 10px; }  // s2
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 16px; min-height: 10px; }  // s4
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 24px; min-height: 10px; }  // s6
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 32px; min-height: 10px; }  // s8
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 48px; min-height: 10px; }  // s12
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 64px; min-height: 10px; }  // s16
                Rectangle { background: rgb(59, 130, 246); border-radius: 3px; min-width: 96px; min-height: 10px; }  // s24
            }

            Rectangle { height: 28px; }

            Text {
                text: "Border Radius";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(139, 92, 246); border-radius: 0px; min-width: 52px; min-height: 30px; }
                Rectangle { background: rgb(139, 92, 246); border-radius: 4px; min-width: 52px; min-height: 30px; }
                Rectangle { background: rgb(139, 92, 246); border-radius: 8px; min-width: 52px; min-height: 30px; }
                Rectangle { background: rgb(139, 92, 246); border-radius: 12px; min-width: 52px; min-height: 30px; }
                Rectangle { background: rgb(139, 92, 246); border-radius: 18px; min-width: 52px; min-height: 30px; }
                Rectangle { background: rgb(139, 92, 246); border-radius: 999px; min-width: 52px; min-height: 30px; }
            }

            Rectangle { height: 28px; }

            Text {
                text: "Typography - Font Sizes";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }

            Rectangle { height: 8px; }

            VerticalBox {
                alignment: start;
                Text { text: "xs (12px)"; font-size: 12px; color: rgb(226, 232, 240); }
                Text { text: "sm (14px)"; font-size: 14px; color: rgb(226, 232, 240); }
                Text { text: "base (16px)"; font-size: 16px; color: rgb(226, 232, 240); }
                Text { text: "lg (18px)"; font-size: 18px; color: rgb(226, 232, 240); }
                Text { text: "xl (20px)"; font-size: 20px; color: rgb(226, 232, 240); }
                Text { text: "2xl (24px)"; font-size: 24px; color: rgb(226, 232, 240); }
            }

            Rectangle { height: 16px; }

            Text {
                text: "Typography - Font Weights";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(15, 23, 42); border-radius: 4px; min-width: 74px; min-height: 26px; Text { text: "thin"; color: white; horizontal-alignment: center; vertical-alignment: center; } }
                Rectangle { background: rgb(30, 41, 59); border-radius: 4px; min-width: 74px; min-height: 26px; Text { text: "normal"; color: white; horizontal-alignment: center; vertical-alignment: center; } }
                Rectangle { background: rgb(51, 65, 85); border-radius: 4px; min-width: 74px; min-height: 26px; Text { text: "semibold"; color: white; horizontal-alignment: center; vertical-alignment: center; } }
                Rectangle { background: rgb(100, 116, 139); border-radius: 4px; min-width: 74px; min-height: 26px; Text { text: "bold"; color: white; horizontal-alignment: center; vertical-alignment: center; } }
                Rectangle { background: rgb(203, 213, 225); border-radius: 4px; min-width: 74px; min-height: 26px; Text { text: "black"; color: rgb(15, 23, 42); horizontal-alignment: center; vertical-alignment: center; } }
            }

            Rectangle { height: 28px; }

            Text {
                text: "Box Shadows";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }

            Rectangle { height: 8px; }

            HorizontalBox {
                spacing: 16px;
                alignment: center;
                Rectangle {
                    min-width: 80px; min-height: 46px; border-radius: 8px; background: rgb(30, 41, 59);
                    Rectangle { x: 3px; y: 3px; min-width: 74px; min-height: 40px; border-radius: 8px; background: rgba(0, 0, 0, 0.20); }
                    Rectangle { x: 0px; y: 0px; min-width: 74px; min-height: 40px; border-radius: 8px; background: white; Text { text: "sm"; color: rgb(51, 65, 85); horizontal-alignment: center; vertical-alignment: center; } }
                }
                Rectangle {
                    min-width: 80px; min-height: 46px; border-radius: 8px; background: rgb(30, 41, 59);
                    Rectangle { x: 5px; y: 5px; min-width: 74px; min-height: 40px; border-radius: 8px; background: rgba(0, 0, 0, 0.28); }
                    Rectangle { x: 0px; y: 0px; min-width: 74px; min-height: 40px; border-radius: 8px; background: white; Text { text: "md"; color: rgb(51, 65, 85); horizontal-alignment: center; vertical-alignment: center; } }
                }
                Rectangle {
                    min-width: 80px; min-height: 46px; border-radius: 8px; background: rgb(30, 41, 59);
                    Rectangle { x: 7px; y: 7px; min-width: 74px; min-height: 40px; border-radius: 8px; background: rgba(0, 0, 0, 0.34); }
                    Rectangle { x: 0px; y: 0px; min-width: 74px; min-height: 40px; border-radius: 8px; background: white; Text { text: "lg"; color: rgb(51, 65, 85); horizontal-alignment: center; vertical-alignment: center; } }
                }
                Rectangle {
                    min-width: 80px; min-height: 46px; border-radius: 8px; background: rgb(30, 41, 59);
                    Rectangle { x: 10px; y: 10px; min-width: 74px; min-height: 40px; border-radius: 8px; background: rgba(0, 0, 0, 0.40); }
                    Rectangle { x: 0px; y: 0px; min-width: 74px; min-height: 40px; border-radius: 8px; background: white; Text { text: "xl"; color: rgb(51, 65, 85); horizontal-alignment: center; vertical-alignment: center; } }
                }
            }

            Rectangle { height: 28px; }

            Text {
                text: "CSS Output Example";
                font-size: 20px;
                color: rgb(248, 250, 252);
                font-weight: 600;
                horizontal-alignment: center;
            }
            Rectangle { height: 8px; }
                Rectangle {
                    background: rgb(2, 6, 23);
                    border-radius: 8px;
                    min-height: 48px;
                    Text {
                        text: "padding: 0.5rem 1rem; background-color: #3b82f6; border-radius: 0.375rem";
                        color: rgb(74, 222, 128);
                        font-size: 12px;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let demo = DemoWindow::new()?;

    // Get colors from rustwind to verify
    let blue_500 = rustwind::backends::SlintColors::blue_500();
    let red_500 = rustwind::backends::SlintColors::red_500();
    let green_500 = rustwind::backends::SlintColors::green_500();

    println!("Rustwind Slint Demo");
    println!(
        "Blue 500: R={}, G={}, B={}",
        blue_500.red(),
        blue_500.green(),
        blue_500.blue()
    );
    println!(
        "Red 500: R={}, G={}, B={}",
        red_500.red(),
        red_500.green(),
        red_500.blue()
    );
    println!(
        "Green 500: R={}, G={}, B={}",
        green_500.red(),
        green_500.green(),
        green_500.blue()
    );

    {
        let demo_weak = demo.as_weak();
        demo.on_increment(move || {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_counter(demo.get_counter() + 1);
        });
    }

    {
        let demo_weak = demo.as_weak();
        demo.on_decrement(move || {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_counter(demo.get_counter() - 1);
        });
    }

    {
        let demo_weak = demo.as_weak();
        demo.on_reset(move || {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_counter(0);
        });
    }

    {
        let demo_weak = demo.as_weak();
        demo.on_toggle_theme(move || {
            let demo = demo_weak.upgrade().unwrap();
            demo.set_dark_mode(!demo.get_dark_mode());
        });
    }

    demo.run()
}
