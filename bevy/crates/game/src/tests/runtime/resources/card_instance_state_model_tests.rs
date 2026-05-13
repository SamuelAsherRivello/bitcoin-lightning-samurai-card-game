use super::*;

#[test]
fn current_inventory_names_existing_card_state_axes() {
    let inventory = CardStateAxisModel::current_inventory();

    assert!(inventory.iter().any(|axis| axis.values.contains(&"Front")));
    assert!(inventory.iter().any(|axis| axis.values.contains(&"Back")));
    assert!(inventory.iter().any(|axis| axis.values.contains(&"Hand")));
    assert!(
        inventory
            .iter()
            .any(|axis| axis.values.contains(&"Dragging"))
    );
    assert!(
        inventory
            .iter()
            .any(|axis| axis.values.contains(&"CurrentTurnHidden"))
    );
}

#[test]
fn current_inventory_maps_axes_to_source_owners() {
    assert_eq!(
        CardStateAxisModel::by_axis("visual_root").map(|axis| axis.owner),
        Some("CardViewBundle/CardView")
    );
    assert_eq!(
        CardStateAxisModel::by_axis("interaction").map(|axis| axis.owner),
        Some("CardGestureModel/CardGestureState")
    );
    assert_eq!(
        CardStateAxisModel::by_axis("cpu_presentation").map(|axis| axis.owner),
        Some("CpuHandCardView/CpuPlacedCardView/CpuPlacedCardAnimation")
    );
}

#[test]
fn card_instance_state_validates_single_zone_and_hidden_location_rule() {
    let valid = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Location {
            location_index: 0,
            side: CardSlotSide::LocalPlayer,
            slot_index: 0,
            lock_state: LocationLockState::CurrentTurnMovable,
        },
        CardRevealPolicy::CurrentTurnHiddenToOpponent,
    );

    assert_eq!(valid.validate(), Ok(()));

    let invalid = CardInstanceStateModel::new(
        CardInstanceId::new(2),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Hand { order_index: 0 },
        CardRevealPolicy::CurrentTurnHiddenToOpponent,
    );

    assert_eq!(
        invalid.validate(),
        Err(CardStateValidationError::HiddenCardOutsideLocation {
            instance_id: CardInstanceId::new(2)
        })
    );
}

#[test]
fn collection_rejects_duplicate_instance_ids() {
    let card = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Hand { order_index: 0 },
        CardRevealPolicy::OwnerVisible,
    );
    let collection = CardInstanceStateCollectionModel::new(vec![card.clone(), card]);

    assert_eq!(
        collection.validate(),
        Err(CardStateValidationError::DuplicateInstanceId {
            instance_id: CardInstanceId::new(1)
        })
    );
}

#[test]
fn reveal_policy_derives_front_for_owner_and_back_for_hidden_opponent() {
    let owner = MatchPlayerSide::Near;

    assert_eq!(
        CardRevealPolicy::CurrentTurnHiddenToOpponent.visible_face(
            MatchPlayerSide::Near,
            owner,
            CardFace::Front
        ),
        CardFace::Front
    );
    assert_eq!(
        CardRevealPolicy::CurrentTurnHiddenToOpponent.visible_face(
            MatchPlayerSide::Far,
            owner,
            CardFace::Front
        ),
        CardFace::Back
    );
    assert_eq!(
        CardRevealPolicy::RevealedToAll.visible_face(MatchPlayerSide::Far, owner, CardFace::Front),
        CardFace::Front
    );
}

#[test]
fn card_view_state_derives_pose_face_and_affordance() {
    let card = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Hand { order_index: 0 },
        CardRevealPolicy::OwnerVisible,
    );

    let view = CardViewStateModel::derive_for_viewer(&card, MatchPlayerSide::Near, None);

    assert_eq!(view.visible_face, CardFace::Front);
    assert_eq!(view.pose, CardViewPoseModel::Hand);
    assert_eq!(view.z_band, CardViewZBand::Hand);
    assert_eq!(view.input_affordance, CardInputAffordance::Draggable);
}

#[test]
fn active_drag_interaction_overrides_view_pose_and_affordance() {
    let card = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Hand { order_index: 0 },
        CardRevealPolicy::OwnerVisible,
    );
    let interaction =
        CardInteractionModel::active(CardInteractionState::Dragging, card.instance_id, card.zone);

    assert_eq!(interaction.validate_for_card(&card), Ok(()));

    let view =
        CardViewStateModel::derive_for_viewer(&card, MatchPlayerSide::Near, Some(&interaction));

    assert_eq!(view.pose, CardViewPoseModel::DragPreview);
    assert_eq!(view.z_band, CardViewZBand::Drag);
    assert_eq!(view.input_affordance, CardInputAffordance::None);
}

#[test]
fn locked_location_card_rejects_drag_interaction() {
    let card = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Location {
            location_index: 0,
            side: CardSlotSide::LocalPlayer,
            slot_index: 0,
            lock_state: LocationLockState::Locked,
        },
        CardRevealPolicy::RevealedToAll,
    );
    let interaction =
        CardInteractionModel::active(CardInteractionState::Dragging, card.instance_id, card.zone);

    assert_eq!(
        interaction.validate_for_card(&card),
        Err(CardStateValidationError::IllegalInteractionForZone {
            instance_id: card.instance_id,
            state: CardInteractionState::Dragging,
            zone: card.zone
        })
    );
}

#[test]
fn collection_finds_cards_by_owner_zone_and_slot() {
    let near_card = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Hand { order_index: 0 },
        CardRevealPolicy::OwnerVisible,
    );
    let far_card = CardInstanceStateModel::new(
        CardInstanceId::new(2),
        "lord_daichi",
        CardOwnerModel::far(),
        CardZoneModel::Location {
            location_index: 2,
            side: CardSlotSide::Opponent,
            slot_index: 1,
            lock_state: LocationLockState::Locked,
        },
        CardRevealPolicy::RevealedToAll,
    );
    let collection = CardInstanceStateCollectionModel::new(vec![near_card, far_card]);

    assert_eq!(collection.by_owner(CardOwnerModel::near()).len(), 1);
    assert_eq!(collection.by_zone_kind(CardZoneKind::Hand).len(), 1);
    assert_eq!(
        collection
            .at_slot(2, CardSlotSide::Opponent, 1)
            .map(|card| card.card_model_id.as_str()),
        Some("lord_daichi")
    );
}

#[test]
fn slot_occupancy_validation_requires_matching_location_zone() {
    let card = CardInstanceStateModel::new(
        CardInstanceId::new(1),
        "kage_ren",
        CardOwnerModel::near(),
        CardZoneModel::Location {
            location_index: 1,
            side: CardSlotSide::LocalPlayer,
            slot_index: 2,
            lock_state: LocationLockState::CurrentTurnMovable,
        },
        CardRevealPolicy::OwnerVisible,
    );
    let collection = CardInstanceStateCollectionModel::new(vec![card]);

    assert_eq!(
        collection.validate_slot_occupancy(&CardPlacementModel {
            instance_id: CardInstanceId::new(1),
            location_index: 1,
            side: CardSlotSide::LocalPlayer,
            slot_index: 2,
            placed_turn: 1,
        }),
        Ok(())
    );
    assert_eq!(
        collection.validate_slot_occupancy(&CardPlacementModel {
            instance_id: CardInstanceId::new(1),
            location_index: 1,
            side: CardSlotSide::LocalPlayer,
            slot_index: 3,
            placed_turn: 1,
        }),
        Err(CardStateValidationError::SlotMismatch {
            instance_id: CardInstanceId::new(1)
        })
    );
}

#[test]
fn local_adapter_maps_hand_and_location_state_from_existing_models() {
    let hand = GameHandModel::new(vec!["kage_ren".to_string(), "lord_daichi".to_string()]);
    let mut states = CardStateModel::with_size(2);
    let mut board = CardSlotBoardModel::default();
    assert!(states.place_in_location(1));
    assert!(board.place_local_with_card_id(1, CardSlotSide::LocalPlayer, 2, 1, "lord_daichi"));

    let collection = local_instances_from_existing_state(&hand, &states, &board);

    assert_eq!(collection.cards.len(), 2);
    assert_eq!(
        collection.cards[0].zone,
        CardZoneModel::Hand { order_index: 0 }
    );
    assert_eq!(
        collection.cards[1].zone,
        CardZoneModel::Location {
            location_index: 1,
            side: CardSlotSide::LocalPlayer,
            slot_index: 2,
            lock_state: LocationLockState::CurrentTurnMovable,
        }
    );
}

#[test]
fn cpu_adapters_map_passive_hand_and_placed_reveal_semantics() {
    let hand_view = CpuHandCardView::new(MatchPlayerSide::Far, 3, "kage_ren", CardFace::Back);
    let hand_state = instance_from_cpu_hand_view(&hand_view);

    assert_eq!(hand_state.owner, CardOwnerModel::far());
    assert_eq!(hand_state.zone, CardZoneModel::Hand { order_index: 3 });

    let placed_view = CpuPlacedCardView::new(
        MatchPlayerSide::Far,
        CardSlotSide::Opponent,
        2,
        1,
        "lord_daichi",
        CardFace::Back,
    );
    let placed_state =
        instance_from_cpu_placed_view(&placed_view, Some(PlacementVisibility::CurrentTurnHidden));

    assert_eq!(
        placed_state.reveal_policy,
        CardRevealPolicy::CurrentTurnHiddenToOpponent
    );
    assert_eq!(
        CardViewStateModel::derive_for_viewer(&placed_state, MatchPlayerSide::Near, None)
            .visible_face,
        CardFace::Back
    );
}

#[test]
fn placement_visibility_adapter_preserves_revealed_state() {
    let placement = PlacementVisibilityModel {
        owner: MatchPlayerSide::Far,
        location_index: 0,
        slot_index: 0,
        placement_turn: 1,
        visibility: PlacementVisibility::Revealed,
    };

    assert_eq!(
        reveal_policy_from_placement(&placement),
        CardRevealPolicy::RevealedToAll
    );
}
