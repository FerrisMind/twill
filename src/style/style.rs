//! Style builder for composing styles fluently.

use crate::tokens::{
    AnimationToken, BorderRadius, BorderStyle, BorderWidth, Color, Easing, FontFamily, FontSize,
    FontWeight, LetterSpacing, LineHeight, Shadow, Spacing, TextAlign, TextDecoration,
    TextTransform, TransitionDuration,
};
use crate::traits::{Merge, ToCss};
use crate::utilities::{
    Display, FlexContainer, GridContainer, Height, Margin, Overflow, Padding, Position,
    SizeConstraints, Width, ZIndex,
};

/// A comprehensive style builder for composing CSS styles.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    // Layout
    pub display: Option<Display>,
    pub position: Option<Position>,
    pub z_index: Option<ZIndex>,
    pub overflow: Option<Overflow>,
    pub overflow_x: Option<Overflow>,
    pub overflow_y: Option<Overflow>,

    // Flex/Grid
    pub flex: Option<FlexContainer>,
    pub grid: Option<GridContainer>,

    // Spacing
    pub padding: Option<Padding>,
    pub margin: Option<Margin>,

    // Size
    pub width: Option<Width>,
    pub height: Option<Height>,
    pub constraints: Option<SizeConstraints>,

    // Background
    pub background_color: Option<Color>,
    pub opacity: Option<f32>,

    // Border
    pub border_radius: Option<BorderRadius>,
    pub border_width: Option<BorderWidth>,
    pub border_style: Option<BorderStyle>,
    pub border_color: Option<Color>,

    // Shadow
    pub box_shadow: Option<Shadow>,

    // Typography
    pub font_family: Option<FontFamily>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
    pub letter_spacing: Option<LetterSpacing>,
    pub line_height: Option<LineHeight>,
    pub text_align: Option<TextAlign>,
    pub text_decoration: Option<TextDecoration>,
    pub text_transform: Option<TextTransform>,
    pub text_color: Option<Color>,

    // Motion (optional)
    pub transition_property: Option<String>,
    pub transition_duration: Option<TransitionDuration>,
    pub transition_timing_function: Option<Easing>,
    pub transition_delay: Option<TransitionDuration>,
    pub animation: Option<AnimationToken>,
}

impl Style {
    /// Create a new empty style.
    pub fn new() -> Self {
        Self::default()
    }

    // === Layout ===

    /// Set display type.
    pub fn display(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }

    /// Set position type.
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Set z-index.
    pub fn z_index(mut self, z: ZIndex) -> Self {
        self.z_index = Some(z);
        self
    }

    /// Set overflow.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Set flex container properties.
    pub fn flex(mut self, flex: FlexContainer) -> Self {
        self.flex = Some(flex);
        self
    }

    /// Set grid container properties.
    pub fn grid(mut self, grid: GridContainer) -> Self {
        self.grid = Some(grid);
        self
    }

    /// Set gap (for flex/grid).
    pub fn gap(mut self, spacing: Spacing) -> Self {
        if let Some(ref mut flex) = self.flex {
            self.flex = Some(FlexContainer {
                gap: Some(spacing),
                ..flex.clone()
            });
        } else {
            self.flex = Some(FlexContainer::new().gap(spacing));
        }
        self
    }

    // === Spacing ===

    /// Set padding.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Set margin.
    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = Some(margin);
        self
    }

    // === Size ===

    /// Set width.
    pub fn width(mut self, width: Width) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height.
    pub fn height(mut self, height: Height) -> Self {
        self.height = Some(height);
        self
    }

    /// Set size constraints.
    pub fn constraints(mut self, constraints: SizeConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }

    // === Background ===

    /// Set background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set background color (alias for bg).
    pub fn background(self, color: Color) -> Self {
        self.bg(color)
    }

    /// Set opacity (0.0 - 1.0).
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    // === Border ===

    /// Set border radius.
    pub fn rounded(mut self, radius: BorderRadius) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Set border.
    pub fn border(mut self, width: BorderWidth, style: BorderStyle, color: Color) -> Self {
        self.border_width = Some(width);
        self.border_style = Some(style);
        self.border_color = Some(color);
        self
    }

    // === Shadow ===

    /// Set box shadow.
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.box_shadow = Some(shadow);
        self
    }

    // === Typography ===

    /// Set font family.
    pub fn font(mut self, family: FontFamily) -> Self {
        self.font_family = Some(family);
        self
    }

    /// Set font size.
    pub fn text_size(mut self, size: FontSize) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set font weight.
    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    /// Set letter spacing.
    pub fn tracking(mut self, spacing: LetterSpacing) -> Self {
        self.letter_spacing = Some(spacing);
        self
    }

    /// Set line height.
    pub fn leading(mut self, height: LineHeight) -> Self {
        self.line_height = Some(height);
        self
    }

    /// Set text alignment.
    pub fn text_align(mut self, align: TextAlign) -> Self {
        self.text_align = Some(align);
        self
    }

    /// Set text decoration.
    pub fn underline(mut self) -> Self {
        self.text_decoration = Some(TextDecoration::Underline);
        self
    }

    /// Set text transform.
    pub fn uppercase(mut self) -> Self {
        self.text_transform = Some(TextTransform::Uppercase);
        self
    }

    /// Set text color.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Set text color (alias).
    pub fn text(self, color: Color) -> Self {
        self.text_color(color)
    }

    // === Motion ===

    /// Set transition property (e.g. "all", "opacity", "transform").
    pub fn transition_property(mut self, property: impl Into<String>) -> Self {
        self.transition_property = Some(property.into());
        self
    }

    /// Set transition duration using Tailwind-aligned duration tokens.
    pub fn transition_duration(mut self, duration: TransitionDuration) -> Self {
        self.transition_duration = Some(duration);
        self
    }

    /// Set transition timing function using Tailwind-aligned easing tokens.
    pub fn transition_ease(mut self, easing: Easing) -> Self {
        self.transition_timing_function = Some(easing);
        self
    }

    /// Set transition delay.
    pub fn transition_delay(mut self, delay: TransitionDuration) -> Self {
        self.transition_delay = Some(delay);
        self
    }

    /// Set animation token.
    pub fn animate(mut self, animation: AnimationToken) -> Self {
        self.animation = Some(animation);
        self
    }

    // === Convenience methods ===

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
}

impl Merge<Self> for Style {
    fn merge(&self, other: Self) -> Self {
        Self {
            display: other.display.or(self.display),
            position: other.position.or(self.position),
            z_index: other.z_index.or(self.z_index),
            overflow: other.overflow.or(self.overflow),
            overflow_x: other.overflow_x.or(self.overflow_x),
            overflow_y: other.overflow_y.or(self.overflow_y),
            flex: other.flex.or(self.flex.clone()),
            grid: other.grid.or(self.grid.clone()),
            padding: other.padding.or(self.padding),
            margin: other.margin.or(self.margin),
            width: other.width.or(self.width),
            height: other.height.or(self.height),
            constraints: other.constraints.or(self.constraints),
            background_color: other.background_color.or(self.background_color),
            opacity: other.opacity.or(self.opacity),
            border_radius: other.border_radius.or(self.border_radius),
            border_width: other.border_width.or(self.border_width),
            border_style: other.border_style.or(self.border_style),
            border_color: other.border_color.or(self.border_color),
            box_shadow: other.box_shadow.or(self.box_shadow),
            font_family: other.font_family.or(self.font_family),
            font_size: other.font_size.or(self.font_size),
            font_weight: other.font_weight.or(self.font_weight),
            letter_spacing: other.letter_spacing.or(self.letter_spacing),
            line_height: other.line_height.or(self.line_height),
            text_align: other.text_align.or(self.text_align),
            text_decoration: other.text_decoration.or(self.text_decoration),
            text_transform: other.text_transform.or(self.text_transform),
            text_color: other.text_color.or(self.text_color),
            transition_property: other
                .transition_property
                .or(self.transition_property.clone()),
            transition_duration: other.transition_duration.or(self.transition_duration),
            transition_timing_function: other
                .transition_timing_function
                .or(self.transition_timing_function),
            transition_delay: other.transition_delay.or(self.transition_delay),
            animation: other.animation.or(self.animation),
        }
    }
}

impl ToCss for Style {
    fn to_css(&self) -> String {
        let mut props = Vec::new();

        // Layout
        if let Some(v) = &self.display {
            props.push(format!("display: {}", v.to_css()));
        }
        if let Some(v) = &self.position {
            props.push(format!("position: {}", v.to_css()));
        }
        if let Some(v) = &self.z_index {
            props.push(format!("z-index: {}", v.to_css()));
        }
        if let Some(v) = &self.overflow {
            props.push(format!("overflow: {}", v.to_css()));
        }
        if let Some(v) = &self.overflow_x {
            props.push(format!("overflow-x: {}", v.to_css()));
        }
        if let Some(v) = &self.overflow_y {
            props.push(format!("overflow-y: {}", v.to_css()));
        }

        // Flex/Grid
        if let Some(v) = &self.flex {
            props.push(v.to_css());
        }
        if let Some(v) = &self.grid {
            props.push(v.to_css());
        }

        // Spacing
        if let Some(v) = &self.padding {
            props.push(v.to_css());
        }
        if let Some(v) = &self.margin {
            props.push(v.to_css());
        }

        // Size
        if let Some(v) = &self.width {
            props.push(v.to_css());
        }
        if let Some(v) = &self.height {
            props.push(v.to_css());
        }
        if let Some(v) = &self.constraints {
            props.push(v.to_css());
        }

        // Background
        if let Some(v) = &self.background_color {
            props.push(format!("background-color: {}", v.to_css()));
        }
        if let Some(v) = &self.opacity {
            props.push(format!("opacity: {}", v));
        }

        // Border
        if let Some(v) = &self.border_radius {
            props.push(format!("border-radius: {}", v.to_css()));
        }
        if let Some(v) = &self.border_width {
            props.push(format!("border-width: {}", v.to_css()));
        }
        if let Some(v) = &self.border_style {
            props.push(format!("border-style: {}", v.to_css()));
        }
        if let Some(v) = &self.border_color {
            props.push(format!("border-color: {}", v.to_css()));
        }

        // Shadow
        if let Some(v) = &self.box_shadow {
            props.push(format!("box-shadow: {}", v.to_css()));
        }

        // Typography
        if let Some(v) = &self.font_family {
            props.push(format!("font-family: {}", v.to_css()));
        }
        if let Some(v) = &self.font_size {
            props.push(format!("font-size: {}", v.to_css()));
        }
        if let Some(v) = &self.font_weight {
            props.push(format!("font-weight: {}", v.to_css()));
        }
        if let Some(v) = &self.letter_spacing {
            props.push(format!("letter-spacing: {}", v.to_css()));
        }
        if let Some(v) = &self.line_height {
            props.push(format!("line-height: {}", v.to_css()));
        }
        if let Some(v) = &self.text_align {
            props.push(format!("text-align: {}", v.to_css()));
        }
        if let Some(v) = &self.text_decoration {
            props.push(format!("text-decoration: {}", v.to_css()));
        }
        if let Some(v) = &self.text_transform {
            props.push(format!("text-transform: {}", v.to_css()));
        }
        if let Some(v) = &self.text_color {
            props.push(format!("color: {}", v.to_css()));
        }

        // Motion
        if let Some(v) = &self.transition_property {
            props.push(format!("transition-property: {}", v));
        }
        if let Some(v) = &self.transition_duration {
            props.push(format!("transition-duration: {}", v.to_css()));
        }
        if let Some(v) = &self.transition_timing_function {
            props.push(format!("transition-timing-function: {}", v.to_css()));
        }
        if let Some(v) = &self.transition_delay {
            props.push(format!("transition-delay: {}", v.to_css()));
        }
        if let Some(v) = &self.animation {
            props.push(format!("animation: {}", v.to_css()));
        }

        props.join("; ")
    }
}

impl Style {
    /// Generate inline style string for HTML.
    pub fn to_inline_style(&self) -> String {
        format!("style=\"{}\"", self.to_css())
    }

    /// Generate CSS class content.
    pub fn to_class_content(&self) -> String {
        format!("{{ {} }}", self.to_css())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{AnimationToken, Easing, Scale, TransitionDuration};

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .padding(Padding::all(Spacing::S4))
            .bg(Color::blue(Scale::S500))
            .rounded(BorderRadius::Md)
            .text_color(Color::slate(Scale::S50));

        let css = style.to_css();
        assert!(css.contains("padding: 1rem"));
        assert!(css.contains("background-color: #3b82f6"));
        assert!(css.contains("border-radius: 0.375rem"));
    }

    #[test]
    fn test_flex_center() {
        let style = Style::centered_col().gap(Spacing::S2);
        let css = style.to_css();
        assert!(css.contains("flex-direction: column"));
        assert!(css.contains("justify-content: center"));
    }

    #[test]
    fn test_motion_tokens() {
        let style = Style::new()
            .transition_property("opacity")
            .transition_duration(TransitionDuration::Ms300)
            .transition_ease(Easing::InOut)
            .animate(AnimationToken::Pulse);
        let css = style.to_css();
        assert!(css.contains("transition-property: opacity"));
        assert!(css.contains("transition-duration: 300ms"));
        assert!(css.contains("transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)"));
        assert!(css.contains("animation: pulse 2s"));
    }
}
