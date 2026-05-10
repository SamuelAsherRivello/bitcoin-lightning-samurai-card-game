use bevy::prelude::*;

use crate::runtime::resources::{
    ActiveCardType, ActiveScene, CardFlipState, CardInspectionDefaults, CardInspectionState,
    CardTypeRegistry, CardUiState, DebugHudState, GameTicks, PrimaryCameraDefaults,
    WindowPlacementState,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::runtime::resources::{create_card_settings_store, create_debug_hud_input_store};
use crate::runtime::systems::{
    advance_ticks, hot_reload_auto_restart_card_browser_scene, load_saved_card_settings,
    load_saved_debug_hud_input, load_saved_window_placement, restart_card_browser_scene,
    restore_window_placement_to_current_monitors, save_window_placement_on_close, scale_debug_hud,
    setup_app_scene, setup_game, setup_game_scene, setup_inspector, smooth_card_rotation,
    toggle_active_scene, toggle_card_type, toggle_debug_hud_inputs, toggle_inspector,
    track_card_pointer_target, track_window_placement, track_window_size,
    update_card_face_visibility, update_card_flip_animation, update_card_frame_shine,
    update_card_parallax_layers, update_debug_hud, update_end_turn_button,
};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        let camera_defaults = PrimaryCameraDefaults::default();

        app.insert_resource(ClearColor(camera_defaults.clear_color))
            .insert_resource(camera_defaults)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<GameTicks>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardTypeRegistry>()
            .init_resource::<ActiveCardType>()
            .init_resource::<CardUiState>()
            .init_resource::<DebugHudState>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<ActiveScene>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement,
                    load_saved_debug_hud_input,
                    load_saved_card_settings,
                    setup_game,
                    setup_app_scene,
                    setup_game_scene,
                    setup_inspector,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    advance_ticks,
                    restore_window_placement_to_current_monitors,
                    track_window_placement,
                    track_window_size,
                    save_window_placement_on_close.before(bevy::window::close_when_requested),
                    toggle_debug_hud_inputs,
                    toggle_active_scene,
                    track_card_pointer_target,
                    update_card_flip_animation,
                    smooth_card_rotation.after(track_card_pointer_target),
                    update_card_face_visibility.after(update_card_flip_animation),
                    update_card_parallax_layers.after(smooth_card_rotation),
                    update_card_frame_shine.after(smooth_card_rotation),
                    toggle_card_type,
                    toggle_inspector,
                    update_end_turn_button,
                    restart_card_browser_scene,
                    hot_reload_auto_restart_card_browser_scene,
                    update_debug_hud
                        .after(toggle_debug_hud_inputs)
                        .after(toggle_inspector)
                        .after(toggle_card_type),
                    scale_debug_hud,
                ),
            );

        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(store) = create_debug_hud_input_store() {
            app.insert_resource(store);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(store) = create_card_settings_store() {
            app.insert_resource(store);
        }
    }
}
