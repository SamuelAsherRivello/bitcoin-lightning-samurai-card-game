use bevy::prelude::*;

use crate::runtime::resources::CardSlotSide;

pub const CARD_GESTURE_DRAG_THRESHOLD: f32 = 8.0;

/// HUMAN: Low-level pointer data used to classify card gestures.
/// AI: Positions are in aspect-ratio-safe GameScene coordinates, not raw window pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerGestureModel {
    pub start_position: Vec2,
    pub current_position: Vec2,
    pub card_center_offset: Vec2,
    pub has_crossed_drag_threshold: bool,
}

impl PointerGestureModel {
    pub fn new(start_position: Vec2, card_center_position: Vec2) -> Self {
        Self {
            start_position,
            current_position: start_position,
            card_center_offset: card_center_position - start_position,
            has_crossed_drag_threshold: false,
        }
    }

    pub fn update(&mut self, current_position: Vec2, drag_threshold: f32) {
        self.current_position = current_position;
        if self.start_position.distance(current_position) >= drag_threshold {
            self.has_crossed_drag_threshold = true;
        }
    }

    pub fn current_card_center(self) -> Vec2 {
        self.current_position + self.card_center_offset
    }
}

/// HUMAN: High-level gesture state for the one active local hand card.
/// AI: Keep this as data; visual interpolation belongs in card gesture systems.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardGestureState {
    Idle,
    Pressed,
    SelectedInspecting,
    Dragging,
    Returning,
    Placed,
}

/// HUMAN: Current card slot selected as a gesture drop target.
/// AI: This mirrors CardSlotModel identity without borrowing the slot board resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardGestureSlotTarget {
    pub location_index: usize,
    pub side: CardSlotSide,
    pub slot_index: usize,
}

impl CardGestureSlotTarget {
    pub const fn new(location_index: usize, side: CardSlotSide, slot_index: usize) -> Self {
        Self {
            location_index,
            side,
            slot_index,
        }
    }
}

/// HUMAN: Final legal destination for a resolved hand-card drag.
/// AI: Keep this explicit so cancelled and successful drags cannot resolve off-board.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardGestureDestination {
    HandCardSlot {
        hand_index: usize,
    },
    LocationCardSlot {
        location_index: usize,
        slot_index: usize,
    },
}

/// HUMAN: Single active card gesture focus for GameScene hand-card interactions.
/// AI: Store source and target transforms so selection, drag, placement, and return do not conflict.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct CardGestureModel {
    pub state: CardGestureState,
    pub active_hand_index: Option<usize>,
    pub pointer: Option<PointerGestureModel>,
    pub source_transform: Option<Transform>,
    pub target_transform: Option<Transform>,
    pub target_slot: Option<CardGestureSlotTarget>,
    pub resolved_destination: Option<CardGestureDestination>,
    pub drag_elapsed_seconds: f32,
    pub transition_start_transform: Option<Transform>,
    pub transition_elapsed_seconds: f32,
}

impl Default for CardGestureModel {
    fn default() -> Self {
        Self {
            state: CardGestureState::Idle,
            active_hand_index: None,
            pointer: None,
            source_transform: None,
            target_transform: None,
            target_slot: None,
            resolved_destination: None,
            drag_elapsed_seconds: 0.0,
            transition_start_transform: None,
            transition_elapsed_seconds: 0.0,
        }
    }
}

impl CardGestureModel {
    pub fn press(
        &mut self,
        hand_index: usize,
        position: Vec2,
        card_center_position: Vec2,
        source_transform: Transform,
    ) -> bool {
        if self.pointer.is_some() {
            return false;
        }

        self.state = CardGestureState::Pressed;
        self.active_hand_index = Some(hand_index);
        self.pointer = Some(PointerGestureModel::new(position, card_center_position));
        self.source_transform = Some(source_transform);
        self.target_transform = None;
        self.target_slot = None;
        self.resolved_destination = None;
        self.drag_elapsed_seconds = 0.0;
        self.transition_start_transform = None;
        self.transition_elapsed_seconds = 0.0;
        true
    }

    pub fn update_pointer(&mut self, position: Vec2, drag_threshold: f32) -> bool {
        let Some(pointer) = &mut self.pointer else {
            return false;
        };
        pointer.update(position, drag_threshold);
        if self.state == CardGestureState::Pressed && pointer.has_crossed_drag_threshold {
            self.state = CardGestureState::Dragging;
            self.drag_elapsed_seconds = 0.0;
            return true;
        }
        false
    }

    pub fn select(&mut self, target_transform: Transform) {
        self.state = CardGestureState::SelectedInspecting;
        self.pointer = None;
        self.transition_start_transform = self.target_transform.or(self.source_transform);
        self.transition_elapsed_seconds = 0.0;
        self.target_transform = Some(target_transform);
        self.target_slot = None;
        self.resolved_destination = None;
        self.drag_elapsed_seconds = 0.0;
    }

    pub fn return_to_source(&mut self) {
        self.state = CardGestureState::Returning;
        self.pointer = None;
        self.transition_start_transform = self.target_transform.or(self.source_transform);
        self.transition_elapsed_seconds = 0.0;
        self.target_transform = self.source_transform;
        self.target_slot = None;
        self.resolved_destination = self
            .active_hand_index
            .map(|hand_index| CardGestureDestination::HandCardSlot { hand_index });
        self.drag_elapsed_seconds = 0.0;
    }

    pub fn return_to_hand_transform(&mut self, hand_index: usize, target_transform: Transform) {
        self.state = CardGestureState::Returning;
        self.pointer = None;
        self.transition_start_transform = self.target_transform.or(self.source_transform);
        self.transition_elapsed_seconds = 0.0;
        self.target_transform = Some(target_transform);
        self.target_slot = None;
        self.resolved_destination = Some(CardGestureDestination::HandCardSlot { hand_index });
        self.drag_elapsed_seconds = 0.0;
    }

    pub fn place(&mut self, target_slot: CardGestureSlotTarget, target_transform: Transform) {
        self.state = CardGestureState::Placed;
        self.pointer = None;
        self.transition_start_transform = self.target_transform.or(self.source_transform);
        self.transition_elapsed_seconds = 0.0;
        self.target_slot = Some(target_slot);
        self.target_transform = Some(target_transform);
        self.resolved_destination = Some(CardGestureDestination::LocationCardSlot {
            location_index: target_slot.location_index,
            slot_index: target_slot.slot_index,
        });
        self.drag_elapsed_seconds = 0.0;
    }

    pub fn clear_if_returned(&mut self) {
        if self.state == CardGestureState::Returning {
            *self = Self::default();
        }
    }

    pub fn is_active_for(&self, hand_index: usize) -> bool {
        self.active_hand_index == Some(hand_index)
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/card_gesture_model_tests.rs"]
mod card_gesture_model_tests;
