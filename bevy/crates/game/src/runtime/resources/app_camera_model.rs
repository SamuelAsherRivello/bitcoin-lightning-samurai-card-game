use bevy::{camera::Viewport, prelude::*};

use crate::runtime::resources::{
    PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN, PRIMARY_CAMERA_FAR, PRIMARY_CAMERA_FOV_RADIANS,
    PRIMARY_CAMERA_NEAR,
};

/// HUMAN: Central locked camera configuration for the persistent AppScene camera.
/// AI: Keep shared camera defaults here so view setup stops owning camera transform data.
#[derive(Clone, Debug, Resource)]
pub struct AppCameraModel {
    pub position: Vec3,
    pub target: Vec3,
    pub scale: Vec3,
    pub fov_radians: f32,
    pub near: f32,
    pub far: f32,
    pub clear_color: Color,
    pub safe_viewport: Option<Viewport>,
    pub is_active: bool,
}

impl Default for AppCameraModel {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN),
            target: Vec3::ZERO,
            scale: Vec3::ONE,
            fov_radians: PRIMARY_CAMERA_FOV_RADIANS,
            near: PRIMARY_CAMERA_NEAR,
            far: PRIMARY_CAMERA_FAR,
            clear_color: Color::srgb(0.08, 0.08, 0.08),
            safe_viewport: None,
            is_active: true,
        }
    }
}

impl AppCameraModel {
    pub fn active() -> Self {
        Self {
            is_active: true,
            ..Default::default()
        }
    }

    pub fn transform(&self) -> Transform {
        let mut transform =
            Transform::from_translation(self.position).looking_at(self.target, Vec3::Y);
        transform.scale = self.scale;
        transform
    }
}
