use super::Style;
use crate::tokens::{Shadow, Spacing};
use crate::utilities::{Display, FlexContainer, Height, Overflow, Width};

impl Style {
    /// Create a flex container with default settings.
    pub fn flex_row() -> Self {
        Self::new()
            .display(Display::Flex)
            .flex(FlexContainer::row())
    }

    /// Create a flex column with default settings.
    pub fn flex_col() -> Self {
        Self::new()
            .display(Display::Flex)
            .flex(FlexContainer::col())
    }

    /// Create a flex row reverse with default settings.
    pub fn flex_row_reverse() -> Self {
        Self::new()
            .display(Display::Flex)
            .flex(FlexContainer::row_reverse())
    }

    /// Create a flex column reverse with default settings.
    pub fn flex_col_reverse() -> Self {
        Self::new()
            .display(Display::Flex)
            .flex(FlexContainer::col_reverse())
    }

    /// Create a centered flex row.
    pub fn centered_row() -> Self {
        Self::new()
            .display(Display::Flex)
            .flex(FlexContainer::centered_row())
    }

    /// Create a centered flex column.
    pub fn centered_col() -> Self {
        Self::new()
            .display(Display::Flex)
            .flex(FlexContainer::centered_col())
    }

    /// Hide element.
    pub fn hidden() -> Self {
        Self::new().display(Display::Hidden)
    }

    /// Hidden overflow.
    pub fn overflow_hidden(mut self) -> Self {
        self.overflow = Some(Overflow::Hidden);
        self
    }

    /// Auto overflow.
    pub fn overflow_auto(mut self) -> Self {
        self.overflow = Some(Overflow::Auto);
        self
    }

    /// Full width.
    pub fn w_full() -> Self {
        Self::new().width(Width::full())
    }

    /// Full height.
    pub fn h_full() -> Self {
        Self::new().height(Height::full())
    }

    /// Full screen.
    pub fn screen() -> Self {
        Self::new().width(Width::screen()).height(Height::screen())
    }

    /// Card-like surface.
    pub fn surface() -> Self {
        Self::new()
            .rounded(crate::tokens::BorderRadius::Lg)
            .shadow(Shadow::Sm)
            .padding(crate::utilities::Padding::all(Spacing::S4))
    }
}
