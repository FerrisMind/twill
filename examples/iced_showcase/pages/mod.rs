mod typography;
mod colors;
mod semantic_colors;
mod spacing;
mod borders;
mod shadows;
mod buttons;
mod kitchen_sink;
mod motion;
mod style_builder;
pub mod real_world;

use iced::Element;
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Typography,
    Colors,
    SemanticColors,
    Spacing,
    Borders,
    Shadows,
    Buttons,
    Motion,
    StyleBuilder,
    KitchenSink,
    Translator,
}



impl Page {
    pub fn title(&self) -> &'static str {
        match self {
            Page::Typography => "Typography",
            Page::Colors => "Colors (Tailwind)",
            Page::SemanticColors => "Colors (Semantic)",
            Page::Spacing => "Spacing",
            Page::Borders => "Borders",
            Page::Shadows => "Shadows",
            Page::Buttons => "Buttons",
            Page::Motion => "Motion & Animation",
            Page::StyleBuilder => "Style Builder",
            Page::KitchenSink => "Kitchen Sink",
            Page::Translator => "Real World: Translator",
        }
    }

    pub fn view<'a>(&self, is_dark: bool, elapsed: f64) -> Element<'a, Message> {
        match self {
            Page::Typography => typography::view(is_dark),
            Page::Colors => colors::view(is_dark),
            Page::SemanticColors => semantic_colors::view(is_dark),
            Page::Spacing => spacing::view(is_dark),
            Page::Borders => borders::view(is_dark),
            Page::Shadows => shadows::view(is_dark),
            Page::Buttons => buttons::view(is_dark),
            Page::Motion => motion::view(is_dark, elapsed),
            Page::StyleBuilder => style_builder::view(is_dark),
            Page::KitchenSink => kitchen_sink::view(is_dark),
            Page::Translator => real_world::translator::view(is_dark),
        }
    }
}
