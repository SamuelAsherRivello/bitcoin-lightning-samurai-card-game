//! Template plugin file demonstrates Bevy plugin coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly plugin structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

// HUMAN: TemplatePlugin registers resources and systems with Bevy.
// AI: Keep plugin files focused on feature composition and app wiring.
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TemplatePluginResource>()
            .add_systems(Startup, template_plugin_startup_system)
            .add_systems(Update, template_plugin_update_system);
    }
}

// HUMAN: TemplatePluginResource stores shared configuration for this plugin example.
// AI: Move reusable resource definitions to resources when this becomes real feature code.
#[derive(Resource, Debug)]
pub struct TemplatePluginResource {
    pub move_speed: f32,
}

impl Default for TemplatePluginResource {
    fn default() -> Self {
        Self { move_speed: 5.0 }
    }
}

// HUMAN: TemplatePluginComponent stores local entity state for this plugin example.
// AI: Move reusable component definitions to components when this becomes real feature code.
#[derive(Component, Debug)]
pub struct TemplatePluginComponent {
    pub velocity: Vec3,
}

fn template_plugin_startup_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Template Plugin Entity"),
        TemplatePluginComponent {
            velocity: Vec3::ZERO,
        },
        Transform::default(),
        GlobalTransform::default(),
    ));
}

fn template_plugin_update_system(
    time: Res<Time>,
    resource: Res<TemplatePluginResource>,
    mut query: Query<(&mut Transform, &TemplatePluginComponent)>,
) {
    for (mut transform, template_component) in &mut query {
        transform.translation +=
            template_component.velocity * resource.move_speed * time.delta_secs();
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/plugins/template_plugin_resource_tests.rs"]
mod template_plugin_resource_tests;
