# Twill
Twill is an idiomatic Rust styling library inspired by Tailwind CSS.

It gives you:
- type-safe design tokens,
- fluent style composition,
- reusable component variants,
- backend adapters for `egui`, `iced`, and `slint`.

## Who this documentation is for
This guide is for Rust developers who want a consistent styling layer across native GUI frameworks, with optional CSS output and semantic tokens.

## What to read first
1. [Installation](getting-started/installation.md)
2. [Quick Start](getting-started/quick-start.md)
3. [Design Tokens](concepts/tokens.md)
4. [Style Builder](concepts/style-builder.md)
5. [Backends Overview](backends/overview.md)

## Project principles
- Type safety over stringly-typed styles
- Composable utilities over large monolithic style objects
- Optional semantic layer (use it only when you need it)
- Framework-agnostic core with thin backend adapters

