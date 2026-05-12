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
#[path = "../../tests/runtime/resources/template_resource_tests.rs"]
mod template_resource_tests;
