use super::*;
use crate::runtime::resources::{
    CardModelRegistry, CardSlotBoardModel, MatchModeModel, MatchModel, MatchPlayerSide,
    STARTING_DECK_CARD_COUNT,
};

#[test]
fn level1_brain_selects_random_affordable_move_with_seeded_choice() {
    let mut match_model = MatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["kage_ren".to_string(); STARTING_DECK_CARD_COUNT],
    );
    match_model.far.hand = vec![
        "kage_ren".to_string(),
        "lord_daichi".to_string(),
        "yokai_placeholder".to_string(),
    ];
    match_model.far.energy_available = 2;
    let slots = CardSlotBoardModel::default();
    let registry = CardModelRegistry::default();

    let first = choose_level1_move(&match_model, MatchPlayerSide::Far, &slots, &registry, 123)
        .expect("brain should choose a move");
    let second = choose_level1_move(&match_model, MatchPlayerSide::Far, &slots, &registry, 123)
        .expect("brain should choose a move");

    assert_eq!(first, second);
    assert_ne!(first.card_id, "yokai_placeholder");
    assert!(first.energy_cost <= 2);
}

#[test]
fn level1_brain_can_plan_multiple_moves_without_mutating_match_or_slots() {
    let mut match_model = MatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["kage_ren".to_string(); STARTING_DECK_CARD_COUNT],
    );
    match_model.far.hand = vec![
        "kage_ren".to_string(),
        "kage_ren".to_string(),
        "lord_daichi".to_string(),
    ];
    match_model.far.hand_instance_ids = vec![21, 22, 23];
    match_model.far.energy_available = 4;
    let slots = CardSlotBoardModel::default();
    let registry = CardModelRegistry::default();

    let moves = choose_level1_moves(&match_model, MatchPlayerSide::Far, &slots, &registry, 123);

    assert_eq!(moves.len(), 3);
    assert_eq!(match_model.far.hand.len(), 3);
    assert_eq!(match_model.far.energy_available, 4);
    assert_eq!(slots.populated_count(), 0);
    assert_ne!(moves[0].instance_id, moves[1].instance_id);
}

#[test]
fn brain_pacing_stays_inside_human_like_delay_bounds() {
    let mut brain = CpuBrainModel::default();

    brain.schedule_next(MatchPlayerSide::Far);

    assert!(brain.far_next_decision_seconds >= minimum_cpu_decision_delay_seconds());
    assert!(brain.far_next_decision_seconds <= maximum_cpu_decision_delay_seconds());
}

#[test]
fn hand_ready_gate_waits_for_settle_then_pause() {
    let mut brain = CpuBrainModel::default();

    assert!(brain.wait_for_settled_hand_pause(MatchPlayerSide::Far, 1, 1, false, 10.0, 0.5));
    assert!(brain.wait_for_settled_hand_pause(MatchPlayerSide::Far, 1, 1, true, 0.49, 0.5));
    assert!(!brain.wait_for_settled_hand_pause(MatchPlayerSide::Far, 1, 1, true, 0.02, 0.5));
}
