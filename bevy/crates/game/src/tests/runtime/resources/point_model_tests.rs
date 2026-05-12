use super::*;

#[test]
fn point_models_render_display_contract_values() {
    for value in [POINT_VIEW_DISPLAY_MIN, 0, POINT_VIEW_DISPLAY_MAX] {
        let cost = CostPointModel::new(value);
        let power = PowerPointModel::new(value);

        assert!(cost.is_in_display_contract());
        assert!(power.is_in_display_contract());
        assert_eq!(cost.display_text(), value.to_string());
        assert_eq!(power.display_text(), value.to_string());
    }

    assert!(!CostPointModel::new(POINT_VIEW_DISPLAY_MIN - 1).is_in_display_contract());
    assert!(!PowerPointModel::new(POINT_VIEW_DISPLAY_MAX + 1).is_in_display_contract());
}

#[test]
fn random_point_models_stay_inside_display_contract() {
    for _ in 0..256 {
        let cost = CostPointModel::random();
        let power = PowerPointModel::random();

        assert!(cost.is_in_display_contract());
        assert!(power.is_in_display_contract());
    }
}

#[test]
fn card_instance_effective_power_keeps_base_power_separate() {
    let card = CardInstanceModel::new(
        "sample",
        PlayerSide::Local,
        0,
        true,
        PowerPointModel::new(3),
    )
    .with_effective_power_delta(PowerPointModel::new(2));

    assert_eq!(card.base_power, PowerPointModel::new(3));
    assert_eq!(card.effective_power(), PowerPointModel::new(5));
}

#[test]
fn card_cost_never_contributes_to_location_total() {
    let cost = CostPointModel::new(9);
    let card = CardInstanceModel::new(
        "sample",
        PlayerSide::Local,
        0,
        true,
        PowerPointModel::new(2),
    );

    let location = LocationScoreModel::from_cards(
        0,
        &[card],
        PowerPointModel::new(0),
        PowerPointModel::new(0),
    )
    .unwrap();

    assert_eq!(cost.value, 9);
    assert_eq!(location.local_total, PowerPointModel::new(2));
}

#[test]
fn location_total_uses_revealed_effective_power_and_modifiers() {
    let cards = [
        CardInstanceModel::new(
            "local_revealed",
            PlayerSide::Local,
            0,
            true,
            PowerPointModel::new(3),
        )
        .with_effective_power_delta(PowerPointModel::new(2)),
        CardInstanceModel::new(
            "local_hidden",
            PlayerSide::Local,
            0,
            false,
            PowerPointModel::new(99),
        ),
        CardInstanceModel::new(
            "opponent_revealed",
            PlayerSide::Opponent,
            0,
            true,
            PowerPointModel::new(4),
        ),
    ];

    let location = LocationScoreModel::from_cards(
        0,
        &cards,
        PowerPointModel::new(-1),
        PowerPointModel::new(3),
    )
    .unwrap();

    assert_eq!(location.local_total, PowerPointModel::new(4));
    assert_eq!(location.opponent_total, PowerPointModel::new(7));
}

#[test]
fn moved_card_contributes_only_to_current_location() {
    let moved_card =
        CardInstanceModel::new("moved", PlayerSide::Local, 1, true, PowerPointModel::new(5));

    let old_location = LocationScoreModel::from_cards(
        0,
        std::slice::from_ref(&moved_card),
        PowerPointModel::new(0),
        PowerPointModel::new(0),
    )
    .unwrap();
    let new_location = LocationScoreModel::from_cards(
        1,
        &[moved_card],
        PowerPointModel::new(0),
        PowerPointModel::new(0),
    )
    .unwrap();

    assert_eq!(old_location.local_total, PowerPointModel::new(0));
    assert_eq!(new_location.local_total, PowerPointModel::new(5));
}

#[test]
fn location_capacity_rejects_more_than_four_cards_per_player() {
    let cards = [
        CardInstanceModel::new("a", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
        CardInstanceModel::new("b", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
        CardInstanceModel::new("c", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
        CardInstanceModel::new("d", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
        CardInstanceModel::new("e", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
    ];

    assert_eq!(
        LocationScoreModel::from_cards(0, &cards, PowerPointModel::new(0), PowerPointModel::new(0),),
        Err(LocationScoreError::TooManyCards {
            owner: PlayerSide::Local,
            location_index: 0,
            count: 5,
            capacity: DEFAULT_LOCATION_CARD_CAPACITY_PER_PLAYER,
        })
    );
}

#[test]
fn location_control_covers_leads_ties_and_empty_locations() {
    let local_lead = LocationScoreModel {
        local_total: PowerPointModel::new(5),
        opponent_total: PowerPointModel::new(3),
        ..LocationScoreModel::empty(0)
    };
    let opponent_lead = LocationScoreModel {
        local_total: PowerPointModel::new(2),
        opponent_total: PowerPointModel::new(7),
        ..LocationScoreModel::empty(1)
    };
    let tied = LocationScoreModel {
        local_total: PowerPointModel::new(4),
        opponent_total: PowerPointModel::new(4),
        ..LocationScoreModel::empty(2)
    };

    assert_eq!(local_lead.control().controller, LocationController::Local);
    assert_eq!(
        opponent_lead.control().controller,
        LocationController::Opponent
    );
    assert_eq!(tied.control().controller, LocationController::None);
    assert_eq!(
        LocationScoreModel::empty(0).control().controller,
        LocationController::None
    );
}

#[test]
fn match_outcome_uses_location_count_before_total_power() {
    let match_score = MatchScoreModel::new([
        LocationScoreModel {
            local_total: PowerPointModel::new(1),
            opponent_total: PowerPointModel::new(0),
            ..LocationScoreModel::empty(0)
        },
        LocationScoreModel {
            local_total: PowerPointModel::new(1),
            opponent_total: PowerPointModel::new(0),
            ..LocationScoreModel::empty(1)
        },
        LocationScoreModel {
            local_total: PowerPointModel::new(0),
            opponent_total: PowerPointModel::new(20),
            ..LocationScoreModel::empty(2)
        },
    ]);

    let outcome = match_score.outcome();

    assert_eq!(outcome.result, MatchOutcome::LocalWin);
    assert_eq!(outcome.local_controlled_count, 2);
    assert_eq!(outcome.opponent_controlled_count, 1);
}

#[test]
fn match_outcome_uses_total_power_tiebreaker_then_draw() {
    let local_tiebreak = MatchScoreModel::new([
        LocationScoreModel {
            local_total: PowerPointModel::new(8),
            opponent_total: PowerPointModel::new(1),
            ..LocationScoreModel::empty(0)
        },
        LocationScoreModel {
            local_total: PowerPointModel::new(1),
            opponent_total: PowerPointModel::new(6),
            ..LocationScoreModel::empty(1)
        },
        LocationScoreModel {
            local_total: PowerPointModel::new(0),
            opponent_total: PowerPointModel::new(0),
            ..LocationScoreModel::empty(2)
        },
    ]);
    let draw = MatchScoreModel::new([
        LocationScoreModel {
            local_total: PowerPointModel::new(5),
            opponent_total: PowerPointModel::new(3),
            ..LocationScoreModel::empty(0)
        },
        LocationScoreModel {
            local_total: PowerPointModel::new(1),
            opponent_total: PowerPointModel::new(3),
            ..LocationScoreModel::empty(1)
        },
        LocationScoreModel {
            local_total: PowerPointModel::new(0),
            opponent_total: PowerPointModel::new(0),
            ..LocationScoreModel::empty(2)
        },
    ]);

    assert_eq!(local_tiebreak.outcome().result, MatchOutcome::LocalWin);
    assert_eq!(draw.outcome().result, MatchOutcome::Draw);
}
