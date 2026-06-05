//! Style module.

mod layers;
mod merge;
mod presets;
pub mod state;
#[allow(clippy::module_inception)]
mod style;

pub use state::{AriaAttr, DataAttr, DataState, StateStyles};
pub use style::Style;
