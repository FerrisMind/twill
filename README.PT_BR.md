<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/English-232323" alt="English"></a>
  <a href="README.RU.md"><img src="https://img.shields.io/badge/Русский-232323" alt="Русский"></a>
  <a href="README.PT_BR.md"><img src="https://img.shields.io/badge/Português_BR-3C9D5B" alt="Português"></a>
</p>

---

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/twill/master/assets/icon.svg" width="128" alt="Twill Logo">
  <h1 align="center">Twill</h1>
</p>

<p align="center">
  <b>Biblioteca idiomática de estilos para Rust inspirada no Tailwind CSS</b><br>
  <i>Estilos tipados e composáveis para GUIs nativas</i>
</p>

## O que é o Twill

Twill é um style system agnóstico de backend para código de UI em Rust. Ele fornece:

- design tokens tipados para cor, espaçamento, tipografia, sombras, motion e aliases semânticos;
- um builder fluente `Style` para composição utility-first;
- camadas de estado como `hover`, `focus`, `focus_visible`, `selected`, `checked`, `open`, `closed`, `data_state` e `aria_state`;
- composição responsiva com `sm`, `md`, `lg`, `xl` e `s2xl`;
- adapters para `egui`, `iced` e `slint`.

O crate **não** inclui componentes de UI como `Button`, `Card` ou `Dialog`, e **não** expõe serialização para CSS.

## Instalação

```toml
[dependencies]
twill = "0.3"

twill = { version = "0.3", features = ["egui"] }
twill = { version = "0.3", features = ["iced"] }
twill = { version = "0.3", features = ["slint"] }
```

## Exemplo rápido

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

## API central

- `Style`
- `twill::prelude::*`
- `SemanticThemeVars`
- `DynamicSemanticTheme`
- `twill::backends::{egui, iced, slint}`

## Desenvolvimento

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" fmt --all --check
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
& "$env:USERPROFILE\.cargo\bin\cargo.exe" clippy -- -D warnings
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test
```

## Licença

MIT License — veja [LICENSE](LICENSE)
