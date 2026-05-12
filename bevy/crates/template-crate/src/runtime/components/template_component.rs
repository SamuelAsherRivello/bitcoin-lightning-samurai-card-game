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
#[path = "../../tests/runtime/components/template_component_tests.rs"]
mod template_component_tests;
