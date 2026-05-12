//! Template system file demonstrates Bevy system coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly system structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

// HUMAN: TemplateSystemResource stores shared configuration for the template system.
// AI: Move shared resource definitions to resources when this becomes real feature code.
#[derive(Resource, Debug)]
pub struct TemplateSystemResource {
    pub move_speed: f32,
}

impl Default for TemplateSystemResource {
    fn default() -> Self {
        Self { move_speed: 5.0 }
    }
}

// HUMAN: TemplateSystemComponent stores per-entity data used by the template system.
// AI: Move shared component definitions to components when this becomes real feature code.
#[derive(Component, Debug)]
pub struct TemplateSystemComponent {
    pub velocity: Vec3,
}

pub fn template_startup_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Template System Entity"),
        TemplateSystemComponent {
            velocity: Vec3::ZERO,
        },
        Transform::default(),
        GlobalTransform::default(),
    ));
}

pub fn template_update_system(
    time: Res<Time>,
    resource: Res<TemplateSystemResource>,
    mut query: Query<(&mut Transform, &TemplateSystemComponent)>,
) {
    for (mut transform, template_component) in &mut query {
        transform.translation +=
            template_component.velocity * resource.move_speed * time.delta_secs();
    }
}

pub fn calculate_forward_vector(rotation_radians: f32) -> Vec3 {
    Vec3::new(rotation_radians.cos(), 0.0, rotation_radians.sin())
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/calculate_forward_vector_tests.rs"]
mod calculate_forward_vector_tests;
