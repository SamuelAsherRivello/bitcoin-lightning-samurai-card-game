use bevy::prelude::*;

/// HUMAN: Named gameplay animation presets shared by card and location presentation.
/// AI: Keep these semantics aligned with specs/007-gameplay-concepts/game-event-sequence.md.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameTweenPreset {
    InstaFlip,
    Flip,
    SwanFlip,
    DealSlide,
    CardMoveToSlot,
    DragPlace,
    LocationIntro,
}

/// HUMAN: Sampled transform state for one point in a reusable gameplay tween.
/// AI: This stays Transform-only so it can drive any 3D entity with compatible presentation needs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameTransformTweenSample {
    pub transform: Transform,
    pub progress: f32,
    pub is_complete: bool,
}

/// HUMAN: Opacity and scale sample for a location intro reveal.
/// AI: Shared by 3D surfaces and safe-area UI overlays that mirror the same intro timing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocationIntroTweenSample {
    pub scale: f32,
    pub opacity: f32,
}

pub const GAME_TWEEN_INSTA_FLIP_SECONDS: f32 = 0.0;
pub const GAME_TWEEN_FLIP_SECONDS: f32 = 1.0;
pub const GAME_TWEEN_SWAN_FLIP_SECONDS: f32 = 1.0;
pub const GAME_TWEEN_SWAN_SCALE_MULTIPLIER: f32 = 1.25;
pub const GAME_TWEEN_SWAN_SCALE_UP_SECONDS: f32 = 0.25;
pub const GAME_TWEEN_SWAN_SCALE_HOLD_SECONDS: f32 = 0.5;
pub const GAME_TWEEN_SWAN_SCALE_DOWN_SECONDS: f32 = 0.25;
pub const GAME_TWEEN_DEAL_SLIDE_SECONDS: f32 = 0.5;
pub const GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS: f32 = 0.5;
pub const GAME_TWEEN_CARD_MOVE_SCALE_MULTIPLIER: f32 = 1.1;
pub const GAME_TWEEN_DRAG_PLACE_SECONDS: f32 = 0.25;
pub const GAME_TWEEN_DRAG_SCALE_SECONDS: f32 = 0.25;
pub const GAME_TWEEN_LOCATION_INTRO_SECONDS: f32 = 0.5;
pub const GAME_TWEEN_LOCATION_INTRO_STAGGER_SECONDS: f32 = 1.0;
pub const GAME_TWEEN_LOCATION_INTRO_POST_REVEAL_HOLD_SECONDS: f32 = 0.5;
pub const GAME_TWEEN_LOCATION_INTRO_START_SCALE: f32 = 1.5;

pub const GAME_TWEEN_CARD_PRESETS: [GameTweenPreset; 6] = [
    GameTweenPreset::InstaFlip,
    GameTweenPreset::Flip,
    GameTweenPreset::SwanFlip,
    GameTweenPreset::DealSlide,
    GameTweenPreset::CardMoveToSlot,
    GameTweenPreset::DragPlace,
];

pub const GAME_TWEEN_ALL_PRESETS: [GameTweenPreset; 7] = [
    GameTweenPreset::InstaFlip,
    GameTweenPreset::Flip,
    GameTweenPreset::SwanFlip,
    GameTweenPreset::DealSlide,
    GameTweenPreset::CardMoveToSlot,
    GameTweenPreset::DragPlace,
    GameTweenPreset::LocationIntro,
];

pub const fn game_tween_duration_seconds(preset: GameTweenPreset) -> f32 {
    match preset {
        GameTweenPreset::InstaFlip => GAME_TWEEN_INSTA_FLIP_SECONDS,
        GameTweenPreset::Flip => GAME_TWEEN_FLIP_SECONDS,
        GameTweenPreset::SwanFlip => GAME_TWEEN_SWAN_FLIP_SECONDS,
        GameTweenPreset::DealSlide => GAME_TWEEN_DEAL_SLIDE_SECONDS,
        GameTweenPreset::CardMoveToSlot => GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS,
        GameTweenPreset::DragPlace => GAME_TWEEN_DRAG_PLACE_SECONDS,
        GameTweenPreset::LocationIntro => GAME_TWEEN_LOCATION_INTRO_SECONDS,
    }
}

pub fn sample_transform_tween(
    start_transform: Transform,
    target_transform: Transform,
    elapsed_seconds: f32,
    preset: GameTweenPreset,
) -> GameTransformTweenSample {
    let duration_seconds = game_tween_duration_seconds(preset);
    if duration_seconds <= 0.0 {
        return GameTransformTweenSample {
            transform: target_transform,
            progress: 1.0,
            is_complete: true,
        };
    }

    let progress = (elapsed_seconds.max(0.0) / duration_seconds).clamp(0.0, 1.0);
    let eased_progress = ease_out_cubic(progress);
    GameTransformTweenSample {
        transform: lerp_transform(start_transform, target_transform, eased_progress),
        progress,
        is_complete: progress >= 1.0,
    }
}

pub fn lerp_transform(
    start_transform: Transform,
    target_transform: Transform,
    progress: f32,
) -> Transform {
    let progress = progress.clamp(0.0, 1.0);
    Transform {
        translation: start_transform
            .translation
            .lerp(target_transform.translation, progress),
        rotation: start_transform
            .rotation
            .slerp(target_transform.rotation, progress),
        scale: start_transform.scale.lerp(target_transform.scale, progress),
    }
}

pub fn sample_card_move_scale_multiplier(progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    if progress <= 0.5 {
        1.0_f32.lerp(GAME_TWEEN_CARD_MOVE_SCALE_MULTIPLIER, progress * 2.0)
    } else {
        GAME_TWEEN_CARD_MOVE_SCALE_MULTIPLIER.lerp(1.0, (progress - 0.5) * 2.0)
    }
}

pub fn sample_card_move_scale(
    start_transform: Transform,
    target_transform: Transform,
    start_world_units_per_pixel: f32,
    target_world_units_per_pixel: f32,
    current_world_units_per_pixel: f32,
    eased_progress: f32,
    scale_multiplier: f32,
) -> Vec3 {
    let start_apparent_scale = start_transform.scale / start_world_units_per_pixel;
    let target_apparent_scale = target_transform.scale / target_world_units_per_pixel;
    let apparent_scale =
        start_apparent_scale.lerp(target_apparent_scale, eased_progress) * scale_multiplier;
    let max_pulsed_start_apparent_scale =
        start_apparent_scale * GAME_TWEEN_CARD_MOVE_SCALE_MULTIPLIER;
    let clamped_apparent_scale = apparent_scale.min(max_pulsed_start_apparent_scale);

    clamped_apparent_scale * current_world_units_per_pixel
}

pub fn sample_flip_y_rotation(
    start_y_rotation: f32,
    target_y_rotation: f32,
    elapsed_seconds: f32,
    preset: GameTweenPreset,
) -> (f32, f32) {
    let duration_seconds = game_tween_duration_seconds(preset).max(f32::EPSILON);
    let progress = (elapsed_seconds.max(0.0) / duration_seconds).clamp(0.0, 1.0);
    (
        start_y_rotation.lerp(target_y_rotation, ease_out_cubic(progress)),
        progress,
    )
}

pub fn sample_swan_scale_multiplier(elapsed_seconds: f32) -> f32 {
    let elapsed_seconds = elapsed_seconds.max(0.0);
    if elapsed_seconds < GAME_TWEEN_SWAN_SCALE_UP_SECONDS {
        let progress = (elapsed_seconds / GAME_TWEEN_SWAN_SCALE_UP_SECONDS).clamp(0.0, 1.0);
        1.0_f32.lerp(GAME_TWEEN_SWAN_SCALE_MULTIPLIER, ease_out_cubic(progress))
    } else if elapsed_seconds
        < GAME_TWEEN_SWAN_SCALE_UP_SECONDS + GAME_TWEEN_SWAN_SCALE_HOLD_SECONDS
    {
        GAME_TWEEN_SWAN_SCALE_MULTIPLIER
    } else if elapsed_seconds
        < GAME_TWEEN_SWAN_SCALE_UP_SECONDS
            + GAME_TWEEN_SWAN_SCALE_HOLD_SECONDS
            + GAME_TWEEN_SWAN_SCALE_DOWN_SECONDS
    {
        let down_elapsed = elapsed_seconds
            - (GAME_TWEEN_SWAN_SCALE_UP_SECONDS + GAME_TWEEN_SWAN_SCALE_HOLD_SECONDS);
        let progress = (down_elapsed / GAME_TWEEN_SWAN_SCALE_DOWN_SECONDS).clamp(0.0, 1.0);
        GAME_TWEEN_SWAN_SCALE_MULTIPLIER.lerp(1.0, ease_out_cubic(progress))
    } else {
        1.0
    }
}

pub fn sample_location_intro_tween(
    location_index: usize,
    elapsed_seconds: f32,
) -> LocationIntroTweenSample {
    let delay_seconds = location_index as f32 * GAME_TWEEN_LOCATION_INTRO_STAGGER_SECONDS;
    let progress =
        ((elapsed_seconds - delay_seconds) / GAME_TWEEN_LOCATION_INTRO_SECONDS).clamp(0.0, 1.0);
    let eased_progress = ease_out_cubic(progress);

    LocationIntroTweenSample {
        scale: GAME_TWEEN_LOCATION_INTRO_START_SCALE.lerp(1.0, eased_progress),
        opacity: eased_progress,
    }
}

pub fn location_intro_hold_gate_seconds(location_count: usize) -> f32 {
    let final_location_index = location_count.saturating_sub(1) as f32;
    (final_location_index * GAME_TWEEN_LOCATION_INTRO_STAGGER_SECONDS)
        + GAME_TWEEN_LOCATION_INTRO_SECONDS
        + GAME_TWEEN_LOCATION_INTRO_POST_REVEAL_HOLD_SECONDS
}

pub fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress.clamp(0.0, 1.0)).powi(3)
}

#[cfg(test)]
#[path = "../../tests/runtime/tweens/tweens_mod_tests.rs"]
mod tweens_mod_tests;
