use bevy::prelude::*;

/// HUMAN: Marker for the root of a shared DebugHUD panel.
/// AI: Use this only for developer-facing diagnostic UI, not Card UI gameplay surfaces.
#[derive(Component, Debug, Default)]
pub struct DebugHudRoot;

#[derive(Component, Debug, Default)]
pub struct DebugHudParent;

#[derive(Component, Debug, Default)]
pub struct DebugHudText;

#[derive(Component, Debug, Default)]
pub struct DebugHudFpsText;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct DebugHudKeyLabel {
    pub key_code: KeyCode,
    pub is_toggle: bool,
}

impl DebugHudKeyLabel {
    pub const fn new(key_code: KeyCode, is_toggle: bool) -> Self {
        Self {
            key_code,
            is_toggle,
        }
    }
}
