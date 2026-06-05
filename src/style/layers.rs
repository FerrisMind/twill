use std::collections::BTreeMap;

use super::{AriaAttr, DataAttr, Style};
use crate::tokens::Breakpoint;

impl Style {
    /// Returns the configured responsive style layers, if any.
    pub const fn responsive_styles(&self) -> Option<&BTreeMap<Breakpoint, Style>> {
        self.responsive.as_ref()
    }

    /// Returns the style registered for a specific breakpoint, if any.
    pub fn breakpoint_style(&self, breakpoint: Breakpoint) -> Option<&Style> {
        self.responsive
            .as_ref()
            .and_then(|responsive| responsive.get(&breakpoint))
    }

    /// Set styles to apply when the element is hovered.
    pub fn hover<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.hover.unwrap_or_default();
        states.hover = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is focused.
    pub fn focus<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.focus.unwrap_or_default();
        states.focus = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is focus-visible.
    pub fn focus_visible<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.focus_visible.unwrap_or_default();
        states.focus_visible = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is active.
    pub fn active<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.active.unwrap_or_default();
        states.active = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is disabled.
    pub fn disabled<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.disabled.unwrap_or_default();
        states.disabled = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is selected.
    pub fn selected<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.selected.unwrap_or_default();
        states.selected = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is checked.
    pub fn checked<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.checked.unwrap_or_default();
        states.checked = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is open.
    pub fn open<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.open.unwrap_or_default();
        states.open = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles to apply when the element is closed.
    pub fn closed<F>(mut self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let mut states = self.states.unwrap_or_default();
        let current = states.closed.unwrap_or_default();
        states.closed = Some(build(current));
        self.states = Some(states);
        self
    }

    /// Set styles for a typed `data-*` hook such as `data-state=open`.
    pub fn data_attr<F, A>(mut self, attr: A, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
        A: Into<DataAttr>,
    {
        let mut states = self.states.unwrap_or_default();
        let selector = attr.into().selector().into_owned();
        let current = states.data.remove(&selector).unwrap_or_default();
        states.data.insert(selector, build(current));
        self.states = Some(states);
        self
    }

    /// Set styles for an arbitrary `data-*` state hook, like `data-state=open`.
    pub fn data_state<F>(self, name: impl Into<String>, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.data_attr(DataAttr::custom(name.into()), build)
    }

    /// Set styles for a typed `aria-*` hook such as `aria-selected`.
    pub fn aria_attr<F, A>(mut self, attr: A, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
        A: Into<AriaAttr>,
    {
        let mut states = self.states.unwrap_or_default();
        let key = attr.into().as_str().to_owned();
        let current = states.aria.remove(&key).unwrap_or_default();
        states.aria.insert(key, build(current));
        self.states = Some(states);
        self
    }

    /// Set styles for an arbitrary `aria-*` state hook, like `aria-selected`.
    pub fn aria_state<F>(self, name: impl Into<String>, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.aria_attr(AriaAttr::custom(name.into()), build)
    }

    /// Get the hover nested style, if any.
    pub fn hover_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.hover.as_ref())
    }

    /// Get the focus nested style, if any.
    pub fn focus_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.focus.as_ref())
    }

    /// Get the focus-visible nested style, if any.
    pub fn focus_visible_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.focus_visible.as_ref())
    }

    /// Get the active nested style, if any.
    pub fn active_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.active.as_ref())
    }

    /// Get the disabled nested style, if any.
    pub fn disabled_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.disabled.as_ref())
    }

    /// Get the selected nested style, if any.
    pub fn selected_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.selected.as_ref())
    }

    /// Get the checked nested style, if any.
    pub fn checked_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.checked.as_ref())
    }

    /// Get the open nested style, if any.
    pub fn open_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.open.as_ref())
    }

    /// Get the closed nested style, if any.
    pub fn closed_style(&self) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.closed.as_ref())
    }

    /// Get a typed `data-*` nested style, if any.
    pub fn data_attr_style<A>(&self, attr: A) -> Option<&Style>
    where
        A: Into<DataAttr>,
    {
        self.states
            .as_ref()
            .and_then(|s| s.data_attr_style(attr.into()))
    }

    /// Get a named `data-*` nested style, if any.
    pub fn data_state_style(&self, name: &str) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.data_style(name))
    }

    /// Get a typed `aria-*` nested style, if any.
    pub fn aria_attr_style<A>(&self, attr: A) -> Option<&Style>
    where
        A: Into<AriaAttr>,
    {
        self.states
            .as_ref()
            .and_then(|s| s.aria_attr_style(attr.into()))
    }

    /// Get a named `aria-*` nested style, if any.
    pub fn aria_state_style(&self, name: &str) -> Option<&Style> {
        self.states.as_ref().and_then(|s| s.aria_style(name))
    }

    /// Set styles to apply at or above a breakpoint.
    pub fn responsive<F>(mut self, breakpoint: Breakpoint, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        let responsive = self.responsive.get_or_insert_with(BTreeMap::new);
        let current = responsive.remove(&breakpoint).unwrap_or_default();
        responsive.insert(breakpoint, build(current));
        self
    }

    /// Resolve all responsive layers that apply up to the requested breakpoint.
    pub fn at_breakpoint(&self, breakpoint: Breakpoint) -> Self {
        let mut resolved = self.clone();
        resolved.responsive = None;

        if let Some(responsive) = &self.responsive {
            for (layer_breakpoint, layer_style) in responsive {
                if *layer_breakpoint <= breakpoint {
                    resolved = resolved.merged(layer_style.clone());
                }
            }
        }

        resolved
    }

    /// Set styles to apply from the `sm` breakpoint upward.
    pub fn sm<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.responsive(Breakpoint::Sm, build)
    }

    /// Set styles to apply from the `md` breakpoint upward.
    pub fn md<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.responsive(Breakpoint::Md, build)
    }

    /// Set styles to apply from the `lg` breakpoint upward.
    pub fn lg<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.responsive(Breakpoint::Lg, build)
    }

    /// Set styles to apply from the `xl` breakpoint upward.
    pub fn xl<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.responsive(Breakpoint::Xl, build)
    }

    /// Set styles to apply from the `2xl` breakpoint upward.
    pub fn s2xl<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.responsive(Breakpoint::S2xl, build)
    }

    /// Verbose alias for `sm(...)`.
    pub fn at_sm<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.sm(build)
    }

    /// Verbose alias for `md(...)`.
    pub fn at_md<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.md(build)
    }

    /// Verbose alias for `lg(...)`.
    pub fn at_lg<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.lg(build)
    }

    /// Verbose alias for `xl(...)`.
    pub fn at_xl<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.xl(build)
    }

    /// Verbose alias for `s2xl(...)`.
    pub fn at_2xl<F>(self, build: F) -> Self
    where
        F: FnOnce(Style) -> Style,
    {
        self.s2xl(build)
    }
}
