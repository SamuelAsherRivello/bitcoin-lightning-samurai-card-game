use bevy::prelude::*;

use crate::runtime::components::{DebugHudFpsText, DebugHudKeyLabel, DebugHudText};
use crate::runtime::resources::{DebugHudState, DebugInputModel, DebugKeyBehavior};

/// HUMAN: Updates shared DebugHUD text, FPS output, and key feedback.
/// AI: This is diagnostic UI state only; do not drive gameplay behavior here.
pub fn debug_hud_update_system(
    time: Res<Time>,
    mut hud_state: ResMut<DebugHudState>,
    mut text_query: Query<&mut Text, With<DebugHudText>>,
    mut fps_query: Query<&mut TextSpan, With<DebugHudFpsText>>,
    mut key_query: Query<(&DebugHudKeyLabel, &mut TextColor)>,
) {
    hud_state.record_frame_sample(time.delta_secs(), 0.5);

    for mut text in &mut text_query {
        *text = Text::new("Scene: GameView\nFrame: --\nKEYS: ");
    }

    let fps_text = hud_state.fps_text();
    for mut text_span in &mut fps_query {
        *text_span = TextSpan::new(fps_text.clone());
    }

    for (label, mut color) in &mut key_query {
        let active = match DebugInputModel::behavior_for_key(label.key_code) {
            Some(DebugKeyBehavior::ToggleFps) => hud_state.is_fps_visible,
            Some(DebugKeyBehavior::ToggleInspector) => hud_state.is_inspector_visible,
            Some(DebugKeyBehavior::HoldIndicator) => hud_state.input.is_pressed(label.key_code),
            None => false,
        };
        color.0 = if active {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.58)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fps_text_is_rendered_only_when_visible() {
        let mut state = DebugHudState {
            is_fps_visible: true,
            fps_display_value: 60.0,
            ..Default::default()
        };

        assert_eq!(state.fps_text(), "\nFPS: 60.0");
        state.is_fps_visible = false;
        assert_eq!(state.fps_text(), "");
    }
}
