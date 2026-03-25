//! Style builder for composing styles fluently.

use std::num::NonZeroU8;

use crate::tokens::{
    AnimationToken, AspectRatio, BackgroundColor, BackgroundColorVar, Blur, BorderRadius,
    BorderStyle, BorderWidth, Color, ColorValueToken, Container, Cursor, DropShadow, Easing,
    FontFamily, FontSize, FontSizeVar, FontWeight, InsetShadow, LetterSpacing, LineHeight,
    MotionDefaults, OutlineStyle, Percentage, Perspective, RingWidth, Shadow, Spacing, TextAlign,
    TextDecoration, TextShadow, TextTransform, TransitionDuration, TransitionProperty,
};
use crate::traits::Merge;
use crate::utilities::{
    AlignItems, Columns, Display, Flex, FlexContainer, FlexDirection, GridContainer, GridTemplate,
    Height, HeightVar, JustifyContent, JustifyItems, JustifySelf, Margin, MarginValue, ObjectFit,
    Overflow, Padding, PaddingValue, PlaceContent, PlaceItems, Position, Size, SizeConstraints,
    Visibility, Width, WidthVar, ZIndex,
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
    pub background_color: Option<BackgroundColor>,
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

    /// Set flex direction.
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

    /// Set align-items utility for flex containers.
    pub fn align_items(mut self, align: AlignItems) -> Self {
        if let Some(ref mut flex) = self.flex {
            self.flex = Some(FlexContainer {
                align: Some(align),
                ..flex.clone()
            });
        } else {
            self.flex = Some(FlexContainer::new().align(align));
        }
        self
    }

    /// Set main-axis content distribution for a flex container.
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        if let Some(ref mut flex) = self.flex {
            self.flex = Some(FlexContainer {
                justify: Some(justify),
                ..flex.clone()
            });
        } else {
            self.flex = Some(FlexContainer::new().justify(justify));
        }
        self
    }

    /// Set cross-axis alignment to start.
    pub fn items_start(self) -> Self {
        self.align_items(AlignItems::Start)
    }

    /// Set cross-axis alignment to end.
    pub fn items_end(self) -> Self {
        self.align_items(AlignItems::End)
    }

    /// Set cross-axis alignment to safe end.
    pub fn items_end_safe(self) -> Self {
        self.align_items(AlignItems::EndSafe)
    }

    /// Set cross-axis alignment to center.
    pub fn items_center(self) -> Self {
        self.align_items(AlignItems::Center)
    }

    /// Set cross-axis alignment to safe center.
    pub fn items_center_safe(self) -> Self {
        self.align_items(AlignItems::CenterSafe)
    }

    /// Set cross-axis alignment to baseline.
    pub fn items_baseline(self) -> Self {
        self.align_items(AlignItems::Baseline)
    }

    /// Set cross-axis alignment to last baseline.
    pub fn items_baseline_last(self) -> Self {
        self.align_items(AlignItems::BaselineLast)
    }

    /// Stretch children on the cross axis.
    pub fn items_stretch(self) -> Self {
        self.align_items(AlignItems::Stretch)
    }

    /// Set flex item behavior.
    pub fn flex_item(mut self, flex: Flex) -> Self {
        self.flex_item = Some(flex);
        self
    }

    /// Set numeric flex grow factor.
    pub fn flex_number(self, number: u16) -> Self {
        self.flex_item(Flex::number(number))
    }

    /// Set flex grow factor as a fraction.
    pub fn flex_fraction(self, numerator: u16, denominator: u16) -> Self {
        self.flex_item(Flex::fraction(numerator, denominator))
    }

    /// Convenience: flex grow factor `1`.
    pub fn flex_1(self) -> Self {
        self.flex_number(1)
    }

    /// Set automatic flex behavior.
    pub fn flex_auto(self) -> Self {
        self.flex_item(Flex::Auto)
    }

    /// Set initial flex behavior.
    pub fn flex_initial(self) -> Self {
        self.flex_item(Flex::Initial)
    }

    /// Disable flex growth and shrink.
    pub fn flex_none(self) -> Self {
        self.flex_item(Flex::None)
    }

    /// Set flex behavior from a custom property.
    pub fn flex_custom_property(self, name: impl Into<String>) -> Self {
        self.flex_item(Flex::custom_property(name))
    }

    /// Set an arbitrary flex behavior value.
    pub fn flex_arbitrary(self, value: impl Into<String>) -> Self {
        self.flex_item(Flex::arbitrary(value))
    }

    /// Set grid container properties.
    pub fn grid(mut self, grid: GridContainer) -> Self {
        self.grid = Some(grid);
        self
    }

    /// Set grid template columns.
    pub fn grid_cols(mut self, cols: GridTemplate) -> Self {
        let mut grid = self.grid.unwrap_or_default();
        grid.columns = Some(cols);
        self.grid = Some(grid);
        self
    }

    /// Set repeated equal grid columns.
    pub fn grid_cols_count(self, count: u16) -> Self {
        self.grid_cols(GridTemplate::count(count))
    }

    /// Set grid columns to `none`.
    pub fn grid_cols_none(self) -> Self {
        self.grid_cols(GridTemplate::none())
    }

    /// Set grid columns to `subgrid`.
    pub fn grid_cols_subgrid(self) -> Self {
        self.grid_cols(GridTemplate::subgrid())
    }

    /// Set grid columns from a custom property.
    pub fn grid_cols_custom_property(self, name: impl Into<String>) -> Self {
        self.grid_cols(GridTemplate::custom_property(name))
    }

    /// Set grid columns from an arbitrary template value.
    pub fn grid_cols_arbitrary(self, value: impl Into<String>) -> Self {
        self.grid_cols(GridTemplate::arbitrary(value))
    }

    /// Set gap (for flex/grid).
    pub fn gap(mut self, spacing: Spacing) -> Self {
        if let Some(flex) = self.flex.take() {
            self.flex = Some(FlexContainer {
                gap: Some(spacing),
                ..flex
            });
        }

        if let Some(grid) = self.grid.take() {
            self.grid = Some(GridContainer {
                gap: Some(spacing),
                ..grid
            });
        }

        if self.flex.is_none() && self.grid.is_none() {
            self.flex = Some(FlexContainer::new().gap(spacing));
        }

        self
    }

    /// Set horizontal gap (`gap-x-*` family) for flex/grid.
    pub fn gap_x(mut self, spacing: Spacing) -> Self {
        if let Some(flex) = self.flex.take() {
            self.flex = Some(FlexContainer {
                col_gap: Some(spacing),
                ..flex
            });
        }

        if let Some(grid) = self.grid.take() {
            self.grid = Some(GridContainer {
                col_gap: Some(spacing),
                ..grid
            });
        }

        if self.flex.is_none() && self.grid.is_none() {
            self.flex = Some(FlexContainer::new().gap_x(spacing));
        }

        self
    }

    /// Set vertical gap (`gap-y-*` family) for flex/grid.
    pub fn gap_y(mut self, spacing: Spacing) -> Self {
        if let Some(flex) = self.flex.take() {
            self.flex = Some(FlexContainer {
                row_gap: Some(spacing),
                ..flex
            });
        }

        if let Some(grid) = self.grid.take() {
            self.grid = Some(GridContainer {
                row_gap: Some(spacing),
                ..grid
            });
        }

        if self.flex.is_none() && self.grid.is_none() {
            self.flex = Some(FlexContainer::new().gap_y(spacing));
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

    fn update_padding_value(
        mut self,
        update: impl FnOnce(&mut Padding, PaddingValue),
        value: PaddingValue,
    ) -> Self {
        let mut padding = self.padding.unwrap_or_default();
        update(&mut padding, value);
        self.padding = Some(padding);
        self
    }

    fn update_margin_value(
        mut self,
        update: impl FnOnce(&mut Margin, MarginValue),
        value: MarginValue,
    ) -> Self {
        let mut margin = self.margin.unwrap_or_default();
        update(&mut margin, value);
        self.margin = Some(margin);
        self
    }

    /// `p-*` family: set all padding sides.
    pub fn p(self, spacing: Spacing) -> Self {
        self.p_value(spacing.into())
    }

    /// `p-(<custom-property>)` / `p-[<value>]` family.
    pub fn p_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.top = Some(value);
                padding.right = Some(value);
                padding.bottom = Some(value);
                padding.left = Some(value);
            },
            value,
        )
    }

    /// `px-*` family: set horizontal padding.
    pub fn px(self, spacing: Spacing) -> Self {
        self.px_value(spacing.into())
    }

    /// `px-(<custom-property>)` / `px-[<value>]` family.
    pub fn px_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.right = Some(value);
                padding.left = Some(value);
            },
            value,
        )
    }

    /// `py-*` family: set vertical padding.
    pub fn py(self, spacing: Spacing) -> Self {
        self.py_value(spacing.into())
    }

    /// `py-(<custom-property>)` / `py-[<value>]` family.
    pub fn py_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.top = Some(value);
                padding.bottom = Some(value);
            },
            value,
        )
    }

    /// `ps-*` family (`padding-inline-start`) for default LTR mapping.
    pub fn ps(self, spacing: Spacing) -> Self {
        self.ps_value(spacing.into())
    }

    /// `ps-(<custom-property>)` / `ps-[<value>]` family.
    pub fn ps_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.left = Some(value);
            },
            value,
        )
    }

    /// `pe-*` family (`padding-inline-end`) for default LTR mapping.
    pub fn pe(self, spacing: Spacing) -> Self {
        self.pe_value(spacing.into())
    }

    /// `pe-(<custom-property>)` / `pe-[<value>]` family.
    pub fn pe_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.right = Some(value);
            },
            value,
        )
    }

    /// `pbs-*` family (`padding-block-start`).
    pub fn pbs(self, spacing: Spacing) -> Self {
        self.pbs_value(spacing.into())
    }

    /// `pbs-(<custom-property>)` / `pbs-[<value>]` family.
    pub fn pbs_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.top = Some(value);
            },
            value,
        )
    }

    /// `pbe-*` family (`padding-block-end`).
    pub fn pbe(self, spacing: Spacing) -> Self {
        self.pbe_value(spacing.into())
    }

    /// `pbe-(<custom-property>)` / `pbe-[<value>]` family.
    pub fn pbe_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.bottom = Some(value);
            },
            value,
        )
    }

    /// `pt-*` family.
    pub fn pt(self, spacing: Spacing) -> Self {
        self.pt_value(spacing.into())
    }

    /// `pt-(<custom-property>)` / `pt-[<value>]` family.
    pub fn pt_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.top = Some(value);
            },
            value,
        )
    }

    /// `pr-*` family.
    pub fn pr(self, spacing: Spacing) -> Self {
        self.pr_value(spacing.into())
    }

    /// `pr-(<custom-property>)` / `pr-[<value>]` family.
    pub fn pr_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.right = Some(value);
            },
            value,
        )
    }

    /// `pb-*` family.
    pub fn pb(self, spacing: Spacing) -> Self {
        self.pb_value(spacing.into())
    }

    /// `pb-(<custom-property>)` / `pb-[<value>]` family.
    pub fn pb_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.bottom = Some(value);
            },
            value,
        )
    }

    /// `pl-*` family.
    pub fn pl(self, spacing: Spacing) -> Self {
        self.pl_value(spacing.into())
    }

    /// `pl-(<custom-property>)` / `pl-[<value>]` family.
    pub fn pl_value(self, value: PaddingValue) -> Self {
        self.update_padding_value(
            |padding, value| {
                padding.left = Some(value);
            },
            value,
        )
    }

    /// `m-*` family: set all margin sides.
    pub fn m(self, spacing: Spacing) -> Self {
        self.m_value(spacing.into())
    }

    /// `m-(<custom-property>)` / `m-[<value>]` family.
    pub fn m_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.top = Some(value);
                margin.right = Some(value);
                margin.bottom = Some(value);
                margin.left = Some(value);
            },
            value,
        )
    }

    /// `-m-*` family.
    pub fn neg_m(self, spacing: Spacing) -> Self {
        self.m_value(MarginValue::neg_scale(spacing))
    }

    /// `mx-*` family: set horizontal margin.
    pub fn mx(self, spacing: Spacing) -> Self {
        self.mx_value(spacing.into())
    }

    /// `mx-(<custom-property>)` / `mx-[<value>]` family.
    pub fn mx_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.right = Some(value);
                margin.left = Some(value);
            },
            value,
        )
    }

    /// `-mx-*` family.
    pub fn neg_mx(self, spacing: Spacing) -> Self {
        self.mx_value(MarginValue::neg_scale(spacing))
    }

    /// `my-*` family: set vertical margin.
    pub fn my(self, spacing: Spacing) -> Self {
        self.my_value(spacing.into())
    }

    /// `my-(<custom-property>)` / `my-[<value>]` family.
    pub fn my_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.top = Some(value);
                margin.bottom = Some(value);
            },
            value,
        )
    }

    /// `-my-*` family.
    pub fn neg_my(self, spacing: Spacing) -> Self {
        self.my_value(MarginValue::neg_scale(spacing))
    }

    /// `ms-*` family (`margin-inline-start`) for default LTR mapping.
    pub fn ms(self, spacing: Spacing) -> Self {
        self.ms_value(spacing.into())
    }

    /// `ms-(<custom-property>)` / `ms-[<value>]` family.
    pub fn ms_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.left = Some(value);
            },
            value,
        )
    }

    /// `-ms-*` family.
    pub fn neg_ms(self, spacing: Spacing) -> Self {
        self.ms_value(MarginValue::neg_scale(spacing))
    }

    /// `me-*` family (`margin-inline-end`) for default LTR mapping.
    pub fn me(self, spacing: Spacing) -> Self {
        self.me_value(spacing.into())
    }

    /// `me-(<custom-property>)` / `me-[<value>]` family.
    pub fn me_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.right = Some(value);
            },
            value,
        )
    }

    /// `-me-*` family.
    pub fn neg_me(self, spacing: Spacing) -> Self {
        self.me_value(MarginValue::neg_scale(spacing))
    }

    /// `mbs-*` family (`margin-block-start`).
    pub fn mbs(self, spacing: Spacing) -> Self {
        self.mbs_value(spacing.into())
    }

    /// `mbs-(<custom-property>)` / `mbs-[<value>]` family.
    pub fn mbs_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.top = Some(value);
            },
            value,
        )
    }

    /// `-mbs-*` family.
    pub fn neg_mbs(self, spacing: Spacing) -> Self {
        self.mbs_value(MarginValue::neg_scale(spacing))
    }

    /// `mbe-*` family (`margin-block-end`).
    pub fn mbe(self, spacing: Spacing) -> Self {
        self.mbe_value(spacing.into())
    }

    /// `mbe-(<custom-property>)` / `mbe-[<value>]` family.
    pub fn mbe_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.bottom = Some(value);
            },
            value,
        )
    }

    /// `-mbe-*` family.
    pub fn neg_mbe(self, spacing: Spacing) -> Self {
        self.mbe_value(MarginValue::neg_scale(spacing))
    }

    /// `mt-*` family.
    pub fn mt(self, spacing: Spacing) -> Self {
        self.mt_value(spacing.into())
    }

    /// `mt-(<custom-property>)` / `mt-[<value>]` family.
    pub fn mt_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.top = Some(value);
            },
            value,
        )
    }

    /// `-mt-*` family.
    pub fn neg_mt(self, spacing: Spacing) -> Self {
        self.mt_value(MarginValue::neg_scale(spacing))
    }

    /// `mr-*` family.
    pub fn mr(self, spacing: Spacing) -> Self {
        self.mr_value(spacing.into())
    }

    /// `mr-(<custom-property>)` / `mr-[<value>]` family.
    pub fn mr_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.right = Some(value);
            },
            value,
        )
    }

    /// `-mr-*` family.
    pub fn neg_mr(self, spacing: Spacing) -> Self {
        self.mr_value(MarginValue::neg_scale(spacing))
    }

    /// `mb-*` family.
    pub fn mb(self, spacing: Spacing) -> Self {
        self.mb_value(spacing.into())
    }

    /// `mb-(<custom-property>)` / `mb-[<value>]` family.
    pub fn mb_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.bottom = Some(value);
            },
            value,
        )
    }

    /// `-mb-*` family.
    pub fn neg_mb(self, spacing: Spacing) -> Self {
        self.mb_value(MarginValue::neg_scale(spacing))
    }

    /// `ml-*` family.
    pub fn ml(self, spacing: Spacing) -> Self {
        self.ml_value(spacing.into())
    }

    /// `ml-(<custom-property>)` / `ml-[<value>]` family.
    pub fn ml_value(self, value: MarginValue) -> Self {
        self.update_margin_value(
            |margin, value| {
                margin.left = Some(value);
            },
            value,
        )
    }

    /// `-ml-*` family.
    pub fn neg_ml(self, spacing: Spacing) -> Self {
        self.ml_value(MarginValue::neg_scale(spacing))
    }

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

    /// `w-<number>` family (`w-px` uses `Spacing::Px`).
    pub fn w(self, spacing: Spacing) -> Self {
        self.width(Width::w(spacing))
    }

    /// `w-<fraction>` family.
    pub fn w_fraction(self, fraction: Percentage) -> Self {
        self.width(Width::w_fraction(fraction))
    }

    /// `w-3xs` ... `w-7xl` family.
    pub fn w_container(self, container: Container) -> Self {
        self.width(Width::w_container(container))
    }

    /// `w-auto`.
    pub fn w_auto(self) -> Self {
        self.width(Width::w_auto())
    }

    /// `w-px`.
    pub fn w_px(self) -> Self {
        self.width(Width::w_px())
    }

    /// `w-screen`.
    pub fn w_screen(self) -> Self {
        self.width(Width::w_screen())
    }

    /// `w-dvw`.
    pub fn w_dvw(self) -> Self {
        self.width(Width::w_dvw())
    }

    /// `w-dvh`.
    pub fn w_dvh(self) -> Self {
        self.width(Width::w_dvh())
    }

    /// `w-lvw`.
    pub fn w_lvw(self) -> Self {
        self.width(Width::w_lvw())
    }

    /// `w-lvh`.
    pub fn w_lvh(self) -> Self {
        self.width(Width::w_lvh())
    }

    /// `w-svw`.
    pub fn w_svw(self) -> Self {
        self.width(Width::w_svw())
    }

    /// `w-svh`.
    pub fn w_svh(self) -> Self {
        self.width(Width::w_svh())
    }

    /// `w-min`.
    pub fn w_min(self) -> Self {
        self.width(Width::w_min())
    }

    /// `w-max`.
    pub fn w_max(self) -> Self {
        self.width(Width::w_max())
    }

    /// `w-fit`.
    pub fn w_fit(self) -> Self {
        self.width(Width::w_fit())
    }

    /// `w-(<custom-property>)`.
    pub fn w_var(self, var: WidthVar) -> Self {
        self.width(Width::w_var(var))
    }

    /// Typed pixel width (`w-[<value>]` in px-focused native layouts).
    pub fn w_px_value(self, px: u16) -> Self {
        self.width(Width::w_px_value(px))
    }

    /// Set height.
    pub fn height(mut self, height: Height) -> Self {
        self.height = Some(height);
        self
    }

    /// `h-<number>` family (`h-px` uses `Spacing::Px`).
    pub fn h(self, spacing: Spacing) -> Self {
        self.height(Height::h(spacing))
    }

    /// `h-<fraction>` family.
    pub fn h_fraction(self, fraction: Percentage) -> Self {
        self.height(Height::h_fraction(fraction))
    }

    /// `h-auto`.
    pub fn h_auto(self) -> Self {
        self.height(Height::h_auto())
    }

    /// `h-px`.
    pub fn h_px(self) -> Self {
        self.height(Height::h_px())
    }

    /// `h-screen`.
    pub fn h_screen(self) -> Self {
        self.height(Height::h_screen())
    }

    /// `h-dvw`.
    pub fn h_dvw(self) -> Self {
        self.height(Height::h_dvw())
    }

    /// `h-dvh`.
    pub fn h_dvh(self) -> Self {
        self.height(Height::h_dvh())
    }

    /// `h-lvw`.
    pub fn h_lvw(self) -> Self {
        self.height(Height::h_lvw())
    }

    /// `h-lvh`.
    pub fn h_lvh(self) -> Self {
        self.height(Height::h_lvh())
    }

    /// `h-svw`.
    pub fn h_svw(self) -> Self {
        self.height(Height::h_svw())
    }

    /// `h-svh`.
    pub fn h_svh(self) -> Self {
        self.height(Height::h_svh())
    }

    /// `h-min`.
    pub fn h_min(self) -> Self {
        self.height(Height::h_min())
    }

    /// `h-max`.
    pub fn h_max(self) -> Self {
        self.height(Height::h_max())
    }

    /// `h-fit`.
    pub fn h_fit(self) -> Self {
        self.height(Height::h_fit())
    }

    /// `h-lh`.
    pub fn h_lh(self) -> Self {
        self.height(Height::h_lh())
    }

    /// `h-(<custom-property>)`.
    pub fn h_var(self, var: HeightVar) -> Self {
        self.height(Height::h_var(var))
    }

    /// Typed pixel height (`h-[<value>]` in px-focused native layouts).
    pub fn h_px_value(self, px: u16) -> Self {
        self.height(Height::h_px_value(px))
    }

    /// Set size constraints.
    pub fn constraints(mut self, constraints: SizeConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }

    /// Set max-width to prose measure (`65ch`).
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
        self.background_color = Some(BackgroundColor::palette(color));
        self
    }

    /// Set background color (alias for bg).
    pub fn background(self, color: Color) -> Self {
        self.bg(color)
    }

    /// Set background token directly.
    pub fn background_token(mut self, token: BackgroundColor) -> Self {
        self.background_color = Some(token);
        self
    }

    /// `bg-inherit` equivalent.
    pub fn bg_inherit(self) -> Self {
        self.background_token(BackgroundColor::inherit())
    }

    /// `bg-current` equivalent.
    pub fn bg_current(self) -> Self {
        self.background_token(BackgroundColor::current())
    }

    /// `bg-transparent` equivalent.
    pub fn bg_transparent(self) -> Self {
        self.background_token(BackgroundColor::transparent())
    }

    /// `bg-(<custom-property>)` equivalent.
    pub fn bg_var(self, var: BackgroundColorVar) -> Self {
        self.background_token(BackgroundColor::custom_property(var))
    }

    /// `bg-[<value>]` equivalent.
    pub fn bg_arbitrary(self, value: ColorValueToken) -> Self {
        self.background_token(BackgroundColor::arbitrary(value))
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

    /// `text-(length:<custom-property>)`.
    pub fn text_size_var(mut self, var: FontSizeVar) -> Self {
        self.font_size = Some(FontSize::var(var));
        self
    }

    /// Typed arbitrary pixel font size (`text-[<value>]`).
    pub fn text_size_px(mut self, px: u16) -> Self {
        self.font_size = Some(FontSize::px(px));
        self
    }

    /// Set font weight.
    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    /// `font-thin`.
    pub fn font_thin(self) -> Self {
        self.font_weight(FontWeight::Thin)
    }

    /// `font-extralight`.
    pub fn font_extralight(self) -> Self {
        self.font_weight(FontWeight::ExtraLight)
    }

    /// `font-light`.
    pub fn font_light(self) -> Self {
        self.font_weight(FontWeight::Light)
    }

    /// `font-normal`.
    pub fn font_normal(self) -> Self {
        self.font_weight(FontWeight::Normal)
    }

    /// `font-medium`.
    pub fn font_medium(self) -> Self {
        self.font_weight(FontWeight::Medium)
    }

    /// `font-semibold`.
    pub fn font_semibold(self) -> Self {
        self.font_weight(FontWeight::SemiBold)
    }

    /// `font-bold`.
    pub fn font_bold(self) -> Self {
        self.font_weight(FontWeight::Bold)
    }

    /// `font-extrabold`.
    pub fn font_extrabold(self) -> Self {
        self.font_weight(FontWeight::ExtraBold)
    }

    /// `font-black`.
    pub fn font_black(self) -> Self {
        self.font_weight(FontWeight::Black)
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

    /// `text-left`.
    pub fn text_left(self) -> Self {
        self.text_align(TextAlign::Left)
    }

    /// `text-center`.
    pub fn text_center(self) -> Self {
        self.text_align(TextAlign::Center)
    }

    /// `text-right`.
    pub fn text_right(self) -> Self {
        self.text_align(TextAlign::Right)
    }

    /// `text-justify`.
    pub fn text_justify(self) -> Self {
        self.text_align(TextAlign::Justify)
    }

    /// `text-start`.
    pub fn text_start(self) -> Self {
        self.text_align(TextAlign::Start)
    }

    /// `text-end`.
    pub fn text_end(self) -> Self {
        self.text_align(TextAlign::End)
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

    /// Set transition property using a predefined token.
    pub fn transition(mut self, property: TransitionProperty) -> Self {
        self.transition_property = Some(property.value().to_string());
        self
    }

    /// Apply default transition preset.
    pub fn transition_default(mut self) -> Self {
        let defaults = MotionDefaults::default();
        self.transition_property = Some(TransitionProperty::Default.value().to_string());
        self.transition_duration = Some(defaults.duration);
        self.transition_timing_function = Some(defaults.easing);
        self
    }

    /// Set transition duration using duration tokens.
    pub fn transition_duration(mut self, duration: TransitionDuration) -> Self {
        self.transition_duration = Some(duration);
        self
    }

    /// Set transition timing function using easing tokens.
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
            flex: merge_flex_container(self.flex.clone(), other.flex),
            flex_item: other.flex_item.or(self.flex_item.clone()),
            grid: merge_grid_container(self.grid.clone(), other.grid),
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
        assert_eq!(
            style.background_color,
            Some(BackgroundColor::palette(Color::blue(Scale::S500)))
        );
        assert_eq!(style.border_radius, Some(BorderRadius::Md));
    }

    #[test]
    fn test_padding_family_builders() {
        let style = Style::new()
            .px(Spacing::S4)
            .pt(Spacing::S2)
            .pb(Spacing::S6)
            .ps(Spacing::S8);

        let padding = style.padding.expect("padding must be present");
        assert_eq!(padding.left, Some(PaddingValue::scale(Spacing::S8)));
        assert_eq!(padding.right, Some(PaddingValue::scale(Spacing::S4)));
        assert_eq!(padding.top, Some(PaddingValue::scale(Spacing::S2)));
        assert_eq!(padding.bottom, Some(PaddingValue::scale(Spacing::S6)));
    }

    #[test]
    fn test_padding_px_token_builders() {
        let style = Style::new()
            .p(Spacing::Px)
            .px(Spacing::S4)
            .py(Spacing::Px)
            .pe(Spacing::Px);

        let padding = style.padding.expect("padding must be present");
        assert_eq!(padding.top, Some(PaddingValue::scale(Spacing::Px)));
        assert_eq!(padding.bottom, Some(PaddingValue::scale(Spacing::Px)));
        assert_eq!(padding.left, Some(PaddingValue::scale(Spacing::S4)));
        assert_eq!(padding.right, Some(PaddingValue::scale(Spacing::Px)));
    }

    #[test]
    fn test_padding_p_overrides_all_sides() {
        let style = Style::new().px(Spacing::S2).py(Spacing::S3).p(Spacing::S5);
        assert_eq!(style.padding, Some(Padding::all(Spacing::S5)));
    }

    #[test]
    fn test_padding_value_builders() {
        let style = Style::new()
            .px_value(PaddingValue::px(5.0))
            .pt_value(PaddingValue::rem(1.5))
            .ps_value(PaddingValue::var(crate::utilities::PaddingVar::new(
                "--pad",
            )));

        let padding = style.padding.expect("padding must be present");
        assert_eq!(
            padding.left,
            Some(PaddingValue::var(crate::utilities::PaddingVar::new(
                "--pad"
            )))
        );
        assert_eq!(padding.right, Some(PaddingValue::px(5.0)));
        assert_eq!(padding.top, Some(PaddingValue::rem(1.5)));
    }

    #[test]
    fn test_width_family_builders() {
        let style = Style::new()
            .w(Spacing::S24)
            .w_fraction(Percentage::S1_2)
            .w_container(Container::Md)
            .w_px();

        assert_eq!(style.width, Some(Width::w_px()));
    }

    #[test]
    fn test_width_variant_builders() {
        const PANEL_W: WidthVar = WidthVar::new("--panel-w");
        let style = Style::new()
            .w_auto()
            .w_screen()
            .w_dvw()
            .w_dvh()
            .w_lvw()
            .w_lvh()
            .w_svw()
            .w_svh()
            .w_min()
            .w_max()
            .w_fit()
            .w_var(PANEL_W)
            .w_px_value(280);

        assert_eq!(style.width, Some(Width::w_px_value(280)));
    }

    #[test]
    fn test_height_family_builders() {
        let style = Style::new()
            .h(Spacing::S24)
            .h_fraction(Percentage::S1_2)
            .h_px();

        assert_eq!(style.height, Some(Height::h_px()));
    }

    #[test]
    fn test_height_variant_builders() {
        const PANEL_H: HeightVar = HeightVar::new("--panel-h");
        let style = Style::new()
            .h_auto()
            .h_screen()
            .h_dvw()
            .h_dvh()
            .h_lvw()
            .h_lvh()
            .h_svw()
            .h_svh()
            .h_min()
            .h_max()
            .h_fit()
            .h_lh()
            .h_var(PANEL_H)
            .h_px_value(220);

        assert_eq!(style.height, Some(Height::h_px_value(220)));
    }

    #[test]
    fn test_text_size_custom_builders() {
        const TITLE_SIZE: FontSizeVar = FontSizeVar::new("--title-size");
        let style = Style::new().text_size_var(TITLE_SIZE).text_size_px(18);
        assert_eq!(style.font_size, Some(FontSize::px(18)));
    }

    #[test]
    fn test_font_weight_class_family_builders() {
        assert_eq!(Style::new().font_thin().font_weight, Some(FontWeight::Thin));
        assert_eq!(
            Style::new().font_extralight().font_weight,
            Some(FontWeight::ExtraLight)
        );
        assert_eq!(
            Style::new().font_light().font_weight,
            Some(FontWeight::Light)
        );
        assert_eq!(
            Style::new().font_normal().font_weight,
            Some(FontWeight::Normal)
        );
        assert_eq!(
            Style::new().font_medium().font_weight,
            Some(FontWeight::Medium)
        );
        assert_eq!(
            Style::new().font_semibold().font_weight,
            Some(FontWeight::SemiBold)
        );
        assert_eq!(Style::new().font_bold().font_weight, Some(FontWeight::Bold));
        assert_eq!(
            Style::new().font_extrabold().font_weight,
            Some(FontWeight::ExtraBold)
        );
        assert_eq!(
            Style::new().font_black().font_weight,
            Some(FontWeight::Black)
        );
    }

    #[test]
    fn test_text_align_class_family_builders() {
        assert_eq!(Style::new().text_left().text_align, Some(TextAlign::Left));
        assert_eq!(
            Style::new().text_center().text_align,
            Some(TextAlign::Center)
        );
        assert_eq!(Style::new().text_right().text_align, Some(TextAlign::Right));
        assert_eq!(
            Style::new().text_justify().text_align,
            Some(TextAlign::Justify)
        );
        assert_eq!(Style::new().text_start().text_align, Some(TextAlign::Start));
        assert_eq!(Style::new().text_end().text_align, Some(TextAlign::End));
    }

    #[test]
    fn test_margin_class_family_builders() {
        let style = Style::new()
            .mx(Spacing::S4)
            .mt(Spacing::S2)
            .mb(Spacing::S6)
            .ms(Spacing::S8);

        let margin = style.margin.expect("margin must be present");
        assert_eq!(margin.left, Some(MarginValue::scale(Spacing::S8)));
        assert_eq!(margin.right, Some(MarginValue::scale(Spacing::S4)));
        assert_eq!(margin.top, Some(MarginValue::scale(Spacing::S2)));
        assert_eq!(margin.bottom, Some(MarginValue::scale(Spacing::S6)));
    }

    #[test]
    fn test_margin_m_overrides_all_sides() {
        let style = Style::new().mx(Spacing::S2).my(Spacing::S3).m(Spacing::S5);
        assert_eq!(style.margin, Some(Margin::all(Spacing::S5)));
    }

    #[test]
    fn test_margin_negative_and_value_builders() {
        let style = Style::new()
            .neg_mt(Spacing::S6)
            .me_value(MarginValue::var(crate::utilities::MarginVar::new(
                "--margin-end",
            )))
            .mbe_value(MarginValue::rem(1.25));

        let margin = style.margin.expect("margin must be present");
        assert_eq!(margin.top, Some(MarginValue::neg_scale(Spacing::S6)));
        assert_eq!(
            margin.right,
            Some(MarginValue::var(crate::utilities::MarginVar::new(
                "--margin-end"
            )))
        );
        assert_eq!(margin.bottom, Some(MarginValue::rem(1.25)));
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
    fn test_align_items_builder_fields() {
        let style = Style::new().align_items(AlignItems::CenterSafe);
        assert_eq!(
            style.flex.and_then(|flex| flex.align),
            Some(AlignItems::CenterSafe)
        );
    }

    #[test]
    fn test_justify_content_builder_fields() {
        let style = Style::new().justify_content(JustifyContent::CenterSafe);
        assert_eq!(
            style.flex.and_then(|flex| flex.justify),
            Some(JustifyContent::CenterSafe)
        );
    }

    #[test]
    fn test_align_items_shortcuts() {
        assert_eq!(
            Style::new().items_start().flex.and_then(|flex| flex.align),
            Some(AlignItems::Start)
        );
        assert_eq!(
            Style::new().items_end().flex.and_then(|flex| flex.align),
            Some(AlignItems::End)
        );
        assert_eq!(
            Style::new()
                .items_end_safe()
                .flex
                .and_then(|flex| flex.align),
            Some(AlignItems::EndSafe)
        );
        assert_eq!(
            Style::new().items_center().flex.and_then(|flex| flex.align),
            Some(AlignItems::Center)
        );
        assert_eq!(
            Style::new()
                .items_center_safe()
                .flex
                .and_then(|flex| flex.align),
            Some(AlignItems::CenterSafe)
        );
        assert_eq!(
            Style::new()
                .items_baseline()
                .flex
                .and_then(|flex| flex.align),
            Some(AlignItems::Baseline)
        );
        assert_eq!(
            Style::new()
                .items_baseline_last()
                .flex
                .and_then(|flex| flex.align),
            Some(AlignItems::BaselineLast)
        );
        assert_eq!(
            Style::new()
                .items_stretch()
                .flex
                .and_then(|flex| flex.align),
            Some(AlignItems::Stretch)
        );
    }

    #[test]
    fn test_gap_axis_builder_fields_for_flex() {
        let style = Style::new().gap_x(Spacing::S6).gap_y(Spacing::S3);
        let flex = style.flex.expect("flex must be initialized");
        assert_eq!(flex.col_gap, Some(Spacing::S6));
        assert_eq!(flex.row_gap, Some(Spacing::S3));
    }

    #[test]
    fn test_gap_builder_fields_for_grid() {
        let style = Style::new()
            .grid(GridContainer::cols_3())
            .gap(Spacing::S4)
            .gap_x(Spacing::S8)
            .gap_y(Spacing::S2);

        let grid = style.grid.expect("grid must be present");
        assert_eq!(grid.gap, Some(Spacing::S4));
        assert_eq!(grid.col_gap, Some(Spacing::S8));
        assert_eq!(grid.row_gap, Some(Spacing::S2));
    }

    #[test]
    fn test_grid_template_columns_builder_fields() {
        let style = Style::new()
            .grid_cols_count(4)
            .grid_cols_subgrid()
            .grid_cols_none()
            .grid_cols_custom_property("--layout-cols")
            .grid_cols_arbitrary("200px_minmax(0,_1fr)_100px");

        let grid = style.grid.expect("grid must be initialized");
        assert_eq!(
            grid.columns,
            Some(GridTemplate::Arbitrary(
                "200px minmax(0, 1fr) 100px".to_string()
            ))
        );
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

    #[test]
    fn test_merge_preserves_partial_padding_and_margin() {
        let merged = Style::new()
            .px(Spacing::S4)
            .mx(Spacing::S3)
            .merge(Style::new().pt(Spacing::S2).mb(Spacing::S6));

        assert_eq!(
            merged.padding,
            Some(Padding {
                top: Some(PaddingValue::scale(Spacing::S2)),
                right: Some(PaddingValue::scale(Spacing::S4)),
                bottom: None,
                left: Some(PaddingValue::scale(Spacing::S4)),
            })
        );
        assert_eq!(
            merged.margin,
            Some(Margin {
                top: None,
                right: Some(MarginValue::scale(Spacing::S3)),
                bottom: Some(MarginValue::scale(Spacing::S6)),
                left: Some(MarginValue::scale(Spacing::S3)),
            })
        );
    }

    #[test]
    fn test_merge_preserves_partial_flex_and_constraints() {
        let merged = Style::new()
            .flex(FlexContainer::row().justify(JustifyContent::Between))
            .constraints(
                SizeConstraints::new()
                    .min_width(Size::Spacing(Spacing::S8))
                    .max_width(Size::Prose),
            )
            .merge(
                Style::new()
                    .align_items(AlignItems::Center)
                    .constraints(SizeConstraints::new().max_height(Size::ScreenHeight)),
            );

        assert_eq!(
            merged.flex,
            Some(FlexContainer {
                direction: Some(FlexDirection::Row),
                wrap: None,
                justify: Some(JustifyContent::Between),
                align: Some(AlignItems::Center),
                gap: None,
                row_gap: None,
                col_gap: None,
            })
        );
        assert_eq!(
            merged.constraints,
            Some(SizeConstraints {
                min_width: Some(Size::Spacing(Spacing::S8)),
                max_width: Some(Size::Prose),
                min_height: None,
                max_height: Some(Size::ScreenHeight),
            })
        );
    }
}
