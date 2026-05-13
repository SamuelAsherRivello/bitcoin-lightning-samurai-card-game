use super::*;

#[test]
fn match_mode_cycles_between_two_user_facing_labels() {
    assert_eq!(MatchModeModel::HumanVersusCpu.label(), "Human versus CPU");
    assert_eq!(MatchModeModel::CpuVersusCpu.label(), "CPU versus CPU");
    assert_eq!(
        MatchModeModel::HumanVersusCpu.next(),
        MatchModeModel::CpuVersusCpu
    );
    assert_eq!(
        MatchModeModel::CpuVersusCpu.next(),
        MatchModeModel::HumanVersusCpu
    );
}

#[test]
fn mode_mapping_assigns_expected_controllers() {
    let human_vs_cpu = OpponentMatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["a".to_string(); STARTING_DECK_CARD_COUNT],
    );
    assert!(!human_vs_cpu.near.controller.is_cpu());
    assert!(human_vs_cpu.far.controller.is_cpu());

    let cpu_vs_cpu = OpponentMatchModel::new(
        MatchModeModel::CpuVersusCpu,
        vec!["a".to_string(); STARTING_DECK_CARD_COUNT],
    );
    assert!(cpu_vs_cpu.near.controller.is_cpu());
    assert!(cpu_vs_cpu.far.controller.is_cpu());
}

#[test]
fn winner_status_uses_player_number_and_controller_type_without_brain_wording() {
    let winner = MatchWinnerModel {
        side: MatchPlayerSide::Near,
        controller: PlayerControllerModel::cpu(),
    };

    let status = winner.status_text();

    assert_eq!(status, "Status: Winner is Player 1 (CPU)");
    assert!(!status.contains("Brain"));
}

#[test]
fn current_turn_placements_hide_from_opponent_until_revealed() {
    let mut model = OpponentMatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["a".to_string(); STARTING_DECK_CARD_COUNT],
    );

    model.record_placement(MatchPlayerSide::Near, 1, 2);

    assert!(model.revealed_to_controller(MatchPlayerSide::Near, MatchPlayerSide::Near, 1, 2));
    assert!(!model.revealed_to_controller(MatchPlayerSide::Far, MatchPlayerSide::Near, 1, 2));

    model.reveal_current_turn_placements();

    assert!(model.revealed_to_controller(MatchPlayerSide::Far, MatchPlayerSide::Near, 1, 2));
}
