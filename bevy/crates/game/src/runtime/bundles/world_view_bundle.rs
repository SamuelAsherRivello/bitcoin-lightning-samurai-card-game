use bevy::prelude::*;

use crate::runtime::components::{
    GameSceneEntity, WorldBackground, WorldFadeOverlay, WorldThemeTransition,
};

/// HUMAN: 3D flat world backdrop for the gameplay view.
/// AI: Keep world presentation as mesh data while systems decide theme and projection size.
#[derive(Bundle, Debug)]
pub struct WorldViewBundle {
    name: Name,
    background: WorldBackground,
    scene_entity: GameSceneEntity,
    transition: WorldThemeTransition,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}

impl WorldViewBundle {
    pub fn new(
        display_name: &'static str,
        world_index: usize,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        transform: Transform,
    ) -> Self {
        Self {
            name: Name::new(format!("{display_name} World Background")),
            background: WorldBackground,
            scene_entity: GameSceneEntity,
            transition: WorldThemeTransition::startup(world_index),
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform,
            visibility: Visibility::Visible,
        }
    }
}

/// HUMAN: Black overlay plane for world-background theme transitions.
/// AI: Share the world mesh and animate only the material alpha from systems.
#[derive(Bundle, Debug)]
pub struct WorldFadeOverlayBundle {
    name: Name,
    overlay: WorldFadeOverlay,
    scene_entity: GameSceneEntity,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}

impl WorldFadeOverlayBundle {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            name: Name::new("World Fade Overlay"),
            overlay: WorldFadeOverlay,
            scene_entity: GameSceneEntity,
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.001)),
            visibility: Visibility::Visible,
        }
    }
}
