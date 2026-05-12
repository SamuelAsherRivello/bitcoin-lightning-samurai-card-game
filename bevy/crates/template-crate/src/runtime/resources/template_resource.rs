//! Template resource file demonstrates Bevy resource coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly resource structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

// HUMAN: TemplateResource stores globally shared gameplay configuration.
// AI: Keep resource files focused on shared state and deterministic helpers.
#[derive(Resource, Debug)]
pub struct TemplateResource {
    pub move_speed: f32,
    pub rotation_speed: f32,
}

impl Default for TemplateResource {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            rotation_speed: 180.0,
        }
    }
}

impl TemplateResource {
    pub fn movement_delta(&self, velocity: Vec3, delta_seconds: f32) -> Vec3 {
        velocity * self.move_speed * delta_seconds
    }
}

#[cfg(test)]
mod template_resource_tests {
    use super::*;

    #[test]
    fn template_resource_when_default_is_called_sets_expected_values() {
        let result = TemplateResource::default();

        assert_eq!(result.move_speed, 5.0);
        assert_eq!(result.rotation_speed, 180.0);
    }

    #[test]
    fn movement_delta_when_called_scales_velocity_by_speed_and_time() {
        let resource = TemplateResource::default();

        let result = resource.movement_delta(Vec3::X, 2.0);

        assert_eq!(result, Vec3::new(10.0, 0.0, 0.0));
    }
}
