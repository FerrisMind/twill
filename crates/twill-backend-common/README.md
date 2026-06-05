# twill-backend-common

`twill-backend-common` contains small shared helper types for Twill adapter crates.

It is primarily an internal ecosystem crate used by:

- `twill-egui`
- `twill-iced`
- `twill-slint`
- the `twill` facade crate

Most application and library users should depend on `twill-core`, a direct adapter crate, or the `twill` facade instead of using this crate directly.
