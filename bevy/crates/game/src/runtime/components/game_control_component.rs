use bevy::prelude::*;

/// HUMAN: User-facing gameplay control actions in GameScene.
/// AI: Button systems route through this enum instead of relying on text labels.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub enum GameControlAction {
    Mode,
    Restart,
    Undo,
    EndRound,
}

/// HUMAN: Marker for a clickable gameplay control button.
/// AI: Keep visual button components separate from action routing.
#[derive(Component, Debug)]
pub struct GameControlButton {
    pub action: GameControlAction,
}

impl GameControlButton {
    pub const fn new(action: GameControlAction) -> Self {
        Self { action }
    }
}

/// HUMAN: Marker for gameplay control labels that need runtime text updates.
/// AI: Text update systems should match labels by action instead of tree shape.
#[derive(Component, Debug)]
pub struct GameControlLabel {
    pub action: GameControlAction,
}

impl GameControlLabel {
    pub const fn new(action: GameControlAction) -> Self {
        Self { action }
    }
}
