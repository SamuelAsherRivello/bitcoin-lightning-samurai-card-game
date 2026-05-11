use bevy::prelude::*;

use crate::runtime::resources::DebugDrawingTarget;

/// HUMAN: Temporary runtime visual annotation for discussing game-scene areas.
/// AI: Debug drawings are removable developer aids, not final art or gameplay UI.
#[derive(Component, Debug)]
pub struct DebugDrawing {
    pub target: DebugDrawingTarget,
    pub generation: u64,
}

impl DebugDrawing {
    pub const fn new(target: DebugDrawingTarget, generation: u64) -> Self {
        Self { target, generation }
    }
}
