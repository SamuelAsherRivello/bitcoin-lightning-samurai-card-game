use super::*;

#[test]
fn board_has_three_locations_and_twenty_four_slots() {
    let board = CardSlotBoardModel::default();

    assert_eq!(board.slot_count(), CARD_SLOT_TOTAL_COUNT);
    assert_eq!(
        board.local_direct_placement_count(),
        CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT
    );
    for location_index in 0..CARD_SLOT_LOCATION_COUNT {
        for side in [CardSlotSide::Opponent, CardSlotSide::LocalPlayer] {
            for slot_index in 0..CARD_SLOT_ROW_COUNT {
                assert!(board.slot(location_index, side, slot_index).is_some());
            }
        }
    }
}

#[test]
fn slot_rects_match_debug_drawn_reference_lines() {
    let board = CardSlotBoardModel::default();

    assert_eq!(
        board.slot_rect(1, CardSlotSide::Opponent, 0),
        Some(CardSlotRect::new(548.0, 44.0, 92.0, 90.0))
    );
    assert_eq!(
        board.slot_rect(1, CardSlotSide::Opponent, 3),
        Some(CardSlotRect::new(640.0, 134.0, 92.0, 90.0))
    );
    assert_eq!(
        board.slot_rect(2, CardSlotSide::LocalPlayer, 3),
        Some(CardSlotRect::new(824.0, 522.0, 92.0, 90.0))
    );
    assert_eq!(
        board.local_slots_area_rect(0),
        Some(CardSlotRect::new(364.0, 432.0, 184.0, 180.0))
    );
    assert_eq!(
        board.location_area_rect(0),
        Some(CardSlotRect::new(364.0, 224.0, 184.0, 208.0))
    );
    assert_eq!(
        board.location_area_rect(1),
        Some(CardSlotRect::new(548.0, 224.0, 184.0, 208.0))
    );
    assert_eq!(
        board.location_area_rect(2),
        Some(CardSlotRect::new(732.0, 224.0, 184.0, 208.0))
    );
    assert_eq!(board.location_area_rect(99), None);
}

#[test]
fn only_empty_local_slots_accept_direct_placement() {
    let mut board = CardSlotBoardModel::default();

    assert!(board.can_place_local(0, CardSlotSide::LocalPlayer, 0));
    assert!(!board.can_place_local(0, CardSlotSide::Opponent, 0));
    assert!(board.place_local(0, CardSlotSide::LocalPlayer, 0, 2));
    assert!(!board.can_place_local(0, CardSlotSide::LocalPlayer, 0));
    assert_eq!(board.populated_count(), 1);
}

#[test]
fn valid_local_placement_covers_all_twelve_local_slots() {
    let mut board = CardSlotBoardModel::default();
    let mut placed = 0;

    for location_index in 0..CARD_SLOT_LOCATION_COUNT {
        for slot_index in 0..CARD_SLOT_ROW_COUNT {
            assert!(board.place_local(
                location_index,
                CardSlotSide::LocalPlayer,
                slot_index,
                placed
            ));
            placed += 1;
        }
    }

    assert_eq!(placed, CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT);
    assert_eq!(
        board.populated_count(),
        CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT
    );
}

#[test]
fn next_available_local_slot_uses_upper_left_upper_right_lower_left_lower_right_order() {
    let mut board = CardSlotBoardModel::default();

    assert_eq!(board.next_available_local_slot(1), Some(0));
    assert_eq!(board.place_next_local(1, 10), Some(0));
    assert_eq!(board.place_next_local(1, 11), Some(1));
    assert_eq!(board.place_next_local(1, 12), Some(2));
    assert_eq!(board.place_next_local(1, 13), Some(3));
    assert_eq!(board.next_available_local_slot(1), None);
    assert!(!board.location_has_available_local_slot(1));
}

#[test]
fn replacing_a_local_card_moves_it_instead_of_duplicating_slots() {
    let mut board = CardSlotBoardModel::default();

    assert_eq!(board.place_next_local(0, 7), Some(0));
    assert_eq!(board.local_slot_for_card(7), Some((0, 0)));
    assert_eq!(board.place_next_local(1, 7), Some(0));

    assert_eq!(board.populated_count(), 1);
    assert_eq!(board.local_slot_for_card(7), Some((1, 0)));
    assert_eq!(
        board.slot(0, CardSlotSide::LocalPlayer, 0).unwrap().state,
        CardSlotState::Empty
    );
}

#[test]
fn opponent_populated_and_missing_slots_reject_placement() {
    let mut board = CardSlotBoardModel::default();

    assert!(!board.place_local(0, CardSlotSide::Opponent, 0, 0));
    assert!(board.place_local(0, CardSlotSide::LocalPlayer, 0, 0));
    assert!(!board.place_local(0, CardSlotSide::LocalPlayer, 0, 1));
    assert!(!board.place_local(99, CardSlotSide::LocalPlayer, 0, 1));
    assert_eq!(board.populated_count(), 1);
}

#[test]
fn card_state_allows_drag_from_hand_and_current_round_location_only() {
    let mut states = CardStateModel::default();

    assert!(states.is_draggable(0));
    assert!(states.begin_drag(0));
    assert_eq!(states.state(0), Some(CardState::Dragging));
    assert!(!states.is_draggable(0));
    assert!(!states.begin_drag(0));
    assert!(states.return_to_hand(0));
    assert!(states.is_draggable(0));
    assert!(states.place_in_location(0));
    assert_eq!(states.state(0), Some(CardState::Location));
    assert!(states.is_draggable(0));
    states.lock_location_cards();
    assert_eq!(states.state(0), Some(CardState::LocationLocked));
    assert!(!states.is_draggable(0));
}

#[test]
fn card_state_tracks_reordered_hand_layout() {
    let mut states = CardStateModel::with_size(4);

    assert!(states.begin_drag(1));
    assert_eq!(states.indices_with_state(CardState::Hand), vec![0, 2, 3]);
    assert!(states.return_to_hand_at_order(1, 3));
    assert_eq!(states.indices_with_state(CardState::Hand), vec![0, 2, 3, 1]);
    assert_eq!(states.hand_index_at_order(2), Some(3));
}
