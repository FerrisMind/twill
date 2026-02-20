//! Shadow design tokens following Tailwind shadow scale.

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

impl Shadow {
    /// Tailwind `--shadow` default token.
    pub const DEFAULT: Shadow = Shadow::Sm;
}

impl Shadow {
    pub fn value(&self) -> &'static str {
        match self {
            Shadow::Xs2 => "0 1px rgb(0 0 0 / 0.05)",
            Shadow::Xs => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
            Shadow::Sm => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
            Shadow::Md => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
            Shadow::Lg => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
            Shadow::Xl => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
            Shadow::S2xl => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
            Shadow::None => "none",
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

impl InsetShadow {
    /// Tailwind `--shadow-inner` default token.
    pub const DEFAULT: InsetShadow = InsetShadow::Sm;
}

impl InsetShadow {
    pub fn value(&self) -> &'static str {
        match self {
            InsetShadow::Xs2 => "inset 0 1px rgb(0 0 0 / 0.05)",
            InsetShadow::Xs => "inset 0 1px 1px rgb(0 0 0 / 0.05)",
            InsetShadow::Sm => "inset 0 2px 4px rgb(0 0 0 / 0.05)",
            InsetShadow::None => "none",
        }
    }
}

/// Drop shadow tokens (for filters).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropShadow {
    /// 0 1px 2px rgb(0 0 0 / 0.1), 0 1px 1px rgb(0 0 0 / 0.06)
    Base,
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

impl DropShadow {
    /// Tailwind `--drop-shadow` default token.
    pub const DEFAULT: DropShadow = DropShadow::Base;
}

impl DropShadow {
    pub fn value(&self) -> &'static str {
        match self {
            DropShadow::Base => {
                "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))"
            }
            DropShadow::Xs => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
            DropShadow::Sm => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.15))",
            DropShadow::Md => "drop-shadow(0 3px 3px rgb(0 0 0 / 0.12))",
            DropShadow::Lg => "drop-shadow(0 4px 4px rgb(0 0 0 / 0.15))",
            DropShadow::Xl => "drop-shadow(0 9px 7px rgb(0 0 0 / 0.1))",
            DropShadow::S2xl => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
            DropShadow::None => "none",
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

impl TextShadow {
    pub fn value(&self) -> &'static str {
        match self {
            TextShadow::Xs2 => "0px 1px 0px rgb(0 0 0 / 0.15)",
            TextShadow::Xs => "0px 1px 1px rgb(0 0 0 / 0.2)",
            TextShadow::Sm => {
                "0px 1px 0px rgb(0 0 0 / 0.075), 0px 1px 1px rgb(0 0 0 / 0.075), 0px 2px 2px rgb(0 0 0 / 0.075)"
            }
            TextShadow::Md => {
                "0px 1px 1px rgb(0 0 0 / 0.1), 0px 1px 2px rgb(0 0 0 / 0.1), 0px 2px 4px rgb(0 0 0 / 0.1)"
            }
            TextShadow::Lg => {
                "0px 1px 2px rgb(0 0 0 / 0.1), 0px 3px 2px rgb(0 0 0 / 0.1), 0px 4px 8px rgb(0 0 0 / 0.1)"
            }
            TextShadow::None => "none",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_value() {
        assert!(Shadow::Sm.value().contains("0 1px 3px"));
        assert_eq!(Shadow::None.value(), "none");
    }

    #[test]
    fn test_drop_shadow_base_value() {
        let value = DropShadow::Base.value();
        assert!(value.contains("drop-shadow(0 1px 2px"));
        assert!(value.contains("drop-shadow(0 1px 1px"));
    }
}
