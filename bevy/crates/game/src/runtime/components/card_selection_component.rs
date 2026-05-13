use bevy::prelude::*;

use crate::runtime::resources::{ActiveView, CardSlotSide, MatchPlayerSide};

/// HUMAN: Rendered card source that can enter selected inspection when eligible.
/// AI: Keep this render-facing; durable gameplay state remains in card models/resources.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub enum CardSelectionSource {
    CardViewBundle,
    LocalHand {
        hand_index: usize,
    },
    LocalLocation {
        location_index: usize,
        slot_index: usize,
        hand_index: usize,
    },
    OpponentHand {
        owner: MatchPlayerSide,
        hand_index: usize,
    },
    OpponentLocation {
        owner: MatchPlayerSide,
        side: CardSlotSide,
        location_index: usize,
        slot_index: usize,
    },
    ScreenCard {
        view: ActiveView,
    },
}

/// HUMAN: Movement state used to decide whether a rendered card is selectable.
/// AI: Use Moving for any transient tween that should block click-to-select.
#[derive(Clone, Copy, Component, Debug, Default, Eq, PartialEq)]
pub enum CardSelectionMovementState {
    #[default]
    Stationary,
    Pressed,
    Dragging,
    Returning,
    Dealing,
    Moving,
    Revealing,
    Flipping,
}

impl CardSelectionMovementState {
    pub const fn is_stationary(self) -> bool {
        matches!(self, Self::Stationary)
    }
}

/// HUMAN: Explicit selectable affordance for rendered card roots.
/// AI: Selection systems still check face visibility and movement before selecting.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct SelectableCard {
    pub source: CardSelectionSource,
    pub movement_state: CardSelectionMovementState,
}

impl SelectableCard {
    pub const fn new(source: CardSelectionSource) -> Self {
        Self {
            source,
            movement_state: CardSelectionMovementState::Stationary,
        }
    }

    pub const fn with_movement_state(
        source: CardSelectionSource,
        movement_state: CardSelectionMovementState,
    ) -> Self {
        Self {
            source,
            movement_state,
        }
    }

    pub const fn is_stationary(self) -> bool {
        self.movement_state.is_stationary()
    }
}

/// HUMAN: Marker for the fullscreen selected-card modal dim layer.
/// AI: Rendering and input behavior are driven by SelectedCardModalModel.
#[derive(Component, Debug, Default)]
pub struct SelectedCardModalBackdrop;

#[cfg(test)]
#[path = "../../tests/runtime/components/card_selection_component_tests.rs"]
mod card_selection_component_tests;
