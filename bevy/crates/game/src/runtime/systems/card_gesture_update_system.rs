use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::runtime::components::{CardGestureView, DropTargetHint, HandCardGestureTarget};
use crate::runtime::resources::{
    ActiveView, CARD_GESTURE_DRAG_THRESHOLD, CardGestureModel, CardGestureSlotTarget,
    CardGestureState, CardInspectionDefaults, CardSlotBoardModel, CardSlotSide, CardStateModel,
};

use super::{
    active_pointer_position, drag_preview_transform, game_view_card_index_at,
    hand_source_transform, just_pressed_pointer_position, local_slots_area_hit_target,
    selected_inspection_transform, slot_transform, window_pointer_to_game_view,
};

/// HUMAN: Updates card gesture state from unified pointer input in GameView.
/// AI: This replaces GameView click-to-DeckBuilderScene navigation without touching DeckBuilderScene behavior.
pub fn card_gesture_update_system(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    active_view: Res<ActiveView>,
    card_defaults: Res<CardInspectionDefaults>,
    mut gesture_model: ResMut<CardGestureModel>,
    mut slot_board: ResMut<CardSlotBoardModel>,
    mut card_states: ResMut<CardStateModel>,
    mut card_query: Query<(&HandCardGestureTarget, &mut Visibility), With<CardGestureView>>,
) {
    if *active_view != ActiveView::GameView {
        return;
    }

    let Ok(primary_window) = primary_window_query.single() else {
        return;
    };
    let window_size = Vec2::new(
        primary_window.resolution.width(),
        primary_window.resolution.height(),
    );

    if let Some(pointer_position) =
        just_pressed_pointer_position(primary_window, &mouse_buttons, &touches)
    {
        handle_press(
            pointer_position,
            window_size,
            &card_defaults,
            &mut gesture_model,
            &card_states,
        );
    }

    if let Some(pointer_position) = active_pointer_position(primary_window, &touches)
        && let Some(game_view_position) = window_pointer_to_game_view(pointer_position, window_size)
    {
        handle_move(
            game_view_position,
            &card_defaults,
            &mut gesture_model,
            &mut card_states,
        );
    }

    if pointer_just_released(&mouse_buttons, &touches) {
        let game_view_position = active_pointer_position(primary_window, &touches)
            .and_then(|pointer_position| window_pointer_to_game_view(pointer_position, window_size))
            .or_else(|| {
                gesture_model
                    .pointer
                    .as_ref()
                    .map(|pointer| pointer.current_position)
            });
        handle_release(
            game_view_position,
            &card_defaults,
            &mut gesture_model,
            &mut slot_board,
            &mut card_states,
            &mut card_query,
        );
    }
}

/// HUMAN: Shows user-facing drop target hints for locations that can accept dragged cards.
/// AI: This is gameplay feedback, not debug drawing, and should not depend on DebugHUD state.
pub fn drop_target_hint_update_system(
    active_view: Res<ActiveView>,
    gesture_model: Res<CardGestureModel>,
    slot_board: Res<CardSlotBoardModel>,
    mut hint_query: Query<(&DropTargetHint, &mut Visibility)>,
) {
    let should_show =
        *active_view == ActiveView::GameView && gesture_model.state == CardGestureState::Dragging;

    for (hint, mut visibility) in &mut hint_query {
        *visibility =
            if should_show && slot_board.location_has_available_local_slot(hint.location_index) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
    }
}

fn handle_press(
    pointer_position: Vec2,
    window_size: Vec2,
    card_defaults: &CardInspectionDefaults,
    gesture_model: &mut CardGestureModel,
    card_states: &CardStateModel,
) {
    let Some(game_view_position) = window_pointer_to_game_view(pointer_position, window_size)
    else {
        return;
    };

    if gesture_model.state == CardGestureState::SelectedInspecting {
        gesture_model.return_to_source();
        return;
    }

    let Some(hand_index) = game_view_card_index_at(pointer_position, window_size) else {
        return;
    };
    if !card_states.is_draggable(hand_index) {
        return;
    }
    let source_transform = hand_source_transform(
        hand_index,
        card_states.len(),
        card_defaults,
    );
    let Some((card_min, card_max)) = super::game_view_card_hitboxes_for_count(card_states.len())
        .get(hand_index)
        .copied()
    else {
        return;
    };
    gesture_model.press(
        hand_index,
        game_view_position,
        (card_min + card_max) * 0.5,
        source_transform,
    );
}

fn handle_move(
    game_view_position: Vec2,
    card_defaults: &CardInspectionDefaults,
    gesture_model: &mut CardGestureModel,
    card_states: &mut CardStateModel,
) {
    let started_drag =
        gesture_model.update_pointer(game_view_position, CARD_GESTURE_DRAG_THRESHOLD);
    if started_drag
        && let Some(hand_index) = gesture_model.active_hand_index
        && !card_states.begin_drag(hand_index)
    {
        gesture_model.return_to_source();
        return;
    }
    if gesture_model.state == CardGestureState::Dragging {
        let Some(pointer) = gesture_model.pointer else {
            return;
        };
        let Some(source_transform) = gesture_model.source_transform else {
            return;
        };
        gesture_model.target_transform = Some(drag_preview_transform(
            pointer.current_card_center(),
            source_transform,
            card_defaults,
        ));
    }
}

fn handle_release(
    game_view_position: Option<Vec2>,
    card_defaults: &CardInspectionDefaults,
    gesture_model: &mut CardGestureModel,
    slot_board: &mut CardSlotBoardModel,
    card_states: &mut CardStateModel,
    card_query: &mut Query<(&HandCardGestureTarget, &mut Visibility), With<CardGestureView>>,
) {
    match gesture_model.state {
        CardGestureState::Pressed => {
            gesture_model.select(selected_inspection_transform(card_defaults));
        }
        CardGestureState::Dragging => {
            let Some(hand_index) = gesture_model.active_hand_index else {
                gesture_model.return_to_source();
                return;
            };
            let Some(game_view_position) = game_view_position else {
                gesture_model.return_to_source();
                card_states.return_to_hand(hand_index);
                return;
            };
            let Some(location_index) = local_slots_area_hit_target(game_view_position, slot_board)
            else {
                gesture_model.return_to_source();
                card_states.return_to_hand(hand_index);
                return;
            };

            if let Some(slot_index) = slot_board.place_next_local(location_index, hand_index) {
                let target_slot = CardGestureSlotTarget::new(
                    location_index,
                    CardSlotSide::LocalPlayer,
                    slot_index,
                );
                gesture_model.place(
                    target_slot,
                    slot_transform(
                        location_index,
                        slot_index,
                        CardSlotSide::LocalPlayer,
                        slot_board,
                        card_defaults,
                    ),
                );
                card_states.place_in_location(hand_index);
                hide_placed_hand_card(hand_index, card_query);
            } else {
                gesture_model.return_to_source();
                card_states.return_to_hand(hand_index);
            }
        }
        _ => {}
    }
}

fn hide_placed_hand_card(
    hand_index: usize,
    card_query: &mut Query<(&HandCardGestureTarget, &mut Visibility), With<CardGestureView>>,
) {
    for (target, mut visibility) in card_query.iter_mut() {
        if target.hand_index == hand_index {
            *visibility = Visibility::Visible;
        }
    }
}

fn pointer_just_released(mouse_buttons: &ButtonInput<MouseButton>, touches: &Touches) -> bool {
    mouse_buttons.just_released(MouseButton::Left) || touches.iter_just_released().next().is_some()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use crate::runtime::resources::{CardGestureDestination, CardSlotState, CardState};

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
                        &mut gesture,
                        &mut states,
                    );
                    handle_release(
                        None,
                        &defaults,
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
        {
            let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
            gesture.press(1, Vec2::ZERO, Vec2::ZERO, Transform::default());
            gesture.state = CardGestureState::Dragging;
        }

        app.world_mut()
            .run_system_once(
                move |card_defaults: Res<CardInspectionDefaults>,
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
                .map(|slot| slot.state),
            Some(CardSlotState::Populated { hand_index: 1 })
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
            .init_resource::<CardSlotBoardModel>()
            .add_systems(Update, drop_target_hint_update_system);
        for location_index in 0..3 {
            app.world_mut()
                .spawn((DropTargetHint::new(location_index), Visibility::Hidden));
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
    fn press_ignores_card_that_is_not_in_hand_state() {
        let defaults = CardInspectionDefaults::default();
        let mut gesture = CardGestureModel::default();
        let mut card_states = CardStateModel::default();
        assert!(card_states.place_in_location(2));
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
        );

        assert_eq!(gesture.state, CardGestureState::Idle);
        assert_eq!(gesture.active_hand_index, None);
    }

    fn test_app_with_gesture_card(hand_index: usize) -> App {
        let mut app = App::new();
        app.init_resource::<CardInspectionDefaults>()
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
}
