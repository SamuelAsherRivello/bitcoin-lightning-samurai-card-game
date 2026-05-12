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
