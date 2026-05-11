use bevy::prelude::*;

use crate::runtime::resources::CardSlotSide;

pub const CARD_GESTURE_DRAG_THRESHOLD: f32 = 8.0;

/// HUMAN: Low-level pointer data used to classify card gestures.
/// AI: Positions are in aspect-ratio-safe GameView coordinates, not raw window pixels.
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

/// HUMAN: Single active card gesture focus for GameView hand-card interactions.
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
        self.target_transform = Some(target_transform);
        self.target_slot = None;
        self.resolved_destination = None;
        self.drag_elapsed_seconds = 0.0;
    }

    pub fn return_to_source(&mut self) {
        self.state = CardGestureState::Returning;
        self.pointer = None;
        self.target_transform = self.source_transform;
        self.target_slot = None;
        self.resolved_destination = self
            .active_hand_index
            .map(|hand_index| CardGestureDestination::HandCardSlot { hand_index });
        self.drag_elapsed_seconds = 0.0;
    }

    pub fn place(&mut self, target_slot: CardGestureSlotTarget, target_transform: Transform) {
        self.state = CardGestureState::Placed;
        self.pointer = None;
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
mod tests {
    use super::*;

    fn source_transform() -> Transform {
        Transform::from_translation(Vec3::new(1.0, 2.0, 0.3)).with_scale(Vec3::splat(0.5))
    }

    #[test]
    fn press_records_one_active_focus_and_source_transform() {
        let mut model = CardGestureModel::default();
        let source = source_transform();

        assert!(model.press(2, Vec2::new(10.0, 20.0), Vec2::new(12.0, 24.0), source));

        assert_eq!(model.state, CardGestureState::Pressed);
        assert_eq!(model.active_hand_index, Some(2));
        assert_eq!(model.source_transform, Some(source));
        assert!(model.is_active_for(2));
    }

    #[test]
    fn selected_card_can_return_to_source() {
        let mut model = CardGestureModel::default();
        let source = source_transform();
        let target = Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.2));

        assert!(model.press(1, Vec2::ZERO, Vec2::ZERO, source));
        model.select(target);
        assert_eq!(model.state, CardGestureState::SelectedInspecting);
        assert_eq!(model.target_transform, Some(target));

        model.return_to_source();
        assert_eq!(model.state, CardGestureState::Returning);
        assert_eq!(model.target_transform, Some(source));
        assert_eq!(
            model.resolved_destination,
            Some(CardGestureDestination::HandCardSlot { hand_index: 1 })
        );
    }

    #[test]
    fn threshold_boundary_converts_to_drag_once() {
        let mut model = CardGestureModel::default();
        assert!(model.press(0, Vec2::ZERO, Vec2::ZERO, source_transform()));

        model.update_pointer(
            Vec2::new(CARD_GESTURE_DRAG_THRESHOLD - 0.1, 0.0),
            CARD_GESTURE_DRAG_THRESHOLD,
        );
        assert_eq!(model.state, CardGestureState::Pressed);

        model.update_pointer(
            Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
            CARD_GESTURE_DRAG_THRESHOLD,
        );
        assert_eq!(model.state, CardGestureState::Dragging);
        assert!(
            model
                .pointer
                .expect("pointer should stay active while dragging")
                .has_crossed_drag_threshold
        );
    }

    #[test]
    fn pointer_keeps_card_center_offset_for_dragging() {
        let pointer = PointerGestureModel::new(Vec2::new(10.0, 20.0), Vec2::new(18.0, 32.0));

        assert_eq!(pointer.card_center_offset, Vec2::new(8.0, 12.0));
        assert_eq!(pointer.current_card_center(), Vec2::new(18.0, 32.0));
    }

    #[test]
    fn active_pointer_gesture_cannot_be_replaced() {
        let mut model = CardGestureModel::default();
        let first_source = source_transform();
        let second_source = Transform::from_translation(Vec3::new(8.0, 9.0, 0.3));

        assert!(model.press(0, Vec2::ZERO, Vec2::ZERO, first_source));
        model.update_pointer(
            Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
            CARD_GESTURE_DRAG_THRESHOLD,
        );

        assert!(!model.press(1, Vec2::splat(5.0), Vec2::splat(5.0), second_source));
        assert_eq!(model.state, CardGestureState::Dragging);
        assert_eq!(model.active_hand_index, Some(0));
        assert_eq!(model.source_transform, Some(first_source));
    }

    #[test]
    fn successful_drag_resolves_to_location_card_slot() {
        let mut model = CardGestureModel::default();
        let target_slot = CardGestureSlotTarget::new(2, CardSlotSide::LocalPlayer, 3);

        assert!(model.press(0, Vec2::ZERO, Vec2::ZERO, source_transform()));
        model.place(target_slot, Transform::from_translation(Vec3::X));

        assert_eq!(
            model.resolved_destination,
            Some(CardGestureDestination::LocationCardSlot {
                location_index: 2,
                slot_index: 3,
            })
        );
    }
}
