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
- state-слои: `hover`, `focus`, `focus_visible`, `selected`, `checked`, `open`, `closed`, `data_state`, `aria_state`;
- breakpoint-композицию через `sm`, `md`, `lg`, `xl`, `s2xl`;
- адаптеры под `egui`, `iced` и `slint`.

В `Twill` **нет** UI-компонентов вроде `Button`, `Card` или `Dialog`, и **нет** CSS-сериализации.

## Установка

```toml
[dependencies]
twill = "0.3"

# Опциональные адаптеры бэкендов
twill = { version = "0.3", features = ["egui"] }
twill = { version = "0.3", features = ["iced"] }
twill = { version = "0.3", features = ["slint"] }
```

## Быстрый старт

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

## Основной API

- `Style` — центральный тип композиции стилей.
- `twill::prelude::*` — основной импорт для повседневного использования.
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
- `data_state("...")`
- `aria_state("...")`

Responsive-слои:

- `sm`
- `md`
- `lg`
- `xl`
- `s2xl`
- `Responsive::at_breakpoint(...)`

## Бэкенды

Поддерживаются:

- `egui`
- `iced`
- `slint`

Каждый backend переводит токены и `Style` в framework-specific primitives, не смешивая их с компонентной семантикой.

## Документация

- исходники mdBook: [`docs/`](docs/)
- API-документация crate: [docs.rs/twill](https://docs.rs/twill)

## Разработка

Проверки:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" fmt --all --check
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
& "$env:USERPROFILE\.cargo\bin\cargo.exe" clippy -- -D warnings
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test
```

## Лицензия

MIT License — см. [LICENSE](LICENSE)
