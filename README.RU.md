<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/English-232323" alt="English"></a>
  <a href="README.RU.md"><img src="https://img.shields.io/badge/Русский-D65C5C" alt="Русский"></a>
  <a href="README.PT_BR.md"><img src="https://img.shields.io/badge/Português_BR-232323" alt="Português"></a>
</p>

---

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/twill/master/assets/icon.svg" width="128" alt="Twill Logo">
  <h1 align="center">Twill</h1>
</p>

<p align="center">
  <b>Идиоматичная библиотека стилей для Rust, вдохновлённая Tailwind CSS</b><br>
  <i>Типобезопасные и компонуемые стили для нативных GUI-приложений</i>
</p>

## Что такое Twill

Twill — это backend-agnostic style-system для UI-кода на Rust. Библиотека даёт:

- типизированные дизайн-токены для цветов, отступов, типографики, теней, motion и semantic theme aliases;
- fluent API через `Style` для utility-style композиции;
- typed arbitrary/custom-property escape hatches для ключевых семейств utility API;
- state-слои: `hover`, `focus`, `focus_visible`, `selected`, `checked`, `open`, `closed`, `data_attr`, `aria_attr`;
- breakpoint-композицию через `sm`, `md`, `lg`, `xl`, `s2xl`;
- адаптеры под `egui`, `iced` и `slint`.

В `Twill` **нет** UI-компонентов вроде `Button`, `Card` или `Dialog`, и **нет** CSS-сериализации.

В экосистеме теперь есть несколько слоёв:

- `twill-core` для приложений и библиотек, которым нужен только backend-agnostic style engine;
- `twill-egui`, `twill-iced` и `twill-slint` как узкие adapter crates поверх `twill-core`;
- `twill` как facade crate, который реэкспортирует `twill-core` и эти adapters по feature flags.

## Установка

```toml
[dependencies]
twill-core = "0.3"
twill-egui = "0.3"
twill-iced = "0.3"
twill-slint = "0.3"

# или facade crate
twill = "0.3"

# Опциональные адаптеры бэкендов
twill = { version = "0.3", features = ["egui"] }
twill = { version = "0.3", features = ["iced"] }
twill = { version = "0.3", features = ["slint"] }
```

MSRV: Rust `1.93`.

## Быстрый старт

```rust
use twill::prelude::core::*;

let style = Style::card()
    .merged(Style::interactive())
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .hover(|style| style.opacity(0.9))
    .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
    .at_md(|style| style.padding(Padding::all(Spacing::S6)));
```

## Основной API

- `Style` — центральный тип композиции стилей.
- `twill-core` — самый лёгкий dependency, если backend adapters не нужны.
- `twill-egui`, `twill-iced` и `twill-slint` — самый узкий путь, если нужен только один runtime adapter.
- `twill::prelude::core::*` — рекомендуемый импорт для повседневного использования.
- `twill::prelude::theme::*`, `arbitrary::*` и `traits::*` подключаются по мере необходимости.
- `SemanticThemeVars` и `DynamicSemanticTheme` — semantic alias mapping в духе shadcn theme variables.
- `twill::backends::{egui, iced, slint}` — типизированные адаптеры под конкретные runtime.

## State и responsive composition

Поддерживаемые state-слои:

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

Responsive-слои:

- `sm`
- `md`
- `lg`
- `xl`
- `s2xl`
- `Style::at_breakpoint(...)`

## Бэкенды

Поддерживаются:

- `egui`
- `iced`
- `slint`

Каждый backend переводит токены и `Style` в framework-specific primitives, не смешивая их с компонентной семантикой.
Если facade crate не нужен, можно зависеть напрямую от `twill-egui`, `twill-iced` или `twill-slint`.

## Документация

- исходники mdBook: [`docs/`](docs/)
- API-документация crate: [docs.rs/twill](https://docs.rs/twill)

## Разработка

Проверки:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-features --examples
```

## Лицензия

MIT License — см. [LICENSE](LICENSE)
