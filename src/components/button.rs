//! Button component with variants.

use std::fmt;

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, FontSize, FontWeight, Scale, Shadow, Spacing};
use crate::traits::Merge;

/// Button variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

impl ButtonVariant {
    /// All built-in button variants in a stable order.
    pub const ALL: [Self; 6] = [
        Self::Primary,
        Self::Secondary,
        Self::Outline,
        Self::Ghost,
        Self::Destructive,
        Self::Link,
    ];

    /// Returns the stable API-facing name of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Destructive => "destructive",
            Self::Link => "link",
        }
    }
}

impl AsRef<str> for ButtonVariant {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Button size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Icon,
}

impl ButtonSize {
    /// All built-in button sizes in a stable order.
    pub const ALL: [Self; 4] = [Self::Sm, Self::Md, Self::Lg, Self::Icon];

    /// Returns the stable API-facing name of the size.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Icon => "icon",
        }
    }
}

impl AsRef<str> for ButtonSize {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Button component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub full_width: bool,
}

impl Button {
    /// Creates a new button descriptor with an explicit variant and size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twill::{Button, ButtonSize, ButtonVariant};
    ///
    /// let button = Button::new(ButtonVariant::Outline, ButtonSize::Lg);
    ///
    /// assert_eq!(button.variant(), ButtonVariant::Outline);
    /// assert_eq!(button.size(), ButtonSize::Lg);
    /// ```
    pub fn new(variant: ButtonVariant, size: ButtonSize) -> Self {
        Self {
            variant,
            size,
            disabled: false,
            full_width: false,
        }
    }

    /// Creates a primary medium button.
    pub fn primary() -> Self {
        Self::new(ButtonVariant::Primary, ButtonSize::Md)
    }

    /// Creates a secondary medium button.
    pub fn secondary() -> Self {
        Self::new(ButtonVariant::Secondary, ButtonSize::Md)
    }

    /// Creates an outline medium button.
    pub fn outline() -> Self {
        Self::new(ButtonVariant::Outline, ButtonSize::Md)
    }

    /// Creates a ghost medium button.
    pub fn ghost() -> Self {
        Self::new(ButtonVariant::Ghost, ButtonSize::Md)
    }

    /// Creates a destructive medium button.
    pub fn destructive() -> Self {
        Self::new(ButtonVariant::Destructive, ButtonSize::Md)
    }

    /// Creates a link-styled medium button.
    pub fn link() -> Self {
        Self::new(ButtonVariant::Link, ButtonSize::Md)
    }

    /// Returns the current variant.
    pub const fn variant(&self) -> ButtonVariant {
        self.variant
    }

    /// Returns the current size.
    pub const fn size(&self) -> ButtonSize {
        self.size
    }

    /// Returns whether the button is disabled.
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns whether the button should stretch to the full available width.
    pub const fn is_full_width(&self) -> bool {
        self.full_width
    }

    /// Returns a copy with a different variant.
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Returns a copy with a different size.
    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Returns a copy with the disabled state explicitly set.
    pub fn disabled_if(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Returns a copy with the full-width state explicitly set.
    pub fn full_width_if(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// Mark the button as small.
    pub fn sm(mut self) -> Self {
        self.size = ButtonSize::Sm;
        self
    }

    /// Mark the button as medium.
    pub fn md(mut self) -> Self {
        self.size = ButtonSize::Md;
        self
    }

    /// Mark the button as large.
    pub fn lg(mut self) -> Self {
        self.size = ButtonSize::Lg;
        self
    }

    /// Mark the button as icon-sized.
    pub fn icon(mut self) -> Self {
        self.size = ButtonSize::Icon;
        self
    }

    /// Marks the button as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Marks the button as enabled.
    pub fn enabled(mut self) -> Self {
        self.disabled = false;
        self
    }

    /// Marks the button as full width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Marks the button as auto-width again.
    pub fn auto_width(mut self) -> Self {
        self.full_width = false;
        self
    }

    /// Get the computed style for this button.
    pub fn style(&self) -> Style {
        let base = self.base_style();
        let variant_style = self.variant_style();
        let size_style = self.size_style();

        let mut style = base.merge(variant_style).merge(size_style);

        if self.full_width {
            style = style.width(crate::utilities::Width::full());
        }

        if self.disabled {
            style.opacity = Some(0.5);
        }

        style
    }

    fn base_style(&self) -> Style {
        Style::new()
            .display(crate::utilities::Display::InlineFlex)
            .flex(
                crate::utilities::FlexContainer::row()
                    .justify(crate::utilities::JustifyContent::Center)
                    .align(crate::utilities::AlignItems::Center),
            )
            .font_weight(FontWeight::Medium)
            .rounded(BorderRadius::Md)
            .text_align(crate::tokens::TextAlign::Center)
    }

    fn variant_style(&self) -> Style {
        match self.variant {
            ButtonVariant::Primary => Style::new()
                .bg(Color::blue(Scale::S500))
                .text_color(Color::slate(Scale::S50))
                .shadow(Shadow::Sm),
            ButtonVariant::Secondary => Style::new()
                .bg(Color::gray(Scale::S100))
                .text_color(Color::gray(Scale::S900)),
            ButtonVariant::Outline => Style::new().text_color(Color::gray(Scale::S900)).border(
                crate::tokens::BorderWidth::S1,
                crate::tokens::BorderStyle::Solid,
                Color::gray(Scale::S200),
            ),
            ButtonVariant::Ghost => Style::new().text_color(Color::gray(Scale::S900)),
            ButtonVariant::Destructive => Style::new()
                .bg(Color::red(Scale::S500))
                .text_color(Color::slate(Scale::S50))
                .shadow(Shadow::Sm),
            ButtonVariant::Link => Style::new()
                .text_color(Color::blue(Scale::S500))
                .underline(),
        }
    }

    fn size_style(&self) -> Style {
        match self.size {
            ButtonSize::Sm => Style::new()
                .padding(crate::utilities::Padding::symmetric(
                    Spacing::S1,
                    Spacing::S3,
                ))
                .text_size(FontSize::Sm)
                .rounded(BorderRadius::Sm),
            ButtonSize::Md => Style::new()
                .padding(crate::utilities::Padding::symmetric(
                    Spacing::S2,
                    Spacing::S4,
                ))
                .text_size(FontSize::Sm),
            ButtonSize::Lg => Style::new()
                .padding(crate::utilities::Padding::symmetric(
                    Spacing::S3,
                    Spacing::S6,
                ))
                .text_size(FontSize::Base)
                .rounded(BorderRadius::Lg),
            ButtonSize::Icon => Style::new().padding(crate::utilities::Padding::all(Spacing::S2)),
        }
    }
}

impl From<(ButtonVariant, ButtonSize)> for Button {
    fn from((variant, size): (ButtonVariant, ButtonSize)) -> Self {
        Self::new(variant, size)
    }
}

impl From<Button> for Style {
    fn from(value: Button) -> Self {
        value.style()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_button() {
        let btn = Button::primary();
        let style = btn.style();
        assert_eq!(
            style.background_color,
            Some(crate::tokens::BackgroundColor::palette(Color::blue(
                Scale::S500
            )))
        );
    }

    #[test]
    fn test_button_sizes() {
        let sm = Button::primary().sm().style();
        let lg = Button::primary().lg().style();
        assert_ne!(sm.padding, lg.padding);
    }

    #[test]
    fn test_button_defaults_match_primary_medium() {
        let default_button = Button::default();
        assert_eq!(default_button, Button::primary());
        assert_eq!(ButtonVariant::default(), ButtonVariant::Primary);
        assert_eq!(ButtonSize::default(), ButtonSize::Md);
    }

    #[test]
    fn test_button_getters_and_explicit_state_helpers() {
        let button = Button::secondary()
            .with_size(ButtonSize::Lg)
            .disabled_if(true)
            .full_width_if(true);

        assert_eq!(button.variant(), ButtonVariant::Secondary);
        assert_eq!(button.size(), ButtonSize::Lg);
        assert!(button.is_disabled());
        assert!(button.is_full_width());
    }

    #[test]
    fn test_button_toggle_helpers_can_reset_state() {
        let button = Button::primary()
            .disabled()
            .enabled()
            .full_width()
            .auto_width();
        assert!(!button.is_disabled());
        assert!(!button.is_full_width());
    }

    #[test]
    fn test_button_standard_conversions() {
        let icon: Button = (ButtonVariant::Ghost, ButtonSize::Icon).into();
        assert_eq!(icon, Button::ghost().icon());

        let style = Style::from(Button::destructive());
        assert_eq!(
            style.background_color,
            Some(crate::tokens::BackgroundColor::palette(Color::red(
                Scale::S500
            )))
        );
    }

    #[test]
    fn test_button_variant_and_size_labels() {
        assert_eq!(ButtonVariant::Primary.as_str(), "primary");
        assert_eq!(ButtonVariant::Link.to_string(), "link");
        assert_eq!(ButtonSize::Lg.as_ref(), "lg");
        assert_eq!(
            ButtonVariant::ALL,
            [
                ButtonVariant::Primary,
                ButtonVariant::Secondary,
                ButtonVariant::Outline,
                ButtonVariant::Ghost,
                ButtonVariant::Destructive,
                ButtonVariant::Link,
            ]
        );
        assert_eq!(
            ButtonSize::ALL,
            [
                ButtonSize::Sm,
                ButtonSize::Md,
                ButtonSize::Lg,
                ButtonSize::Icon,
            ]
        );
    }
}
