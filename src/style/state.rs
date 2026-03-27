use std::collections::BTreeMap;

use crate::style::Style;
use crate::traits::Merge;

/// Modifiers for interaction and stateful styling like hover, focus, data, and aria hooks.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StateStyles {
    /// Styles applied when the user hovers over the element.
    pub(crate) hover: Option<Style>,
    /// Styles applied when the element has programmatic or keyboard focus.
    pub(crate) focus: Option<Style>,
    /// Styles applied when the element matches `:focus-visible`.
    pub(crate) focus_visible: Option<Style>,
    /// Styles applied when the element is active (e.g. while being clicked/pressed).
    pub(crate) active: Option<Style>,
    /// Styles applied when the element is disabled and non-interactive.
    pub(crate) disabled: Option<Style>,
    /// Styles applied when the element is selected.
    pub(crate) selected: Option<Style>,
    /// Styles applied when the element is checked.
    pub(crate) checked: Option<Style>,
    /// Styles applied when the element is in an open state.
    pub(crate) open: Option<Style>,
    /// Styles applied when the element is in a closed state.
    pub(crate) closed: Option<Style>,
    /// Styles keyed by arbitrary `data-*` state names.
    pub(crate) data: BTreeMap<String, Style>,
    /// Styles keyed by arbitrary `aria-*` state names.
    pub(crate) aria: BTreeMap<String, Style>,
}

impl StateStyles {
    /// Create a new empty state styles container.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hover_style(&self) -> Option<&Style> {
        self.hover.as_ref()
    }

    pub fn focus_style(&self) -> Option<&Style> {
        self.focus.as_ref()
    }

    pub fn focus_visible_style(&self) -> Option<&Style> {
        self.focus_visible.as_ref()
    }

    pub fn active_style(&self) -> Option<&Style> {
        self.active.as_ref()
    }

    pub fn disabled_style(&self) -> Option<&Style> {
        self.disabled.as_ref()
    }

    pub fn selected_style(&self) -> Option<&Style> {
        self.selected.as_ref()
    }

    pub fn checked_style(&self) -> Option<&Style> {
        self.checked.as_ref()
    }

    pub fn open_style(&self) -> Option<&Style> {
        self.open.as_ref()
    }

    pub fn closed_style(&self) -> Option<&Style> {
        self.closed.as_ref()
    }

    pub fn data_style(&self, name: &str) -> Option<&Style> {
        self.data.get(name)
    }

    pub fn aria_style(&self, name: &str) -> Option<&Style> {
        self.aria.get(name)
    }
}

impl Merge<Self> for StateStyles {
    fn merge(&self, other: Self) -> Self {
        self.merge_ref(&other)
    }
}

impl StateStyles {
    pub(crate) fn merge_ref(&self, other: &Self) -> Self {
        Self {
            hover: merge_style_opt_ref(self.hover.as_ref(), other.hover.as_ref()),
            focus: merge_style_opt_ref(self.focus.as_ref(), other.focus.as_ref()),
            focus_visible: merge_style_opt_ref(
                self.focus_visible.as_ref(),
                other.focus_visible.as_ref(),
            ),
            active: merge_style_opt_ref(self.active.as_ref(), other.active.as_ref()),
            disabled: merge_style_opt_ref(self.disabled.as_ref(), other.disabled.as_ref()),
            selected: merge_style_opt_ref(self.selected.as_ref(), other.selected.as_ref()),
            checked: merge_style_opt_ref(self.checked.as_ref(), other.checked.as_ref()),
            open: merge_style_opt_ref(self.open.as_ref(), other.open.as_ref()),
            closed: merge_style_opt_ref(self.closed.as_ref(), other.closed.as_ref()),
            data: merge_named_states(&self.data, &other.data),
            aria: merge_named_states(&self.aria, &other.aria),
        }
    }
}

fn merge_style_opt_ref(current: Option<&Style>, incoming: Option<&Style>) -> Option<Style> {
    match (current, incoming) {
        (Some(current), Some(incoming)) => Some(current.merge(incoming.clone())),
        (None, Some(incoming)) => Some(incoming.clone()),
        (Some(current), None) => Some(current.clone()),
        (None, None) => None,
    }
}

fn merge_named_states(
    current: &BTreeMap<String, Style>,
    incoming: &BTreeMap<String, Style>,
) -> BTreeMap<String, Style> {
    let mut merged = current.clone();

    for (name, style) in incoming {
        if let Some(existing) = merged.get(name).cloned() {
            merged.insert(name.clone(), existing.merge(style.clone()));
        } else {
            merged.insert(name.clone(), style.clone());
        }
    }

    merged
}
