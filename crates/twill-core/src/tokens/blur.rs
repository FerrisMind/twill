/// Blur token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blur {
    /// 0px
    None,
    /// 4px
    Xs,
    /// 8px
    Sm,
    /// 8px (legacy alias)
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
    pub const DEFAULT: Blur = Blur::Sm;

    pub fn radius_px(&self) -> u16 {
        match self {
            Blur::None => 0,
            Blur::Xs => 4,
            Blur::Sm | Blur::Base => 8,
            Blur::Md => 12,
            Blur::Lg => 16,
            Blur::Xl => 24,
            Blur::S2xl => 40,
            Blur::S3xl => 64,
            Blur::Custom(px) => *px,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blur_scale_alignment() {
        assert_eq!(Blur::Xs.radius_px(), 4);
        assert_eq!(Blur::Sm.radius_px(), 8);
        assert_eq!(Blur::DEFAULT.radius_px(), 8);
    }
}
