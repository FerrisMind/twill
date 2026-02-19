<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/English-5B7CFA" alt="English"></a>
  <a href="README.RU.md"><img src="https://img.shields.io/badge/Русский-232323" alt="Русский"></a>
  <a href="README.PT_BR.md"><img src="https://img.shields.io/badge/Português_BR-232323" alt="Português"></a>
</p>

---

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/twill/master/assets/icon.svg" width="128" alt="Twill Logo">
  <h1 align="center">Twill</h1>
</p>

<p align="center">
  <b>Idiomatic Rust styling library inspired by Tailwind CSS</b><br>
  <i>Type-safe, composable styles for native GUI applications</i>
</p>

<p align="center">
  <a href="https://github.com/FerrisMind/twill/blob/main/LICENSE"><img src="https://img.shields.io/github/license/FerrisMind/twill" alt="License"></a>
  <a href="https://crates.io/crates/twill"><img src="https://img.shields.io/crates/v/twill" alt="Crates.io"></a>
  <a href="https://docs.rs/twill"><img src="https://img.shields.io/docsrs/twill" alt="Docs.rs"></a>
  <a href="https://github.com/FerrisMind/twill/stargazers"><img src="https://img.shields.io/github/stars/FerrisMind/twill?logo=github" alt="GitHub Stars"></a>
</p>

---

## 📚 Table of Contents

- [What is Twill?](#-what-is-twill)
- [Key Features](#-key-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Design Tokens](#-design-tokens)
- [Style Builder](#-style-builder)
- [Components](#-components)
- [Backend Support](#-backend-support)
- [Examples](#-examples)
- [API Reference](#-api-reference)
- [mdBook Docs](#-mdbook-docs)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ What is Twill?

Twill is a styling library for Rust that brings the best ideas from Tailwind CSS to native GUI development:

- **Design Tokens** — type-safe base values (colors, spacing, sizes)
- **Utility-first** — composable atomic styles
- **Component Variants** — pre-built component variants

But implements them through **Rust types instead of CSS classes**!

```rust
use twill::{Style, Color, Scale, Spacing, Padding, BorderRadius, ToCss};

let button_style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .text_color(Color::slate(Scale::S50))
    .rounded(BorderRadius::Md)
    .to_css();

// Result: "padding: 0.5rem 1rem; background-color: #3b82f6; color: #f8fafc; border-radius: 0.375rem"
```

## 🚀 Key Features

| Feature | Description |
|---------|-------------|
| ✅ **Type-safe** | Impossible to specify invalid colors or sizes |
| ✅ **IDE Autocomplete** | All available options suggested by your IDE |
| ✅ **Compile-time checks** | Style errors caught at compile time |
| ✅ **Composable** | Styles can be combined and reused |
| ✅ **Multi-backend** | CSS, egui, iced, slint support |
| ✅ **Zero runtime cost** | All styles computed at compile time |

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
twill = "0.1"

# Optional: enable backend support
twill = { version = "0.1", features = ["egui"] }   # For egui
twill = { version = "0.1", features = ["iced"] }   # For iced
twill = { version = "0.1", features = ["slint"] }  # For slint
```

## 🎯 Quick Start

### Basic Style Builder

```rust
use twill::{
    Style, Color, Scale, Spacing, Padding, Margin, 
    BorderRadius, Shadow, ToCss
};

// Create a card style
let card = Style::new()
    .padding(Padding::all(Spacing::S6))
    .bg(Color::white())
    .rounded(BorderRadius::Lg)
    .shadow(Shadow::Md);

println!("{}", card.to_css());
// "padding: 1.5rem; background-color: #ffffff; border-radius: 0.5rem; box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1)"
```

### Flex Layouts

```rust
// Centered column
let centered = Style::centered_col()
    .gap(Spacing::S4)
    .padding(Padding::all(Spacing::S8));

// Flex row
let row = Style::flex_row()
    .gap(Spacing::S2)
    .justify(JustifyContent::SpaceBetween);
```

### Pre-built Components

```rust
use twill::{Button, ToCss};

// Button variants
let primary = Button::primary().to_css();
let outline = Button::outline().to_css();
let destructive = Button::destructive().to_css();

// Button sizes
let small = Button::primary().sm().to_css();
let large = Button::primary().lg().to_css();
let full_width = Button::primary().full_width().to_css();
```

## 🎨 Design Tokens

### Colors

Full Tailwind CSS color palette with type-safe scale values:

```rust
use twill::{Color, Scale};

// Color families
Color::slate(Scale::S500)    // #64748b
Color::gray(Scale::S500)     // #6b7280
Color::red(Scale::S500)      // #ef4444
Color::orange(Scale::S500)   // #f97316
Color::blue(Scale::S500)     // #3b82f6
Color::green(Scale::S500)    // #22c55e
Color::purple(Scale::S500)   // #a855f7
Color::pink(Scale::S500)     // #ec4899

// Special colors
Color::white()               // #ffffff
Color::black()               // #000000
Color::transparent()         // transparent

// Scale values: S50, S100, S200, S300, S400, S500, S600, S700, S800, S900, S950
```

### Spacing

```rust
use twill::Spacing;

Spacing::S0   // 0
Spacing::S1   // 0.25rem (4px)
Spacing::S2   // 0.5rem  (8px)
Spacing::S4   // 1rem    (16px)
Spacing::S6   // 1.5rem  (24px)
Spacing::S8   // 2rem    (32px)
Spacing::S12  // 3rem    (48px)
Spacing::S16  // 4rem    (64px)
// ... up to S96
```

### Border Radius

```rust
use twill::BorderRadius;

BorderRadius::None  // 0
BorderRadius::Sm    // 0.125rem
BorderRadius::Md    // 0.375rem
BorderRadius::Lg    // 0.5rem
BorderRadius::Xl    // 0.75rem
BorderRadius::Full  // 9999px
```

### Shadows

```rust
use twill::Shadow;

Shadow::Sm   // Small shadow
Shadow::Md   // Medium shadow
Shadow::Lg   // Large shadow
Shadow::Xl   // Extra large shadow
Shadow::None // No shadow
```

### Motion (Tailwind-aligned)

```rust
use twill::{Style, TransitionDuration, Easing, AnimationToken};

let motion = Style::new()
    .transition_property("opacity, transform")
    .transition_duration(TransitionDuration::Ms300)
    .transition_ease(Easing::InOut)
    .animate(AnimationToken::Pulse);
```

### Semantic Variables (Optional, shadcn-style)

```rust
use twill::{SemanticColor, SemanticThemeVars};

let semantic_bg = SemanticColor::Background.to_css(); // "var(--background)"
let css_vars = SemanticThemeVars::shadcn_neutral().to_css_variables(); // :root + .dark blocks
let bg_dark = SemanticThemeVars::shadcn_neutral()
    .resolve(SemanticColor::Background, true)
    .unwrap(); // Color::gray(Scale::S950)
```

## 🔧 Style Builder

The `Style` struct provides a fluent API for composing styles:

```rust
use twill::{Style, Color, Scale, Spacing, Padding, Margin, BorderRadius, Shadow};

let style = Style::new()
    // Layout
    .display(Display::Flex)
    .position(Position::Relative)
    .z_index(ZIndex::S10)
    
    // Flex/Grid
    .flex(FlexContainer::centered_col())
    .gap(Spacing::S4)
    
    // Spacing
    .padding(Padding::all(Spacing::S4))
    .margin(Margin::symmetric(Spacing::S2, Spacing::S4))
    
    // Size
    .width(Width::full())
    .height(Height::auto())
    
    // Background
    .bg(Color::blue(Scale::S500))
    .opacity(0.9)
    
    // Border
    .rounded(BorderRadius::Md)
    .border(BorderWidth::S1, BorderStyle::Solid, Color::gray(Scale::S200))
    
    // Shadow
    .shadow(Shadow::Lg)
    
    // Typography
    .text_size(FontSize::Lg)
    .font_weight(FontWeight::Bold)
    .text_color(Color::slate(Scale::S900));
```

## 🧩 Components

### Button

```rust
use twill::{Button, ButtonVariant, ButtonSize, ToCss};

// Variants
Button::primary()      // Solid blue background
Button::secondary()    // Gray background
Button::outline()      // Transparent with border
Button::ghost()        // Transparent, no border
Button::destructive()  // Red background
Button::link()         // Link style

// Sizes
Button::primary().sm()           // Small
Button::primary()                // Medium (default)
Button::primary().lg()           // Large
Button::primary().icon()         // Square icon button

// Modifiers
Button::primary().disabled()     // 50% opacity
Button::primary().full_width()   // Width: 100%
```

## 🔌 Backend Support

Twill supports multiple GUI frameworks through feature flags:

| Backend | Feature | Status | Description |
|---------|---------|:------:|-------------|
| CSS | — | ✅ | Default, outputs CSS strings |
| egui | `egui` | ✅ | Native Rust GUI |
| iced | `iced` | ✅ | Cross-platform GUI |
| slint | `slint` | ✅ | Declarative UI toolkit |

### Using with egui

```rust
use twill::{Style, Color, Scale, Spacing, Padding};

#[cfg(feature = "egui")]
fn show_button(ui: &mut egui::Ui) {
    let style = Style::new()
        .padding(Padding::all(Spacing::S4))
        .bg(Color::blue(Scale::S500));
    
    // Convert to egui style
    let egui_style = twill::backends::egui::convert(&style);
    // Apply to egui widgets...
}
```

## 📝 Examples

Run the examples:

```bash
# Basic demo (outputs CSS)
cargo run --example demo

# egui demo
cargo run --example demo-egui --features egui

# iced demo
cargo run --example demo-iced --features iced

# slint demo
cargo run --example demo-slint --features slint
```

## 📖 API Reference

Full API documentation is available at [docs.rs/twill](https://docs.rs/twill).

### Core Traits

| Trait | Description |
|-------|-------------|
| `ToCss` | Convert style to CSS string |
| `Merge` | Combine two styles |
| `ComputeValue` | Compute final value |

### Main Types

| Type | Description |
|------|-------------|
| `Style` | Main style builder |
| `Button` | Button component |
| `Color` | Color values |
| `Spacing` | Spacing scale |
| `Padding` | Padding utilities |
| `Margin` | Margin utilities |
| `BorderRadius` | Border radius values |

## 📚 mdBook Docs

Documentation is built with `mdbook` from `docs/` sources.

- Source: [`docs/`](docs/)
- Config: [`docs/book.toml`](docs/book.toml)
- Build output: `mdbook-build/`

Commands:

```bash
cd docs
mdbook build
mdbook serve --open
mdbook test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

MIT License — see [LICENSE](LICENSE)

Copyright (c) 2024 FerrisMind

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/FerrisMind">FerrisMind</a>
</p>
