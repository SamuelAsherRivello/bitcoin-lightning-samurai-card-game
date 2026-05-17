use bevy::math::curve::{
    Curve,
    easing::{EaseFunction, EasingCurve},
};
use bevy::prelude::*;

use crate::runtime::components::AppSceneCamera;
use crate::runtime::components::ScreenTransitionOverlay;
use crate::runtime::resources::{
    ActiveCardModel, CardFlipState, MetaGameSettingsModel, SCREEN_TRANSITION_BLACK_HOLD_SECONDS,
    SCREEN_TRANSITION_DURATION_MIN_SECONDS, ScreenTransitionPhase, ScreenTransitionResource,
};
use crate::runtime::systems::{ViewChangeParams, composed_rotation_for_face};

/// HUMAN: Drives fullscreen transition overlay fading and gated view changes.
/// AI: Apply the requested view switch only at full-black, then fade back in to avoid popping.
pub fn transition_update_system(
    time: Res<Time>,
    active_card_model: Res<ActiveCardModel>,
    flip_state: Res<CardFlipState>,
    settings: Res<MetaGameSettingsModel>,
    mut transition: ResMut<ScreenTransitionResource>,
    mut params: ViewChangeParams,
    mut overlay_query: Query<&mut BackgroundColor, With<ScreenTransitionOverlay>>,
) {
    let half_duration =
        (transition.total_duration_seconds * 0.5).max(SCREEN_TRANSITION_DURATION_MIN_SECONDS);

    match transition.phase {
        ScreenTransitionPhase::StartupFadeIn => {
            let eased = advance_transition_phase_progress(
                &mut transition,
                time.delta_secs(),
                half_duration,
                EaseFunction::QuadraticIn,
            );
            transition.overlay_alpha = 1.0 - eased;
            if transition.phase_elapsed_seconds >= half_duration {
                transition.overlay_alpha = 0.0;
                transition.phase_elapsed_seconds = 0.0;
                transition.phase = ScreenTransitionPhase::Idle;
            }
        }
        ScreenTransitionPhase::Idle => {
            if let Some(queued) = transition.queued_view.take() {
                transition.pending_view = Some(queued);
                transition.phase_elapsed_seconds = 0.0;
                transition.overlay_alpha = 0.0;
                transition.phase = ScreenTransitionPhase::FadeOutPendingSwitch;
            }
        }
        ScreenTransitionPhase::FadeOutPendingSwitch => {
            let eased = advance_transition_phase_progress(
                &mut transition,
                time.delta_secs(),
                half_duration,
                EaseFunction::QuadraticOut,
            );
            transition.overlay_alpha = eased;
            if transition.phase_elapsed_seconds >= half_duration {
                transition.overlay_alpha = 1.0;
                transition.phase_elapsed_seconds = 0.0;
                transition.phase = ScreenTransitionPhase::SwitchAtBlack;
            }
        }
        ScreenTransitionPhase::SwitchAtBlack => {
            if let Some(target) = transition.pending_view.take() {
                let initial_rotation =
                    composed_rotation_for_face(&params.card_state, flip_state.visible_face);
                params.transition_to_requested_view(
                    target,
                    &settings,
                    &active_card_model,
                    flip_state.visible_face,
                    initial_rotation,
                );
            }
            transition.phase_elapsed_seconds = 0.0;
            transition.phase = ScreenTransitionPhase::HoldAtBlack;
        }
        ScreenTransitionPhase::HoldAtBlack => {
            transition.phase_elapsed_seconds = (transition.phase_elapsed_seconds
                + time.delta_secs().max(0.0))
            .min(SCREEN_TRANSITION_BLACK_HOLD_SECONDS);
            transition.overlay_alpha = 1.0;
            if transition.phase_elapsed_seconds >= SCREEN_TRANSITION_BLACK_HOLD_SECONDS {
                transition.phase_elapsed_seconds = 0.0;
                transition.phase = ScreenTransitionPhase::FadeInAfterSwitch;
            }
        }
        ScreenTransitionPhase::FadeInAfterSwitch => {
            let eased = advance_transition_phase_progress(
                &mut transition,
                time.delta_secs(),
                half_duration,
                EaseFunction::QuadraticIn,
            );
            transition.overlay_alpha = 1.0 - eased;
            if transition.phase_elapsed_seconds >= half_duration {
                transition.overlay_alpha = 0.0;
                transition.phase_elapsed_seconds = 0.0;
                if let Some(queued) = transition.queued_view.take() {
                    transition.pending_view = Some(queued);
                    transition.phase_elapsed_seconds = 0.0;
                    transition.overlay_alpha = 0.0;
                    transition.phase = ScreenTransitionPhase::FadeOutPendingSwitch;
                } else {
                    transition.phase = ScreenTransitionPhase::Idle;
                }
            }
        }
    }

    for mut background in &mut overlay_query {
        let mut color = transition.color;
        color.set_alpha(transition.overlay_alpha.clamp(0.0, 1.0));
        background.0 = color;
    }
}

/// HUMAN: Keeps the fullscreen transition overlay bound to the shared AppScene camera.
/// AI: Retargeting is centralized so transitions no longer depend on any 2D camera.
pub fn transition_overlay_target_camera_update_system(
    mut commands: Commands,
    overlay_query: Query<(Entity, Option<&UiTargetCamera>), With<ScreenTransitionOverlay>>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
) {
    let Ok(app_camera) = app_camera_query.single() else {
        return;
    };

    for (overlay_entity, current_target) in &overlay_query {
        let needs_update = current_target
            .map(|target| target.0 != app_camera)
            .unwrap_or(true);
        if needs_update {
            commands
                .entity(overlay_entity)
                .insert(UiTargetCamera(app_camera));
        }
    }
}

fn advance_transition_phase_progress(
    transition: &mut ScreenTransitionResource,
    delta_seconds: f32,
    duration_seconds: f32,
    ease: EaseFunction,
) -> f32 {
    transition.phase_elapsed_seconds =
        (transition.phase_elapsed_seconds + delta_seconds.max(0.0)).min(duration_seconds);
    let progress = (transition.phase_elapsed_seconds / duration_seconds).clamp(0.0, 1.0);
    EasingCurve::new(0.0, 1.0, ease)
        .sample(progress)
        .unwrap_or(progress)
}
