use bevy::{
    camera::Viewport,
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

use crate::runtime::bundles::AppCameraBundle;
use crate::runtime::components::{AppSceneCamera, AppSceneRoot};
use crate::runtime::resources::AppCameraModel;

use super::CardPointTextCamera;

const GAME_SCENE_WIDTH: f32 = 1280.0;
const GAME_SCENE_HEIGHT: f32 = 800.0;

/// HUMAN: Spawns the staged shared AppScene camera once AppScene exists.
/// AI: Keep creation separate from screen setup so child views stop owning cameras later.
pub fn app_camera_startup_system(
    mut commands: Commands,
    app_camera_model: Res<AppCameraModel>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
) {
    if !app_camera_query.is_empty() {
        return;
    }

    let Ok(app_scene) = app_scene_query.single() else {
        return;
    };

    let app_camera = commands.spawn(AppCameraBundle::new(&app_camera_model)).id();
    commands.entity(app_scene).add_child(app_camera);
}

/// HUMAN: Keeps the shared AppScene camera viewport aligned to the safe game area.
/// AI: This mirrors current per-view viewport math before the migration removes those cameras.
pub fn app_camera_update_system(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    app_camera_model: Option<ResMut<AppCameraModel>>,
    mut camera_query: Query<&mut Camera, Or<(With<AppSceneCamera>, With<CardPointTextCamera>)>>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    let safe_viewport = app_camera_safe_area_viewport_for_window(window);
    if let Some(mut app_camera_model) = app_camera_model {
        app_camera_model.safe_viewport = safe_viewport.clone();
    }

    for mut camera in &mut camera_query {
        camera.viewport = safe_viewport.clone();
    }
}

pub fn app_camera_safe_area_viewport_for_window(window: &Window) -> Option<Viewport> {
    if should_use_default_camera_viewport(window) {
        return None;
    }

    app_camera_safe_area_viewport(window.resolution.physical_size())
}

#[cfg(not(target_arch = "wasm32"))]
fn should_use_default_camera_viewport(window: &Window) -> bool {
    !matches!(window.mode, WindowMode::Windowed)
}

#[cfg(target_arch = "wasm32")]
fn should_use_default_camera_viewport(_window: &Window) -> bool {
    false
}

fn app_camera_safe_area_viewport(window_size: UVec2) -> Option<Viewport> {
    if window_size.x == 0 || window_size.y == 0 {
        return None;
    }

    let game_scene_size = Vec2::new(GAME_SCENE_WIDTH, GAME_SCENE_HEIGHT);
    let window_size_f32 = window_size.as_vec2();
    let scale = (window_size_f32.x / game_scene_size.x).min(window_size_f32.y / game_scene_size.y);
    if scale <= 0.0 {
        return None;
    }

    let viewport_size = (game_scene_size * scale).round().as_uvec2();
    let viewport_position = ((window_size - viewport_size).as_vec2() * 0.5)
        .round()
        .as_uvec2();

    Some(Viewport {
        physical_position: viewport_position,
        physical_size: viewport_size,
        depth: 0.0..1.0,
    })
}
