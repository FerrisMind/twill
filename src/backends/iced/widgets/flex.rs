use crate::style::Style;
use crate::tokens::Spacing;
use crate::utilities::{
    AlignItems, Flex, FlexDirection, JustifyContent, MarginValue, PaddingValue,
};
use iced::Length;

use super::common::spacing_to_px;
use super::layout::apply_layout_with_custom_properties;

pub(crate) fn is_reverse_direction(direction: FlexDirection) -> bool {
    matches!(
        direction,
        FlexDirection::RowReverse | FlexDirection::ColReverse
    )
}

pub(crate) fn row_alignment_for_items(items: AlignItems) -> iced::alignment::Vertical {
    match items {
        AlignItems::Start | AlignItems::Stretch => iced::alignment::Vertical::Top,
        AlignItems::End | AlignItems::EndSafe => iced::alignment::Vertical::Bottom,
        AlignItems::Center | AlignItems::CenterSafe => iced::alignment::Vertical::Center,
        AlignItems::Baseline | AlignItems::BaselineLast => iced::alignment::Vertical::Bottom,
    }
}

pub(crate) fn column_alignment_for_items(items: AlignItems) -> iced::alignment::Horizontal {
    match items {
        AlignItems::Start
        | AlignItems::Stretch
        | AlignItems::Baseline
        | AlignItems::BaselineLast => iced::alignment::Horizontal::Left,
        AlignItems::End | AlignItems::EndSafe => iced::alignment::Horizontal::Right,
        AlignItems::Center | AlignItems::CenterSafe => iced::alignment::Horizontal::Center,
    }
}

pub(crate) fn normalize_justify_content(justify: JustifyContent) -> JustifyContent {
    match justify {
        JustifyContent::EndSafe => JustifyContent::End,
        JustifyContent::CenterSafe => JustifyContent::Center,
        _ => justify,
    }
}

fn main_axis_spacer<'a, Message: Clone + 'a>(
    direction: FlexDirection,
    portion: u16,
) -> iced::Element<'a, Message> {
    let portion = portion.max(1);
    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => iced::widget::Space::new()
            .width(Length::FillPortion(portion))
            .height(Length::Shrink)
            .into(),
        FlexDirection::Col | FlexDirection::ColReverse => iced::widget::Space::new()
            .width(Length::Shrink)
            .height(Length::FillPortion(portion))
            .into(),
    }
}

fn distribute_items_for_justify<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    justify_content: Option<JustifyContent>,
) -> (Vec<iced::Element<'a, Message>>, bool) {
    let justify = justify_content
        .map(normalize_justify_content)
        .unwrap_or(JustifyContent::Start);
    let start_at_end = is_reverse_direction(direction);
    let item_count = items.len();

    let mut distributed = Vec::new();
    let mut needs_main_axis_fill = false;

    match justify {
        JustifyContent::Start | JustifyContent::Normal | JustifyContent::Baseline => {
            if start_at_end && item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
            distributed.extend(items);
        }
        JustifyContent::End => {
            if !start_at_end && item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
            distributed.extend(items);
        }
        JustifyContent::Center => {
            if item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
            distributed.extend(items);
            if item_count > 0 {
                distributed.push(main_axis_spacer(direction, 1));
                needs_main_axis_fill = true;
            }
        }
        JustifyContent::Between => {
            if item_count <= 1 {
                if start_at_end && item_count > 0 {
                    distributed.push(main_axis_spacer(direction, 1));
                    needs_main_axis_fill = true;
                }
                distributed.extend(items);
            } else {
                for (index, item) in items.into_iter().enumerate() {
                    if index > 0 {
                        distributed.push(main_axis_spacer(direction, 1));
                        needs_main_axis_fill = true;
                    }
                    distributed.push(item);
                }
            }
        }
        JustifyContent::Around => {
            if item_count == 0 {
                return (distributed, false);
            }
            distributed.push(main_axis_spacer(direction, 1));
            needs_main_axis_fill = true;

            for (index, item) in items.into_iter().enumerate() {
                distributed.push(item);
                if index + 1 < item_count {
                    distributed.push(main_axis_spacer(direction, 2));
                }
            }

            distributed.push(main_axis_spacer(direction, 1));
        }
        JustifyContent::Evenly => {
            if item_count == 0 {
                return (distributed, false);
            }
            distributed.push(main_axis_spacer(direction, 1));
            needs_main_axis_fill = true;

            for (index, item) in items.into_iter().enumerate() {
                distributed.push(item);
                if index + 1 < item_count {
                    distributed.push(main_axis_spacer(direction, 1));
                }
            }

            distributed.push(main_axis_spacer(direction, 1));
        }
        JustifyContent::Stretch => {
            distributed.extend(items);
            needs_main_axis_fill = true;
        }
        JustifyContent::EndSafe | JustifyContent::CenterSafe => {
            unreachable!("safe variants are normalized before distribution")
        }
    }

    (distributed, needs_main_axis_fill)
}

fn flex_layout<'a, Message: Clone + 'a>(
    mut items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
    align_items: Option<AlignItems>,
    justify_content: Option<JustifyContent>,
) -> iced::Element<'a, Message> {
    if is_reverse_direction(direction) {
        items.reverse();
    }

    if matches!(align_items, Some(AlignItems::Stretch)) {
        items = items
            .into_iter()
            .map(|item| match direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    iced::widget::container(item).height(Length::Fill).into()
                }
                FlexDirection::Col | FlexDirection::ColReverse => {
                    iced::widget::container(item).width(Length::Fill).into()
                }
            })
            .collect();
    }

    let justify_content = justify_content.map(normalize_justify_content);
    if matches!(justify_content, Some(JustifyContent::Stretch)) {
        items = items
            .into_iter()
            .map(|item| match direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    iced::widget::container(item).width(Length::Fill).into()
                }
                FlexDirection::Col | FlexDirection::ColReverse => {
                    iced::widget::container(item).height(Length::Fill).into()
                }
            })
            .collect();
    }

    let (items, needs_main_axis_fill) =
        distribute_items_for_justify(items, direction, justify_content);

    let gap = spacing_to_px(gap);

    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            let mut row = iced::widget::Row::new().spacing(gap).width(Length::Fill);
            if let Some(items) = align_items {
                row = row
                    .align_y(row_alignment_for_items(items))
                    .height(Length::Fill);
            }
            for item in items {
                row = row.push(item);
            }
            row.into()
        }
        FlexDirection::Col | FlexDirection::ColReverse => {
            let mut col = iced::widget::Column::new().spacing(gap).width(Length::Fill);
            if let Some(items) = align_items {
                col = col.align_x(column_alignment_for_items(items));
            }
            col = if needs_main_axis_fill {
                col.height(Length::Fill)
            } else {
                col.height(Length::Shrink)
            };
            for item in items {
                col = col.push(item);
            }
            col.into()
        }
    }
}

pub(crate) fn gap_on_main_axis(
    direction: FlexDirection,
    gap: Option<Spacing>,
    row_gap: Option<Spacing>,
    col_gap: Option<Spacing>,
) -> Spacing {
    match direction {
        FlexDirection::Row | FlexDirection::RowReverse => col_gap.or(gap).unwrap_or(Spacing::S0),
        FlexDirection::Col | FlexDirection::ColReverse => row_gap.or(gap).unwrap_or(Spacing::S0),
    }
}

/// Create an iced flex layout for a given direction.
pub fn flex_direction_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced layout for `gap-*` utilities.
pub fn gap_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced layout for `gap-x-*` utilities.
///
/// In single-line flex layouts, this maps to the main axis for row directions.
pub fn gap_x_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap_x: Spacing,
) -> iced::Element<'a, Message> {
    let gap = gap_on_main_axis(direction, None, None, Some(gap_x));
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced layout for `gap-y-*` utilities.
///
/// In single-line flex layouts, this maps to the main axis for column directions.
pub fn gap_y_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap_y: Spacing,
) -> iced::Element<'a, Message> {
    let gap = gap_on_main_axis(direction, None, Some(gap_y), None);
    flex_layout(items, direction, gap, None, None)
}

/// Create an iced flex layout for a typed align-items value.
pub fn align_items_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
    align_items: AlignItems,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, Some(align_items), None)
}

/// Create an iced flex layout for a typed justify-content value.
pub fn justify_content_layout<'a, Message: Clone + 'a>(
    items: Vec<iced::Element<'a, Message>>,
    direction: FlexDirection,
    gap: Spacing,
    justify_content: JustifyContent,
) -> iced::Element<'a, Message> {
    flex_layout(items, direction, gap, None, Some(justify_content))
}

fn fraction_to_portion(numerator: u16, denominator: u16) -> u16 {
    if denominator == 0 || numerator == 0 {
        return 0;
    }

    let scaled = (u32::from(numerator) * 100) / u32::from(denominator.max(1));
    scaled.clamp(1, u32::from(u16::MAX)) as u16
}

fn arbitrary_to_length(value: &str) -> Option<Length> {
    let raw = value.trim();
    if raw.eq_ignore_ascii_case("auto") {
        return Some(Length::Fill);
    }
    if raw.eq_ignore_ascii_case("none")
        || raw.eq_ignore_ascii_case("initial")
        || raw == "0 auto"
        || raw == "0 1 auto"
        || raw == "0"
    {
        return Some(Length::Shrink);
    }

    if let Ok(number) = raw.parse::<u16>() {
        return if number == 0 {
            Some(Length::Shrink)
        } else {
            Some(Length::FillPortion(number))
        };
    }

    if let Some((num, den)) = raw.split_once('/') {
        let numerator = num.trim().parse::<u16>().ok()?;
        let denominator = den.trim().parse::<u16>().ok()?;
        let portion = fraction_to_portion(numerator, denominator);
        return if portion == 0 {
            Some(Length::Shrink)
        } else {
            Some(Length::FillPortion(portion))
        };
    }

    // Handle common arbitrary flex shorthands like "3 1 auto" or "2 0 10rem"
    // by mapping the first grow factor to iced FillPortion.
    if let Some(first) = raw.split_whitespace().next()
        && let Ok(grow) = first.parse::<u16>()
    {
        return if grow == 0 {
            Some(Length::Shrink)
        } else {
            Some(Length::FillPortion(grow))
        };
    }

    None
}

fn flex_item_length(flex: &Flex) -> Length {
    match flex {
        Flex::Number(0) => Length::Shrink,
        Flex::Number(number) => Length::FillPortion(*number),
        Flex::Fraction {
            numerator,
            denominator,
        } => {
            let portion = fraction_to_portion(*numerator, denominator.get());
            if portion == 0 {
                Length::Shrink
            } else {
                Length::FillPortion(portion)
            }
        }
        Flex::Auto => Length::Fill,
        Flex::Initial | Flex::None => Length::Shrink,
        Flex::CustomProperty(_) => Length::Shrink,
        Flex::Arbitrary(value) => arbitrary_to_length(value).unwrap_or(Length::Shrink),
    }
}

fn custom_property_length(name: &str, vars: &[(&str, &str)]) -> Option<Length> {
    vars.iter()
        .find(|(key, _)| *key == name)
        .and_then(|(_, value)| arbitrary_to_length(value))
}

/// Apply flex-item shorthand (`flex-*`) for a child element in an iced flex container.
///
/// `direction` is the parent container direction and determines which axis receives
/// the fill/shrink strategy.
pub fn apply_flex_item<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
    direction: FlexDirection,
) -> iced::Element<'a, Message> {
    apply_flex_item_with_custom_properties(content, style, direction, &[])
}

/// Apply flex-item shorthand with custom property values resolver for
/// `flex-(<custom-property>)` use cases.
pub fn apply_flex_item_with_custom_properties<'a, Message: Clone + 'a>(
    content: iced::Element<'a, Message>,
    style: &Style,
    direction: FlexDirection,
    custom_properties: &[(&str, &str)],
) -> iced::Element<'a, Message> {
    let numeric_custom_properties;
    let element = if style_uses_numeric_custom_properties(style) {
        numeric_custom_properties = custom_properties
            .iter()
            .filter_map(|(name, value)| value.parse::<f32>().ok().map(|parsed| (*name, parsed)))
            .collect::<Vec<_>>();
        apply_layout_with_custom_properties(content, style, &numeric_custom_properties)
    } else {
        apply_layout_with_custom_properties(content, style, &[])
    };
    let Some(flex) = style.flex_item.as_ref() else {
        return element;
    };

    let length = match flex {
        Flex::CustomProperty(name) => {
            custom_property_length(name, custom_properties).unwrap_or(Length::Shrink)
        }
        _ => flex_item_length(flex),
    };
    let wrapper = match direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            iced::widget::container(element).width(length)
        }
        FlexDirection::Col | FlexDirection::ColReverse => {
            iced::widget::container(element).height(length)
        }
    };

    wrapper.into()
}

fn style_uses_numeric_custom_properties(style: &Style) -> bool {
    let padding_uses_var = style.padding.is_some_and(|padding| {
        [padding.top, padding.right, padding.bottom, padding.left]
            .into_iter()
            .flatten()
            .any(|value| matches!(value, PaddingValue::Var(_)))
    });

    let margin_uses_var = style.margin.is_some_and(|margin| {
        [margin.top, margin.right, margin.bottom, margin.left]
            .into_iter()
            .flatten()
            .any(|value| matches!(value, MarginValue::Var(_)))
    });

    let width_uses_var = style
        .width
        .is_some_and(|width| matches!(width.size(), Some(crate::utilities::Size::Var(_))));
    let height_uses_var = style
        .height
        .is_some_and(|height| matches!(height.size(), Some(crate::utilities::Size::HeightVar(_))));

    padding_uses_var || margin_uses_var || width_uses_var || height_uses_var
}
