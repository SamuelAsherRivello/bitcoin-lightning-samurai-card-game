use bevy::prelude::*;

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
mod ai_runtime_plugin;

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub use ai_runtime_plugin::{
    AI_RUNTIME_BRP_ENDPOINT, AI_RUNTIME_SCREENSHOT_METHOD, AiRuntimePlugin,
};

use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveView, ActiveWorldModel, CardFlipState,
    CardGestureModel, CardInspectionDefaults, CardInspectionState, CardModelRegistry,
    CardSlotBoardModel, CardStateModel, CardUiState, DebugDrawingModel, DebugHudState,
    FullscreenViewportTransitionState, GameDeckModel, GameHandModel, GameLocationModel,
    GameRoundModel, GameTicks, LocationModelRegistry, PlayerDeckCollectionModel,
    PrimaryCameraDefaults, WindowPlacementState, WorldModelRegistry,
    create_player_deck_collection_store,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::runtime::resources::{
    DebugHudInputStore, create_card_settings_store, create_debug_hud_input_store,
};
use crate::runtime::systems::{
    advance_ticks, card_gesture_animation_system, card_gesture_update_system,
    card_model_input_system, constrain_debug_settings_camera_to_safe_area,
    constrain_deck_builder_camera_to_safe_area, constrain_game_view_3d_cameras_to_safe_area,
    debug_drawing_update_system, drop_target_hint_update_system, hot_reload_auto_restart_app_scene,
    initialize_game_models, load_saved_card_settings, load_saved_debug_hud_input,
    load_saved_player_deck_collection, load_saved_window_placement,
    log_game_view_card_render_diagnostics, quit_app_on_escape,
    record_desktop_hot_reload_patch_message, restart_app_scene,
    restore_window_placement_to_current_monitors, save_window_placement_on_close, scale_debug_hud,
    scene_input_system, setup_app_scene, setup_game, setup_game_view_with_params, setup_inspector,
    smooth_card_rotation, sync_browser_fullscreen_state_system,
    sync_game_view_hand_card_entities_system, toggle_debug_hud_inputs, toggle_inspector,
    track_card_pointer_target, track_window_placement, track_window_size,
    update_card_face_visibility, update_card_flip_animation, update_card_frame_shine,
    update_card_parallax_layers, update_card_point_text2d_overlay_system,
    update_card_power_point_views_system, update_debug_hud, update_end_turn_button,
    update_game_control_ui_system, update_game_location_views_system, update_location_power_points,
    view_input_system,
};

/// HUMAN: Bevy plugin that wires game resources and runtime systems.
/// AI: Keep this focused on plugin composition; move behavior into resources, components, or systems.
pub struct CoreGamePlugin;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum StartupBootstrapSet {
    LoadWindowPlacement,
    LoadDebugHudInput,
    LoadCardSettings,
    LoadPlayerDeckCollection,
    InitializeGameModels,
    SetupGame,
    SetupAppScene,
}

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        let camera_defaults = PrimaryCameraDefaults::default();

        app.configure_sets(
            Startup,
            StartupBootstrapSet::LoadWindowPlacement
                .before(StartupBootstrapSet::LoadDebugHudInput)
                .before(StartupBootstrapSet::LoadCardSettings)
                .before(StartupBootstrapSet::LoadPlayerDeckCollection)
                .before(StartupBootstrapSet::InitializeGameModels)
                .before(StartupBootstrapSet::SetupGame)
                .before(StartupBootstrapSet::SetupAppScene),
        );

        app.insert_resource(ClearColor(camera_defaults.clear_color))
            .insert_resource(camera_defaults)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<GameTicks>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<CardGestureModel>()
            .init_resource::<CardSlotBoardModel>()
            .init_resource::<CardStateModel>()
            .init_resource::<GameDeckModel>()
            .init_resource::<GameHandModel>()
            .init_resource::<GameRoundModel>()
            .init_resource::<GameLocationModel>()
            .init_resource::<PlayerDeckCollectionModel>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .init_resource::<CardUiState>()
            .init_resource::<DebugHudState>()
            .init_resource::<DebugDrawingModel>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<FullscreenViewportTransitionState>()
            .init_resource::<ActiveView>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement.in_set(StartupBootstrapSet::LoadWindowPlacement),
                    load_saved_debug_hud_input.in_set(StartupBootstrapSet::LoadDebugHudInput),
                    load_saved_card_settings.in_set(StartupBootstrapSet::LoadCardSettings),
                    load_saved_player_deck_collection
                        .in_set(StartupBootstrapSet::LoadPlayerDeckCollection),
                    initialize_game_models.in_set(StartupBootstrapSet::InitializeGameModels),
                    setup_game.in_set(StartupBootstrapSet::SetupGame),
                    setup_app_scene.in_set(StartupBootstrapSet::SetupAppScene),
                ),
            )
            .add_systems(Startup, setup_inspector)
            .add_systems(Startup, setup_game_view_with_params)
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
                update_game_control_ui_system.after(update_end_turn_button),
            )
            .add_systems(
                Update,
                update_game_location_views_system.after(update_end_turn_button),
            )
            .add_systems(
                Update,
                sync_game_view_hand_card_entities_system
                    .after(update_end_turn_button)
                    .before(card_gesture_animation_system),
            )
            .add_systems(Update, scene_input_system.before(view_input_system))
            .add_systems(
                Update,
                (
                    card_gesture_animation_system.after(card_gesture_update_system),
                    drop_target_hint_update_system.after(card_gesture_update_system),
                ),
            )
            .add_systems(
                Update,
                card_gesture_update_system.before(update_end_turn_button),
            )
            .add_systems(
                Update,
                update_location_power_points.after(card_gesture_update_system),
            )
            .add_systems(
                Update,
                update_card_power_point_views_system.after(card_gesture_update_system),
            )
            .add_systems(
                Update,
                update_card_point_text2d_overlay_system.after(card_gesture_animation_system),
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
            .add_systems(Update, constrain_deck_builder_camera_to_safe_area)
            .add_systems(Update, constrain_debug_settings_camera_to_safe_area)
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
        if !app
            .world()
            .contains_resource::<bevy_persistent::prelude::Persistent<PlayerDeckCollectionModel>>()
            && let Ok(store) = create_player_deck_collection_store()
        {
            app.insert_resource(store);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(store) = create_card_settings_store() {
            app.insert_resource(store);
        }
    }
}
