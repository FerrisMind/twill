#[cfg(feature = "slint")]
mod common;
#[cfg(feature = "slint")]
mod showcase_slint_shared;

#[cfg(feature = "slint")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    showcase_slint_shared::run_slint_showcase()
}

#[cfg(not(feature = "slint"))]
fn main() {}
