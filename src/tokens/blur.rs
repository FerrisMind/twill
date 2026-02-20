use crate::traits::ToCss;

/// Blur token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blur {
    /// 0px
    None,
    /// 4px
    Sm,
    /// 8px
    Base,
    /// 12px
    Md,
    /// 16px
    Lg,
    /// 24px
    Xl,
    /// 40px
    S2xl,
    /// 64px
    S3xl,
    /// Custom blur radius in pixels
    Custom(u16),
}

impl Blur {
    pub const DEFAULT: Blur = Blur::Base;
}

impl ToCss for Blur {
    fn to_css(&self) -> String {
        match self {
            Blur::None => "0".to_string(),
            Blur::Sm => "4px".to_string(),
            Blur::Base => "8px".to_string(),
            Blur::Md => "12px".to_string(),
            Blur::Lg => "16px".to_string(),
            Blur::Xl => "24px".to_string(),
            Blur::S2xl => "40px".to_string(),
            Blur::S3xl => "64px".to_string(),
            Blur::Custom(px) => format!("{}px", px),
        }
    }
}
