use bevy::prelude::*;

pub const SELECTED_CARD_MODAL_FADE_SECONDS: f32 = 0.5;
pub const SELECTED_CARD_MODAL_MAX_OPACITY: f32 = 0.9;
pub const SELECTED_CARD_MODAL_SETTLE_EPSILON: f32 = 0.001;

/// HUMAN: Tracks the one card currently inspected as a modal selection.
/// AI: Keep this transient; gameplay ownership and legality remain in card state models.
#[derive(Clone, Debug, PartialEq, Resource)]
pub struct SelectedCardModalModel {
    pub selected_entity: Option<Entity>,
    pub source_transform: Option<Transform>,
    pub target_transform: Option<Transform>,
    pub fade_elapsed_seconds: f32,
    pub max_opacity: f32,
    pub dismiss_pending: bool,
    pub suppress_next_pointer_dismiss: bool,
    pub press_candidate: Option<SelectedCardPressCandidate>,
}

impl Default for SelectedCardModalModel {
    fn default() -> Self {
        Self {
            selected_entity: None,
            source_transform: None,
            target_transform: None,
            fade_elapsed_seconds: 0.0,
            max_opacity: SELECTED_CARD_MODAL_MAX_OPACITY,
            dismiss_pending: false,
            suppress_next_pointer_dismiss: false,
            press_candidate: None,
        }
    }
}

impl SelectedCardModalModel {
    pub fn is_active(&self) -> bool {
        self.selected_entity.is_some()
    }

    pub fn blocks_lower_interactions(&self) -> bool {
        self.is_active()
    }

    pub fn begin_press_candidate(
        &mut self,
        entity: Entity,
        position: Vec2,
        source_transform: Transform,
    ) {
        if self.is_active() {
            return;
        }
        self.press_candidate = Some(SelectedCardPressCandidate {
            entity,
            start_position: position,
            source_transform,
            has_crossed_drag_threshold: false,
        });
    }

    pub fn update_press_candidate(&mut self, position: Vec2, drag_threshold: f32) {
        let Some(candidate) = &mut self.press_candidate else {
            return;
        };
        if candidate.start_position.distance(position) >= drag_threshold {
            candidate.has_crossed_drag_threshold = true;
        }
    }

    pub fn take_click_candidate(&mut self) -> Option<SelectedCardPressCandidate> {
        let candidate = self.press_candidate.take()?;
        (!candidate.has_crossed_drag_threshold).then_some(candidate)
    }

    pub fn cancel_press_candidate(&mut self) {
        self.press_candidate = None;
    }

    pub fn select_entity(
        &mut self,
        entity: Entity,
        source_transform: Transform,
        target_transform: Transform,
    ) {
        self.selected_entity = Some(entity);
        self.source_transform = Some(source_transform);
        self.target_transform = Some(target_transform);
        self.fade_elapsed_seconds = 0.0;
        self.dismiss_pending = false;
        self.suppress_next_pointer_dismiss = true;
        self.press_candidate = None;
    }

    pub fn take_suppressed_pointer_dismiss(&mut self) -> bool {
        std::mem::take(&mut self.suppress_next_pointer_dismiss)
    }

    pub fn request_dismiss(&mut self) {
        if self.is_active() {
            self.dismiss_pending = true;
        }
    }

    pub fn advance_fade(&mut self, delta_seconds: f32) {
        let delta_seconds = delta_seconds.max(0.0);
        if self.dismiss_pending {
            self.fade_elapsed_seconds = (self.fade_elapsed_seconds - delta_seconds)
                .clamp(0.0, SELECTED_CARD_MODAL_FADE_SECONDS);
        } else if self.is_active() {
            self.fade_elapsed_seconds = (self.fade_elapsed_seconds + delta_seconds)
                .clamp(0.0, SELECTED_CARD_MODAL_FADE_SECONDS);
        }
    }

    pub fn advance_fade_with_interpolation(&mut self, interpolation: f32) {
        if !self.is_active() {
            return;
        }
        let target_progress = if self.dismiss_pending { 0.0 } else { 1.0 };
        let next_progress = self
            .fade_progress()
            .lerp(target_progress, interpolation.clamp(0.0, 1.0));
        self.fade_elapsed_seconds = next_progress * SELECTED_CARD_MODAL_FADE_SECONDS;
    }

    pub fn opacity(&self) -> f32 {
        self.max_opacity * self.fade_progress()
    }

    pub fn fade_progress(&self) -> f32 {
        (self.fade_elapsed_seconds / SELECTED_CARD_MODAL_FADE_SECONDS).clamp(0.0, 1.0)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// HUMAN: Press candidate that may become a selected card if the pointer does not drag.
/// AI: This mirrors CardGestureModel's click-vs-drag threshold for passive card roots.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectedCardPressCandidate {
    pub entity: Entity,
    pub start_position: Vec2,
    pub source_transform: Transform,
    pub has_crossed_drag_threshold: bool,
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/selected_card_modal_model_tests.rs"]
mod selected_card_modal_model_tests;
