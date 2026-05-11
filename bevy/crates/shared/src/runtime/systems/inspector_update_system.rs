use bevy::prelude::*;

use crate::runtime::components::InspectorState;
use crate::runtime::resources::DebugHudState;

/// HUMAN: Toggles shared inspector visibility from approved debug input.
/// AI: Inspector state is developer tooling and must not affect FPS or Card UI state.
pub fn inspector_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut hud_state: ResMut<DebugHudState>,
    mut inspector_query: Query<&mut InspectorState>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    hud_state.is_inspector_visible = !hud_state.is_inspector_visible;
    for mut inspector in &mut inspector_query {
        inspector.is_visible = hud_state.is_inspector_visible;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_toggles_inspector_without_toggling_fps() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, inspector_update_system);
        app.world_mut().spawn(InspectorState::default());

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyI);
        app.update();

        let state = app.world().resource::<DebugHudState>();
        assert!(state.is_inspector_visible);
        assert!(!state.is_fps_visible);
    }
}
