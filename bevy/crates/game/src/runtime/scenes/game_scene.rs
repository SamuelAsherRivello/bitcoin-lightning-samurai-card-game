use bevy::prelude::*;
use bevy_aspect_ratio_mask::Hud;

use crate::runtime::components::{AppSceneCamera, AppSceneRoot};
use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveView, ActiveWorldModel, CardInspectionDefaults,
    CardModelRegistry, CardSlotBoardModel, GameLocationModel, GameRoundModel,
    LocationModelRegistry, MatchModel, PrimaryCameraDefaults, WorldModelRegistry,
};
use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;
use crate::runtime::systems::SetupGameSceneParams;

/// HUMAN: Spawns the gameplay sub-screen view.
/// AI: GameScene is a view, not the persistent scene; keep AppScene parenting intact.
pub fn setup_game_scene(
    commands: Commands,
    active_view: Option<Res<ActiveView>>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
    hud: Option<Res<Hud>>,
    asset_server: Res<AssetServer>,
    camera_defaults: Option<Res<PrimaryCameraDefaults>>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    slot_board: Option<Res<CardSlotBoardModel>>,
    active_card_model: Res<ActiveCardModel>,
    world_model_registry: Res<WorldModelRegistry>,
    active_world_model: ResMut<ActiveWorldModel>,
    location_model_registry: Res<LocationModelRegistry>,
    active_locations: ResMut<ActiveLocations>,
    game_round_model: Option<ResMut<GameRoundModel>>,
    game_location_model: Option<ResMut<GameLocationModel>>,
    match_model: Option<ResMut<MatchModel>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    crate::runtime::systems::setup_game_scene(SetupGameSceneParams {
        commands,
        active_view,
        app_scene_query,
        app_camera_query,
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
        game_round_model,
        game_location_model,
        match_model,
        card_states: None,
        pending_round_deal: None,
        meshes,
        materials,
        masked_background_materials,
    });
}
