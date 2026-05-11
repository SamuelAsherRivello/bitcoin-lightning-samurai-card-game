use bevy::prelude::*;

use crate::runtime::resources::CardSlotSide;

/// HUMAN: Identifies a local hand card that can be inspected or dragged.
/// AI: Store stable hand index here; gesture state lives in CardGestureModel.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct HandCardGestureTarget {
    pub hand_index: usize,
}

impl HandCardGestureTarget {
    pub const fn new(hand_index: usize) -> Self {
        Self { hand_index }
    }
}

/// HUMAN: Identifies a board slot that card gestures can test as a drop target.
/// AI: Slot legality stays in CardSlotBoardModel; this component only tags presentation.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardSlotGestureTarget {
    pub location_index: usize,
    pub side: CardSlotSide,
    pub slot_index: usize,
}

impl CardSlotGestureTarget {
    pub const fn new(location_index: usize, side: CardSlotSide, slot_index: usize) -> Self {
        Self {
            location_index,
            side,
            slot_index,
        }
    }
}

/// HUMAN: Presentation marker for a card currently managed by gesture behavior.
/// AI: Pair with CardView roots; model state decides selected, dragged, placed, or returning.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CardGestureView;

/// HUMAN: User-facing highlight for a location's available local drop area.
/// AI: This is gameplay feedback, not DebugHUD/debug drawing, so visibility follows gesture state.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct DropTargetHint {
    pub location_index: usize,
}

impl DropTargetHint {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}
