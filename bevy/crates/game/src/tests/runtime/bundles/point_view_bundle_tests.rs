use super::*;
use crate::runtime::resources::{CostPointModel, PowerPointModel};

#[test]
fn point_view_base_text_font_size_is_four_times_previous_size() {
    assert_eq!(POINT_VIEW_BASE_TEXT_FONT_SIZE, 168.0);
}

#[test]
fn point_view_bundle_uses_dedicated_font_choice() {
    assert_eq!(
        POINT_VIEW_BUNDLE_FONT.asset_path(),
        POINT_VIEW_FONT.asset_path()
    );
}

#[test]
fn point_model_constructs_expected_type_and_payload() {
    let location_power = PointModel::location_power(7);
    let card_power = PointModel::card_power(-8);
    let card_energy = PointModel::card_energy(99);

    assert_eq!(location_power.point_type, PointType::LocationPower);
    assert_eq!(location_power.value, 7);
    assert_eq!(card_power.point_type, PointType::CardPower);
    assert_eq!(card_power.value, -8);
    assert_eq!(card_energy.point_type, PointType::CardEnergy);
    assert_eq!(card_energy.value, 99);
}

#[test]
fn point_model_formats_numeric_display_with_negatives() {
    let point = PointModel::new(PointType::CardPower, -99);

    assert_eq!(point.display_text(), "-99");
}

#[test]
fn point_model_from_cost_and_power_inputs_preserves_type_and_value() {
    let location_from_power =
        PointModel::from_power_point(PointType::LocationPower, PowerPointModel::new(-42));
    let card_energy = PointModel::from_cost_point(CostPointModel::new(42));
    let card_power = PointModel::from_power_point(PointType::CardPower, PowerPointModel::new(11));

    assert_eq!(
        location_from_power,
        PointModel::new(PointType::LocationPower, -42)
    );
    assert_eq!(card_energy, PointModel::new(PointType::CardEnergy, 42));
    assert_eq!(card_power, PointModel::new(PointType::CardPower, 11));
}

#[test]
fn point_model_styles_power_red_energy_blue_with_white_text() {
    assert_eq!(
        PointModel::location_power(0).background_color(),
        Color::srgb(0.74, 0.18, 0.18)
    );
    assert_eq!(
        PointModel::card_power(0).background_color(),
        Color::srgb(0.74, 0.18, 0.18)
    );
    assert_eq!(
        PointModel::card_energy(0).background_color(),
        Color::srgb(0.04, 0.18, 0.60)
    );
    assert_eq!(PointModel::card_power(0).text_color(), Color::WHITE);
    assert_eq!(PointModel::card_energy(0).text_color(), Color::WHITE);
}

#[test]
fn point_view_bundle_contains_name_and_view_payload() {
    let model = PointModel::card_power(4);
    let bundle = PointViewBundle::new("Test Point Bundle", model);

    assert_eq!(bundle.name.as_str(), "Test Point Bundle");
    assert_eq!(bundle.view, PointView::new(model));
}
