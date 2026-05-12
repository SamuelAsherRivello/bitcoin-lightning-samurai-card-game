use super::*;

#[test]
fn round_schedule_matches_contract() {
    let requested: Vec<usize> = (1..=6).map(requested_cards_for_round).collect();
    let energy: Vec<i32> = (1..=6).map(energy_for_round).collect();

    assert_eq!(requested, vec![1, 2, 3, 1, 1, 1]);
    assert_eq!(energy, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn energy_spend_restore_and_move_history_are_round_scoped() {
    let mut model = GameRoundModel::for_round(2);

    assert!(model.spend(1));
    assert_eq!(model.energy_available, 1);
    model.record_move(CurrentRoundMoveRecord {
        hand_index: 3,
        card_id: "test".to_string(),
        location_index: 1,
        slot_index: 0,
        energy_cost: 1,
        location_energy_delta: 0,
    });
    assert!(model.has_undoable_moves());

    model.restore(1);
    assert_eq!(model.energy_available, 2);
    assert!(model.advance_round());
    assert_eq!(model.round, 3);
    assert!(!model.has_undoable_moves());
}

#[test]
fn round_six_end_turn_resolves_without_advancing() {
    let mut model = GameRoundModel::for_round(6);

    assert!(!model.advance_round());
    assert_eq!(model.round, 6);
    assert!(model.end_turn_resolved);
}
