use super::Style;
use crate::tokens::{
    BackgroundColor, BorderColor, BorderRadius, BorderStyle, BorderWidth, Cursor, Easing,
    RingColor, RingWidth, SemanticColor, Shadow, Spacing, TextColor, TransitionDuration,
    TransitionProperty,
};
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

    /// Structural surface preset with spacing, radius, and elevation.
    pub fn surface() -> Self {
        Self::new()
            .rounded(BorderRadius::Lg)
            .shadow(Shadow::Sm)
            .padding(crate::utilities::Padding::all(Spacing::S4))
    }

    /// Theme-aware card surface using semantic card, foreground, and border tokens.
    pub fn card() -> Self {
        Self::surface()
            .background_token(BackgroundColor::semantic(SemanticColor::Card))
            .text_color_token(TextColor::semantic(SemanticColor::CardForeground))
            .border_width(BorderWidth::S1)
            .border_style(BorderStyle::Solid)
            .border_color_token(BorderColor::semantic(SemanticColor::Border))
    }

    /// Reusable interactive affordances without hardcoding a concrete palette.
    pub fn interactive() -> Self {
        Self::new()
            .cursor(Cursor::Pointer)
            .transition(TransitionProperty::Default)
            .transition_duration(TransitionDuration::Ms150)
            .transition_ease(Easing::InOut)
            .focus_visible(|style| {
                style.ring_token(RingWidth::S2, RingColor::semantic(SemanticColor::Ring))
            })
            .disabled(|style| style.opacity(0.55))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::Padding;

    #[test]
    fn test_surface_preset_keeps_structural_defaults() {
        let style = Style::surface();

        assert_eq!(style.border_radius_value(), Some(BorderRadius::Lg));
        assert_eq!(style.box_shadow_value(), Some(Shadow::Sm));
        assert_eq!(style.padding_value(), Some(&Padding::all(Spacing::S4)));
        assert_eq!(style.background_color_value(), None);
    }

    #[test]
    fn test_card_preset_uses_semantic_tokens() {
        let style = Style::card();

        assert_eq!(
            style.background_color,
            Some(BackgroundColor::semantic(SemanticColor::Card))
        );
        assert_eq!(
            style.text_color_token_value(),
            Some(TextColor::semantic(SemanticColor::CardForeground))
        );
        assert_eq!(style.border_width_value(), Some(BorderWidth::S1));
        assert_eq!(style.border_style_value(), Some(BorderStyle::Solid));
        assert_eq!(
            style.border_color_token_value(),
            Some(BorderColor::semantic(SemanticColor::Border))
        );
    }

    #[test]
    fn test_interactive_preset_adds_generic_affordances() {
        let style = Style::interactive();

        assert_eq!(style.cursor_value(), Some(Cursor::Pointer));
        assert_eq!(
            style.transition_property_value(),
            Some(TransitionProperty::Default.value())
        );
        assert_eq!(
            style.transition_duration_value(),
            Some(TransitionDuration::Ms150)
        );
        assert_eq!(
            style.transition_timing_function_value(),
            Some(Easing::InOut)
        );
        assert_eq!(
            style
                .focus_visible_style()
                .and_then(Style::ring_width_value),
            Some(RingWidth::S2)
        );
        assert_eq!(
            style
                .focus_visible_style()
                .and_then(Style::ring_color_token_value),
            Some(RingColor::semantic(SemanticColor::Ring))
        );
        assert_eq!(
            style.disabled_style().and_then(Style::opacity_value),
            Some(0.55)
        );
    }
}
