use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}
