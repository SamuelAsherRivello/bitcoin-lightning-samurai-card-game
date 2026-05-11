use bevy::prelude::*;

use crate::runtime::components::{CardGestureView, HandCardGestureTarget};
use crate::runtime::resources::{
    CardGestureModel, CardGestureState, CardInspectionDefaults, CardSlotBoardModel, CardSlotSide,
};

use super::{
    DECK_BUILDER_CARD_HEIGHT_FRACTION, GAME_SCENE_HAND_CARD_HEIGHT, GAME_SCENE_HAND_CARD_WORLD_Z,
    GAME_VIEW_HEIGHT, GAME_VIEW_WIDTH, game_view_world_height_for_game_view_height,
    game_view_world_position_from_game_view,
};

const CARD_GESTURE_ANIMATION_RATE: f32 = 14.0;
const CARD_GESTURE_DRAG_SCALE_MULTIPLIER: f32 = 1.5;
const CARD_GESTURE_DRAG_SCALE_SECONDS: f32 = 0.25;
const CARD_GESTURE_RETURN_SETTLE_EPSILON: f32 = 0.001;
pub(super) const CARD_GESTURE_SELECTED_Z: f32 = 0.48;
pub(super) const CARD_GESTURE_DRAG_Z: f32 = 0.56;
pub(super) const CARD_GESTURE_SLOT_Z: f32 = 0.5;

/// HUMAN: Animates card gesture views toward selected, dragged, placed, or source poses.
/// AI: Keeps interpolation visual-only; CardGestureModel owns legal state transitions.
pub fn card_gesture_animation_system(
    time: Res<Time>,
    mut gesture_model: ResMut<CardGestureModel>,
    mut card_query: Query<(&HandCardGestureTarget, &mut Transform), With<CardGestureView>>,
) {
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
    for (target, mut transform) in &mut card_query {
        if target.hand_index != hand_index {
            continue;
        }

        if gesture_model.state == CardGestureState::Dragging {
            transform.translation = target_transform.translation;
            if let Some(source_transform) = gesture_model.source_transform {
                transform.scale = source_transform
                    .scale
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
    let height = GAME_VIEW_HEIGHT * DECK_BUILDER_CARD_HEIGHT_FRACTION;
    let scale = game_view_world_height_for_game_view_height(height, CARD_GESTURE_SELECTED_Z)
        / card_defaults.height;

    Transform {
        translation: game_view_world_position_from_game_view(
            Vec2::new(GAME_VIEW_WIDTH * 0.5, GAME_VIEW_HEIGHT * 0.5),
            CARD_GESTURE_SELECTED_Z,
        ),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

pub(super) fn drag_preview_transform(
    game_view_center_position: Vec2,
    source_transform: Transform,
    _card_defaults: &CardInspectionDefaults,
) -> Transform {
    let scale = source_transform.scale * CARD_GESTURE_DRAG_SCALE_MULTIPLIER;

    Transform {
        translation: game_view_world_position_from_game_view(
            game_view_center_position,
            CARD_GESTURE_DRAG_Z,
        ),
        scale,
        ..Default::default()
    }
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
    let scale = game_view_world_height_for_game_view_height(rect.height, CARD_GESTURE_SLOT_Z)
        / card_defaults.height;

    Transform {
        translation: game_view_world_position_from_game_view(rect.center(), CARD_GESTURE_SLOT_Z),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

pub(super) fn hand_source_transform(
    hand_index: usize,
    card_defaults: &CardInspectionDefaults,
) -> Transform {
    let Some((min, max)) = super::game_view_card_hitboxes().get(hand_index).copied() else {
        return Transform::default();
    };
    let scale = game_view_world_height_for_game_view_height(
        GAME_SCENE_HAND_CARD_HEIGHT,
        GAME_SCENE_HAND_CARD_WORLD_Z,
    ) / card_defaults.height;

    Transform {
        translation: game_view_world_position_from_game_view(
            (min + max) * 0.5,
            GAME_SCENE_HAND_CARD_WORLD_Z,
        ),
        scale: Vec3::splat(scale),
        ..Default::default()
    }
}

pub(super) fn local_slots_area_hit_target(
    game_view_position: Vec2,
    slot_board: &CardSlotBoardModel,
) -> Option<usize> {
    slot_board.local_slots_area_hit_target(game_view_position)
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
mod tests {
    use crate::runtime::resources::CardInspectionDefaults;

    use super::*;

    #[test]
    fn selected_inspection_uses_ninety_percent_safe_height() {
        let defaults = CardInspectionDefaults::default();
        let transform = selected_inspection_transform(&defaults);
        let rendered_height = defaults.height * transform.scale.y;
        let expected = game_view_world_height_for_game_view_height(
            GAME_VIEW_HEIGHT * DECK_BUILDER_CARD_HEIGHT_FRACTION,
            CARD_GESTURE_SELECTED_Z,
        );

        assert!((rendered_height - expected).abs() < 0.0001);
    }

    #[test]
    fn slot_transform_preserves_card_aspect_ratio() {
        let defaults = CardInspectionDefaults::default();
        let board = CardSlotBoardModel::default();
        let transform = slot_transform(1, 2, CardSlotSide::LocalPlayer, &board, &defaults);

        assert_eq!(transform.scale.x, transform.scale.y);
        assert_eq!(transform.scale.y, transform.scale.z);
    }

    #[test]
    fn local_slots_area_hit_target_detects_whole_available_location_area() {
        let board = CardSlotBoardModel::default();

        assert_eq!(
            local_slots_area_hit_target(
                slot_center(0, 0, CardSlotSide::LocalPlayer, &board).unwrap(),
                &board
            ),
            Some(0)
        );
        assert_eq!(
            local_slots_area_hit_target(
                slot_center(2, 3, CardSlotSide::LocalPlayer, &board).unwrap(),
                &board
            ),
            Some(2)
        );
        assert_eq!(
            local_slots_area_hit_target(
                slot_center(2, 3, CardSlotSide::Opponent, &board).unwrap(),
                &board
            ),
            None
        );
        assert_eq!(
            local_slots_area_hit_target(Vec2::new(GAME_VIEW_WIDTH * 0.5, GAME_VIEW_HEIGHT), &board),
            None
        );
    }

    #[test]
    fn drag_preview_scales_to_one_hundred_fifty_percent_of_source() {
        let defaults = CardInspectionDefaults::default();
        let source = Transform::from_scale(Vec3::splat(0.4));
        let transform = drag_preview_transform(Vec2::new(10.0, 10.0), source, &defaults);

        assert_eq!(transform.scale, Vec3::splat(0.6));
    }

    #[test]
    fn returning_card_settles_only_when_original_scale_is_restored() {
        let target =
            Transform::from_translation(Vec3::new(1.0, 2.0, 0.32)).with_scale(Vec3::splat(0.42));
        let wrong_scale = Transform::from_translation(target.translation)
            .with_scale(Vec3::splat(target.scale.x + 0.01));
        let nearly_restored = Transform::from_translation(target.translation)
            .with_scale(Vec3::splat(target.scale.x + 0.0005));

        assert!(!return_transform_is_settled(&wrong_scale, &target));
        assert!(return_transform_is_settled(&nearly_restored, &target));
    }
}
