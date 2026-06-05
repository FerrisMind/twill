use std::collections::BTreeMap;

use super::Style;
use crate::tokens::Breakpoint;
use crate::traits::{IntoStyle, Merge};
use crate::utilities::{
    Display, FlexContainer, GridContainer, Height, Margin, Padding, SizeConstraints, Width,
};

fn merge_flex_container(
    base: Option<FlexContainer>,
    override_value: Option<FlexContainer>,
) -> Option<FlexContainer> {
    match (base, override_value) {
        (Some(base), Some(override_value)) => Some(FlexContainer {
            direction: override_value.direction.or(base.direction),
            wrap: override_value.wrap.or(base.wrap),
            justify: override_value.justify.or(base.justify),
            align: override_value.align.or(base.align),
            gap: override_value.gap.or(base.gap),
            row_gap: override_value.row_gap.or(base.row_gap),
            col_gap: override_value.col_gap.or(base.col_gap),
        }),
        (None, Some(override_value)) => Some(override_value),
        (Some(base), None) => Some(base),
        (None, None) => None,
    }
}

fn merge_grid_container(
    base: Option<GridContainer>,
    override_value: Option<GridContainer>,
) -> Option<GridContainer> {
    match (base, override_value) {
        (Some(base), Some(override_value)) => Some(GridContainer {
            columns: override_value.columns.or(base.columns),
            rows: override_value.rows.or(base.rows),
            gap: override_value.gap.or(base.gap),
            row_gap: override_value.row_gap.or(base.row_gap),
            col_gap: override_value.col_gap.or(base.col_gap),
            justify: override_value.justify.or(base.justify),
            align: override_value.align.or(base.align),
        }),
        (None, Some(override_value)) => Some(override_value),
        (Some(base), None) => Some(base),
        (None, None) => None,
    }
}

fn merge_padding(base: Option<Padding>, override_value: Option<Padding>) -> Option<Padding> {
    match (base, override_value) {
        (Some(base), Some(override_value)) => Some(Padding {
            top: override_value.top.or(base.top),
            right: override_value.right.or(base.right),
            bottom: override_value.bottom.or(base.bottom),
            left: override_value.left.or(base.left),
        }),
        (None, Some(override_value)) => Some(override_value),
        (Some(base), None) => Some(base),
        (None, None) => None,
    }
}

fn merge_margin(base: Option<Margin>, override_value: Option<Margin>) -> Option<Margin> {
    match (base, override_value) {
        (Some(base), Some(override_value)) => Some(Margin {
            top: override_value.top.or(base.top),
            right: override_value.right.or(base.right),
            bottom: override_value.bottom.or(base.bottom),
            left: override_value.left.or(base.left),
        }),
        (None, Some(override_value)) => Some(override_value),
        (Some(base), None) => Some(base),
        (None, None) => None,
    }
}

fn merge_constraints(
    base: Option<SizeConstraints>,
    override_value: Option<SizeConstraints>,
) -> Option<SizeConstraints> {
    match (base, override_value) {
        (Some(base), Some(override_value)) => Some(SizeConstraints {
            min_width: override_value.min_width.or(base.min_width),
            max_width: override_value.max_width.or(base.max_width),
            min_height: override_value.min_height.or(base.min_height),
            max_height: override_value.max_height.or(base.max_height),
        }),
        (None, Some(override_value)) => Some(override_value),
        (Some(base), None) => Some(base),
        (None, None) => None,
    }
}

fn merge_responsive_layers(
    current: &Option<BTreeMap<Breakpoint, Style>>,
    incoming: Option<BTreeMap<Breakpoint, Style>>,
) -> Option<BTreeMap<Breakpoint, Style>> {
    match (current, incoming) {
        (Some(current), Some(incoming)) => {
            let mut merged = current.clone();

            for (breakpoint, style) in incoming {
                if let Some(existing) = merged.get(&breakpoint).cloned() {
                    merged.insert(breakpoint, existing.merge(style));
                } else {
                    merged.insert(breakpoint, style);
                }
            }

            Some(merged)
        }
        (None, Some(incoming)) => Some(incoming),
        (Some(current), None) => Some(current.clone()),
        (None, None) => None,
    }
}

impl Style {
    /// Merge any style-like value, consuming `self` and returning the merged style.
    pub fn merged<T>(self, other: T) -> Self
    where
        T: IntoStyle,
    {
        let other = other.into_style();
        self.merge_style_ref(&other)
    }

    /// Merge another style-like value into `self` in place.
    pub fn merge_in_place<T>(&mut self, other: T) -> &mut Self
    where
        T: IntoStyle,
    {
        *self = self.clone().merged(other);
        self
    }

    pub(crate) fn merge_style_ref(&self, other: &Self) -> Self {
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
            flex: merge_flex_container(self.flex.clone(), other.flex.clone()),
            flex_item: other.flex_item.clone().or(self.flex_item.clone()),
            grid: merge_grid_container(self.grid.clone(), other.grid.clone()),
            place_content: other.place_content.or(self.place_content),
            place_items: other.place_items.or(self.place_items),
            justify_items: other.justify_items.or(self.justify_items),
            justify_self: other.justify_self.or(self.justify_self),
            padding: merge_padding(self.padding, other.padding),
            margin: merge_margin(self.margin, other.margin),
            width: other.width.or(self.width),
            height: other.height.or(self.height),
            constraints: merge_constraints(self.constraints, other.constraints),
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
                .clone()
                .or(self.transition_property.clone()),
            transition_duration: other.transition_duration.or(self.transition_duration),
            transition_timing_function: other
                .transition_timing_function
                .or(self.transition_timing_function),
            transition_delay: other.transition_delay.or(self.transition_delay),
            animation: other.animation.or(self.animation),
            cursor: other.cursor.or(self.cursor),
            states: match (&self.states, &other.states) {
                (Some(current), Some(incoming)) => {
                    Some(Box::new(current.as_ref().merge_ref(incoming.as_ref())))
                }
                (None, Some(incoming)) => Some(incoming.clone()),
                (Some(current), None) => Some(current.clone()),
                (None, None) => None,
            },
            responsive: merge_responsive_layers(&self.responsive, other.responsive.clone()),
        }
    }
}

impl<T> Merge<T> for Style
where
    T: IntoStyle,
{
    fn merge(&self, other: T) -> Self {
        self.clone().merged(other)
    }
}

impl crate::traits::Responsive for Style {
    type Breakpoint = Breakpoint;

    fn at_breakpoint(&self, breakpoint: Self::Breakpoint) -> Self {
        Self::at_breakpoint(self, breakpoint)
    }
}

impl From<Padding> for Style {
    fn from(value: Padding) -> Self {
        Self::new().padding(value)
    }
}

impl From<Margin> for Style {
    fn from(value: Margin) -> Self {
        Self::new().margin(value)
    }
}

impl From<Width> for Style {
    fn from(value: Width) -> Self {
        Self::new().width(value)
    }
}

impl From<Height> for Style {
    fn from(value: Height) -> Self {
        Self::new().height(value)
    }
}

impl From<FlexContainer> for Style {
    fn from(value: FlexContainer) -> Self {
        Self::new().display(Display::Flex).flex(value)
    }
}

impl From<GridContainer> for Style {
    fn from(value: GridContainer) -> Self {
        Self::new().display(Display::Grid).grid(value)
    }
}

impl<T> Extend<T> for Style
where
    T: IntoStyle,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.merge_in_place(item);
        }
    }
}

impl<T> std::iter::FromIterator<T> for Style
where
    T: IntoStyle,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut style = Self::new();
        style.extend(iter);
        style
    }
}
