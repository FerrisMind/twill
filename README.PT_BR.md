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
- escape hatches tipados para valores arbitrários e custom properties;
- camadas de estado como `hover`, `focus`, `focus_visible`, `selected`, `checked`, `open`, `closed`, `data_attr` e `aria_attr`;
- composição responsiva com `sm`, `md`, `lg`, `xl` e `s2xl`;
- adapters para `egui`, `iced` e `slint`.

O crate **não** inclui componentes de UI como `Button`, `Card` ou `Dialog`, e **não** expõe serialização para CSS.

O ecossistema agora tem uma estrutura em camadas:

- `twill-core` para bibliotecas e apps que só precisam do style engine agnóstico de backend;
- `twill-egui`, `twill-iced` e `twill-slint` como adapter crates finos sobre `twill-core`;
- `twill` como crate facade que reexporta `twill-core` e esses adapters via feature flags.

## Instalação

```toml
[dependencies]
twill-core = "0.3"
twill-egui = "0.3"
twill-iced = "0.3"
twill-slint = "0.3"

# ou use o crate facade
twill = "0.3"

twill = { version = "0.3", features = ["egui"] }
twill = { version = "0.3", features = ["iced"] }
twill = { version = "0.3", features = ["slint"] }
```

MSRV: Rust `1.93`.

## Exemplo rápido

```rust
use twill::prelude::core::*;

let style = Style::card()
    .merged(Style::interactive())
    .padding(Padding::symmetric(Spacing::S2, Spacing::S4))
    .hover(|style| style.opacity(0.9))
    .data_attr(DataState::Open, |style| style.shadow(Shadow::Lg))
    .at_md(|style| style.padding(Padding::all(Spacing::S6)));
```

## API central

- `Style`
- `twill-core`
- `twill-egui`, `twill-iced`, `twill-slint`
- `twill::prelude::core::*`
- `twill::prelude::{theme::*, arbitrary::*, traits::*}`
- `SemanticThemeVars`
- `DynamicSemanticTheme`
- `twill::backends::{egui, iced, slint}`

Se você não quiser o facade crate, pode depender diretamente de `twill-egui`, `twill-iced` ou `twill-slint`.

## Desenvolvimento

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-features --examples
```

## Licença

MIT License — veja [LICENSE](LICENSE)
