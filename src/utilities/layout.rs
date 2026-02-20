//! Layout utilities for flexbox, grid, display, and positioning.

use crate::tokens::Spacing;
use crate::traits::ToCss;

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

impl ToCss for Display {
    fn to_css(&self) -> String {
        match self {
            Display::Block => "block".to_string(),
            Display::InlineBlock => "inline-block".to_string(),
            Display::Inline => "inline".to_string(),
            Display::Flex => "flex".to_string(),
            Display::InlineFlex => "inline-flex".to_string(),
            Display::Grid => "grid".to_string(),
            Display::InlineGrid => "inline-grid".to_string(),
            Display::Hidden => "none".to_string(),
            Display::Contents => "contents".to_string(),
            Display::FlowRoot => "flow-root".to_string(),
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

impl ToCss for Position {
    fn to_css(&self) -> String {
        match self {
            Position::Static => "static".to_string(),
            Position::Relative => "relative".to_string(),
            Position::Absolute => "absolute".to_string(),
            Position::Fixed => "fixed".to_string(),
            Position::Sticky => "sticky".to_string(),
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

impl ToCss for FlexDirection {
    fn to_css(&self) -> String {
        match self {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::RowReverse => "row-reverse".to_string(),
            FlexDirection::Col => "column".to_string(),
            FlexDirection::ColReverse => "column-reverse".to_string(),
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

impl ToCss for FlexWrap {
    fn to_css(&self) -> String {
        match self {
            FlexWrap::Wrap => "wrap".to_string(),
            FlexWrap::WrapReverse => "wrap-reverse".to_string(),
            FlexWrap::NoWrap => "nowrap".to_string(),
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

impl ToCss for JustifyContent {
    fn to_css(&self) -> String {
        match self {
            JustifyContent::Start => "flex-start".to_string(),
            JustifyContent::End => "flex-end".to_string(),
            JustifyContent::Center => "center".to_string(),
            JustifyContent::Between => "space-between".to_string(),
            JustifyContent::Around => "space-around".to_string(),
            JustifyContent::Evenly => "space-evenly".to_string(),
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

impl ToCss for AlignItems {
    fn to_css(&self) -> String {
        match self {
            AlignItems::Start => "flex-start".to_string(),
            AlignItems::End => "flex-end".to_string(),
            AlignItems::Center => "center".to_string(),
            AlignItems::Baseline => "baseline".to_string(),
            AlignItems::Stretch => "stretch".to_string(),
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

impl ToCss for AlignSelf {
    fn to_css(&self) -> String {
        match self {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::Start => "flex-start".to_string(),
            AlignSelf::End => "flex-end".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
        }
    }
}

/// Flex grow/shrink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexGrow {
    S0,
    S1,
}

impl ToCss for FlexGrow {
    fn to_css(&self) -> String {
        match self {
            FlexGrow::S0 => "0".to_string(),
            FlexGrow::S1 => "1".to_string(),
        }
    }
}

/// Flex shorthand utility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flex {
    /// flex: 1 1 0%
    S1,
    /// flex: 1 1 auto
    Auto,
    /// flex: 0 1 auto
    Initial,
    /// flex: none
    None,
}

impl ToCss for Flex {
    fn to_css(&self) -> String {
        match self {
            Flex::S1 => "1 1 0%".to_string(),
            Flex::Auto => "1 1 auto".to_string(),
            Flex::Initial => "0 1 auto".to_string(),
            Flex::None => "none".to_string(),
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

impl ToCss for FlexContainer {
    fn to_css(&self) -> String {
        let mut props = Vec::new();

        if let Some(d) = &self.direction {
            props.push(format!("flex-direction: {}", d.to_css()));
        }
        if let Some(w) = &self.wrap {
            props.push(format!("flex-wrap: {}", w.to_css()));
        }
        if let Some(j) = &self.justify {
            props.push(format!("justify-content: {}", j.to_css()));
        }
        if let Some(a) = &self.align {
            props.push(format!("align-items: {}", a.to_css()));
        }
        if let Some(g) = &self.gap {
            props.push(format!("gap: {}", g.to_css()));
        }
        if let Some(g) = &self.row_gap {
            props.push(format!("row-gap: {}", g.to_css()));
        }
        if let Some(g) = &self.col_gap {
            props.push(format!("column-gap: {}", g.to_css()));
        }

        props.join("; ")
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

/// Grid template columns/rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridTemplate {
    /// grid-template-columns: 1
    Cols1,
    /// grid-template-columns: 2
    Cols2,
    /// grid-template-columns: 3
    Cols3,
    /// grid-template-columns: 4
    Cols4,
    /// grid-template-columns: 5
    Cols5,
    /// grid-template-columns: 6
    Cols6,
    /// grid-template-columns: 12
    Cols12,
    /// grid-template-columns: none
    None,
}

impl ToCss for GridTemplate {
    fn to_css(&self) -> String {
        match self {
            GridTemplate::Cols1 => "repeat(1, minmax(0, 1fr))".to_string(),
            GridTemplate::Cols2 => "repeat(2, minmax(0, 1fr))".to_string(),
            GridTemplate::Cols3 => "repeat(3, minmax(0, 1fr))".to_string(),
            GridTemplate::Cols4 => "repeat(4, minmax(0, 1fr))".to_string(),
            GridTemplate::Cols5 => "repeat(5, minmax(0, 1fr))".to_string(),
            GridTemplate::Cols6 => "repeat(6, minmax(0, 1fr))".to_string(),
            GridTemplate::Cols12 => "repeat(12, minmax(0, 1fr))".to_string(),
            GridTemplate::None => "none".to_string(),
        }
    }
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

impl ToCss for GridContainer {
    fn to_css(&self) -> String {
        let mut props = Vec::new();

        if let Some(c) = &self.columns {
            props.push(format!("grid-template-columns: {}", c.to_css()));
        }
        if let Some(r) = &self.rows {
            props.push(format!("grid-template-rows: {}", r.to_css()));
        }
        if let Some(g) = &self.gap {
            props.push(format!("gap: {}", g.to_css()));
        }
        if let Some(g) = &self.row_gap {
            props.push(format!("row-gap: {}", g.to_css()));
        }
        if let Some(g) = &self.col_gap {
            props.push(format!("column-gap: {}", g.to_css()));
        }
        if let Some(j) = &self.justify {
            props.push(format!("justify-content: {}", j.to_css()));
        }
        if let Some(a) = &self.align {
            props.push(format!("align-items: {}", a.to_css()));
        }

        props.join("; ")
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

impl ToCss for Overflow {
    fn to_css(&self) -> String {
        match self {
            Overflow::Auto => "auto".to_string(),
            Overflow::Hidden => "hidden".to_string(),
            Overflow::Clip => "clip".to_string(),
            Overflow::Visible => "visible".to_string(),
            Overflow::Scroll => "scroll".to_string(),
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

impl ToCss for ZIndex {
    fn to_css(&self) -> String {
        match self {
            ZIndex::Auto => "auto".to_string(),
            ZIndex::S0 => "0".to_string(),
            ZIndex::S10 => "10".to_string(),
            ZIndex::S20 => "20".to_string(),
            ZIndex::S30 => "30".to_string(),
            ZIndex::S40 => "40".to_string(),
            ZIndex::S50 => "50".to_string(),
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

impl ToCss for Visibility {
    fn to_css(&self) -> String {
        match self {
            Visibility::Visible => "visible".to_string(),
            Visibility::Hidden => "hidden".to_string(),
            Visibility::Collapse => "collapse".to_string(),
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

impl ToCss for PlaceContent {
    fn to_css(&self) -> String {
        match self {
            PlaceContent::Center => "center".to_string(),
            PlaceContent::Start => "start".to_string(),
            PlaceContent::End => "end".to_string(),
            PlaceContent::Between => "space-between".to_string(),
            PlaceContent::Around => "space-around".to_string(),
            PlaceContent::Evenly => "space-evenly".to_string(),
            PlaceContent::Baseline => "baseline".to_string(),
            PlaceContent::Stretch => "stretch".to_string(),
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

impl ToCss for PlaceItems {
    fn to_css(&self) -> String {
        match self {
            PlaceItems::Center => "center".to_string(),
            PlaceItems::Start => "start".to_string(),
            PlaceItems::End => "end".to_string(),
            PlaceItems::Baseline => "baseline".to_string(),
            PlaceItems::Stretch => "stretch".to_string(),
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

impl ToCss for JustifyItems {
    fn to_css(&self) -> String {
        match self {
            JustifyItems::Normal => "normal".to_string(),
            JustifyItems::Center => "center".to_string(),
            JustifyItems::Start => "start".to_string(),
            JustifyItems::End => "end".to_string(),
            JustifyItems::Stretch => "stretch".to_string(),
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

impl ToCss for JustifySelf {
    fn to_css(&self) -> String {
        match self {
            JustifySelf::Auto => "auto".to_string(),
            JustifySelf::Start => "start".to_string(),
            JustifySelf::End => "end".to_string(),
            JustifySelf::Center => "center".to_string(),
            JustifySelf::Stretch => "stretch".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_container() {
        let flex = FlexContainer::centered_row().gap(Spacing::S4);
        let css = flex.to_css();
        assert!(css.contains("flex-direction: row"));
        assert!(css.contains("justify-content: center"));
        assert!(css.contains("gap: 1rem"));
    }

    #[test]
    fn test_grid_container() {
        let grid = GridContainer::cols_3().gap(Spacing::S2);
        let css = grid.to_css();
        assert!(css.contains("grid-template-columns: repeat(3"));
    }
}
