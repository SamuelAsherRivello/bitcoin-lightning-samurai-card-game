use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugRect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

/// HUMAN: Aspect-ratio-safe layout constants for debug surfaces.
/// AI: Keep debug UI inside the same 1280x800 safe area as gameplay HUD elements.
#[derive(Clone, Copy, Debug, Resource)]
pub struct DebugSafeArea {
    pub width: f32,
    pub height: f32,
    pub padding_left: f32,
    pub padding_top: f32,
}

impl Default for DebugSafeArea {
    fn default() -> Self {
        Self {
            width: 1280.0,
            height: 800.0,
            padding_left: 16.0,
            padding_top: 16.0,
        }
    }
}

impl DebugSafeArea {
    pub fn debug_hud_rect(&self) -> DebugRect {
        DebugRect {
            left: self.padding_left,
            top: self.padding_top,
            width: 273.0,
            height: 84.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_hud_rect_stays_inside_safe_area() {
        let safe_area = DebugSafeArea::default();
        let rect = safe_area.debug_hud_rect();

        assert!(rect.left >= 0.0);
        assert!(rect.top >= 0.0);
        assert!(rect.left + rect.width <= safe_area.width);
        assert!(rect.top + rect.height <= safe_area.height);
    }
}
