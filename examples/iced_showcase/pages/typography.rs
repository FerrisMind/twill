use crate::Message;
use crate::components::Snippet;
use iced::Element;
use iced::widget::{column, text};
use twill::iced::{to_font_size, to_font_weight};
use twill::tokens::{FontFamily, FontSize, FontWeight};

pub fn view<'a>(is_dark: bool) -> Element<'a, Message> {
    column![
        text("Typography").size(32).font(iced::Font {
            weight: to_font_weight(FontWeight::Bold),
            ..Default::default()
        }),
        text("Map Twill's typography scale and weights seamlessly into Iced Text widgets.")
            .size(16),
        font_sizes_section(is_dark),
        font_weights_section(is_dark),
        font_family_section(is_dark),
    ]
    .spacing(32)
    .into()
}

// ── Font Sizes ──────────────────────────────────────────────────────

fn font_sizes_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let size = to_font_size(FontSize::S3xl); // 30px
text("Hello").size(size)"#;

    let font_sizes = [
        (FontSize::Xs, "Xs"),
        (FontSize::Sm, "Sm"),
        (FontSize::Base, "Base"),
        (FontSize::Lg, "Lg"),
        (FontSize::Xl, "Xl"),
        (FontSize::S2xl, "2XL"),
        (FontSize::S3xl, "3XL"),
        (FontSize::S4xl, "4XL"),
        (FontSize::S5xl, "5XL"),
        (FontSize::S6xl, "6XL"),
        (FontSize::S7xl, "7XL"),
        (FontSize::S8xl, "8XL"),
        (FontSize::S9xl, "9XL"),
    ];

    let mut col = column![].spacing(8);
    for (size, label) in font_sizes.iter() {
        col = col.push(
            text(format!("{} — {:.0}px", label, to_font_size(*size))).size(to_font_size(*size)),
        );
    }

    Snippet::new("Font Sizes", code, col).view(is_dark)
}

// ── Font Weights ────────────────────────────────────────────────────

fn font_weights_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"let weight = to_font_weight(FontWeight::Bold);
text("Bold text").font(iced::Font { weight, ..Default::default() })"#;

    let weights = [
        (FontWeight::Thin, "Thin (100)"),
        (FontWeight::ExtraLight, "ExtraLight (200)"),
        (FontWeight::Light, "Light (300)"),
        (FontWeight::Normal, "Normal (400)"),
        (FontWeight::Medium, "Medium (500)"),
        (FontWeight::SemiBold, "SemiBold (600)"),
        (FontWeight::Bold, "Bold (700)"),
        (FontWeight::ExtraBold, "ExtraBold (800)"),
        (FontWeight::Black, "Black (900)"),
    ];

    let mut col = column![].spacing(8);
    for (weight, label) in weights.iter() {
        col = col.push(
            text(*label)
                .size(to_font_size(FontSize::Xl))
                .font(iced::Font {
                    weight: to_font_weight(*weight),
                    ..Default::default()
                }),
        );
    }

    Snippet::new("Font Weights", code, col).view(is_dark)
}

// ── Font Family ─────────────────────────────────────────────────────

fn font_family_section<'a>(is_dark: bool) -> Element<'a, Message> {
    let code = r#"// FontFamily::Sans → system sans-serif
// FontFamily::Serif → system serif
// FontFamily::Mono → system monospace
text("The quick brown fox").font(iced::Font::MONOSPACE)"#;

    let families = [
        (FontFamily::Sans, "Sans-Serif", iced::Font::DEFAULT),
        (FontFamily::Serif, "Serif", iced::Font::with_name("serif")),
        (FontFamily::Mono, "Monospace", iced::Font::MONOSPACE),
    ];

    let mut col = column![].spacing(16);
    for (family, label, iced_font) in families.iter() {
        let sample_text = format!(
            "{}: The quick brown fox jumps over the lazy dog — {}",
            label,
            family.stack()
        );
        col = col.push(
            text(sample_text)
                .size(to_font_size(FontSize::Lg))
                .font(*iced_font),
        );
    }

    Snippet::new("Font Family", code, col).view(is_dark)
}
