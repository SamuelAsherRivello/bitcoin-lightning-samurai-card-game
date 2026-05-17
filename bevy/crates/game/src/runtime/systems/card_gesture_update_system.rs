use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::runtime::components::{
    CardGestureView, DropTargetHint, HandCardGestureTarget, LocalPlayerHandCardPreview,
};
use crate::runtime::resources::{
    ActiveView, CARD_GESTURE_DRAG_THRESHOLD, CardGestureModel, CardGestureSlotTarget,
    CardGestureState, CardInspectionDefaults, CardModelRegistry, CardSlotBoardModel, CardSlotRect,
    CardSlotSide, CardState, CardStateModel, CurrentRoundMoveRecord, GameHandModel,
    GameLocationModel, GameRoundModel, MatchModel, PendingRoundDealResource,
    SelectedCardModalModel,
};

use super::{
    CARD_GESTURE_DRAG_SCALE_MULTIPLIER, active_pointer_position, drag_preview_transform,
    game_scene_card_index_at_for_count, hand_insertion_index, hand_source_transform,
    just_pressed_pointer_position, selected_inspection_transform, slot_transform,
    window_pointer_to_game_scene,
};

const DROP_TARGET_GENERAL_BORDER_COLOR: Color = Color::srgb(0.48, 0.82, 1.0);
const DROP_TARGET_GENERAL_BACKGROUND_COLOR: Color = Color::srgba(0.28, 0.72, 1.0, 0.06);
const DROP_TARGET_CLOSE_BORDER_COLOR: Color = Color::srgb(0.72, 0.94, 1.0);
const DROP_TARGET_CLOSE_BACKGROUND_COLOR: Color = Color::srgba(0.36, 0.86, 1.0, 0.24);
const DROP_TARGET_MIN_CARD_OVERLAP_RATIO: f32 = 0.25;

#[derive(SystemParam)]
pub struct CardGestureUpdateResources<'w> {
    card_model_registry: Option<Res<'w, CardModelRegistry>>,
    game_hand_model: Option<Res<'w, GameHandModel>>,
    game_location_model: Option<Res<'w, GameLocationModel>>,
    match_model: Option<Res<'w, MatchModel>>,
    pending_round_deal: Option<Res<'w, PendingRoundDealResource>>,
    game_round_model: Option<ResMut<'w, GameRoundModel>>,
}

/// HUMAN: Updates card gesture state from unified pointer input in GameScene.
/// AI: This replaces GameScene click-to-DeckScene navigation without touching DeckScene behavior.
pub fn card_gesture_update_system(
    mut commands: Commands,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    active_view: Res<ActiveView>,
    card_defaults: Res<CardInspectionDefaults>,
    mut resources: CardGestureUpdateResources,
    mut gesture_model: ResMut<CardGestureModel>,
    mut selected_modal: Option<ResMut<SelectedCardModalModel>>,
    mut slot_board: ResMut<CardSlotBoardModel>,
    mut card_states: ResMut<CardStateModel>,
    mut card_query: Query<
        (Entity, &HandCardGestureTarget, &Transform, &mut Visibility),
        With<CardGestureView>,
    >,
) {
    if *active_view != ActiveView::GameScene {
        return;
    }
    if selected_modal
        .as_ref()
        .is_some_and(|modal| modal.blocks_lower_interactions())
    {
        return;
    }
    if resources
        .match_model
        .as_deref()
        .is_some_and(|model| model.near.controller.is_cpu())
    {
        return;
    }
    if resources
        .pending_round_deal
        .as_deref()
        .is_some_and(|deal| deal.is_pending || !deal.is_round_deal_complete)
    {
        return;
    }
    let dragging_allowed = resources
        .match_model
        .as_deref()
        .is_none_or(|model| !model.is_complete());

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
        && let Some(game_scene_position) =
            window_pointer_to_game_scene(pointer_position, window_size)
    {
        handle_move(
            game_scene_position,
            &card_defaults,
            resources.card_model_registry.as_deref(),
            resources.game_hand_model.as_deref(),
            resources.game_round_model.as_deref(),
            dragging_allowed,
            &mut gesture_model,
            &mut card_states,
        );
    }

    if pointer_just_released(&mouse_buttons, &touches) {
        let game_scene_position = active_pointer_position(primary_window, &touches)
            .and_then(|pointer_position| {
                window_pointer_to_game_scene(pointer_position, window_size)
            })
            .or_else(|| {
                gesture_model
                    .pointer
                    .as_ref()
                    .map(|pointer| pointer.current_position)
            });
        let mut fallback_selected_modal;
        let selected_modal = if let Some(selected_modal) = selected_modal.as_deref_mut() {
            selected_modal
        } else {
            fallback_selected_modal = SelectedCardModalModel::default();
            &mut fallback_selected_modal
        };
        handle_release_with_commands(
            game_scene_position,
            &card_defaults,
            resources.card_model_registry.as_deref(),
            resources.game_hand_model.as_deref(),
            resources.game_location_model.as_deref(),
            resources.game_round_model.as_deref_mut(),
            &mut gesture_model,
            selected_modal,
            &mut slot_board,
            &mut card_states,
            &mut card_query,
            Some(&mut commands),
        );
    }
}

/// HUMAN: Shows user-facing drop target hints for locations that can accept dragged cards.
/// AI: This is gameplay feedback, not debug drawing, and should not depend on DebugHUD state.
pub fn drop_target_hint_update_system(
    active_view: Res<ActiveView>,
    gesture_model: Res<CardGestureModel>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Option<Res<CardModelRegistry>>,
    game_hand_model: Option<Res<GameHandModel>>,
    game_round_model: Option<Res<GameRoundModel>>,
    match_model: Option<Res<MatchModel>>,
    card_states: Option<Res<CardStateModel>>,
    slot_board: Res<CardSlotBoardModel>,
    selected_modal: Res<SelectedCardModalModel>,
    mut hint_query: Query<(
        &DropTargetHint,
        &mut Visibility,
        &mut BorderColor,
        &mut BackgroundColor,
    )>,
) {
    let can_pay_for_dragged_card = dragged_card_can_show_drop_targets(
        &gesture_model,
        card_states.as_deref(),
        card_model_registry.as_deref(),
        game_hand_model.as_deref(),
        game_round_model.as_deref(),
    );
    let should_show = *active_view == ActiveView::GameScene
        && gesture_model.state == CardGestureState::Dragging
        && !selected_modal.blocks_lower_interactions()
        && match_model
            .as_deref()
            .is_none_or(|model| !model.is_complete())
        && can_pay_for_dragged_card;
    let focused_location_index = should_show
        .then(|| dragged_card_drop_location_index(&gesture_model, &card_defaults, &slot_board))
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
            location_has_available_slot && focused_location_index == Some(hint.location_index);
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
        return;
    }

    let Some(game_scene_position) = window_pointer_to_game_scene(pointer_position, window_size)
    else {
        return;
    };

    let hand_visual_count = card_states.indices_with_state(CardState::Hand).len();
    let Some(hand_index) =
        game_scene_card_index_at_for_count(pointer_position, window_size, hand_visual_count)
            .and_then(|order_index| card_states.hand_index_at_order(order_index))
            .or_else(|| {
                slot_board.slots().find_map(|slot| {
                    if slot.side != CardSlotSide::LocalPlayer
                        || !slot.rect.contains(game_scene_position)
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
            (source_transform, game_scene_position)
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
            super::game_scene_card_hitboxes_for_count(hand_indices.len())
                .get(order_index)
                .copied()
        else {
            return;
        };
        (source_transform, (card_min + card_max) * 0.5)
    };
    gesture_model.press(
        hand_index,
        game_scene_position,
        card_center,
        source_transform,
    );
}

fn handle_move(
    game_scene_position: Vec2,
    card_defaults: &CardInspectionDefaults,
    _card_model_registry: Option<&CardModelRegistry>,
    _game_hand_model: Option<&GameHandModel>,
    _game_round_model: Option<&GameRoundModel>,
    dragging_allowed: bool,
    gesture_model: &mut CardGestureModel,
    card_states: &mut CardStateModel,
) {
    if !dragging_allowed {
        if gesture_model.state == CardGestureState::Dragging {
            if let Some(hand_index) = gesture_model.active_hand_index
                && card_states.state(hand_index) == Some(CardState::Dragging)
            {
                card_states.return_to_hand(hand_index);
            }
            gesture_model.return_to_source();
            return;
        }
        if gesture_model.update_pointer(game_scene_position, CARD_GESTURE_DRAG_THRESHOLD) {
            gesture_model.return_to_source();
        }
        return;
    }

    let started_drag =
        gesture_model.update_pointer(game_scene_position, CARD_GESTURE_DRAG_THRESHOLD);
    if started_drag && let Some(hand_index) = gesture_model.active_hand_index {
        if !card_states.is_draggable(hand_index) {
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

fn dragged_card_can_show_drop_targets(
    gesture_model: &CardGestureModel,
    card_states: Option<&CardStateModel>,
    card_model_registry: Option<&CardModelRegistry>,
    game_hand_model: Option<&GameHandModel>,
    game_round_model: Option<&GameRoundModel>,
) -> bool {
    let Some(hand_index) = gesture_model.active_hand_index else {
        return true;
    };
    let Some(card_states) = card_states else {
        return true;
    };
    if !matches!(
        card_states.state(hand_index),
        Some(CardState::Hand | CardState::Dragging)
    ) {
        return true;
    }

    can_play_hand_card_with_energy(
        hand_index,
        card_model_registry,
        game_hand_model,
        game_round_model,
    )
}

fn can_play_hand_card_with_energy(
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

fn dragged_card_drop_location_index(
    gesture_model: &CardGestureModel,
    card_defaults: &CardInspectionDefaults,
    slot_board: &CardSlotBoardModel,
) -> Option<usize> {
    let card_rect = dragged_card_game_scene_rect(gesture_model, card_defaults)?;
    let card_area = card_rect.width * card_rect.height;
    if card_area <= 0.0 {
        return None;
    }

    (0..crate::runtime::resources::CARD_SLOT_LOCATION_COUNT)
        .filter_map(|location_index| {
            if !slot_board.location_has_available_local_slot(location_index) {
                return None;
            }
            let zone_rect = location_drop_area_rect(slot_board, location_index)?;
            let overlap_area = rect_overlap_area(card_rect, zone_rect);
            (overlap_area / card_area >= DROP_TARGET_MIN_CARD_OVERLAP_RATIO)
                .then_some((location_index, overlap_area))
        })
        .max_by(|(_, left_area), (_, right_area)| {
            left_area
                .partial_cmp(right_area)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(location_index, _)| location_index)
}

fn location_drop_area_rect(
    slot_board: &CardSlotBoardModel,
    location_index: usize,
) -> Option<CardSlotRect> {
    match (
        slot_board.location_area_rect(location_index),
        slot_board.local_slots_area_rect(location_index),
    ) {
        (Some(location_area), Some(local_slots_area)) => {
            Some(location_area.union(local_slots_area))
        }
        (Some(location_area), None) => Some(location_area),
        (None, Some(local_slots_area)) => Some(local_slots_area),
        (None, None) => None,
    }
}

fn dragged_card_game_scene_rect(
    gesture_model: &CardGestureModel,
    card_defaults: &CardInspectionDefaults,
) -> Option<CardSlotRect> {
    let pointer = gesture_model.pointer?;
    let source_transform = gesture_model.source_transform?;
    let source_game_scene_height = card_defaults.height * source_transform.scale.y
        / super::game_scene_world_height_for_game_scene_height(1.0, source_transform.translation.z);
    let height = source_game_scene_height * CARD_GESTURE_DRAG_SCALE_MULTIPLIER;
    let width = height * (card_defaults.width / card_defaults.height);
    let center = pointer.current_card_center();

    Some(CardSlotRect::new(
        center.x - width * 0.5,
        center.y - height * 0.5,
        width,
        height,
    ))
}

fn rect_overlap_area(left: CardSlotRect, right: CardSlotRect) -> f32 {
    let overlap_left = left.left.max(right.left);
    let overlap_top = left.top.max(right.top);
    let overlap_right = (left.left + left.width).min(right.left + right.width);
    let overlap_bottom = (left.top + left.height).min(right.top + right.height);
    let width = (overlap_right - overlap_left).max(0.0);
    let height = (overlap_bottom - overlap_top).max(0.0);

    width * height
}

#[cfg(test)]
fn handle_release(
    game_scene_position: Option<Vec2>,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: Option<&CardModelRegistry>,
    game_hand_model: Option<&GameHandModel>,
    game_location_model: Option<&GameLocationModel>,
    game_round_model: Option<&mut GameRoundModel>,
    gesture_model: &mut CardGestureModel,
    selected_modal: &mut SelectedCardModalModel,
    slot_board: &mut CardSlotBoardModel,
    card_states: &mut CardStateModel,
    card_query: &mut Query<
        (Entity, &HandCardGestureTarget, &Transform, &mut Visibility),
        With<CardGestureView>,
    >,
) {
    handle_release_with_commands(
        game_scene_position,
        card_defaults,
        card_model_registry,
        game_hand_model,
        game_location_model,
        game_round_model,
        gesture_model,
        selected_modal,
        slot_board,
        card_states,
        card_query,
        None,
    );
}

fn handle_release_with_commands(
    game_scene_position: Option<Vec2>,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: Option<&CardModelRegistry>,
    game_hand_model: Option<&GameHandModel>,
    game_location_model: Option<&GameLocationModel>,
    mut game_round_model: Option<&mut GameRoundModel>,
    gesture_model: &mut CardGestureModel,
    selected_modal: &mut SelectedCardModalModel,
    slot_board: &mut CardSlotBoardModel,
    card_states: &mut CardStateModel,
    card_query: &mut Query<
        (Entity, &HandCardGestureTarget, &Transform, &mut Visibility),
        With<CardGestureView>,
    >,
    mut commands: Option<&mut Commands>,
) {
    match gesture_model.state {
        CardGestureState::Pressed => {
            let target_transform = selected_inspection_transform(card_defaults);
            if let Some(hand_index) = gesture_model.active_hand_index
                && let Some((entity, source_transform)) =
                    card_entity_and_current_transform_for_hand_index(hand_index, card_query)
            {
                selected_modal.select_entity(entity, source_transform, target_transform);
            }
            gesture_model.select(target_transform);
        }
        CardGestureState::Dragging => {
            let Some(hand_index) = gesture_model.active_hand_index else {
                gesture_model.return_to_source();
                return;
            };
            let Some(game_scene_position) = game_scene_position else {
                gesture_model.return_to_source();
                card_states.return_to_hand(hand_index);
                return;
            };
            if card_states.state(hand_index) == Some(CardState::Location)
                && hand_area_contains(game_scene_position)
            {
                let hand_count = card_states.indices_with_state(CardState::Hand).len();
                let insertion_index = hand_insertion_index(game_scene_position, hand_count);
                if let Some(game_round_model) = game_round_model.as_deref_mut()
                    && let Some(record) = game_round_model.remove_move_for_hand_index(hand_index)
                {
                    slot_board.remove_local_card(hand_index);
                    game_round_model.restore(record.energy_cost);
                }
                card_states.return_to_hand_at_order(hand_index, insertion_index);
                if let Some(commands) = commands.as_deref_mut() {
                    set_local_hand_preview_marker(hand_index, card_query, commands, true);
                }
                gesture_model.return_to_hand_transform(
                    hand_index,
                    hand_source_transform(insertion_index, hand_count + 1, card_defaults),
                );
                return;
            }
            if card_states.state(hand_index) == Some(CardState::Dragging)
                && hand_area_contains(game_scene_position)
            {
                let hand_count = card_states.indices_with_state(CardState::Hand).len();
                let insertion_index = hand_insertion_index(game_scene_position, hand_count);
                card_states.return_to_hand_at_order(hand_index, insertion_index);
                gesture_model.return_to_hand_transform(
                    hand_index,
                    hand_source_transform(insertion_index, hand_count + 1, card_defaults),
                );
                return;
            }
            let Some(location_index) =
                dragged_card_drop_location_index(gesture_model, card_defaults, slot_board)
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
                if let Some(commands) = commands.as_deref_mut() {
                    set_local_hand_preview_marker(hand_index, card_query, commands, false);
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

fn set_local_hand_preview_marker(
    hand_index: usize,
    card_query: &mut Query<
        (Entity, &HandCardGestureTarget, &Transform, &mut Visibility),
        With<CardGestureView>,
    >,
    commands: &mut Commands,
    is_hand_preview: bool,
) {
    for (entity, target, _, _) in card_query.iter_mut() {
        if target.hand_index != hand_index {
            continue;
        }
        if is_hand_preview {
            commands.entity(entity).insert(LocalPlayerHandCardPreview);
        } else {
            commands
                .entity(entity)
                .remove::<LocalPlayerHandCardPreview>();
        }
    }
}

fn hide_placed_hand_card(
    hand_index: usize,
    card_query: &mut Query<
        (Entity, &HandCardGestureTarget, &Transform, &mut Visibility),
        With<CardGestureView>,
    >,
) {
    for (_, target, _, mut visibility) in card_query.iter_mut() {
        if target.hand_index == hand_index {
            *visibility = Visibility::Visible;
        }
    }
}

fn card_entity_and_current_transform_for_hand_index(
    hand_index: usize,
    card_query: &mut Query<
        (Entity, &HandCardGestureTarget, &Transform, &mut Visibility),
        With<CardGestureView>,
    >,
) -> Option<(Entity, Transform)> {
    card_query
        .iter_mut()
        .find_map(|(entity, target, transform, _)| {
            (target.hand_index == hand_index).then_some((entity, *transform))
        })
}

fn hand_area_contains(game_scene_position: Vec2) -> bool {
    let min = super::game_scene_hand_area_min();
    let max = min + super::game_scene_hand_area_size();
    game_scene_position.x >= min.x
        && game_scene_position.x <= max.x
        && game_scene_position.y >= min.y
        && game_scene_position.y <= max.y
}

fn pointer_just_released(mouse_buttons: &ButtonInput<MouseButton>, touches: &Touches) -> bool {
    mouse_buttons.just_released(MouseButton::Left) || touches.iter_just_released().next().is_some()
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/card_gesture_update_system_tests.rs"]
mod card_gesture_update_system_tests;
