use std::collections::BTreeMap;

use crate::style::Style;
use crate::traits::Merge;

/// Modifiers for interaction and stateful styling like hover, focus, data, and aria hooks.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StateStyles {
    /// Styles applied when the user hovers over the element.
    pub hover: Option<Style>,
    /// Styles applied when the element has programmatic or keyboard focus.
    pub focus: Option<Style>,
    /// Styles applied when the element matches `:focus-visible`.
    pub focus_visible: Option<Style>,
    /// Styles applied when the element is active (e.g. while being clicked/pressed).
    pub active: Option<Style>,
    /// Styles applied when the element is disabled and non-interactive.
    pub disabled: Option<Style>,
    /// Styles applied when the element is selected.
    pub selected: Option<Style>,
    /// Styles applied when the element is checked.
    pub checked: Option<Style>,
    /// Styles applied when the element is in an open state.
    pub open: Option<Style>,
    /// Styles applied when the element is in a closed state.
    pub closed: Option<Style>,
    /// Styles keyed by arbitrary `data-*` state names.
    pub data: BTreeMap<String, Style>,
    /// Styles keyed by arbitrary `aria-*` state names.
    pub aria: BTreeMap<String, Style>,
}

impl StateStyles {
    /// Create a new empty state styles container.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Merge<Self> for StateStyles {
    fn merge(&self, other: Self) -> Self {
        Self {
            hover: merge_style_opt(self.hover.clone(), other.hover),
            focus: merge_style_opt(self.focus.clone(), other.focus),
            focus_visible: merge_style_opt(self.focus_visible.clone(), other.focus_visible),
            active: merge_style_opt(self.active.clone(), other.active),
            disabled: merge_style_opt(self.disabled.clone(), other.disabled),
            selected: merge_style_opt(self.selected.clone(), other.selected),
            checked: merge_style_opt(self.checked.clone(), other.checked),
            open: merge_style_opt(self.open.clone(), other.open),
            closed: merge_style_opt(self.closed.clone(), other.closed),
            data: merge_named_states(&self.data, other.data),
            aria: merge_named_states(&self.aria, other.aria),
        }
    }
}

fn merge_style_opt(a: Option<Style>, b: Option<Style>) -> Option<Style> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.merge(b)),
        (None, Some(b)) => Some(b),
        (Some(a), None) => Some(a),
        (None, None) => None,
    }
}

fn merge_named_states(
    current: &BTreeMap<String, Style>,
    incoming: BTreeMap<String, Style>,
) -> BTreeMap<String, Style> {
    let mut merged = current.clone();

    for (name, style) in incoming {
        if let Some(existing) = merged.get(&name).cloned() {
            merged.insert(name, existing.merge(style));
        } else {
            merged.insert(name, style);
        }
    }

    merged
}
