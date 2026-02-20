use crate::traits::ToCss;

/// AspectRatio token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectRatio {
    Auto,
    Square,
    Video,
    Custom(u16, u16),
}

impl ToCss for AspectRatio {
    fn to_css(&self) -> String {
        match self {
            AspectRatio::Auto => "auto".to_string(),
            AspectRatio::Square => "1 / 1".to_string(),
            AspectRatio::Video => "16 / 9".to_string(),
            AspectRatio::Custom(w, h) => format!("{} / {}", w, h),
        }
    }
}
