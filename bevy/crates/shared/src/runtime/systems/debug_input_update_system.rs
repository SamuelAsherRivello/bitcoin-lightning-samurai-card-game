use bevy::prelude::*;

use crate::runtime::resources::DebugHudState;

/// HUMAN: Applies approved debug input to diagnostic state.
/// AI: F toggles FPS, WASD records hold feedback, and unknown keys are ignored.
pub fn debug_input_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut hud_state: ResMut<DebugHudState>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        hud_state.is_fps_visible = !hud_state.is_fps_visible;
    }

    for key_code in [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD] {
        hud_state
            .input
            .set_pressed(key_code, keys.pressed(key_code));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_input_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, debug_input_update_system);
        app
    }

    #[test]
    fn f_toggles_fps_without_toggling_inspector() {
        let mut app = debug_input_app();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        let state = app.world().resource::<DebugHudState>();
        assert!(state.is_fps_visible);
        assert!(!state.is_inspector_visible);
    }

    #[test]
    fn wasd_updates_hold_indicators_without_toggling_fps_or_inspector() {
        let mut app = debug_input_app();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();

        let state = app.world().resource::<DebugHudState>();
        assert!(state.input.is_pressed(KeyCode::KeyW));
        assert!(!state.is_fps_visible);
        assert!(!state.is_inspector_visible);
    }
}
