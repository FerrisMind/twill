//! Shadow design tokens following Tailwind CSS shadow scale.

use crate::traits::ToCss;

/// Box shadow tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Shadow {
    /// 0 1px rgb(0 0 0 / 0.05)
    Xs2,
    /// 0 1px 2px 0 rgb(0 0 0 / 0.05)
    Xs,
    /// 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)
    Sm,
    /// 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)
    Md,
    /// 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)
    Lg,
    /// 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)
    Xl,
    /// 0 25px 50px -12px rgb(0 0 0 / 0.25)
    S2xl,
    /// No shadow
    #[default]
    None,
}

impl ToCss for Shadow {
    fn to_css(&self) -> String {
        match self {
            Shadow::Xs2 => "0 1px rgb(0 0 0 / 0.05)".to_string(),
            Shadow::Xs => "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            Shadow::Sm => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
            Shadow::Md => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            Shadow::Lg => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            Shadow::Xl => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
            Shadow::S2xl => "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
            Shadow::None => "none".to_string(),
        }
    }
}

/// Inset box shadow tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsetShadow {
    /// inset 0 1px rgb(0 0 0 / 0.05)
    Xs2,
    /// inset 0 1px 1px rgb(0 0 0 / 0.05)
    Xs,
    /// inset 0 2px 4px rgb(0 0 0 / 0.05)
    Sm,
    /// No shadow
    None,
}

impl ToCss for InsetShadow {
    fn to_css(&self) -> String {
        match self {
            InsetShadow::Xs2 => "inset 0 1px rgb(0 0 0 / 0.05)".to_string(),
            InsetShadow::Xs => "inset 0 1px 1px rgb(0 0 0 / 0.05)".to_string(),
            InsetShadow::Sm => "inset 0 2px 4px rgb(0 0 0 / 0.05)".to_string(),
            InsetShadow::None => "none".to_string(),
        }
    }
}

/// Drop shadow tokens (for filters).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropShadow {
    /// 0 1px 1px rgb(0 0 0 / 0.05)
    Xs,
    /// 0 1px 2px rgb(0 0 0 / 0.15)
    Sm,
    /// 0 3px 3px rgb(0 0 0 / 0.12)
    Md,
    /// 0 4px 4px rgb(0 0 0 / 0.15)
    Lg,
    /// 0 9px 7px rgb(0 0 0 / 0.1)
    Xl,
    /// 0 25px 25px rgb(0 0 0 / 0.15)
    S2xl,
    /// No shadow
    None,
}

impl ToCss for DropShadow {
    fn to_css(&self) -> String {
        match self {
            DropShadow::Xs => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))".to_string(),
            DropShadow::Sm => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.15))".to_string(),
            DropShadow::Md => "drop-shadow(0 3px 3px rgb(0 0 0 / 0.12))".to_string(),
            DropShadow::Lg => "drop-shadow(0 4px 4px rgb(0 0 0 / 0.15))".to_string(),
            DropShadow::Xl => "drop-shadow(0 9px 7px rgb(0 0 0 / 0.1))".to_string(),
            DropShadow::S2xl => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))".to_string(),
            DropShadow::None => "none".to_string(),
        }
    }
}

/// Text shadow tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextShadow {
    /// 0px 1px 0px rgb(0 0 0 / 0.15)
    Xs2,
    /// 0px 1px 1px rgb(0 0 0 / 0.2)
    Xs,
    /// Multiple layers
    Sm,
    /// Multiple layers
    Md,
    /// Multiple layers
    Lg,
    /// No shadow
    None,
}

impl ToCss for TextShadow {
    fn to_css(&self) -> String {
        match self {
            TextShadow::Xs2 => "0px 1px 0px rgb(0 0 0 / 0.15)".to_string(),
            TextShadow::Xs => "0px 1px 1px rgb(0 0 0 / 0.2)".to_string(),
            TextShadow::Sm => "0px 1px 0px rgb(0 0 0 / 0.075), 0px 1px 1px rgb(0 0 0 / 0.075), 0px 2px 2px rgb(0 0 0 / 0.075)".to_string(),
            TextShadow::Md => "0px 1px 1px rgb(0 0 0 / 0.1), 0px 1px 2px rgb(0 0 0 / 0.1), 0px 2px 4px rgb(0 0 0 / 0.1)".to_string(),
            TextShadow::Lg => "0px 1px 2px rgb(0 0 0 / 0.1), 0px 3px 2px rgb(0 0 0 / 0.1), 0px 4px 8px rgb(0 0 0 / 0.1)".to_string(),
            TextShadow::None => "none".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_css() {
        assert!(Shadow::Sm.to_css().contains("0 1px 3px"));
        assert_eq!(Shadow::None.to_css(), "none");
    }
}