use bevy::{
    app::AppExit,
    asset::RenderAssetUsages,
    camera::{
        ClearColorConfig, ScalingMode, Viewport,
        visibility::{NoCpuCulling, NoFrustumCulling, RenderLayers},
    },
    ecs::system::SystemParam,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::view::NoIndirectDrawing,
    sprite::Anchor,
    text::{Justify, Underline, UnderlineColor},
    window::{
        Monitor, PrimaryWindow, WindowCloseRequested, WindowMode, WindowMoved, WindowResized,
        WindowResolution,
    },
};
use bevy_aspect_ratio_mask::Hud;
use bevy_card_game_shared::{
    GameTitle,
    window::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH},
};
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::{HotPatched, hot};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
    bevy_inspector::EntityFilter,
};
use bevy_persistent::prelude::Persistent;

pub mod card_gesture_animation_system;
pub mod card_gesture_update_system;
pub mod debug_drawing_update_system;

pub use card_gesture_animation_system::*;
pub use card_gesture_update_system::*;
pub use debug_drawing_update_system::*;

use crate::runtime::bundles::{
    CardViewBundle, LocationViewBundle, POINT_VIEW_BASE_TEXT_FONT_SIZE, POINT_VIEW_BUNDLE_FONT,
    PointLocationView, PointModel, PointType, PointView, PointViewBundle,
};
use crate::runtime::components::{
    AppSceneEntity, AppSceneRoot, CardBackgroundLayer, CardFaceLayer, CardFrameLayer,
    CardGestureView, CardLayerRole, CardParallaxLayer, CardSlotGestureTarget, CardView,
    DebugHudFpsText, DebugHudKeyText, DebugHudText, DebugSettingsSceneEntity,
    DebugSettingsSceneRoot, DeckBuilderSceneEntity, DeckBuilderSceneRoot, DropTargetHint,
    EndTurnButton, GameControlAction, GameControlButton, GameControlLabel, GameLocation,
    GameLocationBodyText, GameLocationBorder, GameLocationTitleText, GameViewEntity, GameViewRoot,
    HandCardGestureTarget, InspectorState, LocalPlayerHand, LocalPlayerHandCardPreview,
    LocationRevealState, Player, PrimaryViewCamera, TurnUi, WorldBackground,
};
#[cfg(test)]
use crate::runtime::resources::CardState;
use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveView, ActiveWorldModel, CARD_BACK_TEXTURE_PATH,
    CARD_DEPTH_FACTOR_DEFAULT, CARD_DEPTH_FACTOR_MAX, CARD_DEPTH_FACTOR_MIN, CARD_LAYER_SCALE_MAX,
    CARD_LAYER_SCALE_MIN, CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT, CARD_SAFE_AREA_TEXTURE_PATH,
    CARD_SLOT_LOCATION_COUNT, CardFace, CardFlipState, CardGestureModel, CardGestureState,
    CardInspectionDefaults, CardInspectionState, CardModel, CardModelRegistry, CardSettingsStore,
    CardSlotBoardModel, CardSlotSide, CardSlotState, CardStateModel, CardUiState, CostPointModel,
    DEFAULT_DECK_NAME, DebugHudInputStore, DebugHudState, DeckModel,
    FullscreenViewportTransitionState, GAME_BUTTON_FONT, GameDeckModel, GameHandModel,
    GameLocationModel, GameRoundModel, GameTicks, LocationModelRegistry, LocationScoreModel,
    PRIMARY_CAMERA_FOV_RADIANS, PlayerDeckCollectionModel, PowerPointModel, PrimaryCameraDefaults,
    STARTING_HAND_CARD_COUNT, WindowPlacement, WindowPlacementState, WindowPlacementStore,
    WorldModelRegistry, ensure_player_deck_collection_model, load_window_placement,
    random_shuffled_default_deck_cards, valid_window_placement,
};
use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;

#[cfg(feature = "desktop-hot-reload")]
use crate::runtime::resources::{
    DebugDrawingModel, desktop_hot_reload_patch_count, record_desktop_hot_reload_patch,
};

#[cfg(test)]
use bevy::mesh::VertexAttributeValues;

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;
const SCREEN_PADDING_TOP: f32 = 24.0;
const SCREEN_PADDING_LEFT: f32 = 24.0;
const TARGET_WIDTH: f32 = DEFAULT_WINDOW_WIDTH as f32;
const TARGET_HEIGHT: f32 = DEFAULT_WINDOW_HEIGHT as f32;
const GAME_VIEW_WIDTH: f32 = 1280.0;
const GAME_VIEW_HEIGHT: f32 = 800.0;
const DEBUG_HUD_FONT_SIZE: f32 = 22.0;
const DEBUG_WINDOW_FONT_SIZE: f32 = 14.0;
const DEBUG_WINDOW_WIDTH: f32 = 338.0;
const CARD_UI_RESET_BUTTON_WIDTH: f32 = 24.0;
const BACKGROUND_APPARENT_DEPTH: f32 = -1.0;
const FRAME_APPARENT_DEPTH: f32 = 0.0;
const SAFE_AREA_APPARENT_DEPTH: f32 = 0.0;
const FOREGROUND_APPARENT_DEPTH: f32 = 1.0;
const TITLE_APPARENT_DEPTH: f32 = 2.0;
const LAYER_RENDER_Z_STEP: f32 = 0.0001;
const BACKGROUND_DEPTH_BIAS: f32 = 0.0;
const FRAME_DEPTH_BIAS: f32 = 8.0;
const SAFE_AREA_DEPTH_BIAS: f32 = 12.0;
const FOREGROUND_DEPTH_BIAS: f32 = 16.0;
const TITLE_DEPTH_BIAS: f32 = 24.0;
const POINT_DEPTH_BIAS: f32 = 32.0;
const PARALLAX_OFFSET_RATIO: f32 = 0.065;
const FRAME_THICKNESS_RATIO: f32 = 0.05;
const BACKGROUND_APERTURE_SCALE: f32 = 1.0;
const FRAME_SHINE_STRENGTH: f32 = 0.22;
const GAME_VIEW_ASPECT_RATIO: f32 = GAME_VIEW_WIDTH / GAME_VIEW_HEIGHT;
const GAME_SCENE_HAND_LEFT: f32 = 364.0;
const GAME_SCENE_HAND_TOP: f32 = 612.0;
const GAME_SCENE_HAND_WIDTH: f32 = 552.0;
const GAME_SCENE_HAND_HEIGHT: f32 = GAME_VIEW_HEIGHT - GAME_SCENE_HAND_TOP;
const GAME_SCENE_HAND_CARD_HEIGHT_FRACTION: f32 = 0.9;
const GAME_SCENE_HAND_CARD_HEIGHT: f32 =
    GAME_SCENE_HAND_HEIGHT * GAME_SCENE_HAND_CARD_HEIGHT_FRACTION;
const GAME_SCENE_HAND_CARD_WIDTH: f32 =
    GAME_SCENE_HAND_CARD_HEIGHT * CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT;
const GAME_SCENE_HAND_CARD_GAP: f32 = 8.0;
// Card roots use depth bands wider than their internal child-layer spread.
// Overlapping hand cards must not share a band; non-overlapping slots may.
const GAME_SCENE_HAND_CARD_WORLD_Z: f32 = 0.3;
const GAME_SCENE_HAND_CARD_Z_STEP: f32 = 0.035;
const GAME_SCENE_HAND_CARD_HOVER_Z: f32 = 0.74;
const GAME_SCENE_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.33;
const GAME_SCENE_WORLD_BACKGROUND_BLEED: f32 = 1.18;
const GAME_SCENE_WORLD_BACKGROUND_Z: f32 = -0.16;
const CARD_RENDER_LAYER: usize = 1;
const CARD_POINT_TEXT_RENDER_LAYER: usize = 2;
const GAME_SCENE_CARD_TILT_RADIANS: f32 = 0.07;
const DECK_BUILDER_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.33;
const DECK_BUILDER_CARD_HEIGHT_FRACTION: f32 = 0.9;
const DEBUG_HUD_Z_INDEX: i32 = 100;
const END_TURN_BUTTON_NORMAL_COLOR: Color = Color::srgba(0.22, 0.04, 0.44, 0.82);
const END_TURN_BUTTON_HOVER_COLOR: Color = Color::srgba(0.36, 0.08, 0.68, 0.9);
const END_TURN_BUTTON_PRESSED_COLOR: Color = Color::srgba(0.12, 0.02, 0.28, 0.95);
const END_TURN_BUTTON_NORMAL_BORDER_COLOR: Color = Color::srgb(0.45, 0.18, 0.9);
const END_TURN_BUTTON_HOVER_BORDER_COLOR: Color = Color::srgb(0.7, 0.42, 1.0);
const END_TURN_BUTTON_PRESSED_BORDER_COLOR: Color = Color::srgb(0.95, 0.82, 1.0);
const GAME_CONTROL_DISABLED_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.55);
const GAME_CONTROL_DISABLED_BORDER_COLOR: Color = Color::srgb(0.28, 0.28, 0.28);
const GAME_CONTROL_BUTTON_WIDTH: f32 = 220.0;
const GAME_CONTROL_BUTTON_HEIGHT: f32 = 88.0;
const DEBUG_SETTINGS_CARD_GAP_TO_CARD_UI: f32 = 20.0;
const POINT_VIEW_WIDTH: f32 = 46.0;
const POINT_VIEW_HEIGHT: f32 = 36.0;
const LOCATION_POINT_VIEW_WIDTH: f32 = POINT_VIEW_WIDTH.min(POINT_VIEW_HEIGHT);
const LOCATION_POINT_VIEW_HEIGHT: f32 = LOCATION_POINT_VIEW_WIDTH;
const LOCATION_POINT_VIEW_HALF_HEIGHT: f32 = LOCATION_POINT_VIEW_HEIGHT / 2.0;
const CARD_POINT_BADGE_SIZE: f32 = 0.17;
const CARD_POINT_BADGE_INSET_RATIO: f32 = 0.16;
const CARD_POINT_TEXT_FONT_SIZE: f32 = POINT_VIEW_BASE_TEXT_FONT_SIZE;
const CARD_POINT_TEXT_Z: f32 = 10.0;
#[cfg(not(target_arch = "wasm32"))]
const FULLSCREEN_VIEWPORT_TRANSITION_FRAMES: u8 = 6;

/// HUMAN: Spawns the local player entity for the app.
/// AI: Startup system; keep player setup separate from AppScene and view setup.
pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn setup_primary_camera(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    spawn_primary_camera(&mut commands, &camera_defaults);
}

pub fn constrain_deck_builder_camera_to_safe_area(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<
        &mut Camera,
        (
            With<PrimaryViewCamera>,
            With<DeckBuilderSceneEntity>,
            With<Camera3d>,
        ),
    >,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    let safe_area_viewport = game_view_safe_area_viewport_for_window(window);

    for mut camera in &mut camera_query {
        camera.viewport = safe_area_viewport.clone();
    }
}

pub fn constrain_debug_settings_camera_to_safe_area(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<
        &mut Camera,
        (
            With<PrimaryViewCamera>,
            With<DebugSettingsSceneEntity>,
            With<Camera3d>,
        ),
    >,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    let safe_area_viewport = game_view_safe_area_viewport_for_window(window);

    for mut camera in &mut camera_query {
        camera.viewport = safe_area_viewport.clone();
    }
}

/// HUMAN: Keeps GameView card cameras aligned with the aspect-ratio-safe area.
/// AI: Align 3D cards and their Text2d point overlay camera to the same viewport.
pub fn constrain_game_view_3d_cameras_to_safe_area(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut fullscreen_viewport_transition: Option<ResMut<FullscreenViewportTransitionState>>,
    mut camera_query: Query<
        &mut Camera,
        (
            With<GameViewEntity>,
            Or<(With<Camera3d>, With<CardPointTextCamera>)>,
        ),
    >,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    let safe_area_viewport = game_view_safe_area_viewport_for_window_transition(
        window,
        fullscreen_viewport_transition.as_deref(),
    );

    for mut camera in &mut camera_query {
        camera.viewport = safe_area_viewport.clone();
    }

    if let Some(ref mut transition) = fullscreen_viewport_transition
        && transition.frames_remaining > 0
    {
        transition.frames_remaining -= 1;
    }
}

fn game_view_safe_area_viewport_for_window(window: &Window) -> Option<Viewport> {
    if should_use_default_camera_viewport(window) {
        return None;
    }

    game_view_safe_area_viewport(window.resolution.physical_size())
}

fn game_view_safe_area_viewport_for_window_transition(
    window: &Window,
    fullscreen_viewport_transition: Option<&FullscreenViewportTransitionState>,
) -> Option<Viewport> {
    if fullscreen_viewport_transition.is_some_and(|transition| transition.frames_remaining > 0) {
        return None;
    }

    game_view_safe_area_viewport_for_window(window)
}

#[cfg(not(target_arch = "wasm32"))]
fn should_use_default_camera_viewport(window: &Window) -> bool {
    !matches!(window.mode, WindowMode::Windowed)
}

#[cfg(target_arch = "wasm32")]
fn should_use_default_camera_viewport(_window: &Window) -> bool {
    false
}

fn game_view_safe_area_viewport(window_size: UVec2) -> Option<Viewport> {
    if window_size.x == 0 || window_size.y == 0 {
        return None;
    }

    let game_view_size = Vec2::new(GAME_VIEW_WIDTH, GAME_VIEW_HEIGHT);
    let window_size_f32 = window_size.as_vec2();
    let scale = (window_size_f32.x / game_view_size.x).min(window_size_f32.y / game_view_size.y);
    if scale <= 0.0 {
        return None;
    }

    let viewport_size = (game_view_size * scale).round().as_uvec2();
    let viewport_position = ((window_size - viewport_size).as_vec2() * 0.5)
        .round()
        .as_uvec2();

    Some(Viewport {
        physical_position: viewport_position,
        physical_size: viewport_size,
        depth: 0.0..1.0,
    })
}

fn spawn_primary_camera(
    commands: &mut Commands,
    camera_defaults: &PrimaryCameraDefaults,
) -> Entity {
    let mut camera_transform = camera_defaults.clone();
    camera_transform.position.z = DECK_BUILDER_CAMERA_DISTANCE_FROM_ORIGIN;

    commands
        .spawn((
            Name::new("Primary 3D Camera"),
            PrimaryViewCamera,
            DeckBuilderSceneEntity,
            Camera3d::default(),
            NoIndirectDrawing,
            Projection::Perspective(PerspectiveProjection {
                fov: camera_defaults.fov_radians,
                near: camera_defaults.near,
                far: camera_defaults.far,
                ..Default::default()
            }),
            RenderLayers::layer(CARD_RENDER_LAYER),
            camera_transform.transform(),
        ))
        .id()
}

fn spawn_deck_builder_ui_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("DeckBuilderScene UI Camera"),
            DeckBuilderSceneEntity,
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            IsDefaultUiCamera,
            PrimaryEguiContext,
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: GAME_VIEW_WIDTH,
                    min_height: GAME_VIEW_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
        ))
        .id()
}

fn spawn_debug_settings_primary_camera(
    commands: &mut Commands,
    camera_defaults: &PrimaryCameraDefaults,
) -> Entity {
    let mut camera_transform = camera_defaults.clone();
    camera_transform.position.z = DECK_BUILDER_CAMERA_DISTANCE_FROM_ORIGIN;

    commands
        .spawn((
            Name::new("DebugSettingsScene 3D Camera"),
            PrimaryViewCamera,
            DebugSettingsSceneEntity,
            Camera3d::default(),
            NoIndirectDrawing,
            Projection::Perspective(PerspectiveProjection {
                fov: camera_defaults.fov_radians,
                near: camera_defaults.near,
                far: camera_defaults.far,
                ..Default::default()
            }),
            RenderLayers::layer(CARD_RENDER_LAYER),
            camera_transform.transform(),
        ))
        .id()
}

fn spawn_debug_settings_ui_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("DebugSettingsScene UI Camera"),
            DebugSettingsSceneEntity,
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            IsDefaultUiCamera,
            PrimaryEguiContext,
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: GAME_VIEW_WIDTH,
                    min_height: GAME_VIEW_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
        ))
        .id()
}

fn spawn_game_view_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("GameView UI Camera"),
            GameViewEntity,
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            IsDefaultUiCamera,
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: GAME_VIEW_WIDTH,
                    min_height: GAME_VIEW_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
        ))
        .id()
}

fn spawn_game_view_card_camera(
    commands: &mut Commands,
    camera_defaults: &PrimaryCameraDefaults,
) -> Entity {
    let mut camera_transform = camera_defaults.clone();
    camera_transform.position.z = GAME_SCENE_CAMERA_DISTANCE_FROM_ORIGIN;

    commands
        .spawn((
            Name::new("GameView 3D Card Camera"),
            PrimaryViewCamera,
            GameViewEntity,
            Camera3d::default(),
            Camera {
                order: 0,
                ..Default::default()
            },
            NoIndirectDrawing,
            Projection::Perspective(PerspectiveProjection {
                fov: camera_defaults.fov_radians,
                near: camera_defaults.near,
                far: camera_defaults.far,
                ..Default::default()
            }),
            camera_transform.transform(),
        ))
        .id()
}

fn spawn_game_view_card_overlay_camera(
    commands: &mut Commands,
    camera_defaults: &PrimaryCameraDefaults,
) -> Entity {
    let mut camera_transform = camera_defaults.clone();
    camera_transform.position.z = GAME_SCENE_CAMERA_DISTANCE_FROM_ORIGIN;

    commands
        .spawn((
            Name::new("GameView 3D Card Overlay Camera"),
            PrimaryViewCamera,
            GameViewEntity,
            Camera3d::default(),
            Camera {
                order: 2,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            NoIndirectDrawing,
            Projection::Perspective(PerspectiveProjection {
                fov: camera_defaults.fov_radians,
                near: camera_defaults.near,
                far: camera_defaults.far,
                ..Default::default()
            }),
            RenderLayers::layer(CARD_RENDER_LAYER),
            camera_transform.transform(),
        ))
        .id()
}

fn spawn_game_view_card_point_text_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("GameView Card Point Text Camera"),
            CardPointTextCamera,
            GameViewEntity,
            Camera2d,
            Camera {
                order: 3,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: GAME_VIEW_WIDTH,
                    min_height: GAME_VIEW_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
            RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER),
        ))
        .id()
}

/// HUMAN: Marks the GameView 2D camera that draws card point Text2d above point circles.
/// AI: The marker lets safe-area viewport code match the 3D card cameras exactly.
#[derive(Clone, Copy, Component, Debug, Default, Eq, PartialEq)]
pub struct CardPointTextCamera;

#[cfg_attr(feature = "desktop-hot-reload", hot)]
/// HUMAN: Spawns the persistent AppScene and debug HUD.
/// AI: AppScene remains present while GameView, DeckBuilderScene, and DebugSettingsScene swap on top.
pub fn setup_app_scene(
    mut commands: Commands,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    hud: Option<Res<Hud>>,
) {
    if !app_scene_query.is_empty() {
        return;
    }

    spawn_app_scene_contents(&mut commands, hud.as_ref().map(|hud| hud.0));
}

fn spawn_app_scene_contents(commands: &mut Commands, hud_parent: Option<Entity>) -> Entity {
    let app_scene = commands
        .spawn((
            Name::new("AppScene"),
            AppSceneRoot,
            AppSceneEntity,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id();
    let debug_hud = spawn_debug_hud(commands);
    if let Some(hud_parent) = hud_parent {
        commands.entity(hud_parent).add_child(debug_hud);
    } else {
        commands.entity(app_scene).add_child(debug_hud);
    }
    app_scene
}

/// HUMAN: Spawns the gameplay sub-screen view.
/// AI: GameView is a view, not the persistent scene; keep AppScene parenting intact.
#[derive(SystemParam)]
pub struct SetupGameViewParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub app_scene_query: Query<'w, 's, Entity, With<AppSceneRoot>>,
    pub hud: Option<Res<'w, Hud>>,
    pub asset_server: Res<'w, AssetServer>,
    pub camera_defaults: Option<Res<'w, PrimaryCameraDefaults>>,
    pub card_defaults: Res<'w, CardInspectionDefaults>,
    pub card_model_registry: Res<'w, CardModelRegistry>,
    pub slot_board: Option<Res<'w, CardSlotBoardModel>>,
    pub active_card_model: Res<'w, ActiveCardModel>,
    pub world_model_registry: Res<'w, WorldModelRegistry>,
    pub active_world_model: Res<'w, ActiveWorldModel>,
    pub location_model_registry: Res<'w, LocationModelRegistry>,
    pub active_locations: Res<'w, ActiveLocations>,
    pub player_deck_collection: Option<Res<'w, PlayerDeckCollectionModel>>,
    pub game_deck_model: Option<ResMut<'w, GameDeckModel>>,
    pub game_hand_model: Option<ResMut<'w, GameHandModel>>,
    pub game_round_model: Option<ResMut<'w, GameRoundModel>>,
    pub game_location_model: Option<ResMut<'w, GameLocationModel>>,
    pub card_states: Option<ResMut<'w, CardStateModel>>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub masked_background_materials: Option<ResMut<'w, Assets<CardBackgroundMaskMaterial>>>,
}

pub fn setup_game_view(mut params: SetupGameViewParams) {
    let fallback_camera_defaults = PrimaryCameraDefaults::default();
    let camera_defaults = params
        .camera_defaults
        .as_deref()
        .unwrap_or(&fallback_camera_defaults);
    let fallback_slot_board = CardSlotBoardModel::default();
    let slot_board = params.slot_board.as_deref().unwrap_or(&fallback_slot_board);
    let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
    let player_deck_collection = params
        .player_deck_collection
        .as_deref()
        .unwrap_or(&fallback_player_deck_collection);
    let mut fallback_game_deck_model = GameDeckModel::default();
    let mut fallback_game_hand_model = GameHandModel::default();
    let fallback_game_round_model = GameRoundModel::default();
    let fallback_game_location_model = GameLocationModel::default();
    let mut fallback_card_states = CardStateModel::default();
    let game_hand_cards = match (
        params.game_deck_model.as_mut(),
        params.game_hand_model.as_mut(),
        params.game_round_model.as_mut(),
        params.game_location_model.as_mut(),
        params.card_states.as_deref_mut(),
    ) {
        (
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(game_location_model),
            Some(card_states),
        ) => {
            initialize_game_models_for_player(
                player_deck_collection,
                &mut *game_deck_model,
                &mut *game_hand_model,
                &mut *game_round_model,
                &mut *game_location_model,
                card_states,
            );
            game_hand_model.cards.clone()
        }
        (Some(game_deck_model), Some(game_hand_model), None, None, None) => {
            initialize_legacy_game_models_for_player(
                player_deck_collection,
                &mut *game_deck_model,
                &mut *game_hand_model,
                &mut fallback_card_states,
            );
            game_hand_model.cards.clone()
        }
        _ => {
            initialize_legacy_game_models_for_player(
                player_deck_collection,
                &mut fallback_game_deck_model,
                &mut fallback_game_hand_model,
                &mut fallback_card_states,
            );
            fallback_game_hand_model.cards.clone()
        }
    };
    let app_scene_parent = params.app_scene_query.iter().next().or_else(|| {
        Some(spawn_app_scene_contents(
            &mut params.commands,
            params.hud.as_ref().map(|hud| hud.0),
        ))
    });
    spawn_game_view_contents(
        &mut params.commands,
        app_scene_parent,
        params.hud.as_ref().map(|hud| hud.0),
        &params.asset_server,
        camera_defaults,
        &params.card_defaults,
        &params.card_model_registry,
        game_hand_cards.as_slice(),
        params
            .game_round_model
            .as_deref()
            .unwrap_or(&fallback_game_round_model),
        params
            .game_location_model
            .as_deref()
            .unwrap_or(&fallback_game_location_model),
        slot_board,
        &params.active_card_model,
        &params.world_model_registry,
        &params.active_world_model,
        &params.location_model_registry,
        &params.active_locations,
        &mut params.meshes,
        &mut params.materials,
        params
            .masked_background_materials
            .map(|materials| materials.into_inner()),
    );
}

pub fn setup_game_view_with_params(params: SetupGameViewParams) {
    setup_game_view(params);
}

fn spawn_game_view_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    hud_parent: Option<Entity>,
    asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    game_hand_cards: &[String],
    game_round_model: &GameRoundModel,
    game_location_model: &GameLocationModel,
    slot_board: &CardSlotBoardModel,
    active_card_model: &ActiveCardModel,
    world_model_registry: &WorldModelRegistry,
    active_world_model: &ActiveWorldModel,
    location_model_registry: &LocationModelRegistry,
    active_locations: &ActiveLocations,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mut masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
) {
    let mut scene = commands.spawn((
        Name::new("GameView"),
        GameViewRoot,
        GameViewEntity,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
    ));
    scene.with_children(|parent| {
        spawn_game_view_ui(
            parent,
            asset_server,
            location_model_registry,
            active_locations,
            game_round_model,
            game_location_model,
            slot_board,
        );
    });
    let scene_entity = scene.id();
    spawn_game_view_camera(commands);
    spawn_game_view_card_camera(commands, camera_defaults);
    spawn_game_view_card_overlay_camera(commands, camera_defaults);
    spawn_game_view_card_point_text_camera(commands);
    spawn_game_view_world_background(
        commands,
        asset_server,
        world_model_registry,
        active_world_model,
        meshes,
        materials,
    );
    spawn_game_view_hand_cards(
        commands,
        asset_server,
        card_defaults,
        card_model_registry,
        game_hand_cards,
        meshes,
        materials,
        masked_background_materials.as_deref_mut(),
    );
    spawn_card_slot_gesture_targets(commands, card_defaults, slot_board);

    if let Some(parent) = hud_parent.or(app_scene_parent) {
        commands.entity(parent).add_child(scene_entity);
    }
    let _ = active_card_model;
}

fn initialize_game_models_for_player(
    player_deck_collection: &PlayerDeckCollectionModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    game_round_model: &mut GameRoundModel,
    game_location_model: &mut GameLocationModel,
    card_states: &mut CardStateModel,
) {
    let mut source_deck = player_deck_collection
        .primary_deck()
        .cloned()
        .unwrap_or_else(DeckModel::default);
    if source_deck.cards.is_empty() {
        source_deck.cards = random_shuffled_default_deck_cards();
    }

    game_deck_model.cards = source_deck.cards;
    game_deck_model
        .cards
        .truncate(crate::runtime::resources::STARTING_DECK_CARD_COUNT);
    if game_deck_model.cards.is_empty() {
        game_deck_model.reset_randomized();
    }
    game_round_model.reset();
    game_location_model.reset();
    game_hand_model.cards.clear();
    game_deck_model.deal_to_hand(game_round_model.requested_cards_to_deal, game_hand_model);
    card_states.reset_to_size(game_hand_model.len());
}

fn initialize_legacy_game_models_for_player(
    player_deck_collection: &PlayerDeckCollectionModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    card_states: &mut CardStateModel,
) {
    let mut source_deck = player_deck_collection
        .primary_deck()
        .cloned()
        .unwrap_or_else(DeckModel::default);
    if source_deck.cards.is_empty() {
        source_deck.cards = random_shuffled_default_deck_cards();
    }

    game_deck_model.cards = source_deck.cards;
    game_hand_model.cards.clear();
    game_deck_model.draw_to_hand(STARTING_HAND_CARD_COUNT, game_hand_model);
    card_states.reset_to_size(game_hand_model.len());
}

fn spawn_card_slot_gesture_targets(
    commands: &mut Commands,
    card_defaults: &CardInspectionDefaults,
    slot_board: &CardSlotBoardModel,
) {
    for location_index in 0..3 {
        for slot_index in 0..4 {
            for side in [CardSlotSide::Opponent, CardSlotSide::LocalPlayer] {
                commands.spawn((
                    Name::new(format!(
                        "GameView {:?} Card Slot {}-{}",
                        side, location_index, slot_index
                    )),
                    GameViewEntity,
                    CardSlotGestureTarget::new(location_index, side, slot_index),
                    card_gesture_animation_system::slot_transform(
                        location_index,
                        slot_index,
                        side,
                        slot_board,
                        card_defaults,
                    ),
                    GlobalTransform::default(),
                    Visibility::Hidden,
                ));
            }
        }
    }
}

fn spawn_drop_target_hints(parent: &mut ChildSpawnerCommands, slot_board: &CardSlotBoardModel) {
    for location_index in 0..3 {
        let Some((min, max)) =
            card_gesture_animation_system::local_slots_area_rect(location_index, slot_board)
        else {
            continue;
        };
        let size = max - min;
        parent.spawn((
            Name::new(format!("DropTargetHint {location_index}")),
            GameViewEntity,
            DropTargetHint::new(location_index),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(min.x),
                top: Val::Px(min.y),
                width: Val::Px(size.x),
                height: Val::Px(size.y),
                border: UiRect::all(Val::Px(3.0)),
                ..Default::default()
            },
            BorderColor::all(Color::srgb(0.48, 0.82, 1.0)),
            BackgroundColor(Color::srgba(0.28, 0.72, 1.0, 0.12)),
            GlobalZIndex(12),
            Visibility::Hidden,
        ));
    }
}

fn spawn_game_view_ui(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    location_model_registry: &LocationModelRegistry,
    active_locations: &ActiveLocations,
    game_round_model: &GameRoundModel,
    game_location_model: &GameLocationModel,
    slot_board: &CardSlotBoardModel,
) {
    parent
        .spawn((
            Name::new("GameView UI"),
            GameViewEntity,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            spawn_location_area_bundles(
                parent,
                asset_server,
                slot_board,
                location_model_registry,
                active_locations,
                game_location_model,
            );
            spawn_drop_target_hints(parent, slot_board);
            spawn_local_player_hand(parent);
            spawn_game_controls(parent, asset_server, game_round_model);
        });
}

fn spawn_game_view_world_background(
    commands: &mut Commands,
    asset_server: &AssetServer,
    world_model_registry: &WorldModelRegistry,
    active_world_model: &ActiveWorldModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let world_model = world_model_registry.active_world_model(active_world_model);
    let background_size = game_view_world_background_size();
    commands
        .spawn((
            Name::new(format!("{} World Background", world_model.display_name)),
            WorldBackground,
            GameViewEntity,
            Mesh3d(meshes.add(Rectangle::new(background_size.x, background_size.y))),
            MeshMaterial3d(card_model_material(
                asset_server,
                materials,
                world_model.background_texture,
                AlphaMode::Opaque,
                BACKGROUND_DEPTH_BIAS,
            )),
            Transform::from_translation(Vec3::new(0.0, 0.0, GAME_SCENE_WORLD_BACKGROUND_Z)),
            Visibility::Visible,
        ))
        .id()
}

/// HUMAN: Spawns one visual overlay in each runtime location area.
/// AI: Draws the bundle background at the bundle rect and overlays the yellow border.
fn spawn_location_area_bundles(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    slot_board: &CardSlotBoardModel,
    location_model_registry: &LocationModelRegistry,
    active_locations: &ActiveLocations,
    game_location_model: &GameLocationModel,
) {
    let selected_locations = location_model_registry.selected_locations(active_locations);

    for location_index in 0..CARD_SLOT_LOCATION_COUNT {
        let Some(area_rect) = slot_board.location_area_rect(location_index) else {
            continue;
        };
        let bundle_size = LocationViewBundle::scaled_size(area_rect);

        if let Some(location) = selected_locations.get(location_index) {
            let location_definition = game_location_model.definition(location_index);
            let title = location_definition
                .map(|definition| definition.display_title(game_location_model.round))
                .unwrap_or(location.display_name);
            let body = location_definition
                .map(|definition| definition.display_body(game_location_model.round))
                .unwrap_or("");
            let border_color = game_location_model.border_color(location_index);
            let mut bundle_entity = parent.spawn((
                LocationViewBundle::new(area_rect),
                GameLocation::new(
                    location_index,
                    if location_definition
                        .is_some_and(|definition| definition.is_open(game_location_model.round))
                    {
                        LocationRevealState::Revealed
                    } else {
                        LocationRevealState::Unrevealed
                    },
                ),
            ));
            bundle_entity.with_children(|parent| {
                parent.spawn((
                    Name::new(format!("Game Location Background {location_index}")),
                    ImageNode::new(asset_server.load(location.texture))
                        .with_mode(bevy::ui::widget::NodeImageMode::Stretch),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Px(bundle_size.x),
                        height: Val::Px(bundle_size.y),
                        ..Default::default()
                    },
                ));

                spawn_location_title_and_body(parent, location_index, title, body, bundle_size);

                parent.spawn((
                    Name::new(format!("Game Location Border {location_index}")),
                    GameLocationBorder::new(location_index),
                    BackgroundColor(Color::NONE),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Px(bundle_size.x),
                        height: Val::Px(bundle_size.y),
                        border: UiRect::all(Val::Px(LocationViewBundle::BORDER_THICKNESS)),
                        ..Default::default()
                    },
                    BorderColor::all(border_color),
                    GlobalZIndex(2),
                ));

                let location_score = LocationScoreModel::empty(location_index);
                spawn_location_power_point_view(
                    parent,
                    asset_server,
                    location_score.opponent_total,
                    location_index,
                    CardSlotSide::Opponent,
                    bundle_size.x,
                    true,
                );
                spawn_location_power_point_view(
                    parent,
                    asset_server,
                    location_score.local_total,
                    location_index,
                    CardSlotSide::LocalPlayer,
                    bundle_size.x,
                    false,
                );
            });
        }
    }
}

fn game_view_perspective_view_size_at_z(z: f32) -> Vec2 {
    let distance = (GAME_SCENE_CAMERA_DISTANCE_FROM_ORIGIN - z).abs();
    let height = 2.0 * (PRIMARY_CAMERA_FOV_RADIANS * 0.5).tan() * distance;

    Vec2::new(height * GAME_VIEW_ASPECT_RATIO, height)
}

/// HUMAN: Sizes the GameView world backdrop to cover the safe gameplay viewport.
/// AI: Keep this tied to the GameView camera projection so background tests match runtime framing.
fn game_view_world_background_size() -> Vec2 {
    game_view_perspective_view_size_at_z(GAME_SCENE_WORLD_BACKGROUND_Z)
        * GAME_SCENE_WORLD_BACKGROUND_BLEED
}

/// HUMAN: Scales the deck builder card to fill most of the centered presentation view.
/// AI: The centered card uses world units so it remains independent of Bevy UI layout.
fn deck_builder_centered_card_scale(card_defaults: &CardInspectionDefaults) -> f32 {
    game_view_world_height_for_game_view_height(
        GAME_VIEW_HEIGHT * DECK_BUILDER_CARD_HEIGHT_FRACTION,
        0.0,
    ) / card_defaults.height
}

fn game_view_world_position_from_game_view(game_view_position: Vec2, z: f32) -> Vec3 {
    let view_size = game_view_perspective_view_size_at_z(z);

    Vec3::new(
        ((game_view_position.x / GAME_VIEW_WIDTH) - 0.5) * view_size.x,
        (0.5 - (game_view_position.y / GAME_VIEW_HEIGHT)) * view_size.y,
        z,
    )
}

fn game_view_position_from_world_position(world_position: Vec3) -> Vec2 {
    let view_size = game_view_perspective_view_size_at_z(world_position.z);

    Vec2::new(
        ((world_position.x / view_size.x) + 0.5) * GAME_VIEW_WIDTH,
        (0.5 - (world_position.y / view_size.y)) * GAME_VIEW_HEIGHT,
    )
}

fn game_view_text2d_position_from_game_view(game_view_position: Vec2, z: f32) -> Vec3 {
    Vec3::new(
        game_view_position.x - (GAME_VIEW_WIDTH * 0.5),
        (GAME_VIEW_HEIGHT * 0.5) - game_view_position.y,
        z,
    )
}

fn game_view_world_height_for_game_view_height(game_view_height: f32, z: f32) -> f32 {
    game_view_perspective_view_size_at_z(z).y * (game_view_height / GAME_VIEW_HEIGHT)
}

fn game_view_world_width_for_game_view_width(game_view_width: f32, z: f32) -> f32 {
    game_view_perspective_view_size_at_z(z).x * (game_view_width / GAME_VIEW_WIDTH)
}

fn spawn_location_title_and_body(
    parent: &mut ChildSpawnerCommands,
    location_index: usize,
    title: &'static str,
    body: &'static str,
    bundle_size: Vec2,
) {
    parent.spawn((
        Name::new("Game Location Title Text"),
        GameLocationTitleText::new(location_index),
        Text::new(title),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(8.0),
            top: Val::Px(bundle_size.y * 0.3),
            width: Val::Px(bundle_size.x - 16.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        GlobalZIndex(3),
    ));
    parent.spawn((
        Name::new("Game Location Body Text"),
        GameLocationBodyText::new(location_index),
        Text::new(body),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: 13.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(8.0),
            top: Val::Px((bundle_size.y * 0.3) + 50.0),
            width: Val::Px(bundle_size.x - 16.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        GlobalZIndex(3),
    ));
}

/// HUMAN: Refreshes visible location reveal state after round changes.
/// AI: Keep spawned GameView location components in sync with GameLocationModel.
pub fn update_game_location_views_system(
    game_location_model: Option<Res<GameLocationModel>>,
    mut location_query: Query<&mut GameLocation>,
    mut text_queries: ParamSet<(
        Query<(&GameLocationTitleText, &mut Text)>,
        Query<(&GameLocationBodyText, &mut Text)>,
    )>,
    mut border_query: Query<(&GameLocationBorder, &mut BorderColor)>,
) {
    let Some(game_location_model) = game_location_model.as_deref() else {
        return;
    };

    for mut location in &mut location_query {
        location.reveal_state = game_location_model
            .definition(location.index)
            .filter(|definition| definition.is_open(game_location_model.round))
            .map_or(LocationRevealState::Unrevealed, |_| {
                LocationRevealState::Revealed
            });
    }

    for (title, mut text) in &mut text_queries.p0() {
        if let Some(definition) = game_location_model.definition(title.location_index) {
            text.0 = definition
                .display_title(game_location_model.round)
                .to_string();
        }
    }

    for (body, mut text) in &mut text_queries.p1() {
        if let Some(definition) = game_location_model.definition(body.location_index) {
            text.0 = definition
                .display_body(game_location_model.round)
                .to_string();
        }
    }

    for (border, mut color) in &mut border_query {
        *color = BorderColor::all(game_location_model.border_color(border.location_index));
    }
}

/// HUMAN: Spawns top and bottom location power badges centered on bundle edges.
/// AI: Reuses the point root bundle and child circle/text structure used by card badges.
fn spawn_location_power_point_view(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    model: PowerPointModel,
    location_index: usize,
    side: CardSlotSide,
    location_width: f32,
    is_top: bool,
) {
    let point_model = PointModel::from_power_point(PointType::LocationPower, model);
    let point_width = LOCATION_POINT_VIEW_WIDTH;
    let point_height = LOCATION_POINT_VIEW_HEIGHT;
    let point_offset = LOCATION_POINT_VIEW_HALF_HEIGHT;
    let x_offset = (location_width - point_width) / 2.0;
    let mut node = Node {
        width: Val::Px(point_width),
        height: Val::Px(point_height),
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        left: Val::Px(x_offset),
        ..Default::default()
    };

    if is_top {
        node.top = Val::Px(-point_offset);
    } else {
        node.bottom = Val::Px(-point_offset);
    }

    parent
        .spawn((
            PointViewBundle::new("PowerPointView", point_model),
            PointLocationView::new(location_index, side),
            node,
            GlobalZIndex(3),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("PowerPointView Circle"),
                BackgroundColor(point_model.background_color()),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(point_width),
                    height: Val::Px(point_height),
                    border_radius: BorderRadius::all(Val::Px(point_height * 0.5)),
                    ..Default::default()
                },
            ));
            parent.spawn((
                Name::new("PowerPointView Text"),
                Text::new(point_model.display_text()),
                TextFont {
                    font: POINT_VIEW_BUNDLE_FONT.handle(asset_server),
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(point_model.text_color()),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });
}

fn spawn_card_cost_point_view(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    model: CostPointModel,
    background_mesh: Handle<Mesh>,
    background_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
) {
    let point_model = PointModel::from_cost_point(model);
    spawn_card_point_view_world(
        parent,
        asset_server,
        "Card EnergyPointView Background",
        point_model,
        background_mesh,
        background_material,
        background_translation,
        text_translation,
        is_visible,
    );
}

fn spawn_card_power_point_view(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    model: PowerPointModel,
    background_mesh: Handle<Mesh>,
    background_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
) {
    let point_model = PointModel::from_power_point(PointType::CardPower, model);
    spawn_card_point_view_world(
        parent,
        asset_server,
        "Card PowerPointView Background",
        point_model,
        background_mesh,
        background_material,
        background_translation,
        text_translation,
        is_visible,
    );
}

/// HUMAN: Recalculates visible location power totals from runtime slot occupancy.
/// AI: This is the GameView bridge from placed card slots to point presentation.
pub fn update_location_power_points(
    slot_board: Res<CardSlotBoardModel>,
    card_model_registry: Res<CardModelRegistry>,
    game_location_model: Option<Res<GameLocationModel>>,
    game_hand_model: Option<Res<GameHandModel>>,
    mut power_query: Query<(&PointLocationView, &mut PointView, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (location_power_view, mut power_view, children) in &mut power_query {
        if power_view.model.point_type != PointType::LocationPower {
            continue;
        }
        let next_model = location_side_power_total(
            &slot_board,
            &card_model_registry,
            game_hand_model
                .as_deref()
                .map(|hand| hand.cards.as_slice())
                .unwrap_or_default(),
            game_location_model.as_deref(),
            location_power_view.location_index,
            location_power_view.side,
        );
        let next_model = PointModel::from_power_point(PointType::LocationPower, next_model);
        if power_view.model == next_model {
            continue;
        }

        power_view.model = next_model;
        let display_text = next_model.display_text();
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = display_text.clone();
            }
        }
    }
}

fn location_side_power_total(
    slot_board: &CardSlotBoardModel,
    card_model_registry: &CardModelRegistry,
    game_hand_cards: &[String],
    game_location_model: Option<&GameLocationModel>,
    location_index: usize,
    side: CardSlotSide,
) -> PowerPointModel {
    let total = slot_board
        .slots()
        .filter(|slot| slot.location_index == location_index && slot.side == side)
        .filter_map(|slot| match &slot.state {
            CardSlotState::Empty => None,
            CardSlotState::Populated {
                hand_index,
                card_id,
            } => card_model_registry
                .card_model_for_id(card_id)
                .or_else(|| {
                    game_hand_cards.get(*hand_index).and_then(|hand_card_id| {
                        card_model_registry.card_model_for_id(hand_card_id)
                    })
                })
                .map(|card_model| {
                    card_model.base_power.value
                        + game_location_model
                            .map(|locations| {
                                locations.ability_delta_for_location(slot.location_index)
                            })
                            .unwrap_or(0)
                }),
        })
        .sum();

    PowerPointModel::new(total)
}

/// HUMAN: Applies open location power abilities to the red power point value on placed cards.
/// AI: Keep this in PointView data; presentation can render its display_text normally.
pub(crate) fn update_card_power_point_views_system(
    slot_board: Res<CardSlotBoardModel>,
    card_model_registry: Res<CardModelRegistry>,
    game_location_model: Option<Res<GameLocationModel>>,
    game_hand_model: Option<Res<GameHandModel>>,
    card_query: Query<(&HandCardGestureTarget, &Children), With<CardGestureView>>,
    mut point_query: Query<(&mut PointView, Option<&Children>)>,
    mut text_query: Query<(&CardPointTextView, &mut Text2d)>,
) {
    for (hand_target, children) in &card_query {
        let Some(card_id) = game_hand_model
            .as_deref()
            .and_then(|hand| hand.cards.get(hand_target.hand_index))
        else {
            continue;
        };
        let Some(card_model) = card_model_registry.card_model_for_id(card_id) else {
            continue;
        };
        let effective_power = card_model.base_power.value
            + slot_board
                .local_slot_for_card(hand_target.hand_index)
                .and_then(|(location_index, _)| {
                    game_location_model
                        .as_deref()
                        .map(|locations| locations.ability_delta_for_location(location_index))
                })
                .unwrap_or(0);
        let next_point_model = PointModel::card_power(effective_power);
        for child in children.iter() {
            if let Ok((mut point_view, point_children)) = point_query.get_mut(child)
                && point_view.model.point_type == PointType::CardPower
            {
                point_view.model = next_point_model;
                if let Some(point_children) = point_children {
                    for point_child in point_children.iter() {
                        if let Ok((text_view, mut text)) = text_query.get_mut(point_child)
                            && text_view.point_type == next_point_model.point_type
                        {
                            text.0 = next_point_model.display_text();
                        }
                    }
                }
            }
        }
    }
}

fn spawn_card_point_view_world(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    name: &str,
    point_model: PointModel,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
) {
    parent
        .spawn((
            PointViewBundle::new(name, point_model),
            Transform::from_translation(background_translation),
            RenderLayers::layer(CARD_RENDER_LAYER),
            CardFaceLayer::new(CardFace::Front),
            if is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new(format!("{name} Circle")),
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::default(),
                RenderLayers::layer(CARD_RENDER_LAYER),
                NoCpuCulling,
                NoFrustumCulling,
            ));
            parent.spawn((
                Name::new(format!("{name} Text")),
                CardPointTextView::new(point_model.point_type),
                Text2d::new(point_model.display_text()),
                TextFont {
                    font: POINT_VIEW_BUNDLE_FONT.handle(asset_server),
                    font_size: CARD_POINT_TEXT_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(point_model.text_color()),
                TextLayout::new_with_justify(Justify::Center),
                Anchor::CENTER,
                Transform {
                    translation: text_translation - background_translation,
                    ..Default::default()
                },
                RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER),
                NoCpuCulling,
            ));
        })
        .observe(card_click_navigation);
}

/// HUMAN: Projects card-owned point Text2d onto the 2D text camera while keeping PointView ownership.
/// AI: Text2d is a 2D renderable, so derive its 2D world transform from the 3D point root.
pub(crate) fn update_card_point_text2d_overlay_system(
    point_query: Query<&GlobalTransform, With<PointView>>,
    mut text_query: Query<(&ChildOf, &mut Transform), With<CardPointTextView>>,
) {
    for (child_of, mut text_transform) in &mut text_query {
        let Ok(point_transform) = point_query.get(child_of.parent()) else {
            continue;
        };

        *text_transform = card_point_text2d_local_transform(point_transform);
    }
}

fn card_point_text2d_local_transform(point_transform: &GlobalTransform) -> Transform {
    let game_view_position = game_view_position_from_world_position(point_transform.translation());
    let text_position =
        game_view_text2d_position_from_game_view(game_view_position, CARD_POINT_TEXT_Z);
    let text_global_transform = GlobalTransform::from(Transform {
        translation: text_position,
        scale: Vec3::splat(card_point_text2d_scale_from_point_transform(
            point_transform,
        )),
        ..Default::default()
    });
    let local_matrix = point_transform.affine().inverse() * text_global_transform.affine();

    Transform::from_matrix(Mat4::from(local_matrix))
}

fn card_point_text2d_scale_from_point_transform(point_transform: &GlobalTransform) -> f32 {
    let (point_scale, _, point_translation) = point_transform.to_scale_rotation_translation();
    let point_world_scale = point_scale
        .x
        .abs()
        .max(point_scale.y.abs())
        .max(f32::EPSILON);
    let point_view_size = game_view_perspective_view_size_at_z(point_translation.z);
    let reference_view_size = game_view_perspective_view_size_at_z(GAME_SCENE_HAND_CARD_WORLD_Z);
    let point_pixels_per_world_unit = GAME_VIEW_HEIGHT / point_view_size.y;
    let reference_pixels_per_world_unit = GAME_VIEW_HEIGHT / reference_view_size.y;

    point_world_scale * (point_pixels_per_world_unit / reference_pixels_per_world_unit)
}

/// HUMAN: World-space Text2d label for a card point badge.
/// AI: Keep point text flat in the 3D world; do not use mesh digits or UI text here.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct CardPointTextView {
    point_type: PointType,
}

impl CardPointTextView {
    pub const fn new(point_type: PointType) -> Self {
        Self { point_type }
    }
}

fn spawn_local_player_hand(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Name::new("Local Player Hand"),
        LocalPlayerHand,
        GameViewEntity,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(GAME_SCENE_HAND_LEFT),
            top: Val::Px(GAME_SCENE_HAND_TOP),
            width: Val::Px(GAME_SCENE_HAND_WIDTH),
            height: Val::Px(GAME_SCENE_HAND_HEIGHT),
            border: UiRect::all(Val::Px(3.0)),
            ..Default::default()
        },
        BorderColor::all(Color::srgb(0.44, 0.35, 0.22)),
        BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.22)),
        GlobalZIndex(10),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
    ));
}

fn game_view_hand_area_min() -> Vec2 {
    Vec2::new(GAME_SCENE_HAND_LEFT, GAME_SCENE_HAND_TOP)
}

fn game_view_hand_area_size() -> Vec2 {
    Vec2::new(GAME_SCENE_HAND_WIDTH, GAME_SCENE_HAND_HEIGHT)
}

fn game_view_hand_card_size() -> Vec2 {
    Vec2::new(GAME_SCENE_HAND_CARD_WIDTH, GAME_SCENE_HAND_CARD_HEIGHT)
}

// HUMAN: Size and position hand cards using shared hand-area geometry.
// AI: Use a single source of truth for card height and group centering calculations.
fn spawn_game_view_hand_cards(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    game_hand_cards: &[String],
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mut masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
) {
    let card_models: Vec<CardModel> = game_hand_cards
        .iter()
        .filter_map(|card_id| card_model_registry.card_model_for_id(card_id))
        .cloned()
        .collect();
    let hitboxes = game_view_card_hitboxes_for_count(card_models.len());
    let card_size = game_view_hand_card_size();

    for (index, card_model) in card_models.into_iter().enumerate() {
        let (card_min, card_max) = hitboxes[index];
        let card_z = game_view_hand_card_z(index, None);
        let card_world_scale =
            game_view_world_height_for_game_view_height(card_size.y, card_z) / card_defaults.height;
        spawn_game_view_hand_card(
            commands,
            asset_server,
            card_defaults,
            card_model,
            meshes,
            materials,
            masked_background_materials.as_deref_mut(),
            index,
            Transform {
                translation: game_view_world_position_from_game_view(
                    (card_min + card_max) * 0.5,
                    card_z,
                ),
                scale: Vec3::splat(card_world_scale),
                ..Default::default()
            },
        );
    }
}

fn spawn_game_view_hand_card(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model: CardModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    hand_index: usize,
    transform: Transform,
) {
    let card = spawn_card_structure_for_type(
        commands,
        asset_server,
        card_defaults,
        card_model,
        meshes,
        materials,
        masked_background_materials,
        CardFace::Front,
        transform,
    );
    commands
        .entity(card)
        .insert((
            GameViewEntity,
            LocalPlayerHandCardPreview,
            HandCardGestureTarget::new(hand_index),
            CardGestureView,
        ))
        .observe(card_click_navigation);
}

/// HUMAN: Creates and removes rendered hand card entities as cards are dealt between rounds.
/// AI: GameHandModel is authoritative; this system reconciles spawned card roots to it.
pub fn sync_game_view_hand_card_entities_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    game_hand_model: Option<Res<GameHandModel>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
    card_query: Query<(Entity, &HandCardGestureTarget), With<CardGestureView>>,
) {
    let Some(game_hand_model) = game_hand_model.as_deref() else {
        return;
    };

    let mut existing_indices = std::collections::HashSet::new();
    for (entity, target) in &card_query {
        if target.hand_index >= game_hand_model.len() {
            commands.entity(entity).despawn();
        } else {
            existing_indices.insert(target.hand_index);
        }
    }

    let card_world_scale = game_view_world_height_for_game_view_height(
        GAME_SCENE_HAND_CARD_HEIGHT,
        GAME_SCENE_HAND_CARD_WORLD_Z,
    ) / card_defaults.height;
    let deal_transform = Transform {
        translation: game_view_world_position_from_game_view(
            Vec2::new(GAME_VIEW_WIDTH * 0.5, GAME_VIEW_HEIGHT + 140.0),
            GAME_SCENE_HAND_CARD_WORLD_Z,
        ),
        scale: Vec3::splat(card_world_scale),
        ..Default::default()
    };

    for (hand_index, card_id) in game_hand_model.cards.iter().enumerate() {
        if existing_indices.contains(&hand_index) {
            continue;
        }
        let Some(card_model) = card_model_registry.card_model_for_id(card_id).cloned() else {
            continue;
        };
        spawn_game_view_hand_card(
            &mut commands,
            &asset_server,
            &card_defaults,
            card_model,
            &mut meshes,
            &mut materials,
            masked_background_materials.as_deref_mut(),
            hand_index,
            deal_transform,
        );
    }
}

fn spawn_game_controls(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    round_model: &GameRoundModel,
) {
    parent
        .spawn((
            Name::new("Restart Button"),
            GameViewEntity,
            Button,
            GameControlButton::new(GameControlAction::Restart),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(2.0),
                bottom: Val::Px(132.0),
                width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor::all(END_TURN_BUTTON_NORMAL_BORDER_COLOR),
            BackgroundColor(END_TURN_BUTTON_NORMAL_COLOR),
            GlobalZIndex(10),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Restart"),
                TextFont {
                    font: GAME_BUTTON_FONT.handle(asset_server),
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });

    parent
        .spawn((
            Name::new("Undo Button"),
            GameViewEntity,
            Button,
            GameControlButton::new(GameControlAction::Undo),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(2.0),
                bottom: Val::Px(36.0),
                width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor::all(if round_model.has_undoable_moves() {
                END_TURN_BUTTON_NORMAL_BORDER_COLOR
            } else {
                GAME_CONTROL_DISABLED_BORDER_COLOR
            }),
            BackgroundColor(if round_model.has_undoable_moves() {
                END_TURN_BUTTON_NORMAL_COLOR
            } else {
                GAME_CONTROL_DISABLED_COLOR
            }),
            GlobalZIndex(10),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(round_model.energy_label()),
                TextFont {
                    font: GAME_BUTTON_FONT.handle(asset_server),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                GameControlLabel::new(GameControlAction::Undo),
            ));
            parent.spawn((
                Text::new("Undo"),
                TextFont {
                    font: GAME_BUTTON_FONT.handle(asset_server),
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });

    parent
        .spawn((
            Name::new("TurnUI"),
            TurnUi,
            GameViewEntity,
            Button,
            GameControlButton::new(GameControlAction::EndTurn),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Percent(2.0),
                bottom: Val::Percent(3.0),
                width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor::all(END_TURN_BUTTON_NORMAL_BORDER_COLOR),
            BackgroundColor(END_TURN_BUTTON_NORMAL_COLOR),
            GlobalZIndex(10),
            Visibility::Visible,
            EndTurnButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!(
                    "Turn {}/{}",
                    round_model.round, round_model.max_rounds
                )),
                TextFont {
                    font: GAME_BUTTON_FONT.handle(asset_server),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                GameControlLabel::new(GameControlAction::EndTurn),
            ));
            parent.spawn((
                Text::new("Next"),
                TextFont {
                    font: GAME_BUTTON_FONT.handle(asset_server),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_deck_builder_light(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("DeckBuilderScene Key Light"),
            DeckBuilderSceneEntity,
            DirectionalLight {
                illuminance: 1500.0,
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, -0.35, 0.0)),
        ))
        .id()
}

fn spawn_debug_settings_light(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("DebugSettingsScene Key Light"),
            DebugSettingsSceneEntity,
            DirectionalLight {
                illuminance: 1500.0,
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, -0.35, 0.0)),
        ))
        .id()
}

/// HUMAN: Spawns the deck builder sub-screen view.
/// AI: DeckBuilderScene now renders a deck list and deck card list (no CardView cards).
pub fn setup_deck_builder_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_defaults: Res<PrimaryCameraDefaults>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    player_deck_collection: Option<Res<PlayerDeckCollectionModel>>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
    let player_deck_collection = player_deck_collection
        .as_deref()
        .unwrap_or(&fallback_player_deck_collection);
    spawn_deck_builder_scene_contents(
        &mut commands,
        &asset_server,
        &camera_defaults,
        &card_defaults,
        &card_model_registry,
        &player_deck_collection,
        &mut meshes,
        &mut materials,
        masked_background_materials.map(|materials| materials.into_inner()),
        app_scene_query.single().ok(),
        CardFace::Front,
        Quat::IDENTITY,
    );
}

fn spawn_deck_builder_scene_contents(
    commands: &mut Commands,
    _asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    _card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    player_deck_collection: &PlayerDeckCollectionModel,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
    _masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    _app_scene_parent: Option<Entity>,
    _visible_face: CardFace,
    _initial_rotation: Quat,
) {
    let scene_root = commands
        .spawn((
            Name::new("DeckBuilderScene"),
            DeckBuilderSceneRoot,
            DeckBuilderSceneEntity,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id();
    let camera = spawn_primary_camera(commands, camera_defaults);
    let ui_camera = spawn_deck_builder_ui_camera(commands);
    let light = spawn_deck_builder_light(commands);
    let deck_cards = player_deck_collection
        .primary_deck()
        .filter(|deck| !deck.cards.is_empty())
        .map(|deck| deck.cards.clone())
        .unwrap_or_else(random_shuffled_default_deck_cards);
    let deck_panel = commands
        .spawn((
            Name::new("DeckBuilder Content"),
            DeckBuilderSceneEntity,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                column_gap: Val::Px(16.0),
                ..Default::default()
            },
            Visibility::Visible,
        ))
        .id();
    let deck_list_panel = commands
        .spawn((
            Name::new("Deck List"),
            Node {
                width: Val::Percent(24.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(8.0),
                ..Default::default()
            },
        ))
        .id();
    let card_list_panel = commands
        .spawn((
            Name::new("Deck Cards"),
            Node {
                width: Val::Percent(72.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(deck_list_panel).with_children(|parent| {
        parent
            .spawn((
                Name::new("Deck Name Button"),
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(36.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.14, 0.14, 0.14)),
                BorderColor::all(Color::srgba(0.34, 0.34, 0.34, 0.95)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(DEFAULT_DECK_NAME),
                    TextFont {
                        font_size: 16.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
    });
    commands.entity(card_list_panel).with_children(|parent| {
        let card_count = deck_cards.len();
        parent
            .spawn((
                Name::new("Deck Cards Header"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(28.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(format!("Deck Cards ({card_count})")),
                    TextFont {
                        font_size: 14.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            });

        for card_id in &deck_cards {
            let card_model = card_model_registry.card_model_for_id(card_id);
            let card_label = card_model.map_or(card_id.as_str(), |card| card.display_name);
            parent
                .spawn((
                    Name::new(format!("Deck Card Preview {card_label}")),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(34.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.09, 0.09, 0.09, 0.85)),
                    BorderColor::all(Color::srgb(0.34, 0.34, 0.34)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(card_label),
                        TextFont {
                            font_size: 12.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        }
    });
    // Keep 3D content out of the UI node hierarchy so resize-driven UI layout
    // transforms cannot move or scale the deck builder presentation.
    commands.entity(scene_root).add_child(camera);
    commands.entity(scene_root).add_child(ui_camera);
    commands.entity(scene_root).add_child(light);
    commands.entity(scene_root).add_child(deck_panel);
    commands.entity(deck_panel).add_child(deck_list_panel);
    commands.entity(deck_panel).add_child(card_list_panel);
}

/// HUMAN: Spawns the debug settings sub-screen scene.
/// AI: DebugSettingsScene duplicates DeckBuilderScene presentation for debug configuration work.
pub fn setup_debug_settings_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_defaults: Res<PrimaryCameraDefaults>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    active_card_model: Res<ActiveCardModel>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    spawn_debug_settings_scene_contents(
        &mut commands,
        &asset_server,
        &camera_defaults,
        &card_defaults,
        &card_model_registry,
        &active_card_model,
        &mut meshes,
        &mut materials,
        masked_background_materials.map(|materials| materials.into_inner()),
        app_scene_query.single().ok(),
        CardFace::Front,
        Quat::IDENTITY,
    );
}

fn spawn_debug_settings_scene_contents(
    commands: &mut Commands,
    asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    active_card_model: &ActiveCardModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    _app_scene_parent: Option<Entity>,
    visible_face: CardFace,
    initial_rotation: Quat,
) {
    let scene_root = commands
        .spawn((
            Name::new("DebugSettingsScene"),
            DebugSettingsSceneRoot,
            DebugSettingsSceneEntity,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id();
    let camera = spawn_debug_settings_primary_camera(commands, camera_defaults);
    let ui_camera = spawn_debug_settings_ui_camera(commands);
    let light = spawn_debug_settings_light(commands);
    let card = spawn_card_structure(
        commands,
        asset_server,
        card_defaults,
        card_model_registry,
        active_card_model,
        meshes,
        materials,
        masked_background_materials,
        visible_face,
        debug_settings_scene_card_transform(card_defaults, initial_rotation),
    );
    commands.entity(scene_root).add_child(camera);
    commands.entity(scene_root).add_child(ui_camera);
    commands.entity(scene_root).add_child(light);
    commands.entity(scene_root).add_child(card);
    commands
        .entity(card)
        .insert(DebugSettingsSceneEntity)
        .observe(card_click_navigation);
}

pub fn setup_card_placeholder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    active_card_model: Res<ActiveCardModel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    spawn_card_structure(
        &mut commands,
        &asset_server,
        &card_defaults,
        &card_model_registry,
        &active_card_model,
        &mut meshes,
        &mut materials,
        masked_background_materials.map(|materials| materials.into_inner()),
        CardFace::Front,
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(deck_builder_centered_card_scale(&card_defaults)),
        },
    );
}

fn spawn_card_structure(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    active_card_model: &ActiveCardModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    visible_face: CardFace,
    transform: Transform,
) -> Entity {
    let card_model = card_model_registry
        .active_card_model(&active_card_model)
        .cloned()
        .unwrap_or_else(CardModel::kage_ren);
    spawn_card_structure_for_type(
        commands,
        asset_server,
        card_defaults,
        card_model,
        meshes,
        materials,
        masked_background_materials,
        visible_face,
        transform,
    )
}

/// HUMAN: Positions the DebugSettingsScene card near the card control panel.
/// AI: Size uses Card UI width and offsets by a fixed gap so the model and UI sit beside each other.
fn debug_settings_scene_card_transform(
    card_defaults: &CardInspectionDefaults,
    rotation: Quat,
) -> Transform {
    let target_card_width = DEBUG_WINDOW_WIDTH;
    let target_card_scale =
        game_view_world_width_for_game_view_width(target_card_width, 0.0) / card_defaults.width;
    let target_card_height = target_card_width / (card_defaults.width / card_defaults.height);
    let card_center = game_view_world_position_from_game_view(
        Vec2::new(
            GAME_VIEW_WIDTH
                - SCREEN_PADDING_LEFT
                - DEBUG_SETTINGS_CARD_GAP_TO_CARD_UI
                - (target_card_width * 1.5),
            SCREEN_PADDING_TOP + (target_card_height * 0.5),
        ),
        0.0,
    );

    Transform {
        translation: card_center,
        rotation,
        scale: Vec3::splat(target_card_scale),
    }
}

fn spawn_card_structure_for_type(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model: CardModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    visible_face: CardFace,
    transform: Transform,
) -> Entity {
    let background_material = card_model_material(
        asset_server,
        materials,
        card_model.background_texture,
        AlphaMode::Opaque,
        BACKGROUND_DEPTH_BIAS,
    );
    let frame_material = card_model_material(
        asset_server,
        materials,
        card_model.frame_texture,
        AlphaMode::Opaque,
        FRAME_DEPTH_BIAS,
    );
    let foreground_material = card_model_material(
        asset_server,
        materials,
        card_model.foreground_texture,
        AlphaMode::AlphaToCoverage,
        FOREGROUND_DEPTH_BIAS,
    );
    let title_material = card_model_material(
        asset_server,
        materials,
        card_model.title_texture,
        AlphaMode::AlphaToCoverage,
        TITLE_DEPTH_BIAS,
    );
    let safe_area_material = card_model_material(
        asset_server,
        materials,
        CARD_SAFE_AREA_TEXTURE_PATH,
        AlphaMode::Blend,
        SAFE_AREA_DEPTH_BIAS,
    );
    let card_back_material = card_model_material(
        asset_server,
        materials,
        CARD_BACK_TEXTURE_PATH,
        AlphaMode::Opaque,
        BACKGROUND_DEPTH_BIAS,
    );

    let frame_dimensions = frame_dimensions(&card_defaults);
    let frame_mask_inner_aperture = frame_mask_inner_aperture(card_defaults, &frame_dimensions);
    let card_front_z = (card_defaults.thickness * 0.5) + LAYER_RENDER_Z_STEP;
    let background_z = card_front_z;
    let frame_z = card_front_z + (LAYER_RENDER_Z_STEP * 3.0);
    let safe_area_z = card_front_z + (LAYER_RENDER_Z_STEP * 4.0);
    let foreground_z = card_front_z + (LAYER_RENDER_Z_STEP * 5.0);
    let title_z = card_front_z + (LAYER_RENDER_Z_STEP * 7.0);
    let point_background_z = card_front_z + (LAYER_RENDER_Z_STEP * 8.0);
    let point_text_z = card_front_z + (LAYER_RENDER_Z_STEP * 9.0);

    let background_mesh = if card_model.background_uses_frame_mask {
        meshes.add(background_frame_mask_mesh(card_defaults, Vec2::ZERO))
    } else {
        meshes.add(background_aperture_mesh(&frame_dimensions, Vec2::ZERO))
    };
    let frame_mesh = meshes.add(frame_cutout_mesh(card_defaults, &frame_dimensions));
    let foreground_width = card_defaults.width * card_model.foreground_width_ratio;
    let foreground_height = card_defaults.height * card_model.foreground_height_ratio;
    let foreground_mesh = meshes.add(Rectangle::new(foreground_width, foreground_height));
    let safe_area_mesh = meshes.add(Rectangle::new(card_defaults.width, card_defaults.height));
    let title_mesh = meshes.add(Rectangle::new(card_defaults.width, card_defaults.height));
    let card_back_mesh = meshes.add(Rectangle::new(card_defaults.width, card_defaults.height));
    let cost_point_background_mesh = meshes.add(Circle::new(CARD_POINT_BADGE_SIZE * 0.5));
    let power_point_background_mesh = meshes.add(Circle::new(CARD_POINT_BADGE_SIZE * 0.5));
    let cost_point_background_material = materials.add(StandardMaterial {
        base_color: PointType::CardEnergy.background_color(),
        alpha_mode: AlphaMode::Opaque,
        depth_bias: POINT_DEPTH_BIAS,
        unlit: true,
        ..Default::default()
    });
    let power_point_background_material = materials.add(StandardMaterial {
        base_color: PointType::CardPower.background_color(),
        alpha_mode: AlphaMode::Opaque,
        depth_bias: POINT_DEPTH_BIAS,
        unlit: true,
        ..Default::default()
    });
    let point_x =
        (card_defaults.width * 0.5) - (card_defaults.width * CARD_POINT_BADGE_INSET_RATIO);
    let cost_point_x = point_x + (CARD_POINT_BADGE_SIZE * 0.5);
    let power_point_x = point_x + (CARD_POINT_BADGE_SIZE * 0.5);
    let point_y =
        (card_defaults.height * 0.5) - (card_defaults.height * CARD_POINT_BADGE_INSET_RATIO);

    let mut scene_root = commands.spawn(CardViewBundle::new(&card_model, transform));
    scene_root.with_children(|parent| {
        spawn_card_back_plane(
            parent,
            card_back_mesh,
            card_back_material,
            card_defaults,
            visible_face == CardFace::Back,
        );

        if card_model.background_uses_frame_mask {
            if let Some(masked_background_materials) = masked_background_materials {
                spawn_masked_background_plane(
                    parent,
                    Name::new(format!("Card Background {}", card_model.display_name)),
                    background_mesh,
                    masked_background_materials.add(CardBackgroundMaskMaterial {
                        background_texture: asset_server.load(card_model.background_texture),
                        frame_texture: asset_server.load(card_model.frame_texture),
                        inner_aperture: frame_mask_inner_aperture,
                        alpha_mode: AlphaMode::Blend,
                    }),
                    BACKGROUND_APPARENT_DEPTH,
                    Vec3::new(0.0, 0.0, background_z),
                    visible_face == CardFace::Front,
                );
            } else {
                spawn_parallax_plane(
                    parent,
                    Name::new(format!("Card Background {}", card_model.display_name)),
                    background_mesh,
                    background_material,
                    CardLayerRole::Background,
                    BACKGROUND_APPARENT_DEPTH,
                    Vec3::new(0.0, 0.0, background_z),
                    Some(CardBackgroundLayer::new(true)),
                    false,
                    visible_face == CardFace::Front,
                );
            }
        } else {
            spawn_parallax_plane(
                parent,
                Name::new(format!("Card Background {}", card_model.display_name)),
                background_mesh,
                background_material,
                CardLayerRole::Background,
                BACKGROUND_APPARENT_DEPTH,
                Vec3::new(0.0, 0.0, background_z),
                Some(CardBackgroundLayer::new(false)),
                false,
                visible_face == CardFace::Front,
            );
        }

        spawn_parallax_plane(
            parent,
            Name::new("Card Frame Cutout"),
            frame_mesh,
            frame_material.clone(),
            CardLayerRole::Frame,
            FRAME_APPARENT_DEPTH,
            Vec3::new(0.0, 0.0, frame_z),
            None,
            true,
            visible_face == CardFace::Front,
        );

        spawn_parallax_plane(
            parent,
            Name::new("Card Safe Area Reference"),
            safe_area_mesh,
            safe_area_material,
            CardLayerRole::SafeArea,
            SAFE_AREA_APPARENT_DEPTH,
            Vec3::new(0.0, 0.0, safe_area_z),
            None,
            false,
            visible_face == CardFace::Front,
        );

        spawn_parallax_plane(
            parent,
            Name::new(format!(
                "Card Foreground {} Character",
                card_model.display_name
            )),
            foreground_mesh,
            foreground_material,
            CardLayerRole::Foreground,
            FOREGROUND_APPARENT_DEPTH,
            Vec3::new(
                card_defaults.width * card_model.foreground_x_ratio,
                card_defaults.height * card_model.foreground_y_ratio,
                foreground_z,
            ),
            None,
            false,
            visible_face == CardFace::Front,
        );
        spawn_parallax_plane(
            parent,
            Name::new(format!("Card Title {}", card_model.display_name)),
            title_mesh,
            title_material,
            CardLayerRole::Title,
            TITLE_APPARENT_DEPTH,
            Vec3::new(0.0, 0.0, title_z),
            None,
            false,
            visible_face == CardFace::Front,
        );
        spawn_card_power_point_view(
            parent,
            asset_server,
            card_model.base_power,
            power_point_background_mesh,
            power_point_background_material,
            Vec3::new(-cost_point_x, point_y, point_background_z),
            Vec3::new(-cost_point_x, point_y, point_text_z),
            visible_face == CardFace::Front,
        );
        spawn_card_cost_point_view(
            parent,
            asset_server,
            card_model.cost,
            cost_point_background_mesh,
            cost_point_background_material,
            Vec3::new(power_point_x, -point_y, point_background_z),
            Vec3::new(power_point_x, -point_y, point_text_z),
            visible_face == CardFace::Front,
        );
    });
    scene_root.id()
}

#[derive(Clone, Copy, Debug)]
struct FrameDimensions {
    frame_thickness_x: f32,
    frame_thickness_y: f32,
    hole_width: f32,
    hole_height: f32,
}

fn frame_dimensions(card_defaults: &CardInspectionDefaults) -> FrameDimensions {
    let frame_thickness = card_defaults.width * FRAME_THICKNESS_RATIO;

    FrameDimensions {
        frame_thickness_x: frame_thickness,
        frame_thickness_y: frame_thickness,
        hole_width: card_defaults.width - (frame_thickness * 2.0),
        hole_height: card_defaults.height - (frame_thickness * 2.0),
    }
}

fn frame_mask_inner_aperture(
    card_defaults: &CardInspectionDefaults,
    frame_dimensions: &FrameDimensions,
) -> Vec4 {
    let min_x = frame_dimensions.frame_thickness_x / card_defaults.width;
    let min_y = frame_dimensions.frame_thickness_y / card_defaults.height;

    Vec4::new(min_x, min_y, 1.0 - min_x, 1.0 - min_y)
}

fn frame_cutout_mesh(
    card_defaults: &CardInspectionDefaults,
    frame_dimensions: &FrameDimensions,
) -> Mesh {
    let outer_left = -card_defaults.width * 0.5;
    let outer_right = card_defaults.width * 0.5;
    let outer_bottom = -card_defaults.height * 0.5;
    let outer_top = card_defaults.height * 0.5;
    let inner_left = outer_left + frame_dimensions.frame_thickness_x;
    let inner_right = outer_right - frame_dimensions.frame_thickness_x;
    let inner_bottom = outer_bottom + frame_dimensions.frame_thickness_y;
    let inner_top = outer_top - frame_dimensions.frame_thickness_y;

    let mut positions = Vec::with_capacity(16);
    let mut normals = Vec::with_capacity(16);
    let mut uvs = Vec::with_capacity(16);
    let mut indices = Vec::with_capacity(24);

    add_frame_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        Vec2::new(outer_left, outer_bottom),
        Vec2::new(inner_left, outer_top),
        card_defaults,
    );
    add_frame_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        Vec2::new(inner_right, outer_bottom),
        Vec2::new(outer_right, outer_top),
        card_defaults,
    );
    add_frame_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        Vec2::new(inner_left, inner_top),
        Vec2::new(inner_right, outer_top),
        card_defaults,
    );
    add_frame_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        Vec2::new(inner_left, outer_bottom),
        Vec2::new(inner_right, inner_bottom),
        card_defaults,
    );

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn background_aperture_mesh(frame_dimensions: &FrameDimensions, uv_offset: Vec2) -> Mesh {
    let positions = background_aperture_positions(frame_dimensions, 1.0);
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = background_aperture_uvs(uv_offset, 1.0);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn background_frame_mask_mesh(card_defaults: &CardInspectionDefaults, uv_offset: Vec2) -> Mesh {
    let positions = background_frame_mask_positions(card_defaults);
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let background_uvs = background_frame_mask_background_uvs(uv_offset, 1.0);
    let frame_mask_uvs = background_full_card_uvs();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, background_uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, frame_mask_uvs)
}

fn background_frame_mask_positions(card_defaults: &CardInspectionDefaults) -> Vec<[f32; 3]> {
    let half_width = card_defaults.width * 0.5;
    let half_height = card_defaults.height * 0.5;

    vec![
        [-half_width, -half_height, 0.0],
        [half_width, -half_height, 0.0],
        [half_width, half_height, 0.0],
        [-half_width, half_height, 0.0],
    ]
}

fn background_frame_mask_background_uvs(uv_offset: Vec2, layer_scale: f32) -> Vec<[f32; 2]> {
    let layer_scale = layer_scale.max(f32::EPSILON);
    let visible_uv_size = 1.0 / layer_scale;
    let min = Vec2::splat((1.0 - visible_uv_size) * 0.5) + uv_offset;
    let max = min + Vec2::splat(visible_uv_size);

    vec![
        [min.x, max.y],
        [max.x, max.y],
        [max.x, min.y],
        [min.x, min.y],
    ]
}

fn background_full_card_uvs() -> Vec<[f32; 2]> {
    vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]
}

fn background_aperture_positions(
    frame_dimensions: &FrameDimensions,
    layer_scale: f32,
) -> Vec<[f32; 3]> {
    let inverse_scale = if layer_scale <= f32::EPSILON {
        1.0
    } else {
        1.0 / layer_scale
    };
    let half_width = frame_dimensions.hole_width * 0.5 * inverse_scale;
    let half_height = frame_dimensions.hole_height * 0.5 * inverse_scale;

    vec![
        [-half_width, -half_height, 0.0],
        [half_width, -half_height, 0.0],
        [half_width, half_height, 0.0],
        [-half_width, half_height, 0.0],
    ]
}

fn background_aperture_uvs(uv_offset: Vec2, layer_scale: f32) -> Vec<[f32; 2]> {
    let layer_scale = layer_scale.max(f32::EPSILON);
    let visible_uv_size = (1.0 / BACKGROUND_APERTURE_SCALE) / layer_scale;
    let min = Vec2::splat((1.0 - visible_uv_size) * 0.5) + uv_offset;
    let max = min + Vec2::splat(visible_uv_size);

    vec![
        [min.x, max.y],
        [max.x, max.y],
        [max.x, min.y],
        [min.x, min.y],
    ]
}

fn add_frame_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    min: Vec2,
    max: Vec2,
    card_defaults: &CardInspectionDefaults,
) {
    let start = positions.len() as u32;
    let corners = [
        Vec2::new(min.x, min.y),
        Vec2::new(max.x, min.y),
        Vec2::new(max.x, max.y),
        Vec2::new(min.x, max.y),
    ];

    for corner in corners {
        positions.push([corner.x, corner.y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([
            (corner.x / card_defaults.width) + 0.5,
            1.0 - ((corner.y / card_defaults.height) + 0.5),
        ]);
    }

    indices.extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
}

fn card_model_material(
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
    texture_path: &'static str,
    alpha_mode: AlphaMode,
    depth_bias: f32,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load(texture_path)),
        alpha_mode,
        depth_bias,
        cull_mode: None,
        unlit: true,
        ..Default::default()
    })
}

fn spawn_parallax_plane(
    parent: &mut ChildSpawnerCommands,
    name: Name,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    role: CardLayerRole,
    apparent_depth: f32,
    neutral_translation: Vec3,
    background_layer: Option<CardBackgroundLayer>,
    is_frame: bool,
    is_visible: bool,
) {
    let mut entity = parent.spawn((
        name,
        Mesh3d(mesh),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(neutral_translation),
        RenderLayers::layer(CARD_RENDER_LAYER),
        NoCpuCulling,
        if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        CardFaceLayer::new(CardFace::Front),
        CardParallaxLayer::new(role, apparent_depth, neutral_translation),
    ));
    entity.observe(card_click_navigation);
    if let Some(background_layer) = background_layer {
        entity.insert(background_layer);
    }
    if is_frame {
        entity.insert(CardFrameLayer);
    }
}

fn spawn_masked_background_plane(
    parent: &mut ChildSpawnerCommands,
    name: Name,
    mesh: Handle<Mesh>,
    material: Handle<CardBackgroundMaskMaterial>,
    apparent_depth: f32,
    neutral_translation: Vec3,
    is_visible: bool,
) {
    parent
        .spawn((
            name,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(neutral_translation),
            RenderLayers::layer(CARD_RENDER_LAYER),
            NoCpuCulling,
            if is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            CardFaceLayer::new(CardFace::Front),
            CardParallaxLayer::new(
                CardLayerRole::Background,
                apparent_depth,
                neutral_translation,
            ),
            CardBackgroundLayer::new(true),
        ))
        .observe(card_click_navigation);
}

fn spawn_card_back_plane(
    parent: &mut ChildSpawnerCommands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    card_defaults: &CardInspectionDefaults,
    is_visible: bool,
) {
    parent
        .spawn((
            Name::new("Card Back CardSeries Pattern"),
            Mesh3d(mesh),
            MeshMaterial3d(material),
            RenderLayers::layer(CARD_RENDER_LAYER),
            Transform {
                translation: Vec3::new(
                    0.0,
                    0.0,
                    (-card_defaults.thickness * 0.5) - LAYER_RENDER_Z_STEP,
                ),
                rotation: Quat::from_rotation_y(std::f32::consts::PI),
                ..Default::default()
            },
            NoCpuCulling,
            if is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            CardFaceLayer::new(CardFace::Back),
        ))
        .observe(card_click_navigation);
}

pub fn track_card_pointer_target(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    touches: Res<Touches>,
    card_defaults: Res<CardInspectionDefaults>,
    mut card_state: ResMut<CardInspectionState>,
) {
    let Ok(primary_window) = primary_window_query.single() else {
        return;
    };
    let Some(pointer_position) = active_pointer_position(primary_window, &touches) else {
        return;
    };

    let window_size = Vec2::new(
        primary_window.resolution.width(),
        primary_window.resolution.height(),
    );
    update_card_target_from_pointer(
        pointer_position,
        window_size,
        &card_defaults,
        &mut card_state,
    );
}

pub fn smooth_card_rotation(
    time: Res<Time>,
    card_defaults: Res<CardInspectionDefaults>,
    card_state: Res<CardInspectionState>,
    flip_state: Res<CardFlipState>,
    mut card_query: Query<&mut Transform, (With<CardView>, With<DeckBuilderSceneEntity>)>,
    mut game_card_query: Query<
        &mut Transform,
        (
            With<LocalPlayerHandCardPreview>,
            With<GameViewEntity>,
            Without<DeckBuilderSceneEntity>,
        ),
    >,
) {
    let response_seconds = card_defaults.smoothing_response_seconds.max(f32::EPSILON);
    let blend = 1.0 - 0.01_f32.powf(time.delta_secs() / response_seconds);
    if let Ok(mut transform) = card_query.single_mut() {
        let target_rotation = composed_card_rotation(&card_state, &flip_state);
        transform.rotation = transform.rotation.slerp(target_rotation, blend);
        transform.translation = Vec3::ZERO;
    }

    let game_card_rotation =
        Quat::from_rotation_z(-card_state.last_pointer_normalized.x * GAME_SCENE_CARD_TILT_RADIANS);
    for mut transform in &mut game_card_query {
        transform.rotation = transform.rotation.slerp(game_card_rotation, blend);
    }
}

pub fn log_game_view_card_render_diagnostics(
    mut has_logged: Local<bool>,
    active_view: Res<ActiveView>,
    card_query: Query<
        (&Name, &Transform, &GlobalTransform),
        (
            With<LocalPlayerHandCardPreview>,
            With<CardView>,
            With<GameViewEntity>,
            Without<DeckBuilderSceneEntity>,
        ),
    >,
    layer_query: Query<
        (&Name, &Visibility, &GlobalTransform),
        (
            With<CardParallaxLayer>,
            With<CardFaceLayer>,
            Without<DeckBuilderSceneEntity>,
        ),
    >,
    camera_query: Query<(&Name, &Camera, Option<&Projection>), With<GameViewEntity>>,
) {
    if *has_logged || *active_view != ActiveView::GameView {
        return;
    }

    let cards: Vec<String> = card_query
        .iter()
        .map(|(name, transform, global_transform)| {
            format!(
                "{} local=({:.3},{:.3},{:.3}) scale=({:.3},{:.3},{:.3}) world=({:.3},{:.3},{:.3})",
                name.as_str(),
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                transform.scale.x,
                transform.scale.y,
                transform.scale.z,
                global_transform.translation().x,
                global_transform.translation().y,
                global_transform.translation().z,
            )
        })
        .collect();
    let layer_details: Vec<String> = layer_query
        .iter()
        .map(|(name, visibility, global_transform)| {
            format!(
                "{} visibility={:?} world=({:.3},{:.3},{:.3})",
                name.as_str(),
                visibility,
                global_transform.translation().x,
                global_transform.translation().y,
                global_transform.translation().z
            )
        })
        .collect();
    let cameras: Vec<String> = camera_query
        .iter()
        .map(|(name, camera, projection)| {
            let projection_name = match projection {
                Some(Projection::Perspective(_)) => "Perspective",
                Some(Projection::Orthographic(_)) => "Orthographic",
                _ => "Other",
            };
            format!(
                "{} order={} clear={:?} projection={}",
                name.as_str(),
                camera.order,
                camera.clear_color,
                projection_name
            )
        })
        .collect();

    info!(
        "GameView 3D card render diagnostics: cards={} layers={} cameras={} card_roots=[{}] layers=[{}] cameras=[{}]",
        cards.len(),
        layer_details.len(),
        cameras.len(),
        cards.join(" | "),
        layer_details.join(" | "),
        cameras.join(" | ")
    );
    let diagnostic_path = std::path::Path::new("target/run-app-desktop/game-scene-card-render.log");
    let diagnostic_result = diagnostic_path
        .parent()
        .map(std::fs::create_dir_all)
        .transpose()
        .and_then(|_| {
            std::fs::write(
                diagnostic_path,
                format!(
                    "GameView 3D card render diagnostics: cards={} layers={} cameras={} card_roots=[{}] layers=[{}] cameras=[{}]\n",
                    cards.len(),
                    layer_details.len(),
                    cameras.len(),
                    cards.join(" | "),
                    layer_details.join(" | "),
                    cameras.join(" | ")
                ),
            )
        });
    if let Err(error) = diagnostic_result {
        warn!("Failed to write GameView render diagnostics: {error}");
    }
    *has_logged = true;
}

pub fn composed_card_rotation(
    card_state: &CardInspectionState,
    flip_state: &CardFlipState,
) -> Quat {
    card_state.target_rotation * flip_state.rotation()
}

fn composed_rotation_for_face(card_state: &CardInspectionState, face: CardFace) -> Quat {
    match face {
        CardFace::Front => card_state.target_rotation,
        CardFace::Back => card_state.target_rotation * Quat::from_rotation_y(std::f32::consts::PI),
    }
}

pub fn update_card_flip_animation(time: Res<Time>, mut flip_state: ResMut<CardFlipState>) {
    flip_state.advance(time.delta_secs());
}

pub fn update_card_face_visibility(
    flip_state: Res<CardFlipState>,
    card_ui_state: Res<CardUiState>,
    mut face_query: Query<(&CardFaceLayer, Option<&CardParallaxLayer>, &mut Visibility)>,
) {
    if !flip_state.is_changed() && !card_ui_state.is_changed() {
        return;
    }

    for (face_layer, parallax_layer, mut visibility) in &mut face_query {
        let is_hidden_safe_area = parallax_layer
            .is_some_and(|layer| layer.role == CardLayerRole::SafeArea)
            && !card_ui_state.show_safe_area;
        *visibility = if face_layer.face == flip_state.visible_face && !is_hidden_safe_area {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn update_card_parallax_layers(
    card_defaults: Res<CardInspectionDefaults>,
    card_state: Res<CardInspectionState>,
    card_ui_state: Res<CardUiState>,
    mut layer_query: Query<(
        &CardParallaxLayer,
        &mut Transform,
        Option<&Mesh3d>,
        Option<&CardBackgroundLayer>,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (yaw, pitch, _) = card_state.target_rotation.to_euler(EulerRot::YXZ);
    let max_tilt = card_defaults.max_tilt_radians.max(f32::EPSILON);
    let tilt =
        Vec2::new(yaw / max_tilt, -pitch / max_tilt).clamp(Vec2::splat(-1.0), Vec2::splat(1.0));
    let max_offset = Vec2::new(
        card_defaults.width * PARALLAX_OFFSET_RATIO,
        card_defaults.height * PARALLAX_OFFSET_RATIO,
    );
    let depth_multiplier = card_ui_state.depth_multiplier();

    let frame_dimensions = frame_dimensions(&card_defaults);
    for (layer, mut transform, mesh_handle, background_layer) in &mut layer_query {
        let offset = tilt * max_offset * layer.apparent_depth * depth_multiplier;
        let layer_scale = card_layer_scale(&card_ui_state, layer.role);
        transform.scale = Vec3::new(layer_scale, layer_scale, 1.0);
        if layer.role == CardLayerRole::Background {
            transform.translation = layer.neutral_translation;
            if let Some(mesh_handle) = mesh_handle {
                if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                    if background_layer.is_some_and(|layer| layer.uses_frame_mask) {
                        let frame_scale = card_layer_scale(&card_ui_state, CardLayerRole::Frame);
                        transform.scale = Vec3::new(frame_scale, frame_scale, 1.0);
                        let uv_offset = Vec2::new(
                            -offset.x / card_defaults.width,
                            offset.y / card_defaults.height,
                        );
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_POSITION,
                            background_frame_mask_positions(&card_defaults),
                        );
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_UV_0,
                            background_frame_mask_background_uvs(uv_offset, layer_scale),
                        );
                        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, background_full_card_uvs());
                    } else {
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_POSITION,
                            background_aperture_positions(&frame_dimensions, layer_scale),
                        );
                        let background_virtual_size = Vec2::new(
                            frame_dimensions.hole_width * BACKGROUND_APERTURE_SCALE,
                            frame_dimensions.hole_height * BACKGROUND_APERTURE_SCALE,
                        );
                        let uv_offset = Vec2::new(
                            -offset.x / background_virtual_size.x,
                            offset.y / background_virtual_size.y,
                        );
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_UV_0,
                            background_aperture_uvs(uv_offset, layer_scale),
                        );
                    }
                }
            }
        } else {
            transform.translation = layer.neutral_translation + Vec3::new(offset.x, offset.y, 0.0);
        }
    }
}

fn card_layer_scale(card_ui_state: &CardUiState, role: CardLayerRole) -> f32 {
    match role {
        CardLayerRole::Background => card_ui_state.background_layer_scale,
        CardLayerRole::Frame => card_ui_state.frame_layer_scale,
        CardLayerRole::SafeArea => 1.0,
        CardLayerRole::Foreground => card_ui_state.foreground_layer_scale,
        CardLayerRole::Title => card_ui_state.title_layer_scale,
    }
    .clamp(CARD_LAYER_SCALE_MIN, CARD_LAYER_SCALE_MAX)
}

pub fn update_card_frame_shine(
    card_defaults: Res<CardInspectionDefaults>,
    card_state: Res<CardInspectionState>,
    frame_query: Query<&MeshMaterial3d<StandardMaterial>, With<CardFrameLayer>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (yaw, pitch, _) = card_state.target_rotation.to_euler(EulerRot::YXZ);
    let max_tilt = card_defaults.max_tilt_radians.max(f32::EPSILON);
    let tilt =
        Vec2::new(yaw / max_tilt, -pitch / max_tilt).clamp(Vec2::splat(-1.0), Vec2::splat(1.0));
    let shine = ((tilt.x * 0.65) + (tilt.y * 0.35) + 1.0) * 0.5;
    let lift = shine * FRAME_SHINE_STRENGTH;
    let frame_tint = Color::srgb(0.82 + lift, 0.84 + lift, 0.88 + lift);

    for material_handle in &frame_query {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = frame_tint;
        }
    }
}

/// HUMAN: Handles T-key model/view cycling behavior.
/// AI: Uses domain_schedule_system naming; non-game scenes toggle card UI depth.
pub fn card_model_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    active_card_model: Res<ActiveCardModel>,
    mut card_ui_state: ResMut<CardUiState>,
    mut persistent_settings: Option<ResMut<Persistent<CardSettingsStore>>>,
    flip_state: Res<CardFlipState>,
    mut scene: ViewChangeParams,
) {
    if !keys.just_pressed(KeyCode::KeyT) {
        return;
    }

    match *scene.active_view {
        ActiveView::GameView => {
            scene.active_world_model.toggle(&scene.world_model_registry);
            scene
                .active_locations
                .reroll(&scene.location_model_registry, &scene.active_world_model);
            scene.reload_active_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
        }
        ActiveView::DeckBuilderScene | ActiveView::DebugSettingsScene => {
            card_ui_state.depth_factor = next_card_ui_depth_factor(card_ui_state.depth_factor);
            if let Some(persistent_settings) = persistent_settings.as_deref_mut() {
                if let Err(error) =
                    persistent_settings.set(CardSettingsStore::from_state(&card_ui_state))
                {
                    warn!("Failed to save card settings: {error}");
                }
            }
            let initial_rotation =
                composed_rotation_for_face(&scene.card_state, flip_state.visible_face);
            scene.reload_active_view(
                &active_card_model,
                flip_state.visible_face,
                initial_rotation,
            );
        }
    }
}

fn next_card_ui_depth_factor(current: f32) -> f32 {
    if current < CARD_DEPTH_FACTOR_DEFAULT {
        CARD_DEPTH_FACTOR_DEFAULT
    } else if current < CARD_DEPTH_FACTOR_MAX {
        CARD_DEPTH_FACTOR_MAX
    } else {
        CARD_DEPTH_FACTOR_MIN
    }
}

#[cfg_attr(feature = "desktop-hot-reload", hot)]
#[derive(SystemParam)]
pub struct RestartAppSceneParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    active_card_model: Res<'w, ActiveCardModel>,
    player_deck_collection: Option<Res<'w, PlayerDeckCollectionModel>>,
    flip_state: ResMut<'w, CardFlipState>,
    ticks: ResMut<'w, GameTicks>,
    gesture_model: Option<ResMut<'w, CardGestureModel>>,
    game_deck_model: Option<ResMut<'w, GameDeckModel>>,
    card_states: Option<ResMut<'w, CardStateModel>>,
    scene: ViewChangeParams<'w, 's>,
}

#[cfg_attr(feature = "desktop-hot-reload", hot)]
pub fn restart_app_scene(params: RestartAppSceneParams) {
    let RestartAppSceneParams {
        keys,
        active_card_model,
        player_deck_collection,
        mut flip_state,
        mut ticks,
        mut gesture_model,
        mut game_deck_model,
        mut card_states,
        mut scene,
    } = params;

    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    reset_game_model(
        gesture_model.as_deref_mut(),
        scene.slot_board.as_deref_mut(),
        card_states.as_deref_mut(),
        game_deck_model.as_deref_mut(),
        scene.game_hand_model.as_deref_mut(),
        scene.game_round_model.as_deref_mut(),
        scene.game_location_model.as_deref_mut(),
        player_deck_collection.as_deref(),
    );
    *scene.active_view = ActiveView::GameView;
    scene.reload_app_scene_and_active_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
    *flip_state = CardFlipState::default();
    *scene.card_state = CardInspectionState::default();
    ticks.0 = 0;
}

fn reset_game_model(
    gesture_model: Option<&mut CardGestureModel>,
    slot_board: Option<&mut CardSlotBoardModel>,
    card_states: Option<&mut CardStateModel>,
    game_deck_model: Option<&mut GameDeckModel>,
    game_hand_model: Option<&mut GameHandModel>,
    game_round_model: Option<&mut GameRoundModel>,
    game_location_model: Option<&mut GameLocationModel>,
    player_deck_collection: Option<&PlayerDeckCollectionModel>,
) {
    if let Some(gesture_model) = gesture_model {
        *gesture_model = CardGestureModel::default();
    }
    if let Some(slot_board) = slot_board {
        *slot_board = CardSlotBoardModel::default();
    }
    if let Some(mut card_states) = card_states {
        *card_states = CardStateModel::default();
        if let (
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(game_location_model),
            Some(player_deck_collection),
        ) = (
            game_deck_model,
            game_hand_model,
            game_round_model,
            game_location_model,
            player_deck_collection,
        ) {
            initialize_game_models_for_player(
                player_deck_collection,
                game_deck_model,
                game_hand_model,
                game_round_model,
                game_location_model,
                &mut card_states,
            );
        }
    }
}

#[cfg(feature = "desktop-hot-reload")]
pub fn record_desktop_hot_reload_patch_message(mut patches: MessageReader<HotPatched>) {
    for _ in patches.read() {
        info!("Desktop hot reload patch applied");
        record_desktop_hot_reload_patch();
    }
}

#[cfg(not(feature = "desktop-hot-reload"))]
pub fn record_desktop_hot_reload_patch_message() {}

#[cfg(feature = "desktop-hot-reload")]
#[cfg_attr(feature = "desktop-hot-reload", hot)]
pub fn hot_reload_auto_restart_app_scene(
    mut last_seen_patch_count: Local<u64>,
    hud_state: Res<DebugHudState>,
    mut debug_drawing_model: ResMut<DebugDrawingModel>,
    active_card_model: Res<ActiveCardModel>,
    mut flip_state: ResMut<CardFlipState>,
    mut ticks: ResMut<GameTicks>,
    mut scene: ViewChangeParams,
) {
    let patch_count = desktop_hot_reload_patch_count();
    if patch_count == *last_seen_patch_count {
        return;
    }

    *last_seen_patch_count = patch_count;

    if !hud_state.is_hot_reload_autorestart_enabled {
        return;
    }

    let fallback_slot_board = CardSlotBoardModel::default();
    let slot_board = scene.slot_board.as_deref().unwrap_or(&fallback_slot_board);
    debug_drawing_model.request_reference_layout(slot_board);
    scene.reload_app_scene_and_active_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
    *flip_state = CardFlipState::default();
    *scene.card_state = CardInspectionState::default();
    ticks.0 = 0;
}

#[cfg(not(feature = "desktop-hot-reload"))]
pub fn hot_reload_auto_restart_app_scene() {}

#[derive(SystemParam)]
pub struct ViewChangeParams<'w, 's> {
    commands: Commands<'w, 's>,
    active_view: ResMut<'w, ActiveView>,
    app_scene_query: Query<'w, 's, Entity, With<AppSceneRoot>>,
    hud: Option<Res<'w, Hud>>,
    game_view_roots: Query<'w, 's, Entity, With<GameViewRoot>>,
    standalone_game_view_entities: Query<
        'w,
        's,
        Entity,
        (
            With<GameViewEntity>,
            Without<GameViewRoot>,
            Without<ChildOf>,
        ),
    >,
    standalone_deck_builder_scene_entities: Query<
        'w,
        's,
        Entity,
        (
            With<DeckBuilderSceneEntity>,
            Without<DeckBuilderSceneRoot>,
            Without<ChildOf>,
        ),
    >,
    standalone_debug_settings_scene_entities: Query<
        'w,
        's,
        Entity,
        (
            With<DebugSettingsSceneEntity>,
            Without<DebugSettingsSceneRoot>,
            Without<ChildOf>,
        ),
    >,
    deck_builder_scene_roots: Query<'w, 's, Entity, With<DeckBuilderSceneRoot>>,
    debug_settings_scene_roots: Query<'w, 's, Entity, With<DebugSettingsSceneRoot>>,
    primary_window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    deck_builder_camera_query: Query<
        'w,
        's,
        (&'static Camera, &'static GlobalTransform),
        (
            With<PrimaryViewCamera>,
            With<DeckBuilderSceneEntity>,
            With<Camera3d>,
        ),
    >,
    deck_builder_card_query:
        Query<'w, 's, &'static GlobalTransform, (With<CardView>, With<DeckBuilderSceneEntity>)>,
    debug_settings_camera_query: Query<
        'w,
        's,
        (&'static Camera, &'static GlobalTransform),
        (
            With<PrimaryViewCamera>,
            With<DebugSettingsSceneEntity>,
            With<Camera3d>,
        ),
    >,
    debug_settings_card_query:
        Query<'w, 's, &'static GlobalTransform, (With<CardView>, With<DebugSettingsSceneEntity>)>,
    mouse_buttons: Res<'w, ButtonInput<MouseButton>>,
    touches: Res<'w, Touches>,
    asset_server: Res<'w, AssetServer>,
    camera_defaults: Res<'w, PrimaryCameraDefaults>,
    card_defaults: Res<'w, CardInspectionDefaults>,
    card_model_registry: Res<'w, CardModelRegistry>,
    slot_board: Option<ResMut<'w, CardSlotBoardModel>>,
    world_model_registry: Res<'w, WorldModelRegistry>,
    active_world_model: ResMut<'w, ActiveWorldModel>,
    location_model_registry: Res<'w, LocationModelRegistry>,
    active_locations: ResMut<'w, ActiveLocations>,
    player_deck_collection: Option<Res<'w, PlayerDeckCollectionModel>>,
    game_hand_model: Option<ResMut<'w, GameHandModel>>,
    game_round_model: Option<ResMut<'w, GameRoundModel>>,
    game_location_model: Option<ResMut<'w, GameLocationModel>>,
    card_state: ResMut<'w, CardInspectionState>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<'w, Assets<CardBackgroundMaskMaterial>>>,
}

impl ViewChangeParams<'_, '_> {
    fn despawn_game_view(&mut self) {
        for entity in self.game_view_roots.iter() {
            self.commands.entity(entity).despawn();
        }
        for entity in self.standalone_game_view_entities.iter() {
            self.commands.entity(entity).despawn();
        }
        for entity in self.standalone_deck_builder_scene_entities.iter() {
            self.commands.entity(entity).despawn();
        }
        for entity in self.standalone_debug_settings_scene_entities.iter() {
            self.commands.entity(entity).despawn();
        }
    }

    fn despawn_deck_builder_scene(&mut self) {
        for entity in self.deck_builder_scene_roots.iter() {
            self.commands.entity(entity).despawn();
        }
    }

    fn despawn_debug_settings_scene(&mut self) {
        for entity in self.debug_settings_scene_roots.iter() {
            self.commands.entity(entity).despawn();
        }
    }

    fn spawn_game_view(&mut self, active_card_model: &ActiveCardModel) {
        let fallback_slot_board = CardSlotBoardModel::default();
        let slot_board = self.slot_board.as_deref().unwrap_or(&fallback_slot_board);
        let fallback_hand = GameHandModel::default();
        let game_hand_model = self.game_hand_model.as_deref().unwrap_or(&fallback_hand);
        let fallback_round = GameRoundModel::default();
        let game_round_model = self.game_round_model.as_deref().unwrap_or(&fallback_round);
        let fallback_locations = GameLocationModel::default();
        let game_location_model = self
            .game_location_model
            .as_deref()
            .unwrap_or(&fallback_locations);
        spawn_game_view_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            self.hud.as_ref().map(|hud| hud.0),
            &self.asset_server,
            &self.camera_defaults,
            &self.card_defaults,
            &self.card_model_registry,
            game_hand_model.cards.as_slice(),
            game_round_model,
            game_location_model,
            slot_board,
            active_card_model,
            &self.world_model_registry,
            &self.active_world_model,
            &self.location_model_registry,
            &self.active_locations,
            &mut self.meshes,
            &mut self.materials,
            self.masked_background_materials.as_deref_mut(),
        );
    }

    #[allow(dead_code)]
    fn spawn_deck_builder_scene(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        let _ = (active_card_model, visible_face, initial_rotation);
        let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
        let player_deck_collection = self
            .player_deck_collection
            .as_deref()
            .unwrap_or(&fallback_player_deck_collection);
        spawn_deck_builder_scene_contents(
            &mut self.commands,
            &self.asset_server,
            &self.camera_defaults,
            &self.card_defaults,
            &self.card_model_registry,
            player_deck_collection,
            &mut self.meshes,
            &mut self.materials,
            self.masked_background_materials.as_deref_mut(),
            self.app_scene_query.single().ok(),
            CardFace::Front,
            Quat::IDENTITY,
        );
    }

    fn spawn_debug_settings_scene(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        spawn_debug_settings_scene_contents(
            &mut self.commands,
            &self.asset_server,
            &self.camera_defaults,
            &self.card_defaults,
            &self.card_model_registry,
            active_card_model,
            &mut self.meshes,
            &mut self.materials,
            self.masked_background_materials.as_deref_mut(),
            self.app_scene_query.single().ok(),
            visible_face,
            initial_rotation,
        );
    }

    fn reload_active_view(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        match *self.active_view {
            ActiveView::GameView => {
                self.despawn_game_view();
                let fallback_slot_board = CardSlotBoardModel::default();
                let slot_board = self.slot_board.as_deref().unwrap_or(&fallback_slot_board);
                let fallback_hand = GameHandModel::default();
                let game_hand_model = self.game_hand_model.as_deref().unwrap_or(&fallback_hand);
                let fallback_round = GameRoundModel::default();
                let game_round_model = self.game_round_model.as_deref().unwrap_or(&fallback_round);
                let fallback_locations = GameLocationModel::default();
                let game_location_model = self
                    .game_location_model
                    .as_deref()
                    .unwrap_or(&fallback_locations);
                spawn_game_view_contents(
                    &mut self.commands,
                    self.app_scene_query.single().ok(),
                    self.hud.as_ref().map(|hud| hud.0),
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    game_hand_model.cards.as_slice(),
                    game_round_model,
                    game_location_model,
                    slot_board,
                    active_card_model,
                    &self.world_model_registry,
                    &self.active_world_model,
                    &self.location_model_registry,
                    &self.active_locations,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                );
            }
            ActiveView::DeckBuilderScene => {
                self.despawn_deck_builder_scene();
                let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
                let player_deck_collection = self
                    .player_deck_collection
                    .as_deref()
                    .unwrap_or(&fallback_player_deck_collection);
                spawn_deck_builder_scene_contents(
                    &mut self.commands,
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    player_deck_collection,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                    self.app_scene_query.single().ok(),
                    CardFace::Front,
                    Quat::IDENTITY,
                );
            }
            ActiveView::DebugSettingsScene => {
                self.despawn_debug_settings_scene();
                spawn_debug_settings_scene_contents(
                    &mut self.commands,
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    active_card_model,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                    self.app_scene_query.single().ok(),
                    visible_face,
                    initial_rotation,
                );
            }
        }
    }

    fn despawn_app_scene(&mut self) {
        self.despawn_game_view();
        self.despawn_deck_builder_scene();
        self.despawn_debug_settings_scene();
        for entity in self.app_scene_query.iter() {
            self.commands.entity(entity).despawn();
        }
    }

    fn reload_app_scene_and_active_view(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        self.despawn_app_scene();
        let app_scene =
            spawn_app_scene_contents(&mut self.commands, self.hud.as_ref().map(|hud| hud.0));
        match *self.active_view {
            ActiveView::GameView => {
                let fallback_slot_board = CardSlotBoardModel::default();
                let slot_board = self.slot_board.as_deref().unwrap_or(&fallback_slot_board);
                let fallback_hand = GameHandModel::default();
                let game_hand_model = self.game_hand_model.as_deref().unwrap_or(&fallback_hand);
                let fallback_round = GameRoundModel::default();
                let game_round_model = self.game_round_model.as_deref().unwrap_or(&fallback_round);
                let fallback_locations = GameLocationModel::default();
                let game_location_model = self
                    .game_location_model
                    .as_deref()
                    .unwrap_or(&fallback_locations);
                spawn_game_view_contents(
                    &mut self.commands,
                    Some(app_scene),
                    self.hud.as_ref().map(|hud| hud.0),
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    game_hand_model.cards.as_slice(),
                    game_round_model,
                    game_location_model,
                    slot_board,
                    active_card_model,
                    &self.world_model_registry,
                    &self.active_world_model,
                    &self.location_model_registry,
                    &self.active_locations,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                );
            }
            ActiveView::DeckBuilderScene => {
                let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
                let player_deck_collection = self
                    .player_deck_collection
                    .as_deref()
                    .unwrap_or(&fallback_player_deck_collection);
                spawn_deck_builder_scene_contents(
                    &mut self.commands,
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    player_deck_collection,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                    Some(app_scene),
                    CardFace::Front,
                    Quat::IDENTITY,
                );
            }
            ActiveView::DebugSettingsScene => {
                spawn_debug_settings_scene_contents(
                    &mut self.commands,
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    active_card_model,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                    Some(app_scene),
                    visible_face,
                    initial_rotation,
                );
            }
        }
    }
}

/// HUMAN: Handles the S-key debug shortcut that cycles active scenes.
/// AI: Keep it non-toggle and wrap through GameView, DeckBuilderScene, and DebugSettingsScene.
pub fn scene_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    active_card_model: Res<ActiveCardModel>,
    flip_state: Res<CardFlipState>,
    mut params: ViewChangeParams,
) {
    if !keys.just_pressed(KeyCode::KeyS) {
        return;
    }

    match *params.active_view {
        ActiveView::GameView => {
            let initial_rotation =
                composed_rotation_for_face(&params.card_state, flip_state.visible_face);
            params.despawn_game_view();
            params.spawn_deck_builder_scene(
                &active_card_model,
                flip_state.visible_face,
                initial_rotation,
            );
            *params.active_view = ActiveView::DeckBuilderScene;
        }
        ActiveView::DeckBuilderScene => {
            params.despawn_deck_builder_scene();
            let initial_rotation =
                composed_rotation_for_face(&params.card_state, flip_state.visible_face);
            params.spawn_debug_settings_scene(
                &active_card_model,
                flip_state.visible_face,
                initial_rotation,
            );
            *params.active_view = ActiveView::DebugSettingsScene;
        }
        ActiveView::DebugSettingsScene => {
            params.despawn_debug_settings_scene();
            params.spawn_game_view(&active_card_model);
            *params.active_view = ActiveView::GameView;
        }
    }
}

/// HUMAN: Handles pointer navigation from scene card inspection back to GameView.
/// AI: Keep pointer return behavior separate from the S-key scene cycle shortcut.
pub fn view_input_system(
    mut params: ViewChangeParams,
    mut active_card_model: ResMut<ActiveCardModel>,
    mut flip_state: ResMut<CardFlipState>,
) {
    let Ok(primary_window) = params.primary_window_query.single() else {
        return;
    };
    let Some(pointer_position) =
        just_pressed_pointer_position(primary_window, &params.mouse_buttons, &params.touches)
    else {
        return;
    };

    match *params.active_view {
        ActiveView::GameView => {
            let _ = pointer_position;
            let _ = (&mut active_card_model, &mut flip_state);
        }
        ActiveView::DeckBuilderScene => {
            if !is_deck_builder_card_hit(
                pointer_position,
                params.deck_builder_camera_query.single().ok(),
                params.deck_builder_card_query.single().ok(),
                &params.card_defaults,
            ) {
                return;
            }

            params.despawn_deck_builder_scene();
            params.spawn_game_view(&active_card_model);
            *params.active_view = ActiveView::GameView;
        }
        ActiveView::DebugSettingsScene => {
            if !is_deck_builder_card_hit(
                pointer_position,
                params.debug_settings_camera_query.single().ok(),
                params.debug_settings_card_query.single().ok(),
                &params.card_defaults,
            ) {
                return;
            }

            params.despawn_debug_settings_scene();
            params.spawn_game_view(&active_card_model);
            *params.active_view = ActiveView::GameView;
        }
    }
}

fn card_click_navigation(
    _click: On<Pointer<Click>>,
    mut params: ViewChangeParams,
    active_card_model: Res<ActiveCardModel>,
) {
    match *params.active_view {
        ActiveView::GameView => {}
        ActiveView::DeckBuilderScene => {
            params.despawn_deck_builder_scene();
            params.spawn_game_view(&active_card_model);
            *params.active_view = ActiveView::GameView;
        }
        ActiveView::DebugSettingsScene => {
            params.despawn_debug_settings_scene();
            params.spawn_game_view(&active_card_model);
            *params.active_view = ActiveView::GameView;
        }
    }
}

fn active_pointer_position(primary_window: &Window, touches: &Touches) -> Option<Vec2> {
    touches
        .iter()
        .next()
        .map(|touch| touch.position())
        .or_else(|| primary_window.cursor_position())
}

fn just_pressed_pointer_position(
    primary_window: &Window,
    mouse_buttons: &ButtonInput<MouseButton>,
    touches: &Touches,
) -> Option<Vec2> {
    touches
        .iter_just_pressed()
        .next()
        .map(|touch| touch.position())
        .or_else(|| {
            mouse_buttons
                .just_pressed(MouseButton::Left)
                .then(|| primary_window.cursor_position())
                .flatten()
        })
}

#[cfg(test)]
fn is_game_view_card_hit(pointer_position: Vec2, window_size: Vec2) -> bool {
    game_view_card_index_at(pointer_position, window_size).is_some()
}

#[cfg(test)]
fn game_view_card_index_at(pointer_position: Vec2, window_size: Vec2) -> Option<usize> {
    game_view_card_index_at_for_count(pointer_position, window_size, STARTING_HAND_CARD_COUNT)
}

fn game_view_card_index_at_for_count(
    pointer_position: Vec2,
    window_size: Vec2,
    card_count: usize,
) -> Option<usize> {
    if window_size.x <= 0.0 || window_size.y <= 0.0 {
        return None;
    }

    let Some(pointer_position) = window_pointer_to_game_view(pointer_position, window_size) else {
        return None;
    };
    game_view_card_hitboxes_for_count(card_count)
        .iter()
        .rposition(|(min, max)| {
            pointer_position.x >= min.x
                && pointer_position.x <= max.x
                && pointer_position.y >= min.y
                && pointer_position.y <= max.y
        })
}

fn is_deck_builder_card_hit(
    pointer_position: Vec2,
    camera: Option<(&Camera, &GlobalTransform)>,
    card_transform: Option<&GlobalTransform>,
    card_defaults: &CardInspectionDefaults,
) -> bool {
    let (camera, camera_transform) = match camera {
        Some(camera) => camera,
        None => return false,
    };
    let Some(card_transform) = card_transform else {
        return false;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, pointer_position) else {
        return false;
    };
    let card_world_transform = card_transform.compute_transform();
    let card_normal = card_world_transform.rotation * Vec3::Z;
    let Some(hit_point) = ray.plane_intersection_point(
        card_world_transform.translation,
        InfinitePlane3d::new(card_normal),
    ) else {
        return false;
    };
    let local_hit_point = card_transform
        .affine()
        .inverse()
        .transform_point3(hit_point);

    local_hit_point.x.abs() <= card_defaults.width * 0.5
        && local_hit_point.y.abs() <= card_defaults.height * 0.5
}

fn window_pointer_to_game_view(pointer_position: Vec2, window_size: Vec2) -> Option<Vec2> {
    let game_view_size = Vec2::new(GAME_VIEW_WIDTH, GAME_VIEW_HEIGHT);
    let scale = (window_size.x / game_view_size.x).min(window_size.y / game_view_size.y);
    if scale <= 0.0 {
        return None;
    }

    let scaled_game_view_size = game_view_size * scale;
    let offset = (window_size - scaled_game_view_size) * 0.5;
    let pointer_position = (pointer_position - offset) / scale;

    (pointer_position.x >= 0.0
        && pointer_position.x <= GAME_VIEW_WIDTH
        && pointer_position.y >= 0.0
        && pointer_position.y <= GAME_VIEW_HEIGHT)
        .then_some(pointer_position)
}

#[cfg(test)]
fn game_view_pointer_to_window(pointer_position: Vec2, window_size: Vec2) -> Vec2 {
    let game_view_size = Vec2::new(GAME_VIEW_WIDTH, GAME_VIEW_HEIGHT);
    let scale = (window_size.x / game_view_size.x).min(window_size.y / game_view_size.y);
    let scaled_game_view_size = game_view_size * scale;
    let offset = (window_size - scaled_game_view_size) * 0.5;

    offset + (pointer_position * scale)
}

#[cfg(test)]
fn game_view_card_hitboxes() -> Vec<(Vec2, Vec2)> {
    game_view_card_hitboxes_for_count(STARTING_HAND_CARD_COUNT)
}

// HUMAN: Builds hand-row hitboxes directly from hand area geometry for stable gestures.
// AI: Uses the hand-area card size for row centering and spacing behavior.
fn game_view_card_hitboxes_for_count(card_count: usize) -> Vec<(Vec2, Vec2)> {
    game_view_card_hitboxes_for_count_with_hover(card_count, None)
}

fn game_view_card_hitboxes_for_count_with_hover(
    card_count: usize,
    hovered_index: Option<usize>,
) -> Vec<(Vec2, Vec2)> {
    if card_count == 0 {
        return Vec::new();
    }

    let hand_min = game_view_hand_area_min();
    let hand_size = game_view_hand_area_size();
    let card_size = game_view_hand_card_size();
    let row_height = card_size.y;
    let centers = game_view_hand_card_centers(card_count, hovered_index, hand_min, hand_size);
    let row_min_y = hand_min.y + ((hand_size.y - row_height) * 0.5).max(0.0);

    centers
        .into_iter()
        .map(|center_x| {
            let card_min = Vec2::new(center_x - (card_size.x * 0.5), row_min_y);
            (card_min, card_min + card_size.min(hand_size))
        })
        .collect()
}

fn game_view_hand_card_centers(
    card_count: usize,
    hovered_index: Option<usize>,
    hand_min: Vec2,
    hand_size: Vec2,
) -> Vec<f32> {
    let card_width = game_view_hand_card_size().x.min(hand_size.x);
    let min_center_x = hand_min.x + (card_width * 0.5);
    let max_center_x = hand_min.x + hand_size.x - (card_width * 0.5);
    if card_count == 1 || min_center_x >= max_center_x {
        return vec![(min_center_x + max_center_x) * 0.5; card_count];
    }

    let relaxed_step = card_width + GAME_SCENE_HAND_CARD_GAP;
    let fitted_step =
        ((max_center_x - min_center_x) / card_count.saturating_sub(1) as f32).min(relaxed_step);
    let row_width = fitted_step * card_count.saturating_sub(1) as f32;
    let row_start = (min_center_x + ((max_center_x - min_center_x - row_width) * 0.5))
        .clamp(min_center_x, max_center_x);
    let mut centers: Vec<f32> = (0..card_count)
        .map(|index| row_start + (index as f32 * fitted_step))
        .collect();

    let Some(hovered_index) = hovered_index.filter(|index| *index < card_count) else {
        return centers;
    };

    let has_left = hovered_index > 0;
    let has_right = hovered_index + 1 < card_count;
    let hover_min_center = if has_left {
        min_center_x + card_width
    } else {
        min_center_x
    };
    let hover_max_center = if has_right {
        max_center_x - card_width
    } else {
        max_center_x
    };
    if hover_min_center > hover_max_center {
        return centers;
    }
    centers[hovered_index] = centers[hovered_index].clamp(hover_min_center, hover_max_center);
    let hovered_center = centers[hovered_index];

    if has_left {
        let left_max_center = hovered_center - card_width;
        let left_step = if hovered_index > 1 {
            ((left_max_center - min_center_x) / hovered_index as f32).min(fitted_step)
        } else {
            0.0
        };
        for (index, center) in centers.iter_mut().enumerate().take(hovered_index) {
            *center = (left_max_center - ((hovered_index - index - 1) as f32 * left_step))
                .clamp(min_center_x, left_max_center);
        }
    }

    if has_right {
        let right_min_center = hovered_center + card_width;
        let right_count = card_count - hovered_index - 1;
        let right_step = if right_count > 1 {
            ((max_center_x - right_min_center) / (right_count - 1) as f32).min(fitted_step)
        } else {
            0.0
        };
        for (offset, center) in centers.iter_mut().skip(hovered_index + 1).enumerate() {
            *center = (right_min_center + (offset as f32 * right_step))
                .clamp(right_min_center, max_center_x);
        }
    }

    centers
}

fn game_view_hand_card_z(card_index: usize, hovered_index: Option<usize>) -> f32 {
    if hovered_index == Some(card_index) {
        GAME_SCENE_HAND_CARD_HOVER_Z
    } else {
        GAME_SCENE_HAND_CARD_WORLD_Z + (card_index as f32 * GAME_SCENE_HAND_CARD_Z_STEP)
    }
}

pub fn update_end_turn_button(
    mut button_query: Query<
        (
            &Interaction,
            &GameControlButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        Changed<Interaction>,
    >,
    mut game_deck_model: Option<ResMut<GameDeckModel>>,
    mut game_hand_model: Option<ResMut<GameHandModel>>,
    mut game_round_model: Option<ResMut<GameRoundModel>>,
    mut game_location_model: Option<ResMut<GameLocationModel>>,
    mut slot_board: Option<ResMut<CardSlotBoardModel>>,
    mut card_states: Option<ResMut<CardStateModel>>,
    mut gesture_model: Option<ResMut<CardGestureModel>>,
) {
    if card_gesture_blocks_game_controls(gesture_model.as_deref()) {
        return;
    }

    for (interaction, control, mut background, mut border) in &mut button_query {
        let (background_color, border_color) = match *interaction {
            Interaction::Pressed => {
                match control.action {
                    GameControlAction::EndTurn => {
                        if let (
                            Some(game_deck_model),
                            Some(game_hand_model),
                            Some(game_round_model),
                            Some(game_location_model),
                            Some(card_states),
                        ) = (
                            game_deck_model.as_deref_mut(),
                            game_hand_model.as_deref_mut(),
                            game_round_model.as_deref_mut(),
                            game_location_model.as_deref_mut(),
                            card_states.as_deref_mut(),
                        ) {
                            if !game_round_model.can_end_turn() {
                                return;
                            }
                            card_states.lock_location_cards();
                            if game_round_model.advance_round() {
                                game_location_model.set_round(game_round_model.round);
                                game_deck_model.deal_to_hand(
                                    game_round_model.requested_cards_to_deal,
                                    game_hand_model,
                                );
                                card_states.ensure_size(game_hand_model.len());
                            }
                        }
                    }
                    GameControlAction::Restart => {
                        if let (
                            Some(game_deck_model),
                            Some(game_hand_model),
                            Some(game_round_model),
                            Some(game_location_model),
                            Some(slot_board),
                            Some(card_states),
                            Some(gesture_model),
                        ) = (
                            game_deck_model.as_deref_mut(),
                            game_hand_model.as_deref_mut(),
                            game_round_model.as_deref_mut(),
                            game_location_model.as_deref_mut(),
                            slot_board.as_deref_mut(),
                            card_states.as_deref_mut(),
                            gesture_model.as_deref_mut(),
                        ) {
                            game_deck_model.reset_randomized();
                            game_hand_model.cards.clear();
                            game_round_model.reset();
                            game_location_model.reset();
                            *slot_board = CardSlotBoardModel::default();
                            *gesture_model = CardGestureModel::default();
                            game_deck_model.deal_to_hand(
                                game_round_model.requested_cards_to_deal,
                                game_hand_model,
                            );
                            card_states.reset_to_size(game_hand_model.len());
                        }
                    }
                    GameControlAction::Undo => {
                        if let (
                            Some(game_round_model),
                            Some(slot_board),
                            Some(card_states),
                            Some(gesture_model),
                        ) = (
                            game_round_model.as_deref_mut(),
                            slot_board.as_deref_mut(),
                            card_states.as_deref_mut(),
                            gesture_model.as_deref_mut(),
                        ) && game_round_model.has_undoable_moves()
                        {
                            let moves: Vec<_> =
                                game_round_model.current_round_moves.drain(..).collect();
                            for record in moves {
                                slot_board.remove_local_card(record.hand_index);
                                card_states.return_to_hand(record.hand_index);
                                game_round_model.restore(record.energy_cost);
                            }
                            *gesture_model = CardGestureModel::default();
                        }
                    }
                }
                (
                    END_TURN_BUTTON_PRESSED_COLOR,
                    END_TURN_BUTTON_PRESSED_BORDER_COLOR,
                )
            }
            Interaction::Hovered => (
                END_TURN_BUTTON_HOVER_COLOR,
                END_TURN_BUTTON_HOVER_BORDER_COLOR,
            ),
            Interaction::None => (
                END_TURN_BUTTON_NORMAL_COLOR,
                END_TURN_BUTTON_NORMAL_BORDER_COLOR,
            ),
        };
        let is_disabled =
            game_control_action_is_disabled(control.action, game_round_model.as_deref());
        if is_disabled {
            background.0 = GAME_CONTROL_DISABLED_COLOR;
            *border = BorderColor::all(GAME_CONTROL_DISABLED_BORDER_COLOR);
        } else {
            background.0 = background_color;
            *border = BorderColor::all(border_color);
        }
    }
}

fn card_gesture_blocks_game_controls(gesture_model: Option<&CardGestureModel>) -> bool {
    gesture_model.is_some_and(|gesture_model| {
        !matches!(
            gesture_model.state,
            CardGestureState::Idle | CardGestureState::Placed
        )
    })
}

/// HUMAN: Keeps GameView control labels and disabled states synced to gameplay state.
/// AI: Run this separately from button interactions so round, energy, and undo text stay live.
pub fn update_game_control_ui_system(
    game_round_model: Option<Res<GameRoundModel>>,
    mut label_query: Query<(&GameControlLabel, &mut Text)>,
    mut button_query: Query<(
        &Interaction,
        &GameControlButton,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    let Some(game_round_model) = game_round_model.as_deref() else {
        return;
    };

    for (label, mut text) in &mut label_query {
        match label.action {
            GameControlAction::Undo => {
                text.0 = game_round_model.energy_label();
            }
            GameControlAction::EndTurn => {
                text.0 = format!(
                    "Turn {}/{}",
                    game_round_model.round, game_round_model.max_rounds
                );
            }
            GameControlAction::Restart => {}
        }
    }

    for (interaction, control, mut background, mut border) in &mut button_query {
        if game_control_action_is_disabled(control.action, Some(game_round_model)) {
            background.0 = GAME_CONTROL_DISABLED_COLOR;
            *border = BorderColor::all(GAME_CONTROL_DISABLED_BORDER_COLOR);
            continue;
        }

        let (background_color, border_color) = match *interaction {
            Interaction::Pressed => (
                END_TURN_BUTTON_PRESSED_COLOR,
                END_TURN_BUTTON_PRESSED_BORDER_COLOR,
            ),
            Interaction::Hovered => (
                END_TURN_BUTTON_HOVER_COLOR,
                END_TURN_BUTTON_HOVER_BORDER_COLOR,
            ),
            Interaction::None => (
                END_TURN_BUTTON_NORMAL_COLOR,
                END_TURN_BUTTON_NORMAL_BORDER_COLOR,
            ),
        };
        background.0 = background_color;
        *border = BorderColor::all(border_color);
    }
}

fn game_control_action_is_disabled(
    action: GameControlAction,
    game_round_model: Option<&GameRoundModel>,
) -> bool {
    match action {
        GameControlAction::Restart => false,
        GameControlAction::Undo => {
            game_round_model.is_some_and(|model| !model.has_undoable_moves())
        }
        GameControlAction::EndTurn => game_round_model.is_some_and(|model| !model.can_end_turn()),
    }
}

pub fn update_card_target_from_pointer(
    pointer_position: Vec2,
    window_size: Vec2,
    card_defaults: &CardInspectionDefaults,
    card_state: &mut CardInspectionState,
) {
    if window_size.x <= 0.0 || window_size.y <= 0.0 {
        return;
    }

    let normalized = Vec2::new(
        (pointer_position.x / window_size.x) * 2.0 - 1.0,
        (pointer_position.y / window_size.y) * 2.0 - 1.0,
    )
    .clamp(Vec2::splat(-1.0), Vec2::splat(1.0));

    card_state.last_pointer_normalized = normalized;
    card_state.target_rotation = target_rotation_for_pointer(normalized, card_defaults);
}

pub fn target_rotation_for_pointer(
    pointer_normalized: Vec2,
    card_defaults: &CardInspectionDefaults,
) -> Quat {
    let clamped = pointer_normalized.clamp(Vec2::splat(-1.0), Vec2::splat(1.0));
    let yaw = clamped.x * card_defaults.max_tilt_radians;
    let pitch = clamped.y * card_defaults.max_tilt_radians;

    Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0)
}

pub fn load_saved_window_placement(
    mut placement_state: ResMut<WindowPlacementState>,
    persistent_placement: Option<Res<Persistent<WindowPlacementStore>>>,
) {
    placement_state.current = persistent_placement
        .and_then(|persistent_placement| {
            valid_window_placement(persistent_placement.current.clone())
        })
        .or_else(load_window_placement);
}

pub fn load_saved_debug_hud_input(
    mut hud_state: ResMut<DebugHudState>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    persistent_input: Option<Res<Persistent<DebugHudInputStore>>>,
) {
    if let Some(persistent_input) = persistent_input {
        persistent_input.apply_to_state(&mut hud_state);
    }

    if let Ok(mut window) = primary_window_query.single_mut() {
        apply_fullscreen_mode(&mut window, hud_state.is_fullscreen);
    }
}

pub fn load_saved_card_settings(
    mut card_ui_state: ResMut<CardUiState>,
    persistent_settings: Option<Res<Persistent<CardSettingsStore>>>,
) {
    if let Some(settings) = persistent_settings {
        settings.apply_to_state(&mut card_ui_state);
    }
}

pub fn load_saved_player_deck_collection(
    mut player_deck_collection: ResMut<PlayerDeckCollectionModel>,
    persistent_player_decks: Option<Res<Persistent<PlayerDeckCollectionModel>>>,
) {
    let mut loaded_model = if let Some(persistent_player_decks) = persistent_player_decks.as_ref() {
        (**persistent_player_decks).clone()
    } else {
        player_deck_collection.as_ref().clone()
    };
    loaded_model = ensure_player_deck_collection_model(loaded_model);
    *player_deck_collection = loaded_model;
}

pub fn initialize_game_models(
    player_deck_collection: Res<PlayerDeckCollectionModel>,
    mut game_deck_model: ResMut<GameDeckModel>,
    mut game_hand_model: ResMut<GameHandModel>,
    mut game_round_model: ResMut<GameRoundModel>,
    mut game_location_model: ResMut<GameLocationModel>,
    mut card_states: ResMut<CardStateModel>,
) {
    initialize_game_models_for_player(
        &player_deck_collection,
        &mut game_deck_model,
        &mut game_hand_model,
        &mut game_round_model,
        &mut game_location_model,
        &mut card_states,
    );
}

pub fn advance_ticks(mut ticks: ResMut<GameTicks>) {
    ticks.0 += 1;
}

pub fn setup_inspector(mut commands: Commands, hud_state: Res<DebugHudState>) {
    commands.spawn((
        Name::new("Bevy Inspector"),
        InspectorState {
            is_visible: hud_state.is_inspector_visible,
            ..Default::default()
        },
    ));
}

pub fn setup_debug_hud(mut commands: Commands) {
    spawn_debug_hud(&mut commands);
}

fn spawn_debug_hud(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Text::new("Scene: GameView\nFrame: 0\nKEYS: "),
            TextFont {
                font_size: DEBUG_HUD_FONT_SIZE,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SCREEN_PADDING_LEFT),
                top: Val::Px(SCREEN_PADDING_TOP),
                width: Val::Px(273.0),
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Px(40.0),
                    right: Val::Px(12.0),
                    top: Val::Px(8.0),
                    bottom: Val::Px(8.0),
                },
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..Default::default()
            },
            GlobalZIndex(DEBUG_HUD_Z_INDEX),
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
            AppSceneEntity,
            DebugHudText,
        ))
        .with_children(|parent| {
            spawn_key_span(parent, "R", KeyCode::KeyR, false);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "S", KeyCode::KeyS, false);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "T", KeyCode::KeyT, false);
            parent.spawn((TextSpan::new("\nKEYS: "), debug_hud_text_font()));
            spawn_key_span(parent, "D", KeyCode::KeyD, true);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "F", KeyCode::KeyF, true);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "P", KeyCode::KeyP, true);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "I", KeyCode::KeyI, true);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "H", KeyCode::KeyH, true);
            parent.spawn((TextSpan::new(""), debug_hud_text_font(), DebugHudFpsText));
        })
        .id()
}

#[derive(SystemParam)]
pub struct DebugHudUpdateParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    ticks: Res<'w, GameTicks>,
    active_view: Res<'w, ActiveView>,
    hud_state: ResMut<'w, DebugHudState>,
    inspector_query: Query<'w, 's, &'static InspectorState>,
    text_query: Query<'w, 's, &'static mut Text, With<DebugHudText>>,
    fps_text_query: Query<'w, 's, &'static mut TextSpan, With<DebugHudFpsText>>,
    key_text_query: Query<'w, 's, (&'static DebugHudKeyText, &'static mut UnderlineColor)>,
}

pub fn update_debug_hud(mut params: DebugHudUpdateParams) {
    params.hud_state.fps_accumulated_seconds += params.time.delta_secs();
    params.hud_state.fps_accumulated_frames += 1;

    if params.hud_state.fps_accumulated_seconds >= FPS_UPDATE_INTERVAL_SECONDS {
        params.hud_state.fps_display_value = if params.hud_state.fps_accumulated_seconds > 0.0 {
            params.hud_state.fps_accumulated_frames as f32
                / params.hud_state.fps_accumulated_seconds
        } else {
            0.0
        };

        params.hud_state.fps_accumulated_seconds = 0.0;
        params.hud_state.fps_accumulated_frames = 0;
    }

    let fps_on = params.hud_state.is_fps_visible;
    let inspector_on = params
        .inspector_query
        .single()
        .map(|inspector| inspector.is_visible)
        .unwrap_or(false);

    for (key_text, mut underline_color) in &mut params.key_text_query {
        let is_active = if key_text.is_toggle {
            match key_text.key_code {
                KeyCode::KeyD => params.hud_state.is_debug_drawing_visible,
                KeyCode::KeyF => params.hud_state.is_fullscreen,
                KeyCode::KeyP => fps_on,
                KeyCode::KeyI => inspector_on,
                KeyCode::KeyH => params.hud_state.is_hot_reload_autorestart_enabled,
                _ => false,
            }
        } else {
            params.keys.pressed(key_text.key_code)
        };

        underline_color.0 = if is_active {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.0)
        };
    }

    let scene_name = match *params.active_view {
        ActiveView::GameView => "GameView",
        ActiveView::DeckBuilderScene => "DeckBuilderScene",
        ActiveView::DebugSettingsScene => "DebugSettingsScene",
    };
    let full_text = format!("Scene: {scene_name}\nFrame: {}\nKEYS: ", params.ticks.0);
    for mut text in &mut params.text_query {
        *text = Text::new(full_text.clone());
    }

    let fps_line = if params.hud_state.is_fps_visible {
        format!("\nFPS: {:.1}", params.hud_state.fps_display_value)
    } else {
        String::new()
    };

    for mut fps_text in &mut params.fps_text_query {
        *fps_text = TextSpan::new(fps_line.clone());
    }
}

pub fn toggle_debug_hud_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut hud_state: ResMut<DebugHudState>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    monitor_entity_query: Query<(Entity, &Monitor)>,
    mut fullscreen_viewport_transition: Option<ResMut<FullscreenViewportTransitionState>>,
    mut placement_state: Option<ResMut<WindowPlacementState>>,
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
    mut persistent_placement: Option<ResMut<Persistent<WindowPlacementStore>>>,
) {
    let mut changed = false;
    let mut placement_to_save = None;

    if keys.just_pressed(KeyCode::KeyF) {
        hud_state.is_fullscreen = !hud_state.is_fullscreen;
        start_fullscreen_viewport_transition(fullscreen_viewport_transition.as_deref_mut());
        apply_browser_fullscreen(hud_state.is_fullscreen);
        if let Ok(mut window) = primary_window_query.single_mut() {
            if hud_state.is_fullscreen {
                let fallback_placement = placement_state
                    .as_deref()
                    .and_then(|placement_state| placement_state.current.clone());
                if let Some(ref mut placement_state) = placement_state {
                    if let Some(placement) = current_windowed_placement(
                        &window,
                        fallback_placement.as_ref(),
                        &monitor_query,
                    ) {
                        placement_to_save = Some(placement.clone());
                        placement_state.current = Some(placement);
                    }
                }
                let monitor_selection = current_monitor_selection(
                    &window,
                    fallback_placement.as_ref(),
                    &monitor_entity_query,
                );
                apply_fullscreen_mode_on_monitor(&mut window, monitor_selection);
            } else {
                window.mode = WindowMode::Windowed;
                if let Some(ref placement_state) = placement_state {
                    restore_windowed_placement(&mut window, placement_state, &monitor_query);
                    placement_to_save = placement_state.current.clone();
                }
            }
        }
        changed = true;
    }

    if keys.just_pressed(KeyCode::KeyP) {
        hud_state.is_fps_visible = !hud_state.is_fps_visible;
        changed = true;
    }

    if keys.just_pressed(KeyCode::KeyD) {
        hud_state.is_debug_drawing_visible = !hud_state.is_debug_drawing_visible;
        changed = true;
    }

    if keys.just_pressed(KeyCode::KeyH) {
        hud_state.is_hot_reload_autorestart_enabled = !hud_state.is_hot_reload_autorestart_enabled;
        changed = true;
    }

    if !changed {
        return;
    }

    if let Some(ref mut persistent_input) = persistent_input {
        if let Err(error) = persistent_input.set(DebugHudInputStore::from_state(&hud_state)) {
            warn!("Failed to save DebugHUD input state: {error}");
        }
    }

    if let Some(placement) = placement_to_save
        && let Some(ref mut persistent_placement) = persistent_placement
        && let Err(error) = persistent_placement.set(WindowPlacementStore {
            current: Some(placement),
        })
    {
        warn!("Failed to save window placement: {error}");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn start_fullscreen_viewport_transition(
    fullscreen_viewport_transition: Option<&mut FullscreenViewportTransitionState>,
) {
    if let Some(transition) = fullscreen_viewport_transition {
        transition.frames_remaining = FULLSCREEN_VIEWPORT_TRANSITION_FRAMES;
    }
}

#[cfg(target_arch = "wasm32")]
fn start_fullscreen_viewport_transition(
    _fullscreen_viewport_transition: Option<&mut FullscreenViewportTransitionState>,
) {
}

/// HUMAN: Applies the project fullscreen preference to the primary window.
/// AI: Preserve the original native Bevy fullscreen path; browser uses the Fullscreen API separately.
fn apply_fullscreen_mode(window: &mut Window, is_fullscreen: bool) {
    window.mode = if is_fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
}

/// HUMAN: Fullscreens the app on the selected monitor.
/// AI: Keep monitor selection explicit for the native path that worked before browser support.
fn apply_fullscreen_mode_on_monitor(window: &mut Window, monitor_selection: MonitorSelection) {
    window.mode = WindowMode::BorderlessFullscreen(monitor_selection);
}

#[cfg(target_arch = "wasm32")]
fn apply_browser_fullscreen(is_fullscreen: bool) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        warn!("Browser fullscreen toggle skipped: document is unavailable");
        return;
    };

    if is_fullscreen {
        if document.fullscreen_element().is_some() {
            return;
        }

        let Some(element) = document.document_element() else {
            warn!("Browser fullscreen toggle skipped: document element is unavailable");
            return;
        };

        if let Err(error) = element.request_fullscreen() {
            warn!("Browser fullscreen request failed: {error:?}");
        }
    } else if document.fullscreen_element().is_some() {
        document.exit_fullscreen();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_browser_fullscreen(_is_fullscreen: bool) {}

/// HUMAN: Mirrors browser-level fullscreen exits back into DebugHUD state.
/// AI: Keep the F toggle usable after users leave browser fullscreen with Escape.
pub fn sync_browser_fullscreen_state_system(mut hud_state: ResMut<DebugHudState>) {
    sync_browser_fullscreen_state(&mut hud_state);
}

#[cfg(target_arch = "wasm32")]
fn sync_browser_fullscreen_state(hud_state: &mut DebugHudState) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        hud_state.is_fullscreen = document.fullscreen_element().is_some();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn sync_browser_fullscreen_state(_hud_state: &mut DebugHudState) {}

pub fn toggle_inspector(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector_query: Query<&mut InspectorState>,
    mut hud_state: ResMut<DebugHudState>,
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    let Ok(mut inspector) = inspector_query.single_mut() else {
        return;
    };

    inspector.is_visible = !inspector.is_visible;
    hud_state.is_inspector_visible = inspector.is_visible;

    if let Some(ref mut persistent_input) = persistent_input {
        if let Err(error) = persistent_input.set(DebugHudInputStore::from_state(&hud_state)) {
            warn!("Failed to save DebugHUD input state: {error}");
        }
    }
}

pub fn quit_app_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut close_requested_events: MessageWriter<WindowCloseRequested>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    mut app_exit_events: MessageWriter<AppExit>,
    hud_state: Option<Res<DebugHudState>>,
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    if let Some(hud_state) = hud_state
        && let Some(ref mut persistent_input) = persistent_input
        && let Err(error) = persistent_input.set(DebugHudInputStore::from_state(&hud_state))
    {
        warn!("Failed to save DebugHUD input state: {error}");
    }

    if let Ok(primary_window) = primary_window_query.single() {
        close_requested_events.write(WindowCloseRequested {
            window: primary_window,
        });
    } else {
        app_exit_events.write(AppExit::Success);
    }
}

pub fn scale_debug_hud(
    mut window_resized_events: Option<MessageReader<WindowResized>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };

    let mut primary_window_resized = false;
    for resized_event in window_resized_events.read() {
        if resized_event.window == primary_window_entity {
            primary_window_resized = true;
        }
    }

    if !primary_window_resized {
        return;
    }

    let _width_scale = primary_window.resolution.width() / TARGET_WIDTH;
    let _height_scale = primary_window.resolution.height() / TARGET_HEIGHT;
}

/// HUMAN: Restores saved windowed placement after monitor data becomes available.
/// AI: Do not apply windowed geometry while the saved launch state is fullscreen.
pub fn restore_window_placement_to_current_monitors(
    mut placement_state: ResMut<WindowPlacementState>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    monitor_entity_query: Query<(Entity, &Monitor)>,
    hud_state: Option<Res<DebugHudState>>,
) {
    if placement_state.restored {
        return;
    }
    if monitor_query.iter().next().is_none() {
        return;
    }
    if hud_state
        .as_deref()
        .is_some_and(|hud_state| hud_state.is_fullscreen)
    {
        if let Ok(mut window) = primary_window_query.single_mut() {
            let monitor_selection = placement_state
                .current
                .as_ref()
                .map(|placement| saved_monitor_selection(placement, &monitor_entity_query))
                .unwrap_or(MonitorSelection::Current);
            apply_fullscreen_mode_on_monitor(&mut window, monitor_selection);
        }
        placement_state.restored = true;
        return;
    }

    let Some(saved_placement) = placement_state.current.clone() else {
        placement_state.restored = true;
        return;
    };

    let Ok(mut window) = primary_window_query.single_mut() else {
        return;
    };

    if let Some(restored_position) = restored_position(&monitor_query, &saved_placement) {
        window.resolution =
            restored_window_resolution(&window.resolution, saved_placement.window_size);
        window.position = WindowPosition::At(restored_position);
    } else {
        apply_primary_centered_fallback(&mut window);
    }

    placement_state.restored = true;
}

pub fn track_window_placement(
    mut window_moved_events: Option<MessageReader<WindowMoved>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    mut placement_state: ResMut<WindowPlacementState>,
    hud_state: Option<Res<DebugHudState>>,
) {
    let Some(ref mut window_moved_events) = window_moved_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };
    if hud_state
        .as_deref()
        .is_some_and(|hud_state| hud_state.is_fullscreen)
    {
        return;
    }
    if !is_windowed(primary_window) {
        return;
    }

    for moved_event in window_moved_events.read() {
        if moved_event.window != primary_window_entity {
            continue;
        }

        placement_state.current = placement_for_window(
            moved_event.position,
            logical_window_size(primary_window),
            primary_window.resolution.physical_size(),
            &monitor_query,
        );
    }
}

pub fn track_window_size(
    mut window_resized_events: Option<MessageReader<WindowResized>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    mut placement_state: ResMut<WindowPlacementState>,
    hud_state: Option<Res<DebugHudState>>,
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };
    if hud_state
        .as_deref()
        .is_some_and(|hud_state| hud_state.is_fullscreen)
    {
        return;
    }
    if !is_windowed(primary_window) {
        return;
    }

    for resized_event in window_resized_events.read() {
        if resized_event.window != primary_window_entity {
            continue;
        }

        let window_position = placement_state
            .current
            .as_ref()
            .map(|placement| placement.window_position)
            .or_else(|| match primary_window.position {
                WindowPosition::At(position) => Some(position),
                WindowPosition::Automatic | WindowPosition::Centered(_) => None,
            });

        let Some(window_position) = window_position else {
            continue;
        };

        placement_state.current = placement_for_window(
            window_position,
            logical_size_from_resize(resized_event),
            primary_window.resolution.physical_size(),
            &monitor_query,
        );
    }
}

pub fn save_window_placement_on_close(
    mut close_requested_events: Option<MessageReader<WindowCloseRequested>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    placement_state: Res<WindowPlacementState>,
    mut persistent_placement: Option<ResMut<Persistent<WindowPlacementStore>>>,
    hud_state: Option<Res<DebugHudState>>,
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
) {
    let Some(ref mut close_requested_events) = close_requested_events else {
        return;
    };
    let Ok((primary_window_entity, window)) = primary_window_query.single() else {
        return;
    };

    let should_save = close_requested_events
        .read()
        .any(|event| event.window == primary_window_entity);

    if !should_save {
        return;
    }

    let is_fullscreen = hud_state
        .as_deref()
        .is_some_and(|hud_state| hud_state.is_fullscreen)
        || !is_windowed(window);

    if let Some(hud_state) = hud_state
        && let Some(ref mut persistent_input) = persistent_input
        && let Err(error) = persistent_input.set(DebugHudInputStore::from_state(&hud_state))
    {
        warn!("Failed to save DebugHUD input state: {error}");
    }

    let current_window_placement = if is_fullscreen {
        placement_state.current.clone()
    } else {
        current_windowed_placement(window, placement_state.current.as_ref(), &monitor_query)
    };

    let placement_with_current_size = if is_fullscreen {
        None
    } else {
        placement_state.current.as_ref().map(|placement| {
            placement_with_current_window_size(
                placement,
                logical_window_size(window),
                window.resolution.physical_size(),
                &monitor_query,
            )
        })
    };
    let placement = current_window_placement
        .or(placement_with_current_size)
        .or_else(|| placement_state.current.clone());

    let Some(placement) = placement else {
        return;
    };

    let Some(ref mut persistent_placement) = persistent_placement else {
        warn!("Failed to save window placement: persistent store unavailable");
        return;
    };

    if let Err(error) = persistent_placement.set(WindowPlacementStore {
        current: Some(placement),
    }) {
        warn!("Failed to save window placement: {error}");
    }
}

pub fn inspector_ui(world: &mut World) {
    let Some((is_visible, x, y, width, height)) = inspector_window_settings(world) else {
        return;
    };

    if !is_visible {
        return;
    }

    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    let egui_context = egui_context.get_mut();
    use_matching_debug_window_text_style(egui_context);

    egui::Window::new("Bevy Inspector")
        .default_pos(egui::pos2(x, y))
        .default_size(egui::vec2(width, height))
        .show(egui_context, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("deck builder");
                bevy_inspector::ui_for_entities_filtered(world, ui, true, &InspectorEntityFilter);
                ui.allocate_space(ui.available_size());
            });
        });
}

pub fn card_ui(world: &mut World) {
    let active_view = world
        .get_resource::<ActiveView>()
        .copied()
        .unwrap_or_default();
    if !should_show_card_ui(active_view) {
        return;
    }

    let card_ui_anchor_offset = card_ui_safe_area_anchor_offset(
        world
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(world)
            .map(|window| Vec2::new(window.resolution.width(), window.resolution.height()))
            .unwrap_or(Vec2::new(GAME_VIEW_WIDTH, GAME_VIEW_HEIGHT)),
    );

    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    let egui_context = egui_context.get_mut();
    use_matching_debug_window_text_style(egui_context);

    let mut flip_requested = false;
    let mut card_settings_to_save = None;

    {
        let Some(mut card_ui_state) = world.get_resource_mut::<CardUiState>() else {
            return;
        };

        egui::Window::new("Card UI")
            .anchor(egui::Align2::RIGHT_TOP, card_ui_anchor_offset)
            .default_width(DEBUG_WINDOW_WIDTH)
            .resizable(false)
            .show(egui_context, |ui| {
                if ui.button("Flip").clicked() {
                    flip_requested = true;
                }
                let safe_area_response =
                    ui.checkbox(&mut card_ui_state.show_safe_area, "Show Safe Area");
                if safe_area_response.changed() {
                    card_settings_to_save = Some(CardSettingsStore::from_state(&card_ui_state));
                }
                ui.add_space(DEBUG_WINDOW_FONT_SIZE);
                let depth_factor_changed = depth_factor_slider_with_reset(
                    ui,
                    "DepthFactor",
                    &mut card_ui_state.depth_factor,
                );
                if depth_factor_changed {
                    card_settings_to_save = Some(CardSettingsStore::from_state(&card_ui_state));
                }
                ui.add_space(DEBUG_WINDOW_FONT_SIZE);
                let background_scale_changed = layer_scale_slider_with_reset(
                    ui,
                    "LayerScale: Background",
                    &mut card_ui_state.background_layer_scale,
                );
                let frame_scale_changed = layer_scale_slider_with_reset(
                    ui,
                    "LayerScale: Frame",
                    &mut card_ui_state.frame_layer_scale,
                );
                let foreground_scale_changed = layer_scale_slider_with_reset(
                    ui,
                    "LayerScale: Foreground",
                    &mut card_ui_state.foreground_layer_scale,
                );
                let title_scale_changed = layer_scale_slider_with_reset(
                    ui,
                    "LayerScale: Title",
                    &mut card_ui_state.title_layer_scale,
                );
                if background_scale_changed
                    || frame_scale_changed
                    || foreground_scale_changed
                    || title_scale_changed
                {
                    card_settings_to_save = Some(CardSettingsStore::from_state(&card_ui_state));
                }
            });
    }

    if let Some(settings) = card_settings_to_save {
        if let Some(mut persistent_settings) =
            world.get_resource_mut::<Persistent<CardSettingsStore>>()
        {
            if let Err(error) = persistent_settings.set(settings) {
                warn!("Failed to save card settings: {error}");
            }
        }
    }

    if flip_requested {
        if let Some(mut flip_state) = world.get_resource_mut::<CardFlipState>() {
            flip_state.request_flip();
        }
    }
}

fn should_show_card_ui(active_view: ActiveView) -> bool {
    matches!(active_view, ActiveView::DebugSettingsScene)
}

fn depth_factor_slider_with_reset(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
    let mut changed = false;
    ui.label(label);
    ui.horizontal(|ui| {
        let slider_width = (ui.available_width() - CARD_UI_RESET_BUTTON_WIDTH).max(0.0);
        let slider_response = ui.add_sized(
            [slider_width, DEBUG_WINDOW_FONT_SIZE],
            egui::Slider::new(value, CARD_DEPTH_FACTOR_MIN..=CARD_DEPTH_FACTOR_MAX),
        );
        changed |= slider_response.changed();
        if ui
            .add_sized(
                [CARD_UI_RESET_BUTTON_WIDTH, DEBUG_WINDOW_FONT_SIZE],
                egui::Button::new("x"),
            )
            .clicked()
        {
            *value = CARD_DEPTH_FACTOR_DEFAULT;
            changed = true;
        }
    });
    changed
}

fn layer_scale_slider_with_reset(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
    let mut changed = false;
    ui.label(label);
    ui.horizontal(|ui| {
        let slider_width = (ui.available_width() - CARD_UI_RESET_BUTTON_WIDTH).max(0.0);
        let slider_response = ui.add_sized(
            [slider_width, DEBUG_WINDOW_FONT_SIZE],
            egui::Slider::new(value, CARD_LAYER_SCALE_MIN..=CARD_LAYER_SCALE_MAX),
        );
        changed |= slider_response.changed();
        if ui
            .add_sized(
                [CARD_UI_RESET_BUTTON_WIDTH, DEBUG_WINDOW_FONT_SIZE],
                egui::Button::new("x"),
            )
            .clicked()
        {
            *value = crate::runtime::resources::CARD_LAYER_SCALE_DEFAULT;
            changed = true;
        }
    });
    changed
}

fn card_ui_safe_area_anchor_offset(window_size: Vec2) -> egui::Vec2 {
    let Some((safe_area_margin, scale)) = game_view_layout(window_size) else {
        return egui::vec2(-SCREEN_PADDING_LEFT, SCREEN_PADDING_TOP);
    };

    egui::vec2(
        -(safe_area_margin.x + (SCREEN_PADDING_LEFT * scale)),
        safe_area_margin.y + (SCREEN_PADDING_TOP * scale),
    )
}

fn game_view_layout(window_size: Vec2) -> Option<(Vec2, f32)> {
    if window_size.x <= 0.0 || window_size.y <= 0.0 {
        return None;
    }

    let game_view_size = Vec2::new(GAME_VIEW_WIDTH, GAME_VIEW_HEIGHT);
    let scale = (window_size.x / game_view_size.x).min(window_size.y / game_view_size.y);
    if scale <= 0.0 {
        return None;
    }

    let safe_area_margin = ((window_size - (game_view_size * scale)) * 0.5).max(Vec2::ZERO);
    Some((safe_area_margin, scale))
}

fn use_matching_debug_window_text_style(context: &egui::Context) {
    let mut style = (*context.style()).clone();
    let font_id = egui::FontId::proportional(DEBUG_WINDOW_FONT_SIZE);

    for text_style in style.text_styles.values_mut() {
        *text_style = font_id.clone();
    }

    context.set_style(style);
}

fn spawn_key_span(
    parent: &mut ChildSpawnerCommands,
    text: &'static str,
    key_code: KeyCode,
    is_toggle: bool,
) {
    parent.spawn((
        TextSpan::new(text),
        debug_hud_text_font(),
        Underline,
        UnderlineColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        DebugHudKeyText::new(key_code, is_toggle),
    ));
}

fn debug_hud_text_font() -> TextFont {
    TextFont {
        font_size: DEBUG_HUD_FONT_SIZE,
        ..Default::default()
    }
}

struct InspectorEntityFilter;

impl EntityFilter for InspectorEntityFilter {
    type StaticFilter = ();

    fn filter_entity(&self, world: &mut World, entity: Entity) -> bool {
        world.get::<Name>(entity).is_some()
    }
}

fn inspector_window_settings(world: &mut World) -> Option<(bool, f32, f32, f32, f32)> {
    let mut query = world.query::<&InspectorState>();
    let inspector = query.iter(world).next()?;
    Some((
        inspector.is_visible,
        inspector.x,
        inspector.y,
        inspector.width,
        inspector.height,
    ))
}

fn placement_for_window(
    window_position: IVec2,
    logical_window_size: UVec2,
    physical_window_size: UVec2,
    monitor_query: &Query<&Monitor>,
) -> Option<WindowPlacement> {
    let monitor = monitor_query
        .iter()
        .max_by_key(|monitor| {
            window_monitor_overlap_area(monitor, window_position, physical_window_size)
        })
        .or_else(|| monitor_query.iter().next())?;

    Some(WindowPlacement {
        window_position,
        window_size: logical_window_size,
        monitor_name: monitor.name.clone(),
        monitor_position: monitor.physical_position,
        monitor_size: monitor.physical_size(),
        relative_position: window_position - monitor.physical_position,
    })
}

fn current_windowed_placement(
    window: &Window,
    fallback_placement: Option<&WindowPlacement>,
    monitor_query: &Query<&Monitor>,
) -> Option<WindowPlacement> {
    match window.position {
        WindowPosition::At(position) => placement_for_window(
            position,
            logical_window_size(window),
            window.resolution.physical_size(),
            monitor_query,
        ),
        WindowPosition::Automatic | WindowPosition::Centered(_) => fallback_placement.cloned(),
    }
}

fn current_monitor_selection(
    window: &Window,
    fallback_placement: Option<&WindowPlacement>,
    monitor_query: &Query<(Entity, &Monitor)>,
) -> MonitorSelection {
    let window_position = match window.position {
        WindowPosition::At(position) => Some(position),
        WindowPosition::Automatic | WindowPosition::Centered(_) => {
            fallback_placement.map(|placement| placement.window_position)
        }
    };

    let Some(window_position) = window_position else {
        return MonitorSelection::Current;
    };

    monitor_query
        .iter()
        .filter_map(|(entity, monitor)| {
            let overlap_area = window_monitor_overlap_area(
                monitor,
                window_position,
                window.resolution.physical_size(),
            );
            (overlap_area > 0).then_some((entity, overlap_area))
        })
        .max_by_key(|(_, overlap_area)| *overlap_area)
        .map(|(entity, _)| MonitorSelection::Entity(entity))
        .unwrap_or(MonitorSelection::Current)
}

fn saved_monitor_selection(
    saved_placement: &WindowPlacement,
    monitor_query: &Query<(Entity, &Monitor)>,
) -> MonitorSelection {
    monitor_query
        .iter()
        .find(|(_, monitor)| {
            monitor.name == saved_placement.monitor_name
                && monitor.physical_size() == saved_placement.monitor_size
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|(_, monitor)| monitor.name == saved_placement.monitor_name)
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|(_, monitor)| monitor.physical_position == saved_placement.monitor_position)
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|(_, monitor)| monitor.physical_size() == saved_placement.monitor_size)
        })
        .map(|(entity, _)| MonitorSelection::Entity(entity))
        .unwrap_or(MonitorSelection::Current)
}

fn restore_windowed_placement(
    window: &mut Window,
    placement_state: &WindowPlacementState,
    monitor_query: &Query<&Monitor>,
) {
    let Some(saved_placement) = placement_state.current.as_ref() else {
        return;
    };

    if let Some(restored_position) = restored_position(monitor_query, saved_placement) {
        window.resolution =
            restored_window_resolution(&window.resolution, saved_placement.window_size);
        window.position = WindowPosition::At(restored_position);
    }
}

fn placement_with_current_window_size(
    saved_placement: &WindowPlacement,
    current_logical_window_size: UVec2,
    current_physical_window_size: UVec2,
    monitor_query: &Query<&Monitor>,
) -> WindowPlacement {
    placement_for_window(
        saved_placement.window_position,
        current_logical_window_size,
        current_physical_window_size,
        monitor_query,
    )
    .unwrap_or_else(|| {
        saved_placement_with_current_window_size(saved_placement, current_logical_window_size)
    })
}

fn saved_placement_with_current_window_size(
    saved_placement: &WindowPlacement,
    current_logical_window_size: UVec2,
) -> WindowPlacement {
    let mut placement = saved_placement.clone();
    placement.window_size = current_logical_window_size;
    placement
}

fn window_monitor_overlap_area(
    monitor: &Monitor,
    window_position: IVec2,
    physical_window_size: UVec2,
) -> i64 {
    let monitor_min = monitor.physical_position;
    let monitor_max = monitor_min + monitor.physical_size().as_ivec2();
    let window_max = window_position + physical_window_size.as_ivec2();

    let overlap_width =
        (window_max.x.min(monitor_max.x) - window_position.x.max(monitor_min.x)).max(0);
    let overlap_height =
        (window_max.y.min(monitor_max.y) - window_position.y.max(monitor_min.y)).max(0);

    i64::from(overlap_width) * i64::from(overlap_height)
}

fn monitor_overlaps_window(monitor: &Monitor, window_position: IVec2, window_size: UVec2) -> bool {
    window_monitor_overlap_area(monitor, window_position, window_size) > 0
}

fn is_windowed(window: &Window) -> bool {
    window.mode == WindowMode::Windowed
}

fn logical_window_size(window: &Window) -> UVec2 {
    UVec2::new(
        window.resolution.width().round().max(1.0) as u32,
        window.resolution.height().round().max(1.0) as u32,
    )
}

fn logical_size_from_resize(resized_event: &WindowResized) -> UVec2 {
    UVec2::new(
        resized_event.width.round().max(1.0) as u32,
        resized_event.height.round().max(1.0) as u32,
    )
}

fn restored_window_resolution(
    current_resolution: &WindowResolution,
    saved_logical_size: UVec2,
) -> WindowResolution {
    let mut resolution = current_resolution.clone();
    resolution.set(saved_logical_size.x as f32, saved_logical_size.y as f32);
    resolution
}

fn restored_position(
    monitor_query: &Query<&Monitor>,
    saved_placement: &WindowPlacement,
) -> Option<IVec2> {
    if monitor_query.iter().any(|monitor| {
        monitor_overlaps_window(
            monitor,
            saved_placement.window_position,
            estimated_physical_window_size(saved_placement, monitor),
        )
    }) {
        return Some(saved_placement.window_position);
    }

    let monitor = find_matching_monitor(monitor_query, saved_placement)?;
    let remapped_position = monitor.physical_position + saved_placement.relative_position;

    if monitor_overlaps_window(
        monitor,
        remapped_position,
        estimated_physical_window_size(saved_placement, monitor),
    ) {
        Some(remapped_position)
    } else {
        None
    }
}

fn estimated_physical_window_size(placement: &WindowPlacement, monitor: &Monitor) -> UVec2 {
    let scale_factor = monitor.scale_factor.max(1.0) as f32;
    UVec2::new(
        (placement.window_size.x as f32 * scale_factor)
            .round()
            .max(1.0) as u32,
        (placement.window_size.y as f32 * scale_factor)
            .round()
            .max(1.0) as u32,
    )
}

fn apply_primary_centered_fallback(window: &mut Window) {
    window.resolution = WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);
    window.position = WindowPosition::Centered(MonitorSelection::Primary);
}

fn find_matching_monitor<'a>(
    monitor_query: &'a Query<&Monitor>,
    saved_placement: &WindowPlacement,
) -> Option<&'a Monitor> {
    monitor_query
        .iter()
        .find(|monitor| {
            monitor.name == saved_placement.monitor_name
                && monitor.physical_size() == saved_placement.monitor_size
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|monitor| monitor.name == saved_placement.monitor_name)
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|monitor| monitor.physical_position == saved_placement.monitor_position)
        })
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/systems_tests.rs"]
mod systems_tests;
