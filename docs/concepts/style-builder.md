# Style Builder

`Style` is the central composition object. You combine typed tokens and utilities, then map the resulting structure to a native backend.

```rust
use twill::prelude::*;

let card = Style::new()
    .bg(Color::white())
    .text_color(Color::gray(Scale::S900))
    .px_px(18.0)
    .pb_rem(1.25)
    .w_var(WidthVar::new("--card-width"))
    .min_h_var(HeightVar::new("--card-min-h"))
    .flex_arbitrary("2_1_auto")
    .transition_custom("filter, transform");
```

Preset tokens are still the default path. Arbitrary/custom values are controlled escape hatches for layout, spacing, color, typography, and motion when a backend-specific edge case is not covered by the standard scale.
