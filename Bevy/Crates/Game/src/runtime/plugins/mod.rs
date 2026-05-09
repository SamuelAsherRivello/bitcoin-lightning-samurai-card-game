use bevy::prelude::*;

use crate::runtime::resources::{DebugHudState, GameTicks, WindowPlacementState};
use crate::runtime::systems::{
    advance_ticks, load_saved_window_placement, restore_window_placement_to_current_monitors,
    save_window_placement_on_close, scale_debug_hud, setup_debug_hud, setup_game, setup_inspector,
    toggle_inspector, track_window_placement, track_window_size, update_debug_hud,
};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTicks>()
            .init_resource::<DebugHudState>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement,
                    setup_game,
                    setup_inspector,
                    setup_debug_hud,
                ),
            )
            .add_systems(
                Update,
                (
                    advance_ticks,
                    restore_window_placement_to_current_monitors,
                    track_window_placement,
                    track_window_size,
                    save_window_placement_on_close,
                    toggle_inspector,
                    update_debug_hud.after(toggle_inspector),
                    scale_debug_hud,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::CoreGamePlugin;
    use crate::runtime::components::{DebugHudKeyText, DebugHudText, InspectorState, Player};
    use crate::runtime::resources::{DebugHudState, GameTicks};

    #[test]
    fn plugin_spawns_player_debug_hud_inspector_and_advances_ticks() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut player_query = app.world_mut().query::<&Player>();
        let player_count = player_query.iter(app.world()).count();
        let mut hud_query = app.world_mut().query::<&DebugHudText>();
        let hud_count = hud_query.iter(app.world()).count();
        let mut inspector_query = app.world_mut().query::<&InspectorState>();
        let inspector_count = inspector_query.iter(app.world()).count();
        let ticks = app.world().resource::<GameTicks>().0;

        assert_eq!(player_count, 1);
        assert_eq!(hud_count, 1);
        assert_eq!(inspector_count, 1);
        assert_eq!(ticks, 1);
    }

    #[test]
    fn hud_contains_wasd_f_and_i_key_labels() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let keys: Vec<KeyCode> = key_query
            .iter(app.world())
            .map(|key_text| key_text.key_code)
            .collect();

        assert!(keys.contains(&KeyCode::KeyW));
        assert!(keys.contains(&KeyCode::KeyA));
        assert!(keys.contains(&KeyCode::KeyS));
        assert!(keys.contains(&KeyCode::KeyD));
        assert!(keys.contains(&KeyCode::KeyF));
        assert!(keys.contains(&KeyCode::KeyI));
    }

    #[test]
    fn f_toggles_fps_visibility() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        assert!(app.world().resource::<DebugHudState>().is_fps_visible);
    }

    #[test]
    fn i_toggles_inspector_visibility() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyI);
        app.update();

        let mut inspector_query = app.world_mut().query::<&InspectorState>();
        let inspector = inspector_query.single(app.world()).unwrap();
        assert!(inspector.is_visible);
    }

    #[test]
    fn wasd_keys_do_not_toggle_debug_features() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
            keys.press(KeyCode::KeyA);
            keys.press(KeyCode::KeyS);
            keys.press(KeyCode::KeyD);
        }
        app.update();

        let mut inspector_query = app.world_mut().query::<&InspectorState>();
        let inspector = inspector_query.single(app.world()).unwrap();
        assert!(!app.world().resource::<DebugHudState>().is_fps_visible);
        assert!(!inspector.is_visible);
    }
}
