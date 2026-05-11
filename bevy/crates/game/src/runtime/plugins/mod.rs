use bevy::prelude::*;

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
mod ai_runtime_plugin;

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub use ai_runtime_plugin::{
    AI_RUNTIME_BRP_ENDPOINT, AI_RUNTIME_SCREENSHOT_METHOD, AiRuntimePlugin,
};

use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveView, ActiveWorldModel, CardFlipState,
    CardInspectionDefaults, CardInspectionState, CardModelRegistry, CardUiState, DebugDrawingModel,
    DebugHudState, GameTicks, LocationModelRegistry, PrimaryCameraDefaults, WindowPlacementState,
    WorldModelRegistry,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::runtime::resources::{
    DebugHudInputStore, create_card_settings_store, create_debug_hud_input_store,
};
use crate::runtime::systems::{
    advance_ticks, card_model_input_system, constrain_card_browser_camera_to_safe_area,
    constrain_game_view_3d_cameras_to_safe_area, debug_drawing_update_system,
    hot_reload_auto_restart_app_scene, load_saved_card_settings, load_saved_debug_hud_input,
    load_saved_window_placement, log_game_view_card_render_diagnostics, quit_app_on_escape,
    record_desktop_hot_reload_patch_message, restart_app_scene,
    restore_window_placement_to_current_monitors, save_window_placement_on_close, scale_debug_hud,
    setup_app_scene, setup_game, setup_game_view, setup_inspector, smooth_card_rotation,
    sync_browser_fullscreen_state_system, toggle_debug_hud_inputs, toggle_inspector,
    track_card_pointer_target, track_window_placement, track_window_size,
    update_card_face_visibility, update_card_flip_animation, update_card_frame_shine,
    update_card_parallax_layers, update_debug_hud, update_end_turn_button, view_input_system,
};

/// HUMAN: Bevy plugin that wires game resources and runtime systems.
/// AI: Keep this focused on plugin composition; move behavior into resources, components, or systems.
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
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .init_resource::<CardUiState>()
            .init_resource::<DebugHudState>()
            .init_resource::<DebugDrawingModel>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<ActiveView>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement,
                    load_saved_debug_hud_input,
                    load_saved_card_settings,
                    setup_game,
                    setup_app_scene,
                    setup_game_view,
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
                    view_input_system,
                    track_card_pointer_target,
                    update_card_flip_animation,
                    smooth_card_rotation.after(track_card_pointer_target),
                    update_card_face_visibility.after(update_card_flip_animation),
                    update_card_parallax_layers.after(smooth_card_rotation),
                    update_card_frame_shine.after(smooth_card_rotation),
                    log_game_view_card_render_diagnostics.after(smooth_card_rotation),
                    debug_drawing_update_system,
                    card_model_input_system,
                    toggle_inspector,
                    update_end_turn_button,
                    update_debug_hud
                        .after(toggle_debug_hud_inputs)
                        .after(toggle_inspector)
                        .after(sync_browser_fullscreen_state_system)
                        .after(card_model_input_system),
                    scale_debug_hud,
                ),
            )
            .add_systems(
                Update,
                sync_browser_fullscreen_state_system.after(toggle_debug_hud_inputs),
            )
            .add_systems(
                Update,
                (
                    restart_app_scene,
                    record_desktop_hot_reload_patch_message,
                    hot_reload_auto_restart_app_scene,
                )
                    .chain(),
            )
            .add_systems(Update, constrain_card_browser_camera_to_safe_area)
            .add_systems(Update, constrain_game_view_3d_cameras_to_safe_area)
            .add_systems(
                Update,
                quit_app_on_escape.before(save_window_placement_on_close),
            );

        #[cfg(not(target_arch = "wasm32"))]
        if !app
            .world()
            .contains_resource::<bevy_persistent::prelude::Persistent<DebugHudInputStore>>()
            && let Ok(store) = create_debug_hud_input_store()
        {
            app.insert_resource(store);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(store) = create_card_settings_store() {
            app.insert_resource(store);
        }
    }
}
