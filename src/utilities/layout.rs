//! Layout utilities for flexbox, grid, display, and positioning.

use std::fmt;
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
    pub const fn value(&self) -> &'static str {
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

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
        match self {
            Position::Static => "static",
            Position::Relative => "relative",
            Position::Absolute => "absolute",
            Position::Fixed => "fixed",
            Position::Sticky => "sticky",
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Col => "column",
            FlexDirection::ColReverse => "column-reverse",
        }
    }
}

impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
        match self {
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
            FlexWrap::NoWrap => "nowrap",
        }
    }
}

impl fmt::Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

/// Justify content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    Start,
    End,
    EndSafe,
    Center,
    CenterSafe,
    Between,
    Around,
    Evenly,
    Stretch,
    Baseline,
    Normal,
}

impl JustifyContent {
    pub const fn value(&self) -> &'static str {
        match self {
            JustifyContent::Start => "flex-start",
            JustifyContent::End => "flex-end",
            JustifyContent::EndSafe => "safe flex-end",
            JustifyContent::Center => "center",
            JustifyContent::CenterSafe => "safe center",
            JustifyContent::Between => "space-between",
            JustifyContent::Around => "space-around",
            JustifyContent::Evenly => "space-evenly",
            JustifyContent::Stretch => "stretch",
            JustifyContent::Baseline => "baseline",
            JustifyContent::Normal => "normal",
        }
    }
}

impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

/// Align items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignItems {
    Start,
    End,
    EndSafe,
    Center,
    CenterSafe,
    Baseline,
    BaselineLast,
    Stretch,
}

impl AlignItems {
    pub const fn value(&self) -> &'static str {
        match self {
            AlignItems::Start => "flex-start",
            AlignItems::End => "flex-end",
            AlignItems::EndSafe => "safe flex-end",
            AlignItems::Center => "center",
            AlignItems::CenterSafe => "safe center",
            AlignItems::Baseline => "baseline",
            AlignItems::BaselineLast => "last baseline",
            AlignItems::Stretch => "stretch",
        }
    }
}

impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
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

impl fmt::Display for AlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

/// Flex grow/shrink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexGrow {
    S0,
    S1,
}

impl FlexGrow {
    pub const fn value(&self) -> u8 {
        match self {
            FlexGrow::S0 => 0,
            FlexGrow::S1 => 1,
        }
    }
}

impl fmt::Display for FlexGrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
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

impl fmt::Display for Flex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value())
    }
}

/// Flex container utility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FlexContainer {
    pub(crate) direction: Option<FlexDirection>,
    pub(crate) wrap: Option<FlexWrap>,
    pub(crate) justify: Option<JustifyContent>,
    pub(crate) align: Option<AlignItems>,
    pub(crate) gap: Option<Spacing>,
    pub(crate) row_gap: Option<Spacing>,
    pub(crate) col_gap: Option<Spacing>,
}

impl FlexContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn direction_value(&self) -> Option<FlexDirection> {
        self.direction
    }

    pub const fn wrap_value(&self) -> Option<FlexWrap> {
        self.wrap
    }

    pub const fn justify_value(&self) -> Option<JustifyContent> {
        self.justify
    }

    pub const fn align_value(&self) -> Option<AlignItems> {
        self.align
    }

    pub const fn gap_value(&self) -> Option<Spacing> {
        self.gap
    }

    pub const fn row_gap_value(&self) -> Option<Spacing> {
        self.row_gap
    }

    pub const fn col_gap_value(&self) -> Option<Spacing> {
        self.col_gap
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

    /// Align items to start.
    pub fn items_start(self) -> Self {
        self.align(AlignItems::Start)
    }

    /// Align items to end.
    pub fn items_end(self) -> Self {
        self.align(AlignItems::End)
    }

    /// Align items to safe end.
    pub fn items_end_safe(self) -> Self {
        self.align(AlignItems::EndSafe)
    }

    /// Align items to center.
    pub fn items_center(self) -> Self {
        self.align(AlignItems::Center)
    }

    /// Align items to safe center.
    pub fn items_center_safe(self) -> Self {
        self.align(AlignItems::CenterSafe)
    }

    /// Align items to baseline.
    pub fn items_baseline(self) -> Self {
        self.align(AlignItems::Baseline)
    }

    /// Align items to last baseline.
    pub fn items_baseline_last(self) -> Self {
        self.align(AlignItems::BaselineLast)
    }

    /// Stretch items on cross axis.
    pub fn items_stretch(self) -> Self {
        self.align(AlignItems::Stretch)
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

    /// Set column gap (`gap-x-*` family).
    pub fn gap_x(self, gap: Spacing) -> Self {
        self.col_gap(gap)
    }

    /// Set row gap (`gap-y-*` family).
    pub fn gap_y(self, gap: Spacing) -> Self {
        self.row_gap(gap)
    }

    /// Row direction.
    pub fn row() -> Self {
        Self::new().direction(FlexDirection::Row)
    }

    /// Column direction.
    pub fn col() -> Self {
        Self::new().direction(FlexDirection::Col)
    }

    /// Row reverse direction.
    pub fn row_reverse() -> Self {
        Self::new().direction(FlexDirection::RowReverse)
    }

    /// Column reverse direction.
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
    pub const fn value(&self) -> &'static str {
        match self {
            ObjectFit::Contain => "contain",
            ObjectFit::Cover => "cover",
            ObjectFit::Fill => "fill",
            ObjectFit::None => "none",
            ObjectFit::ScaleDown => "scale-down",
        }
    }
}

impl fmt::Display for ObjectFit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

/// Grid template value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GridTemplate {
    /// Repeat `n` equal tracks.
    Count(NonZeroU16),
    /// No explicit template.
    None,
    /// Inherit track definition from parent grid.
    Subgrid,
    /// Use a CSS variable-like external value provider.
    CustomProperty(String),
    /// Use an explicit raw template value.
    Arbitrary(String),
}

impl GridTemplate {
    /// Create a repeated equal-track template.
    pub fn count(count: u16) -> Self {
        let count = count.max(1);
        Self::Count(NonZeroU16::new(count).expect("count is clamped to at least 1"))
    }

    /// Create a `none` template.
    pub fn none() -> Self {
        Self::None
    }

    /// Create a `subgrid` template.
    pub fn subgrid() -> Self {
        Self::Subgrid
    }

    /// Create a custom-property-backed template.
    pub fn custom_property(name: impl Into<String>) -> Self {
        Self::CustomProperty(name.into())
    }

    /// Create an arbitrary template value.
    pub fn arbitrary(value: impl Into<String>) -> Self {
        let normalized = value.into().replace('_', " ");
        Self::Arbitrary(normalized)
    }

    /// Return known track count when statically available.
    pub fn track_count_hint(&self) -> Option<u16> {
        match self {
            GridTemplate::Count(count) => Some(count.get()),
            GridTemplate::None
            | GridTemplate::Subgrid
            | GridTemplate::CustomProperty(_)
            | GridTemplate::Arbitrary(_) => None,
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
    pub(crate) columns: Option<GridTemplate>,
    pub(crate) rows: Option<GridTemplate>,
    pub(crate) gap: Option<Spacing>,
    pub(crate) row_gap: Option<Spacing>,
    pub(crate) col_gap: Option<Spacing>,
    pub(crate) justify: Option<JustifyContent>,
    pub(crate) align: Option<AlignItems>,
}

impl GridContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn columns_value(&self) -> Option<&GridTemplate> {
        self.columns.as_ref()
    }

    pub const fn rows_value(&self) -> Option<&GridTemplate> {
        self.rows.as_ref()
    }

    pub const fn gap_value(&self) -> Option<Spacing> {
        self.gap
    }

    pub const fn row_gap_value(&self) -> Option<Spacing> {
        self.row_gap
    }

    pub const fn col_gap_value(&self) -> Option<Spacing> {
        self.col_gap
    }

    pub const fn justify_value(&self) -> Option<JustifyContent> {
        self.justify
    }

    pub const fn align_value(&self) -> Option<AlignItems> {
        self.align
    }

    pub fn columns(mut self, cols: GridTemplate) -> Self {
        self.columns = Some(cols);
        self
    }

    /// Set repeated equal grid columns.
    pub fn cols_count(self, count: u16) -> Self {
        self.columns(GridTemplate::count(count))
    }

    /// Set grid columns to `none`.
    pub fn cols_none(self) -> Self {
        self.columns(GridTemplate::none())
    }

    /// Set grid columns to `subgrid`.
    pub fn cols_subgrid(self) -> Self {
        self.columns(GridTemplate::subgrid())
    }

    /// Set grid columns from a custom property.
    pub fn cols_custom_property(self, name: impl Into<String>) -> Self {
        self.columns(GridTemplate::custom_property(name))
    }

    /// Set grid columns from an arbitrary template value.
    pub fn cols_arbitrary(self, value: impl Into<String>) -> Self {
        self.columns(GridTemplate::arbitrary(value))
    }

    pub fn rows(mut self, rows: GridTemplate) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn gap(mut self, gap: Spacing) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Set row gap (`gap-y-*` family).
    pub fn row_gap(mut self, gap: Spacing) -> Self {
        self.row_gap = Some(gap);
        self
    }

    /// Set column gap (`gap-x-*` family).
    pub fn col_gap(mut self, gap: Spacing) -> Self {
        self.col_gap = Some(gap);
        self
    }

    /// Set column gap (`gap-x-*` family).
    pub fn gap_x(self, gap: Spacing) -> Self {
        self.col_gap(gap)
    }

    /// Set row gap (`gap-y-*` family).
    pub fn gap_y(self, gap: Spacing) -> Self {
        self.row_gap(gap)
    }

    /// Grid with 2 equal columns.
    pub fn cols_2() -> Self {
        Self::new().cols_count(2)
    }

    /// Grid with 3 equal columns.
    pub fn cols_3() -> Self {
        Self::new().cols_count(3)
    }

    /// Grid with 4 equal columns.
    pub fn cols_4() -> Self {
        Self::new().cols_count(4)
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
    pub const fn value(&self) -> &'static str {
        match self {
            Overflow::Auto => "auto",
            Overflow::Hidden => "hidden",
            Overflow::Clip => "clip",
            Overflow::Visible => "visible",
            Overflow::Scroll => "scroll",
        }
    }
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> Option<i16> {
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

impl fmt::Display for ZIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value() {
            Some(value) => write!(f, "{value}"),
            None => f.write_str("auto"),
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
    pub const fn value(&self) -> &'static str {
        match self {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Collapse => "collapse",
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
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

impl fmt::Display for PlaceContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
        match self {
            PlaceItems::Center => "center",
            PlaceItems::Start => "start",
            PlaceItems::End => "end",
            PlaceItems::Baseline => "baseline",
            PlaceItems::Stretch => "stretch",
        }
    }
}

impl fmt::Display for PlaceItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
        match self {
            JustifyItems::Normal => "normal",
            JustifyItems::Center => "center",
            JustifyItems::Start => "start",
            JustifyItems::End => "end",
            JustifyItems::Stretch => "stretch",
        }
    }
}

impl fmt::Display for JustifyItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    pub const fn value(&self) -> &'static str {
        match self {
            JustifySelf::Auto => "auto",
            JustifySelf::Start => "start",
            JustifySelf::End => "end",
            JustifySelf::Center => "center",
            JustifySelf::Stretch => "stretch",
        }
    }
}

impl fmt::Display for JustifySelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
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
    fn test_flex_direction_values() {
        assert_eq!(FlexDirection::Row.value(), "row");
        assert_eq!(FlexDirection::RowReverse.value(), "row-reverse");
        assert_eq!(FlexDirection::Col.value(), "column");
        assert_eq!(FlexDirection::ColReverse.value(), "column-reverse");
    }

    #[test]
    fn test_align_items_values_cover_flex_variants() {
        assert_eq!(AlignItems::Start.value(), "flex-start");
        assert_eq!(AlignItems::End.value(), "flex-end");
        assert_eq!(AlignItems::EndSafe.value(), "safe flex-end");
        assert_eq!(AlignItems::Center.value(), "center");
        assert_eq!(AlignItems::CenterSafe.value(), "safe center");
        assert_eq!(AlignItems::Baseline.value(), "baseline");
        assert_eq!(AlignItems::BaselineLast.value(), "last baseline");
        assert_eq!(AlignItems::Stretch.value(), "stretch");
    }

    #[test]
    fn test_justify_content_values() {
        assert_eq!(JustifyContent::Start.value(), "flex-start");
        assert_eq!(JustifyContent::End.value(), "flex-end");
        assert_eq!(JustifyContent::EndSafe.value(), "safe flex-end");
        assert_eq!(JustifyContent::Center.value(), "center");
        assert_eq!(JustifyContent::CenterSafe.value(), "safe center");
        assert_eq!(JustifyContent::Between.value(), "space-between");
        assert_eq!(JustifyContent::Around.value(), "space-around");
        assert_eq!(JustifyContent::Evenly.value(), "space-evenly");
        assert_eq!(JustifyContent::Stretch.value(), "stretch");
        assert_eq!(JustifyContent::Baseline.value(), "baseline");
        assert_eq!(JustifyContent::Normal.value(), "normal");
    }

    #[test]
    fn test_flex_container_align_items_helpers() {
        assert_eq!(
            FlexContainer::row().items_start().align,
            Some(AlignItems::Start)
        );
        assert_eq!(
            FlexContainer::row().items_end().align,
            Some(AlignItems::End)
        );
        assert_eq!(
            FlexContainer::row().items_end_safe().align,
            Some(AlignItems::EndSafe)
        );
        assert_eq!(
            FlexContainer::row().items_center().align,
            Some(AlignItems::Center)
        );
        assert_eq!(
            FlexContainer::row().items_center_safe().align,
            Some(AlignItems::CenterSafe)
        );
        assert_eq!(
            FlexContainer::row().items_baseline().align,
            Some(AlignItems::Baseline)
        );
        assert_eq!(
            FlexContainer::row().items_baseline_last().align,
            Some(AlignItems::BaselineLast)
        );
        assert_eq!(
            FlexContainer::row().items_stretch().align,
            Some(AlignItems::Stretch)
        );
    }

    #[test]
    fn test_grid_container_builder() {
        let grid = GridContainer::cols_3().gap(Spacing::S2);
        assert_eq!(grid.columns, Some(GridTemplate::count(3)));
        assert_eq!(grid.gap, Some(Spacing::S2));
    }

    #[test]
    fn test_grid_template_variants() {
        assert_eq!(GridTemplate::count(4).track_count_hint(), Some(4));
        assert_eq!(GridTemplate::count(0).track_count_hint(), Some(1));
        assert_eq!(GridTemplate::none().track_count_hint(), None);
        assert_eq!(GridTemplate::subgrid().track_count_hint(), None);
        assert_eq!(
            GridTemplate::custom_property("--layout-cols"),
            GridTemplate::CustomProperty("--layout-cols".to_string())
        );
        assert_eq!(
            GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px"),
            GridTemplate::Arbitrary("200px minmax(0, 1fr) 100px".to_string())
        );
    }

    #[test]
    fn test_grid_container_columns_helpers() {
        assert_eq!(
            GridContainer::new().cols_count(5).columns,
            Some(GridTemplate::count(5))
        );
        assert_eq!(
            GridContainer::new().cols_none().columns,
            Some(GridTemplate::none())
        );
        assert_eq!(
            GridContainer::new().cols_subgrid().columns,
            Some(GridTemplate::subgrid())
        );
        assert_eq!(
            GridContainer::new()
                .cols_custom_property("--layout-cols")
                .columns,
            Some(GridTemplate::custom_property("--layout-cols"))
        );
        assert_eq!(
            GridContainer::new()
                .cols_arbitrary("200px_minmax(0,_1fr)_100px")
                .columns,
            Some(GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px"))
        );
    }

    #[test]
    fn test_flex_container_gap_axis_helpers() {
        let flex = FlexContainer::row().gap_x(Spacing::S6).gap_y(Spacing::S3);
        assert_eq!(flex.col_gap, Some(Spacing::S6));
        assert_eq!(flex.row_gap, Some(Spacing::S3));
    }

    #[test]
    fn test_grid_container_gap_axis_helpers() {
        let grid = GridContainer::cols_2()
            .gap_x(Spacing::S8)
            .gap_y(Spacing::S4);
        assert_eq!(grid.col_gap, Some(Spacing::S8));
        assert_eq!(grid.row_gap, Some(Spacing::S4));
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
    fn test_flex_builders() {
        assert_eq!(Flex::number(1), Flex::Number(1));
        assert_eq!(Flex::fraction(1, 2).value(), "calc(1/2 * 100%)");
        assert_eq!(Flex::Auto, Flex::Auto);
        assert_eq!(Flex::Initial, Flex::Initial);
        assert_eq!(Flex::None, Flex::None);
        assert_eq!(
            Flex::custom_property("--my-flex"),
            Flex::CustomProperty("--my-flex".to_string())
        );
        assert_eq!(
            Flex::arbitrary("3_1_auto"),
            Flex::Arbitrary("3 1 auto".to_string())
        );
    }
}
