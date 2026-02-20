# Components

Component helpers expose common variants and sizes while returning strongly typed `Style` data.

```rust
use twill::Button;

let primary = Button::primary();
let large = Button::primary().lg();
let outline = Button::outline().sm();
let danger = Button::destructive().full_width();
let disabled = Button::secondary().disabled();
```
