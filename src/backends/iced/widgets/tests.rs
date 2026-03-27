use super::*;
use crate::iced::TextDirection;
use crate::prelude::*;
use crate::tokens::AspectRatio;
use crate::tokens::Scale;
use iced::Length;

#[test]
fn test_color_conversion() {
    let blue = Color::blue(Scale::S500);
    let c = to_color(blue);
    let raw = blue.compute();
    let (r, g, b) = raw.to_rgb8();
    assert!((c.r - r as f32 / 255.0).abs() < 0.01);
    assert!((c.g - g as f32 / 255.0).abs() < 0.01);
    assert!((c.b - b as f32 / 255.0).abs() < 0.01);
}

#[test]
fn test_color_conversion_uses_raw_values() {
    let color = Color::blue(Scale::S500);
    let converted = to_color(color);
    let raw = color.compute();
    let (r, g, b) = raw.to_rgb8();
    assert!((converted.r - r as f32 / 255.0).abs() < 0.001);
    assert!((converted.g - g as f32 / 255.0).abs() < 0.001);
    assert!((converted.b - b as f32 / 255.0).abs() < 0.001);
}

#[test]
fn test_font_size_resolution_variants() {
    const TITLE_SIZE: crate::tokens::FontSizeVar = crate::tokens::FontSizeVar::new("--title");
    assert!((to_font_size(FontSize::Base) - 16.0).abs() < f32::EPSILON);
    assert_eq!(resolve_font_size(FontSize::var(TITLE_SIZE), &[]), None);
    assert_eq!(
        resolve_font_size(FontSize::var(TITLE_SIZE), &[("--title", 28.0)]),
        Some(28.0)
    );
    assert_eq!(resolve_font_size(FontSize::px(22), &[]), Some(22.0));
}

#[test]
fn test_font_weight_mapping_variants() {
    assert_eq!(to_font_weight(FontWeight::Thin), iced::font::Weight::Thin);
    assert_eq!(
        to_font_weight(FontWeight::ExtraLight),
        iced::font::Weight::ExtraLight
    );
    assert_eq!(to_font_weight(FontWeight::Light), iced::font::Weight::Light);
    assert_eq!(
        to_font_weight(FontWeight::Normal),
        iced::font::Weight::Normal
    );
    assert_eq!(
        to_font_weight(FontWeight::Medium),
        iced::font::Weight::Medium
    );
    assert_eq!(
        to_font_weight(FontWeight::SemiBold),
        iced::font::Weight::Semibold
    );
    assert_eq!(to_font_weight(FontWeight::Bold), iced::font::Weight::Bold);
    assert_eq!(
        to_font_weight(FontWeight::ExtraBold),
        iced::font::Weight::ExtraBold
    );
    assert_eq!(to_font_weight(FontWeight::Black), iced::font::Weight::Black);
}

#[test]
fn test_text_alignment_mapping_variants() {
    assert_eq!(
        to_text_alignment(TextAlign::Left),
        iced::widget::text::Alignment::Left
    );
    assert_eq!(
        to_text_alignment(TextAlign::Center),
        iced::widget::text::Alignment::Center
    );
    assert_eq!(
        to_text_alignment(TextAlign::Right),
        iced::widget::text::Alignment::Right
    );
    assert_eq!(
        to_text_alignment(TextAlign::Justify),
        iced::widget::text::Alignment::Justified
    );
    assert_eq!(
        to_text_alignment_with_direction(TextAlign::Start, TextDirection::LeftToRight),
        iced::widget::text::Alignment::Left
    );
    assert_eq!(
        to_text_alignment_with_direction(TextAlign::Start, TextDirection::RightToLeft),
        iced::widget::text::Alignment::Right
    );
    assert_eq!(
        to_text_alignment_with_direction(TextAlign::End, TextDirection::LeftToRight),
        iced::widget::text::Alignment::Right
    );
    assert_eq!(
        to_text_alignment_with_direction(TextAlign::End, TextDirection::RightToLeft),
        iced::widget::text::Alignment::Left
    );
}

#[test]
fn test_spacing_px_padding() {
    let p = to_padding(Spacing::Px);
    assert!((p.top - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_style_padding_allows_partial_sides() {
    let px = to_style_padding(crate::utilities::Padding::x(Spacing::S4), &[]);
    assert!((px.left - 16.0).abs() < f32::EPSILON);
    assert!((px.right - 16.0).abs() < f32::EPSILON);
    assert!((px.top - 0.0).abs() < f32::EPSILON);
    assert!((px.bottom - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_style_padding_logical_aliases_map_to_physical_sides() {
    let ps = to_style_padding(crate::utilities::Padding::ps(Spacing::S6), &[]);
    let pe = to_style_padding(crate::utilities::Padding::pe(Spacing::S6), &[]);
    let pbs = to_style_padding(crate::utilities::Padding::pbs(Spacing::S2), &[]);
    let pbe = to_style_padding(crate::utilities::Padding::pbe(Spacing::S2), &[]);

    assert!((ps.left - 24.0).abs() < f32::EPSILON);
    assert!((ps.right - 0.0).abs() < f32::EPSILON);
    assert!((pe.left - 0.0).abs() < f32::EPSILON);
    assert!((pe.right - 24.0).abs() < f32::EPSILON);
    assert!((pbs.top - 8.0).abs() < f32::EPSILON);
    assert!((pbe.bottom - 8.0).abs() < f32::EPSILON);
}

#[test]
fn test_style_padding_px_token_families() {
    let p = to_style_padding(crate::utilities::Padding::p(Spacing::Px), &[]);
    let px = to_style_padding(crate::utilities::Padding::px(Spacing::Px), &[]);
    let py = to_style_padding(crate::utilities::Padding::py(Spacing::Px), &[]);
    let pt = to_style_padding(crate::utilities::Padding::pt(Spacing::Px), &[]);
    let pr = to_style_padding(crate::utilities::Padding::pr(Spacing::Px), &[]);
    let pb = to_style_padding(crate::utilities::Padding::pb(Spacing::Px), &[]);
    let pl = to_style_padding(crate::utilities::Padding::pl(Spacing::Px), &[]);
    let ps = to_style_padding(crate::utilities::Padding::ps(Spacing::Px), &[]);
    let pe = to_style_padding(crate::utilities::Padding::pe(Spacing::Px), &[]);
    let pbs = to_style_padding(crate::utilities::Padding::pbs(Spacing::Px), &[]);
    let pbe = to_style_padding(crate::utilities::Padding::pbe(Spacing::Px), &[]);

    assert!((p.top - 1.0).abs() < f32::EPSILON);
    assert!((p.right - 1.0).abs() < f32::EPSILON);
    assert!((p.bottom - 1.0).abs() < f32::EPSILON);
    assert!((p.left - 1.0).abs() < f32::EPSILON);
    assert!((px.left - 1.0).abs() < f32::EPSILON);
    assert!((px.right - 1.0).abs() < f32::EPSILON);
    assert!((py.top - 1.0).abs() < f32::EPSILON);
    assert!((py.bottom - 1.0).abs() < f32::EPSILON);
    assert!((pt.top - 1.0).abs() < f32::EPSILON);
    assert!((pr.right - 1.0).abs() < f32::EPSILON);
    assert!((pb.bottom - 1.0).abs() < f32::EPSILON);
    assert!((pl.left - 1.0).abs() < f32::EPSILON);
    assert!((ps.left - 1.0).abs() < f32::EPSILON);
    assert!((pe.right - 1.0).abs() < f32::EPSILON);
    assert!((pbs.top - 1.0).abs() < f32::EPSILON);
    assert!((pbe.bottom - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_style_padding_custom_property_and_arbitrary_values() {
    let var = crate::utilities::PaddingVar::new("--pad");
    let p = crate::utilities::Padding::individual_value(
        PaddingValue::var(var),
        PaddingValue::px(5.0),
        PaddingValue::rem(1.0),
        PaddingValue::Scale(Spacing::S2),
    );
    let resolved = to_style_padding(p, &[("--pad", 12.0)]);
    assert!((resolved.top - 12.0).abs() < f32::EPSILON);
    assert!((resolved.right - 5.0).abs() < f32::EPSILON);
    assert!((resolved.bottom - 16.0).abs() < f32::EPSILON);
    assert!((resolved.left - 8.0).abs() < f32::EPSILON);
}

#[test]
fn test_resolve_width_fixed_and_container_variants() {
    assert_eq!(
        resolve_width(crate::utilities::Width::w(Spacing::S24), &[]),
        Some(ResolvedWidth::Length(Length::Fixed(96.0)))
    );
    assert_eq!(
        resolve_width(
            crate::utilities::Width::w_container(crate::tokens::Container::Md),
            &[]
        ),
        Some(ResolvedWidth::Length(Length::Fixed(448.0)))
    );
    assert_eq!(
        resolve_width(crate::utilities::Width::w_px(), &[]),
        Some(ResolvedWidth::Length(Length::Fixed(1.0)))
    );
    assert_eq!(
        resolve_width(crate::utilities::Width::w_full(), &[]),
        Some(ResolvedWidth::Length(Length::Fill))
    );
    assert_eq!(
        resolve_width(crate::utilities::Width::w_auto(), &[]),
        Some(ResolvedWidth::Length(Length::Shrink))
    );
}

#[test]
fn test_resolve_width_fraction_variants() {
    let half = resolve_width(
        crate::utilities::Width::w_fraction(crate::tokens::Percentage::S1_2),
        &[],
    );
    match half {
        Some(ResolvedWidth::Ratio(ratio)) => assert!((ratio - 0.5).abs() < f32::EPSILON),
        _ => panic!("expected ratio width"),
    }

    let two_fifths = resolve_width(
        crate::utilities::Width::w_fraction(crate::tokens::Percentage::S2_5),
        &[],
    );
    match two_fifths {
        Some(ResolvedWidth::Ratio(ratio)) => assert!((ratio - 0.4).abs() < 0.0001),
        _ => panic!("expected ratio width"),
    }
}

#[test]
fn test_resolve_width_custom_property_and_px_value() {
    let var = crate::utilities::WidthVar::new("--panel-w");
    assert_eq!(
        resolve_width(crate::utilities::Width::w_var(var), &[("--panel-w", 280.0)]),
        Some(ResolvedWidth::Length(Length::Fixed(280.0)))
    );
    assert_eq!(
        resolve_width(crate::utilities::Width::w_px_value(320), &[]),
        Some(ResolvedWidth::Length(Length::Fixed(320.0)))
    );
}

#[test]
fn test_resolve_height_fixed_and_variant_families() {
    assert_eq!(
        resolve_height(crate::utilities::Height::h(Spacing::S24), &[]),
        Some(ResolvedHeight::Length(Length::Fixed(96.0)))
    );
    assert_eq!(
        resolve_height(crate::utilities::Height::h_px(), &[]),
        Some(ResolvedHeight::Length(Length::Fixed(1.0)))
    );
    assert_eq!(
        resolve_height(crate::utilities::Height::h_full(), &[]),
        Some(ResolvedHeight::Length(Length::Fill))
    );
    assert_eq!(
        resolve_height(crate::utilities::Height::h_auto(), &[]),
        Some(ResolvedHeight::Length(Length::Shrink))
    );
    assert_eq!(
        resolve_height(crate::utilities::Height::h_lh(), &[]),
        Some(ResolvedHeight::Length(Length::Shrink))
    );
}

#[test]
fn test_resolve_height_fraction_variants() {
    let half = resolve_height(
        crate::utilities::Height::h_fraction(crate::tokens::Percentage::S1_2),
        &[],
    );
    match half {
        Some(ResolvedHeight::Ratio(ratio)) => assert!((ratio - 0.5).abs() < f32::EPSILON),
        _ => panic!("expected ratio height"),
    }

    let two_fifths = resolve_height(
        crate::utilities::Height::h_fraction(crate::tokens::Percentage::S2_5),
        &[],
    );
    match two_fifths {
        Some(ResolvedHeight::Ratio(ratio)) => assert!((ratio - 0.4).abs() < 0.0001),
        _ => panic!("expected ratio height"),
    }
}

#[test]
fn test_resolve_height_custom_property_and_px_value() {
    let var = crate::utilities::HeightVar::new("--panel-h");
    assert_eq!(
        resolve_height(
            crate::utilities::Height::h_var(var),
            &[("--panel-h", 240.0)]
        ),
        Some(ResolvedHeight::Length(Length::Fixed(240.0)))
    );
    assert_eq!(
        resolve_height(crate::utilities::Height::h_px_value(360), &[]),
        Some(ResolvedHeight::Length(Length::Fixed(360.0)))
    );
}

#[test]
fn test_aspect_ratio_zero_denominator() {
    assert_eq!(to_aspect_ratio(AspectRatio::Custom(16, 0)), None);
}

#[test]
fn test_shadow_uses_custom_color() {
    let shadow = to_shadow_with_color(
        Shadow::Sm,
        crate::backends::ShadowColor::Explicit(Color::red(Scale::S500)),
    );
    assert!(shadow.color.r > shadow.color.g);
}

#[test]
fn test_shadow_layers_for_sm_are_two() {
    let layers = to_shadow_layers_with_color(Shadow::Sm, crate::backends::ShadowColor::Default);
    assert_eq!(layers.len(), 2);
    assert!((layers[0].offset.y - 1.0).abs() < f32::EPSILON);
    assert!((layers[1].offset.y - 1.0).abs() < f32::EPSILON);
    assert!((layers[0].blur_radius - 3.0).abs() < f32::EPSILON);
    assert!((layers[1].blur_radius - 2.0).abs() < f32::EPSILON);
}

#[test]
fn test_shadow_layers_for_2xl_are_single() {
    let layers = to_shadow_layers_with_color(Shadow::S2xl, crate::backends::ShadowColor::Default);
    assert_eq!(layers.len(), 1);
    assert!((layers[0].offset.y - 25.0).abs() < f32::EPSILON);
    assert!((layers[0].blur_radius - 50.0).abs() < f32::EPSILON);
    assert!((layers[0].color.a - 0.25).abs() < f32::EPSILON);
}

#[test]
fn test_resolve_columns_count_from_count() {
    let count = resolve_columns_count(Columns::count(3), 900.0, 16.0, DEFAULT_MAX_COLUMNS);
    assert_eq!(count, 3);
}

#[test]
fn test_resolve_columns_count_from_width() {
    let count = resolve_columns_count(
        Columns::width(Container::S3xs),
        820.0,
        16.0,
        DEFAULT_MAX_COLUMNS,
    );
    assert_eq!(count, 3);
}

#[test]
fn test_resolve_columns_count_from_custom_width() {
    let count = resolve_columns_count(Columns::width_px(280.0), 900.0, 20.0, DEFAULT_MAX_COLUMNS);
    assert_eq!(count, 3);
}

#[test]
fn test_resolve_columns_count_auto_is_one() {
    let count = resolve_columns_count(Columns::auto(), 1200.0, 16.0, DEFAULT_MAX_COLUMNS);
    assert_eq!(count, 1);
}

#[test]
fn test_resolve_columns_count_small_width_is_one() {
    let count = resolve_columns_count(
        Columns::width(Container::S3xs),
        120.0,
        16.0,
        DEFAULT_MAX_COLUMNS,
    );
    assert_eq!(count, 1);
}

#[test]
fn test_resolve_columns_count_is_capped() {
    let by_count = resolve_columns_count(Columns::count(255), 5000.0, 0.0, usize::MAX);
    assert_eq!(by_count, ABSOLUTE_MAX_COLUMNS);

    let by_width = resolve_columns_count(Columns::width_px(1.0), 20_000.0, 0.0, usize::MAX);
    assert_eq!(by_width, ABSOLUTE_MAX_COLUMNS);
}

#[test]
fn test_resolve_columns_count_respects_style_max() {
    let count = resolve_columns_count(Columns::width(Container::S3xs), 1200.0, 16.0, 2);
    assert_eq!(count, 2);
}

#[test]
fn test_track_count_from_repeat_value() {
    assert_eq!(
        track_count_from_template_value("repeat(4, minmax(0, 1fr))"),
        Some(4)
    );
}

#[test]
fn test_track_count_from_explicit_tracks() {
    assert_eq!(
        track_count_from_template_value("200px minmax(0, 1fr) 100px"),
        Some(3)
    );
}

#[test]
fn test_resolve_grid_template_track_count_variants() {
    assert_eq!(
        resolve_grid_template_track_count(&GridTemplate::count(3), None, &[]),
        3
    );
    assert_eq!(
        resolve_grid_template_track_count(&GridTemplate::none(), None, &[]),
        1
    );
    assert_eq!(
        resolve_grid_template_track_count(&GridTemplate::subgrid(), Some(5), &[]),
        5
    );
    assert_eq!(
        resolve_grid_template_track_count(
            &GridTemplate::custom_property("--layout-cols"),
            None,
            &[("--layout-cols", "repeat(6, minmax(0, 1fr))")]
        ),
        6
    );
    assert_eq!(
        resolve_grid_template_track_count(
            &GridTemplate::arbitrary("200px_minmax(0,_1fr)_100px"),
            None,
            &[]
        ),
        3
    );
}

#[test]
fn test_grid_template_columns_layout_builds() {
    let _: iced::Element<'_, ()> =
        grid_template_columns_layout(vec![], GridTemplate::count(4), Spacing::S4);
    let _: iced::Element<'_, ()> = grid_template_columns_layout_with_context(
        vec![],
        GridTemplate::subgrid(),
        Spacing::S2,
        Some(3),
        &[],
    );
}

#[test]
fn test_resolve_column_width_handles_tight_space() {
    let width = resolve_column_width(100.0, 3, 80.0);
    assert_eq!(width, 0.0);
}

#[test]
fn test_resolve_column_width_handles_invalid_gap() {
    let width = resolve_column_width(600.0, 3, f32::NAN);
    assert!((width - 200.0).abs() < f32::EPSILON);
}

#[test]
fn test_resolve_aspect_size_by_width() {
    let size = resolve_aspect_size(240.0, 600.0, 1.0);
    assert!((size.width - 240.0).abs() < f32::EPSILON);
    assert!((size.height - 240.0).abs() < f32::EPSILON);
}

#[test]
fn test_resolve_aspect_size_height_capped() {
    let size = resolve_aspect_size(400.0, 100.0, 16.0 / 9.0);
    assert!((size.height - 100.0).abs() < f32::EPSILON);
    assert!(size.width > 170.0 && size.width < 180.0);
}

#[test]
fn test_resolve_aspect_size_invalid_ratio_defaults() {
    let size = resolve_aspect_size(120.0, 80.0, 0.0);
    assert!((size.width - 80.0).abs() < f32::EPSILON);
    assert!((size.height - 80.0).abs() < f32::EPSILON);
}

#[test]
fn test_reverse_directions_detection() {
    assert!(is_reverse_direction(FlexDirection::RowReverse));
    assert!(is_reverse_direction(FlexDirection::ColReverse));
    assert!(!is_reverse_direction(FlexDirection::Row));
    assert!(!is_reverse_direction(FlexDirection::Col));
}

#[test]
fn test_flex_direction_layout_reverse_builds() {
    let _: iced::Element<'_, ()> =
        flex_direction_layout(vec![], FlexDirection::RowReverse, Spacing::S4);
    let _: iced::Element<'_, ()> =
        flex_direction_layout(vec![], FlexDirection::ColReverse, Spacing::S4);
}

#[test]
fn test_align_items_safe_and_baseline_mapping() {
    assert_eq!(
        row_alignment_for_items(AlignItems::EndSafe),
        iced::alignment::Vertical::Bottom
    );
    assert_eq!(
        row_alignment_for_items(AlignItems::BaselineLast),
        iced::alignment::Vertical::Bottom
    );
    assert_eq!(
        column_alignment_for_items(AlignItems::CenterSafe),
        iced::alignment::Horizontal::Center
    );
}

#[test]
fn test_align_items_layout_builds() {
    let _: iced::Element<'_, ()> = align_items_layout(
        vec![],
        FlexDirection::Row,
        Spacing::S4,
        AlignItems::CenterSafe,
    );
    let _: iced::Element<'_, ()> = align_items_layout(
        vec![],
        FlexDirection::ColReverse,
        Spacing::S2,
        AlignItems::Stretch,
    );
}

#[test]
fn test_gap_main_axis_mapping() {
    assert_eq!(
        gap_on_main_axis(
            FlexDirection::Row,
            Some(Spacing::S4),
            Some(Spacing::S2),
            Some(Spacing::S8)
        ),
        Spacing::S8
    );
    assert_eq!(
        gap_on_main_axis(
            FlexDirection::RowReverse,
            Some(Spacing::S4),
            Some(Spacing::S2),
            None
        ),
        Spacing::S4
    );
    assert_eq!(
        gap_on_main_axis(
            FlexDirection::Col,
            Some(Spacing::S4),
            Some(Spacing::S6),
            Some(Spacing::S8)
        ),
        Spacing::S6
    );
    assert_eq!(
        gap_on_main_axis(FlexDirection::ColReverse, None, None, Some(Spacing::S8)),
        Spacing::S0
    );
}

#[test]
fn test_gap_layout_builds() {
    let _: iced::Element<'_, ()> = gap_layout(vec![], FlexDirection::Row, Spacing::S4);
    let _: iced::Element<'_, ()> = gap_x_layout(vec![], FlexDirection::Row, Spacing::S6);
    let _: iced::Element<'_, ()> = gap_y_layout(vec![], FlexDirection::Col, Spacing::S3);
}

#[test]
fn test_justify_content_safe_mapping() {
    assert_eq!(
        normalize_justify_content(JustifyContent::EndSafe),
        JustifyContent::End
    );
    assert_eq!(
        normalize_justify_content(JustifyContent::CenterSafe),
        JustifyContent::Center
    );
    assert_eq!(
        normalize_justify_content(JustifyContent::Between),
        JustifyContent::Between
    );
}

#[test]
fn test_justify_content_layout_builds() {
    let _: iced::Element<'_, ()> = justify_content_layout(
        vec![],
        FlexDirection::Row,
        Spacing::S4,
        JustifyContent::Between,
    );
    let _: iced::Element<'_, ()> = justify_content_layout(
        vec![],
        FlexDirection::RowReverse,
        Spacing::S2,
        JustifyContent::CenterSafe,
    );
    let _: iced::Element<'_, ()> = justify_content_layout(
        vec![],
        FlexDirection::ColReverse,
        Spacing::S2,
        JustifyContent::Evenly,
    );
}
