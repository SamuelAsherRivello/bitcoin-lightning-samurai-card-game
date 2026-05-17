use bevy::{
    camera::{ClearColorConfig, visibility::RenderLayers},
    prelude::*,
    render::view::NoIndirectDrawing,
};

use crate::runtime::components::{AppSceneCamera, AppSceneEntity};
use crate::runtime::resources::AppCameraModel;

/// HUMAN: Single shared 3D camera bundle for the persistent AppScene.
/// AI: This is staged inactive until view-owned cameras are removed by the shared-camera migration.
#[derive(Bundle)]
pub struct AppCameraBundle {
    pub name: Name,
    pub app_camera: AppSceneCamera,
    pub app_scene_entity: AppSceneEntity,
    pub camera_3d: Camera3d,
    pub camera: Camera,
    pub render_layers: RenderLayers,
    pub no_indirect_drawing: NoIndirectDrawing,
    pub projection: Projection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AppCameraBundle {
    pub fn new(model: &AppCameraModel) -> Self {
        Self {
            name: Name::new("AppScene 3D Camera"),
            app_camera: AppSceneCamera,
            app_scene_entity: AppSceneEntity,
            camera_3d: Camera3d::default(),
            camera: Camera {
                is_active: model.is_active,
                clear_color: ClearColorConfig::Custom(model.clear_color),
                viewport: model.safe_viewport.clone(),
                ..Default::default()
            },
            render_layers: RenderLayers::from_layers(&[0, 1, 2]),
            no_indirect_drawing: NoIndirectDrawing,
            projection: Projection::Perspective(PerspectiveProjection {
                fov: model.fov_radians,
                near: model.near,
                far: model.far,
                ..Default::default()
            }),
            transform: model.transform(),
            global_transform: GlobalTransform::from(model.transform()),
        }
    }
}
