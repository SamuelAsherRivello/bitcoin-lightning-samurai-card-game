use crate::runtime::resources::CardInspectionDefaults;

use super::*;

#[test]
fn selected_inspection_uses_ninety_percent_safe_height() {
    let defaults = CardInspectionDefaults::default();
    let transform = selected_inspection_transform(&defaults);
    let rendered_height = selected_card_visual_height(&defaults) * transform.scale.y;
    let expected = game_scene_world_height_for_game_scene_height(
        GAME_SCENE_HEIGHT * DECK_SCENE_CARD_HEIGHT_FRACTION,
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
fn hand_source_depth_increases_to_the_right() {
    let defaults = CardInspectionDefaults::default();
    let left = hand_source_transform(0, 4, &defaults);
    let right = hand_source_transform(3, 4, &defaults);

    assert!(right.translation.z > left.translation.z);
}

#[test]
fn overlapping_hand_cards_use_distinct_card_depth_bands() {
    let defaults = CardInspectionDefaults::default();
    let left = hand_source_transform(0, 4, &defaults);
    let right = hand_source_transform(1, 4, &defaults);

    assert!((right.translation.z - left.translation.z) > defaults.thickness);
}

#[test]
fn hovered_hand_source_uses_highest_hand_depth() {
    let defaults = CardInspectionDefaults::default();
    let hovered = hand_source_transform_for_layout(1, 4, Some(1), &defaults);
    let right = hand_source_transform_for_layout(3, 4, Some(1), &defaults);

    assert!(hovered.translation.z > right.translation.z);
}

#[test]
fn drag_preview_uses_top_card_depth_band() {
    let defaults = CardInspectionDefaults::default();
    let source = hand_source_transform(0, 12, &defaults);
    let dragged = drag_preview_transform(Vec2::new(10.0, 10.0), source, &defaults);
    let selected = selected_inspection_transform(&defaults);
    let hovered = hand_source_transform_for_layout(1, 12, Some(1), &defaults);
    let rightmost = hand_source_transform(11, 12, &defaults);
    let board = CardSlotBoardModel::default();
    let slotted = slot_transform(1, 2, CardSlotSide::LocalPlayer, &board, &defaults);

    assert!(dragged.translation.z > selected.translation.z + defaults.thickness);
    assert!(dragged.translation.z > hovered.translation.z + defaults.thickness);
    assert!(dragged.translation.z > rightmost.translation.z + defaults.thickness);
    assert!(dragged.translation.z > slotted.translation.z + defaults.thickness);
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
        local_slots_area_hit_target(Vec2::new(GAME_SCENE_WIDTH * 0.5, GAME_SCENE_HEIGHT), &board),
        None
    );
}

#[test]
fn drag_preview_scales_to_one_hundred_fifty_percent_of_source_visual_height() {
    let defaults = CardInspectionDefaults::default();
    let source = hand_source_transform(0, 4, &defaults);
    let transform = drag_preview_transform(Vec2::new(10.0, 10.0), source, &defaults);

    assert_close(
        card_game_scene_height(transform, &defaults),
        card_game_scene_height(source, &defaults) * 1.5,
    );
    assert!(transform.scale.y < source.scale.y * 1.5);
}

#[test]
fn drag_preview_source_scale_preserves_current_visual_height_at_drag_depth() {
    let defaults = CardInspectionDefaults::default();
    let source = hand_source_transform(0, 4, &defaults);
    let lifted_source = Transform {
        translation: Vec3::new(0.0, 0.0, CARD_GESTURE_DRAG_Z),
        scale: drag_preview_source_scale(source, &defaults),
        ..Default::default()
    };

    assert_close(
        card_game_scene_height(lifted_source, &defaults),
        card_game_scene_height(source, &defaults),
    );
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

fn card_game_scene_height(transform: Transform, defaults: &CardInspectionDefaults) -> f32 {
    defaults.height * transform.scale.y
        / game_scene_world_height_for_game_scene_height(1.0, transform.translation.z)
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.0001,
        "actual {actual} did not match expected {expected}"
    );
}
