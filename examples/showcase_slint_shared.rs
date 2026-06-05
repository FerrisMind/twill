#[cfg(feature = "slint")]
use crate::common::{
    composition_style, interactive_style, responsive_style, surface_style, token_palette,
};
#[cfg(feature = "slint")]
use slint::{Color as SlintColor, SharedString};
#[cfg(feature = "slint")]
use twill::backends::slint::{self as twill_slint, ToSlint};
#[cfg(feature = "slint")]
use twill::prelude::{core::*, theme::*};
#[cfg(feature = "slint")]
use twill::tokens::{BackgroundColor, BorderColor, ColorValue, ShadowColorToken, TextColor};
#[cfg(feature = "slint")]
use twill::traits::ComputeValue;
#[cfg(feature = "slint")]
use twill::utilities::{Padding, PaddingValue};

#[cfg(feature = "slint")]
slint::slint! {
    import { HorizontalBox, ScrollView, VerticalBox } from "std-widgets.slint";

    component ActionButton inherits Rectangle {
        in property <string> label;
        in property <color> bg;
        in property <color> fg;
        callback clicked();

        min-width: 220px;
        height: 40px;
        border-radius: 10px;
        background: bg;

        Text {
            text: root.label;
            color: root.fg;
            font-size: 16px;
            font-weight: 600;
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
        }

        touch := TouchArea {
            clicked => { root.clicked(); }
        }
    }

    component Swatch inherits Rectangle {
        in property <string> label;
        in property <color> bg;
        in property <color> fg;

        min-width: 108px;
        height: 44px;
        border-radius: 10px;
        background: bg;

        Text {
            text: root.label;
            color: root.fg;
            font-size: 18px;
            font-weight: 600;
            x: 12px;
            y: (parent.height - self.height) / 2;
        }
    }

    component PreviewCard inherits Rectangle {
        in property <string> title;
        in property <string> line_1;
        in property <string> line_2;
        in property <color> bg;
        in property <color> fg;
        in property <color> border;
        in property <color> shadow_color;
        in property <length> radius;
        in property <length> shadow_blur;
        in property <length> shadow_offset_y;
        in property <length> padding_size;

        min-width: 300px;
        min-height: 112px;
        background: bg;
        border-color: border;
        border-width: 1px;
        border-radius: radius;
        drop-shadow-blur: shadow_blur;
        drop-shadow-offset-y: shadow_offset_y;
        drop-shadow-color: shadow_color;

        VerticalBox {
            padding: root.padding_size;
            spacing: 6px;

            Text {
                text: root.title;
                color: root.fg;
                font-size: 18px;
                font-weight: 700;
            }

            Text {
                text: root.line_1;
                color: root.fg;
                wrap: word-wrap;
            }

            Text {
                text: root.line_2;
                color: root.fg;
                wrap: word-wrap;
                visible: root.line_2 != "";
            }
        }
    }

    component SemanticRow inherits Rectangle {
        in property <string> label;
        in property <string> value;
        in property <color> sample;
        in property <color> bg;
        in property <color> fg;
        in property <color> border;

        min-height: 36px;
        background: bg;
        border-color: border;
        border-width: 1px;
        border-radius: 10px;

        HorizontalBox {
            padding: 10px;
            spacing: 10px;

            Rectangle {
                width: 14px;
                height: 14px;
                border-radius: 7px;
                background: root.sample;
                y: 2px;
            }

            Text {
                text: root.label + "  " + root.value;
                color: root.fg;
                wrap: word-wrap;
            }
        }
    }

    component ResponsiveCard inherits Rectangle {
        in property <color> bg;
        in property <color> fg;
        in property <color> border;
        in property <color> shadow_color;
        in property <length> radius;
        in property <length> shadow_blur;
        in property <length> shadow_offset_y;
        in property <length> padding_size;
        in property <string> line_sm;
        in property <string> line_md;
        in property <string> line_lg;
        in property <string> line_2xl;

        min-height: 220px;
        background: bg;
        border-color: border;
        border-width: 1px;
        border-radius: radius;
        drop-shadow-blur: shadow_blur;
        drop-shadow-offset-y: shadow_offset_y;
        drop-shadow-color: shadow_color;

        VerticalBox {
            padding: root.padding_size;
            spacing: 8px;

            Text {
                text: "Responsive resolved preview";
                color: root.fg;
                font-size: 20px;
                font-weight: 700;
            }

            Text {
                text: "The same Style resolves into distinct cards at each breakpoint.";
                color: root.fg;
                wrap: word-wrap;
            }

            SemanticRow {
                label: "Sm";
                value: root.line_sm;
                sample: root.fg;
                bg: root.bg;
                fg: root.fg;
                border: root.border;
            }

            SemanticRow {
                label: "Md";
                value: root.line_md;
                sample: root.fg;
                bg: root.bg;
                fg: root.fg;
                border: root.border;
            }

            SemanticRow {
                label: "Lg";
                value: root.line_lg;
                sample: root.fg;
                bg: root.bg;
                fg: root.fg;
                border: root.border;
            }

            SemanticRow {
                label: "2xl";
                value: root.line_2xl;
                sample: root.fg;
                bg: root.bg;
                fg: root.fg;
                border: root.border;
            }
        }
    }

    export component ShowcaseWindow inherits Window {
        in-out property <bool> dark_mode: false;
        in property <string> window_title;
        in property <string> runtime_label;
        in property <color> page_bg;
        in property <color> heading_fg;
        in property <color> muted_fg;
        in property <color> accent_bg;
        in property <color> accent_fg;
        in property <color> token_brand;
        in property <color> token_success;
        in property <color> token_warning;
        in property <color> token_accent;
        in property <color> surface_bg;
        in property <color> surface_fg;
        in property <color> surface_border;
        in property <color> surface_shadow_color;
        in property <length> surface_radius;
        in property <length> surface_shadow_blur;
        in property <length> surface_shadow_offset;
        in property <length> surface_padding;
        in property <length> spacing_preview_padding;
        in property <color> large_shadow_color;
        in property <length> large_shadow_blur;
        in property <length> large_shadow_offset;
        in property <color> composition_bg;
        in property <color> composition_fg;
        in property <color> composition_border;
        in property <color> composition_shadow_color;
        in property <length> composition_radius;
        in property <length> composition_shadow_blur;
        in property <length> composition_shadow_offset;
        in property <length> composition_padding;
        in property <color> interactive_bg;
        in property <color> interactive_fg;
        in property <color> interactive_border;
        in property <color> interactive_shadow_color;
        in property <length> interactive_radius;
        in property <length> interactive_shadow_blur;
        in property <length> interactive_shadow_offset;
        in property <length> interactive_padding;
        in property <color> semantic_bg;
        in property <color> semantic_fg;
        in property <color> semantic_border;
        in property <color> semantic_shadow_color;
        in property <length> semantic_radius;
        in property <length> semantic_shadow_blur;
        in property <length> semantic_shadow_offset;
        in property <length> semantic_padding;
        in property <color> semantic_sample_background;
        in property <string> semantic_value_background;
        in property <color> semantic_sample_foreground;
        in property <string> semantic_value_foreground;
        in property <color> semantic_sample_primary;
        in property <string> semantic_value_primary;
        in property <color> semantic_sample_primary_foreground;
        in property <string> semantic_value_primary_foreground;
        in property <color> semantic_sample_border;
        in property <string> semantic_value_border;
        in property <color> semantic_sample_ring;
        in property <string> semantic_value_ring;
        in property <color> responsive_bg;
        in property <color> responsive_fg;
        in property <color> responsive_border;
        in property <color> responsive_shadow_color;
        in property <length> responsive_radius;
        in property <length> responsive_shadow_blur;
        in property <length> responsive_shadow_offset;
        in property <length> responsive_padding;
        in property <string> responsive_line_sm;
        in property <string> responsive_line_md;
        in property <string> responsive_line_lg;
        in property <string> responsive_line_2xl;

        callback toggle_theme();

        title: window_title;
        width: 1080px;
        height: 860px;
        background: page_bg;

        ScrollView {
            width: 100%;
            height: 100%;

            VerticalBox {
                width: parent.width - 24px;
                padding: 24px;
                spacing: 16px;

                HorizontalBox {
                    spacing: 16px;

                    VerticalBox {
                        spacing: 4px;

                        Text {
                            text: "Twill 0.3.x layered examples";
                            color: root.heading_fg;
                            font-size: 28px;
                            font-weight: 800;
                        }

                        Text {
                            text: root.runtime_label;
                            color: root.muted_fg;
                        }
                    }

                    Rectangle { min-width: 1px; horizontal-stretch: 1; background: transparent; }

                    ActionButton {
                        label: "Toggle semantic theme";
                        bg: root.accent_bg;
                        fg: root.accent_fg;
                        clicked => { root.toggle_theme(); }
                    }
                }

                Text {
                    text: "Tokens";
                    color: root.heading_fg;
                    font-size: 20px;
                    font-weight: 700;
                }

                HorizontalBox {
                    spacing: 12px;
                    Swatch { label: "Brand"; bg: root.token_brand; fg: white; }
                    Swatch { label: "Success"; bg: root.token_success; fg: white; }
                    Swatch { label: "Warning"; bg: root.token_warning; fg: white; }
                    Swatch { label: "Accent"; bg: root.token_accent; fg: white; }
                }

                HorizontalBox {
                    spacing: 12px;

                    PreviewCard {
                        title: "Radius";
                        line_1: "Rounded XL surface";
                        line_2: "";
                        bg: root.surface_bg;
                        fg: root.surface_fg;
                        border: root.surface_border;
                        shadow_color: root.surface_shadow_color;
                        radius: root.surface_radius;
                        shadow_blur: root.surface_shadow_blur;
                        shadow_offset_y: root.surface_shadow_offset;
                        padding_size: root.surface_padding;
                    }

                    PreviewCard {
                        title: "Shadow";
                        line_1: "Large shadow token";
                        line_2: "";
                        bg: root.surface_bg;
                        fg: root.surface_fg;
                        border: root.surface_border;
                        shadow_color: root.large_shadow_color;
                        radius: root.surface_radius;
                        shadow_blur: root.large_shadow_blur;
                        shadow_offset_y: root.large_shadow_offset;
                        padding_size: root.surface_padding;
                    }

                    PreviewCard {
                        title: "Spacing";
                        line_1: "Padding S6 preview";
                        line_2: "";
                        bg: root.surface_bg;
                        fg: root.surface_fg;
                        border: root.surface_border;
                        shadow_color: root.surface_shadow_color;
                        radius: root.surface_radius;
                        shadow_blur: root.surface_shadow_blur;
                        shadow_offset_y: root.surface_shadow_offset;
                        padding_size: root.spacing_preview_padding;
                    }
                }

                Text {
                    text: "Semantic theme";
                    color: root.heading_fg;
                    font-size: 20px;
                    font-weight: 700;
                }

                VerticalBox {
                    spacing: 8px;

                    SemanticRow {
                        label: "Background";
                        value: root.semantic_value_background;
                        sample: root.semantic_sample_background;
                        bg: root.semantic_bg;
                        fg: root.semantic_fg;
                        border: root.semantic_border;
                    }

                    SemanticRow {
                        label: "Foreground";
                        value: root.semantic_value_foreground;
                        sample: root.semantic_sample_foreground;
                        bg: root.semantic_bg;
                        fg: root.semantic_fg;
                        border: root.semantic_border;
                    }

                    SemanticRow {
                        label: "Primary";
                        value: root.semantic_value_primary;
                        sample: root.semantic_sample_primary;
                        bg: root.semantic_bg;
                        fg: root.semantic_fg;
                        border: root.semantic_border;
                    }

                    SemanticRow {
                        label: "Primary Foreground";
                        value: root.semantic_value_primary_foreground;
                        sample: root.semantic_sample_primary_foreground;
                        bg: root.semantic_bg;
                        fg: root.semantic_fg;
                        border: root.semantic_border;
                    }

                    SemanticRow {
                        label: "Border";
                        value: root.semantic_value_border;
                        sample: root.semantic_sample_border;
                        bg: root.semantic_bg;
                        fg: root.semantic_fg;
                        border: root.semantic_border;
                    }

                    SemanticRow {
                        label: "Ring";
                        value: root.semantic_value_ring;
                        sample: root.semantic_sample_ring;
                        bg: root.semantic_bg;
                        fg: root.semantic_fg;
                        border: root.semantic_border;
                    }
                }

                Text {
                    text: "Composed sections";
                    color: root.heading_fg;
                    font-size: 20px;
                    font-weight: 700;
                }

                PreviewCard {
                    title: "Base style";
                    line_1: "Reusable surface built from one Style.";
                    line_2: "Padding, border, text color, radius, and shadow come from core API.";
                    bg: root.composition_bg;
                    fg: root.composition_fg;
                    border: root.composition_border;
                    shadow_color: root.composition_shadow_color;
                    radius: root.composition_radius;
                    shadow_blur: root.composition_shadow_blur;
                    shadow_offset_y: root.composition_shadow_offset;
                    padding_size: root.composition_padding;
                }

                PreviewCard {
                    title: "Interactive states";
                    line_1: "Hover, focus-visible, disabled, data-state, and aria-state live next to the base style.";
                    line_2: "Open state adds a larger shadow; focus-visible adds a ring.";
                    bg: root.interactive_bg;
                    fg: root.interactive_fg;
                    border: root.interactive_border;
                    shadow_color: root.interactive_shadow_color;
                    radius: root.interactive_radius;
                    shadow_blur: root.interactive_shadow_blur;
                    shadow_offset_y: root.interactive_shadow_offset;
                    padding_size: root.interactive_padding;
                }

                PreviewCard {
                    title: "Semantic aliases";
                    line_1: "This card itself uses semantic Background / Foreground / Border colors.";
                    line_2: "The toggle at the top switches the semantic surface.";
                    bg: root.semantic_bg;
                    fg: root.semantic_fg;
                    border: root.semantic_border;
                    shadow_color: root.semantic_shadow_color;
                    radius: root.semantic_radius;
                    shadow_blur: root.semantic_shadow_blur;
                    shadow_offset_y: root.semantic_shadow_offset;
                    padding_size: root.semantic_padding;
                }

                ResponsiveCard {
                    bg: root.responsive_bg;
                    fg: root.responsive_fg;
                    border: root.responsive_border;
                    shadow_color: root.responsive_shadow_color;
                    radius: root.responsive_radius;
                    shadow_blur: root.responsive_shadow_blur;
                    shadow_offset_y: root.responsive_shadow_offset;
                    padding_size: root.responsive_padding;
                    line_sm: root.responsive_line_sm;
                    line_md: root.responsive_line_md;
                    line_lg: root.responsive_line_lg;
                    line_2xl: root.responsive_line_2xl;
                }
            }
        }
    }
}

#[cfg(feature = "slint")]
#[derive(Clone, Copy)]
struct CardVisual {
    bg: SlintColor,
    fg: SlintColor,
    border: SlintColor,
    shadow_color: SlintColor,
    radius: f32,
    shadow_blur: f32,
    shadow_offset_y: f32,
    padding: f32,
}

#[cfg(feature = "slint")]
fn transparent() -> SlintColor {
    SlintColor::from_argb_u8(0, 0, 0, 0)
}

#[cfg(feature = "slint")]
fn to_slint_color_value(value: ColorValue) -> SlintColor {
    twill_slint::to_slint_color_value(value)
}

#[cfg(feature = "slint")]
fn theme_color(
    theme: &SemanticThemeVars,
    semantic: SemanticColor,
    variant: ThemeVariant,
    fallback: Color,
) -> Color {
    theme.resolve(semantic, variant).unwrap_or(fallback)
}

#[cfg(feature = "slint")]
fn resolve_background(
    token: BackgroundColor,
    theme: &SemanticThemeVars,
    variant: ThemeVariant,
) -> Option<SlintColor> {
    match token {
        BackgroundColor::Transparent => Some(transparent()),
        BackgroundColor::Palette(color) => Some(color.to_slint()),
        BackgroundColor::Semantic(color) => theme.resolve(color, variant).map(ToSlint::to_slint),
        BackgroundColor::Arbitrary(value) => Some(to_slint_color_value(ColorValue::from(value))),
        BackgroundColor::Inherit
        | BackgroundColor::Current
        | BackgroundColor::CustomProperty(_) => None,
    }
}

#[cfg(feature = "slint")]
fn resolve_text(
    token: TextColor,
    theme: &SemanticThemeVars,
    variant: ThemeVariant,
) -> Option<SlintColor> {
    match token {
        TextColor::Transparent => Some(transparent()),
        TextColor::Palette(color) => Some(color.to_slint()),
        TextColor::Semantic(color) => theme.resolve(color, variant).map(ToSlint::to_slint),
        TextColor::Arbitrary(value) => Some(to_slint_color_value(ColorValue::from(value))),
        TextColor::Inherit | TextColor::Current | TextColor::CustomProperty(_) => None,
    }
}

#[cfg(feature = "slint")]
fn resolve_border(
    token: BorderColor,
    theme: &SemanticThemeVars,
    variant: ThemeVariant,
) -> Option<SlintColor> {
    match token {
        BorderColor::Transparent => Some(transparent()),
        BorderColor::Palette(color) => Some(color.to_slint()),
        BorderColor::Semantic(color) => theme.resolve(color, variant).map(ToSlint::to_slint),
        BorderColor::Arbitrary(value) => Some(to_slint_color_value(ColorValue::from(value))),
        BorderColor::Inherit | BorderColor::Current | BorderColor::CustomProperty(_) => None,
    }
}

#[cfg(feature = "slint")]
fn resolve_shadow_color(
    token: ShadowColorToken,
    theme: &SemanticThemeVars,
    variant: ThemeVariant,
) -> Option<SlintColor> {
    match token {
        ShadowColorToken::Palette(color) => Some(color.to_slint()),
        ShadowColorToken::Semantic(color) => theme.resolve(color, variant).map(ToSlint::to_slint),
        ShadowColorToken::Arbitrary(value) => Some(to_slint_color_value(ColorValue::from(value))),
        ShadowColorToken::Transparent => Some(transparent()),
        ShadowColorToken::Inherit
        | ShadowColorToken::Current
        | ShadowColorToken::CustomProperty(_) => None,
    }
}

#[cfg(feature = "slint")]
fn padding_value_to_px(value: PaddingValue) -> Option<f32> {
    match value {
        PaddingValue::Scale(spacing) => Some(spacing.to_slint()),
        PaddingValue::Px(px) => Some(px.max(0.0)),
        PaddingValue::Rem(rem) => Some((rem * 16.0).max(0.0)),
        PaddingValue::Var(_) => None,
    }
}

#[cfg(feature = "slint")]
fn padding_to_px(padding: Padding, fallback: f32) -> f32 {
    padding
        .top_side()
        .and_then(padding_value_to_px)
        .or_else(|| padding.left_side().and_then(padding_value_to_px))
        .unwrap_or(fallback)
}

#[cfg(feature = "slint")]
fn card_visual(
    style: Style,
    theme: &SemanticThemeVars,
    variant: ThemeVariant,
    fallback_bg: Color,
    fallback_fg: Color,
    fallback_border: Color,
    fallback_padding: f32,
) -> CardVisual {
    let resolved = style.resolved_theme(theme, variant);
    let bg = resolved
        .background_color_value()
        .and_then(|token| resolve_background(token, theme, variant))
        .unwrap_or_else(|| fallback_bg.to_slint());
    let fg = resolved
        .text_color_token_value()
        .and_then(|token| resolve_text(token, theme, variant))
        .unwrap_or_else(|| fallback_fg.to_slint());
    let border = resolved
        .border_color_token_value()
        .and_then(|token| resolve_border(token, theme, variant))
        .unwrap_or_else(|| fallback_border.to_slint());
    let radius = resolved
        .border_radius_value()
        .map_or(12.0, ToSlint::to_slint);
    let padding = resolved
        .padding_value()
        .copied()
        .map(|padding| padding_to_px(padding, fallback_padding))
        .unwrap_or(fallback_padding);
    let (shadow_offset_y, shadow_blur, shadow_color) = resolved
        .box_shadow_value()
        .map(|shadow| {
            let (offset, blur) = twill_slint::to_shadow(shadow);
            let color = resolved
                .shadow_color_token_value()
                .and_then(|token| resolve_shadow_color(token, theme, variant))
                .unwrap_or_else(|| {
                    twill_slint::to_shadow_with_color(shadow, twill::backends::ShadowColor::Default)
                        .2
                });
            (offset, blur, color)
        })
        .unwrap_or((0.0, 0.0, transparent()));

    CardVisual {
        bg,
        fg,
        border,
        shadow_color,
        radius,
        shadow_blur,
        shadow_offset_y,
        padding,
    }
}

#[cfg(feature = "slint")]
fn rgba_hex(color: Color) -> SharedString {
    let (r, g, b, a) = color.compute().to_rgba8();
    format!("#{r:02X}{g:02X}{b:02X}{a:02X}").into()
}

#[cfg(feature = "slint")]
fn responsive_line(style: &Style, breakpoint: Breakpoint) -> SharedString {
    let resolved = style.at_breakpoint(breakpoint);
    format!(
        "width={:?}, padding={:?}, shadow={:?}",
        resolved.width_value(),
        resolved.padding_value(),
        resolved.box_shadow_value()
    )
    .into()
}

#[cfg(feature = "slint")]
fn apply_theme(window: &ShowcaseWindow, dark_mode: bool, runtime_label: &str) {
    let variant = if dark_mode {
        ThemeVariant::Dark
    } else {
        ThemeVariant::Light
    };
    let theme = SemanticThemeVars::shadcn_neutral();

    let page_bg = theme_color(theme, SemanticColor::Background, variant, Color::white());
    let page_fg = theme_color(
        theme,
        SemanticColor::Foreground,
        variant,
        Color::slate(Scale::S900),
    );
    let border = theme_color(
        theme,
        SemanticColor::Border,
        variant,
        Color::slate(Scale::S300),
    );
    let primary = theme_color(
        theme,
        SemanticColor::Primary,
        variant,
        Color::blue(Scale::S600),
    );
    let primary_fg = theme_color(
        theme,
        SemanticColor::PrimaryForeground,
        variant,
        Color::white(),
    );
    let muted = if dark_mode {
        Color::slate(Scale::S300)
    } else {
        Color::slate(Scale::S600)
    };

    window.set_dark_mode(dark_mode);
    window.set_runtime_label(runtime_label.into());
    window.set_page_bg(page_bg.to_slint());
    window.set_heading_fg(page_fg.to_slint());
    window.set_muted_fg(muted.to_slint());
    window.set_accent_bg(primary.to_slint());
    window.set_accent_fg(primary_fg.to_slint());

    let [(brand, _), (success, _), (warning, _), (accent, _)] = token_palette();
    window.set_token_brand(brand.to_slint());
    window.set_token_success(success.to_slint());
    window.set_token_warning(warning.to_slint());
    window.set_token_accent(accent.to_slint());

    let surface = card_visual(
        surface_style(),
        theme,
        variant,
        theme_color(
            theme,
            SemanticColor::Card,
            variant,
            Color::slate(Scale::S50),
        ),
        theme_color(
            theme,
            SemanticColor::CardForeground,
            variant,
            Color::slate(Scale::S900),
        ),
        border,
        Spacing::S4.to_slint(),
    );
    window.set_surface_bg(surface.bg);
    window.set_surface_fg(surface.fg);
    window.set_surface_border(surface.border);
    window.set_surface_shadow_color(surface.shadow_color);
    window.set_surface_radius(surface.radius);
    window.set_surface_shadow_blur(surface.shadow_blur);
    window.set_surface_shadow_offset(surface.shadow_offset_y);
    window.set_surface_padding(surface.padding);

    let spacing_preview = surface_style().padding(Padding::all(Spacing::S6));
    let spacing_card = card_visual(
        spacing_preview,
        theme,
        variant,
        theme_color(
            theme,
            SemanticColor::Card,
            variant,
            Color::slate(Scale::S50),
        ),
        theme_color(
            theme,
            SemanticColor::CardForeground,
            variant,
            Color::slate(Scale::S900),
        ),
        border,
        Spacing::S6.to_slint(),
    );
    window.set_spacing_preview_padding(spacing_card.padding);

    let shadow_card = card_visual(
        surface_style().shadow(Shadow::Lg),
        theme,
        variant,
        theme_color(
            theme,
            SemanticColor::Card,
            variant,
            Color::slate(Scale::S50),
        ),
        theme_color(
            theme,
            SemanticColor::CardForeground,
            variant,
            Color::slate(Scale::S900),
        ),
        border,
        Spacing::S4.to_slint(),
    );
    window.set_large_shadow_color(shadow_card.shadow_color);
    window.set_large_shadow_blur(shadow_card.shadow_blur);
    window.set_large_shadow_offset(shadow_card.shadow_offset_y);

    let composition = card_visual(
        composition_style(),
        theme,
        variant,
        Color::slate(Scale::S50),
        Color::slate(Scale::S900),
        Color::slate(Scale::S300),
        Spacing::S4.to_slint(),
    );
    window.set_composition_bg(composition.bg);
    window.set_composition_fg(composition.fg);
    window.set_composition_border(composition.border);
    window.set_composition_shadow_color(composition.shadow_color);
    window.set_composition_radius(composition.radius);
    window.set_composition_shadow_blur(composition.shadow_blur);
    window.set_composition_shadow_offset(composition.shadow_offset_y);
    window.set_composition_padding(composition.padding);

    let interactive = card_visual(
        interactive_style(),
        theme,
        variant,
        Color::blue(Scale::S500),
        Color::white(),
        border,
        Spacing::S4.to_slint(),
    );
    window.set_interactive_bg(interactive.bg);
    window.set_interactive_fg(interactive.fg);
    window.set_interactive_border(interactive.border);
    window.set_interactive_shadow_color(interactive.shadow_color);
    window.set_interactive_radius(interactive.radius);
    window.set_interactive_shadow_blur(interactive.shadow_blur);
    window.set_interactive_shadow_offset(interactive.shadow_offset_y);
    window.set_interactive_padding(interactive.padding);

    let semantic_style = Style::card()
        .merged(Style::interactive())
        .padding(Padding::all(Spacing::S4))
        .rounded(BorderRadius::Xl);
    let semantic = card_visual(
        semantic_style,
        theme,
        variant,
        theme_color(
            theme,
            SemanticColor::Card,
            variant,
            Color::slate(Scale::S50),
        ),
        theme_color(
            theme,
            SemanticColor::CardForeground,
            variant,
            Color::slate(Scale::S900),
        ),
        border,
        Spacing::S4.to_slint(),
    );
    window.set_semantic_bg(semantic.bg);
    window.set_semantic_fg(semantic.fg);
    window.set_semantic_border(semantic.border);
    window.set_semantic_shadow_color(semantic.shadow_color);
    window.set_semantic_radius(semantic.radius);
    window.set_semantic_shadow_blur(semantic.shadow_blur);
    window.set_semantic_shadow_offset(semantic.shadow_offset_y);
    window.set_semantic_padding(semantic.padding);

    let responsive_style = responsive_style();
    let responsive = card_visual(
        responsive_style.clone(),
        theme,
        variant,
        Color::slate(Scale::S50),
        Color::slate(Scale::S900),
        Color::slate(Scale::S300),
        Spacing::S4.to_slint(),
    );
    window.set_responsive_bg(responsive.bg);
    window.set_responsive_fg(responsive.fg);
    window.set_responsive_border(responsive.border);
    window.set_responsive_shadow_color(responsive.shadow_color);
    window.set_responsive_radius(responsive.radius);
    window.set_responsive_shadow_blur(responsive.shadow_blur);
    window.set_responsive_shadow_offset(responsive.shadow_offset_y);
    window.set_responsive_padding(responsive.padding);
    window.set_responsive_line_sm(responsive_line(&responsive_style, Breakpoint::Sm));
    window.set_responsive_line_md(responsive_line(&responsive_style, Breakpoint::Md));
    window.set_responsive_line_lg(responsive_line(&responsive_style, Breakpoint::Lg));
    window.set_responsive_line_2xl(responsive_line(&responsive_style, Breakpoint::S2xl));

    let semantic_background =
        theme_color(theme, SemanticColor::Background, variant, Color::white());
    let semantic_foreground = theme_color(
        theme,
        SemanticColor::Foreground,
        variant,
        Color::slate(Scale::S900),
    );
    let semantic_primary = theme_color(
        theme,
        SemanticColor::Primary,
        variant,
        Color::blue(Scale::S600),
    );
    let semantic_primary_fg = theme_color(
        theme,
        SemanticColor::PrimaryForeground,
        variant,
        Color::white(),
    );
    let semantic_border = theme_color(
        theme,
        SemanticColor::Border,
        variant,
        Color::slate(Scale::S300),
    );
    let semantic_ring = theme_color(
        theme,
        SemanticColor::Ring,
        variant,
        Color::slate(Scale::S500),
    );

    window.set_semantic_sample_background(semantic_background.to_slint());
    window.set_semantic_value_background(rgba_hex(semantic_background));
    window.set_semantic_sample_foreground(semantic_foreground.to_slint());
    window.set_semantic_value_foreground(rgba_hex(semantic_foreground));
    window.set_semantic_sample_primary(semantic_primary.to_slint());
    window.set_semantic_value_primary(rgba_hex(semantic_primary));
    window.set_semantic_sample_primary_foreground(semantic_primary_fg.to_slint());
    window.set_semantic_value_primary_foreground(rgba_hex(semantic_primary_fg));
    window.set_semantic_sample_border(semantic_border.to_slint());
    window.set_semantic_value_border(rgba_hex(semantic_border));
    window.set_semantic_sample_ring(semantic_ring.to_slint());
    window.set_semantic_value_ring(rgba_hex(semantic_ring));
}

#[cfg(feature = "slint")]
fn run_showcase(
    title: &'static str,
    initial_runtime_label: String,
    force_winit_backend: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if force_winit_backend {
        slint::BackendSelector::new()
            .backend_name("winit".into())
            .select()?;
    }

    let window = ShowcaseWindow::new()?;
    window.set_window_title(title.into());
    apply_theme(&window, false, &initial_runtime_label);

    let window_weak = window.as_weak();
    let runtime_label = initial_runtime_label.clone();
    window.on_toggle_theme(move || {
        let window = window_weak.unwrap();
        let next = !window.get_dark_mode();
        apply_theme(&window, next, &runtime_label);
    });

    window.run()?;
    Ok(())
}

#[cfg(feature = "slint")]
#[allow(dead_code)]
pub fn run_slint_showcase() -> Result<(), Box<dyn std::error::Error>> {
    run_showcase(
        "twill showcase (slint)",
        String::from("slint runtime"),
        false,
    )
}

#[cfg(feature = "slint")]
#[allow(dead_code)]
pub fn run_winit_showcase() -> Result<(), Box<dyn std::error::Error>> {
    use slint::winit_030::WinitWindowAccessor;

    slint::BackendSelector::new()
        .backend_name("winit".into())
        .select()?;

    let window = ShowcaseWindow::new()?;
    window.set_window_title("twill showcase (winit)".into());

    let mut runtime_label = String::from("slint + explicit winit backend");
    window.window().with_winit_window(|winit_window| {
        runtime_label = format!("{runtime_label} | window id = {:?}", winit_window.id());
    });

    apply_theme(&window, false, &runtime_label);

    let window_weak = window.as_weak();
    window.on_toggle_theme(move || {
        let window = window_weak.unwrap();
        let next = !window.get_dark_mode();
        apply_theme(&window, next, &runtime_label);
    });

    window.run()?;
    Ok(())
}
