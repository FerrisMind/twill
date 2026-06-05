use std::collections::BTreeMap;

use super::{StateStyles, Style};
use crate::tokens::{
    BackgroundColor, BorderColor, ColorValueToken, OutlineColor, RingColor, SemanticThemeSource,
    ShadowColorToken, TextColor, ThemeVariant,
};

fn resolve_background_token<S: SemanticThemeSource + ?Sized>(
    token: BackgroundColor,
    theme: &S,
    variant: ThemeVariant,
) -> BackgroundColor {
    match token {
        BackgroundColor::Semantic(color) => theme
            .resolve_value(color, variant)
            .map(ColorValueToken::from)
            .map(BackgroundColor::arbitrary)
            .unwrap_or(token),
        _ => token,
    }
}

fn resolve_text_token<S: SemanticThemeSource + ?Sized>(
    token: TextColor,
    theme: &S,
    variant: ThemeVariant,
) -> TextColor {
    match token {
        TextColor::Semantic(color) => theme
            .resolve_value(color, variant)
            .map(ColorValueToken::from)
            .map(TextColor::arbitrary)
            .unwrap_or(token),
        _ => token,
    }
}

fn resolve_border_token<S: SemanticThemeSource + ?Sized>(
    token: BorderColor,
    theme: &S,
    variant: ThemeVariant,
) -> BorderColor {
    match token {
        BorderColor::Semantic(color) => theme
            .resolve_value(color, variant)
            .map(ColorValueToken::from)
            .map(BorderColor::arbitrary)
            .unwrap_or(token),
        _ => token,
    }
}

fn resolve_outline_token<S: SemanticThemeSource + ?Sized>(
    token: OutlineColor,
    theme: &S,
    variant: ThemeVariant,
) -> OutlineColor {
    match token {
        OutlineColor::Semantic(color) => theme
            .resolve_value(color, variant)
            .map(ColorValueToken::from)
            .map(OutlineColor::arbitrary)
            .unwrap_or(token),
        _ => token,
    }
}

fn resolve_ring_token<S: SemanticThemeSource + ?Sized>(
    token: RingColor,
    theme: &S,
    variant: ThemeVariant,
) -> RingColor {
    match token {
        RingColor::Semantic(color) => theme
            .resolve_value(color, variant)
            .map(ColorValueToken::from)
            .map(RingColor::arbitrary)
            .unwrap_or(token),
        _ => token,
    }
}

fn resolve_shadow_token<S: SemanticThemeSource + ?Sized>(
    token: ShadowColorToken,
    theme: &S,
    variant: ThemeVariant,
) -> ShadowColorToken {
    match token {
        ShadowColorToken::Semantic(color) => theme
            .resolve_value(color, variant)
            .map(ColorValueToken::from)
            .map(ShadowColorToken::arbitrary)
            .unwrap_or(token),
        _ => token,
    }
}

fn resolve_state_styles<S: SemanticThemeSource + ?Sized>(
    states: &StateStyles,
    theme: &S,
    variant: ThemeVariant,
) -> StateStyles {
    StateStyles {
        hover: states
            .hover
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        focus: states
            .focus
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        focus_visible: states
            .focus_visible
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        active: states
            .active
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        disabled: states
            .disabled
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        selected: states
            .selected
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        checked: states
            .checked
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        open: states
            .open
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        closed: states
            .closed
            .as_ref()
            .map(|style| style.resolved_theme(theme, variant)),
        data: states
            .data
            .iter()
            .map(|(selector, style)| (selector.clone(), style.resolved_theme(theme, variant)))
            .collect::<BTreeMap<_, _>>(),
        aria: states
            .aria
            .iter()
            .map(|(selector, style)| (selector.clone(), style.resolved_theme(theme, variant)))
            .collect::<BTreeMap<_, _>>(),
    }
}

impl Style {
    /// Resolve semantic color aliases into concrete tokens using a theme source.
    ///
    /// Non-semantic tokens are preserved as-is. Semantic tokens resolve into
    /// typed arbitrary color values so palette-independent theme data stays exact.
    pub fn resolved_theme<S>(&self, theme: &S, variant: ThemeVariant) -> Self
    where
        S: SemanticThemeSource + ?Sized,
    {
        let mut resolved = self.clone();

        resolved.background_color = resolved
            .background_color
            .map(|token| resolve_background_token(token, theme, variant));
        resolved.text_color = resolved
            .text_color
            .map(|token| resolve_text_token(token, theme, variant));
        resolved.border_color = resolved
            .border_color
            .map(|token| resolve_border_token(token, theme, variant));
        resolved.outline_color = resolved
            .outline_color
            .map(|token| resolve_outline_token(token, theme, variant));
        resolved.ring_color = resolved
            .ring_color
            .map(|token| resolve_ring_token(token, theme, variant));
        resolved.shadow_color = resolved
            .shadow_color
            .map(|token| resolve_shadow_token(token, theme, variant));

        resolved.states = resolved
            .states
            .as_ref()
            .map(|states| Box::new(resolve_state_styles(states.as_ref(), theme, variant)));
        resolved.responsive = resolved.responsive.as_ref().map(|layers| {
            layers
                .iter()
                .map(|(breakpoint, style)| (*breakpoint, style.resolved_theme(theme, variant)))
                .collect::<BTreeMap<_, _>>()
        });

        resolved
    }

    /// Resolve semantic color aliases against a light variant of the supplied theme source.
    pub fn resolved_light_theme<S>(&self, theme: &S) -> Self
    where
        S: SemanticThemeSource + ?Sized,
    {
        self.resolved_theme(theme, ThemeVariant::Light)
    }

    /// Resolve semantic color aliases against a dark variant of the supplied theme source.
    pub fn resolved_dark_theme<S>(&self, theme: &S) -> Self
    where
        S: SemanticThemeSource + ?Sized,
    {
        self.resolved_theme(theme, ThemeVariant::Dark)
    }

    /// Resolve semantic color aliases in place using the supplied theme source.
    pub fn resolve_theme_in_place<S>(&mut self, theme: &S, variant: ThemeVariant) -> &mut Self
    where
        S: SemanticThemeSource + ?Sized,
    {
        *self = self.resolved_theme(theme, variant);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{DataState, Style};
    use crate::tokens::{
        BackgroundColor, BorderColor, Color, ColorValue, RingColor, SemanticColor, TextColor,
    };
    use crate::traits::ComputeValue;
    use crate::utilities::Padding;

    struct PartialTheme;

    impl SemanticThemeSource for PartialTheme {
        fn resolve_value(
            &self,
            semantic: SemanticColor,
            variant: ThemeVariant,
        ) -> Option<ColorValue> {
            match (semantic, variant) {
                (SemanticColor::Primary, ThemeVariant::Dark) => {
                    Some(Color::blue(crate::tokens::Scale::S500).compute())
                }
                (SemanticColor::Ring, ThemeVariant::Dark) => {
                    Some(Color::amber(crate::tokens::Scale::S400).compute())
                }
                _ => None,
            }
        }
    }

    #[test]
    fn test_resolved_theme_replaces_semantic_tokens_recursively() {
        let style = Style::card()
            .bg(Color::slate(crate::tokens::Scale::S50))
            .text_color_token(TextColor::semantic(SemanticColor::Primary))
            .ring_token(
                crate::tokens::RingWidth::S2,
                RingColor::semantic(SemanticColor::Ring),
            )
            .hover(|style| style.border_color_token(BorderColor::semantic(SemanticColor::Primary)))
            .data_attr(DataState::Open, |style| {
                style.background_token(BackgroundColor::semantic(SemanticColor::Primary))
            })
            .at_md(|style| style.text_color_token(TextColor::semantic(SemanticColor::Primary)));

        let resolved = style.resolved_dark_theme(&PartialTheme);
        let expected_primary =
            ColorValueToken::from(Color::blue(crate::tokens::Scale::S500).compute());
        let expected_ring =
            ColorValueToken::from(Color::amber(crate::tokens::Scale::S400).compute());

        assert_eq!(
            resolved.text_color_token_value(),
            Some(TextColor::arbitrary(expected_primary))
        );
        assert_eq!(
            resolved.ring_color_token_value(),
            Some(RingColor::arbitrary(expected_ring))
        );
        assert_eq!(
            resolved
                .hover_style()
                .and_then(Style::border_color_token_value),
            Some(BorderColor::arbitrary(expected_primary))
        );
        assert_eq!(
            resolved
                .data_attr_style(DataState::Open)
                .and_then(Style::background_color_value),
            Some(BackgroundColor::arbitrary(expected_primary))
        );
        assert_eq!(
            resolved
                .breakpoint_style(crate::tokens::Breakpoint::Md)
                .and_then(Style::text_color_token_value),
            Some(TextColor::arbitrary(expected_primary))
        );
    }

    #[test]
    fn test_resolved_theme_preserves_unresolved_tokens() {
        let style = Style::new()
            .padding(Padding::all(crate::tokens::Spacing::S4))
            .background_token(BackgroundColor::semantic(SemanticColor::Background))
            .text_color_token(TextColor::semantic(SemanticColor::Foreground));

        let resolved = style.resolved_dark_theme(&PartialTheme);

        assert_eq!(
            resolved.background_color_value(),
            Some(BackgroundColor::semantic(SemanticColor::Background))
        );
        assert_eq!(
            resolved.text_color_token_value(),
            Some(TextColor::semantic(SemanticColor::Foreground))
        );
        assert_eq!(resolved.padding_value(), style.padding_value());
    }

    #[test]
    fn test_resolve_theme_in_place_mutates_style() {
        let mut style = Style::new().text_color_token(TextColor::semantic(SemanticColor::Primary));
        let expected = ColorValueToken::from(Color::blue(crate::tokens::Scale::S500).compute());

        style.resolve_theme_in_place(&PartialTheme, ThemeVariant::Dark);

        assert_eq!(
            style.text_color_token_value(),
            Some(TextColor::arbitrary(expected))
        );
    }
}
