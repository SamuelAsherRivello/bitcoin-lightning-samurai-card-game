//! Template component file demonstrates Bevy component coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly component structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

// HUMAN: TemplateComponent stores per-entity gameplay data.
// AI: Keep component files focused on data attached to entities.
#[derive(Component, Debug)]
pub struct TemplateComponent {
    pub name: String,
    pub velocity: Vec3,
}

impl TemplateComponent {
    pub fn new(name: impl Into<String>, velocity: Vec3) -> Self {
        Self {
            name: name.into(),
            velocity,
        }
    }
}

#[cfg(test)]
mod template_component_tests {
    use super::*;

    #[test]
    fn template_component_when_new_is_called_sets_expected_values() {
        let result = TemplateComponent::new("Player", Vec3::ZERO);

        assert_eq!(result.name, "Player");
        assert_eq!(result.velocity, Vec3::ZERO);
    }
}
