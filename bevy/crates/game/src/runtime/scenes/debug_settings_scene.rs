use bevy::prelude::*;

use crate::runtime::components::AppSceneRoot;
use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;
use crate::runtime::resources::{
    ActiveCardModel, CardInspectionDefaults, CardModelRegistry, PrimaryCameraDefaults,
};

/// HUMAN: Spawns the debug settings sub-screen scene.
/// AI: DebugSettingsScene duplicates DeckBuilderScene presentation for debug configuration work.
pub fn setup_debug_settings_scene(
    commands: Commands,
    asset_server: Res<AssetServer>,
    camera_defaults: Res<PrimaryCameraDefaults>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    _active_card_model: Res<ActiveCardModel>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    crate::runtime::systems::setup_debug_settings_scene(
        commands,
        asset_server,
        camera_defaults,
        card_defaults,
        card_model_registry,
        _active_card_model,
        app_scene_query,
        meshes,
        materials,
        masked_background_materials,
    );
}
