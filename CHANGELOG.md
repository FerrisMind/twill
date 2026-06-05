# Changelog

## Unreleased

### Added

- Added `twill::prelude::core::*` as the recommended starter import path.
- Added typed state hooks through `DataState`, `DataAttr`, `AriaAttr`,
  `Style::data_attr(...)`, and `Style::aria_attr(...)`.
- Added readable responsive aliases:
  `at_sm`, `at_md`, `at_lg`, `at_xl`, `at_2xl`.
- Added `Style::background_color(...)` as a discoverable alias for `bg(...)`.
- Added `Style::at_breakpoint(...)` as the concrete day-to-day resolution API.
- Added `CONTRIBUTING.md` and a basic CI workflow for `fmt`, `clippy`, `test`, and `doc`.

### Changed

- Updated README, rustdoc, examples, and mdBook pages to prefer the narrow onboarding path.
- Clarified that `Merge`, `Responsive`, `ThemedStyle`, and `ComputeValue` are primarily infrastructure-facing traits.
- Documented raw `data_state(...)` and `aria_state(...)` as escape hatches instead of the main API.

### Fixed

- Enabled Linux display backends for the optional `iced` backend feature so `--all-features`
  builds work on Unix.
- Enabled Linux display backends for the `eframe` dev-dependency so default test runs work on Unix.
- Cleaned rustdoc comments that produced HTML-tag warnings during `cargo doc`.
