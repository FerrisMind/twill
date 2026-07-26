#[cfg(feature = "iced")]
mod common;

#[cfg(feature = "iced")]
	use common::{
    composition_style_for, interactive_style, responsive_style_for, semantic_demo_chip,
    semantic_summary, themed_surface_style, token_palette,
};
#[cfg(feature = "iced")]
use iced::widget::{button, column, container, row, scrollable, text};
#[cfg(feature = "iced")]
use iced::{Element, Fill, Theme};
#[cfg(feature = "iced")]
use twill::backends::iced::{self as twill_iced, styled_container};
#[cfg(feature = "iced")]
use twill::prelude::{core::*, theme::*};

#[cfg(feature = "iced")]
fn main() -> iced::Result {
    iced::application(ShowcaseApp::default, ShowcaseApp::update, ShowcaseApp::view)
        .title(ShowcaseApp::title)
        .theme(ShowcaseApp::theme)
        .run()
}

#[cfg(feature = "iced")]
#[derive(Default)]
struct ShowcaseApp {
    dark_mode: bool,
}

#[cfg(feature = "iced")]
#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleTheme,
}

#[cfg(feature = "iced")]
impl ShowcaseApp {
    fn title(&self) -> String {
        String::from("twill showcase (iced)")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
            }
        }
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::TokyoNight
        } else {
            Theme::Light
        }
    }

    fn text_color(style: &Style, fallback: iced::Color) -> iced::Color {
        style
            .text_color_value()
            .map(twill_iced::to_color)
            .unwrap_or(fallback)
    }

    fn default_body_color(&self) -> iced::Color {
        if self.dark_mode {
            twill_iced::to_color(Color::slate(Scale::S100))
        } else {
            twill_iced::to_color(Color::slate(Scale::S900))
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let variant = if self.dark_mode {
            ThemeVariant::Dark
        } else {
            ThemeVariant::Light
        };
        let surface = themed_surface_style(variant);

        let topbar = row![
            text("Twill 0.3.x layered examples").size(28),
            button("Toggle semantic theme").on_press(Message::ToggleTheme),
        ]
        .spacing(16);

        let tokens = row(token_palette()
            .into_iter()
            .map(|(color, label)| {
                let style = Style::new()
                    .bg(color)
                    .text_color(Color::white())
                    .padding(Padding::all(Spacing::S3))
                    .rounded(BorderRadius::Lg);
                styled_container(
                    text(label)
                        .color(Self::text_color(
                            &style,
                            twill_iced::to_color(Color::white()),
                        ))
                        .into(),
                    &style,
                )
                .into()
            })
            .collect::<Vec<Element<'_, Message>>>())
        .spacing(12);

        let surface_ink = Self::text_color(&surface, self.default_body_color());
        let radius_card: Element<'_, Message> = styled_container::<Message>(
            column![
                text("Radius").size(18).color(surface_ink),
                text("Rounded XL surface").color(surface_ink)
            ]
            .spacing(6)
            .into(),
            &surface,
        )
        .into();

        let shadow_preview = surface.clone().shadow(Shadow::Lg);
        let shadow_ink = Self::text_color(&shadow_preview, self.default_body_color());
        let shadow_card: Element<'_, Message> = styled_container::<Message>(
            column![
                text("Shadow").size(18).color(shadow_ink),
                text("Large shadow token").color(shadow_ink)
            ]
            .spacing(6)
            .into(),
            &shadow_preview,
        )
        .into();

        let spacing_preview = surface.clone().padding(Padding::all(Spacing::S6));
        let spacing_ink = Self::text_color(&spacing_preview, self.default_body_color());
        let spacing_card: Element<'_, Message> = styled_container::<Message>(
            column![
                text("Spacing").size(18).color(spacing_ink),
                text("Padding S6 preview").color(spacing_ink)
            ]
            .spacing(6)
            .into(),
            &spacing_preview,
        )
        .into();

        let token_details = row![radius_card, shadow_card, spacing_card].spacing(12);
        let semantic_surface = Style::new()
            .bg(SemanticThemeVars::shadcn_neutral()
                .resolve(SemanticColor::Background, variant)
                .unwrap_or(Color::white()))
            .text_color(
                SemanticThemeVars::shadcn_neutral()
                    .resolve(SemanticColor::Foreground, variant)
                    .unwrap_or(Color::slate(Scale::S900)),
            )
            .padding(Padding::all(Spacing::S4))
            .rounded(BorderRadius::Xl)
            .border(
                BorderWidth::S1,
                BorderStyle::Solid,
                SemanticThemeVars::shadcn_neutral()
                    .resolve(SemanticColor::Border, variant)
                    .unwrap_or(Color::slate(Scale::S300)),
            );
        let semantic_surface_text =
            Self::text_color(&semantic_surface, self.default_body_color());
        let semantic_swatches = column(
            semantic_summary()
                .into_iter()
                .map(|(label, token)| {
                    // Foreground / Primary Foreground are ink colors: show them on their
                    // paired surface so they stay opposite the theme fill and remain readable.
                    let (surface_token, ink_token) = semantic_demo_chip(token, variant);
                    let chip_style = Style::new()
                        .bg(SemanticThemeVars::shadcn_neutral()
                            .resolve(surface_token, variant)
                            .unwrap_or(Color::white()))
                        .text_color(
                            SemanticThemeVars::shadcn_neutral()
                                .resolve(ink_token, variant)
                                .unwrap_or(Color::slate(Scale::S900)),
                        )
                        .padding(Padding::symmetric(Spacing::S2, Spacing::S3))
                        .rounded(BorderRadius::Md)
                        .border(
                            BorderWidth::S1,
                            BorderStyle::Solid,
                            SemanticThemeVars::shadcn_neutral()
                                .resolve(SemanticColor::Border, variant)
                                .unwrap_or(Color::slate(Scale::S300)),
                        );
                    let ink = twill_iced::to_semantic_color(ink_token, variant);
                    styled_container(text(label).color(ink).into(), &chip_style).into()
                })
                .collect::<Vec<Element<'_, Message>>>(),
        )
        .spacing(8);
        let semantic_column: Element<'_, Message> = styled_container::<Message>(
            column![
                text("Light/dark semantic aliases")
                    .size(18)
                    .color(semantic_surface_text),
                text("These values switch with the semantic theme toggle.")
                    .color(semantic_surface_text),
                semantic_swatches,
            ]
            .spacing(8)
            .into(),
            &semantic_surface,
        )
        .into();

        let composition: Element<'_, Message> = {
            let style = composition_style_for(variant);
            let text_color = Self::text_color(&style, self.default_body_color());
            styled_container::<Message>(
                column![
                    text("Base style").size(22).color(text_color),
                    text("Reusable surface built from one Style.").color(text_color),
                    text("Padding, border, text color, radius, and shadow come from core API.")
                        .color(text_color),
                ]
                .spacing(8)
                .into(),
                &style,
            )
            .into()
        };

        let states: Element<'_, Message> = {
            let style = interactive_style();
            let text_color = Self::text_color(&style, self.default_body_color());
            styled_container::<Message>(
                column![
                    text("Interactive states").size(22).color(text_color),
                    text("Hover, focus-visible, disabled, data-state, and aria-state live next to the base style.")
                        .color(text_color),
                    text("Open state adds a larger shadow; focus-visible adds a ring.")
                        .color(text_color)
                ]
                .spacing(8)
                .into(),
                &style,
            )
            .into()
        };

        let semantic_card: Element<'_, Message> = {
            let text_color = Self::text_color(&semantic_surface, self.default_body_color());
            styled_container::<Message>(
                column![
                    text("Semantic aliases").size(22).color(text_color),
                    text("This card itself uses semantic Background / Foreground / Border colors.")
                        .color(text_color),
                    text("The toggle at the top switches the entire semantic surface.")
                        .color(text_color)
                ]
                .spacing(8)
                .into(),
                &semantic_surface,
            )
            .into()
        };

        let responsive: Element<'_, Message> = {
            let style = responsive_style_for(variant);
            let text_color = Self::text_color(&style, self.default_body_color());
            let previews = column(
                [
                    Breakpoint::Sm,
                    Breakpoint::Md,
                    Breakpoint::Lg,
                    Breakpoint::S2xl,
                ]
                .into_iter()
                .map(|breakpoint| {
                    let resolved = style.at_breakpoint(breakpoint);
                    let resolved_text = Self::text_color(&resolved, text_color);
                    styled_container::<Message>(
                        column![
                            text(format!("{breakpoint:?}"))
                                .size(18)
                                .color(resolved_text),
                            text("Resolved card preview").color(resolved_text),
                            text(format!(
                                "width={:?}, padding={:?}, shadow={:?}",
                                resolved.width_value(),
                                resolved.padding_value(),
                                resolved.box_shadow_value()
                            ))
                            .size(14)
                            .color(resolved_text)
                        ]
                        .spacing(6)
                        .into(),
                        &resolved,
                    )
                    .into()
                })
                .collect::<Vec<Element<'_, Message>>>(),
            )
            .spacing(8);
            let body = column![
                text("Responsive resolved preview")
                    .size(22)
                    .color(text_color),
                text("The same Style resolves into distinct cards at each breakpoint.")
                    .color(text_color),
                previews
            ]
            .spacing(8);
            styled_container::<Message>(body.into(), &style).into()
        };

        // Embed the scrollbar with spacing so it does not overlay example cards.
        container(
            scrollable(
                column![
                    topbar,
                    text("Tokens"),
                    tokens,
                    token_details,
                    text("Semantic theme"),
                    semantic_column,
                    text("Composed sections"),
                    composition,
                    states,
                    semantic_card,
                    responsive
                ]
                .spacing(16),
            )
            .spacing(16),
        )
        .padding(24)
        .width(Fill)
        .into()
    }
}

#[cfg(not(feature = "iced"))]
fn main() {}
