# State And Responsive Layers

Twill keeps interactive and responsive styling at the `Style` layer instead of shipping UI components.

## Stateful styling

```rust
use twill::prelude::*;

let style = Style::new()
    .hover(|style| style.opacity(0.9))
    .focus_visible(|style| style.ring(RingWidth::S2, Color::blue(Scale::S300)))
    .selected(|style| style.bg(Color::blue(Scale::S500)))
    .checked(|style| {
        style.border(
            BorderWidth::S1,
            BorderStyle::Solid,
            Color::green(Scale::S500),
        )
    })
    .open(|style| style.shadow(Shadow::Lg))
    .data_state("state=open", |style| style.text_color(Color::white()))
    .aria_state("selected", |style| style.font_weight(FontWeight::Bold));
```

Supported built-in state layers:

- `hover`
- `focus`
- `focus_visible`
- `active`
- `disabled`
- `selected`
- `checked`
- `open`
- `closed`

Supported arbitrary hooks:

- `data_state("...")`
- `aria_state("...")`

## Responsive styling

```rust
use twill::prelude::*;

let style = Style::new()
    .w(Spacing::S12)
    .sm(|style| style.w(Spacing::S24))
    .lg(|style| style.h(Spacing::S32));

let resolved = style.at_breakpoint(Breakpoint::Lg);
assert_eq!(resolved.width_value(), Some(Width::from(Spacing::S24)));
assert_eq!(resolved.height_value(), Some(Height::from(Spacing::S32)));
```

Available breakpoint helpers:

- `sm`
- `md`
- `lg`
- `xl`
- `s2xl`

You can also attach layers generically with `responsive(Breakpoint::..., ...)`.
