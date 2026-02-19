//! Demo example showing rustwind capabilities.

use rustwind::{
    Style, Color, Scale, Spacing, Padding, Margin, BorderRadius, 
    FontSize, FontWeight, Shadow, ToCss, Button,
};

fn main() {
    println!("=== Rustwind Demo ===\n");
    
    // 1. Basic style builder
    println!("1. Basic Style Builder:");
    let card = Style::new()
        .padding(Padding::all(Spacing::S6))
        .bg(Color::white())
        .rounded(BorderRadius::Lg)
        .shadow(Shadow::Md);
    println!("   Card: {}\n", card.to_css());
    
    // 2. Flex layout
    println!("2. Flex Layout:");
    let flex_center = Style::centered_col()
        .gap(Spacing::S4)
        .padding(Padding::all(Spacing::S8));
    println!("   Centered: {}\n", flex_center.to_css());
    
    // 3. Button variants
    println!("3. Button Variants:");
    println!("   Primary: {}", Button::primary().to_css());
    println!("   Secondary: {}", Button::secondary().to_css());
    println!("   Outline: {}", Button::outline().to_css());
    println!("   Ghost: {}", Button::ghost().to_css());
    println!("   Destructive: {}", Button::destructive().to_css());
    println!();
    
    // 4. Button sizes
    println!("4. Button Sizes:");
    println!("   Small: {}", Button::primary().sm().to_css());
    println!("   Medium: {}", Button::primary().to_css());
    println!("   Large: {}", Button::primary().lg().to_css());
    println!();
    
    // 5. Color palette
    println!("5. Color Palette:");
    println!("   Blue 500: {}", Color::blue(Scale::S500).to_css());
    println!("   Red 500: {}", Color::red(Scale::S500).to_css());
    println!("   Green 500: {}", Color::green(Scale::S500).to_css());
    println!("   Slate 900: {}", Color::slate(Scale::S900).to_css());
    println!();
    
    // 6. Typography
    println!("6. Typography:");
    let text_style = Style::new()
        .text_size(FontSize::Xl)
        .font_weight(FontWeight::Bold)
        .text_color(Color::slate(Scale::S900));
    println!("   Heading: {}\n", text_style.to_css());
    
    // 7. Complex component
    println!("7. Complex Component (Card):");
    let complex_card = Style::new()
        .display(rustwind::Display::Flex)
        .flex(rustwind::FlexContainer::col().gap(Spacing::S4))
        .padding(Padding::all(Spacing::S6))
        .bg(Color::white())
        .rounded(BorderRadius::Xl)
        .shadow(Shadow::Lg)
        .border(rustwind::BorderWidth::S1, rustwind::BorderStyle::Solid, Color::gray(Scale::S200));
    println!("   {}\n", complex_card.to_css());
    
    println!("=== Demo Complete ===");
}