use crate::Message;
use crate::components::Snippet;
use iced::widget::{Space, column, row, text};
use iced::{Element, Length};
use twill::Style;
use twill::iced::{styled_container, to_color, to_font_weight};
use twill::tokens::{BorderRadius, FontWeight, SemanticColor, SemanticThemeVars};
use twill::traits::ComputeValue;

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// Resolving semantic tokens based on current theme
let light_color = theme.resolve(semantic, false).unwrap();

styled_container(
    text("Light").into(),
    &twill::Style::new().bg(light_color).rounded(BorderRadius::Md)
)
.width(Length::Fixed(80.0))
.height(Length::Fixed(48.0))"#;

    let semantics = [
        (SemanticColor::Background, "Background"),
        (SemanticColor::Foreground, "Foreground"),
        (SemanticColor::Card, "Card"),
        (SemanticColor::CardForeground, "CardForeground"),
        (SemanticColor::Popover, "Popover"),
        (SemanticColor::PopoverForeground, "PopoverForeground"),
        (SemanticColor::Primary, "Primary"),
        (SemanticColor::PrimaryForeground, "PrimaryForeground"),
        (SemanticColor::Secondary, "Secondary"),
        (SemanticColor::SecondaryForeground, "SecondaryForeground"),
        (SemanticColor::Muted, "Muted"),
        (SemanticColor::MutedForeground, "MutedForeground"),
        (SemanticColor::Accent, "Accent"),
        (SemanticColor::AccentForeground, "AccentForeground"),
        (SemanticColor::Destructive, "Destructive"),
        (SemanticColor::Border, "Border"),
        (SemanticColor::Input, "Input"),
        (SemanticColor::Ring, "Ring"),
        (SemanticColor::Sidebar, "Sidebar"),
        (SemanticColor::SidebarForeground, "SidebarForeground"),
        (SemanticColor::SidebarBorder, "SidebarBorder"),
    ];

    let theme = SemanticThemeVars::shadcn_neutral();
    let mut palette_grid = column![].spacing(12);
    let mut current_row = row![].spacing(12);
    let mut count = 0;

    for (semantic, name) in semantics {
        let light_color = theme.resolve(semantic, false).unwrap();
        let dark_color = theme.resolve(semantic, true).unwrap();

        let block = column![
            text(name).size(14).font(iced::Font {
                weight: to_font_weight(FontWeight::Bold),
                ..Default::default()
            }),
            Space::new()
                .width(Length::Shrink)
                .height(Length::Fixed(4.0)),
            row![
                styled_container(
                    text("Light")
                        .size(10)
                        .color(contrast_color(light_color))
                        .into(),
                    &Style::new().bg(light_color).rounded(BorderRadius::Md)
                )
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(48.0))
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
                styled_container(
                    text("Dark")
                        .size(10)
                        .color(contrast_color(dark_color))
                        .into(),
                    &Style::new().bg(dark_color).rounded(BorderRadius::Md)
                )
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(48.0))
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
            ]
        ]
        .spacing(4);

        current_row = current_row.push(block);
        count += 1;

        if count == 2 {
            palette_grid = palette_grid.push(current_row);
            current_row = row![].spacing(12);
            count = 0;
        }
    }

    if count > 0 {
        palette_grid = palette_grid.push(current_row);
    }

    let snippet = Snippet::new("Semantic Color Map", code, palette_grid);

    column![
        text("Semantic Colors").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Showing both Light and Dark mode equivalents of shadcn semantic tokens.").size(16),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}

fn contrast_color(color: twill::tokens::Color) -> iced::Color {
    let raw = color.compute();
    let luminance = (0.299 * raw.r as f32) + (0.587 * raw.g as f32) + (0.114 * raw.b as f32);
    if luminance > 150.0 {
        to_color(twill::tokens::Color::black())
    } else {
        to_color(twill::tokens::Color::white())
    }
}
