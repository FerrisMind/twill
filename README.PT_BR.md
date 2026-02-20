<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/English-232323" alt="English"></a>
  <a href="README.RU.md"><img src="https://img.shields.io/badge/Русский-232323" alt="Русский"></a>
  <a href="README.PT_BR.md"><img src="https://img.shields.io/badge/Português_BR-3ABF7A" alt="Português"></a>
</p>

---

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/twill/master/assets/icon.svg" width="128" alt="Twill Logo">
  <h1 align="center">Twill</h1>
</p>

<p align="center">
  <b>Biblioteca de estilização idiomática para Rust inspirada no Tailwind CSS</b><br>
  <i>Estilos type-safe e compostos para aplicações GUI nativas</i>
</p>

<p align="center">
  <a href="https://github.com/FerrisMind/twill/blob/main/LICENSE"><img src="https://img.shields.io/github/license/FerrisMind/twill" alt="Licença"></a>
  <a href="https://crates.io/crates/twill"><img src="https://img.shields.io/crates/v/twill" alt="Crates.io"></a>
  <a href="https://docs.rs/twill"><img src="https://img.shields.io/docsrs/twill" alt="Docs.rs"></a>
  <a href="https://github.com/FerrisMind/twill/stargazers"><img src="https://img.shields.io/github/stars/FerrisMind/twill?logo=github" alt="GitHub Stars"></a>
</p>

---

## 📚 Índice

- [O que é Twill?](#-o-que-é-twill)
- [Principais Recursos](#-principais-recursos)
- [Instalação](#-instalação)
- [Início Rápido](#-início-rápido)
- [Design Tokens](#-design-tokens)
- [Construtor de Estilos](#-construtor-de-estilos)
- [Componentes](#-componentes)
- [Suporte de Backends](#-suporte-de-backends)
- [Exemplos](#-exemplos)
- [Referência da API](#-referência-da-api)
- [Contribuindo](#-contribuindo)
- [Licença](#-licença)

## ✨ O que é Twill?

Twill é uma biblioteca de estilização para Rust que traz as melhores ideias do Tailwind CSS para o desenvolvimento de GUI nativo:

- **Design Tokens** — valores base type-safe (cores, espaçamentos, tamanhos)
- **Utility-first** — estilos atômicos compostos
- **Variantes de Componentes** — variantes de componentes pré-construídas

Mas implementa através de **tipos Rust em vez de classes CSS**!

```rust
use twill::{Style, Color, Scale, Spacing, Padding, BorderRadius, ToCss};

let button_style = Style::new()
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .bg(Color::blue(Scale::S500))
    .text_color(Color::slate(Scale::S50))
    .rounded(BorderRadius::Md)
    .to_css();

// Resultado: "padding: 0.5rem 1rem; background-color: #3b82f6; color: #f8fafc; border-radius: 0.375rem"
```

## 🚀 Principais Recursos

| Recurso | Descrição |
|---------|-----------|
| ✅ **Type-safe** | Impossível especificar cores ou tamanhos inválidos |
| ✅ **Autocomplete no IDE** | Todas as opções disponíveis são sugeridas pelo IDE |
| ✅ **Verificações em tempo de compilação** | Erros de estilo detectados em tempo de compilação |
| ✅ **Componível** | Estilos podem ser combinados e reutilizados |
| ✅ **Multi-backend** | Suporte a CSS, egui, iced, slint |
| ✅ **Custo zero em runtime** | Todos os estilos computados em tempo de compilação |

## 📦 Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
twill = "0.1"

# Opcional: habilitar suporte de backend
twill = { version = "0.1", features = ["egui"] }   # Para egui
twill = { version = "0.1", features = ["iced"] }   # Para iced
twill = { version = "0.1", features = ["slint"] }  # Para slint
```

## 🎯 Início Rápido

### Construtor de Estilos Básico

```rust
use twill::{
    Style, Color, Scale, Spacing, Padding, Margin, 
    BorderRadius, Shadow, ToCss
};

// Criar um estilo de card
let card = Style::new()
    .padding(Padding::all(Spacing::S6))
    .bg(Color::white())
    .rounded(BorderRadius::Lg)
    .shadow(Shadow::Md);

println!("{}", card.to_css());
// "padding: 1.5rem; background-color: #ffffff; border-radius: 0.5rem; box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1)"
```

### Layouts Flex

```rust
// Coluna centralizada
let centered = Style::centered_col()
    .gap(Spacing::S4)
    .padding(Padding::all(Spacing::S8));

// Linha flex
let row = Style::flex_row()
    .gap(Spacing::S2)
    .justify(JustifyContent::SpaceBetween);
```

### Componentes Pré-construídos

```rust
use twill::{Button, ToCss};

// Variantes de botão
let primary = Button::primary().to_css();
let outline = Button::outline().to_css();
let destructive = Button::destructive().to_css();

// Tamanhos de botão
let small = Button::primary().sm().to_css();
let large = Button::primary().lg().to_css();
let full_width = Button::primary().full_width().to_css();
```

## 🎨 Design Tokens

### Cores

Paleta completa de cores do Tailwind CSS com valores de escala type-safe:

```rust
use twill::{Color, Scale};

// Famílias de cores
Color::slate(Scale::S500)    // #64748b
Color::gray(Scale::S500)     // #6b7280
Color::red(Scale::S500)      // #ef4444
Color::orange(Scale::S500)   // #f97316
Color::blue(Scale::S500)     // #3b82f6
Color::green(Scale::S500)    // #22c55e
Color::purple(Scale::S500)   // #a855f7
Color::pink(Scale::S500)     // #ec4899

// Cores especiais
Color::white()               // #ffffff
Color::black()               // #000000
Color::transparent()         // transparent

// Valores de escala: S50, S100, S200, S300, S400, S500, S600, S700, S800, S900, S950
```

### Espaçamentos

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
// ... até S96
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

### Sombras

```rust
use twill::Shadow;

Shadow::Sm   // Sombra pequena
Shadow::Md   // Sombra média
Shadow::Lg   // Sombra grande
Shadow::Xl   // Sombra extra grande
Shadow::None // Sem sombra
```

## 🔧 Construtor de Estilos

A struct `Style` fornece uma API fluente para compor estilos:

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
    
    // Espaçamento
    .padding(Padding::all(Spacing::S4))
    .margin(Margin::symmetric(Spacing::S2, Spacing::S4))
    
    // Tamanho
    .width(Width::full())
    .height(Height::auto())
    
    // Background
    .bg(Color::blue(Scale::S500))
    .opacity(0.9)
    
    // Borda
    .rounded(BorderRadius::Md)
    .border(BorderWidth::S1, BorderStyle::Solid, Color::gray(Scale::S200))
    
    // Sombra
    .shadow(Shadow::Lg)
    
    // Tipografia
    .text_size(FontSize::Lg)
    .font_weight(FontWeight::Bold)
    .text_color(Color::slate(Scale::S900));
```

## 🧩 Componentes

### Botão

```rust
use twill::{Button, ButtonVariant, ButtonSize, ToCss};

// Variantes
Button::primary()      // Fundo azul sólido
Button::secondary()    // Fundo cinza
Button::outline()      // Transparente com borda
Button::ghost()        // Transparente, sem borda
Button::destructive()  // Fundo vermelho
Button::link()         // Estilo de link

// Tamanhos
Button::primary().sm()           // Pequeno
Button::primary()                // Médio (padrão)
Button::primary().lg()           // Grande
Button::primary().icon()         // Botão quadrado de ícone

// Modificadores
Button::primary().disabled()     // 50% de opacidade
Button::primary().full_width()   // Largura: 100%
```

## 🔌 Suporte de Backends

Twill suporta múltiplos frameworks GUI através de feature flags:

| Backend | Feature | Status | Descrição |
|---------|---------|:------:|-----------|
| CSS | — | ✅ | Padrão, gera strings CSS |
| egui | `egui` | ✅ | GUI nativo em Rust |
| iced | `iced` | ✅ | GUI multiplataforma |
| slint | `slint` | ✅ | Toolkit de UI declarativo |

### Demo Iced (Modo Escuro e Claro)

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/twill/master/assets/twill_iced.png" alt="Twill Iced Demo — Modo Escuro e Claro" width="900">
</p>

### Usando com egui

```rust
use twill::{Style, Color, Scale, Spacing, Padding};

#[cfg(feature = "egui")]
fn show_button(ui: &mut egui::Ui) {
    let style = Style::new()
        .padding(Padding::all(Spacing::S4))
        .bg(Color::blue(Scale::S500));
    
    // Converter para estilo egui
    let egui_style = twill::backends::egui::convert(&style);
    // Aplicar aos widgets egui...
}
```

## 📝 Exemplos

Execute os exemplos:

```bash
# Demo básico (gera CSS)
cargo run --example demo

# Demo egui
cargo run --example demo-egui --features egui

# Demo iced
cargo run --example demo-iced --features iced

# Demo slint
cargo run --example demo-slint --features slint
```

## 📖 Referência da API

Documentação completa da API disponível em [docs.rs/twill](https://docs.rs/twill).

### Traits Principais

| Trait | Descrição |
|-------|-----------|
| `ToCss` | Converte estilo para string CSS |
| `Merge` | Combina dois estilos |
| `ComputeValue` | Computa valor final |

### Tipos Principais

| Tipo | Descrição |
|------|-----------|
| `Style` | Construtor de estilos principal |
| `Button` | Componente de botão |
| `Color` | Valores de cores |
| `Spacing` | Escala de espaçamento |
| `Padding` | Utilitários de padding |
| `Margin` | Utilitários de margin |
| `BorderRadius` | Valores de border radius |

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para enviar um Pull Request.

1. Faça um fork do repositório
2. Crie sua branch de feature (`git checkout -b feature/amazing-feature`)
3. Commit suas mudanças (`git commit -m 'Add some amazing feature'`)
4. Push para a branch (`git push origin feature/amazing-feature`)
5. Abra um Pull Request

## 📄 Licença

MIT License — veja [LICENSE](LICENSE)

Copyright (c) 2024 FerrisMind

---

<p align="center">
  Feito com ❤️ por <a href="https://github.com/FerrisMind">FerrisMind</a>
</p>