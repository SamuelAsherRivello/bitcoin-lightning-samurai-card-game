use bevy::prelude::*;

use crate::runtime::resources::DebugInputModel;

/// HUMAN: Runtime model for shared DebugHUD state and frame-rate samples.
/// AI: Keep this developer-facing state separate from gameplay and Card UI state.
#[derive(Resource, Debug)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub is_inspector_visible: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
    pub input: DebugInputModel,
}

impl Default for DebugHudState {
    fn default() -> Self {
        Self {
            is_fps_visible: false,
            is_inspector_visible: false,
            fps_accumulated_seconds: 0.0,
            fps_accumulated_frames: 0,
            fps_display_value: 0.0,
            input: DebugInputModel::default(),
        }
    }
}

impl DebugHudState {
    pub fn record_frame_sample(&mut self, delta_seconds: f32, update_interval_seconds: f32) {
        self.fps_accumulated_seconds += delta_seconds.max(0.0);
        self.fps_accumulated_frames += 1;

        if self.fps_accumulated_seconds >= update_interval_seconds {
            self.fps_display_value = if self.fps_accumulated_seconds > 0.0 {
                self.fps_accumulated_frames as f32 / self.fps_accumulated_seconds
            } else {
                0.0
            };
            self.fps_accumulated_seconds = 0.0;
            self.fps_accumulated_frames = 0;
        }
    }

    pub fn fps_text(&self) -> String {
        if self.is_fps_visible {
            format!("\nFPS: {:.1}", self.fps_display_value)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fps_samples_update_display_value_after_interval() {
        let mut state = DebugHudState::default();

        state.record_frame_sample(0.25, 0.5);
        state.record_frame_sample(0.25, 0.5);

        assert_eq!(state.fps_display_value, 4.0);
        assert_eq!(state.fps_accumulated_frames, 0);
    }
}
