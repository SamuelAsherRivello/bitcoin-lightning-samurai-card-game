use bevy::ecs::system::RunSystemOnce;

use crate::runtime::resources::{
    CardGestureDestination, CardSlotState, CardState, PointerGestureModel,
};

use super::*;

#[test]
fn release_after_threshold_does_not_select_for_inspection() {
    let mut app = test_app_with_gesture_card(0);
    {
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        gesture.press(0, Vec2::ZERO, Vec2::ZERO, Transform::default());
    }

    app.world_mut()
            .run_system_once(
                |defaults: Res<CardInspectionDefaults>,
                 registry: Res<CardModelRegistry>,
                 hand: Res<GameHandModel>,
                 locations: Res<GameLocationModel>,
                 mut round: ResMut<GameRoundModel>,
                 mut gesture: ResMut<CardGestureModel>,
                 mut slots: ResMut<CardSlotBoardModel>,
                 mut states: ResMut<CardStateModel>,
                 mut cards: Query<
                    (&HandCardGestureTarget, &mut Visibility),
                    With<CardGestureView>,
                >| {
                    handle_move(
                        Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
                        &defaults,
                        Some(&registry),
                        Some(&hand),
                        Some(&round),
                        &mut gesture,
                        &mut states,
                    );
                    handle_release(
                        None,
                        &defaults,
                        Some(&registry),
                        Some(&hand),
                        Some(&locations),
                        Some(&mut round),
                        &mut gesture,
                        &mut slots,
                        &mut states,
                        &mut cards,
                    );
                },
            )
            .unwrap();

    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::Returning
    );
    assert_eq!(
        app.world()
            .resource::<CardGestureModel>()
            .resolved_destination,
        Some(CardGestureDestination::HandCardSlot { hand_index: 0 })
    );
    assert_eq!(
        app.world().resource::<CardStateModel>().state(0),
        Some(CardState::Hand)
    );
}

#[test]
fn drag_release_places_into_empty_local_slot() {
    let mut app = test_app_with_gesture_card(1);
    let card = app
        .world_mut()
        .query_filtered::<Entity, With<CardGestureView>>()
        .single(app.world())
        .unwrap();
    let drop_position = app
        .world()
        .resource::<CardSlotBoardModel>()
        .slot_rect(0, CardSlotSide::LocalPlayer, 0)
        .map(|rect| rect.center())
        .unwrap();
    {
        let source_transform =
            hand_source_transform(0, 1, app.world().resource::<CardInspectionDefaults>());
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        gesture.press(1, drop_position, drop_position, source_transform);
        gesture.state = CardGestureState::Dragging;
    }

    app.world_mut()
        .run_system_once(
            move |card_defaults: Res<CardInspectionDefaults>,
                  registry: Res<CardModelRegistry>,
                  hand: Res<GameHandModel>,
                  locations: Res<GameLocationModel>,
                  mut round: ResMut<GameRoundModel>,
                  mut gesture: ResMut<CardGestureModel>,
                  mut slots: ResMut<CardSlotBoardModel>,
                  mut states: ResMut<CardStateModel>,
                  mut cards: Query<
                (&HandCardGestureTarget, &mut Visibility),
                With<CardGestureView>,
            >| {
                super::handle_release(
                    Some(drop_position),
                    &card_defaults,
                    Some(&registry),
                    Some(&hand),
                    Some(&locations),
                    Some(&mut round),
                    &mut gesture,
                    &mut slots,
                    &mut states,
                    &mut cards,
                );
            },
        )
        .unwrap();

    let board = app.world().resource::<CardSlotBoardModel>();
    assert_eq!(
        board
            .slot(0, CardSlotSide::LocalPlayer, 0)
            .map(|slot| slot.state.clone()),
        Some(CardSlotState::Populated {
            hand_index: 1,
            card_id: String::new()
        })
    );
    assert!(app.world().get::<CardGestureView>(card).is_some());
    assert_eq!(
        app.world()
            .resource::<CardGestureModel>()
            .resolved_destination,
        Some(CardGestureDestination::LocationCardSlot {
            location_index: 0,
            slot_index: 0,
        })
    );
    assert_eq!(
        app.world().resource::<CardStateModel>().state(1),
        Some(CardState::Location)
    );
}

#[test]
fn drag_release_inside_full_location_returns_to_source() {
    let mut app = test_app_with_gesture_card(1);
    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        for hand_index in 0..4 {
            assert_eq!(slots.place_next_local(0, hand_index), Some(hand_index));
        }
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        gesture.press(1, Vec2::ZERO, Vec2::ZERO, Transform::default());
        gesture.state = CardGestureState::Dragging;
    }

    app.world_mut()
        .run_system_once(
            move |card_defaults: Res<CardInspectionDefaults>,
                  registry: Res<CardModelRegistry>,
                  hand: Res<GameHandModel>,
                  locations: Res<GameLocationModel>,
                  mut round: ResMut<GameRoundModel>,
                  mut gesture: ResMut<CardGestureModel>,
                  mut slots: ResMut<CardSlotBoardModel>,
                  mut states: ResMut<CardStateModel>,
                  mut cards: Query<
                (&HandCardGestureTarget, &mut Visibility),
                With<CardGestureView>,
            >| {
                let drop_position = slots
                    .slot_rect(0, CardSlotSide::LocalPlayer, 0)
                    .map(|rect| rect.center())
                    .unwrap();
                super::handle_release(
                    Some(drop_position),
                    &card_defaults,
                    Some(&registry),
                    Some(&hand),
                    Some(&locations),
                    Some(&mut round),
                    &mut gesture,
                    &mut slots,
                    &mut states,
                    &mut cards,
                );
            },
        )
        .unwrap();

    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::Returning
    );
    assert_eq!(
        app.world()
            .resource::<CardGestureModel>()
            .resolved_destination,
        Some(CardGestureDestination::HandCardSlot { hand_index: 1 })
    );
    assert_eq!(
        app.world().resource::<CardStateModel>().state(1),
        Some(CardState::Hand)
    );
}

#[test]
fn drop_target_hints_show_only_available_locations_while_dragging() {
    let mut app = App::new();
    app.init_resource::<ActiveView>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardSlotBoardModel>()
        .add_systems(Update, drop_target_hint_update_system);
    for location_index in 0..3 {
        app.world_mut().spawn((
            DropTargetHint::new(location_index),
            Visibility::Hidden,
            BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
            BackgroundColor(DROP_TARGET_GENERAL_BACKGROUND_COLOR),
        ));
    }
    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        for hand_index in 0..4 {
            assert_eq!(slots.place_next_local(1, hand_index), Some(hand_index));
        }
        app.world_mut().resource_mut::<CardGestureModel>().state = CardGestureState::Dragging;
    }

    app.update();

    let mut hints: Vec<(usize, Visibility)> = app
        .world_mut()
        .query::<(&DropTargetHint, &Visibility)>()
        .iter(app.world())
        .map(|(hint, visibility)| (hint.location_index, *visibility))
        .collect();
    hints.sort_by_key(|(location_index, _)| *location_index);
    assert_eq!(
        hints,
        vec![
            (0, Visibility::Visible),
            (1, Visibility::Hidden),
            (2, Visibility::Visible)
        ]
    );
}

#[test]
fn unaffordable_hand_card_can_drag_but_shows_no_drop_targets() {
    let mut app = App::new();
    app.init_resource::<ActiveView>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
        ]))
        .insert_resource(GameRoundModel {
            energy_available: 1,
            ..Default::default()
        })
        .insert_resource(CardStateModel::with_size(1))
        .add_systems(Update, drop_target_hint_update_system);
    for location_index in 0..3 {
        app.world_mut().spawn((
            DropTargetHint::new(location_index),
            Visibility::Visible,
            BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
            BackgroundColor(DROP_TARGET_GENERAL_BACKGROUND_COLOR),
        ));
    }
    {
        let defaults = CardInspectionDefaults::default();
        let mut gesture = app
            .world_mut()
            .remove_resource::<CardGestureModel>()
            .unwrap();
        let mut states = app.world_mut().remove_resource::<CardStateModel>().unwrap();
        assert!(gesture.press(0, Vec2::ZERO, Vec2::ZERO, Transform::default()));
        handle_move(
            Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
            &defaults,
            None,
            None,
            None,
            &mut gesture,
            &mut states,
        );
        app.insert_resource(gesture).insert_resource(states);
    }

    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::Dragging
    );
    assert_eq!(
        app.world().resource::<CardStateModel>().state(0),
        Some(CardState::Dragging)
    );

    app.update();

    let mut hints: Vec<(usize, Visibility)> = app
        .world_mut()
        .query::<(&DropTargetHint, &Visibility)>()
        .iter(app.world())
        .map(|(hint, visibility)| (hint.location_index, *visibility))
        .collect();
    hints.sort_by_key(|(location_index, _)| *location_index);
    assert_eq!(
        hints,
        vec![
            (0, Visibility::Hidden),
            (1, Visibility::Hidden),
            (2, Visibility::Hidden)
        ]
    );
}

#[test]
fn unaffordable_hand_card_release_over_slot_returns_to_hand() {
    let mut app = test_app_with_gesture_card(0);
    app.insert_resource(GameHandModel::new(vec![
        crate::runtime::resources::YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
    ]));
    app.insert_resource(GameRoundModel {
        energy_available: 1,
        ..Default::default()
    });
    {
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        assert!(gesture.press(0, Vec2::ZERO, Vec2::ZERO, Transform::default()));
        gesture.state = CardGestureState::Dragging;
    }
    app.world_mut()
        .resource_mut::<CardStateModel>()
        .begin_drag(0);

    app.world_mut()
        .run_system_once(
            |card_defaults: Res<CardInspectionDefaults>,
             registry: Res<CardModelRegistry>,
             hand: Res<GameHandModel>,
             locations: Res<GameLocationModel>,
             mut round: ResMut<GameRoundModel>,
             mut gesture: ResMut<CardGestureModel>,
             mut slots: ResMut<CardSlotBoardModel>,
             mut states: ResMut<CardStateModel>,
             mut cards: Query<
                (&HandCardGestureTarget, &mut Visibility),
                With<CardGestureView>,
            >| {
                let drop_position = slots
                    .slot_rect(0, CardSlotSide::LocalPlayer, 0)
                    .map(|rect| rect.center())
                    .unwrap();
                super::handle_release(
                    Some(drop_position),
                    &card_defaults,
                    Some(&registry),
                    Some(&hand),
                    Some(&locations),
                    Some(&mut round),
                    &mut gesture,
                    &mut slots,
                    &mut states,
                    &mut cards,
                );
            },
        )
        .unwrap();

    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::Returning
    );
    assert_eq!(
        app.world().resource::<CardStateModel>().state(0),
        Some(CardState::Hand)
    );
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        0
    );
    assert_eq!(app.world().resource::<GameRoundModel>().energy_available, 1);
}

#[test]
fn release_uses_card_overlap_target_instead_of_pointer_position() {
    let mut app = test_app_with_gesture_card(0);
    let drop_center = app
        .world()
        .resource::<CardSlotBoardModel>()
        .local_slots_area_rect(1)
        .map(|rect| rect.center())
        .unwrap();
    let pointer_position = Vec2::new(32.0, 320.0);
    {
        let defaults = app.world().resource::<CardInspectionDefaults>();
        let source_transform = hand_source_transform(0, 1, defaults);
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        gesture.state = CardGestureState::Dragging;
        gesture.active_hand_index = Some(0);
        gesture.pointer = Some(PointerGestureModel::new(pointer_position, drop_center));
        gesture.source_transform = Some(source_transform);
    }
    app.world_mut()
        .resource_mut::<CardStateModel>()
        .begin_drag(0);

    app.world_mut()
        .run_system_once(
            move |card_defaults: Res<CardInspectionDefaults>,
                  registry: Res<CardModelRegistry>,
                  hand: Res<GameHandModel>,
                  locations: Res<GameLocationModel>,
                  mut round: ResMut<GameRoundModel>,
                  mut gesture: ResMut<CardGestureModel>,
                  mut slots: ResMut<CardSlotBoardModel>,
                  mut states: ResMut<CardStateModel>,
                  mut cards: Query<
                (&HandCardGestureTarget, &mut Visibility),
                With<CardGestureView>,
            >| {
                super::handle_release(
                    Some(pointer_position),
                    &card_defaults,
                    Some(&registry),
                    Some(&hand),
                    Some(&locations),
                    Some(&mut round),
                    &mut gesture,
                    &mut slots,
                    &mut states,
                    &mut cards,
                );
            },
        )
        .unwrap();

    assert_eq!(
        app.world()
            .resource::<CardGestureModel>()
            .resolved_destination,
        Some(CardGestureDestination::LocationCardSlot {
            location_index: 1,
            slot_index: 0
        })
    );
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .slot(1, CardSlotSide::LocalPlayer, 0)
            .map(|slot| slot.state.clone()),
        Some(CardSlotState::Populated {
            hand_index: 0,
            card_id: String::new()
        })
    );
}

#[test]
fn drop_target_hint_focus_style_follows_largest_card_overlap() {
    let mut app = App::new();
    app.init_resource::<ActiveView>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardSlotBoardModel>()
        .add_systems(Update, drop_target_hint_update_system);
    for location_index in 0..3 {
        app.world_mut().spawn((
            DropTargetHint::new(location_index),
            Visibility::Hidden,
            BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
            BackgroundColor(DROP_TARGET_GENERAL_BACKGROUND_COLOR),
        ));
    }
    {
        let hover_position = app
            .world()
            .resource::<CardSlotBoardModel>()
            .local_slots_area_rect(2)
            .map(|rect| rect.center())
            .unwrap();
        let source_transform =
            hand_source_transform(0, 1, app.world().resource::<CardInspectionDefaults>());
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        gesture.state = CardGestureState::Dragging;
        gesture.source_transform = Some(source_transform);
        gesture.pointer = Some(PointerGestureModel::new(hover_position, hover_position));
    }

    app.update();

    let mut hints: Vec<(usize, Visibility, BorderColor, Color)> = app
        .world_mut()
        .query::<(&DropTargetHint, &Visibility, &BorderColor, &BackgroundColor)>()
        .iter(app.world())
        .map(|(hint, visibility, border, background)| {
            (hint.location_index, *visibility, *border, background.0)
        })
        .collect();
    hints.sort_by_key(|(location_index, _, _, _)| *location_index);
    assert_eq!(
        hints,
        vec![
            (
                0,
                Visibility::Visible,
                BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
                DROP_TARGET_GENERAL_BACKGROUND_COLOR
            ),
            (
                1,
                Visibility::Visible,
                BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
                DROP_TARGET_GENERAL_BACKGROUND_COLOR
            ),
            (
                2,
                Visibility::Visible,
                BorderColor::all(DROP_TARGET_CLOSE_BORDER_COLOR),
                DROP_TARGET_CLOSE_BACKGROUND_COLOR
            )
        ]
    );
}

#[test]
fn full_hovered_drop_target_stays_hidden_without_close_style() {
    let mut app = App::new();
    app.init_resource::<ActiveView>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardSlotBoardModel>()
        .add_systems(Update, drop_target_hint_update_system);
    for location_index in 0..3 {
        app.world_mut().spawn((
            DropTargetHint::new(location_index),
            Visibility::Hidden,
            BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
            BackgroundColor(DROP_TARGET_GENERAL_BACKGROUND_COLOR),
        ));
    }
    let hover_position = {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        for hand_index in 0..4 {
            assert_eq!(slots.place_next_local(1, hand_index), Some(hand_index));
        }
        slots
            .local_slots_area_rect(1)
            .map(|rect| rect.center())
            .unwrap()
    };
    {
        let source_transform =
            hand_source_transform(0, 1, app.world().resource::<CardInspectionDefaults>());
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        gesture.state = CardGestureState::Dragging;
        gesture.source_transform = Some(source_transform);
        gesture.pointer = Some(PointerGestureModel::new(hover_position, hover_position));
    }

    app.update();

    let full_hint = app
        .world_mut()
        .query::<(&DropTargetHint, &Visibility, &BorderColor, &BackgroundColor)>()
        .iter(app.world())
        .find(|(hint, _, _, _)| hint.location_index == 1)
        .map(|(_, visibility, border, background)| (*visibility, *border, background.0))
        .unwrap();
    assert_eq!(
        full_hint,
        (
            Visibility::Hidden,
            BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR),
            DROP_TARGET_GENERAL_BACKGROUND_COLOR
        )
    );
}

#[test]
fn release_while_returning_does_not_clear_until_animation_settles() {
    let mut app = test_app_with_gesture_card(0);
    {
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        assert!(gesture.press(
            0,
            Vec2::ZERO,
            Vec2::ZERO,
            Transform::from_scale(Vec3::splat(0.5)),
        ));
        gesture.return_to_source();
    }

    app.world_mut()
            .run_system_once(
                |defaults: Res<CardInspectionDefaults>,
                 registry: Res<CardModelRegistry>,
                 hand: Res<GameHandModel>,
                 locations: Res<GameLocationModel>,
                 mut round: ResMut<GameRoundModel>,
                 mut gesture: ResMut<CardGestureModel>,
                 mut slots: ResMut<CardSlotBoardModel>,
                 mut states: ResMut<CardStateModel>,
                 mut cards: Query<
                    (&HandCardGestureTarget, &mut Visibility),
                    With<CardGestureView>,
                >| {
                    super::handle_release(
                        Some(Vec2::ZERO),
                        &defaults,
                        Some(&registry),
                        Some(&hand),
                        Some(&locations),
                        Some(&mut round),
                        &mut gesture,
                        &mut slots,
                        &mut states,
                        &mut cards,
                    );
                },
            )
            .unwrap();

    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::Returning
    );
    assert_eq!(
        app.world()
            .resource::<CardGestureModel>()
            .target_transform
            .map(|transform| transform.scale),
        Some(Vec3::splat(0.5))
    );
}

#[test]
fn press_anywhere_dismisses_selected_card_before_safe_area_hit_testing() {
    let defaults = CardInspectionDefaults::default();
    let mut gesture = CardGestureModel::default();
    let card_states = CardStateModel::default();
    assert!(gesture.press(
        0,
        Vec2::ZERO,
        Vec2::ZERO,
        Transform::from_scale(Vec3::splat(0.5)),
    ));
    gesture.select(Transform::from_scale(Vec3::splat(2.0)));

    handle_press(
        Vec2::new(-50.0, -50.0),
        Vec2::new(1280.0, 800.0),
        &defaults,
        &mut gesture,
        &card_states,
        &CardSlotBoardModel::default(),
    );

    assert_eq!(gesture.state, CardGestureState::Returning);
    assert_eq!(
        gesture.target_transform.map(|transform| transform.scale),
        Some(Vec3::splat(0.5))
    );
}

#[test]
fn press_ignores_card_that_is_not_in_hand_state() {
    let defaults = CardInspectionDefaults::default();
    let mut gesture = CardGestureModel::default();
    let mut card_states = CardStateModel::default();
    for hand_index in 0..card_states.len() {
        assert!(card_states.place_in_location(hand_index));
    }
    let window_size = Vec2::new(1280.0, 800.0);
    let pointer_position = super::super::game_view_pointer_to_window(
        (super::super::game_view_card_hitboxes()[2].0
            + super::super::game_view_card_hitboxes()[2].1)
            * 0.5,
        window_size,
    );

    handle_press(
        pointer_position,
        window_size,
        &defaults,
        &mut gesture,
        &card_states,
        &CardSlotBoardModel::default(),
    );

    assert_eq!(gesture.state, CardGestureState::Idle);
    assert_eq!(gesture.active_hand_index, None);
}

#[test]
fn press_selects_locked_location_card_for_inspection() {
    let defaults = CardInspectionDefaults::default();
    let mut gesture = CardGestureModel::default();
    let mut card_states = CardStateModel::default();
    let mut slot_board = CardSlotBoardModel::default();
    assert!(slot_board.place_local(1, CardSlotSide::LocalPlayer, 2, 0));
    assert!(card_states.place_in_location(0));
    card_states.lock_location_cards();
    let window_size = Vec2::new(1280.0, 800.0);
    let game_view_position = slot_board
        .slot_rect(1, CardSlotSide::LocalPlayer, 2)
        .map(|rect| rect.center())
        .unwrap();
    let pointer_position =
        super::super::game_view_pointer_to_window(game_view_position, window_size);

    handle_press(
        pointer_position,
        window_size,
        &defaults,
        &mut gesture,
        &card_states,
        &slot_board,
    );

    assert_eq!(gesture.state, CardGestureState::Pressed);
    assert_eq!(gesture.active_hand_index, Some(0));
    assert_eq!(
        gesture.source_transform,
        Some(slot_transform(
            1,
            2,
            CardSlotSide::LocalPlayer,
            &slot_board,
            &defaults,
        ))
    );
}

#[test]
fn locked_location_card_drag_returns_to_source() {
    let defaults = CardInspectionDefaults::default();
    let mut gesture = CardGestureModel::default();
    let mut card_states = CardStateModel::default();
    let mut slot_board = CardSlotBoardModel::default();
    assert!(slot_board.place_local(1, CardSlotSide::LocalPlayer, 2, 0));
    assert!(card_states.place_in_location(0));
    card_states.lock_location_cards();
    let source_transform = slot_transform(1, 2, CardSlotSide::LocalPlayer, &slot_board, &defaults);
    assert!(gesture.press(0, Vec2::ZERO, Vec2::ZERO, source_transform));

    handle_move(
        Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
        &defaults,
        None,
        None,
        None,
        &mut gesture,
        &mut card_states,
    );

    assert_eq!(gesture.state, CardGestureState::Returning);
    assert_eq!(gesture.target_transform, Some(source_transform));
    assert_eq!(
        gesture.resolved_destination,
        Some(CardGestureDestination::HandCardSlot { hand_index: 0 })
    );
    assert_eq!(card_states.state(0), Some(CardState::LocationLocked));
}

fn test_app_with_gesture_card(hand_index: usize) -> App {
    let mut app = App::new();
    app.init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>();
    app.world_mut().spawn((
        HandCardGestureTarget::new(hand_index),
        CardGestureView,
        Visibility::Visible,
    ));
    app
}
