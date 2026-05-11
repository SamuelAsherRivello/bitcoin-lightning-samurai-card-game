use bevy::prelude::*;

/// HUMAN: Marker for Card UI developer-visible egui surfaces.
/// AI: Keep Card UI markers separate from DebugHUD and debug drawing annotations.
#[derive(Component, Debug, Default)]
pub struct CardUiWindow;
