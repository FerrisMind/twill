//! Core traits for rustwind styling system.

use crate::Style;

/// Trait for converting types to CSS string representation.
pub trait ToCss {
    /// Convert to CSS property string (e.g., "padding: 1rem").
    fn to_css(&self) -> String;

    /// Convert to CSS property name and value pair.
    fn to_css_pair(&self) -> Option<(String, String)> {
        None
    }
}

/// Trait for merging styles together.
pub trait Merge<T> {
    /// Merge another style into this one, with other taking precedence.
    fn merge(&self, other: T) -> Self;
}

/// Trait for types that can be converted into a Style.
pub trait IntoStyle {
    /// Convert this value into a Style struct.
    fn into_style(self) -> Style;
}

/// Trait for defining default styles for components.
pub trait DefaultStyle {
    /// Get the default style for this component.
    fn default_style() -> Style;
}

/// Trait for theme-aware styling.
pub trait ThemedStyle {
    /// The theme type this style uses.
    type Theme;

    /// Create a style from a theme.
    fn from_theme(theme: &Self::Theme) -> Self;
}

/// Trait for responsive styles.
pub trait Responsive {
    /// The breakpoint type.
    type Breakpoint;

    /// Get style for a specific breakpoint.
    fn at_breakpoint(&self, breakpoint: Self::Breakpoint) -> Self;
}

/// Trait for style variants (e.g., button variants).
pub trait Variants {
    /// The variant enum type.
    type Variant;

    /// Create a style from a variant.
    fn from_variant(variant: Self::Variant) -> Self;
}

/// Trait for sizes.
pub trait Sizable {
    /// The size enum type.
    type Size;

    /// Create a style from a size.
    fn from_size(size: Self::Size) -> Self;
}

/// Trait for computing actual values from tokens.
pub trait ComputeValue {
    /// The output value type.
    type Output;

    /// Compute the actual value (e.g., rem, px, color hex).
    fn compute(&self) -> Self::Output;
}
