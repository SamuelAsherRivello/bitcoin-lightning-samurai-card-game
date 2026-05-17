use bevy::prelude::*;

use crate::runtime::resources::ActiveView;

pub const SCREEN_TRANSITION_TOTAL_DURATION_SECONDS: f32 = 0.5;
pub const SCREEN_TRANSITION_DURATION_MIN_SECONDS: f32 = 0.01;
pub const SCREEN_TRANSITION_BLACK_HOLD_SECONDS: f32 = 0.1;

/// HUMAN: Ordered phase state for fullscreen screen-transition fades.
/// AI: Keep screen swaps gated at full black by moving through this explicit phase machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreenTransitionPhase {
    StartupFadeIn,
    Idle,
    FadeOutPendingSwitch,
    SwitchAtBlack,
    HoldAtBlack,
    FadeInAfterSwitch,
}

/// HUMAN: Runtime model for fullscreen view transitions and pending target view requests.
/// AI: Queue one follow-up request while active; resolve transitions centrally via transition_update_system.
#[derive(Clone, Debug, Resource)]
pub struct ScreenTransitionResource {
    pub phase: ScreenTransitionPhase,
    pub phase_elapsed_seconds: f32,
    pub overlay_alpha: f32,
    pub color: Color,
    pub total_duration_seconds: f32,
    pub pending_view: Option<ActiveView>,
    pub queued_view: Option<ActiveView>,
}

impl Default for ScreenTransitionResource {
    fn default() -> Self {
        Self {
            phase: ScreenTransitionPhase::StartupFadeIn,
            phase_elapsed_seconds: 0.0,
            overlay_alpha: 1.0,
            color: Color::srgba(0.0, 0.0, 0.0, 1.0),
            total_duration_seconds: SCREEN_TRANSITION_TOTAL_DURATION_SECONDS,
            pending_view: None,
            queued_view: None,
        }
    }
}

impl ScreenTransitionResource {
    pub fn request_view_change(&mut self, current_view: ActiveView, target_view: ActiveView) {
        if target_view == current_view {
            return;
        }

        match self.phase {
            ScreenTransitionPhase::Idle => {
                self.pending_view = Some(target_view);
                self.phase_elapsed_seconds = 0.0;
                self.overlay_alpha = 0.0;
                self.phase = ScreenTransitionPhase::FadeOutPendingSwitch;
            }
            ScreenTransitionPhase::StartupFadeIn
            | ScreenTransitionPhase::FadeOutPendingSwitch
            | ScreenTransitionPhase::SwitchAtBlack
            | ScreenTransitionPhase::HoldAtBlack
            | ScreenTransitionPhase::FadeInAfterSwitch => {
                self.queued_view = Some(target_view);
            }
        }
    }
}
