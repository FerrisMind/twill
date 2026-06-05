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
- typed arbitrary/custom-property escape hatches for key utility families, inspired by Tailwind's arbitrary values;
- state/style layers such as `hover`, `focus`, `focus_visible`, `selected`, `checked`, `open`, `closed`, `data_*`, and `aria_*`;
- breakpoint-based responsive composition via `sm`, `md`, `lg`, `xl`, and `s2xl`;
- backend adapters for `egui`, `iced`, and `slint`.

Twill does **not** ship UI components such as `Button`, `Card`, or `Dialog`, and it does **not** expose a CSS serialization layer.

The ecosystem now has five entry points:

- `twill-core` for libraries and applications that only need the backend-agnostic style engine;
- `twill-egui` for egui-only adapter code on top of `twill-core`;
- `twill-iced` for iced-only adapter code on top of `twill-core`;
- `twill-slint` for slint-only adapter code on top of `twill-core`;
- `twill` as the facade crate that re-exports `twill-core` and optionally adds GUI adapters.

Examples and cookbook pages in `main` target the `0.3.x` API only. Legacy demos built around component APIs or CSS serialization belong to the `0.2.x` release line, not this branch.

## Installation

```toml
[dependencies]
twill-core = "0.3"
twill-egui = "0.3"
twill-iced = "0.3"
twill-slint = "0.3"

# or use the facade crate
twill = "0.3"

# Optional backend adapters
twill = { version = "0.3", features = ["egui"] }
twill = { version = "0.3", features = ["iced"] }
twill = { version = "0.3", features = ["slint"] }
```

MSRV: Rust `1.93`.

Backend notes:

- `twill-core` is synchronous and backend-agnostic.
- `twill-egui`, `twill-iced`, and `twill-slint` are thin adapter crates that depend on
  `twill-core` directly.
- `twill` re-exports the full core API and only pulls GUI dependencies when you enable a backend feature.
- `egui` adds conversion helpers for egui types only.
- `iced` adds the Iced adapter and the Unix windowing/runtime feature set used by this crate configuration.
- `slint` adds Slint conversion helpers only when requested.

## Quick Start

```rust
use twill::prelude::core::*;

let style = Style::card()
    .merged(Style::interactive())
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .hover(|style| style.opacity(0.9))
    .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
    .at_md(|style| style.padding(Padding::all(Spacing::S6)));
```

## Prelude Layers

Twill now has a stratified import surface:

- `twill::prelude::core::*` for day-to-day `Style`, tokens, layout, and state builders;
- `twill::prelude::theme::*` for semantic aliases such as `SemanticColor` and theme resolvers;
- `twill::prelude::arbitrary::*` for typed arbitrary/custom-property escape hatches;
- `twill::prelude::traits::*` for generic integration traits like `Merge` and `Responsive`.

`twill::prelude::*` still exists when you explicitly want the full power-user surface.

Typed escape hatches stay in the style layer:

```rust
use twill::prelude::{arbitrary::*, core::*};

let style = Style::new()
    .text_color_arbitrary(ColorValueToken::from_rgb8(248, 250, 252))
    .px_var(PaddingVar::new("--panel-pad-x"))
    .pb_rem(1.25)
    .border_color_var(BorderColorVar::new("--panel-border"))
    .min_w_var(WidthVar::new("--panel-min-w"))
    .tracking_em(0.035)
    .leading_number(1.75)
    .transition_custom("filter, transform")
    .transition_duration_ms(240);
```

## Compositional Presets

The core crate now ships a few reusable presets that stay intentionally small:

- `Style::surface()` for structural spacing, radius, and elevation;
- `Style::card()` for semantic card/background/border defaults;
- `Style::interactive()` for cursor, transition, focus ring, and disabled affordances.

Use them as building blocks and layer app-specific details with `merged(...)` or
`merge_in_place(...)`.

## Core API

- `Style` is the central style composition type.
- `twill-core` is the lightest dependency when you do not need backend adapters.
- `twill-egui`, `twill-iced`, and `twill-slint` are the narrowest way to depend on one backend adapter.
- `twill::prelude::core::*` is the recommended starter import.
- `twill::prelude::{theme::*, arbitrary::*, traits::*}` expand the API surface only when needed.
- `SemanticThemeVars` and `DynamicSemanticTheme` provide semantic alias mapping inspired by shadcn theme variables.
- `Style::resolved_theme(...)`, `resolved_light_theme(...)`, and `resolved_dark_theme(...)` let you
  turn semantic aliases into concrete color tokens before backend conversion.
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
- `data_attr(DataState::..., ...)`
- `aria_attr(AriaAttr::..., ...)`

Responsive layers:

- `sm`
- `md`
- `lg`
- `xl`
- `s2xl`
- `Style::at_breakpoint(...)`

## Backends

Supported adapters:

- `egui`
- `iced`
- `slint`

Each backend translates Twill tokens and `Style` values into framework-specific primitives without changing the core style model.
Use `twill-egui`, `twill-iced`, or `twill-slint` directly when you want one adapter crate; use
`twill` when you want the facade and feature-gated re-exports.

## Documentation

- mdBook sources live in [`docs/`](docs/)
- crate documentation is published on [docs.rs/twill](https://docs.rs/twill)
- release notes live in [CHANGELOG.md](CHANGELOG.md)

## Development

Useful checks:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-features --examples
```

## License

MIT License — see [LICENSE](LICENSE)
