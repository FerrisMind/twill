# Style Builder

`Style` is the central composition object. You combine typed tokens and utilities, then map the resulting structure to a native backend.

```rust
use twill::prelude::{arbitrary::*, core::*};

let card = Style::card()
    .merged(Style::interactive())
    .px_px(18.0)
    .pb_rem(1.25)
    .w_var(WidthVar::new("--card-width"))
    .min_h_var(HeightVar::new("--card-min-h"))
    .flex_arbitrary("2_1_auto")
    .transition_custom("filter, transform");
```

Preset styles are the default path. Arbitrary/custom values are controlled escape hatches for layout, spacing, color, typography, and motion when a backend-specific edge case is not covered by the standard scale.
Compose reusable layers with `merged(...)` or `merge_in_place(...)` instead of inventing framework-specific wrapper structs too early.
