//! Typography design tokens following Tailwind CSS typography scale.

use crate::traits::ToCss;

/// Font family tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontFamily {
    /// System sans-serif font stack
    Sans,
    /// System serif font stack
    Serif,
    /// System monospace font stack
    Mono,
}

impl ToCss for FontFamily {
    fn to_css(&self) -> String {
        match self {
            FontFamily::Sans => "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"".to_string(),
            FontFamily::Serif => "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif".to_string(),
            FontFamily::Mono => "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace".to_string(),
        }
    }
}

/// Font size tokens with line-height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontSize {
    /// 0.75rem (12px) / line-height: 1rem
    Xs,
    /// 0.875rem (14px) / line-height: 1.25rem
    Sm,
    /// 1rem (16px) / line-height: 1.5rem
    Base,
    /// 1.125rem (18px) / line-height: 1.75rem
    Lg,
    /// 1.25rem (20px) / line-height: 1.75rem
    Xl,
    /// 1.5rem (24px) / line-height: 2rem
    S2xl,
    /// 1.875rem (30px) / line-height: 2.25rem
    S3xl,
    /// 2.25rem (36px) / line-height: 2.5rem
    S4xl,
    /// 3rem (48px) / line-height: 1
    S5xl,
    /// 3.75rem (60px) / line-height: 1
    S6xl,
    /// 4.5rem (72px) / line-height: 1
    S7xl,
    /// 6rem (96px) / line-height: 1
    S8xl,
    /// 8rem (128px) / line-height: 1
    S9xl,
}

impl FontSize {
    /// Get font size in rem.
    pub fn size_rem(&self) -> f32 {
        match self {
            FontSize::Xs => 0.75,
            FontSize::Sm => 0.875,
            FontSize::Base => 1.0,
            FontSize::Lg => 1.125,
            FontSize::Xl => 1.25,
            FontSize::S2xl => 1.5,
            FontSize::S3xl => 1.875,
            FontSize::S4xl => 2.25,
            FontSize::S5xl => 3.0,
            FontSize::S6xl => 3.75,
            FontSize::S7xl => 4.5,
            FontSize::S8xl => 6.0,
            FontSize::S9xl => 8.0,
        }
    }

    /// Get line-height.
    pub fn line_height(&self) -> f32 {
        match self {
            FontSize::Xs => 1.0 / 0.75,
            FontSize::Sm => 1.25 / 0.875,
            FontSize::Base => 1.5,
            FontSize::Lg => 1.75 / 1.125,
            FontSize::Xl => 1.75 / 1.25,
            FontSize::S2xl => 2.0 / 1.5,
            FontSize::S3xl => 2.25 / 1.875,
            FontSize::S4xl => 2.5 / 2.25,
            FontSize::S5xl => 1.0,
            FontSize::S6xl => 1.0,
            FontSize::S7xl => 1.0,
            FontSize::S8xl => 1.0,
            FontSize::S9xl => 1.0,
        }
    }
}

impl ToCss for FontSize {
    fn to_css(&self) -> String {
        format!("{}rem", self.size_rem())
    }
}

/// Font weight tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWeight {
    /// 100
    Thin,
    /// 200
    ExtraLight,
    /// 300
    Light,
    /// 400
    Normal,
    /// 500
    Medium,
    /// 600
    SemiBold,
    /// 700
    Bold,
    /// 800
    ExtraBold,
    /// 900
    Black,
}

impl FontWeight {
    /// Get the numeric weight value.
    pub fn value(&self) -> u16 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

impl ToCss for FontWeight {
    fn to_css(&self) -> String {
        self.value().to_string()
    }
}

/// Letter spacing (tracking) tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LetterSpacing {
    /// -0.05em
    Tighter,
    /// -0.025em
    Tight,
    /// 0em
    Normal,
    /// 0.025em
    Wide,
    /// 0.05em
    Wider,
    /// 0.1em
    Widest,
}

impl ToCss for LetterSpacing {
    fn to_css(&self) -> String {
        match self {
            LetterSpacing::Tighter => "-0.05em".to_string(),
            LetterSpacing::Tight => "-0.025em".to_string(),
            LetterSpacing::Normal => "0em".to_string(),
            LetterSpacing::Wide => "0.025em".to_string(),
            LetterSpacing::Wider => "0.05em".to_string(),
            LetterSpacing::Widest => "0.1em".to_string(),
        }
    }
}

/// Line height (leading) tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineHeight {
    /// 1.25
    Tight,
    /// 1.375
    Snug,
    /// 1.5
    Normal,
    /// 1.625
    Relaxed,
    /// 2
    Loose,
    /// Numeric value (e.g., 1, 2, 3, etc.)
    Numeric(u8),
}

impl ToCss for LineHeight {
    fn to_css(&self) -> String {
        match self {
            LineHeight::Tight => "1.25".to_string(),
            LineHeight::Snug => "1.375".to_string(),
            LineHeight::Normal => "1.5".to_string(),
            LineHeight::Relaxed => "1.625".to_string(),
            LineHeight::Loose => "2".to_string(),
            LineHeight::Numeric(n) => n.to_string(),
        }
    }
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
}

impl ToCss for TextAlign {
    fn to_css(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Justify => "justify".to_string(),
            TextAlign::Start => "start".to_string(),
            TextAlign::End => "end".to_string(),
        }
    }
}

/// Text decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
}

impl ToCss for TextDecoration {
    fn to_css(&self) -> String {
        match self {
            TextDecoration::None => "none".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::Overline => "overline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
        }
    }
}

/// Text transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

impl ToCss for TextTransform {
    fn to_css(&self) -> String {
        match self {
            TextTransform::None => "none".to_string(),
            TextTransform::Uppercase => "uppercase".to_string(),
            TextTransform::Lowercase => "lowercase".to_string(),
            TextTransform::Capitalize => "capitalize".to_string(),
        }
    }
}

/// Text overflow handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

impl ToCss for TextOverflow {
    fn to_css(&self) -> String {
        match self {
            TextOverflow::Clip => "clip".to_string(),
            TextOverflow::Ellipsis => "ellipsis".to_string(),
        }
    }
}

/// White space handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
}

impl ToCss for WhiteSpace {
    fn to_css(&self) -> String {
        match self {
            WhiteSpace::Normal => "normal".to_string(),
            WhiteSpace::NoWrap => "nowrap".to_string(),
            WhiteSpace::Pre => "pre".to_string(),
            WhiteSpace::PreLine => "pre-line".to_string(),
            WhiteSpace::PreWrap => "pre-wrap".to_string(),
        }
    }
}

/// Word break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordBreak {
    Normal,
    BreakAll,
    KeepAll,
    BreakWord,
}

impl ToCss for WordBreak {
    fn to_css(&self) -> String {
        match self {
            WordBreak::Normal => "normal".to_string(),
            WordBreak::BreakAll => "break-all".to_string(),
            WordBreak::KeepAll => "keep-all".to_string(),
            WordBreak::BreakWord => "break-word".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_size() {
        assert_eq!(FontSize::Base.size_rem(), 1.0);
        assert_eq!(FontSize::S2xl.size_rem(), 1.5);
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Bold.value(), 700);
        assert_eq!(FontWeight::Normal.value(), 400);
    }

    #[test]
    fn test_letter_spacing() {
        assert_eq!(LetterSpacing::Tight.to_css(), "-0.025em");
    }
}
