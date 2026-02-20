//! Style builder for composing styles fluently.

use std::num::NonZeroU8;

use crate::tokens::{
    AnimationToken, AspectRatio, Blur, BorderRadius, BorderStyle, BorderWidth, Color, Container,
    Cursor, DropShadow, Easing, FontFamily, FontSize, FontWeight, InsetShadow, LetterSpacing,
    LineHeight, MotionDefaults, OutlineStyle, Perspective, RingWidth, Shadow, Spacing, TextAlign,
    TextDecoration, TextShadow, TextTransform, TransitionDuration, TransitionProperty,
};
use crate::traits::Merge;
use crate::utilities::{
    Columns, Display, Flex, FlexContainer, FlexDirection, GridContainer, Height, JustifyItems,
    JustifySelf, Margin, ObjectFit, Overflow, Padding, PlaceContent, PlaceItems, Position, Size,
    SizeConstraints, Visibility, Width, ZIndex,
};

/// A comprehensive style builder for composing native UI styles.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    // Layout
    pub display: Option<Display>,
    pub visibility: Option<Visibility>,
    pub position: Option<Position>,
    pub z_index: Option<ZIndex>,
    pub overflow: Option<Overflow>,
    pub overflow_x: Option<Overflow>,
    pub overflow_y: Option<Overflow>,
    pub aspect_ratio: Option<AspectRatio>,
    pub object_fit: Option<ObjectFit>,
    pub columns: Option<Columns>,
    pub column_gap: Option<Spacing>,
    pub columns_max_count: Option<NonZeroU8>,

    // Flex/Grid
    pub flex: Option<FlexContainer>,
    pub flex_item: Option<Flex>,
    pub grid: Option<GridContainer>,
    pub place_content: Option<PlaceContent>,
    pub place_items: Option<PlaceItems>,
    pub justify_items: Option<JustifyItems>,
    pub justify_self: Option<JustifySelf>,

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

    // Effects
    pub blur: Option<Blur>,
    pub drop_shadow: Option<DropShadow>,
    pub perspective: Option<Perspective>,

    // Border
    pub border_radius: Option<BorderRadius>,
    pub border_width: Option<BorderWidth>,
    pub border_style: Option<BorderStyle>,
    pub border_color: Option<Color>,
    pub outline_width: Option<BorderWidth>,
    pub outline_style: Option<OutlineStyle>,
    pub outline_color: Option<Color>,
    pub ring_width: Option<RingWidth>,
    pub ring_color: Option<Color>,

    // Shadow
    pub box_shadow: Option<Shadow>,
    pub inset_shadow: Option<InsetShadow>,
    pub shadow_color: Option<Color>,

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
    pub text_shadow: Option<TextShadow>,

    // Motion (optional)
    pub transition_property: Option<String>,
    pub transition_duration: Option<TransitionDuration>,
    pub transition_timing_function: Option<Easing>,
    pub transition_delay: Option<TransitionDuration>,
    pub animation: Option<AnimationToken>,
    // Interactivity
    pub cursor: Option<Cursor>,
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

    /// Set visibility.
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
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

    /// Set aspect ratio.
    pub fn aspect_ratio(mut self, ratio: AspectRatio) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }

    /// Set object fit.
    pub fn object_fit(mut self, fit: ObjectFit) -> Self {
        self.object_fit = Some(fit);
        self
    }

    /// Set columns layout utility.
    pub fn columns(mut self, columns: Columns) -> Self {
        self.columns = Some(columns);
        self
    }

    /// Set columns by explicit count (`columns-<number>`).
    pub fn columns_count(self, count: u8) -> Self {
        self.columns(Columns::count(count))
    }

    /// Set columns by ideal width token (`columns-3xs` ... `columns-7xl`).
    pub fn columns_width(self, width: Container) -> Self {
        self.columns(Columns::width(width))
    }

    /// Set columns by custom ideal width in logical pixels.
    pub fn columns_width_px(self, width: f32) -> Self {
        self.columns(Columns::width_px(width))
    }

    /// Set `columns-auto`.
    pub fn columns_auto(self) -> Self {
        self.columns(Columns::auto())
    }

    /// Set column gap for multi-column layout.
    pub fn column_gap(mut self, gap: Spacing) -> Self {
        self.column_gap = Some(gap);
        self
    }

    /// Set maximum number of columns the backend may create for `columns-*`.
    pub fn columns_max_count(mut self, max_count: u8) -> Self {
        let max_count = max_count.max(1);
        self.columns_max_count =
            Some(NonZeroU8::new(max_count).expect("max_count is clamped to at least 1"));
        self
    }

    /// Set flex container properties.
    pub fn flex(mut self, flex: FlexContainer) -> Self {
        self.flex = Some(flex);
        self
    }

    /// Set flex direction utility class (`flex-row`, `flex-row-reverse`, `flex-col`, `flex-col-reverse`).
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        if let Some(ref mut flex) = self.flex {
            self.flex = Some(FlexContainer {
                direction: Some(direction),
                ..flex.clone()
            });
        } else {
            self.flex = Some(FlexContainer::new().direction(direction));
        }
        self
    }

    /// Set flex item shorthand (`flex-*`).
    pub fn flex_item(mut self, flex: Flex) -> Self {
        self.flex_item = Some(flex);
        self
    }

    /// Set `flex-<number>`.
    pub fn flex_number(self, number: u16) -> Self {
        self.flex_item(Flex::number(number))
    }

    /// Set `flex-<fraction>`.
    pub fn flex_fraction(self, numerator: u16, denominator: u16) -> Self {
        self.flex_item(Flex::fraction(numerator, denominator))
    }

    /// Set `flex-1`.
    pub fn flex_1(self) -> Self {
        self.flex_number(1)
    }

    /// Set `flex-auto`.
    pub fn flex_auto(self) -> Self {
        self.flex_item(Flex::Auto)
    }

    /// Set `flex-initial`.
    pub fn flex_initial(self) -> Self {
        self.flex_item(Flex::Initial)
    }

    /// Set `flex-none`.
    pub fn flex_none(self) -> Self {
        self.flex_item(Flex::None)
    }

    /// Set `flex-(<custom-property>)`.
    pub fn flex_custom_property(self, name: impl Into<String>) -> Self {
        self.flex_item(Flex::custom_property(name))
    }

    /// Set `flex-[<value>]`.
    pub fn flex_arbitrary(self, value: impl Into<String>) -> Self {
        self.flex_item(Flex::arbitrary(value))
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

    /// Set place content.
    pub fn place_content(mut self, content: PlaceContent) -> Self {
        self.place_content = Some(content);
        self
    }

    /// Set place items.
    pub fn place_items(mut self, items: PlaceItems) -> Self {
        self.place_items = Some(items);
        self
    }

    /// Set justify items.
    pub fn justify_items(mut self, items: JustifyItems) -> Self {
        self.justify_items = Some(items);
        self
    }

    /// Set justify self.
    pub fn justify_self(mut self, self_align: JustifySelf) -> Self {
        self.justify_self = Some(self_align);
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

    /// Set max-width to Tailwind prose measure (`65ch`).
    pub fn max_w_prose(mut self) -> Self {
        let constraints = self.constraints.unwrap_or_default().max_width(Size::Prose);
        self.constraints = Some(constraints);
        self
    }

    /// Set max-width.
    pub fn max_w(mut self, size: Size) -> Self {
        let constraints = self.constraints.unwrap_or_default().max_width(size);
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

    // === Effects ===

    /// Set blur.
    pub fn blur(mut self, blur: Blur) -> Self {
        self.blur = Some(blur);
        self
    }

    /// Set drop shadow (filter).
    pub fn drop_shadow(mut self, shadow: DropShadow) -> Self {
        self.drop_shadow = Some(shadow);
        self
    }

    /// Set perspective (3D transforms).
    pub fn perspective(mut self, perspective: Perspective) -> Self {
        self.perspective = Some(perspective);
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

    /// Set outline style.
    pub fn outline(mut self, width: BorderWidth, style: OutlineStyle, color: Color) -> Self {
        self.outline_width = Some(width);
        self.outline_style = Some(style);
        self.outline_color = Some(color);
        self
    }

    /// Set ring (focus-ring-like box shadow).
    pub fn ring(mut self, width: RingWidth, color: Color) -> Self {
        self.ring_width = Some(width);
        self.ring_color = Some(color);
        self
    }

    // === Shadow ===

    /// Set box shadow.
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.box_shadow = Some(shadow);
        self
    }

    /// Set inset shadow.
    pub fn inset_shadow(mut self, shadow: InsetShadow) -> Self {
        self.inset_shadow = Some(shadow);
        self
    }

    /// Set shadow color.
    pub fn shadow_color(mut self, color: Color) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// Set box shadow and color together.
    pub fn shadow_with_color(mut self, shadow: Shadow, color: Color) -> Self {
        self.box_shadow = Some(shadow);
        self.shadow_color = Some(color);
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

    /// Set text shadow.
    pub fn text_shadow(mut self, shadow: TextShadow) -> Self {
        self.text_shadow = Some(shadow);
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

    /// Set transition property using Tailwind token.
    pub fn transition(mut self, property: TransitionProperty) -> Self {
        self.transition_property = Some(property.value().to_string());
        self
    }

    /// Apply Tailwind default transition preset.
    pub fn transition_default(mut self) -> Self {
        let defaults = MotionDefaults::default();
        self.transition_property = Some(TransitionProperty::Default.value().to_string());
        self.transition_duration = Some(defaults.duration);
        self.transition_timing_function = Some(defaults.easing);
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

    // === Interactivity ===

    /// Set cursor.
    pub fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = Some(cursor);
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
}

impl Merge<Self> for Style {
    fn merge(&self, other: Self) -> Self {
        Self {
            display: other.display.or(self.display),
            visibility: other.visibility.or(self.visibility),
            position: other.position.or(self.position),
            z_index: other.z_index.or(self.z_index),
            overflow: other.overflow.or(self.overflow),
            overflow_x: other.overflow_x.or(self.overflow_x),
            overflow_y: other.overflow_y.or(self.overflow_y),
            aspect_ratio: other.aspect_ratio.or(self.aspect_ratio),
            object_fit: other.object_fit.or(self.object_fit),
            columns: other.columns.or(self.columns),
            column_gap: other.column_gap.or(self.column_gap),
            columns_max_count: other.columns_max_count.or(self.columns_max_count),
            flex: other.flex.or(self.flex.clone()),
            flex_item: other.flex_item.or(self.flex_item.clone()),
            grid: other.grid.or(self.grid.clone()),
            place_content: other.place_content.or(self.place_content),
            place_items: other.place_items.or(self.place_items),
            justify_items: other.justify_items.or(self.justify_items),
            justify_self: other.justify_self.or(self.justify_self),
            padding: other.padding.or(self.padding),
            margin: other.margin.or(self.margin),
            width: other.width.or(self.width),
            height: other.height.or(self.height),
            constraints: other.constraints.or(self.constraints),
            background_color: other.background_color.or(self.background_color),
            opacity: other.opacity.or(self.opacity),
            blur: other.blur.or(self.blur),
            drop_shadow: other.drop_shadow.or(self.drop_shadow),
            perspective: other.perspective.or(self.perspective),
            border_radius: other.border_radius.or(self.border_radius),
            border_width: other.border_width.or(self.border_width),
            border_style: other.border_style.or(self.border_style),
            border_color: other.border_color.or(self.border_color),
            outline_width: other.outline_width.or(self.outline_width),
            outline_style: other.outline_style.or(self.outline_style),
            outline_color: other.outline_color.or(self.outline_color),
            ring_width: other.ring_width.or(self.ring_width),
            ring_color: other.ring_color.or(self.ring_color),
            box_shadow: other.box_shadow.or(self.box_shadow),
            inset_shadow: other.inset_shadow.or(self.inset_shadow),
            shadow_color: other.shadow_color.or(self.shadow_color),
            font_family: other.font_family.or(self.font_family),
            font_size: other.font_size.or(self.font_size),
            font_weight: other.font_weight.or(self.font_weight),
            letter_spacing: other.letter_spacing.or(self.letter_spacing),
            line_height: other.line_height.or(self.line_height),
            text_align: other.text_align.or(self.text_align),
            text_decoration: other.text_decoration.or(self.text_decoration),
            text_transform: other.text_transform.or(self.text_transform),
            text_color: other.text_color.or(self.text_color),
            text_shadow: other.text_shadow.or(self.text_shadow),
            transition_property: other
                .transition_property
                .or(self.transition_property.clone()),
            transition_duration: other.transition_duration.or(self.transition_duration),
            transition_timing_function: other
                .transition_timing_function
                .or(self.transition_timing_function),
            transition_delay: other.transition_delay.or(self.transition_delay),
            animation: other.animation.or(self.animation),
            cursor: other.cursor.or(self.cursor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{AnimationToken, Easing, Scale, TransitionDuration};

    #[test]
    fn test_style_builder_fields() {
        let style = Style::new()
            .padding(Padding::all(Spacing::S4))
            .bg(Color::blue(Scale::S500))
            .rounded(BorderRadius::Md)
            .text_color(Color::slate(Scale::S50));

        assert_eq!(style.padding, Some(Padding::all(Spacing::S4)));
        assert_eq!(style.background_color, Some(Color::blue(Scale::S500)));
        assert_eq!(style.border_radius, Some(BorderRadius::Md));
    }

    #[test]
    fn test_flex_center_fields() {
        let style = Style::centered_col().gap(Spacing::S2);
        assert_eq!(style.display, Some(Display::Flex));
        assert!(style.flex.is_some());
    }

    #[test]
    fn test_flex_reverse_direction_helpers() {
        let row_reverse = Style::flex_row_reverse();
        assert_eq!(row_reverse.display, Some(Display::Flex));
        assert_eq!(
            row_reverse.flex.and_then(|flex| flex.direction),
            Some(FlexDirection::RowReverse)
        );

        let col_reverse = Style::flex_col_reverse();
        assert_eq!(col_reverse.display, Some(Display::Flex));
        assert_eq!(
            col_reverse.flex.and_then(|flex| flex.direction),
            Some(FlexDirection::ColReverse)
        );
    }

    #[test]
    fn test_flex_item_builder_fields() {
        let style = Style::new().flex_1();
        assert_eq!(style.flex_item, Some(Flex::number(1)));

        let style = Style::new().flex_number(2);
        assert_eq!(style.flex_item, Some(Flex::number(2)));

        let style = Style::new().flex_fraction(1, 2);
        assert_eq!(style.flex_item, Some(Flex::fraction(1, 2)));

        let style = Style::new().flex_auto();
        assert_eq!(style.flex_item, Some(Flex::Auto));

        let style = Style::new().flex_initial();
        assert_eq!(style.flex_item, Some(Flex::Initial));

        let style = Style::new().flex_none();
        assert_eq!(style.flex_item, Some(Flex::None));

        let style = Style::new().flex_custom_property("--my-flex");
        assert_eq!(style.flex_item, Some(Flex::custom_property("--my-flex")));

        let style = Style::new().flex_arbitrary("3_1_auto");
        assert_eq!(style.flex_item, Some(Flex::arbitrary("3_1_auto")));
    }

    #[test]
    fn test_motion_tokens() {
        let style = Style::new()
            .transition(TransitionProperty::Opacity)
            .transition_duration(TransitionDuration::Ms300)
            .transition_ease(Easing::InOut)
            .animate(AnimationToken::Pulse);
        assert_eq!(style.transition_property, Some("opacity".to_string()));
        assert_eq!(style.transition_duration, Some(TransitionDuration::Ms300));
        assert_eq!(style.transition_timing_function, Some(Easing::InOut));
        assert_eq!(style.animation, Some(AnimationToken::Pulse));
    }

    #[test]
    fn test_shadow_color_merge() {
        let base = Style::new().shadow_with_color(Shadow::Sm, Color::blue(Scale::S500));
        let override_style = Style::new().shadow_color(Color::red(Scale::S500));
        let merged = base.merge(override_style);
        assert_eq!(merged.box_shadow, Some(Shadow::Sm));
        assert_eq!(merged.shadow_color, Some(Color::red(Scale::S500)));
    }

    #[test]
    fn test_columns_builder_fields() {
        let style = Style::new()
            .columns_count(3)
            .column_gap(Spacing::S4)
            .columns_width(Container::S3xs)
            .columns_width_px(280.0)
            .columns_max_count(4);

        assert_eq!(style.columns, Some(Columns::width_px(280.0)));
        assert_eq!(style.column_gap, Some(Spacing::S4));
        assert_eq!(style.columns_max_count.map(NonZeroU8::get), Some(4));
    }
}
