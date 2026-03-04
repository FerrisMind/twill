mod background_color;
mod borders;
mod buttons;
pub mod colors;
mod font_weight;
mod kitchen_sink;
pub mod layout;
mod layout_align_items;
mod layout_aspect_ratio;
mod layout_display;
mod layout_flex;
mod layout_flex_direction;
mod layout_gap;
mod layout_grid_template_columns;
mod layout_height;
mod layout_justify_content;
mod layout_max_width;
mod layout_object_fit;
mod layout_overflow;
mod layout_width;
mod margin;
pub mod motion;
pub mod oklch_demo;
pub mod real_world;
pub mod semantic_colors;
mod shadows;
mod spacing;
mod style_builder;
mod text_align;
mod typography;

use crate::Message;
use iced::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Typography,
    Colors,
    FontWeight,
    TextAlign,
    SemanticColors,
    BackgroundColor,
    OklchDemo,
    Spacing,
    Margin,
    Borders,
    Shadows,
    AspectRatio,
    Columns,
    Overflow,
    Display,
    GridTemplateColumns,
    FlexDirection,
    Gap,
    JustifyContent,
    AlignItems,
    Flex,
    ObjectFit,
    Width,
    Height,
    MaxWidth,
    Buttons,
    Motion,
    StyleBuilder,
    KitchenSink,
    Translator,
}

impl Page {
    pub fn slug(&self) -> &'static str {
        match self {
            Page::Typography => "typography",
            Page::Colors => "colors",
            Page::FontWeight => "font-weight",
            Page::TextAlign => "text-align",
            Page::SemanticColors => "colors-semantic",
            Page::BackgroundColor => "background-color",
            Page::OklchDemo => "oklch-demonstration",
            Page::Spacing => "padding",
            Page::Margin => "margin",
            Page::Borders => "borders",
            Page::Shadows => "shadows",
            Page::AspectRatio => "aspect-ratio",
            Page::Columns => "columns",
            Page::Overflow => "overflow",
            Page::Display => "display",
            Page::GridTemplateColumns => "grid-template-columns",
            Page::FlexDirection => "flex-direction",
            Page::Gap => "gap",
            Page::JustifyContent => "justify-content",
            Page::AlignItems => "align-items",
            Page::Flex => "flex",
            Page::ObjectFit => "object-fit",
            Page::Width => "width",
            Page::Height => "height",
            Page::MaxWidth => "max-width",
            Page::Buttons => "buttons",
            Page::Motion => "motion-animation",
            Page::StyleBuilder => "style-builder",
            Page::KitchenSink => "kitchen-sink",
            Page::Translator => "real-world-translator",
        }
    }

    pub fn docs_title(&self) -> &'static str {
        match self {
            Page::Typography => "Typography",
            Page::Colors => "Colors (Palette)",
            Page::FontWeight => "font-weight",
            Page::TextAlign => "text-align",
            Page::SemanticColors => "Colors (Semantic)",
            Page::BackgroundColor => "Background Color",
            Page::OklchDemo => "OKLCH Demonstration",
            Page::Spacing => "padding",
            Page::Margin => "margin",
            Page::Borders => "Borders",
            Page::Shadows => "Shadows",
            Page::AspectRatio => "aspect-ratio",
            Page::Columns => "columns",
            Page::Overflow => "overflow",
            Page::Display => "display",
            Page::GridTemplateColumns => "grid-template-columns",
            Page::FlexDirection => "flex-direction",
            Page::Gap => "gap",
            Page::JustifyContent => "justify-content",
            Page::AlignItems => "align-items",
            Page::Flex => "flex",
            Page::ObjectFit => "object-fit",
            Page::Width => "width",
            Page::Height => "height",
            Page::MaxWidth => "max-width",
            Page::Buttons => "Buttons",
            Page::Motion => "Motion & Animation",
            Page::StyleBuilder => "Style Builder",
            Page::KitchenSink => "Kitchen Sink",
            Page::Translator => "Real World: Translator",
        }
    }

    pub fn title(&self) -> &'static str {
        self.docs_title()
    }

    pub fn view<'a>(
        &self,
        is_dark: bool,
        elapsed: f64,
        columns_preview_width: f32,
        aspect_preview_width: f32,
    ) -> Element<'a, Message> {
        match self {
            Page::Typography => typography::view(is_dark),
            Page::Colors => colors::view(is_dark),
            Page::FontWeight => font_weight::view(is_dark),
            Page::TextAlign => text_align::view(is_dark),
            Page::SemanticColors => semantic_colors::view(is_dark),
            Page::BackgroundColor => background_color::view(is_dark),
            Page::OklchDemo => oklch_demo::view(is_dark),
            Page::Spacing => spacing::view(is_dark),
            Page::Margin => margin::view(is_dark),
            Page::Borders => borders::view(is_dark),
            Page::Shadows => shadows::view(is_dark),
            Page::AspectRatio => layout_aspect_ratio::view(is_dark, aspect_preview_width),
            Page::Columns => layout::view(is_dark, columns_preview_width),
            Page::Overflow => layout_overflow::view(is_dark),
            Page::Display => layout_display::view(is_dark),
            Page::GridTemplateColumns => layout_grid_template_columns::view(is_dark),
            Page::FlexDirection => layout_flex_direction::view(is_dark),
            Page::Gap => layout_gap::view(is_dark),
            Page::JustifyContent => layout_justify_content::view(is_dark),
            Page::AlignItems => layout_align_items::view(is_dark),
            Page::Flex => layout_flex::view(is_dark),
            Page::ObjectFit => layout_object_fit::view(is_dark),
            Page::Width => layout_width::view(is_dark),
            Page::Height => layout_height::view(is_dark),
            Page::MaxWidth => layout_max_width::view(is_dark),
            Page::Buttons => buttons::view(is_dark),
            Page::Motion => motion::view(is_dark, elapsed),
            Page::StyleBuilder => style_builder::view(is_dark),
            Page::KitchenSink => kitchen_sink::view(is_dark),
            Page::Translator => real_world::translator::view(is_dark),
        }
    }
}
