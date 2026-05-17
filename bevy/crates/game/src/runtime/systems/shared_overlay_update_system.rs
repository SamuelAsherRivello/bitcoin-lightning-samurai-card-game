use bevy::prelude::*;

use crate::runtime::components::{AppSceneCamera, SharedCameraOverlayView};

/// HUMAN: Shared-camera overlay maintenance point for fades, HUDs, and modal surfaces.
/// AI: Keep this lightweight until individual overlay paths move off per-view 2D cameras.
pub fn shared_overlay_update_system(
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
    mut overlay_query: Query<&mut Visibility, With<SharedCameraOverlayView>>,
) {
    let has_app_camera = !app_camera_query.is_empty();
    for mut visibility in &mut overlay_query {
        if !has_app_camera {
            *visibility = Visibility::Hidden;
        }
    }
}
