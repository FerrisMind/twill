use iced::widget::{column, text};
use iced::Element;
use crate::Message;
use twill::iced::to_font_weight;
use twill::tokens::FontWeight;

pub fn view<'a>(_is_dark: bool, _elapsed: f64) -> Element<'a, Message> {
    column![
        text("Motion").size(32).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        text("Motion integration for iced is currently under development. Please check back later!").size(16),
    ]
    .spacing(32)
    .into()
}
