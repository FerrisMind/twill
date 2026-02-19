//! Slint demo showcasing rustwind styling.

slint::slint! {
    import { Button, VerticalBox, HorizontalBox } from "std-widgets.slint";
    
    export component DemoWindow inherits Window {
        title: "Rustwind - Slint Demo";
        background: rgb(15, 23, 42);
        preferred-width: 800px;
        preferred-height: 600px;
        
        in-out property <int> counter: 0;
        callback increment();
        callback decrement();
        callback reset();
        
        VerticalBox {
            padding: 32px;
            alignment: start;
            
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
            
            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(219, 234, 254); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(147, 197, 253); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(59, 130, 246); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(29, 78, 216); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(30, 58, 138); border-radius: 6px; min-width: 40px; min-height: 40px; }
            }
            
            Rectangle { height: 8px; }
            
            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(254, 226, 226); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(252, 165, 165); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(239, 68, 68); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(185, 28, 28); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(127, 29, 29); border-radius: 6px; min-width: 40px; min-height: 40px; }
            }
            
            Rectangle { height: 8px; }
            
            HorizontalBox {
                spacing: 8px;
                alignment: center;
                Rectangle { background: rgb(220, 252, 231); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(134, 239, 172); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(34, 197, 94); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(21, 128, 61); border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: rgb(20, 83, 45); border-radius: 6px; min-width: 40px; min-height: 40px; }
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
    println!("Blue 500: R={}, G={}, B={}", blue_500.red(), blue_500.green(), blue_500.blue());
    println!("Red 500: R={}, G={}, B={}", red_500.red(), red_500.green(), red_500.blue());
    println!("Green 500: R={}, G={}, B={}", green_500.red(), green_500.green(), green_500.blue());
    
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
    
    demo.run()
}