use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DebugKeyBehavior {
    HoldIndicator,
    ToggleFps,
    ToggleInspector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DebugKeyState {
    pub key_code: KeyCode,
    pub is_pressed: bool,
}

/// HUMAN: Approved input model for developer diagnostics.
/// AI: Unknown keys are ignored so debug tooling cannot accidentally drive gameplay.
#[derive(Clone, Debug)]
pub struct DebugInputModel {
    pub key_states: [DebugKeyState; 6],
}

impl Default for DebugInputModel {
    fn default() -> Self {
        Self {
            key_states: [
                DebugKeyState::new(KeyCode::KeyW),
                DebugKeyState::new(KeyCode::KeyA),
                DebugKeyState::new(KeyCode::KeyS),
                DebugKeyState::new(KeyCode::KeyD),
                DebugKeyState::new(KeyCode::KeyF),
                DebugKeyState::new(KeyCode::KeyI),
            ],
        }
    }
}

impl DebugKeyState {
    pub const fn new(key_code: KeyCode) -> Self {
        Self {
            key_code,
            is_pressed: false,
        }
    }
}

impl DebugInputModel {
    pub fn behavior_for_key(key_code: KeyCode) -> Option<DebugKeyBehavior> {
        match key_code {
            KeyCode::KeyW | KeyCode::KeyA | KeyCode::KeyS | KeyCode::KeyD => {
                Some(DebugKeyBehavior::HoldIndicator)
            }
            KeyCode::KeyF => Some(DebugKeyBehavior::ToggleFps),
            KeyCode::KeyI => Some(DebugKeyBehavior::ToggleInspector),
            _ => None,
        }
    }

    pub fn set_pressed(&mut self, key_code: KeyCode, is_pressed: bool) {
        if let Some(key_state) = self
            .key_states
            .iter_mut()
            .find(|key_state| key_state.key_code == key_code)
        {
            key_state.is_pressed = is_pressed;
        }
    }

    pub fn is_pressed(&self, key_code: KeyCode) -> bool {
        self.key_states
            .iter()
            .find(|key_state| key_state.key_code == key_code)
            .is_some_and(|key_state| key_state.is_pressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_only_approved_debug_keys() {
        assert_eq!(
            DebugInputModel::behavior_for_key(KeyCode::KeyF),
            Some(DebugKeyBehavior::ToggleFps)
        );
        assert_eq!(
            DebugInputModel::behavior_for_key(KeyCode::KeyI),
            Some(DebugKeyBehavior::ToggleInspector)
        );
        assert_eq!(
            DebugInputModel::behavior_for_key(KeyCode::KeyW),
            Some(DebugKeyBehavior::HoldIndicator)
        );
        assert_eq!(DebugInputModel::behavior_for_key(KeyCode::Escape), None);
    }
}
