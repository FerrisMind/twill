//! Core traits for twill styling system.

use crate::Style;

mod private {
    use crate::Style;

    pub trait Sealed {}

    impl<T> Sealed for T where T: Into<Style> {}
}

/// Trait for merging style-like values together.
///
/// This trait mainly supports Twill's composition infrastructure and advanced extensions.
pub trait Merge<T> {
    /// Merge another style into this one, with other taking precedence.
    fn merge(&self, other: T) -> Self;
}

/// Trait for types that can be converted into a Style.
///
/// This trait is sealed. Implement [`Into<Style>`] for your type instead.
pub trait IntoStyle: private::Sealed {
    /// Convert this value into a Style struct.
    fn into_style(self) -> Style;
}

impl<T> IntoStyle for T
where
    T: Into<Style>,
{
    fn into_style(self) -> Style {
        self.into()
    }
}

/// Trait for theme-aware styling.
///
/// This trait is intended for static dispatch and typed composition.
pub trait ThemedStyle {
    /// The theme type this style uses.
    type Theme;

    /// Create a style from a theme.
    fn from_theme(theme: &Self::Theme) -> Self;
}

/// Trait for responsive styles.
///
/// Prefer [`crate::Style::at_breakpoint`] when working with concrete [`crate::Style`] values.
pub trait Responsive {
    /// The breakpoint type.
    type Breakpoint;

    /// Get style for a specific breakpoint.
    fn at_breakpoint(&self, breakpoint: Self::Breakpoint) -> Self;
}

/// Trait for computing actual values from tokens.
///
/// This trait is primarily useful for tokens, adapters, and other infrastructure code.
pub trait ComputeValue {
    /// The output value type.
    type Output;

    /// Compute the actual value (e.g., rem, px, color hex).
    fn compute(&self) -> Self::Output;
}
