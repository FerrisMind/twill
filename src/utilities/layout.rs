//! Layout utilities for flexbox, grid, display, and positioning.

use std::num::{NonZeroU8, NonZeroU16};

use crate::tokens::Container;
use crate::tokens::Spacing;

/// Display type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Display {
    Block,
    InlineBlock,
    Inline,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    Hidden,
    Contents,
    FlowRoot,
}

impl Display {
    pub fn value(&self) -> &'static str {
        match self {
            Display::Block => "block",
            Display::InlineBlock => "inline-block",
            Display::Inline => "inline",
            Display::Flex => "flex",
            Display::InlineFlex => "inline-flex",
            Display::Grid => "grid",
            Display::InlineGrid => "inline-grid",
            Display::Hidden => "none",
            Display::Contents => "contents",
            Display::FlowRoot => "flow-root",
        }
    }
}

/// Position type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    pub fn value(&self) -> &'static str {
        match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Absolute => "absolute",
            Position::Fixed => "fixed",
            Position::Sticky => "sticky",
        }
    }
}

/// Flex direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Col,
    ColReverse,
}

impl FlexDirection {
    pub fn value(&self) -> &'static str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Col => "column",
            FlexDirection::ColReverse => "column-reverse",
        }
    }

    pub fn class_name(&self) -> &'static str {
        match self {
            FlexDirection::Row => "flex-row",
            FlexDirection::RowReverse => "flex-row-reverse",
            FlexDirection::Col => "flex-col",
            FlexDirection::ColReverse => "flex-col-reverse",
        }
    }

    /// Parse Tailwind `flex-direction` utility class.
    ///
    /// Supported:
    /// - `flex-row`
    /// - `flex-row-reverse`
    /// - `flex-col`
    /// - `flex-col-reverse`
    pub fn from_tailwind_class(class: &str) -> Option<Self> {
        match class {
            "flex-row" => Some(Self::Row),
            "flex-row-reverse" => Some(Self::RowReverse),
            "flex-col" => Some(Self::Col),
            "flex-col-reverse" => Some(Self::ColReverse),
            _ => None,
        }
    }
}

/// Flex wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexWrap {
    Wrap,
    WrapReverse,
    NoWrap,
}

impl FlexWrap {
    pub fn value(&self) -> &'static str {
        match self {
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
            FlexWrap::NoWrap => "nowrap",
        }
    }
}

/// Justify content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

impl JustifyContent {
    pub fn value(&self) -> &'static str {
        match self {
            JustifyContent::Start => "flex-start",
            JustifyContent::End => "flex-end",
            JustifyContent::Center => "center",
            JustifyContent::Between => "space-between",
            JustifyContent::Around => "space-around",
            JustifyContent::Evenly => "space-evenly",
        }
    }
}

/// Align items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn value(&self) -> &'static str {
        match self {
            AlignItems::Start => "flex-start",
            AlignItems::End => "flex-end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
            AlignItems::Stretch => "stretch",
        }
    }
}

/// Align self.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignSelf {
    Auto,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignSelf {
    pub fn value(&self) -> &'static str {
        match self {
            AlignSelf::Auto => "auto",
            AlignSelf::Start => "flex-start",
            AlignSelf::End => "flex-end",
            AlignSelf::Center => "center",
            AlignSelf::Baseline => "baseline",
            AlignSelf::Stretch => "stretch",
        }
    }
}

/// Flex grow/shrink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexGrow {
    S0,
    S1,
}

impl FlexGrow {
    pub fn value(&self) -> u8 {
        match self {
            FlexGrow::S0 => 0,
            FlexGrow::S1 => 1,
        }
    }
}

/// Flex shorthand utility.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Flex {
    /// flex: <number>
    Number(u16),
    /// flex: calc(<fraction> * 100%)
    Fraction {
        numerator: u16,
        denominator: NonZeroU16,
    },
    /// flex: 1 1 auto
    Auto,
    /// flex: 0 1 auto
    Initial,
    /// flex: none
    None,
    /// flex: var(<custom-property>)
    CustomProperty(String),
    /// flex: <value>
    Arbitrary(String),
}

impl Flex {
    pub fn number(number: u16) -> Self {
        Self::Number(number)
    }

    pub fn fraction(numerator: u16, denominator: u16) -> Self {
        let denominator = denominator.max(1);
        Self::Fraction {
            numerator,
            denominator: NonZeroU16::new(denominator)
                .expect("denominator is clamped to at least 1"),
        }
    }

    pub fn custom_property(name: impl Into<String>) -> Self {
        Self::CustomProperty(name.into())
    }

    pub fn arbitrary(value: impl Into<String>) -> Self {
        let normalized = value.into().replace('_', " ");
        Self::Arbitrary(normalized)
    }

    /// Parse Tailwind `flex-*` class value.
    ///
    /// Supported:
    /// - `flex-<number>`
    /// - `flex-<fraction>`
    /// - `flex-auto`
    /// - `flex-initial`
    /// - `flex-none`
    /// - `flex-(<custom-property>)`
    /// - `flex-[<value>]`
    pub fn from_tailwind_class(class: &str) -> Option<Self> {
        if class == "flex-auto" {
            return Some(Self::Auto);
        }
        if class == "flex-initial" {
            return Some(Self::Initial);
        }
        if class == "flex-none" {
            return Some(Self::None);
        }

        let raw = class.strip_prefix("flex-")?;

        if raw.starts_with('(') && raw.ends_with(')') && raw.len() > 2 {
            return Some(Self::custom_property(raw[1..raw.len() - 1].to_string()));
        }

        if raw.starts_with('[') && raw.ends_with(']') && raw.len() > 2 {
            return Some(Self::arbitrary(raw[1..raw.len() - 1].to_string()));
        }

        if let Some((num, den)) = raw.split_once('/') {
            let numerator = num.parse::<u16>().ok()?;
            let denominator = den.parse::<u16>().ok()?;
            return Some(Self::fraction(numerator, denominator));
        }

        raw.parse::<u16>().ok().map(Self::number)
    }

    pub fn value(&self) -> String {
        match self {
            Flex::Number(number) => number.to_string(),
            Flex::Fraction {
                numerator,
                denominator,
            } => format!("calc({numerator}/{} * 100%)", denominator.get()),
            Flex::Auto => "auto".to_string(),
            Flex::Initial => "0 auto".to_string(),
            Flex::None => "none".to_string(),
            Flex::CustomProperty(name) => format!("var({name})"),
            Flex::Arbitrary(value) => value.clone(),
        }
    }
}

/// Flex container utility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FlexContainer {
    pub direction: Option<FlexDirection>,
    pub wrap: Option<FlexWrap>,
    pub justify: Option<JustifyContent>,
    pub align: Option<AlignItems>,
    pub gap: Option<Spacing>,
    pub row_gap: Option<Spacing>,
    pub col_gap: Option<Spacing>,
}

impl FlexContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify = Some(justify);
        self
    }

    pub fn align(mut self, align: AlignItems) -> Self {
        self.align = Some(align);
        self
    }

    pub fn gap(mut self, gap: Spacing) -> Self {
        self.gap = Some(gap);
        self
    }

    pub fn row_gap(mut self, gap: Spacing) -> Self {
        self.row_gap = Some(gap);
        self
    }

    pub fn col_gap(mut self, gap: Spacing) -> Self {
        self.col_gap = Some(gap);
        self
    }

    /// Row direction (flex-row)
    pub fn row() -> Self {
        Self::new().direction(FlexDirection::Row)
    }

    /// Column direction (flex-col)
    pub fn col() -> Self {
        Self::new().direction(FlexDirection::Col)
    }

    /// Row reverse direction (flex-row-reverse)
    pub fn row_reverse() -> Self {
        Self::new().direction(FlexDirection::RowReverse)
    }

    /// Column reverse direction (flex-col-reverse)
    pub fn col_reverse() -> Self {
        Self::new().direction(FlexDirection::ColReverse)
    }

    /// Centered row
    pub fn centered_row() -> Self {
        Self::row()
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
    }

    /// Centered column
    pub fn centered_col() -> Self {
        Self::col()
            .justify(JustifyContent::Center)
            .align(AlignItems::Center)
    }
}

/// Object fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

impl ObjectFit {
    pub fn value(&self) -> &'static str {
        match self {
            ObjectFit::Contain => "contain",
            ObjectFit::Cover => "cover",
            ObjectFit::Fill => "fill",
            ObjectFit::None => "none",
            ObjectFit::ScaleDown => "scale-down",
        }
    }
}

/// Grid template columns/rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridTemplate {
    Cols1,
    Cols2,
    Cols3,
    Cols4,
    Cols5,
    Cols6,
    Cols12,
    None,
}

impl GridTemplate {
    pub fn track_count(&self) -> Option<u8> {
        match self {
            GridTemplate::Cols1 => Some(1),
            GridTemplate::Cols2 => Some(2),
            GridTemplate::Cols3 => Some(3),
            GridTemplate::Cols4 => Some(4),
            GridTemplate::Cols5 => Some(5),
            GridTemplate::Cols6 => Some(6),
            GridTemplate::Cols12 => Some(12),
            GridTemplate::None => None,
        }
    }
}

/// Multi-column layout utility (`columns-*`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Columns {
    /// Explicit number of columns (`columns-<number>`).
    Count(NonZeroU8),
    /// Ideal column width from container scale (`columns-3xs` ... `columns-7xl`).
    Width(Container),
    /// Custom ideal column width in logical pixels.
    WidthPx(u16),
    /// Automatic behavior (`columns-auto`).
    Auto,
}

impl Columns {
    /// Create `columns-<number>`. `0` is normalized to `1`.
    pub fn count(count: u8) -> Self {
        let count = count.max(1);
        let non_zero = NonZeroU8::new(count).expect("count is clamped to at least 1");
        Self::Count(non_zero)
    }

    /// Create width-based columns (`columns-3xs` ... `columns-7xl`).
    pub fn width(width: Container) -> Self {
        Self::Width(width)
    }

    /// Create custom width-based columns from logical pixels.
    pub fn width_px(width: f32) -> Self {
        let width = width.round().clamp(1.0, u16::MAX as f32) as u16;
        Self::WidthPx(width)
    }

    /// Create `columns-auto`.
    pub fn auto() -> Self {
        Self::Auto
    }
}

/// Grid container utility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GridContainer {
    pub columns: Option<GridTemplate>,
    pub rows: Option<GridTemplate>,
    pub gap: Option<Spacing>,
    pub row_gap: Option<Spacing>,
    pub col_gap: Option<Spacing>,
    pub justify: Option<JustifyContent>,
    pub align: Option<AlignItems>,
}

impl GridContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn columns(mut self, cols: GridTemplate) -> Self {
        self.columns = Some(cols);
        self
    }

    pub fn rows(mut self, rows: GridTemplate) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn gap(mut self, gap: Spacing) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Grid with 2 columns
    pub fn cols_2() -> Self {
        Self::new().columns(GridTemplate::Cols2)
    }

    /// Grid with 3 columns
    pub fn cols_3() -> Self {
        Self::new().columns(GridTemplate::Cols3)
    }

    /// Grid with 4 columns
    pub fn cols_4() -> Self {
        Self::new().columns(GridTemplate::Cols4)
    }
}

/// Overflow behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Overflow {
    Auto,
    Hidden,
    Clip,
    Visible,
    Scroll,
}

impl Overflow {
    pub fn value(&self) -> &'static str {
        match self {
            Overflow::Auto => "auto",
            Overflow::Hidden => "hidden",
            Overflow::Clip => "clip",
            Overflow::Visible => "visible",
            Overflow::Scroll => "scroll",
        }
    }
}

/// Z-index values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZIndex {
    Auto,
    S0,
    S10,
    S20,
    S30,
    S40,
    S50,
}

impl ZIndex {
    pub fn value(&self) -> Option<i16> {
        match self {
            ZIndex::Auto => None,
            ZIndex::S0 => Some(0),
            ZIndex::S10 => Some(10),
            ZIndex::S20 => Some(20),
            ZIndex::S30 => Some(30),
            ZIndex::S40 => Some(40),
            ZIndex::S50 => Some(50),
        }
    }
}

/// Visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Visibility {
    pub fn value(&self) -> &'static str {
        match self {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Collapse => "collapse",
        }
    }
}

/// Place content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceContent {
    Center,
    Start,
    End,
    Between,
    Around,
    Evenly,
    Baseline,
    Stretch,
}

impl PlaceContent {
    pub fn value(&self) -> &'static str {
        match self {
            PlaceContent::Center => "center",
            PlaceContent::Start => "start",
            PlaceContent::End => "end",
            PlaceContent::Between => "space-between",
            PlaceContent::Around => "space-around",
            PlaceContent::Evenly => "space-evenly",
            PlaceContent::Baseline => "baseline",
            PlaceContent::Stretch => "stretch",
        }
    }
}

/// Place items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceItems {
    Center,
    Start,
    End,
    Baseline,
    Stretch,
}

impl PlaceItems {
    pub fn value(&self) -> &'static str {
        match self {
            PlaceItems::Center => "center",
            PlaceItems::Start => "start",
            PlaceItems::End => "end",
            PlaceItems::Baseline => "baseline",
            PlaceItems::Stretch => "stretch",
        }
    }
}

/// Justify items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifyItems {
    Normal,
    Center,
    Start,
    End,
    Stretch,
}

impl JustifyItems {
    pub fn value(&self) -> &'static str {
        match self {
            JustifyItems::Normal => "normal",
            JustifyItems::Center => "center",
            JustifyItems::Start => "start",
            JustifyItems::End => "end",
            JustifyItems::Stretch => "stretch",
        }
    }
}

/// Justify self.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

impl JustifySelf {
    pub fn value(&self) -> &'static str {
        match self {
            JustifySelf::Auto => "auto",
            JustifySelf::Start => "start",
            JustifySelf::End => "end",
            JustifySelf::Center => "center",
            JustifySelf::Stretch => "stretch",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_container_builder() {
        let flex = FlexContainer::centered_row().gap(Spacing::S4);
        assert_eq!(flex.direction, Some(FlexDirection::Row));
        assert_eq!(flex.justify, Some(JustifyContent::Center));
        assert_eq!(flex.align, Some(AlignItems::Center));
        assert_eq!(flex.gap, Some(Spacing::S4));
    }

    #[test]
    fn test_parse_flex_direction_tailwind_class() {
        assert_eq!(
            FlexDirection::from_tailwind_class("flex-row"),
            Some(FlexDirection::Row)
        );
        assert_eq!(
            FlexDirection::from_tailwind_class("flex-row-reverse"),
            Some(FlexDirection::RowReverse)
        );
        assert_eq!(
            FlexDirection::from_tailwind_class("flex-col"),
            Some(FlexDirection::Col)
        );
        assert_eq!(
            FlexDirection::from_tailwind_class("flex-col-reverse"),
            Some(FlexDirection::ColReverse)
        );
        assert_eq!(FlexDirection::from_tailwind_class("flex"), None);
    }

    #[test]
    fn test_flex_direction_class_name_roundtrip() {
        let directions = [
            FlexDirection::Row,
            FlexDirection::RowReverse,
            FlexDirection::Col,
            FlexDirection::ColReverse,
        ];

        for direction in directions {
            assert_eq!(
                FlexDirection::from_tailwind_class(direction.class_name()),
                Some(direction)
            );
        }
    }

    #[test]
    fn test_grid_container_builder() {
        let grid = GridContainer::cols_3().gap(Spacing::S2);
        assert_eq!(grid.columns, Some(GridTemplate::Cols3));
        assert_eq!(grid.gap, Some(Spacing::S2));
    }

    #[test]
    fn test_columns_count_clamps_zero() {
        let columns = Columns::count(0);
        assert_eq!(
            columns,
            Columns::Count(std::num::NonZeroU8::new(1).expect("non-zero"))
        );
    }

    #[test]
    fn test_flex_values() {
        assert_eq!(Flex::number(2).value(), "2");
        assert_eq!(Flex::fraction(1, 2).value(), "calc(1/2 * 100%)");
        assert_eq!(Flex::Auto.value(), "auto");
        assert_eq!(Flex::Initial.value(), "0 auto");
        assert_eq!(Flex::None.value(), "none");
        assert_eq!(Flex::custom_property("--my-flex").value(), "var(--my-flex)");
        assert_eq!(Flex::arbitrary("3_1_auto").value(), "3 1 auto");
    }

    #[test]
    fn test_parse_flex_tailwind_class() {
        assert_eq!(Flex::from_tailwind_class("flex-1"), Some(Flex::number(1)));
        assert_eq!(
            Flex::from_tailwind_class("flex-1/2"),
            Some(Flex::fraction(1, 2))
        );
        assert_eq!(Flex::from_tailwind_class("flex-auto"), Some(Flex::Auto));
        assert_eq!(
            Flex::from_tailwind_class("flex-initial"),
            Some(Flex::Initial)
        );
        assert_eq!(Flex::from_tailwind_class("flex-none"), Some(Flex::None));
        assert_eq!(
            Flex::from_tailwind_class("flex-(--my-flex)"),
            Some(Flex::custom_property("--my-flex"))
        );
        assert_eq!(
            Flex::from_tailwind_class("flex-[3_1_auto]"),
            Some(Flex::arbitrary("3_1_auto"))
        );
        assert_eq!(Flex::from_tailwind_class("flex-nope"), None);
    }
}
