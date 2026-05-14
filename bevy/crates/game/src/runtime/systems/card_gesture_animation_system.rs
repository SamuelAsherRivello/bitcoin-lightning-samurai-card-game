use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::runtime::components::{CardGestureView, HandCardGestureTarget};
use crate::runtime::resources::{
    CardGestureModel, CardGestureState, CardInspectionDefaults, CardSlotBoardModel, CardSlotSide,
    CardState, CardStateModel, SelectedCardModalModel,
};

use super::{
    DECK_SCENE_CARD_HEIGHT_FRACTION, GAME_SCENE_HAND_CARD_HEIGHT, GAME_SCENE_HEIGHT,
    GAME_SCENE_WIDTH, active_pointer_position, game_scene_card_index_at_for_count,
    game_scene_hand_card_z, game_scene_world_height_for_game_scene_height,
    game_scene_world_position_from_game_scene, window_pointer_to_game_scene,
};

const CARD_GESTURE_ANIMATION_RATE: f32 = 14.0;
pub(super) const CARD_GESTURE_DRAG_SCALE_MULTIPLIER: f32 = 1.5;
const CARD_GESTURE_DRAG_SCALE_SECONDS: f32 = 0.25;
const HAND_LAYOUT_TWEEN_SECONDS: f32 = 0.25;
const CARD_GESTURE_RETURN_SETTLE_EPSILON: f32 = 0.001;
// Gesture depths sit above static card bands so dragged/inspected cards stay contiguous.
pub(super) const CARD_GESTURE_SELECTED_Z: f32 = 0.88;
pub(super) const CARD_GESTURE_DRAG_Z: f32 = 0.98;
pub(super) const CARD_GESTURE_SLOT_Z: f32 = 0.52;

/// HUMAN: Animates card gesture views toward selected, dragged, placed, or source poses.
/// AI: Keeps interpolation visual-only; CardGestureModel owns legal state transitions.
pub fn card_gesture_animation_system(
    time: Res<Time>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    touches: Res<Touches>,
    mut gesture_model: ResMut<CardGestureModel>,
    card_defaults: Res<CardInspectionDefaults>,
    card_states: Option<Res<CardStateModel>>,
    selected_modal: Option<Res<SelectedCardModalModel>>,
    mut card_query: Query<(Entity, &HandCardGestureTarget, &mut Transform), With<CardGestureView>>,
) {
    let modal_selected_entity = selected_modal
        .as_ref()
        .and_then(|modal| modal.selected_entity);
    let hand_layout_interpolation = (time.delta_secs() / HAND_LAYOUT_TWEEN_SECONDS).clamp(0.0, 1.0);
    if let Some(card_states) = card_states.as_deref() {
        let hand_indices = card_states.indices_with_state(CardState::Hand);
        let hand_gap_index = hand_layout_gap_index(&gesture_model, hand_indices.len());
        let hand_card_count = hand_indices.len() + usize::from(hand_gap_index.is_some());
        let hovered_order_index = hand_layout_hovered_order_index(
            &primary_window_query,
            &touches,
            &gesture_model,
            hand_card_count,
        );
        for (entity, target, mut transform) in &mut card_query {
            if Some(entity) == modal_selected_entity {
                continue;
            }
            if gesture_model.is_active_for(target.hand_index) {
                continue;
            }
            if let Some(order_index) = hand_indices
                .iter()
                .position(|hand_index| *hand_index == target.hand_index)
            {
                let target_order_index = match hand_gap_index {
                    Some(gap_index) if order_index >= gap_index => order_index + 1,
                    _ => order_index,
                };
                let target_transform = hand_source_transform_for_layout(
                    target_order_index,
                    hand_card_count,
                    hovered_order_index,
                    &card_defaults,
                );
                tween_transform(&mut transform, target_transform, hand_layout_interpolation);
            }
        }
    }

    let Some(hand_index) = gesture_model.active_hand_index else {
        return;
    };
    let Some(target_transform) = gesture_model.target_transform else {
        return;
    };

    if gesture_model.state == CardGestureState::Dragging {
        gesture_model.drag_elapsed_seconds += time.delta_secs();
    }
    let interpolation = (time.delta_secs() * CARD_GESTURE_ANIMATION_RATE).clamp(0.0, 1.0);
    let drag_scale_progress = ease_out_cubic(
        (gesture_model.drag_elapsed_seconds / CARD_GESTURE_DRAG_SCALE_SECONDS).clamp(0.0, 1.0),
    );
    let mut returned_to_source = false;
    for (entity, target, mut transform) in &mut card_query {
        if Some(entity) == modal_selected_entity {
            continue;
        }
        if target.hand_index != hand_index {
            continue;
        }

        if gesture_model.state == CardGestureState::Dragging {
            transform.translation = target_transform.translation;
            if let Some(source_transform) = gesture_model.source_transform {
                transform.scale = drag_preview_source_scale(source_transform, &card_defaults)
                    .lerp(target_transform.scale, drag_scale_progress);
            }
        } else {
            transform.translation = transform
                .translation
                .lerp(target_transform.translation, interpolation);
            transform.scale = transform.scale.lerp(target_transform.scale, interpolation);
        }
        transform.rotation = transform
            .rotation
            .slerp(target_transform.rotation, interpolation);

        if gesture_model.state == CardGestureState::Returning
            && return_transform_is_settled(&transform, &target_transform)
        {
            *transform = target_transform;
            returned_to_source = true;
        }
    }

    if returned_to_source {
        gesture_model.clear_if_returned();
    }
}

pub(super) fn selected_inspection_transform(card_defaults: &CardInspectionDefaults) -> Transform {
    let height = GAME_SCENE_HEIGHT * DECK_SCENE_CARD_HEIGHT_FRACTION;
    let scale = game_scene_world_height_for_game_scene_height(height, CARD_GESTURE_SELECTED_Z)
        / card_defaults.height;

    Transform {
        translation: game_scene_world_position_from_game_scene(
            Vec2::new(GAME_SCENE_WIDTH * 0.5, GAME_SCENE_HEIGHT * 0.5),
            CARD_GESTURE_SELECTED_Z,
        ),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

pub(super) fn drag_preview_transform(
    game_scene_center_position: Vec2,
    source_transform: Transform,
    card_defaults: &CardInspectionDefaults,
) -> Transform {
    let source_game_scene_height = card_defaults.height * source_transform.scale.y
        / game_scene_world_height_for_game_scene_height(1.0, source_transform.translation.z);
    let drag_world_height = game_scene_world_height_for_game_scene_height(
        source_game_scene_height * CARD_GESTURE_DRAG_SCALE_MULTIPLIER,
        CARD_GESTURE_DRAG_Z,
    );
    let scale = drag_world_height / card_defaults.height;

    Transform {
        translation: game_scene_world_position_from_game_scene(
            game_scene_center_position,
            CARD_GESTURE_DRAG_Z,
        ),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

fn drag_preview_source_scale(
    source_transform: Transform,
    card_defaults: &CardInspectionDefaults,
) -> Vec3 {
    let source_game_scene_height = card_defaults.height * source_transform.scale.y
        / game_scene_world_height_for_game_scene_height(1.0, source_transform.translation.z);
    let source_world_height = game_scene_world_height_for_game_scene_height(
        source_game_scene_height,
        CARD_GESTURE_DRAG_Z,
    );
    let scale = source_world_height / card_defaults.height;

    Vec3::splat(scale)
}

pub(super) fn slot_transform(
    location_index: usize,
    slot_index: usize,
    side: CardSlotSide,
    slot_board: &CardSlotBoardModel,
    card_defaults: &CardInspectionDefaults,
) -> Transform {
    let Some(rect) = slot_board.slot_rect(location_index, side, slot_index) else {
        return Transform::default();
    };
    let scale = game_scene_world_height_for_game_scene_height(rect.height, CARD_GESTURE_SLOT_Z)
        / card_defaults.height;

    Transform {
        translation: game_scene_world_position_from_game_scene(rect.center(), CARD_GESTURE_SLOT_Z),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

pub(super) fn hand_source_transform(
    hand_index: usize,
    hand_card_count: usize,
    card_defaults: &CardInspectionDefaults,
) -> Transform {
    hand_source_transform_for_layout(hand_index, hand_card_count, None, card_defaults)
}

pub(super) fn hand_source_transform_for_layout(
    hand_index: usize,
    hand_card_count: usize,
    hovered_hand_index: Option<usize>,
    card_defaults: &CardInspectionDefaults,
) -> Transform {
    let Some((min, max)) =
        super::game_scene_card_hitboxes_for_count_with_hover(hand_card_count, hovered_hand_index)
            .get(hand_index)
            .copied()
    else {
        return Transform::default();
    };
    let scale = game_scene_world_height_for_game_scene_height(
        GAME_SCENE_HAND_CARD_HEIGHT,
        game_scene_hand_card_z(hand_index, hovered_hand_index),
    ) / card_defaults.height;

    Transform {
        translation: game_scene_world_position_from_game_scene(
            (min + max) * 0.5,
            game_scene_hand_card_z(hand_index, hovered_hand_index),
        ),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

fn hand_layout_gap_index(
    gesture_model: &CardGestureModel,
    visible_hand_card_count: usize,
) -> Option<usize> {
    if gesture_model.state != CardGestureState::Dragging {
        return None;
    }
    let pointer = gesture_model.pointer?;
    let current_position = pointer.current_position;
    if !hand_area_contains(current_position) {
        return None;
    }
    Some(hand_insertion_index(
        current_position,
        visible_hand_card_count,
    ))
}

fn hand_layout_hovered_order_index(
    primary_window_query: &Query<&Window, With<PrimaryWindow>>,
    touches: &Touches,
    gesture_model: &CardGestureModel,
    hand_card_count: usize,
) -> Option<usize> {
    if gesture_model.state != CardGestureState::Idle {
        return None;
    }
    let Ok(primary_window) = primary_window_query.single() else {
        return None;
    };
    let window_size = Vec2::new(
        primary_window.resolution.width(),
        primary_window.resolution.height(),
    );
    let pointer_position = active_pointer_position(primary_window, touches)?;
    let game_scene_position = window_pointer_to_game_scene(pointer_position, window_size)?;
    if !hand_area_contains(game_scene_position) {
        return None;
    }

    game_scene_card_index_at_for_count(pointer_position, window_size, hand_card_count)
}

fn hand_area_contains(game_scene_position: Vec2) -> bool {
    let min = super::game_scene_hand_area_min();
    let max = min + super::game_scene_hand_area_size();
    game_scene_position.x >= min.x
        && game_scene_position.x <= max.x
        && game_scene_position.y >= min.y
        && game_scene_position.y <= max.y
}

fn tween_transform(transform: &mut Transform, target: Transform, interpolation: f32) {
    transform.translation = transform
        .translation
        .lerp(target.translation, interpolation);
    transform.scale = transform.scale.lerp(target.scale, interpolation);
    transform.rotation = transform.rotation.slerp(target.rotation, interpolation);
}

pub(super) fn hand_insertion_index(game_scene_position: Vec2, hand_card_count: usize) -> usize {
    let hitboxes = super::game_scene_card_hitboxes_for_count(hand_card_count);
    if hitboxes.is_empty() {
        return 0;
    }

    for (index, (min, max)) in hitboxes.iter().enumerate() {
        if game_scene_position.x < (min.x + max.x) * 0.5 {
            return index;
        }
    }
    hand_card_count
}

pub(super) fn local_slots_area_hit_target(
    game_scene_position: Vec2,
    slot_board: &CardSlotBoardModel,
) -> Option<usize> {
    slot_board.local_slots_area_hit_target(game_scene_position)
}

pub(super) fn local_slots_area_rect(
    location_index: usize,
    slot_board: &CardSlotBoardModel,
) -> Option<(Vec2, Vec2)> {
    let rect = slot_board.local_slots_area_rect(location_index)?;

    Some((
        Vec2::new(rect.left, rect.top),
        Vec2::new(rect.left + rect.width, rect.top + rect.height),
    ))
}

#[cfg(test)]
pub(super) fn slot_center(
    location_index: usize,
    slot_index: usize,
    side: CardSlotSide,
    slot_board: &CardSlotBoardModel,
) -> Option<Vec2> {
    slot_board
        .slot_rect(location_index, side, slot_index)
        .map(|rect| rect.center())
}

fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress).powi(3)
}

fn return_transform_is_settled(transform: &Transform, target_transform: &Transform) -> bool {
    transform.translation.distance(target_transform.translation)
        <= CARD_GESTURE_RETURN_SETTLE_EPSILON
        && transform.scale.distance(target_transform.scale) <= CARD_GESTURE_RETURN_SETTLE_EPSILON
        && transform.rotation.angle_between(target_transform.rotation)
            <= CARD_GESTURE_RETURN_SETTLE_EPSILON
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/card_gesture_animation_system_tests.rs"]
mod card_gesture_animation_system_tests;
