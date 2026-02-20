//! Border design tokens for native UI frameworks.

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

impl BorderRadius {
    /// Tailwind `--radius` default token.
    pub const DEFAULT: BorderRadius = BorderRadius::Sm;
}

impl BorderRadius {
    pub fn px_value(&self) -> f32 {
        match self {
            BorderRadius::None => 0.0,
            BorderRadius::Xs => 2.0,
            BorderRadius::Sm => 4.0,
            BorderRadius::Md => 6.0,
            BorderRadius::Lg => 8.0,
            BorderRadius::Xl => 12.0,
            BorderRadius::S2xl => 16.0,
            BorderRadius::S3xl => 24.0,
            BorderRadius::S4xl => 32.0,
            BorderRadius::Full => 9999.0,
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

impl BorderWidth {
    pub fn px_value(&self) -> u8 {
        match self {
            BorderWidth::S0 => 0,
            BorderWidth::S1 => 1,
            BorderWidth::S2 => 2,
            BorderWidth::S4 => 4,
            BorderWidth::S8 => 8,
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

impl BorderStyle {
    pub fn keyword(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Dotted => "dotted",
            BorderStyle::Double => "double",
            BorderStyle::Hidden => "hidden",
            BorderStyle::None => "none",
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

impl OutlineStyle {
    pub fn keyword(&self) -> &'static str {
        match self {
            OutlineStyle::None => "none",
            OutlineStyle::Solid => "solid",
            OutlineStyle::Dashed => "dashed",
            OutlineStyle::Dotted => "dotted",
            OutlineStyle::Double => "double",
            OutlineStyle::Hidden => "hidden",
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

impl RingWidth {
    pub fn px_value(&self) -> Option<u8> {
        match self {
            RingWidth::None => Some(0),
            RingWidth::S1 => Some(1),
            RingWidth::S2 => Some(2),
            RingWidth::S4 => Some(4),
            RingWidth::S8 => Some(8),
            RingWidth::Inset => None,
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

impl DivideWidth {
    pub fn px_value(&self) -> u8 {
        match self {
            DivideWidth::S0 => 0,
            DivideWidth::S1 => 1,
            DivideWidth::S2 => 2,
            DivideWidth::S4 => 4,
            DivideWidth::S8 => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_radius() {
        assert_eq!(BorderRadius::Md.px_value(), 6.0);
        assert_eq!(BorderRadius::Full.px_value(), 9999.0);
    }

    #[test]
    fn test_border_width() {
        assert_eq!(BorderWidth::S2.px_value(), 2);
    }
}
