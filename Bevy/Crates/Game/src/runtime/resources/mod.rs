use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const DEFAULT_WINDOW_WIDTH: u32 = 800;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 600;

pub const PRIMARY_CAMERA_FOV_RADIANS: f32 = std::f32::consts::FRAC_PI_4;
pub const PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.5;
pub const PRIMARY_CAMERA_NEAR: f32 = 0.1;
pub const PRIMARY_CAMERA_FAR: f32 = 1000.0;
pub const CARD_WIDTH_WORLD_UNITS: f32 = 63.0 / 88.0;
pub const CARD_HEIGHT_WORLD_UNITS: f32 = 1.0;
pub const CARD_THICKNESS_WORLD_UNITS: f32 = 0.02;
pub const CARD_MAX_TILT_DEGREES: f32 = 20.0;
pub const CARD_SMOOTHING_RESPONSE_SECONDS: f32 = 0.1;

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

#[derive(Clone, Debug, Resource)]
pub struct PrimaryCameraDefaults {
    pub position: Vec3,
    pub target: Vec3,
    pub fov_radians: f32,
    pub near: f32,
    pub far: f32,
    pub clear_color: Color,
}

impl Default for PrimaryCameraDefaults {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN),
            target: Vec3::ZERO,
            fov_radians: PRIMARY_CAMERA_FOV_RADIANS,
            near: PRIMARY_CAMERA_NEAR,
            far: PRIMARY_CAMERA_FAR,
            clear_color: Color::srgb(0.08, 0.08, 0.08),
        }
    }
}

impl PrimaryCameraDefaults {
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position).looking_at(self.target, Vec3::Y)
    }
}

#[derive(Clone, Debug, Resource)]
pub struct CardInspectionDefaults {
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub max_tilt_radians: f32,
    pub smoothing_response_seconds: f32,
    pub material_color: Color,
}

impl Default for CardInspectionDefaults {
    fn default() -> Self {
        Self {
            width: CARD_WIDTH_WORLD_UNITS,
            height: CARD_HEIGHT_WORLD_UNITS,
            thickness: CARD_THICKNESS_WORLD_UNITS,
            max_tilt_radians: CARD_MAX_TILT_DEGREES.to_radians(),
            smoothing_response_seconds: CARD_SMOOTHING_RESPONSE_SECONDS,
            material_color: Color::WHITE,
        }
    }
}

impl CardInspectionDefaults {
    pub fn height_width_ratio(&self) -> f32 {
        self.height / self.width
    }
}

#[derive(Debug, Resource)]
pub struct CardInspectionState {
    pub last_pointer_normalized: Vec2,
    pub target_rotation: Quat,
}

impl Default for CardInspectionState {
    fn default() -> Self {
        Self {
            last_pointer_normalized: Vec2::ZERO,
            target_rotation: Quat::IDENTITY,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}

#[derive(Resource, Debug, Default)]
pub struct WindowPlacementState {
    pub current: Option<WindowPlacement>,
    pub restored: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WindowPlacement {
    pub window_position: IVec2,
    pub window_size: UVec2,
    pub monitor_name: Option<String>,
    pub monitor_position: IVec2,
    pub monitor_size: UVec2,
    pub relative_position: IVec2,
}

pub fn window_placement_path() -> PathBuf {
    Path::new("generated")
        .join("runtime")
        .join("window-placement.json")
}

pub fn load_window_placement() -> Option<WindowPlacement> {
    let content = fs::read_to_string(window_placement_path()).ok()?;
    let placement: WindowPlacement = serde_json::from_str(&content).ok()?;
    if !is_valid_window_placement(&placement) {
        return None;
    }

    Some(placement)
}

pub fn save_window_placement(placement: &WindowPlacement) -> std::io::Result<()> {
    let path = window_placement_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(placement)?;
    fs::write(path, content)
}

fn is_valid_window_placement(placement: &WindowPlacement) -> bool {
    placement.window_size.x > 0 && placement.window_size.y > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_placement_serializes_position_size_and_screen_identity() {
        let placement = WindowPlacement {
            window_position: IVec2::new(100, 200),
            window_size: UVec2::new(800, 600),
            monitor_name: Some("Display 1".to_string()),
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(100, 200),
        };

        let serialized = serde_json::to_string(&placement).unwrap();
        let deserialized: WindowPlacement = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, placement);
    }

    #[test]
    fn window_placement_requires_positive_size() {
        let placement = WindowPlacement {
            window_position: IVec2::new(100, 200),
            window_size: UVec2::ZERO,
            monitor_name: None,
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(100, 200),
        };

        assert!(!is_valid_window_placement(&placement));
    }

    #[test]
    fn card_defaults_match_poker_card_ratio() {
        let defaults = CardInspectionDefaults::default();
        let expected_ratio = 88.0 / 63.0;
        let tolerance = expected_ratio * 0.02;

        assert!((defaults.height_width_ratio() - expected_ratio).abs() <= tolerance);
        assert_eq!(
            defaults.max_tilt_radians,
            CARD_MAX_TILT_DEGREES.to_radians()
        );
        assert_eq!(
            defaults.smoothing_response_seconds,
            CARD_SMOOTHING_RESPONSE_SECONDS
        );
    }

    #[test]
    fn card_defaults_fit_inside_unit_bounds() {
        let defaults = CardInspectionDefaults::default();

        assert!(defaults.width <= 1.0);
        assert!(defaults.height <= 1.0);
        assert!(defaults.thickness <= 1.0);
        assert_eq!(defaults.height, 1.0);
    }
}
