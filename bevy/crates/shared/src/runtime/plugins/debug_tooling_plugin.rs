use bevy::prelude::*;

use crate::runtime::resources::{DebugHudState, DebugLogState, DebugSafeArea};
use crate::runtime::systems::{
    debug_hud_setup_system, debug_hud_update_system, debug_input_update_system,
    debug_log_update_system, inspector_update_system,
};

/// HUMAN: Shared plugin for developer-facing debug tooling primitives.
/// AI: Visual HUD spawning is opt-in so host games do not accidentally render duplicate panels.
#[derive(Default)]
pub struct DebugToolingPlugin;

#[derive(Resource, Debug, Default)]
pub struct DebugToolingConfig {
    pub spawn_shared_debug_hud: bool,
}

impl Plugin for DebugToolingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugToolingConfig>()
            .init_resource::<DebugSafeArea>()
            .init_resource::<DebugHudState>()
            .init_resource::<DebugLogState>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                debug_hud_setup_system.run_if(debug_tooling_spawn_hud_enabled),
            )
            .add_systems(
                Update,
                (
                    debug_input_update_system,
                    inspector_update_system,
                    debug_hud_update_system
                        .after(debug_input_update_system)
                        .after(inspector_update_system),
                    debug_log_update_system,
                ),
            );
    }
}

fn debug_tooling_spawn_hud_enabled(config: Res<DebugToolingConfig>) -> bool {
    config.spawn_shared_debug_hud
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::components::DebugHudRoot;
    use crate::runtime::resources::{DebugInputModel, DebugKeyBehavior};

    #[test]
    fn plugin_startup_is_headless_by_default() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, DebugToolingPlugin));

        app.update();

        assert!(app.world().contains_resource::<DebugHudState>());
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<DebugHudRoot>>()
                .iter(app.world())
                .count(),
            0
        );
    }

    #[test]
    fn key_classification_matches_debugging_contract() {
        assert_eq!(
            DebugInputModel::behavior_for_key(KeyCode::KeyF),
            Some(DebugKeyBehavior::ToggleFps)
        );
        assert_eq!(
            DebugInputModel::behavior_for_key(KeyCode::KeyI),
            Some(DebugKeyBehavior::ToggleInspector)
        );
        assert_eq!(DebugInputModel::behavior_for_key(KeyCode::KeyP), None);
    }
}
