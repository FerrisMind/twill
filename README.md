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

## What Twill Is

Twill is a backend-agnostic style system for Rust UI code. It provides:

- typed design tokens for color, spacing, typography, shadows, motion, and semantic theme aliases;
- a fluent `Style` builder for utility-style composition;
- state/style layers such as `hover`, `focus`, `focus_visible`, `selected`, `checked`, `open`, `closed`, `data_*`, and `aria_*`;
- breakpoint-based responsive composition via `sm`, `md`, `lg`, `xl`, and `s2xl`;
- backend adapters for `egui`, `iced`, and `slint`.

Twill does **not** ship UI components such as `Button`, `Card`, or `Dialog`, and it does **not** expose a CSS serialization layer.

## Installation

```toml
[dependencies]
twill = "0.3"

# Optional backend adapters
twill = { version = "0.3", features = ["egui"] }
twill = { version = "0.3", features = ["iced"] }
twill = { version = "0.3", features = ["slint"] }
```

## Quick Start

```rust
use twill::prelude::*;

let style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .text_color(Color::slate(Scale::S50))
    .rounded(BorderRadius::Md)
    .hover(|style| style.opacity(0.9))
    .focus_visible(|style| style.ring_color(Color::blue(Scale::S300)))
    .data_state("state=open", |style| style.shadow(Shadow::Lg))
    .md(|style| style.padding(Padding::all(Spacing::S6)));
```

## Core API

- `Style` is the central style composition type.
- `twill::prelude::*` re-exports the main day-to-day tokens and utilities.
- `SemanticThemeVars` and `DynamicSemanticTheme` provide semantic alias mapping inspired by shadcn theme variables.
- `twill::backends::{egui, iced, slint}` expose typed conversion helpers for each supported runtime.

## State And Responsive Composition

Common state layers:

- `hover`
- `focus`
- `focus_visible`
- `active`
- `disabled`
- `selected`
- `checked`
- `open`
- `closed`
- `data_state("...")`
- `aria_state("...")`

Responsive layers:

- `sm`
- `md`
- `lg`
- `xl`
- `s2xl`
- `Responsive::at_breakpoint(...)`

## Backends

Supported adapters:

- `egui`
- `iced`
- `slint`

Each backend translates Twill tokens and `Style` values into framework-specific primitives without changing the core style model.

## Documentation

- mdBook sources live in [`docs/`](docs/)
- crate documentation is published on [docs.rs/twill](https://docs.rs/twill)

## Development

Useful checks:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" fmt --all --check
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
& "$env:USERPROFILE\.cargo\bin\cargo.exe" clippy -- -D warnings
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test
```

## License

MIT License — see [LICENSE](LICENSE)
