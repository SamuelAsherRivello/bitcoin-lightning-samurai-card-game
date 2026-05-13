use super::*;

#[test]
fn locations_open_left_middle_right_on_rounds_one_two_three() {
    let model = GameLocationModel::default();

    assert_eq!(
        model.definition(0).unwrap().state(1),
        GameLocationState::Open
    );
    assert_eq!(
        model.definition(1).unwrap().state(1),
        GameLocationState::Closed
    );
    assert_eq!(
        model.definition(1).unwrap().state(2),
        GameLocationState::Open
    );
    assert_eq!(
        model.definition(2).unwrap().state(3),
        GameLocationState::Open
    );
}

#[test]
fn closed_and_open_location_text_matches_contract() {
    let model = GameLocationModel::default();
    let middle = model.definition(1).unwrap();

    assert_eq!(middle.display_title(1), "Closed Until Round 2");
    assert_eq!(middle.display_body(1), "");
    assert_eq!(middle.display_title(2), "Bamboo Crossing");
    assert_eq!(middle.display_body(2), "-2 Power to each card here");
    assert_eq!(
        model.definition(2).unwrap().display_title(3),
        "Shrine Ruins"
    );
    assert_eq!(model.definition(2).unwrap().display_body(3), "(No Ability)");
}

#[test]
fn only_open_locations_apply_energy_delta() {
    let mut model = GameLocationModel::default();

    assert_eq!(model.ability_delta_for_location(0), 2);
    assert_eq!(model.ability_delta_for_location(1), 0);
    model.set_round(2);
    assert_eq!(model.ability_delta_for_location(1), -2);
    model.set_round(3);
    assert_eq!(model.ability_delta_for_location(2), 0);
}

#[test]
fn active_location_indices_choose_three_definitions_in_slot_order() {
    let mut model = GameLocationModel::default();

    model.reset_with_active_location_indices(&[5, 3, 4]);

    assert_eq!(model.round, 1);
    assert_eq!(model.definition(0).unwrap().title, "Market Square");
    assert_eq!(model.definition(0).unwrap().opens_on_round, 1);
    assert_eq!(model.definition(1).unwrap().title, "Battlefield");
    assert_eq!(model.definition(1).unwrap().opens_on_round, 2);
    assert_eq!(model.definition(2).unwrap().title, "Spirit Well");
    assert_eq!(model.definition(2).unwrap().opens_on_round, 3);
    assert_eq!(model.definition(2).unwrap().ability.energy_delta(), -1);
}

#[test]
fn market_square_doubles_side_power_only_at_four_cards_when_open() {
    let mut model = GameLocationModel::default();

    model.reset_with_active_location_indices(&[5, 0, 1]);

    assert_eq!(
        model.definition(0).unwrap().display_body(1),
        "Double Power, if 4 cards here"
    );
    assert_eq!(model.power_multiplier_for_location_side(0, 3), 1);
    assert_eq!(model.power_multiplier_for_location_side(0, 4), 2);
    assert_eq!(model.power_multiplier_for_location_side(0, 5), 1);

    model.reset_with_active_location_indices(&[0, 5, 1]);

    assert_eq!(model.power_multiplier_for_location_side(1, 4), 1);
    model.set_round(2);
    assert_eq!(model.power_multiplier_for_location_side(1, 4), 2);
}

#[test]
fn location_definition_pool_contains_all_six_locations() {
    let titles: Vec<&str> = location_definition_pool()
        .iter()
        .map(|definition| definition.title)
        .collect();

    assert_eq!(
        titles,
        vec![
            "Fortress Gate",
            "Bamboo Crossing",
            "Shrine Ruins",
            "Battlefield",
            "Spirit Well",
            "Market Square",
        ]
    );
}
