//! Typography design tokens following a utility-first type scale.

use std::fmt;

/// Named font-size variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontSizeVar(&'static str);

impl FontSizeVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for FontSizeVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for FontSizeVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for FontSizeVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

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

impl FontFamily {
    pub fn stack(&self) -> &'static str {
        match self {
            FontFamily::Sans => {
                "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\""
            }
            FontFamily::Serif => "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif",
            FontFamily::Mono => {
                "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace"
            }
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
    /// Custom property (e.g. `var(--text-size)`).
    Var(FontSizeVar),
    /// Typed arbitrary pixel value (e.g. `14px`).
    Px(u16),
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
            FontSize::Var(_) => 1.0,
            FontSize::Px(px) => f32::from(*px) / 16.0,
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
            FontSize::Var(_) => 1.0,
            FontSize::Px(_) => 1.0,
        }
    }
}

impl FontSize {
    pub const fn var(name: FontSizeVar) -> Self {
        Self::Var(name)
    }

    pub const fn px(value: u16) -> Self {
        Self::Px(value)
    }

    pub fn resolve_px(&self, custom_properties: &[(&str, f32)]) -> Option<f32> {
        match self {
            FontSize::Var(var) => custom_properties
                .iter()
                .find(|(name, _)| *name == var.as_str())
                .map(|(_, value)| (*value).max(0.0)),
            FontSize::Px(px) => Some(f32::from(*px)),
            _ => Some(self.size_rem() * 16.0),
        }
    }

    pub fn value(&self) -> String {
        match self {
            FontSize::Var(var) => format!("var({})", var.as_str()),
            FontSize::Px(px) => format!("{px}px"),
            _ => format!("{}rem", self.size_rem()),
        }
    }
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSize::Var(var) => write!(f, "var({var})"),
            FontSize::Px(px) => write!(f, "{px}px"),
            _ => write!(f, "{}rem", self.size_rem()),
        }
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

impl FontWeight {
    pub fn value_str(&self) -> String {
        self.value().to_string()
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Named letter-spacing variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LetterSpacingVar(&'static str);

impl LetterSpacingVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for LetterSpacingVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for LetterSpacingVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for LetterSpacingVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Letter spacing (tracking) tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Custom property (e.g. `var(--tracking)`).
    Var(LetterSpacingVar),
    /// Typed arbitrary em value (e.g. `0.035em`).
    Em(f32),
}

impl LetterSpacing {
    pub const fn var(name: LetterSpacingVar) -> Self {
        Self::Var(name)
    }

    pub const fn em(value: f32) -> Self {
        Self::Em(value)
    }

    pub fn value(&self) -> String {
        match self {
            LetterSpacing::Tighter => "-0.05em".to_string(),
            LetterSpacing::Tight => "-0.025em".to_string(),
            LetterSpacing::Normal => "0em".to_string(),
            LetterSpacing::Wide => "0.025em".to_string(),
            LetterSpacing::Wider => "0.05em".to_string(),
            LetterSpacing::Widest => "0.1em".to_string(),
            LetterSpacing::Var(var) => format!("var({})", var.as_str()),
            LetterSpacing::Em(value) => format!("{value}em"),
        }
    }
}

impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LetterSpacing::Tighter => f.write_str("-0.05em"),
            LetterSpacing::Tight => f.write_str("-0.025em"),
            LetterSpacing::Normal => f.write_str("0em"),
            LetterSpacing::Wide => f.write_str("0.025em"),
            LetterSpacing::Wider => f.write_str("0.05em"),
            LetterSpacing::Widest => f.write_str("0.1em"),
            LetterSpacing::Var(var) => write!(f, "var({var})"),
            LetterSpacing::Em(value) => write!(f, "{value}em"),
        }
    }
}

/// Named line-height variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineHeightVar(&'static str);

impl LineHeightVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for LineHeightVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for LineHeightVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for LineHeightVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Line height (leading) tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Numeric value (e.g. `1.75`).
    Number(f32),
    /// Custom property (e.g. `var(--leading)`).
    Var(LineHeightVar),
}

impl LineHeight {
    pub const fn number(value: f32) -> Self {
        Self::Number(value)
    }

    pub const fn var(name: LineHeightVar) -> Self {
        Self::Var(name)
    }

    pub fn value(&self) -> String {
        match self {
            LineHeight::Tight => "1.25".to_string(),
            LineHeight::Snug => "1.375".to_string(),
            LineHeight::Normal => "1.5".to_string(),
            LineHeight::Relaxed => "1.625".to_string(),
            LineHeight::Loose => "2".to_string(),
            LineHeight::Number(n) => n.to_string(),
            LineHeight::Var(var) => format!("var({})", var.as_str()),
        }
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineHeight::Tight => f.write_str("1.25"),
            LineHeight::Snug => f.write_str("1.375"),
            LineHeight::Normal => f.write_str("1.5"),
            LineHeight::Relaxed => f.write_str("1.625"),
            LineHeight::Loose => f.write_str("2"),
            LineHeight::Number(n) => write!(f, "{n}"),
            LineHeight::Var(var) => write!(f, "var({var})"),
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

impl TextAlign {
    pub fn value(&self) -> &'static str {
        match self {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
            TextAlign::Justify => "justify",
            TextAlign::Start => "start",
            TextAlign::End => "end",
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

impl TextDecoration {
    pub fn value(&self) -> &'static str {
        match self {
            TextDecoration::None => "none",
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
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

impl TextTransform {
    pub fn value(&self) -> &'static str {
        match self {
            TextTransform::None => "none",
            TextTransform::Uppercase => "uppercase",
            TextTransform::Lowercase => "lowercase",
            TextTransform::Capitalize => "capitalize",
        }
    }
}

/// Text overflow handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

impl TextOverflow {
    pub fn value(&self) -> &'static str {
        match self {
            TextOverflow::Clip => "clip",
            TextOverflow::Ellipsis => "ellipsis",
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

impl WhiteSpace {
    pub fn value(&self) -> &'static str {
        match self {
            WhiteSpace::Normal => "normal",
            WhiteSpace::NoWrap => "nowrap",
            WhiteSpace::Pre => "pre",
            WhiteSpace::PreLine => "pre-line",
            WhiteSpace::PreWrap => "pre-wrap",
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

impl WordBreak {
    pub fn value(&self) -> &'static str {
        match self {
            WordBreak::Normal => "normal",
            WordBreak::BreakAll => "break-all",
            WordBreak::KeepAll => "keep-all",
            WordBreak::BreakWord => "break-word",
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
        assert_eq!(FontSize::px(32).size_rem(), 2.0);
    }

    #[test]
    fn test_font_size_custom_and_px_values() {
        const TITLE: FontSizeVar = FontSizeVar::new("--title-size");
        assert_eq!(FontSize::var(TITLE).value(), "var(--title-size)");
        assert_eq!(FontSize::px(14).value(), "14px");
        assert_eq!(
            FontSize::var(TITLE).resolve_px(&[("--title-size", 28.0)]),
            Some(28.0)
        );
        assert_eq!(FontSize::px(18).resolve_px(&[]), Some(18.0));
    }

    #[test]
    fn test_font_size_var_standard_string_traits() {
        let var = FontSizeVar::from("--title-size");
        assert_eq!(var.as_ref(), "--title-size");
        assert_eq!(var.to_string(), "--title-size");
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Bold.value(), 700);
        assert_eq!(FontWeight::Normal.value(), 400);
    }

    #[test]
    fn test_letter_spacing() {
        assert_eq!(LetterSpacing::Tight.value(), "-0.025em");
        assert_eq!(
            LetterSpacing::var(LetterSpacingVar::new("--tracking")).value(),
            "var(--tracking)"
        );
        assert_eq!(LetterSpacing::em(0.035).value(), "0.035em");
    }

    #[test]
    fn test_letter_spacing_var_standard_string_traits() {
        let var = LetterSpacingVar::new("--tracking");
        assert_eq!(var.as_ref(), "--tracking");
        assert_eq!(var.to_string(), "--tracking");
    }

    #[test]
    fn test_line_height_custom_values() {
        assert_eq!(LineHeight::number(1.75).value(), "1.75");
        assert_eq!(
            LineHeight::var(LineHeightVar::new("--leading")).value(),
            "var(--leading)"
        );
    }

    #[test]
    fn test_line_height_var_standard_string_traits() {
        let var = LineHeightVar::new("--leading");
        assert_eq!(var.as_ref(), "--leading");
        assert_eq!(var.to_string(), "--leading");
    }
}
