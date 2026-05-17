use bevy::prelude::*;

use crate::runtime::components::ScreenTransitionOverlay;

pub const SCREEN_TRANSITION_OVERLAY_Z_INDEX: i32 = 2_000;

/// HUMAN: Fullscreen top-layer overlay used for screen fade transitions.
/// AI: Keep this simple and reusable; transition_update_system drives alpha and timing.
#[derive(Bundle, Debug)]
pub struct ScreenTransitionUiBundle {
    pub name: Name,
    pub overlay: ScreenTransitionOverlay,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub global_z_index: GlobalZIndex,
}

impl Default for ScreenTransitionUiBundle {
    fn default() -> Self {
        Self {
            name: Name::new("ScreenTransitionOverlay"),
            overlay: ScreenTransitionOverlay,
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
            global_z_index: GlobalZIndex(SCREEN_TRANSITION_OVERLAY_Z_INDEX),
        }
    }
}
