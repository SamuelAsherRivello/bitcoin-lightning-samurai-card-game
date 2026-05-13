use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;
use bevy::prelude::*;

use crate::runtime::components::AppSceneRoot;
use crate::runtime::resources::{
    ActiveCardModel, CardInspectionDefaults, CardModelRegistry, PlayerDeckCollectionModel,
    PrimaryCameraDefaults,
};

/// HUMAN: Spawns the deck sub-screen view.
/// AI: DeckScene displays available decks and the selected deck card list.
pub fn setup_deck_scene(
    commands: Commands,
    asset_server: Res<AssetServer>,
    camera_defaults: Res<PrimaryCameraDefaults>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    active_card_model: Res<ActiveCardModel>,
    player_deck_collection: Option<Res<PlayerDeckCollectionModel>>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    crate::runtime::systems::setup_deck_scene(
        commands,
        asset_server,
        camera_defaults,
        card_defaults,
        card_model_registry,
        active_card_model,
        player_deck_collection,
        app_scene_query,
        meshes,
        materials,
        masked_background_materials,
    );
}
