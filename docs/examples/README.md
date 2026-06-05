# Examples

This section documents the checked-in `examples/` targets for `main` and the `0.3.x` API.

Legacy demos built around component APIs or CSS serialization belong to the `0.2.x` release line and are intentionally not reproduced here.

The examples now follow a user journey:

- `00_quick_start_card.rs` is the shortest copy-paste path into Twill.
- `01_*` through `06_*` isolate the core style, state, responsive, semantic, and arbitrary-value concepts.
- `07_*` through `09_*` show how real applications turn those primitives into reusable recipes, composition layers, and state variants.
- `10_*` through `13_*` cover backend adapters and cross-backend mapping.
- `20_*` through `25_*` show full applications and project structure patterns.

They also split cleanly by runtime cost:

- `00_*` through `09_*`, plus `13_*` and `25_*`, are headless examples intended for fast copy-paste, CI, and docs-style inspection.
- `10_*` through `12_*` are feature-gated examples that trade startup cost for backend-specific adapter coverage.
- `20_*` through `23_*` are visual showcases meant to open native windows and demonstrate the full styling story.

If you only want the highest-signal starting points, begin with:

- `quick_start_card`
- `component_recipes`
- `style_composition`
- `backend_mapping_matrix`
- `design_system_module`
