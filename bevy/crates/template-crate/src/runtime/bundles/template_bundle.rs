//! Template bundle file demonstrates Bevy bundle coding standards.
//!
//! Purpose:
//! - Demonstrates Rust style-guide conventions
//! - Shows Bevy-friendly bundle structure
//! - Serves as a reusable starting point

use bevy::prelude::*;

// HUMAN: TemplateBundle groups the components needed for one spawned entity.
// AI: Keep bundle files focused on entity construction data, not runtime behavior.
#[derive(Bundle, Debug)]
pub struct TemplateBundle {
    pub name: Name,
    pub template_component: TemplateBundleComponent,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl TemplateBundle {
    pub fn new(name: impl Into<String>, velocity: Vec3) -> Self {
        let name = name.into();

        Self {
            name: Name::new(name.clone()),
            template_component: TemplateBundleComponent { name, velocity },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

// HUMAN: TemplateBundleComponent provides local data for the template bundle.
// AI: Prefer moving shared component definitions into components when this stops being a local example.
#[derive(Component, Debug)]
pub struct TemplateBundleComponent {
    pub name: String,
    pub velocity: Vec3,
}

#[cfg(test)]
#[path = "../../tests/runtime/bundles/template_bundle_tests.rs"]
mod template_bundle_tests;
