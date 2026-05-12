use bevy::prelude::*;
use bevy_aspect_ratio_mask::Hud;

use crate::runtime::components::AppSceneRoot;
use crate::runtime::systems::SetupGameViewParams;
use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;
use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveWorldModel, CardInspectionDefaults,
    CardModelRegistry, CardSlotBoardModel, LocationModelRegistry, PrimaryCameraDefaults,
    WorldModelRegistry,
};

/// HUMAN: Spawns the gameplay sub-screen view.
/// AI: GameView is a view, not the persistent scene; keep AppScene parenting intact.
pub fn setup_game_view(
    commands: Commands,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    hud: Option<Res<Hud>>,
    asset_server: Res<AssetServer>,
    camera_defaults: Option<Res<PrimaryCameraDefaults>>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    slot_board: Option<Res<CardSlotBoardModel>>,
    active_card_model: Res<ActiveCardModel>,
    world_model_registry: Res<WorldModelRegistry>,
    active_world_model: Res<ActiveWorldModel>,
    location_model_registry: Res<LocationModelRegistry>,
    active_locations: Res<ActiveLocations>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    crate::runtime::systems::setup_game_view(SetupGameViewParams {
        commands,
        app_scene_query,
        hud,
        asset_server,
        camera_defaults,
        card_defaults,
        card_model_registry,
        slot_board,
        active_card_model,
        world_model_registry,
        active_world_model,
        location_model_registry,
        active_locations,
        player_deck_collection: None,
        game_deck_model: None,
        game_hand_model: None,
        card_states: None,
        meshes,
        materials,
        masked_background_materials,
    });
}

