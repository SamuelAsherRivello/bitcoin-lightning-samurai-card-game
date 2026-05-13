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
fn current_round_placements_hide_from_opponent_until_revealed() {
    let mut model = OpponentMatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["a".to_string(); STARTING_DECK_CARD_COUNT],
    );

    model.record_placement(MatchPlayerSide::Near, 1, 2);

    assert!(model.revealed_to_controller(MatchPlayerSide::Near, MatchPlayerSide::Near, 1, 2));
    assert!(!model.revealed_to_controller(MatchPlayerSide::Far, MatchPlayerSide::Near, 1, 2));

    model.reveal_current_round_placements();

    assert!(model.revealed_to_controller(MatchPlayerSide::Far, MatchPlayerSide::Near, 1, 2));
}

#[test]
fn current_round_reveal_targets_skip_empty_slots_and_follow_location_order() {
    let mut model = OpponentMatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["a".to_string(); STARTING_DECK_CARD_COUNT],
    );
    let mut slots = CardSlotBoardModel::default();
    assert!(slots.place_for_side_with_card_id(2, CardSlotSide::Opponent, 0, 20, "far_late"));
    assert!(slots.place_for_side_with_card_id(0, CardSlotSide::Opponent, 1, 21, "far_first"));
    assert!(slots.place_for_side_with_card_id(1, CardSlotSide::LocalPlayer, 3, 10, "near_second"));
    model.record_placement(MatchPlayerSide::Far, 0, 1);
    model.record_placement(MatchPlayerSide::Near, 0, 0);
    model.record_placement(MatchPlayerSide::Near, 1, 3);
    model.record_placement(MatchPlayerSide::Far, 2, 0);

    let targets = model.current_round_reveal_targets(&slots);

    assert_eq!(
        targets,
        vec![
            PlacementRevealTarget {
                owner: MatchPlayerSide::Far,
                location_index: 0,
                slot_index: 1,
            },
            PlacementRevealTarget {
                owner: MatchPlayerSide::Near,
                location_index: 1,
                slot_index: 3,
            },
            PlacementRevealTarget {
                owner: MatchPlayerSide::Far,
                location_index: 2,
                slot_index: 0,
            },
        ]
    );
}

#[test]
fn reveal_targets_use_side_specific_slot_order() {
    let mut model = OpponentMatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec!["a".to_string(); STARTING_DECK_CARD_COUNT],
    );
    let mut slots = CardSlotBoardModel::default();
    for slot_index in 0..4 {
        assert!(slots.place_for_side_with_card_id(
            0,
            CardSlotSide::Opponent,
            slot_index,
            20 + slot_index,
            format!("far_{slot_index}"),
        ));
        assert!(slots.place_for_side_with_card_id(
            1,
            CardSlotSide::LocalPlayer,
            slot_index,
            10 + slot_index,
            format!("near_{slot_index}"),
        ));
        model.record_placement(MatchPlayerSide::Far, 0, slot_index);
        model.record_placement(MatchPlayerSide::Near, 1, slot_index);
    }

    let far_order = model
        .current_round_reveal_targets(&slots)
        .into_iter()
        .filter(|target| target.owner == MatchPlayerSide::Far)
        .map(|target| target.slot_index)
        .collect::<Vec<_>>();
    let near_order = model
        .current_round_reveal_targets(&slots)
        .into_iter()
        .filter(|target| target.owner == MatchPlayerSide::Near)
        .map(|target| target.slot_index)
        .collect::<Vec<_>>();

    assert_eq!(far_order, vec![2, 3, 0, 1]);
    assert_eq!(near_order, vec![0, 1, 2, 3]);
}

#[test]
fn player_draw_appends_new_cards_without_replacing_unused_hand_cards() {
    let mut player = MatchPlayerModel::new(
        MatchPlayerSide::Near,
        PlayerControllerModel::human(),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );

    player.draw(1);
    let first_instance_id = player.hand_instance_id(0).unwrap();
    player.draw(2);

    assert_eq!(player.hand, vec!["a", "b", "c"]);
    assert_eq!(player.hand_instance_id(0), Some(first_instance_id));
    assert_eq!(player.deck.len(), 0);
}

#[test]
fn removing_played_card_preserves_unplayed_card_instance_ids() {
    let mut player = MatchPlayerModel::new(
        MatchPlayerSide::Far,
        PlayerControllerModel::cpu(),
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
    );
    player.draw(3);
    let unused_before = vec![player.hand_instance_id(0), player.hand_instance_id(2)];

    let removed = player.remove_hand_card(1);

    assert_eq!(removed.map(|(_, card_id)| card_id), Some("b".to_string()));
    assert_eq!(player.hand, vec!["a", "c"]);
    assert_eq!(
        vec![player.hand_instance_id(0), player.hand_instance_id(1)],
        unused_before
    );
    player.draw(1);
    assert_eq!(player.hand, vec!["a", "c", "d"]);
}
