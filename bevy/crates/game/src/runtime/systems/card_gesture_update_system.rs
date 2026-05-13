use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::runtime::components::{CardGestureView, DropTargetHint, HandCardGestureTarget};
use crate::runtime::resources::{
    ActiveView, CARD_GESTURE_DRAG_THRESHOLD, CardGestureModel, CardGestureSlotTarget,
    CardGestureState, CardInspectionDefaults, CardModelRegistry, CardSlotBoardModel, CardSlotSide,
    CardState, CardStateModel, CurrentRoundMoveRecord, GameHandModel, GameLocationModel,
    GameRoundModel, OpponentMatchModel,
};

use super::{
    active_pointer_position, drag_preview_transform, game_view_card_index_at_for_count,
    hand_insertion_index, hand_source_transform, just_pressed_pointer_position,
    local_slots_area_hit_target, selected_inspection_transform, slot_transform,
    window_pointer_to_game_view,
};

const DROP_TARGET_GENERAL_BORDER_COLOR: Color = Color::srgb(0.48, 0.82, 1.0);
const DROP_TARGET_GENERAL_BACKGROUND_COLOR: Color = Color::srgba(0.28, 0.72, 1.0, 0.12);
const DROP_TARGET_CLOSE_BORDER_COLOR: Color = Color::srgb(0.72, 0.94, 1.0);
const DROP_TARGET_CLOSE_BACKGROUND_COLOR: Color = Color::srgba(0.36, 0.86, 1.0, 0.24);

/// HUMAN: Updates card gesture state from unified pointer input in GameView.
/// AI: This replaces GameView click-to-DeckBuilderScene navigation without touching DeckBuilderScene behavior.
pub fn card_gesture_update_system(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    active_view: Res<ActiveView>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Option<Res<CardModelRegistry>>,
    game_hand_model: Option<Res<GameHandModel>>,
    game_location_model: Option<Res<GameLocationModel>>,
    opponent_match_model: Option<Res<OpponentMatchModel>>,
    mut game_round_model: Option<ResMut<GameRoundModel>>,
    mut gesture_model: ResMut<CardGestureModel>,
    mut slot_board: ResMut<CardSlotBoardModel>,
    mut card_states: ResMut<CardStateModel>,
    mut card_query: Query<(&HandCardGestureTarget, &mut Visibility), With<CardGestureView>>,
) {
    if *active_view != ActiveView::GameView {
        return;
    }
    if opponent_match_model
        .as_deref()
        .is_some_and(|model| model.near.controller.is_cpu())
    {
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
            &slot_board,
        );
    }

    if let Some(pointer_position) = active_pointer_position(primary_window, &touches)
        && let Some(game_view_position) = window_pointer_to_game_view(pointer_position, window_size)
    {
        handle_move(
            game_view_position,
            &card_defaults,
            card_model_registry.as_deref(),
            game_hand_model.as_deref(),
            game_round_model.as_deref(),
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
            card_model_registry.as_deref(),
            game_hand_model.as_deref(),
            game_location_model.as_deref(),
            game_round_model.as_deref_mut(),
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
    mut hint_query: Query<(
        &DropTargetHint,
        &mut Visibility,
        &mut BorderColor,
        &mut BackgroundColor,
    )>,
) {
    let should_show =
        *active_view == ActiveView::GameView && gesture_model.state == CardGestureState::Dragging;
    let close_location_index = should_show
        .then(|| {
            gesture_model.pointer.and_then(|pointer| {
                local_slots_area_hit_target(pointer.current_position, &slot_board)
            })
        })
        .flatten();

    for (hint, mut visibility, mut border, mut background) in &mut hint_query {
        let location_has_available_slot =
            slot_board.location_has_available_local_slot(hint.location_index);
        *visibility = if should_show && location_has_available_slot {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        let is_close_target =
            location_has_available_slot && close_location_index == Some(hint.location_index);
        if is_close_target {
            *border = BorderColor::all(DROP_TARGET_CLOSE_BORDER_COLOR);
            *background = BackgroundColor(DROP_TARGET_CLOSE_BACKGROUND_COLOR);
        } else {
            *border = BorderColor::all(DROP_TARGET_GENERAL_BORDER_COLOR);
            *background = BackgroundColor(DROP_TARGET_GENERAL_BACKGROUND_COLOR);
        }
    }
}

fn handle_press(
    pointer_position: Vec2,
    window_size: Vec2,
    card_defaults: &CardInspectionDefaults,
    gesture_model: &mut CardGestureModel,
    card_states: &CardStateModel,
    slot_board: &CardSlotBoardModel,
) {
    if gesture_model.state == CardGestureState::SelectedInspecting {
        gesture_model.return_to_source();
        return;
    }

    let Some(game_view_position) = window_pointer_to_game_view(pointer_position, window_size)
    else {
        return;
    };

    let hand_visual_count = card_states.indices_with_state(CardState::Hand).len();
    let Some(hand_index) =
        game_view_card_index_at_for_count(pointer_position, window_size, hand_visual_count)
            .and_then(|order_index| card_states.hand_index_at_order(order_index))
            .or_else(|| {
                slot_board.slots().find_map(|slot| {
                    if slot.side != CardSlotSide::LocalPlayer
                        || !slot.rect.contains(game_view_position)
                    {
                        return None;
                    }
                    match &slot.state {
                        crate::runtime::resources::CardSlotState::Populated {
                            hand_index, ..
                        } => Some(*hand_index),
                        crate::runtime::resources::CardSlotState::Empty => None,
                    }
                })
            })
    else {
        return;
    };
    if !card_states.is_selectable(hand_index) {
        return;
    }
    let (source_transform, card_center) = if matches!(
        card_states.state(hand_index),
        Some(CardState::Location | CardState::LocationLocked)
    ) {
        if let Some((location_index, slot_index)) = slot_board.local_slot_for_card(hand_index) {
            let source_transform = slot_transform(
                location_index,
                slot_index,
                CardSlotSide::LocalPlayer,
                slot_board,
                card_defaults,
            );
            (source_transform, game_view_position)
        } else {
            return;
        }
    } else {
        let hand_indices = card_states.indices_with_state(CardState::Hand);
        let Some(order_index) = hand_indices
            .iter()
            .position(|ordered_hand_index| *ordered_hand_index == hand_index)
        else {
            return;
        };
        let source_transform =
            hand_source_transform(order_index, hand_indices.len(), card_defaults);
        let Some((card_min, card_max)) =
            super::game_view_card_hitboxes_for_count(hand_indices.len())
                .get(order_index)
                .copied()
        else {
            return;
        };
        (source_transform, (card_min + card_max) * 0.5)
    };
    gesture_model.press(
        hand_index,
        game_view_position,
        card_center,
        source_transform,
    );
}

fn handle_move(
    game_view_position: Vec2,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: Option<&CardModelRegistry>,
    game_hand_model: Option<&GameHandModel>,
    game_round_model: Option<&GameRoundModel>,
    gesture_model: &mut CardGestureModel,
    card_states: &mut CardStateModel,
) {
    let started_drag =
        gesture_model.update_pointer(game_view_position, CARD_GESTURE_DRAG_THRESHOLD);
    if started_drag && let Some(hand_index) = gesture_model.active_hand_index {
        if !card_states.is_draggable(hand_index) {
            gesture_model.return_to_source();
            return;
        }
        if card_states.state(hand_index) == Some(CardState::Hand)
            && !can_drag_hand_card_with_energy(
                hand_index,
                card_model_registry,
                game_hand_model,
                game_round_model,
            )
        {
            gesture_model.return_to_source();
            return;
        }
        if card_states.state(hand_index) == Some(CardState::Hand)
            && !card_states.begin_drag(hand_index)
        {
            gesture_model.return_to_source();
            return;
        }
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

fn can_drag_hand_card_with_energy(
    hand_index: usize,
    card_model_registry: Option<&CardModelRegistry>,
    game_hand_model: Option<&GameHandModel>,
    game_round_model: Option<&GameRoundModel>,
) -> bool {
    let Some(game_round_model) = game_round_model else {
        return true;
    };
    let energy_cost = game_hand_model
        .and_then(|hand| hand.cards.get(hand_index))
        .and_then(|card_id| {
            card_model_registry.and_then(|registry| registry.card_model_for_id(card_id))
        })
        .map(|card_model| card_model.cost.value)
        .unwrap_or(0);

    game_round_model.can_spend(energy_cost)
}

fn handle_release(
    game_view_position: Option<Vec2>,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: Option<&CardModelRegistry>,
    game_hand_model: Option<&GameHandModel>,
    game_location_model: Option<&GameLocationModel>,
    mut game_round_model: Option<&mut GameRoundModel>,
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
            if card_states.state(hand_index) == Some(CardState::Location)
                && hand_area_contains(game_view_position)
            {
                let hand_count = card_states.indices_with_state(CardState::Hand).len();
                let insertion_index = hand_insertion_index(game_view_position, hand_count);
                if let Some(game_round_model) = game_round_model.as_deref_mut()
                    && let Some(record) = game_round_model.remove_move_for_hand_index(hand_index)
                {
                    slot_board.remove_local_card(hand_index);
                    game_round_model.restore(record.energy_cost);
                }
                card_states.return_to_hand_at_order(hand_index, insertion_index);
                gesture_model.return_to_hand_transform(
                    hand_index,
                    hand_source_transform(insertion_index, hand_count + 1, card_defaults),
                );
                return;
            }
            if card_states.state(hand_index) == Some(CardState::Dragging)
                && hand_area_contains(game_view_position)
            {
                let hand_count = card_states.indices_with_state(CardState::Hand).len();
                let insertion_index = hand_insertion_index(game_view_position, hand_count);
                card_states.return_to_hand_at_order(hand_index, insertion_index);
                gesture_model.return_to_hand_transform(
                    hand_index,
                    hand_source_transform(insertion_index, hand_count + 1, card_defaults),
                );
                return;
            }
            let Some(location_index) = local_slots_area_hit_target(game_view_position, slot_board)
            else {
                gesture_model.return_to_source();
                if card_states.state(hand_index) != Some(CardState::Location) {
                    card_states.return_to_hand(hand_index);
                }
                return;
            };

            let card_id = game_hand_model
                .map(|hand| hand.cards.as_slice())
                .unwrap_or_default()
                .get(hand_index)
                .cloned()
                .unwrap_or_default();
            if let Some(slot_index) = slot_board.place_next_local_with_card_id(
                location_index,
                hand_index,
                card_id.clone(),
            ) {
                let energy_cost = card_model_registry
                    .and_then(|registry| registry.card_model_for_id(&card_id))
                    .map(|card_model| card_model.cost.value)
                    .unwrap_or(0);
                if let Some(game_round_model) = game_round_model.as_deref_mut()
                    && !game_round_model.spend(energy_cost)
                {
                    slot_board.remove_local_card(hand_index);
                    gesture_model.return_to_source();
                    card_states.return_to_hand(hand_index);
                    return;
                }
                let location_energy_delta = game_location_model
                    .map(|locations| locations.ability_delta_for_location(location_index))
                    .unwrap_or(0);
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
                if let Some(game_round_model) = game_round_model.as_deref_mut() {
                    game_round_model.record_move(CurrentRoundMoveRecord {
                        hand_index,
                        card_id,
                        location_index,
                        slot_index,
                        energy_cost,
                        location_energy_delta,
                    });
                }
                hide_placed_hand_card(hand_index, card_query);
            } else {
                gesture_model.return_to_source();
                if card_states.state(hand_index) != Some(CardState::Location) {
                    card_states.return_to_hand(hand_index);
                }
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

fn hand_area_contains(game_view_position: Vec2) -> bool {
    let min = super::game_view_hand_area_min();
    let max = min + super::game_view_hand_area_size();
    game_view_position.x >= min.x
        && game_view_position.x <= max.x
        && game_view_position.y >= min.y
        && game_view_position.y <= max.y
}

fn pointer_just_released(mouse_buttons: &ButtonInput<MouseButton>, touches: &Touches) -> bool {
    mouse_buttons.just_released(MouseButton::Left) || touches.iter_just_released().next().is_some()
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/card_gesture_update_system_tests.rs"]
mod card_gesture_update_system_tests;
