//! Slint demo showcasing rustwind styling.

slint::slint! {
    import { Button, VerticalBox, HorizontalBox } from "std-widgets.slint";
    
    export component DemoWindow inherits Window {
        title: "Rustwind - Slint Demo";
        background: #0f172a; // slate-900
        preferred-width: 800px;
        preferred-height: 600px;
        
        in-out property <int> counter: 0;
        callback increment();
        callback decrement();
        callback reset();
        
        VerticalBox {
            padding: 32px;
            alignment: start;
            
            // Title
            Text {
                text: "Rustwind + Slint Demo";
                font-size: 28px;
                color: #f8fafc; // slate-50
                font-weight: 700;
                horizontal-alignment: center;
            }
            
            // Description
            Text {
                text: "Type-safe styling for Rust GUI applications";
                font-size: 16px;
                color: #94a3b8; // slate-400
                horizontal-alignment: center;
            }
            
            Rectangle { height: 24px; }
            
            // Counter card
            Rectangle {
                background: #1e293b; // slate-800
                border-radius: 8px;
                min-height: 120px;
                
                VerticalBox {
                    padding: 24px;
                    alignment: center;
                    
                    Text {
                        text: "Counter";
                        font-size: 14px;
                        color: #94a3b8;
                        horizontal-alignment: center;
                    }
                    
                    Text {
                        text: root.counter;
                        font-size: 48px;
                        color: #f8fafc;
                        font-weight: 700;
                        horizontal-alignment: center;
                    }
                }
            }
            
            Rectangle { height: 24px; }
            
            // Buttons
            HorizontalBox {
                spacing: 16px;
                alignment: center;
                
                // Primary button
                Rectangle {
                    background: touch1.pressed ? #2563eb : touch1.has-hover ? #3b82f6 : #3b82f6;
                    border-radius: 6px;
                    min-width: 120px;
                    min-height: 44px;
                    
                    touch1 := TouchArea {}
                    Text {
                        text: "Increment";
                        color: #ffffff;
                        font-size: 14px;
                        font-weight: 500;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
                
                // Secondary button
                Rectangle {
                    background: touch2.pressed ? #e5e7eb : touch2.has-hover ? #f3f4f6 : #f3f4f6;
                    border-radius: 6px;
                    min-width: 120px;
                    min-height: 44px;
                    
                    touch2 := TouchArea {}
                    Text {
                        text: "Decrement";
                        color: #111827;
                        font-size: 14px;
                        font-weight: 500;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
                
                // Danger button
                Rectangle {
                    background: touch3.pressed ? #dc2626 : touch3.has-hover ? #ef4444 : #ef4444;
                    border-radius: 6px;
                    min-width: 120px;
                    min-height: 44px;
                    
                    touch3 := TouchArea {}
                    Text {
                        text: "Reset";
                        color: #ffffff;
                        font-size: 14px;
                        font-weight: 500;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
            }
            
            Rectangle { height: 32px; }
            
            // Color palette section
            Text {
                text: "Color Palette";
                font-size: 20px;
                color: #f8fafc;
                font-weight: 600;
                horizontal-alignment: center;
            }
            
            Rectangle { height: 12px; }
            
            // Blue palette
            HorizontalBox {
                spacing: 8px;
                alignment: center;
                
                Rectangle { background: #dbeafe; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #93c5fd; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #3b82f6; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #1d4ed8; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #1e3a8a; border-radius: 6px; min-width: 40px; min-height: 40px; }
            }
            
            Rectangle { height: 8px; }
            
            // Red palette
            HorizontalBox {
                spacing: 8px;
                alignment: center;
                
                Rectangle { background: #fee2e2; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #fca5a5; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #ef4444; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #b91c1c; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #7f1d1d; border-radius: 6px; min-width: 40px; min-height: 40px; }
            }
            
            Rectangle { height: 8px; }
            
            // Green palette
            HorizontalBox {
                spacing: 8px;
                alignment: center;
                
                Rectangle { background: #dcfce7; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #86efac; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #22c55e; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #15803d; border-radius: 6px; min-width: 40px; min-height: 40px; }
                Rectangle { background: #14532d; border-radius: 6px; min-width: 40px; min-height: 40px; }
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