use super::*;

#[test]
fn catalog_contains_all_game_event_sequence_animation_types() {
    assert_eq!(
        GAME_TWEEN_ALL_PRESETS,
        [
            GameTweenPreset::InstaFlip,
            GameTweenPreset::Flip,
            GameTweenPreset::SwanFlip,
            GameTweenPreset::DealSlide,
            GameTweenPreset::CardMoveToSlot,
            GameTweenPreset::DragPlace,
            GameTweenPreset::LocationIntro,
        ]
    );
}

#[test]
fn card_presets_are_transform3d_compatible() {
    assert_eq!(
        GAME_TWEEN_CARD_PRESETS,
        [
            GameTweenPreset::InstaFlip,
            GameTweenPreset::Flip,
            GameTweenPreset::SwanFlip,
            GameTweenPreset::DealSlide,
            GameTweenPreset::CardMoveToSlot,
            GameTweenPreset::DragPlace,
        ]
    );
}

#[test]
fn transform_tween_samples_eased_translation_rotation_and_scale() {
    let start = Transform::from_translation(Vec3::ZERO).with_scale(Vec3::ONE);
    let target = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0))
        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI))
        .with_scale(Vec3::splat(3.0));

    let sample = sample_transform_tween(
        start,
        target,
        GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS * 0.5,
        GameTweenPreset::CardMoveToSlot,
    );

    assert!(sample.progress == 0.5);
    assert!(sample.transform.translation.x > 5.0);
    assert!(sample.transform.scale.x > 2.0);
    assert!(!sample.is_complete);
}

#[test]
fn insta_flip_completes_immediately_at_target_transform() {
    let start = Transform::from_translation(Vec3::ZERO);
    let target = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));

    let sample = sample_transform_tween(start, target, 0.0, GameTweenPreset::InstaFlip);

    assert_eq!(sample.transform, target);
    assert_eq!(sample.progress, 1.0);
    assert!(sample.is_complete);
}

#[test]
fn swan_scale_has_peak_hold_and_lands_at_one() {
    assert_eq!(sample_swan_scale_multiplier(0.0), 1.0);
    assert_eq!(
        sample_swan_scale_multiplier(GAME_TWEEN_SWAN_SCALE_UP_SECONDS),
        GAME_TWEEN_SWAN_SCALE_MULTIPLIER
    );
    assert_eq!(
        sample_swan_scale_multiplier(
            GAME_TWEEN_SWAN_SCALE_UP_SECONDS + GAME_TWEEN_SWAN_SCALE_HOLD_SECONDS
        ),
        GAME_TWEEN_SWAN_SCALE_MULTIPLIER
    );
    assert_eq!(
        sample_swan_scale_multiplier(GAME_TWEEN_SWAN_FLIP_SECONDS),
        1.0
    );
}

#[test]
fn location_intro_uses_staggered_scale_and_opacity() {
    let first = sample_location_intro_tween(0, 0.0);
    let delayed = sample_location_intro_tween(1, 0.0);
    let complete = sample_location_intro_tween(0, GAME_TWEEN_LOCATION_INTRO_SECONDS);

    assert_eq!(first.scale, GAME_TWEEN_LOCATION_INTRO_START_SCALE);
    assert_eq!(first.opacity, 0.0);
    assert_eq!(delayed.scale, GAME_TWEEN_LOCATION_INTRO_START_SCALE);
    assert_eq!(delayed.opacity, 0.0);
    assert_eq!(complete.scale, 1.0);
    assert_eq!(complete.opacity, 1.0);
}
