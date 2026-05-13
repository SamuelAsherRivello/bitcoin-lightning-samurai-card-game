use super::*;
use crate::runtime::resources::{
    CardModelRegistry, CardSlotBoardModel, MatchModeModel, MatchPlayerSide, OpponentMatchModel,
    STARTING_DECK_CARD_COUNT,
};

#[test]
fn level1_brain_selects_affordable_legal_move_with_seeded_choice() {
    let mut match_model = OpponentMatchModel::new(
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
fn brain_pacing_stays_inside_human_like_delay_bounds() {
    let mut brain = CpuBrainModel::default();

    brain.schedule_next(MatchPlayerSide::Far);

    assert!(brain.far_next_decision_seconds >= minimum_cpu_decision_delay_seconds());
    assert!(brain.far_next_decision_seconds <= maximum_cpu_decision_delay_seconds());
}
