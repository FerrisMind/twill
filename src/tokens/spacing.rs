//! Spacing design tokens following Tailwind CSS spacing scale.
//!
//! The spacing scale is based on a 4px (0.25rem) base unit.

use crate::traits::{ComputeValue, ToCss};

/// Spacing values based on Tailwind's spacing scale.
/// Base unit is 0.25rem (4px in most browsers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spacing {
    /// 0 - 0px
    S0,
    /// px - 1px
    Px,
    /// 0.5 - 0.125rem (2px)
    S0_5,
    /// 1 - 0.25rem (4px)
    S1,
    /// 1.5 - 0.375rem (6px)
    S1_5,
    /// 2 - 0.5rem (8px)
    S2,
    /// 2.5 - 0.625rem (10px)
    S2_5,
    /// 3 - 0.75rem (12px)
    S3,
    /// 3.5 - 0.875rem (14px)
    S3_5,
    /// 4 - 1rem (16px)
    S4,
    /// 5 - 1.25rem (20px)
    S5,
    /// 6 - 1.5rem (24px)
    S6,
    /// 7 - 1.75rem (28px)
    S7,
    /// 8 - 2rem (32px)
    S8,
    /// 9 - 2.25rem (36px)
    S9,
    /// 10 - 2.5rem (40px)
    S10,
    /// 11 - 2.75rem (44px)
    S11,
    /// 12 - 3rem (48px)
    S12,
    /// 14 - 3.5rem (56px)
    S14,
    /// 16 - 4rem (64px)
    S16,
    /// 20 - 5rem (80px)
    S20,
    /// 24 - 6rem (96px)
    S24,
    /// 28 - 7rem (112px)
    S28,
    /// 32 - 8rem (128px)
    S32,
    /// 36 - 9rem (144px)
    S36,
    /// 40 - 10rem (160px)
    S40,
    /// 44 - 11rem (176px)
    S44,
    /// 48 - 12rem (192px)
    S48,
    /// 52 - 13rem (208px)
    S52,
    /// 56 - 14rem (224px)
    S56,
    /// 60 - 15rem (240px)
    S60,
    /// 64 - 16rem (256px)
    S64,
    /// 72 - 18rem (288px)
    S72,
    /// 80 - 20rem (320px)
    S80,
    /// 96 - 24rem (384px)
    S96,
    /// Auto value
    Auto,
}

impl Spacing {
    /// Get the rem value of this spacing.
    pub fn to_rem(&self) -> Option<f32> {
        match self {
            Spacing::S0 => Some(0.0),
            Spacing::Px => None, // 1px special case
            Spacing::S0_5 => Some(0.125),
            Spacing::S1 => Some(0.25),
            Spacing::S1_5 => Some(0.375),
            Spacing::S2 => Some(0.5),
            Spacing::S2_5 => Some(0.625),
            Spacing::S3 => Some(0.75),
            Spacing::S3_5 => Some(0.875),
            Spacing::S4 => Some(1.0),
            Spacing::S5 => Some(1.25),
            Spacing::S6 => Some(1.5),
            Spacing::S7 => Some(1.75),
            Spacing::S8 => Some(2.0),
            Spacing::S9 => Some(2.25),
            Spacing::S10 => Some(2.5),
            Spacing::S11 => Some(2.75),
            Spacing::S12 => Some(3.0),
            Spacing::S14 => Some(3.5),
            Spacing::S16 => Some(4.0),
            Spacing::S20 => Some(5.0),
            Spacing::S24 => Some(6.0),
            Spacing::S28 => Some(7.0),
            Spacing::S32 => Some(8.0),
            Spacing::S36 => Some(9.0),
            Spacing::S40 => Some(10.0),
            Spacing::S44 => Some(11.0),
            Spacing::S48 => Some(12.0),
            Spacing::S52 => Some(13.0),
            Spacing::S56 => Some(14.0),
            Spacing::S60 => Some(15.0),
            Spacing::S64 => Some(16.0),
            Spacing::S72 => Some(18.0),
            Spacing::S80 => Some(20.0),
            Spacing::S96 => Some(24.0),
            Spacing::Auto => None,
        }
    }

    /// Get the pixel value (assuming 16px base font size).
    pub fn to_px(&self) -> Option<i32> {
        match self {
            Spacing::S0 => Some(0),
            Spacing::Px => Some(1),
            Spacing::S0_5 => Some(2),
            Spacing::S1 => Some(4),
            Spacing::S1_5 => Some(6),
            Spacing::S2 => Some(8),
            Spacing::S2_5 => Some(10),
            Spacing::S3 => Some(12),
            Spacing::S3_5 => Some(14),
            Spacing::S4 => Some(16),
            Spacing::S5 => Some(20),
            Spacing::S6 => Some(24),
            Spacing::S7 => Some(28),
            Spacing::S8 => Some(32),
            Spacing::S9 => Some(36),
            Spacing::S10 => Some(40),
            Spacing::S11 => Some(44),
            Spacing::S12 => Some(48),
            Spacing::S14 => Some(56),
            Spacing::S16 => Some(64),
            Spacing::S20 => Some(80),
            Spacing::S24 => Some(96),
            Spacing::S28 => Some(112),
            Spacing::S32 => Some(128),
            Spacing::S36 => Some(144),
            Spacing::S40 => Some(160),
            Spacing::S44 => Some(176),
            Spacing::S48 => Some(192),
            Spacing::S52 => Some(208),
            Spacing::S56 => Some(224),
            Spacing::S60 => Some(240),
            Spacing::S64 => Some(256),
            Spacing::S72 => Some(288),
            Spacing::S80 => Some(320),
            Spacing::S96 => Some(384),
            Spacing::Auto => None,
        }
    }
}

impl ComputeValue for Spacing {
    type Output = String;

    fn compute(&self) -> Self::Output {
        match self {
            Spacing::S0 => "0".to_string(),
            Spacing::Px => "1px".to_string(),
            Spacing::Auto => "auto".to_string(),
            _ => {
                if let Some(rem) = self.to_rem() {
                    format!("{}rem", rem)
                } else {
                    "0".to_string()
                }
            }
        }
    }
}

impl ToCss for Spacing {
    fn to_css(&self) -> String {
        self.compute()
    }
}

/// Percentage-based spacing for widths/heights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Percentage {
    /// 0%
    S0,
    /// 1/2 = 50%
    S1_2,
    /// 1/3 = 33.333%
    S1_3,
    /// 2/3 = 66.667%
    S2_3,
    /// 1/4 = 25%
    S1_4,
    /// 2/4 = 50%
    S2_4,
    /// 3/4 = 75%
    S3_4,
    /// 1/5 = 20%
    S1_5,
    /// 2/5 = 40%
    S2_5,
    /// 3/5 = 60%
    S3_5,
    /// 4/5 = 80%
    S4_5,
    /// 1/6 = 16.667%
    S1_6,
    /// 2/6 = 33.333%
    S2_6,
    /// 3/6 = 50%
    S3_6,
    /// 4/6 = 66.667%
    S4_6,
    /// 5/6 = 83.333%
    S5_6,
    /// Full = 100%
    Full,
    /// Min content
    Min,
    /// Max content
    Max,
    /// Fit content
    Fit,
}

impl ToCss for Percentage {
    fn to_css(&self) -> String {
        match self {
            Percentage::S0 => "0%".to_string(),
            Percentage::S1_2 => "50%".to_string(),
            Percentage::S1_3 => "33.333333%".to_string(),
            Percentage::S2_3 => "66.666667%".to_string(),
            Percentage::S1_4 => "25%".to_string(),
            Percentage::S2_4 => "50%".to_string(),
            Percentage::S3_4 => "75%".to_string(),
            Percentage::S1_5 => "20%".to_string(),
            Percentage::S2_5 => "40%".to_string(),
            Percentage::S3_5 => "60%".to_string(),
            Percentage::S4_5 => "80%".to_string(),
            Percentage::S1_6 => "16.666667%".to_string(),
            Percentage::S2_6 => "33.333333%".to_string(),
            Percentage::S3_6 => "50%".to_string(),
            Percentage::S4_6 => "66.666667%".to_string(),
            Percentage::S5_6 => "83.333333%".to_string(),
            Percentage::Full => "100%".to_string(),
            Percentage::Min => "min-content".to_string(),
            Percentage::Max => "max-content".to_string(),
            Percentage::Fit => "fit-content".to_string(),
        }
    }
}

/// Container max-width values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Container {
    /// 16rem (256px)
    S3xs,
    /// 18rem (288px)
    S2xs,
    /// 20rem (320px)
    Xs,
    /// 24rem (384px)
    Sm,
    /// 28rem (448px)
    Md,
    /// 32rem (512px)
    Lg,
    /// 36rem (576px)
    Xl,
    /// 42rem (672px)
    S2xl,
    /// 48rem (768px)
    S3xl,
    /// 56rem (896px)
    S4xl,
    /// 64rem (1024px)
    S5xl,
    /// 72rem (1152px)
    S6xl,
    /// 80rem (1280px)
    S7xl,
}

impl ToCss for Container {
    fn to_css(&self) -> String {
        match self {
            Container::S3xs => "16rem".to_string(),
            Container::S2xs => "18rem".to_string(),
            Container::Xs => "20rem".to_string(),
            Container::Sm => "24rem".to_string(),
            Container::Md => "28rem".to_string(),
            Container::Lg => "32rem".to_string(),
            Container::Xl => "36rem".to_string(),
            Container::S2xl => "42rem".to_string(),
            Container::S3xl => "48rem".to_string(),
            Container::S4xl => "56rem".to_string(),
            Container::S5xl => "64rem".to_string(),
            Container::S6xl => "72rem".to_string(),
            Container::S7xl => "80rem".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_rem() {
        assert_eq!(Spacing::S4.to_rem(), Some(1.0));
        assert_eq!(Spacing::S8.to_rem(), Some(2.0));
        assert_eq!(Spacing::S16.to_rem(), Some(4.0));
    }

    #[test]
    fn test_spacing_px() {
        assert_eq!(Spacing::S4.to_px(), Some(16));
        assert_eq!(Spacing::Px.to_px(), Some(1));
    }

    #[test]
    fn test_spacing_css() {
        assert_eq!(Spacing::S4.to_css(), "1rem");
        assert_eq!(Spacing::Px.to_css(), "1px");
        assert_eq!(Spacing::Auto.to_css(), "auto");
    }
}
