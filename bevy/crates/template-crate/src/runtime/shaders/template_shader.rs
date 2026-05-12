//! Template shader file demonstrates Bevy shader support coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly shader support structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

pub const TEMPLATE_SHADER_PATH: &str = "shaders/template_shader.wgsl";

// HUMAN: TemplateShaderSettings stores crate-owned shader presentation settings.
// AI: Keep shader source support separate from gameplay systems and scene wiring.
#[derive(Resource, Debug)]
pub struct TemplateShaderSettings {
    pub shader_path: &'static str,
    pub tint: Color,
}

impl Default for TemplateShaderSettings {
    fn default() -> Self {
        Self {
            shader_path: TEMPLATE_SHADER_PATH,
            tint: Color::WHITE,
        }
    }
}

#[cfg(test)]
mod template_shader_settings_tests {
    use super::*;

    #[test]
    fn template_shader_settings_when_default_is_called_sets_expected_values() {
        let result = TemplateShaderSettings::default();

        assert_eq!(result.shader_path, TEMPLATE_SHADER_PATH);
        assert_eq!(result.tint, Color::WHITE);
    }
}
