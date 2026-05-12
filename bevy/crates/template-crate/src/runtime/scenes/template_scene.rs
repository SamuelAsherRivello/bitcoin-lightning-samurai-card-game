//! Template scene file demonstrates Bevy scene and view coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly scene structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

// HUMAN: TemplateScene marks the root entity for a template runtime scene.
// AI: Use Scene for persistent app-level scenes and View for active presentation surfaces.
#[derive(Component, Debug)]
pub struct TemplateScene;

// HUMAN: TemplateView marks a visible child presentation within TemplateScene.
// AI: Keep scene files focused on presentation structure and scene lifecycle.
#[derive(Component, Debug)]
pub struct TemplateView;

pub fn template_scene_startup_system(mut commands: Commands) {
    commands
        .spawn((Name::new("TemplateScene"), TemplateScene))
        .with_children(|parent| {
            parent.spawn((
                Name::new("TemplateView"),
                TemplateView,
                Transform::default(),
                GlobalTransform::default(),
            ));
        });
}

#[cfg(test)]
mod template_scene_tests {
    use super::*;

    #[test]
    fn template_scene_markers_can_be_constructed() {
        let _scene = TemplateScene;
        let _view = TemplateView;
    }
}
