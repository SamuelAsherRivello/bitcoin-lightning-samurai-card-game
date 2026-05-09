use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const DEFAULT_WINDOW_WIDTH: u32 = 800;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 600;

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

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
}
