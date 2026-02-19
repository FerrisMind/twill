//! Border design tokens following Tailwind CSS border scale.

use crate::traits::ToCss;

/// Border radius tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BorderRadius {
    /// 0
    #[default]
    None,
    /// 0.125rem (2px)
    Xs,
    /// 0.25rem (4px)
    Sm,
    /// 0.375rem (6px)
    Md,
    /// 0.5rem (8px)
    Lg,
    /// 0.75rem (12px)
    Xl,
    /// 1rem (16px)
    S2xl,
    /// 1.5rem (24px)
    S3xl,
    /// 2rem (32px)
    S4xl,
    /// 9999px (pill shape)
    Full,
}

impl ToCss for BorderRadius {
    fn to_css(&self) -> String {
        match self {
            BorderRadius::None => "0".to_string(),
            BorderRadius::Xs => "0.125rem".to_string(),
            BorderRadius::Sm => "0.25rem".to_string(),
            BorderRadius::Md => "0.375rem".to_string(),
            BorderRadius::Lg => "0.5rem".to_string(),
            BorderRadius::Xl => "0.75rem".to_string(),
            BorderRadius::S2xl => "1rem".to_string(),
            BorderRadius::S3xl => "1.5rem".to_string(),
            BorderRadius::S4xl => "2rem".to_string(),
            BorderRadius::Full => "9999px".to_string(),
        }
    }
}

/// Border width tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderWidth {
    /// 0
    S0,
    /// 1px
    S1,
    /// 2px
    S2,
    /// 4px
    S4,
    /// 8px
    S8,
}

impl ToCss for BorderWidth {
    fn to_css(&self) -> String {
        match self {
            BorderWidth::S0 => "0".to_string(),
            BorderWidth::S1 => "1px".to_string(),
            BorderWidth::S2 => "2px".to_string(),
            BorderWidth::S4 => "4px".to_string(),
            BorderWidth::S8 => "8px".to_string(),
        }
    }
}

/// Border style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
    None,
}

impl ToCss for BorderStyle {
    fn to_css(&self) -> String {
        match self {
            BorderStyle::Solid => "solid".to_string(),
            BorderStyle::Dashed => "dashed".to_string(),
            BorderStyle::Dotted => "dotted".to_string(),
            BorderStyle::Double => "double".to_string(),
            BorderStyle::Hidden => "hidden".to_string(),
            BorderStyle::None => "none".to_string(),
        }
    }
}

/// Outline style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutlineStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}

impl ToCss for OutlineStyle {
    fn to_css(&self) -> String {
        match self {
            OutlineStyle::None => "none".to_string(),
            OutlineStyle::Solid => "solid".to_string(),
            OutlineStyle::Dashed => "dashed".to_string(),
            OutlineStyle::Dotted => "dotted".to_string(),
            OutlineStyle::Double => "double".to_string(),
            OutlineStyle::Hidden => "hidden".to_string(),
        }
    }
}

/// Ring width tokens (for focus rings).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RingWidth {
    /// 0
    None,
    /// 1px
    S1,
    /// 2px
    S2,
    /// 4px
    S4,
    /// 8px
    S8,
    /// Inset ring
    Inset,
}

impl ToCss for RingWidth {
    fn to_css(&self) -> String {
        match self {
            RingWidth::None => "0".to_string(),
            RingWidth::S1 => "1px".to_string(),
            RingWidth::S2 => "2px".to_string(),
            RingWidth::S4 => "4px".to_string(),
            RingWidth::S8 => "8px".to_string(),
            RingWidth::Inset => "inset".to_string(),
        }
    }
}

/// Divide width (for separating elements).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DivideWidth {
    /// 0
    S0,
    /// 1px
    S1,
    /// 2px
    S2,
    /// 4px
    S4,
    /// 8px
    S8,
}

impl ToCss for DivideWidth {
    fn to_css(&self) -> String {
        match self {
            DivideWidth::S0 => "0".to_string(),
            DivideWidth::S1 => "1px".to_string(),
            DivideWidth::S2 => "2px".to_string(),
            DivideWidth::S4 => "4px".to_string(),
            DivideWidth::S8 => "8px".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_radius() {
        assert_eq!(BorderRadius::Md.to_css(), "0.375rem");
        assert_eq!(BorderRadius::Full.to_css(), "9999px");
    }

    #[test]
    fn test_border_width() {
        assert_eq!(BorderWidth::S2.to_css(), "2px");
    }
}