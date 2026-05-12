#![cfg_attr(windows, windows_subsystem = "windows")]

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
use bevy_card_game::runtime::plugins::{
    AI_RUNTIME_BRP_ENDPOINT, AI_RUNTIME_SCREENSHOT_METHOD, AiRuntimePlugin,
};
use bevy_card_game::{
    GamePlugin,
    runtime::resources::{DebugHudInputStore, WindowPlacementStore, valid_window_placement},
};
use bevy_card_game_shared::window::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::SimpleSubsecondPlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy_card_game::runtime::resources::{
    create_debug_hud_input_store, create_window_placement_store,
};

fn main() {
    let window_placement_store = create_startup_window_placement_store();
    let debug_hud_input_store = create_startup_debug_hud_input_store();
    let should_start_fullscreen = debug_hud_input_store
        .as_ref()
        .is_some_and(|store| store.is_fullscreen);
    let saved_window_placement = startup_window_placement(
        window_placement_store
            .as_ref()
            .and_then(|store| valid_window_placement(store.current.clone())),
        should_start_fullscreen,
    );
    let window_resolution = saved_window_placement
        .as_ref()
        .map(|placement| WindowResolution::new(placement.window_size.x, placement.window_size.y))
        .unwrap_or_else(|| WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT));
    let window_position = saved_window_placement
        .map(|placement| WindowPosition::At(placement.window_position))
        .unwrap_or(WindowPosition::Centered(MonitorSelection::Primary));
    let window_mode = should_start_fullscreen
        .then_some(WindowMode::BorderlessFullscreen(MonitorSelection::Current))
        .unwrap_or(WindowMode::Windowed);

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                file_path: asset_root_path().to_string(),
                watch_for_changes_override: asset_watch_for_changes_override(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Card Game".to_string(),
                    resolution: window_resolution,
                    position: window_position,
                    mode: window_mode,
                    ..default()
                }),
                ..default()
            }),
    );

    if let Some(store) = window_placement_store {
        app.insert_resource(store);
    } else {
        app.insert_resource(WindowPlacementStore::default());
    }
    if let Some(store) = debug_hud_input_store {
        app.insert_resource(store);
    } else {
        app.insert_resource(DebugHudInputStore::default());
    }

    #[cfg(feature = "desktop-hot-reload")]
    app.add_plugins(SimpleSubsecondPlugin::default());

    #[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
    {
        println!("AI runtime bridge enabled: Bevy Remote Protocol at {AI_RUNTIME_BRP_ENDPOINT}");
        println!("AI runtime screenshot method: {AI_RUNTIME_SCREENSHOT_METHOD}");
        app.add_plugins(AiRuntimePlugin);
    }

    app.add_plugins(GamePlugin).run();
}

fn startup_window_placement(
    saved_window_placement: Option<bevy_card_game::runtime::resources::WindowPlacement>,
    should_start_fullscreen: bool,
) -> Option<bevy_card_game::runtime::resources::WindowPlacement> {
    if should_start_fullscreen {
        None
    } else {
        saved_window_placement
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn create_startup_window_placement_store()
-> Option<bevy_persistent::prelude::Persistent<WindowPlacementStore>> {
    create_window_placement_store().ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn create_startup_debug_hud_input_store()
-> Option<bevy_persistent::prelude::Persistent<DebugHudInputStore>> {
    create_debug_hud_input_store().ok()
}

#[cfg(target_arch = "wasm32")]
fn create_startup_window_placement_store()
-> Option<bevy_persistent::prelude::Persistent<WindowPlacementStore>> {
    None
}

#[cfg(target_arch = "wasm32")]
fn create_startup_debug_hud_input_store()
-> Option<bevy_persistent::prelude::Persistent<DebugHudInputStore>> {
    None
}

#[cfg(target_arch = "wasm32")]
fn asset_root_path() -> &'static str {
    "assets"
}

#[cfg(not(target_arch = "wasm32"))]
fn asset_root_path() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
}

#[cfg(all(not(target_arch = "wasm32"), feature = "asset-hot-reload"))]
fn asset_watch_for_changes_override() -> Option<bool> {
    Some(true)
}

#[cfg(not(all(not(target_arch = "wasm32"), feature = "asset-hot-reload")))]
fn asset_watch_for_changes_override() -> Option<bool> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_card_game::runtime::resources::WindowPlacement;
    use std::path::Path;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn desktop_asset_root_is_absolute_and_points_to_game_assets() {
        let asset_root = Path::new(asset_root_path());

        assert!(asset_root.is_absolute());
        assert!(asset_root.is_dir());
        assert!(
            asset_root
                .join("themes/theme_japan/cards/card_kage_ren/background.png")
                .is_file()
        );
    }

    #[test]
    fn startup_ignores_windowed_placement_when_fullscreen_is_saved() {
        let placement = WindowPlacement {
            window_position: IVec2::new(1600, 900),
            window_size: UVec2::new(320, 180),
            monitor_name: Some("Primary".to_string()),
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(1600, 900),
        };

        assert_eq!(startup_window_placement(Some(placement), true), None);
    }

    #[test]
    fn startup_uses_windowed_placement_when_fullscreen_is_not_saved() {
        let placement = WindowPlacement {
            window_position: IVec2::new(200, 100),
            window_size: UVec2::new(1280, 800),
            monitor_name: Some("Primary".to_string()),
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(200, 100),
        };

        assert_eq!(
            startup_window_placement(Some(placement.clone()), false),
            Some(placement)
        );
    }
}
