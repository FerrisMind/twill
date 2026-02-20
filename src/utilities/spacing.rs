//! Spacing utilities for padding and margin.

use crate::tokens::Spacing;
use crate::traits::ComputeValue;

/// Padding utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Padding {
    pub top: Option<Spacing>,
    pub right: Option<Spacing>,
    pub bottom: Option<Spacing>,
    pub left: Option<Spacing>,
}

impl Padding {
    pub fn new() -> Self {
        Self::default()
    }

    /// All sides same value
    pub fn all(value: Spacing) -> Self {
        Self {
            top: Some(value),
            right: Some(value),
            bottom: Some(value),
            left: Some(value),
        }
    }

    /// Horizontal and vertical
    pub fn symmetric(vertical: Spacing, horizontal: Spacing) -> Self {
        Self {
            top: Some(vertical),
            right: Some(horizontal),
            bottom: Some(vertical),
            left: Some(horizontal),
        }
    }

    /// Individual sides
    pub fn individual(top: Spacing, right: Spacing, bottom: Spacing, left: Spacing) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }

    /// Only horizontal (x)
    pub fn x(value: Spacing) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: Some(value),
        }
    }

    /// Only vertical (y)
    pub fn y(value: Spacing) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// Top only
    pub fn top(value: Spacing) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: None,
            left: None,
        }
    }

    /// Right only
    pub fn right(value: Spacing) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: None,
        }
    }

    /// Bottom only
    pub fn bottom(value: Spacing) -> Self {
        Self {
            top: None,
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// Left only
    pub fn left(value: Spacing) -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: Some(value),
        }
    }
}

/// Margin utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Margin {
    pub top: Option<Spacing>,
    pub right: Option<Spacing>,
    pub bottom: Option<Spacing>,
    pub left: Option<Spacing>,
}

impl Margin {
    pub fn new() -> Self {
        Self::default()
    }

    /// All sides same value
    pub fn all(value: Spacing) -> Self {
        Self {
            top: Some(value),
            right: Some(value),
            bottom: Some(value),
            left: Some(value),
        }
    }

    /// Horizontal and vertical
    pub fn symmetric(vertical: Spacing, horizontal: Spacing) -> Self {
        Self {
            top: Some(vertical),
            right: Some(horizontal),
            bottom: Some(vertical),
            left: Some(horizontal),
        }
    }

    /// Individual sides
    pub fn individual(top: Spacing, right: Spacing, bottom: Spacing, left: Spacing) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }

    /// Only horizontal (x)
    pub fn x(value: Spacing) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: Some(value),
        }
    }

    /// Only vertical (y)
    pub fn y(value: Spacing) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// Top only
    pub fn top(value: Spacing) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: None,
            left: None,
        }
    }

    /// Right only
    pub fn right(value: Spacing) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: None,
        }
    }

    /// Bottom only
    pub fn bottom(value: Spacing) -> Self {
        Self {
            top: None,
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// Left only
    pub fn left(value: Spacing) -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: Some(value),
        }
    }

    /// Auto margins (center horizontally)
    pub fn auto_x() -> Self {
        Self {
            top: None,
            right: Some(Spacing::Auto),
            bottom: None,
            left: Some(Spacing::Auto),
        }
    }

    /// Auto margins (center vertically)
    pub fn auto_y() -> Self {
        Self {
            top: Some(Spacing::Auto),
            right: None,
            bottom: Some(Spacing::Auto),
            left: None,
        }
    }

    /// Auto all
    pub fn auto() -> Self {
        Self::all(Spacing::Auto)
    }
}

/// Size utility (width/height).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    Spacing(Spacing),
    Percentage(crate::tokens::Percentage),
    Auto,
    Full,
    /// Tailwind `--max-width-prose` (65ch).
    Prose,
    ScreenWidth,
    ScreenHeight,
    MinContent,
    MaxContent,
    Fit,
}

impl Size {
    pub fn value(&self) -> String {
        match self {
            Size::Spacing(s) => s.compute(),
            Size::Percentage(p) => p.value().to_string(),
            Size::Auto => "auto".to_string(),
            Size::Full => "100%".to_string(),
            Size::Prose => "65ch".to_string(),
            Size::ScreenWidth => "100vw".to_string(),
            Size::ScreenHeight => "100vh".to_string(),
            Size::MinContent => "min-content".to_string(),
            Size::MaxContent => "max-content".to_string(),
            Size::Fit => "fit-content".to_string(),
        }
    }
}

/// Width utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Width(pub Option<Size>);

impl Width {
    pub fn new(size: Size) -> Self {
        Self(Some(size))
    }
    pub fn full() -> Self {
        Self(Some(Size::Full))
    }
    pub fn auto() -> Self {
        Self(Some(Size::Auto))
    }
    pub fn screen() -> Self {
        Self(Some(Size::ScreenWidth))
    }
}

/// Height utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Height(pub Option<Size>);

impl Height {
    pub fn new(size: Size) -> Self {
        Self(Some(size))
    }
    pub fn full() -> Self {
        Self(Some(Size::Full))
    }
    pub fn auto() -> Self {
        Self(Some(Size::Auto))
    }
    pub fn screen() -> Self {
        Self(Some(Size::ScreenHeight))
    }
}

/// Min/Max width constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SizeConstraints {
    pub min_width: Option<Size>,
    pub max_width: Option<Size>,
    pub min_height: Option<Size>,
    pub max_height: Option<Size>,
}

impl SizeConstraints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_width(mut self, size: Size) -> Self {
        self.min_width = Some(size);
        self
    }
    pub fn max_width(mut self, size: Size) -> Self {
        self.max_width = Some(size);
        self
    }
    pub fn min_height(mut self, size: Size) -> Self {
        self.min_height = Some(size);
        self
    }
    pub fn max_height(mut self, size: Size) -> Self {
        self.max_height = Some(size);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_all() {
        let p = Padding::all(Spacing::S4);
        assert_eq!(p.top, Some(Spacing::S4));
        assert_eq!(p.right, Some(Spacing::S4));
        assert_eq!(p.bottom, Some(Spacing::S4));
        assert_eq!(p.left, Some(Spacing::S4));
    }

    #[test]
    fn test_padding_symmetric() {
        let p = Padding::symmetric(Spacing::S2, Spacing::S4);
        assert_eq!(p.top, Some(Spacing::S2));
        assert_eq!(p.right, Some(Spacing::S4));
        assert_eq!(p.bottom, Some(Spacing::S2));
        assert_eq!(p.left, Some(Spacing::S4));
    }

    #[test]
    fn test_margin_auto() {
        let m = Margin::auto_x();
        assert_eq!(m.right, Some(Spacing::Auto));
        assert_eq!(m.left, Some(Spacing::Auto));
    }

    #[test]
    fn test_screen_size_value() {
        assert_eq!(Width::screen().0, Some(Size::ScreenWidth));
        assert_eq!(Height::screen().0, Some(Size::ScreenHeight));
    }

    #[test]
    fn test_prose_size_value() {
        assert_eq!(Size::Prose.value(), "65ch");
    }
}
