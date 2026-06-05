# Component Recipes

This example shows the most common downstream Twill pattern: small functions that return reusable `Style` recipes instead of framework-specific components.

- File: `examples/07_component_recipes.rs`
- Run:

```bash
cargo run --example component_recipes
```

- Expected output: a terminal dump of reusable button, card, and input style traits such as padding, borders, hover, and focus-visible layers.

Why this exists:
Twill intentionally does not ship `Button`, `Card`, or `Dialog`, so applications need a clear example of how to package shared style logic themselves.
