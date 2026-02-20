//! Button component with variants.

use crate::style::Style;
use crate::tokens::{BorderRadius, Color, FontSize, FontWeight, Scale, Shadow, Spacing};
use crate::traits::{Merge, ToCss};

/// Button variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

/// Button size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    Icon,
}

/// Button component.
#[derive(Debug, Clone)]
pub struct Button {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub full_width: bool,
}

impl Button {
    pub fn new(variant: ButtonVariant, size: ButtonSize) -> Self {
        Self {
            variant,
            size,
            disabled: false,
            full_width: false,
        }
    }

    pub fn primary() -> Self {
        Self::new(ButtonVariant::Primary, ButtonSize::Md)
    }
    pub fn secondary() -> Self {
        Self::new(ButtonVariant::Secondary, ButtonSize::Md)
    }
    pub fn outline() -> Self {
        Self::new(ButtonVariant::Outline, ButtonSize::Md)
    }
    pub fn ghost() -> Self {
        Self::new(ButtonVariant::Ghost, ButtonSize::Md)
    }
    pub fn destructive() -> Self {
        Self::new(ButtonVariant::Destructive, ButtonSize::Md)
    }
    pub fn link() -> Self {
        Self::new(ButtonVariant::Link, ButtonSize::Md)
    }

    pub fn sm(mut self) -> Self {
        self.size = ButtonSize::Sm;
        self
    }
    pub fn md(mut self) -> Self {
        self.size = ButtonSize::Md;
        self
    }
    pub fn lg(mut self) -> Self {
        self.size = ButtonSize::Lg;
        self
    }
    pub fn icon(mut self) -> Self {
        self.size = ButtonSize::Icon;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
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

impl ToCss for Button {
    fn to_css(&self) -> String {
        self.style().to_css()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_button() {
        let btn = Button::primary();
        let css = btn.to_css();
        assert!(css.contains("background-color: #3b82f6"));
    }

    #[test]
    fn test_button_sizes() {
        let sm = Button::primary().sm().to_css();
        let lg = Button::primary().lg().to_css();
        assert!(sm.contains("padding: 0.25rem 0.75rem"));
        assert!(lg.contains("padding: 0.75rem 1.5rem"));
    }
}
