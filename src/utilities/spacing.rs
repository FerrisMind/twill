//! Spacing utilities for padding and margin.

use std::fmt;

use crate::tokens::Spacing;
use crate::traits::ComputeValue;

/// Named padding variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaddingVar(&'static str);

impl PaddingVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for PaddingVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for PaddingVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for PaddingVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Padding value used by typed utility families.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaddingValue {
    /// Theme spacing token (`*-<number>` and `*-px` families).
    Scale(Spacing),
    /// Arbitrary pixel value (`*-[<value>]` for px-centric native layouts).
    Px(f32),
    /// Arbitrary rem value (`*-[<value>]`).
    Rem(f32),
    /// Custom-property-like value (`*-(<custom-property>)`).
    Var(PaddingVar),
}

impl PaddingValue {
    pub const fn scale(spacing: Spacing) -> Self {
        Self::Scale(spacing)
    }

    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }

    pub const fn rem(value: f32) -> Self {
        Self::Rem(value)
    }

    pub const fn var(name: PaddingVar) -> Self {
        Self::Var(name)
    }
}

impl From<Spacing> for PaddingValue {
    fn from(value: Spacing) -> Self {
        Self::Scale(value)
    }
}

/// Padding utility.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Padding {
    pub top: Option<PaddingValue>,
    pub right: Option<PaddingValue>,
    pub bottom: Option<PaddingValue>,
    pub left: Option<PaddingValue>,
}

impl Padding {
    pub fn new() -> Self {
        Self::default()
    }

    /// `p-*` family: set padding on all sides.
    pub fn p(value: Spacing) -> Self {
        Self::p_value(value.into())
    }

    /// `p-(<custom-property>)` / `p-[<value>]` family.
    pub fn p_value(value: PaddingValue) -> Self {
        Self::all_value(value)
    }

    pub fn all_value(value: PaddingValue) -> Self {
        Self {
            top: Some(value),
            right: Some(value),
            bottom: Some(value),
            left: Some(value),
        }
    }

    /// All sides same value
    pub fn all(value: Spacing) -> Self {
        Self::all_value(value.into())
    }

    /// Horizontal and vertical
    pub fn symmetric(vertical: Spacing, horizontal: Spacing) -> Self {
        Self::symmetric_value(vertical.into(), horizontal.into())
    }

    pub fn symmetric_value(vertical: PaddingValue, horizontal: PaddingValue) -> Self {
        Self {
            top: Some(vertical),
            right: Some(horizontal),
            bottom: Some(vertical),
            left: Some(horizontal),
        }
    }

    /// Individual sides
    pub fn individual(top: Spacing, right: Spacing, bottom: Spacing, left: Spacing) -> Self {
        Self::individual_value(top.into(), right.into(), bottom.into(), left.into())
    }

    pub fn individual_value(
        top: PaddingValue,
        right: PaddingValue,
        bottom: PaddingValue,
        left: PaddingValue,
    ) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }

    /// `px-*` family: set inline axis padding for the default LTR mapping.
    pub fn px(value: Spacing) -> Self {
        Self::px_value(value.into())
    }

    pub fn px_value(value: PaddingValue) -> Self {
        Self::x_value(value)
    }

    /// Only horizontal (x)
    pub fn x(value: Spacing) -> Self {
        Self::x_value(value.into())
    }

    pub fn x_value(value: PaddingValue) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: Some(value),
        }
    }

    /// `py-*` family: set block axis padding.
    pub fn py(value: Spacing) -> Self {
        Self::py_value(value.into())
    }

    pub fn py_value(value: PaddingValue) -> Self {
        Self::y_value(value)
    }

    /// Only vertical (y)
    pub fn y(value: Spacing) -> Self {
        Self::y_value(value.into())
    }

    pub fn y_value(value: PaddingValue) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// `pt-*` family.
    pub fn pt(value: Spacing) -> Self {
        Self::pt_value(value.into())
    }

    pub fn pt_value(value: PaddingValue) -> Self {
        Self::top_value(value)
    }

    /// Top only
    pub fn top(value: Spacing) -> Self {
        Self::top_value(value.into())
    }

    pub fn top_value(value: PaddingValue) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: None,
            left: None,
        }
    }

    /// `pr-*` family.
    pub fn pr(value: Spacing) -> Self {
        Self::pr_value(value.into())
    }

    pub fn pr_value(value: PaddingValue) -> Self {
        Self::right_value(value)
    }

    /// Right only
    pub fn right(value: Spacing) -> Self {
        Self::right_value(value.into())
    }

    pub fn right_value(value: PaddingValue) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: None,
        }
    }

    /// `pb-*` family.
    pub fn pb(value: Spacing) -> Self {
        Self::pb_value(value.into())
    }

    pub fn pb_value(value: PaddingValue) -> Self {
        Self::bottom_value(value)
    }

    /// Bottom only
    pub fn bottom(value: Spacing) -> Self {
        Self::bottom_value(value.into())
    }

    pub fn bottom_value(value: PaddingValue) -> Self {
        Self {
            top: None,
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// `pl-*` family.
    pub fn pl(value: Spacing) -> Self {
        Self::pl_value(value.into())
    }

    pub fn pl_value(value: PaddingValue) -> Self {
        Self::left_value(value)
    }

    /// Left only
    pub fn left(value: Spacing) -> Self {
        Self::left_value(value.into())
    }

    pub fn left_value(value: PaddingValue) -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: Some(value),
        }
    }

    /// `ps-*` family (`padding-inline-start`) with default LTR mapping.
    pub fn ps(value: Spacing) -> Self {
        Self::ps_value(value.into())
    }

    pub fn ps_value(value: PaddingValue) -> Self {
        Self::inline_start_value(value)
    }

    /// `pe-*` family (`padding-inline-end`) with default LTR mapping.
    pub fn pe(value: Spacing) -> Self {
        Self::pe_value(value.into())
    }

    pub fn pe_value(value: PaddingValue) -> Self {
        Self::inline_end_value(value)
    }

    /// `pbs-*` family (`padding-block-start`).
    pub fn pbs(value: Spacing) -> Self {
        Self::pbs_value(value.into())
    }

    pub fn pbs_value(value: PaddingValue) -> Self {
        Self::block_start_value(value)
    }

    /// `pbe-*` family (`padding-block-end`).
    pub fn pbe(value: Spacing) -> Self {
        Self::pbe_value(value.into())
    }

    pub fn pbe_value(value: PaddingValue) -> Self {
        Self::block_end_value(value)
    }

    /// Logical inline-start padding.
    pub fn inline_start(value: Spacing) -> Self {
        Self::inline_start_value(value.into())
    }

    pub fn inline_start_value(value: PaddingValue) -> Self {
        Self::left_value(value)
    }

    /// Logical inline-end padding.
    pub fn inline_end(value: Spacing) -> Self {
        Self::inline_end_value(value.into())
    }

    pub fn inline_end_value(value: PaddingValue) -> Self {
        Self::right_value(value)
    }

    /// Logical block-start padding.
    pub fn block_start(value: Spacing) -> Self {
        Self::block_start_value(value.into())
    }

    pub fn block_start_value(value: PaddingValue) -> Self {
        Self::top_value(value)
    }

    /// Logical block-end padding.
    pub fn block_end(value: Spacing) -> Self {
        Self::block_end_value(value.into())
    }

    pub fn block_end_value(value: PaddingValue) -> Self {
        Self::bottom_value(value)
    }
}

/// Named margin variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarginVar(&'static str);

impl MarginVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for MarginVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for MarginVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for MarginVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Margin value used by typed utility families.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarginValue {
    /// Theme spacing token (`m-<number>`, `mx-<number>`, etc).
    Scale(Spacing),
    /// Negative theme spacing token (`-m-<number>`, `-mx-<number>`, etc).
    NegativeScale(Spacing),
    /// Arbitrary pixel value (`m-[<value>]` for px-centric native layouts).
    Px(f32),
    /// Arbitrary rem value (`m-[<value>]`).
    Rem(f32),
    /// Custom-property-like value (`m-(<custom-property>)`).
    Var(MarginVar),
    /// Auto layout margin (`m-auto`, `mx-auto`, etc).
    Auto,
}

impl MarginValue {
    pub const fn scale(spacing: Spacing) -> Self {
        if matches!(spacing, Spacing::Auto) {
            Self::Auto
        } else {
            Self::Scale(spacing)
        }
    }

    pub const fn neg_scale(spacing: Spacing) -> Self {
        if matches!(spacing, Spacing::Auto) {
            Self::Auto
        } else {
            Self::NegativeScale(spacing)
        }
    }

    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }

    pub const fn rem(value: f32) -> Self {
        Self::Rem(value)
    }

    pub const fn var(name: MarginVar) -> Self {
        Self::Var(name)
    }

    pub const fn auto() -> Self {
        Self::Auto
    }
}

impl From<Spacing> for MarginValue {
    fn from(value: Spacing) -> Self {
        Self::scale(value)
    }
}

/// Margin utility.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Margin {
    pub top: Option<MarginValue>,
    pub right: Option<MarginValue>,
    pub bottom: Option<MarginValue>,
    pub left: Option<MarginValue>,
}

impl Margin {
    pub fn new() -> Self {
        Self::default()
    }

    /// `m-*` family: set all margin sides.
    pub fn m(value: Spacing) -> Self {
        Self::m_value(value.into())
    }

    /// `m-(<custom-property>)` / `m-[<value>]` family.
    pub fn m_value(value: MarginValue) -> Self {
        Self::all_value(value)
    }

    /// `-m-*` family.
    pub fn neg_m(value: Spacing) -> Self {
        Self::m_value(MarginValue::neg_scale(value))
    }

    pub fn all_value(value: MarginValue) -> Self {
        Self {
            top: Some(value),
            right: Some(value),
            bottom: Some(value),
            left: Some(value),
        }
    }

    /// All sides same value.
    pub fn all(value: Spacing) -> Self {
        Self::all_value(value.into())
    }

    /// Horizontal and vertical.
    pub fn symmetric(vertical: Spacing, horizontal: Spacing) -> Self {
        Self::symmetric_value(vertical.into(), horizontal.into())
    }

    pub fn symmetric_value(vertical: MarginValue, horizontal: MarginValue) -> Self {
        Self {
            top: Some(vertical),
            right: Some(horizontal),
            bottom: Some(vertical),
            left: Some(horizontal),
        }
    }

    /// Individual sides.
    pub fn individual(top: Spacing, right: Spacing, bottom: Spacing, left: Spacing) -> Self {
        Self::individual_value(top.into(), right.into(), bottom.into(), left.into())
    }

    pub fn individual_value(
        top: MarginValue,
        right: MarginValue,
        bottom: MarginValue,
        left: MarginValue,
    ) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }

    /// `mx-*` family: set inline axis margin for the default LTR mapping.
    pub fn mx(value: Spacing) -> Self {
        Self::mx_value(value.into())
    }

    pub fn mx_value(value: MarginValue) -> Self {
        Self::x_value(value)
    }

    /// `-mx-*` family.
    pub fn neg_mx(value: Spacing) -> Self {
        Self::mx_value(MarginValue::neg_scale(value))
    }

    /// Only horizontal (x).
    pub fn x(value: Spacing) -> Self {
        Self::x_value(value.into())
    }

    pub fn x_value(value: MarginValue) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: Some(value),
        }
    }

    /// `my-*` family: set block axis margin.
    pub fn my(value: Spacing) -> Self {
        Self::my_value(value.into())
    }

    pub fn my_value(value: MarginValue) -> Self {
        Self::y_value(value)
    }

    /// `-my-*` family.
    pub fn neg_my(value: Spacing) -> Self {
        Self::my_value(MarginValue::neg_scale(value))
    }

    /// Only vertical (y).
    pub fn y(value: Spacing) -> Self {
        Self::y_value(value.into())
    }

    pub fn y_value(value: MarginValue) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// `ms-*` family (`margin-inline-start`) with default LTR mapping.
    pub fn ms(value: Spacing) -> Self {
        Self::ms_value(value.into())
    }

    pub fn ms_value(value: MarginValue) -> Self {
        Self::inline_start_value(value)
    }

    /// `-ms-*` family.
    pub fn neg_ms(value: Spacing) -> Self {
        Self::ms_value(MarginValue::neg_scale(value))
    }

    /// `me-*` family (`margin-inline-end`) with default LTR mapping.
    pub fn me(value: Spacing) -> Self {
        Self::me_value(value.into())
    }

    pub fn me_value(value: MarginValue) -> Self {
        Self::inline_end_value(value)
    }

    /// `-me-*` family.
    pub fn neg_me(value: Spacing) -> Self {
        Self::me_value(MarginValue::neg_scale(value))
    }

    /// `mbs-*` family (`margin-block-start`).
    pub fn mbs(value: Spacing) -> Self {
        Self::mbs_value(value.into())
    }

    pub fn mbs_value(value: MarginValue) -> Self {
        Self::block_start_value(value)
    }

    /// `-mbs-*` family.
    pub fn neg_mbs(value: Spacing) -> Self {
        Self::mbs_value(MarginValue::neg_scale(value))
    }

    /// `mbe-*` family (`margin-block-end`).
    pub fn mbe(value: Spacing) -> Self {
        Self::mbe_value(value.into())
    }

    pub fn mbe_value(value: MarginValue) -> Self {
        Self::block_end_value(value)
    }

    /// `-mbe-*` family.
    pub fn neg_mbe(value: Spacing) -> Self {
        Self::mbe_value(MarginValue::neg_scale(value))
    }

    /// `mt-*` family.
    pub fn mt(value: Spacing) -> Self {
        Self::mt_value(value.into())
    }

    pub fn mt_value(value: MarginValue) -> Self {
        Self::top_value(value)
    }

    /// `-mt-*` family.
    pub fn neg_mt(value: Spacing) -> Self {
        Self::mt_value(MarginValue::neg_scale(value))
    }

    /// Top only.
    pub fn top(value: Spacing) -> Self {
        Self::top_value(value.into())
    }

    pub fn top_value(value: MarginValue) -> Self {
        Self {
            top: Some(value),
            right: None,
            bottom: None,
            left: None,
        }
    }

    /// `mr-*` family.
    pub fn mr(value: Spacing) -> Self {
        Self::mr_value(value.into())
    }

    pub fn mr_value(value: MarginValue) -> Self {
        Self::right_value(value)
    }

    /// `-mr-*` family.
    pub fn neg_mr(value: Spacing) -> Self {
        Self::mr_value(MarginValue::neg_scale(value))
    }

    /// Right only.
    pub fn right(value: Spacing) -> Self {
        Self::right_value(value.into())
    }

    pub fn right_value(value: MarginValue) -> Self {
        Self {
            top: None,
            right: Some(value),
            bottom: None,
            left: None,
        }
    }

    /// `mb-*` family.
    pub fn mb(value: Spacing) -> Self {
        Self::mb_value(value.into())
    }

    pub fn mb_value(value: MarginValue) -> Self {
        Self::bottom_value(value)
    }

    /// `-mb-*` family.
    pub fn neg_mb(value: Spacing) -> Self {
        Self::mb_value(MarginValue::neg_scale(value))
    }

    /// Bottom only.
    pub fn bottom(value: Spacing) -> Self {
        Self::bottom_value(value.into())
    }

    pub fn bottom_value(value: MarginValue) -> Self {
        Self {
            top: None,
            right: None,
            bottom: Some(value),
            left: None,
        }
    }

    /// `ml-*` family.
    pub fn ml(value: Spacing) -> Self {
        Self::ml_value(value.into())
    }

    pub fn ml_value(value: MarginValue) -> Self {
        Self::left_value(value)
    }

    /// `-ml-*` family.
    pub fn neg_ml(value: Spacing) -> Self {
        Self::ml_value(MarginValue::neg_scale(value))
    }

    /// Left only.
    pub fn left(value: Spacing) -> Self {
        Self::left_value(value.into())
    }

    pub fn left_value(value: MarginValue) -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: Some(value),
        }
    }

    /// Logical inline-start margin.
    pub fn inline_start(value: Spacing) -> Self {
        Self::inline_start_value(value.into())
    }

    pub fn inline_start_value(value: MarginValue) -> Self {
        Self::left_value(value)
    }

    /// Logical inline-end margin.
    pub fn inline_end(value: Spacing) -> Self {
        Self::inline_end_value(value.into())
    }

    pub fn inline_end_value(value: MarginValue) -> Self {
        Self::right_value(value)
    }

    /// Logical block-start margin.
    pub fn block_start(value: Spacing) -> Self {
        Self::block_start_value(value.into())
    }

    pub fn block_start_value(value: MarginValue) -> Self {
        Self::top_value(value)
    }

    /// Logical block-end margin.
    pub fn block_end(value: Spacing) -> Self {
        Self::block_end_value(value.into())
    }

    pub fn block_end_value(value: MarginValue) -> Self {
        Self::bottom_value(value)
    }

    /// Auto margins (center horizontally).
    pub fn auto_x() -> Self {
        Self::x_value(MarginValue::auto())
    }

    /// Auto margins (center vertically).
    pub fn auto_y() -> Self {
        Self::y_value(MarginValue::auto())
    }

    /// Auto all.
    pub fn auto() -> Self {
        Self::all_value(MarginValue::auto())
    }
}

/// Named width variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidthVar(&'static str);

impl WidthVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for WidthVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for WidthVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for WidthVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Named height variable for custom-property style mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeightVar(&'static str);

impl HeightVar {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for HeightVar {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl From<&'static str> for HeightVar {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for HeightVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Size utility (width/height).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    Spacing(Spacing),
    Percentage(crate::tokens::Percentage),
    Container(crate::tokens::Container),
    Auto,
    Full,
    /// Prose max-width token (65ch).
    Prose,
    ScreenWidth,
    ScreenHeight,
    Dvw,
    Dvh,
    Lvw,
    Lvh,
    Svw,
    Svh,
    MinContent,
    MaxContent,
    Fit,
    Lh,
    Var(WidthVar),
    HeightVar(HeightVar),
    Px(u16),
}

impl Size {
    pub fn value(&self) -> String {
        match self {
            Size::Spacing(s) => s.compute(),
            Size::Percentage(p) => p.value().to_string(),
            Size::Container(c) => c.value().to_string(),
            Size::Auto => "auto".to_string(),
            Size::Full => "100%".to_string(),
            Size::Prose => "65ch".to_string(),
            Size::ScreenWidth => "100vw".to_string(),
            Size::ScreenHeight => "100vh".to_string(),
            Size::Dvw => "100dvw".to_string(),
            Size::Dvh => "100dvh".to_string(),
            Size::Lvw => "100lvw".to_string(),
            Size::Lvh => "100lvh".to_string(),
            Size::Svw => "100svw".to_string(),
            Size::Svh => "100svh".to_string(),
            Size::MinContent => "min-content".to_string(),
            Size::MaxContent => "max-content".to_string(),
            Size::Fit => "fit-content".to_string(),
            Size::Lh => "1lh".to_string(),
            Size::Var(var) => format!("var({})", var.as_str()),
            Size::HeightVar(var) => format!("var({})", var.as_str()),
            Size::Px(px) => format!("{px}px"),
        }
    }
}

/// Width utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Width(Option<Size>);

impl Width {
    pub fn new(size: Size) -> Self {
        Self(Some(size))
    }

    pub const fn size(self) -> Option<Size> {
        self.0
    }

    /// Width from spacing scale (`w-<number>` and `w-px`).
    pub fn spacing(spacing: Spacing) -> Self {
        Self::new(Size::Spacing(spacing))
    }

    /// Fractional width (`w-<fraction>` / `w-full`).
    pub fn fraction(fraction: crate::tokens::Percentage) -> Self {
        Self::new(Size::Percentage(fraction))
    }

    /// Container scale width (`w-3xs` ... `w-7xl`).
    pub fn container(container: crate::tokens::Container) -> Self {
        Self::new(Size::Container(container))
    }

    pub fn full() -> Self {
        Self(Some(Size::Full))
    }
    pub fn auto() -> Self {
        Self(Some(Size::Auto))
    }
    pub fn screen() -> Self {
        Self(Some(Size::ScreenWidth))
    }

    /// `w-<number>` and `w-px` family.
    pub fn w(spacing: Spacing) -> Self {
        Self::spacing(spacing)
    }

    /// `w-<fraction>` family.
    pub fn w_fraction(fraction: crate::tokens::Percentage) -> Self {
        Self::fraction(fraction)
    }

    /// `w-3xs` ... `w-7xl` family.
    pub fn w_container(container: crate::tokens::Container) -> Self {
        Self::container(container)
    }

    /// `w-auto`.
    pub fn w_auto() -> Self {
        Self::auto()
    }

    /// `w-px`.
    pub fn w_px() -> Self {
        Self::spacing(Spacing::Px)
    }

    /// `w-full`.
    pub fn w_full() -> Self {
        Self::full()
    }

    /// `w-screen`.
    pub fn w_screen() -> Self {
        Self::screen()
    }

    /// `w-dvw`.
    pub fn w_dvw() -> Self {
        Self::new(Size::Dvw)
    }

    /// `w-dvh`.
    pub fn w_dvh() -> Self {
        Self::new(Size::Dvh)
    }

    /// `w-lvw`.
    pub fn w_lvw() -> Self {
        Self::new(Size::Lvw)
    }

    /// `w-lvh`.
    pub fn w_lvh() -> Self {
        Self::new(Size::Lvh)
    }

    /// `w-svw`.
    pub fn w_svw() -> Self {
        Self::new(Size::Svw)
    }

    /// `w-svh`.
    pub fn w_svh() -> Self {
        Self::new(Size::Svh)
    }

    /// `w-min`.
    pub fn w_min() -> Self {
        Self::new(Size::MinContent)
    }

    /// `w-max`.
    pub fn w_max() -> Self {
        Self::new(Size::MaxContent)
    }

    /// `w-fit`.
    pub fn w_fit() -> Self {
        Self::new(Size::Fit)
    }

    /// `w-(<custom-property>)`.
    pub fn w_var(var: WidthVar) -> Self {
        Self::new(Size::Var(var))
    }

    /// Typed pixel width value (`w-[<value>]` in px-focused native layouts).
    pub fn w_px_value(px: u16) -> Self {
        Self::new(Size::Px(px))
    }
}

impl From<Size> for Width {
    fn from(value: Size) -> Self {
        Self::new(value)
    }
}

impl From<Spacing> for Width {
    fn from(value: Spacing) -> Self {
        Self::spacing(value)
    }
}

impl From<crate::tokens::Percentage> for Width {
    fn from(value: crate::tokens::Percentage) -> Self {
        Self::fraction(value)
    }
}

impl From<crate::tokens::Container> for Width {
    fn from(value: crate::tokens::Container) -> Self {
        Self::container(value)
    }
}

/// Height utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Height(Option<Size>);

impl Height {
    pub fn new(size: Size) -> Self {
        Self(Some(size))
    }

    pub const fn size(self) -> Option<Size> {
        self.0
    }
    /// Height from spacing scale (`h-<number>` and `h-px`).
    pub fn spacing(spacing: Spacing) -> Self {
        Self::new(Size::Spacing(spacing))
    }
    /// Fractional height (`h-<fraction>` / `h-full`).
    pub fn fraction(fraction: crate::tokens::Percentage) -> Self {
        Self::new(Size::Percentage(fraction))
    }
    pub fn full() -> Self {
        Self(Some(Size::Full))
    }
    pub fn auto() -> Self {
        Self(Some(Size::Auto))
    }
    pub fn screen() -> Self {
        Self(Some(Size::ScreenHeight))
    }

    /// `h-<number>` and `h-px` family.
    pub fn h(spacing: Spacing) -> Self {
        Self::spacing(spacing)
    }

    /// `h-<fraction>` family.
    pub fn h_fraction(fraction: crate::tokens::Percentage) -> Self {
        Self::fraction(fraction)
    }

    /// `h-auto`.
    pub fn h_auto() -> Self {
        Self::auto()
    }

    /// `h-px`.
    pub fn h_px() -> Self {
        Self::spacing(Spacing::Px)
    }

    /// `h-full`.
    pub fn h_full() -> Self {
        Self::full()
    }

    /// `h-screen`.
    pub fn h_screen() -> Self {
        Self::screen()
    }

    /// `h-dvw`.
    pub fn h_dvw() -> Self {
        Self::new(Size::Dvw)
    }

    /// `h-dvh`.
    pub fn h_dvh() -> Self {
        Self::new(Size::Dvh)
    }

    /// `h-lvw`.
    pub fn h_lvw() -> Self {
        Self::new(Size::Lvw)
    }

    /// `h-lvh`.
    pub fn h_lvh() -> Self {
        Self::new(Size::Lvh)
    }

    /// `h-svw`.
    pub fn h_svw() -> Self {
        Self::new(Size::Svw)
    }

    /// `h-svh`.
    pub fn h_svh() -> Self {
        Self::new(Size::Svh)
    }

    /// `h-min`.
    pub fn h_min() -> Self {
        Self::new(Size::MinContent)
    }

    /// `h-max`.
    pub fn h_max() -> Self {
        Self::new(Size::MaxContent)
    }

    /// `h-fit`.
    pub fn h_fit() -> Self {
        Self::new(Size::Fit)
    }

    /// `h-lh`.
    pub fn h_lh() -> Self {
        Self::new(Size::Lh)
    }

    /// `h-(<custom-property>)`.
    pub fn h_var(var: HeightVar) -> Self {
        Self::new(Size::HeightVar(var))
    }

    /// Typed pixel height value (`h-[<value>]` in px-focused native layouts).
    pub fn h_px_value(px: u16) -> Self {
        Self::new(Size::Px(px))
    }
}

impl From<Size> for Height {
    fn from(value: Size) -> Self {
        Self::new(value)
    }
}

impl From<Spacing> for Height {
    fn from(value: Spacing) -> Self {
        Self::spacing(value)
    }
}

impl From<crate::tokens::Percentage> for Height {
    fn from(value: crate::tokens::Percentage) -> Self {
        Self::fraction(value)
    }
}

impl From<crate::tokens::Container> for Height {
    fn from(value: crate::tokens::Container) -> Self {
        Self::new(Size::Container(value))
    }
}

/// Min/Max width constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SizeConstraints {
    pub(crate) min_width: Option<Size>,
    pub(crate) max_width: Option<Size>,
    pub(crate) min_height: Option<Size>,
    pub(crate) max_height: Option<Size>,
}

impl SizeConstraints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_width(mut self, size: Size) -> Self {
        self.min_width = Some(size);
        self
    }
    pub fn max_width(mut self, size: Size) -> Self {
        self.max_width = Some(size);
        self
    }
    pub fn min_height(mut self, size: Size) -> Self {
        self.min_height = Some(size);
        self
    }
    pub fn max_height(mut self, size: Size) -> Self {
        self.max_height = Some(size);
        self
    }

    pub const fn min_width_value(&self) -> Option<Size> {
        self.min_width
    }

    pub const fn max_width_value(&self) -> Option<Size> {
        self.max_width
    }

    pub const fn min_height_value(&self) -> Option<Size> {
        self.min_height
    }

    pub const fn max_height_value(&self) -> Option<Size> {
        self.max_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_all() {
        let p = Padding::all(Spacing::S4);
        assert_eq!(p.top, Some(PaddingValue::scale(Spacing::S4)));
        assert_eq!(p.right, Some(PaddingValue::scale(Spacing::S4)));
        assert_eq!(p.bottom, Some(PaddingValue::scale(Spacing::S4)));
        assert_eq!(p.left, Some(PaddingValue::scale(Spacing::S4)));
    }

    #[test]
    fn test_padding_symmetric() {
        let p = Padding::symmetric(Spacing::S2, Spacing::S4);
        assert_eq!(p.top, Some(PaddingValue::scale(Spacing::S2)));
        assert_eq!(p.right, Some(PaddingValue::scale(Spacing::S4)));
        assert_eq!(p.bottom, Some(PaddingValue::scale(Spacing::S2)));
        assert_eq!(p.left, Some(PaddingValue::scale(Spacing::S4)));
    }

    #[test]
    fn test_padding_family_aliases() {
        assert_eq!(Padding::p(Spacing::S4), Padding::all(Spacing::S4));
        assert_eq!(Padding::px(Spacing::S3), Padding::x(Spacing::S3));
        assert_eq!(Padding::py(Spacing::S2), Padding::y(Spacing::S2));
        assert_eq!(Padding::pt(Spacing::S1), Padding::top(Spacing::S1));
        assert_eq!(Padding::pr(Spacing::S1), Padding::right(Spacing::S1));
        assert_eq!(Padding::pb(Spacing::S1), Padding::bottom(Spacing::S1));
        assert_eq!(Padding::pl(Spacing::S1), Padding::left(Spacing::S1));
    }

    #[test]
    fn test_padding_px_token_family() {
        assert_eq!(Padding::p(Spacing::Px), Padding::all(Spacing::Px));
        assert_eq!(Padding::px(Spacing::Px), Padding::x(Spacing::Px));
        assert_eq!(Padding::py(Spacing::Px), Padding::y(Spacing::Px));
        assert_eq!(Padding::ps(Spacing::Px), Padding::left(Spacing::Px));
        assert_eq!(Padding::pe(Spacing::Px), Padding::right(Spacing::Px));
        assert_eq!(Padding::pbs(Spacing::Px), Padding::top(Spacing::Px));
        assert_eq!(Padding::pbe(Spacing::Px), Padding::bottom(Spacing::Px));
    }

    #[test]
    fn test_padding_logical_aliases() {
        assert_eq!(Padding::ps(Spacing::S4), Padding::left(Spacing::S4));
        assert_eq!(Padding::pe(Spacing::S4), Padding::right(Spacing::S4));
        assert_eq!(Padding::pbs(Spacing::S4), Padding::top(Spacing::S4));
        assert_eq!(Padding::pbe(Spacing::S4), Padding::bottom(Spacing::S4));
    }

    #[test]
    fn test_padding_value_families() {
        let value = PaddingValue::px(5.0);
        let p = Padding::px_value(value);
        assert_eq!(p.left, Some(PaddingValue::Px(5.0)));
        assert_eq!(p.right, Some(PaddingValue::Px(5.0)));
        assert_eq!(p.top, None);
        assert_eq!(p.bottom, None);
    }

    #[test]
    fn test_padding_custom_property_variant() {
        const PAD: PaddingVar = PaddingVar::new("--pad");
        let p = Padding::p_value(PaddingValue::var(PAD));
        assert_eq!(p.top, Some(PaddingValue::Var(PAD)));
        assert_eq!(p.right, Some(PaddingValue::Var(PAD)));
        assert_eq!(p.bottom, Some(PaddingValue::Var(PAD)));
        assert_eq!(p.left, Some(PaddingValue::Var(PAD)));
    }

    #[test]
    fn test_variable_wrappers_support_standard_string_traits() {
        let padding_var = PaddingVar::from("--pad");
        let margin_var = MarginVar::from("--margin");
        let width_var = WidthVar::from("--width");
        let height_var = HeightVar::from("--height");

        assert_eq!(padding_var.as_ref(), "--pad");
        assert_eq!(margin_var.to_string(), "--margin");
        assert_eq!(width_var.as_str(), "--width");
        assert_eq!(height_var.to_string(), "--height");
    }

    #[test]
    fn test_margin_class_aliases() {
        assert_eq!(Margin::m(Spacing::S4), Margin::all(Spacing::S4));
        assert_eq!(Margin::mx(Spacing::S3), Margin::x(Spacing::S3));
        assert_eq!(Margin::my(Spacing::S2), Margin::y(Spacing::S2));
        assert_eq!(Margin::mt(Spacing::S1), Margin::top(Spacing::S1));
        assert_eq!(Margin::mr(Spacing::S1), Margin::right(Spacing::S1));
        assert_eq!(Margin::mb(Spacing::S1), Margin::bottom(Spacing::S1));
        assert_eq!(Margin::ml(Spacing::S1), Margin::left(Spacing::S1));
    }

    #[test]
    fn test_margin_logical_aliases() {
        assert_eq!(Margin::ms(Spacing::S4), Margin::left(Spacing::S4));
        assert_eq!(Margin::me(Spacing::S4), Margin::right(Spacing::S4));
        assert_eq!(Margin::mbs(Spacing::S4), Margin::top(Spacing::S4));
        assert_eq!(Margin::mbe(Spacing::S4), Margin::bottom(Spacing::S4));
    }

    #[test]
    fn test_margin_negative_scale_value() {
        let m = Margin::neg_mt(Spacing::S8);
        assert_eq!(m.top, Some(MarginValue::NegativeScale(Spacing::S8)));
    }

    #[test]
    fn test_margin_auto() {
        let m = Margin::auto_x();
        assert_eq!(m.right, Some(MarginValue::Auto));
        assert_eq!(m.left, Some(MarginValue::Auto));
    }

    #[test]
    fn test_margin_custom_property_variant() {
        const MARGIN_X: MarginVar = MarginVar::new("--margin-x");
        let m = Margin::mx_value(MarginValue::var(MARGIN_X));
        assert_eq!(m.left, Some(MarginValue::Var(MARGIN_X)));
        assert_eq!(m.right, Some(MarginValue::Var(MARGIN_X)));
    }

    #[test]
    fn test_screen_size_value() {
        assert_eq!(Width::screen().size(), Some(Size::ScreenWidth));
        assert_eq!(Height::screen().size(), Some(Size::ScreenHeight));
    }

    #[test]
    fn test_width_family_constructors() {
        assert_eq!(
            Width::w(Spacing::S24).size(),
            Some(Size::Spacing(Spacing::S24))
        );
        assert_eq!(
            Width::w_fraction(crate::tokens::Percentage::S1_2).size(),
            Some(Size::Percentage(crate::tokens::Percentage::S1_2))
        );
        assert_eq!(
            Width::w_container(crate::tokens::Container::Md).size(),
            Some(Size::Container(crate::tokens::Container::Md))
        );
        assert_eq!(Width::w_px().size(), Some(Size::Spacing(Spacing::Px)));
        assert_eq!(Width::w_full().size(), Some(Size::Full));
        assert_eq!(Width::w_auto().size(), Some(Size::Auto));
        assert_eq!(Width::w_screen().size(), Some(Size::ScreenWidth));
        assert_eq!(Width::w_dvw().size(), Some(Size::Dvw));
        assert_eq!(Width::w_lvw().size(), Some(Size::Lvw));
        assert_eq!(Width::w_svw().size(), Some(Size::Svw));
        assert_eq!(Width::w_min().size(), Some(Size::MinContent));
        assert_eq!(Width::w_max().size(), Some(Size::MaxContent));
        assert_eq!(Width::w_fit().size(), Some(Size::Fit));
    }

    #[test]
    fn test_width_custom_value_constructors() {
        const PANEL_W: WidthVar = WidthVar::new("--panel-w");
        assert_eq!(Width::w_var(PANEL_W).size(), Some(Size::Var(PANEL_W)));
        assert_eq!(Width::w_px_value(280).size(), Some(Size::Px(280)));
    }

    #[test]
    fn test_height_family_constructors() {
        assert_eq!(
            Height::h(Spacing::S24).size(),
            Some(Size::Spacing(Spacing::S24))
        );
        assert_eq!(
            Height::h_fraction(crate::tokens::Percentage::S1_2).size(),
            Some(Size::Percentage(crate::tokens::Percentage::S1_2))
        );
        assert_eq!(Height::h_px().size(), Some(Size::Spacing(Spacing::Px)));
        assert_eq!(Height::h_full().size(), Some(Size::Full));
        assert_eq!(Height::h_auto().size(), Some(Size::Auto));
        assert_eq!(Height::h_screen().size(), Some(Size::ScreenHeight));
        assert_eq!(Height::h_dvw().size(), Some(Size::Dvw));
        assert_eq!(Height::h_dvh().size(), Some(Size::Dvh));
        assert_eq!(Height::h_lvw().size(), Some(Size::Lvw));
        assert_eq!(Height::h_lvh().size(), Some(Size::Lvh));
        assert_eq!(Height::h_svw().size(), Some(Size::Svw));
        assert_eq!(Height::h_svh().size(), Some(Size::Svh));
        assert_eq!(Height::h_min().size(), Some(Size::MinContent));
        assert_eq!(Height::h_max().size(), Some(Size::MaxContent));
        assert_eq!(Height::h_fit().size(), Some(Size::Fit));
        assert_eq!(Height::h_lh().size(), Some(Size::Lh));
    }

    #[test]
    fn test_height_custom_value_constructors() {
        const PANEL_H: HeightVar = HeightVar::new("--panel-h");
        assert_eq!(
            Height::h_var(PANEL_H).size(),
            Some(Size::HeightVar(PANEL_H))
        );
        assert_eq!(Height::h_px_value(320).size(), Some(Size::Px(320)));
    }

    #[test]
    fn test_size_value_width_variants() {
        assert_eq!(
            Size::Container(crate::tokens::Container::Sm).value(),
            "24rem"
        );
        assert_eq!(Size::Dvw.value(), "100dvw");
        assert_eq!(Size::Dvh.value(), "100dvh");
        assert_eq!(Size::Lvw.value(), "100lvw");
        assert_eq!(Size::Lvh.value(), "100lvh");
        assert_eq!(Size::Svw.value(), "100svw");
        assert_eq!(Size::Svh.value(), "100svh");
        assert_eq!(Size::Lh.value(), "1lh");
        assert_eq!(Size::Var(WidthVar::new("--w")).value(), "var(--w)");
        assert_eq!(Size::HeightVar(HeightVar::new("--h")).value(), "var(--h)");
        assert_eq!(Size::Px(5).value(), "5px");
    }

    #[test]
    fn test_prose_size_value() {
        assert_eq!(Size::Prose.value(), "65ch");
    }
}
