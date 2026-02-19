<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/English-232323" alt="English"></a>
  <a href="README.RU.md"><img src="https://img.shields.io/badge/Русский-D65C5C" alt="Русский"></a>
  <a href="README.PT_BR.md"><img src="https://img.shields.io/badge/Português_BR-232323" alt="Português"></a>
</p>

---

<p align="center">
  <img src="assets/icon.svg" width="128" alt="Twill Logo">
  <h1 align="center">Twill</h1>
</p>

<p align="center">
  <b>Идиоматичная библиотека стилей для Rust, вдохновлённая Tailwind CSS</b><br>
  <i>Типобезопасные, компонуемые стили для нативных GUI-приложений</i>
</p>

<p align="center">
  <a href="https://github.com/FerrisMind/twill/blob/main/LICENSE"><img src="https://img.shields.io/github/license/FerrisMind/twill" alt="Лицензия"></a>
  <a href="https://crates.io/crates/twill"><img src="https://img.shields.io/crates/v/twill" alt="Crates.io"></a>
  <a href="https://docs.rs/twill"><img src="https://img.shields.io/docsrs/twill" alt="Docs.rs"></a>
  <a href="https://github.com/FerrisMind/twill/stargazers"><img src="https://img.shields.io/github/stars/FerrisMind/twill?logo=github" alt="GitHub Stars"></a>
</p>

---

## 📚 Содержание

- [Что такое Twill?](#-что-такое-twill)
- [Ключевые особенности](#-ключевые-особенности)
- [Установка](#-установка)
- [Быстрый старт](#-быстрый-старт)
- [Дизайн-токены](#-дизайн-токены)
- [Построитель стилей](#-построитель-стилей)
- [Компоненты](#-компоненты)
- [Поддержка бэкендов](#-поддержка-бэкендов)
- [Примеры](#-примеры)
- [Справочник API](#-справочник-api)
- [Участие в разработке](#-участие-в-разработке)
- [Лицензия](#-лицензия)

## ✨ Что такое Twill?

Twill — это библиотека стилизации для Rust, которая переносит лучшие идеи Tailwind CSS в разработку нативных GUI-приложений:

- **Дизайн-токены** — типобезопасные базовые значения (цвета, отступы, размеры)
- **Utility-first** — компонуемые атомарные стили
- **Варианты компонентов** — готовые варианты компонентов

Но реализует их через **типы Rust вместо CSS-классов**!

```rust
use twill::{Style, Color, Scale, Spacing, Padding, BorderRadius, ToCss};

let button_style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .text_color(Color::slate(Scale::S50))
    .rounded(BorderRadius::Md)
    .to_css();

// Результат: "padding: 0.5rem 1rem; background-color: #3b82f6; color: #f8fafc; border-radius: 0.375rem"
```

## 🚀 Ключевые особенности

| Особенность | Описание |
|-------------|----------|
| ✅ **Типобезопасность** | Невозможно указать недопустимые цвета или размеры |
| ✅ **Автодополнение в IDE** | Все доступные варианты предлагаются IDE |
| ✅ **Проверки при компиляции** | Ошибки стилей выявляются при компиляции |
| ✅ **Компонуемость** | Стили можно комбинировать и переиспользовать |
| ✅ **Мультибэкенд** | Поддержка CSS, egui, iced, slint |
| ✅ **Нулевые runtime-затраты** | Все стили вычисляются при компиляции |

## 📦 Установка

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
twill = "0.1"

# Опционально: включить поддержку бэкенда
twill = { version = "0.1", features = ["egui"] }   # Для egui
twill = { version = "0.1", features = ["iced"] }   # Для iced
twill = { version = "0.1", features = ["slint"] }  # Для slint
```

## 🎯 Быстрый старт

### Базовый построитель стилей

```rust
use twill::{
    Style, Color, Scale, Spacing, Padding, Margin, 
    BorderRadius, Shadow, ToCss
};

// Создать стиль карточки
let card = Style::new()
    .padding(Padding::all(Spacing::S6))
    .bg(Color::white())
    .rounded(BorderRadius::Lg)
    .shadow(Shadow::Md);

println!("{}", card.to_css());
// "padding: 1.5rem; background-color: #ffffff; border-radius: 0.5rem; box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1)"
```

### Flex-раскладки

```rust
// Центрированная колонка
let centered = Style::centered_col()
    .gap(Spacing::S4)
    .padding(Padding::all(Spacing::S8));

// Flex-строка
let row = Style::flex_row()
    .gap(Spacing::S2)
    .justify(JustifyContent::SpaceBetween);
```

### Готовые компоненты

```rust
use twill::{Button, ToCss};

// Варианты кнопок
let primary = Button::primary().to_css();
let outline = Button::outline().to_css();
let destructive = Button::destructive().to_css();

// Размеры кнопок
let small = Button::primary().sm().to_css();
let large = Button::primary().lg().to_css();
let full_width = Button::primary().full_width().to_css();
```

## 🎨 Дизайн-токены

### Цвета

Полная палитра цветов Tailwind CSS с типобезопасными значениями шкалы:

```rust
use twill::{Color, Scale};

// Цветовые семейства
Color::slate(Scale::S500)    // #64748b
Color::gray(Scale::S500)     // #6b7280
Color::red(Scale::S500)      // #ef4444
Color::orange(Scale::S500)   // #f97316
Color::blue(Scale::S500)     // #3b82f6
Color::green(Scale::S500)    // #22c55e
Color::purple(Scale::S500)   // #a855f7
Color::pink(Scale::S500)     // #ec4899

// Специальные цвета
Color::white()               // #ffffff
Color::black()               // #000000
Color::transparent()         // transparent

// Значения шкалы: S50, S100, S200, S300, S400, S500, S600, S700, S800, S900, S950
```

### Отступы

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
// ... до S96
```

### Скругление границ

```rust
use twill::BorderRadius;

BorderRadius::None  // 0
BorderRadius::Sm    // 0.125rem
BorderRadius::Md    // 0.375rem
BorderRadius::Lg    // 0.5rem
BorderRadius::Xl    // 0.75rem
BorderRadius::Full  // 9999px
```

### Тени

```rust
use twill::Shadow;

Shadow::Sm   // Маленькая тень
Shadow::Md   // Средняя тень
Shadow::Lg   // Большая тень
Shadow::Xl   // Очень большая тень
Shadow::None // Без тени
```

## 🔧 Построитель стилей

Структура `Style` предоставляет fluent API для компоновки стилей:

```rust
use twill::{Style, Color, Scale, Spacing, Padding, Margin, BorderRadius, Shadow};

let style = Style::new()
    // Раскладка
    .display(Display::Flex)
    .position(Position::Relative)
    .z_index(ZIndex::S10)
    
    // Flex/Grid
    .flex(FlexContainer::centered_col())
    .gap(Spacing::S4)
    
    // Отступы
    .padding(Padding::all(Spacing::S4))
    .margin(Margin::symmetric(Spacing::S2, Spacing::S4))
    
    // Размер
    .width(Width::full())
    .height(Height::auto())
    
    // Фон
    .bg(Color::blue(Scale::S500))
    .opacity(0.9)
    
    // Границы
    .rounded(BorderRadius::Md)
    .border(BorderWidth::S1, BorderStyle::Solid, Color::gray(Scale::S200))
    
    // Тень
    .shadow(Shadow::Lg)
    
    // Типографика
    .text_size(FontSize::Lg)
    .font_weight(FontWeight::Bold)
    .text_color(Color::slate(Scale::S900));
```

## 🧩 Компоненты

### Кнопка

```rust
use twill::{Button, ButtonVariant, ButtonSize, ToCss};

// Варианты
Button::primary()      // Сплошной синий фон
Button::secondary()    // Серый фон
Button::outline()      // Прозрачная с границей
Button::ghost()        // Прозрачная, без границы
Button::destructive()  // Красный фон
Button::link()         // Стиль ссылки

// Размеры
Button::primary().sm()           // Маленькая
Button::primary()                // Средняя (по умолчанию)
Button::primary().lg()           // Большая
Button::primary().icon()         // Квадратная кнопка-иконка

// Модификаторы
Button::primary().disabled()     // Прозрачность 50%
Button::primary().full_width()   // Ширина: 100%
```

## 🔌 Поддержка бэкендов

Twill поддерживает несколько GUI-фреймворков через feature flags:

| Бэкенд | Feature | Статус | Описание |
|--------|---------|:------:|----------|
| CSS | — | ✅ | По умолчанию, выводит CSS-строки |
| egui | `egui` | ✅ | Нативный GUI на Rust |
| iced | `iced` | ✅ | Кроссплатформенный GUI |
| slint | `slint` | ✅ | Декларативный UI-тулкит |

### Использование с egui

```rust
use twill::{Style, Color, Scale, Spacing, Padding};

#[cfg(feature = "egui")]
fn show_button(ui: &mut egui::Ui) {
    let style = Style::new()
        .padding(Padding::all(Spacing::S4))
        .bg(Color::blue(Scale::S500));
    
    // Конвертировать в стиль egui
    let egui_style = twill::backends::egui::convert(&style);
    // Применить к виджетам egui...
}
```

## 📝 Примеры

Запуск примеров:

```bash
# Базовый демо (выводит CSS)
cargo run --example demo

# Демо egui
cargo run --example demo-egui --features egui

# Демо iced
cargo run --example demo-iced --features iced

# Демо slint
cargo run --example demo-slint --features slint
```

## 📖 Справочник API

Полная документация API доступна на [docs.rs/twill](https://docs.rs/twill).

### Основные трейты

| Трейт | Описание |
|-------|----------|
| `ToCss` | Конвертировать стиль в CSS-строку |
| `Merge` | Объединить два стиля |
| `ComputeValue` | Вычислить итоговое значение |

### Основные типы

| Тип | Описание |
|-----|----------|
| `Style` | Основной построитель стилей |
| `Button` | Компонент кнопки |
| `Color` | Значения цветов |
| `Spacing` | Шкала отступов |
| `Padding` | Утилиты внутренних отступов |
| `Margin` | Утилиты внешних отступов |
| `BorderRadius` | Значения скругления границ |

## 🤝 Участие в разработке

Приветствуются любые участие! Не стесняйтесь отправлять Pull Request.

1. Сделайте форк репозитория
2. Создайте ветку функции (`git checkout -b feature/amazing-feature`)
3. Зафиксируйте изменения (`git commit -m 'Add some amazing feature'`)
4. Отправьте в ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request

## 📄 Лицензия

MIT License — см. [LICENSE](LICENSE)

Copyright (c) 2024 FerrisMind

---

<p align="center">
  Сделано с ❤️ <a href="https://github.com/FerrisMind">FerrisMind</a>
</p>