use iced::widget::{column, row, text, Space};
use iced::{Element, Length};
use crate::components::Snippet;
use crate::Message;
use twill::tokens::{SemanticColor, SemanticThemeVars, FontWeight, BorderRadius};
use twill::iced::{to_color, to_font_weight, styled_container};
use twill::Style;

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
    let mut palette_grid = column![].spacing(16);
    let mut current_row = row![].spacing(8);
    let mut count = 0;

    for (semantic, name) in semantics {
        let light_color = theme.resolve(semantic, false).unwrap();
        let dark_color = theme.resolve(semantic, true).unwrap();

        let block = column![
            text(name).size(14).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
            Space::new().width(Length::Shrink).height(Length::Fixed(4.0)),
            row![
                styled_container(
                    column![text("Light").size(10).color(if is_dark { to_color(twill::tokens::Color::black()) } else { to_color(twill::tokens::Color::white()) })].into(),
                    &Style::new().bg(light_color).rounded(BorderRadius::Md)
                )
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(48.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
                
                styled_container(
                    column![text("Dark").size(10).color(if is_dark { to_color(twill::tokens::Color::black()) } else { to_color(twill::tokens::Color::white()) })].into(),
                    &Style::new().bg(dark_color).rounded(BorderRadius::Md)
                )
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(48.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            ]
        ].spacing(4);

        current_row = current_row.push(block);
        count += 1;

        if count == 3 {
            palette_grid = palette_grid.push(current_row);
            current_row = row![].spacing(8);
            count = 0;
        }
    }

    if count > 0 {
        palette_grid = palette_grid.push(current_row);
    }

    let snippet = Snippet::new("Semantic Color Map", code, palette_grid);

    column![
        text("Semantic Colors").size(32).font(iced::Font { weight: to_font_weight(FontWeight::Bold), ..Default::default() }),
        text("Showing both Light and Dark mode equivalents of shadcn semantic tokens.").size(16),
        snippet.view(is_dark),
    ]
    .spacing(32)
    .into()
}
