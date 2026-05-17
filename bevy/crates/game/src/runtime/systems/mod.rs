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
    sprite::Anchor,
    text::{Justify, Underline, UnderlineColor},
    window::{
        Monitor, PrimaryWindow, WindowCloseRequested, WindowMode, WindowMoved, WindowResized,
        WindowResolution,
    },
    winit::{UpdateMode, WinitSettings},
};
use bevy_aspect_ratio_mask::Hud;
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::{HotPatched, hot};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
    bevy_inspector::EntityFilter,
};
use bevy_persistent::prelude::Persistent;
use samurai_card_game_shared::{
    GameTitle,
    window::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH},
};
use std::time::Duration;

pub mod app_camera_update_system;
pub mod audio_update_system;
pub mod card_gesture_animation_system;
pub mod card_gesture_update_system;
pub mod card_point_overlay_selection_update_system;
pub mod card_selected_modal_update_system;
pub mod card_selection_update_system;
pub mod debug_drawing_update_system;
pub mod shared_overlay_update_system;
pub mod transition_update_system;
pub mod visual_modifier_update_system;

pub use app_camera_update_system::*;
pub use audio_update_system::*;
pub use card_gesture_animation_system::*;
pub use card_gesture_update_system::*;
pub use card_point_overlay_selection_update_system::*;
pub use card_selected_modal_update_system::*;
pub use card_selection_update_system::*;
pub use debug_drawing_update_system::*;
pub use shared_overlay_update_system::*;
pub use transition_update_system::*;
pub use visual_modifier_update_system::*;

use crate::runtime::bundles::{
    AppCameraBundle, ButtonUiBaseColor, ButtonUiBundle, ButtonUiStyle, CardViewBundle,
    DECK_VIEW_TILE_HEIGHT, DECK_VIEW_TILE_WIDTH, DebugScreenBundle, DeckScreenBundle,
    DeckViewBundle, GameScreenBundle, GridViewUiBundle, LightningScreenBundle, LocationViewBundle,
    MainMenuScreenBundle, MatchmakingScreenBundle, ModalButtonUiBundle, ModalMenuUiBundle,
    ModalPromptUiBundle, ModalUiBundle, POINT_VIEW_BASE_TEXT_FONT_SIZE, PointLocationView,
    PointModel, PointType, PointView, PointViewBundle, PointViewLayering, ScreenTransitionUiBundle,
    SettingsScreenBundle, TopNavigationViewBundle, WorldFadeOverlayBundle, WorldViewBundle,
};
use crate::runtime::components::{
    AppSceneCamera, AppSceneEntity, AppSceneRoot, CardAnimation, CardAnimationFaceLayer,
    CardAnimationFlipStyle, CardAnimationMarker, CardAnimationPhase, CardBackgroundLayer,
    CardFaceLayer, CardFrameLayer, CardGestureView, CardGrid, CardLayerRole, CardParallaxLayer,
    CardSelectionSource, CardSlotGestureTarget, CardView, CpuHandCardView, CpuPlacedCardView,
    DebugHudFpsText, DebugHudKeyText, DebugHudText, DebugSceneEntity, DebugSceneRoot,
    DeckPromptBackdropBlur, DeckSceneEntity, DeckSceneRoot, DeckScreenCardView,
    DeckScreenDeckCommandButton, DeckScreenDeckTileButton, DeckScreenGridBackdrop,
    DeckScreenGridBackdropRole, DeckScreenModalActionButton, DeckScreenModalRoot,
    DeckScreenSelectedCardMenuRoot, DeckScreenTabButton, DeckScreenValidationOkButton,
    DropTargetHint, EndRoundButton, GameControlAction, GameControlButton, GameControlLabel,
    GameLocation, GameLocationBodyText, GameLocationBorder, GameLocationTitleText, GameSceneEntity,
    GameSceneRoot, GridViewContentArea, GridViewMenuArea, GridViewTitleArea, HandCardGestureTarget,
    InspectorState, LocalPlayerHand, LocalPlayerHandCardPreview, LocationBundle,
    LocationBundleIntro, LocationBundleIntroSample, LocationBundleIntroVisual,
    LocationBundleOverlay, LocationBundleSurface, LocationRevealState, MatchStatusText,
    MetaSceneEntity, MetaSceneRoot, MetaScreenButton, MetaScreenButtonAction, Player,
    PointViewCircle, PointViewOutlineTreatment, RoundUi, SelectableCard, SettingsButtonLabel,
    TopNavigationButton, TopNavigationRoot, VISUAL_MODIFIER_CARD_OUTLINE_SCALE,
    VisualModificationTarget, VisualModifier, WORLD_THEME_BLACK_HOLD_SECONDS,
    WORLD_THEME_FADE_SECONDS, WorldBackground, WorldFadeOverlay, WorldThemeTransition,
    WorldThemeTransitionPhase,
};
#[cfg(test)]
use crate::runtime::resources::MatchModeModel;
use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveView, ActiveWorldModel, AppCameraModel, AudioEnum,
    AudioManagerModel, CARD_BACK_TEXTURE_PATH, CARD_DEPTH_FACTOR_DEFAULT, CARD_DEPTH_FACTOR_MAX,
    CARD_DEPTH_FACTOR_MIN, CARD_LAYER_SCALE_MAX, CARD_LAYER_SCALE_MIN,
    CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT, CARD_SAFE_AREA_TEXTURE_PATH,
    CARD_SLOT_LOCATION_COUNT, CardFace, CardFlipState, CardGestureModel, CardGestureState,
    CardInspectionDefaults, CardInspectionState, CardModel, CardModelRegistry, CardSettingsStore,
    CardSlotBoardModel, CardSlotSide, CardSlotState, CardState, CardStateModel, CardUiState,
    CostPointModel, CpuBrainModel, CpuPlacementMotionSourceModel, DECK_SCREEN_CARD_COUNT,
    DECK_SCREEN_COMING_SOON_MESSAGE, DECK_SCREEN_COMING_SOON_TITLE, DECK_SCREEN_DECK_NAME,
    DECK_SCREEN_VALIDATION_MESSAGE, DECK_SCREEN_VALIDATION_TITLE, DebugHudInputStore,
    DebugHudState, DeckEditableZoneModel, DeckEditorTabModel, DeckModel, DeckScreenModel,
    FullscreenViewportTransitionState, GameDeckModel, GameHandModel, GameLocationModel,
    GameRoundModel, GameTicks, LocationModelRegistry, LocationScoreModel,
    MATCH_ASSETS_PRELOAD_ENABLED, MatchModePreferenceStore, MatchModel, MatchPlayerSide,
    MatchResolutionPhase, MatchWinnerModel, MatchmakingModel, MatchmakingPhaseModel,
    MetaGameSettingsModel, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN, PRIMARY_CAMERA_FOV_RADIANS,
    PendingRoundDealResource, PlacementVisibility, PlayerDeckCollectionModel, PowerPointModel,
    PrimaryCameraDefaults, STARTING_HAND_CARD_COUNT, ScreenTransitionPhase,
    ScreenTransitionResource, SelectedCardModalModel, TopNavigationDestination, TopNavigationModel,
    WORLD_MODEL_COUNT, WindowPlacement, WindowPlacementState, WindowPlacementStore,
    WorldModelRegistry, choose_level1_moves, cpu_slot_hand_index, deck_screen_deck_cards,
    deck_screen_library_cards, ensure_deck_screen_collection_no_auto_fill,
    ensure_player_deck_collection_model, final_winner_from_slots, load_window_placement,
    modal_actions_for, move_deck_card_to_library, move_library_card_to_deck,
    random_shuffled_default_deck_cards, reset_two_player_match,
    reset_two_player_match_without_starting_round, start_match_round,
    sync_near_human_from_game_models, valid_window_placement,
};
use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;
use crate::runtime::tweens::{
    GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS, GAME_TWEEN_DEAL_SLIDE_SECONDS, GAME_TWEEN_FLIP_SECONDS,
    GAME_TWEEN_SWAN_FLIP_SECONDS, GAME_TWEEN_SWAN_SCALE_UP_SECONDS, GameTweenPreset,
    ease_out_cubic, location_intro_hold_gate_seconds, sample_card_move_scale,
    sample_card_move_scale_multiplier, sample_flip_y_rotation, sample_swan_scale_multiplier,
};

#[cfg(feature = "desktop-hot-reload")]
use crate::runtime::resources::{
    DebugDrawingModel, HotReloadScreenModel, desktop_hot_reload_patch_count,
    record_desktop_hot_reload_patch,
};

#[cfg(test)]
use bevy::mesh::VertexAttributeValues;

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;
const SCREEN_PADDING_TOP: f32 = 24.0;
const SCREEN_PADDING_LEFT: f32 = 24.0;
const TARGET_WIDTH: f32 = DEFAULT_WINDOW_WIDTH as f32;
const TARGET_HEIGHT: f32 = DEFAULT_WINDOW_HEIGHT as f32;
const GAME_SCENE_WIDTH: f32 = 1280.0;
const GAME_SCENE_HEIGHT: f32 = 800.0;
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
const FRAME_DEPTH_BIAS: f32 = 0.0;
const SAFE_AREA_DEPTH_BIAS: f32 = 0.0;
const FOREGROUND_DEPTH_BIAS: f32 = 0.0;
const TITLE_DEPTH_BIAS: f32 = 0.0;
const POINT_DEPTH_BIAS: f32 = 0.0;
const PARALLAX_OFFSET_RATIO: f32 = 0.065;
const FRAME_THICKNESS_RATIO: f32 = 0.05;
const BACKGROUND_APERTURE_SCALE: f32 = 1.0;
const FRAME_SHINE_STRENGTH: f32 = 0.22;
const GAME_SCENE_ASPECT_RATIO: f32 = GAME_SCENE_WIDTH / GAME_SCENE_HEIGHT;
const GAME_SCENE_HAND_LEFT: f32 = 364.0;
const GAME_SCENE_HAND_TOP: f32 = 612.0;
const GAME_SCENE_HAND_WIDTH: f32 = 552.0;
const GAME_SCENE_HAND_HEIGHT: f32 = GAME_SCENE_HEIGHT - GAME_SCENE_HAND_TOP;
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
const GAME_SCENE_LOCAL_HAND_DEAL_SOURCE_Y: f32 = GAME_SCENE_HEIGHT + 140.0;
const GAME_SCENE_FAR_HAND_Y: f32 = -142.0;
const GAME_SCENE_WORLD_BACKGROUND_Z: f32 = -0.16;
const GAME_SCENE_LOCATION_BUNDLE_Z: f32 = -0.10;
const CARD_RENDER_LAYER: usize = 1;
const CARD_POINT_TEXT_RENDER_LAYER: usize = 2;
const CARD_POINT_TEXT_Z: f32 = 10.0;
const GAME_SCENE_CARD_TILT_RADIANS: f32 = 0.07;
const DECK_SCENE_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.33;
const DECK_SCENE_CARD_HEIGHT_FRACTION: f32 = 0.9;
const DECK_SCREEN_DECK_GRID_LEFT: f32 = 30.0;
const DECK_SCREEN_DECK_GRID_TOP: f32 = 200.0;
const DECK_SCREEN_GRID_MENU_HEIGHT: f32 = 40.0;
const DECK_SCREEN_GRID_TITLE_TOP: f32 = 140.0;
const DECK_SCREEN_GRID_TITLE_OFFSET_X: f32 = 10.0;
const DECK_SCREEN_GRID_TITLE_OFFSET_Y: f32 = 20.0;
const DECK_SCREEN_GRID_TITLE_WIDTH: f32 = 300.0;
const DECK_SCREEN_GRID_TITLE_HEIGHT: f32 = 40.0;
const DECK_SCREEN_DECK_COMMAND_WIDTH: f32 = 150.0;
const DECK_SCREEN_DECK_COMMAND_HEIGHT: f32 = 34.0;
const DECK_SCREEN_DECK_GRID_COLUMN_GAP: f32 = 20.0;
const DECK_SCREEN_DECK_GRID_ROW_GAP: f32 = 20.0;
const DECK_SCREEN_DECK_GRID_COLUMN_WIDTH: f32 = 135.0;
const DECK_SCREEN_DECK_GRID_ROW_HEIGHT: f32 = 180.0;
const DECK_SCREEN_DECK_CARD_HEIGHT: f32 = 154.0;
const DECK_SCREEN_DECK_CARD_WORLD_Z: f32 = 0.08;
const DECK_SCREEN_LIBRARY_GRID_LEFT: f32 = 650.0;
const DECK_SCREEN_GRID_PANEL_WIDTH: f32 = 600.0;
const DECK_SCREEN_GRID_PANEL_HEIGHT: f32 = 580.0;
const DECK_SCREEN_GRID_BACKDROP_WORLD_Z: f32 = -0.05;
const DECK_SCREEN_GRID_BORDER_THICKNESS: f32 = 2.0;
const DECK_SCREEN_SELECTED_CARD_MENU_LEFT: f32 = 1038.0;
const DECK_SCREEN_SELECTED_CARD_MENU_TOP: f32 = 236.0;
const DECK_SCREEN_SELECTED_CARD_MENU_WIDTH: f32 = 212.0;
const DECK_SCREEN_SELECTED_CARD_MENU_BUTTON_HEIGHT: f32 = 42.0;
const SETTINGS_BUTTONS_TOP_PX: f32 = 255.0;
const SETTINGS_COLUMN_WIDTH_PERCENT: f32 = 100.0 / 3.0;
const SETTINGS_COLUMN_GAP_PX: f32 = 20.0;
const DEBUG_HUD_Z_INDEX: i32 = 1_200;
const END_ROUND_BUTTON_NORMAL_COLOR: Color = Color::srgba(0.22, 0.04, 0.44, 0.82);
const END_ROUND_BUTTON_PRESSED_COLOR: Color = Color::srgba(0.12, 0.02, 0.28, 0.95);
const END_ROUND_BUTTON_NORMAL_BORDER_COLOR: Color = Color::srgb(0.45, 0.18, 0.9);
const END_ROUND_BUTTON_PRESSED_BORDER_COLOR: Color = Color::srgb(0.95, 0.82, 1.0);
const GAME_CONTROL_DISABLED_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.55);
const GAME_CONTROL_DISABLED_BORDER_COLOR: Color = Color::srgb(0.28, 0.28, 0.28);
#[cfg(test)]
const CPU_CARD_MOVE_SECONDS: f32 = GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS;
const CPU_HAND_SETTLED_PAUSE_SECONDS: f32 = 0.5;
const CPU_CARD_REVEAL_STAGGER_SECONDS: f32 = 0.25;
const CPU_CARD_MOVING_FRONT_Z: f32 = 0.99;
#[cfg(test)]
const CPU_CARD_MOVE_SCALE_MULTIPLIER: f32 =
    crate::runtime::tweens::GAME_TWEEN_CARD_MOVE_SCALE_MULTIPLIER;
const CPU_CARD_ANIMATION_SETTLE_EPSILON: f32 = 0.001;
const GAME_CONTROL_BUTTON_WIDTH: f32 = 220.0;
const GAME_CONTROL_BUTTON_HEIGHT: f32 = 88.0;
const DEBUG_SCENE_CARD_GAP_TO_CARD_UI: f32 = 20.0;
const DEBUG_SCENE_CARD_VERTICAL_OFFSET: f32 = 100.0;
const DEBUG_SCENE_CARD_EXTRA_DOWN_OFFSET_PX: f32 = 100.0;
const DEBUG_SCENE_CARD_LEFT_OFFSET_PX: f32 = 200.0;
const DEBUG_SCENE_LOCATION_WIDTH: f32 = 240.0;
const DEBUG_SCENE_LOCATION_HEIGHT: f32 = 180.0;
const POINT_VIEW_WIDTH: f32 = 46.0;
const POINT_VIEW_HEIGHT: f32 = 36.0;
const LOCATION_POINT_VIEW_WIDTH: f32 = POINT_VIEW_WIDTH.min(POINT_VIEW_HEIGHT);
const LOCATION_POINT_VIEW_HEIGHT: f32 = LOCATION_POINT_VIEW_WIDTH;
const LOCATION_POINT_TEXT_FONT_SIZE: f32 = 42.0;
const CARD_POINT_BADGE_SIZE: f32 = 0.17;
const CARD_POINT_BADGE_INSET_RATIO: f32 = 0.16;
const CARD_POINT_TEXT_FONT_SIZE: f32 = POINT_VIEW_BASE_TEXT_FONT_SIZE;
const GENERIC_QR_CODE_TEXTURE_PATH: &str = "ui/generic_qr_code.png";
const LIGHTNING_BOLT_ICON_TEXTURE_PATH: &str = "ui/lightning_bolt_icon.png";
#[cfg(not(target_arch = "wasm32"))]
const FULLSCREEN_VIEWPORT_TRANSITION_FRAMES: u8 = 6;

/// HUMAN: Spawns the local player entity for the app.
/// AI: Startup system; keep player setup separate from AppScene and view setup.
pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn setup_primary_camera(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    let _ = (&mut commands, &camera_defaults);
}

pub fn constrain_deck_camera_to_safe_area(
    _primary_window: Query<&Window, With<PrimaryWindow>>,
    _camera_query: Query<&mut Camera, With<DeckSceneEntity>>,
) {
}

pub fn constrain_debug_camera_to_safe_area(
    _primary_window: Query<&Window, With<PrimaryWindow>>,
    _camera_query: Query<&mut Camera, With<DebugSceneEntity>>,
) {
}

/// HUMAN: Keeps GameScene card cameras aligned with the aspect-ratio-safe area.
/// AI: Align 3D cards and their Text2d point overlay camera to the same viewport.
pub fn constrain_game_scene_3d_cameras_to_safe_area(
    _primary_window: Query<&Window, With<PrimaryWindow>>,
    mut fullscreen_viewport_transition: Option<ResMut<FullscreenViewportTransitionState>>,
    _camera_query: Query<&mut Camera, With<GameSceneEntity>>,
) {
    if let Some(ref mut transition) = fullscreen_viewport_transition
        && transition.frames_remaining > 0
    {
        transition.frames_remaining -= 1;
    }
}

#[allow(dead_code)]
fn game_scene_safe_area_viewport_for_window(window: &Window) -> Option<Viewport> {
    app_camera_safe_area_viewport_for_window(window)
}

#[allow(dead_code)]
fn game_scene_safe_area_viewport_for_window_transition(
    window: &Window,
    fullscreen_viewport_transition: Option<&FullscreenViewportTransitionState>,
) -> Option<Viewport> {
    if fullscreen_viewport_transition.is_some_and(|transition| transition.frames_remaining > 0) {
        return None;
    }

    game_scene_safe_area_viewport_for_window(window)
}

#[allow(dead_code)]
fn game_scene_safe_area_viewport(window_size: UVec2) -> Option<Viewport> {
    if window_size.x == 0 || window_size.y == 0 {
        return None;
    }

    let game_scene_size = Vec2::new(GAME_SCENE_WIDTH, GAME_SCENE_HEIGHT);
    let window_size_f32 = window_size.as_vec2();
    let scale = (window_size_f32.x / game_scene_size.x).min(window_size_f32.y / game_scene_size.y);
    if scale <= 0.0 {
        return None;
    }

    let viewport_size = (game_scene_size * scale).round().as_uvec2();
    let viewport_position = ((window_size - viewport_size).as_vec2() * 0.5)
        .round()
        .as_uvec2();

    Some(Viewport {
        physical_position: viewport_position,
        physical_size: viewport_size,
        depth: 0.0..1.0,
    })
}

/// HUMAN: Marks the 2D camera that draws card point Text2d above point circles.
/// AI: Text2d needs this 2D camera even while cards share the AppScene 3D camera.
#[derive(Clone, Copy, Component, Debug, Default, Eq, PartialEq)]
pub struct CardPointTextCamera;

#[cfg_attr(feature = "desktop-hot-reload", hot)]
/// HUMAN: Spawns the persistent AppScene and debug HUD.
/// AI: AppScene remains present while GameScene, DeckScene, and DebugScene swap on top.
pub fn setup_app_scene(
    mut commands: Commands,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
    hud: Option<Res<Hud>>,
) {
    if !app_scene_query.is_empty() {
        return;
    }

    let _ = spawn_app_scene_contents(
        &mut commands,
        hud.as_ref().map(|hud| hud.0),
        app_camera_query.iter().next(),
    );
}

fn spawn_app_scene_contents(
    commands: &mut Commands,
    hud_parent: Option<Entity>,
    app_camera_entity: Option<Entity>,
) -> (Entity, Entity) {
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
    let app_camera = ensure_shared_app_camera(commands, Some(app_scene), app_camera_entity);
    commands.spawn((
        ScreenTransitionUiBundle::default(),
        UiTargetCamera(app_camera),
        AppSceneEntity,
    ));
    let debug_hud = spawn_debug_hud(commands);
    if let Some(hud_parent) = hud_parent {
        commands.entity(hud_parent).add_child(debug_hud);
    } else {
        commands.entity(app_scene).add_child(debug_hud);
    }
    (app_scene, app_camera)
}

/// HUMAN: Ensures normal runtime screens share the persistent AppScene camera.
/// AI: Use this only at AppScene setup boundaries; screen setup must reuse the returned entity.
fn ensure_shared_app_camera(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    app_camera_entity: Option<Entity>,
) -> Entity {
    if let Some(app_camera) = app_camera_entity {
        return app_camera;
    }

    let app_camera = commands
        .spawn(AppCameraBundle::new(&AppCameraModel::active()))
        .id();
    if let Some(parent) = app_scene_parent {
        commands.entity(parent).add_child(app_camera);
    }
    app_camera
}

fn spawn_card_point_text_camera(
    commands: &mut Commands,
    name: &'static str,
    scene_marker: impl Bundle,
) -> Entity {
    commands
        .spawn((
            Name::new(name),
            CardPointTextCamera,
            scene_marker,
            Camera2d,
            Camera {
                order: 3,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: GAME_SCENE_WIDTH,
                    min_height: GAME_SCENE_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
            RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER),
        ))
        .id()
}

/// HUMAN: Spawns the gameplay sub-screen view.
/// AI: GameScene is a view, not the persistent scene; keep AppScene parenting intact.
#[derive(SystemParam)]
pub struct SetupGameSceneParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub active_view: Option<Res<'w, ActiveView>>,
    pub app_scene_query: Query<'w, 's, Entity, With<AppSceneRoot>>,
    pub app_camera_query: Query<'w, 's, Entity, With<AppSceneCamera>>,
    pub hud: Option<Res<'w, Hud>>,
    pub asset_server: Res<'w, AssetServer>,
    pub camera_defaults: Option<Res<'w, PrimaryCameraDefaults>>,
    pub card_defaults: Res<'w, CardInspectionDefaults>,
    pub card_model_registry: Res<'w, CardModelRegistry>,
    pub slot_board: Option<Res<'w, CardSlotBoardModel>>,
    pub active_card_model: Res<'w, ActiveCardModel>,
    pub world_model_registry: Res<'w, WorldModelRegistry>,
    pub active_world_model: ResMut<'w, ActiveWorldModel>,
    pub location_model_registry: Res<'w, LocationModelRegistry>,
    pub active_locations: ResMut<'w, ActiveLocations>,
    pub player_deck_collection: Option<Res<'w, PlayerDeckCollectionModel>>,
    pub game_deck_model: Option<ResMut<'w, GameDeckModel>>,
    pub game_hand_model: Option<ResMut<'w, GameHandModel>>,
    pub game_round_model: Option<ResMut<'w, GameRoundModel>>,
    pub game_location_model: Option<ResMut<'w, GameLocationModel>>,
    pub match_model: Option<ResMut<'w, MatchModel>>,
    pub card_states: Option<ResMut<'w, CardStateModel>>,
    pub pending_round_deal: Option<ResMut<'w, PendingRoundDealResource>>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub masked_background_materials: Option<ResMut<'w, Assets<CardBackgroundMaskMaterial>>>,
}

pub fn setup_game_scene(mut params: SetupGameSceneParams) {
    params
        .active_world_model
        .randomize(&params.world_model_registry);
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
        params.match_model.as_mut(),
        params.card_states.as_deref_mut(),
    ) {
        (
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(game_location_model),
            Some(match_model),
            Some(card_states),
        ) => {
            if let Some(pending_round_deal) = params.pending_round_deal.as_deref_mut() {
                reset_two_player_match_without_starting_round(
                    match_model.mode,
                    &mut *match_model,
                    &mut *game_deck_model,
                    &mut *game_hand_model,
                    &mut *game_round_model,
                    &mut *game_location_model,
                    Some(&params.location_model_registry),
                    Some(&mut *params.active_locations),
                    Some(&params.active_world_model),
                    player_deck_collection.primary_deck(),
                );
                pending_round_deal.is_pending = true;
                pending_round_deal.is_round_deal_complete = false;
                pending_round_deal.waits_for_location_intro = true;
            } else {
                reset_two_player_match(
                    match_model.mode,
                    &mut *match_model,
                    &mut *game_deck_model,
                    &mut *game_hand_model,
                    &mut *game_round_model,
                    &mut *game_location_model,
                    Some(&params.location_model_registry),
                    Some(&mut *params.active_locations),
                    Some(&params.active_world_model),
                    player_deck_collection.primary_deck(),
                );
            }
            card_states.reset_to_size(game_hand_model.len());
            game_hand_model.cards.clone()
        }
        (Some(game_deck_model), Some(game_hand_model), None, None, None, None) => {
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
    let existing_app_camera = params.app_camera_query.iter().next();
    let (app_scene_parent, app_camera) = if let Some(app_scene) =
        params.app_scene_query.iter().next()
    {
        (
            Some(app_scene),
            ensure_shared_app_camera(&mut params.commands, Some(app_scene), existing_app_camera),
        )
    } else {
        let (app_scene, app_camera) = spawn_app_scene_contents(
            &mut params.commands,
            params.hud.as_ref().map(|hud| hud.0),
            existing_app_camera,
        );
        (Some(app_scene), app_camera)
    };
    spawn_game_scene_contents(
        &mut params.commands,
        app_scene_parent,
        app_camera,
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

pub fn setup_game_scene_with_params(params: SetupGameSceneParams) {
    if params
        .active_view
        .as_deref()
        .is_none_or(|active_view| *active_view == ActiveView::GameScene)
    {
        setup_game_scene(params);
    }
}

/// HUMAN: Spawns the initial meta-game screen when the app starts outside GameScreen.
/// AI: Keep startup entry separate from GameScene setup so MainMenuScreen is the default.
pub fn setup_initial_meta_scene(
    mut commands: Commands,
    active_view: Res<ActiveView>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
    asset_server: Res<AssetServer>,
    matchmaking: Res<MatchmakingModel>,
    settings: Res<MetaGameSettingsModel>,
) {
    let app_scene_parent = app_scene_query.single().ok();
    let app_camera = ensure_shared_app_camera(
        &mut commands,
        app_scene_parent,
        app_camera_query.iter().next(),
    );
    match *active_view {
        ActiveView::MainMenuScene => {
            spawn_main_menu_scene_contents(
                &mut commands,
                app_scene_parent,
                app_camera,
                &asset_server,
            );
        }
        ActiveView::LightningScene => {
            spawn_lightning_login_scene_contents(
                &mut commands,
                app_scene_parent,
                app_camera,
                &asset_server,
            );
        }
        ActiveView::MatchmakingScene => {
            spawn_matchmaking_scene_contents(
                &mut commands,
                app_scene_parent,
                app_camera,
                &matchmaking,
            );
        }
        ActiveView::SettingsScene => {
            spawn_settings_scene_contents(&mut commands, app_scene_parent, app_camera, &settings);
        }
        ActiveView::GameScene | ActiveView::DeckScene | ActiveView::DebugScene => {}
    }
}

fn spawn_screen_root(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    bundle: impl Bundle,
) -> Entity {
    let root = commands.spawn(bundle).id();
    if let Some(parent) = app_scene_parent {
        commands.entity(parent).add_child(root);
    }
    root
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    name: &'static str,
    text: impl Into<String>,
    marker: impl Bundle,
) {
    spawn_menu_button_with_optional_icon(parent, None, name, text, marker, false);
}

fn spawn_lightning_menu_button(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    name: &'static str,
    text: impl Into<String>,
    marker: impl Bundle,
) {
    spawn_menu_button_with_optional_icon(parent, Some(asset_server), name, text, marker, true);
}

fn spawn_menu_button_with_optional_icon(
    parent: &mut ChildSpawnerCommands,
    asset_server: Option<&AssetServer>,
    name: &'static str,
    text: impl Into<String>,
    marker: impl Bundle,
    is_lightning: bool,
) {
    let text = text.into();
    parent
        .spawn((
            ButtonUiBundle::new(name)
                .with_node(Node {
                    width: Val::Px(380.0),
                    height: Val::Px(68.0),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    ..Default::default()
                })
                .with_colors(
                    if is_lightning {
                        Color::srgb(0.86, 0.63, 0.18)
                    } else {
                        Color::srgb(0.20, 0.24, 0.32)
                    },
                    if is_lightning {
                        Color::srgb(1.0, 0.82, 0.32)
                    } else {
                        Color::srgb(0.60, 0.64, 0.72)
                    },
                ),
            marker,
        ))
        .with_children(|parent| {
            if let Some(asset_server) = asset_server {
                parent.spawn((
                    ImageNode::new(asset_server.load(LIGHTNING_BOLT_ICON_TEXTURE_PATH))
                        .with_mode(bevy::ui::widget::NodeImageMode::Stretch),
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(34.0),
                        ..Default::default()
                    },
                ));
            }
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(if is_lightning {
                    Color::srgb(0.04, 0.06, 0.08)
                } else {
                    Color::WHITE
                }),
            ));
        });
}

fn spawn_main_menu_scene_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    app_camera: Entity,
    asset_server: &AssetServer,
) {
    let root = spawn_screen_root(commands, app_scene_parent, MainMenuScreenBundle::default());
    let ui_camera = app_camera;
    commands.entity(root).insert(UiTargetCamera(ui_camera));
    commands.entity(root).with_children(|parent| {
        spawn_top_navigation_view(parent, ui_camera, TopNavigationDestination::PlayGame, false);
        parent
            .spawn((
                Name::new("MainMenuScreen Buttons"),
                MetaSceneEntity,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(245.0),
                    left: Val::Px(450.0),
                    width: Val::Px(380.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(18.0),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                spawn_lightning_menu_button(
                    parent,
                    asset_server,
                    "Login with Lightning",
                    "Login with Lightning",
                    MetaScreenButton::new(MetaScreenButtonAction::LightningLogin),
                );
                spawn_menu_button(
                    parent,
                    "Start Game",
                    "Start Game",
                    MetaScreenButton::new(MetaScreenButtonAction::StartGame),
                );
            });
    });
}

fn spawn_lightning_login_scene_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    app_camera: Entity,
    asset_server: &AssetServer,
) {
    let root = spawn_screen_root(commands, app_scene_parent, LightningScreenBundle::default());
    let ui_camera = app_camera;
    commands.entity(root).insert(UiTargetCamera(ui_camera));
    commands.entity(root).with_children(|parent| {
        spawn_top_navigation_view(parent, ui_camera, TopNavigationDestination::PlayGame, false);
        parent.spawn((
            Name::new("Lightning QR Code"),
            MetaSceneEntity,
            ImageNode::new(asset_server.load(GENERIC_QR_CODE_TEXTURE_PATH))
                .with_mode(bevy::ui::widget::NodeImageMode::Stretch),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(175.0),
                left: Val::Px(475.0),
                width: Val::Px(330.0),
                height: Val::Px(330.0),
                border: UiRect::all(Val::Px(12.0)),
                ..Default::default()
            },
            BackgroundColor(Color::WHITE),
            BorderColor::all(Color::srgb(0.94, 0.95, 0.97)),
        ));
        parent
            .spawn((
                Name::new("Lightning Instructions"),
                MetaSceneEntity,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(175.0),
                    left: Val::Px(879.0),
                    width: Val::Px(330.0),
                    height: Val::Px(330.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.08, 0.1, 0.14, 0.55)),
                BorderColor::all(Color::srgba(0.87, 0.9, 0.95, 0.22)),
            ))
            .with_children(|parent| {
                for instruction in [
                    "Scan QR code with your lightning wallet.",
                    "Login is required to play.",
                    "Item purchase and transfer are optional.",
                ] {
                    parent.spawn((
                        Text::new(instruction),
                        TextFont {
                            font_size: 28.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.75, 0.79, 0.85)),
                    ));
                }
            });
        parent
            .spawn((
                Name::new("Lightning Login Buttons"),
                MetaSceneEntity,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(540.0),
                    left: Val::Px(475.0),
                    width: Val::Px(330.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                spawn_lightning_menu_button(
                    parent,
                    asset_server,
                    "Learn About Lightning",
                    "Learn About Lightning",
                    MetaScreenButton::new(MetaScreenButtonAction::LearnLightning),
                );
            });
    });
}

fn spawn_matchmaking_scene_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    app_camera: Entity,
    matchmaking: &MatchmakingModel,
) {
    let root = spawn_screen_root(
        commands,
        app_scene_parent,
        MatchmakingScreenBundle::default(),
    );
    let ui_camera = app_camera;
    commands.entity(root).insert(UiTargetCamera(ui_camera));
    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                ButtonUiBundle::new("Matchmaking Back").with_node(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(24.0),
                    left: Val::Px(565.0),
                    width: Val::Px(150.0),
                    height: Val::Px(46.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                }),
                MetaScreenButton::new(MetaScreenButtonAction::MatchmakingBack),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Back"),
                    TextFont {
                        font_size: 18.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        parent
            .spawn((
                Name::new("Matchmaking Stack"),
                MetaSceneEntity,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(250.0),
                    left: Val::Px(460.0),
                    width: Val::Px(360.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(22.0),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                spawn_matchmaking_text_panel(parent, "Player 01", true);
                parent.spawn((
                    Text::new("vs"),
                    TextFont {
                        font_size: 24.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.40, 0.72, 0.78)),
                ));
                spawn_matchmaking_text_panel(
                    parent,
                    matchmaking.opponent_label(),
                    matchmaking.phase != MatchmakingPhaseModel::Searching,
                );
                parent.spawn((
                    Text::new(matchmaking.status_label()),
                    TextFont {
                        font_size: 30.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.82, 0.88, 0.92)),
                ));
            });
    });
}

fn spawn_matchmaking_text_panel(
    parent: &mut ChildSpawnerCommands,
    label: &'static str,
    is_found: bool,
) {
    parent
        .spawn((
            Name::new("Matchmaking Player Panel"),
            MetaSceneEntity,
            Node {
                width: Val::Px(360.0),
                height: Val::Px(88.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.08, 0.10, 0.14, 0.55)),
            BorderColor::all(Color::srgba(0.68, 0.72, 0.78, 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
                TextColor(if is_found {
                    Color::WHITE
                } else {
                    Color::srgb(0.68, 0.72, 0.78)
                }),
            ));
        });
}

/// HUMAN: Builds the settings screen with three flexible option columns.
/// AI: Keeps settings layout percentage-based so columns track the safe UI width.
fn spawn_settings_scene_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    app_camera: Entity,
    settings: &MetaGameSettingsModel,
) {
    let root = spawn_screen_root(commands, app_scene_parent, SettingsScreenBundle::default());
    let ui_camera = app_camera;
    commands.entity(root).insert(UiTargetCamera(ui_camera));
    commands.entity(root).with_children(|parent| {
        spawn_top_navigation_view(parent, ui_camera, TopNavigationDestination::Settings, false);
        parent
            .spawn((
                Name::new("Settings Buttons"),
                MetaSceneEntity,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(SETTINGS_BUTTONS_TOP_PX),
                    left: Val::Percent(0.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(SETTINGS_COLUMN_GAP_PX),
                    padding: UiRect {
                        right: Val::Px(SETTINGS_COLUMN_GAP_PX),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                spawn_settings_group(
                    parent,
                    "Modes",
                    [
                        (
                            format!("CPU AI Brain: {}", settings.cpu_brain_level.label()),
                            MetaScreenButtonAction::CpuBrain,
                        ),
                        (
                            format!("Mode: {}", settings.selected_mode.label()),
                            MetaScreenButtonAction::MatchMode,
                        ),
                    ],
                );
                spawn_settings_group(
                    parent,
                    "Visuals",
                    [
                        (
                            format!("Framerate: {}", settings.framerate_label()),
                            MetaScreenButtonAction::CycleFramerate,
                        ),
                        (
                            "Quality: Med".to_string(),
                            MetaScreenButtonAction::CycleQuality,
                        ),
                    ],
                );
                spawn_settings_group(
                    parent,
                    "Audio",
                    [
                        (
                            format!(
                                "SFX: {}",
                                MetaGameSettingsModel::audio_label(settings.sfx_enabled)
                            ),
                            MetaScreenButtonAction::ToggleSfx,
                        ),
                        (
                            format!(
                                "Music: {}",
                                MetaGameSettingsModel::audio_label(settings.music_enabled)
                            ),
                            MetaScreenButtonAction::ToggleMusic,
                        ),
                    ],
                );
            });
    });
}

/// HUMAN: Builds one settings column with title and two action buttons.
/// AI: Uses flex percentage width while the parent owns the fixed horizontal gaps.
fn spawn_settings_group(
    parent: &mut ChildSpawnerCommands,
    title: &'static str,
    buttons: [(String, MetaScreenButtonAction); 2],
) {
    parent
        .spawn((
            Name::new(title),
            MetaSceneEntity,
            Node {
                width: Val::Percent(SETTINGS_COLUMN_WIDTH_PERCENT),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(title),
                TextFont {
                    font_size: 26.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
            for (label, action) in buttons {
                spawn_settings_button(
                    parent,
                    "Settings Button",
                    label,
                    action,
                    MetaScreenButton::new(action),
                );
            }
        });
}

/// HUMAN: Builds a settings action button sized to its flexible settings column.
/// AI: Keeps settings-specific layout from changing shared fixed-width menu buttons.
fn spawn_settings_button(
    parent: &mut ChildSpawnerCommands,
    name: &'static str,
    text: impl Into<String>,
    action: MetaScreenButtonAction,
    marker: impl Bundle,
) {
    parent
        .spawn((
            ButtonUiBundle::new(name).with_node(Node {
                width: Val::Percent(100.0),
                height: Val::Px(68.0),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            }),
            marker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text.into()),
                SettingsButtonLabel::new(action),
                TextFont {
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_game_scene_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    app_camera: Entity,
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
    let ui_camera = app_camera;
    let mut scene = commands.spawn(GameScreenBundle::default());
    scene.insert(UiTargetCamera(ui_camera));
    scene.with_children(|parent| {
        spawn_game_scene_ui(
            parent,
            ui_camera,
            asset_server,
            game_round_model,
            slot_board,
        );
    });
    let scene_entity = scene.id();
    let _ = camera_defaults;
    spawn_card_point_text_camera(
        commands,
        "GameScene Card Point Text Camera",
        GameSceneEntity,
    );
    spawn_game_scene_world_background(
        commands,
        asset_server,
        world_model_registry,
        active_world_model,
        meshes,
        materials,
    );
    spawn_location_bundle_surfaces(
        commands,
        asset_server,
        slot_board,
        location_model_registry,
        active_locations,
        game_location_model,
        meshes,
        materials,
    );
    spawn_game_scene_hand_cards(
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
    fastrand::shuffle(&mut source_deck.cards);

    game_deck_model.cards = source_deck.cards;
    game_hand_model.cards.clear();
    game_deck_model.draw_to_hand(STARTING_HAND_CARD_COUNT, game_hand_model);
    card_states.reset_to_size(game_hand_model.len());
}

fn fallback_starting_hand_cards() -> Vec<String> {
    random_shuffled_default_deck_cards()
        .into_iter()
        .take(STARTING_HAND_CARD_COUNT)
        .collect()
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
                        "GameScene {:?} Card Slot {}-{}",
                        side, location_index, slot_index
                    )),
                    GameSceneEntity,
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
            GameSceneEntity,
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

fn spawn_game_scene_ui(
    parent: &mut ChildSpawnerCommands,
    ui_camera: Entity,
    asset_server: &AssetServer,
    game_round_model: &GameRoundModel,
    slot_board: &CardSlotBoardModel,
) {
    parent
        .spawn((
            Name::new("GameScene UI"),
            GameSceneEntity,
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
            spawn_top_navigation_view(parent, ui_camera, TopNavigationDestination::PlayGame, false);
            spawn_drop_target_hints(parent, slot_board);
            spawn_local_player_hand(parent);
            spawn_game_controls(parent, asset_server, game_round_model);
        });
}

fn spawn_game_scene_world_background(
    commands: &mut Commands,
    asset_server: &AssetServer,
    world_model_registry: &WorldModelRegistry,
    active_world_model: &ActiveWorldModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let world_model = world_model_registry.active_world_model(active_world_model);
    let background_size = game_scene_world_background_size();
    let background_mesh = meshes.add(Rectangle::new(background_size.x, background_size.y));
    let background_material = card_model_material(
        asset_server,
        materials,
        world_model.background_texture,
        AlphaMode::Opaque,
        BACKGROUND_DEPTH_BIAS,
    );
    let fade_overlay_material = world_fade_overlay_material(materials, 1.0);
    commands
        .spawn(WorldViewBundle::new(
            world_model.display_name,
            active_world_model.index,
            background_mesh.clone(),
            background_material,
            Transform::from_translation(Vec3::new(0.0, 0.0, GAME_SCENE_WORLD_BACKGROUND_Z)),
        ))
        .with_children(|parent| {
            parent.spawn(WorldFadeOverlayBundle::new(
                background_mesh,
                fade_overlay_material,
            ));
        })
        .id()
}

/// HUMAN: Spawns the 3D background surface for each GameScene location bundle.
/// AI: World-space placement comes from the same safe-area location rects as card slots.
fn spawn_location_bundle_surfaces(
    commands: &mut Commands,
    asset_server: &AssetServer,
    slot_board: &CardSlotBoardModel,
    location_model_registry: &LocationModelRegistry,
    active_locations: &ActiveLocations,
    game_location_model: &GameLocationModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let selected_locations = location_model_registry.selected_locations(active_locations);

    for location_index in 0..CARD_SLOT_LOCATION_COUNT {
        let Some(area_rect) = slot_board.location_area_rect(location_index) else {
            continue;
        };
        let Some(location) = selected_locations.get(location_index) else {
            continue;
        };
        let location_definition = game_location_model.definition(location_index);
        let reveal_state = if location_definition
            .is_some_and(|definition| definition.is_open(game_location_model.round))
        {
            LocationRevealState::Revealed
        } else {
            LocationRevealState::Unrevealed
        };
        let title = location_definition
            .map(|definition| definition.display_title(game_location_model.round))
            .unwrap_or(location.display_name);
        let body = location_definition
            .map(|definition| definition.display_body(game_location_model.round))
            .unwrap_or("");
        let bundle_size = LocationViewBundle::scaled_size(area_rect);
        let world_width = game_scene_world_width_for_game_scene_width(
            bundle_size.x,
            GAME_SCENE_LOCATION_BUNDLE_Z,
        );
        let world_height = game_scene_world_height_for_game_scene_height(
            bundle_size.y,
            GAME_SCENE_LOCATION_BUNDLE_Z,
        );
        let game_scene_center = Vec2::new(
            area_rect.left + (area_rect.width * 0.5),
            area_rect.top + (area_rect.height * 0.5),
        );
        let transform = Transform {
            translation: game_scene_world_position_from_game_scene(
                game_scene_center,
                GAME_SCENE_LOCATION_BUNDLE_Z,
            ),
            scale: Vec3::splat(LocationBundleIntroSample::at(location_index, 0.0).scale),
            ..Default::default()
        };
        let intro_sample = LocationBundleIntroSample::at(location_index, 0.0);
        let border_color = game_location_model.border_color(location_index);
        let border_material = flat_color_material(
            materials,
            color_with_alpha(border_color, intro_sample.opacity),
        );
        let point_circle_mesh = meshes.add(Circle::new(
            game_scene_world_height_for_game_scene_height(
                LOCATION_POINT_VIEW_HEIGHT,
                GAME_SCENE_LOCATION_BUNDLE_Z,
            ) * 0.5,
        ));
        let location_score = LocationScoreModel::empty(location_index);

        commands
            .spawn((
                Name::new(format!("location_bundle Surface {location_index}")),
                LocationBundle::new(location_index),
                LocationBundleSurface::new(location_index),
                LocationBundleOverlay::new(location_index),
                LocationBundleIntro::new(location_index),
                GameLocation::new(location_index, reveal_state),
                GameSceneEntity,
                Mesh3d(meshes.add(Rectangle::new(world_width, world_height))),
                MeshMaterial3d(location_bundle_material(
                    asset_server,
                    materials,
                    location.texture,
                    intro_sample.opacity,
                )),
                transform,
                Visibility::Visible,
                NoCpuCulling,
                NoFrustumCulling,
            ))
            .with_children(|parent| {
                spawn_location_border_meshes(
                    parent,
                    location_index,
                    world_width,
                    world_height,
                    border_material,
                    meshes,
                );
                spawn_location_title_and_body_3d(
                    parent,
                    location_index,
                    title,
                    body,
                    world_height,
                    intro_sample.opacity,
                );
                spawn_location_power_point_view(
                    parent,
                    materials,
                    location_score.opponent_total,
                    location_index,
                    CardSlotSide::Opponent,
                    world_height,
                    point_circle_mesh.clone(),
                    true,
                    intro_sample.opacity,
                );
                spawn_location_power_point_view(
                    parent,
                    materials,
                    location_score.local_total,
                    location_index,
                    CardSlotSide::LocalPlayer,
                    world_height,
                    point_circle_mesh,
                    false,
                    intro_sample.opacity,
                );
            });
    }
}

fn flat_color_material(
    materials: &mut Assets<StandardMaterial>,
    color: Color,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        unlit: true,
        ..Default::default()
    })
}

fn location_bundle_material(
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
    texture_path: &'static str,
    initial_opacity: f32,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, initial_opacity.clamp(0.0, 1.0)),
        base_color_texture: Some(asset_server.load(texture_path)),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        unlit: true,
        ..Default::default()
    })
}

/// HUMAN: Spawns four 3D strips that form the border of a location surface.
/// AI: Keep these as children of the location mesh so background and border share projection.
fn spawn_location_border_meshes(
    parent: &mut ChildSpawnerCommands,
    location_index: usize,
    world_width: f32,
    world_height: f32,
    border_material: Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) {
    let horizontal_thickness = game_scene_world_height_for_game_scene_height(
        LocationViewBundle::BORDER_THICKNESS,
        GAME_SCENE_LOCATION_BUNDLE_Z,
    );
    let vertical_thickness = game_scene_world_width_for_game_scene_width(
        LocationViewBundle::BORDER_THICKNESS,
        GAME_SCENE_LOCATION_BUNDLE_Z,
    );
    let half_width = world_width * 0.5;
    let half_height = world_height * 0.5;

    for (name, size, translation) in [
        (
            "Top",
            Vec2::new(world_width, horizontal_thickness),
            Vec3::new(0.0, half_height, LAYER_RENDER_Z_STEP),
        ),
        (
            "Bottom",
            Vec2::new(world_width, horizontal_thickness),
            Vec3::new(0.0, -half_height, LAYER_RENDER_Z_STEP),
        ),
        (
            "Left",
            Vec2::new(vertical_thickness, world_height),
            Vec3::new(-half_width, 0.0, LAYER_RENDER_Z_STEP),
        ),
        (
            "Right",
            Vec2::new(vertical_thickness, world_height),
            Vec3::new(half_width, 0.0, LAYER_RENDER_Z_STEP),
        ),
    ] {
        parent.spawn((
            Name::new(format!("Game Location Border {location_index} {name}")),
            GameLocationBorder::new(location_index),
            LocationBundleIntroVisual::new(location_index),
            Mesh3d(meshes.add(Rectangle::new(size.x, size.y))),
            MeshMaterial3d(border_material.clone()),
            Transform::from_translation(translation),
            NoCpuCulling,
            NoFrustumCulling,
        ));
    }
}

/// HUMAN: Spawns location title/body as world-space text above the location surface.
/// AI: Use Text2d here so location copy follows the same 3D projection as the surface.
fn spawn_location_title_and_body_3d(
    parent: &mut ChildSpawnerCommands,
    location_index: usize,
    title: &'static str,
    body: &'static str,
    world_height: f32,
    initial_opacity: f32,
) {
    parent.spawn((
        Name::new("Game Location Title Text"),
        GameLocationTitleText::new(location_index),
        LocationBundleIntroVisual::new(location_index),
        Text2d::new(title),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, initial_opacity.clamp(0.0, 1.0))),
        Anchor::CENTER,
        Transform::from_translation(Vec3::new(
            0.0,
            world_height * 0.2,
            LAYER_RENDER_Z_STEP * 2.0,
        )),
        NoCpuCulling,
    ));
    parent.spawn((
        Name::new("Game Location Body Text"),
        GameLocationBodyText::new(location_index),
        LocationBundleIntroVisual::new(location_index),
        Text2d::new(body),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: 13.0,
            ..Default::default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, initial_opacity.clamp(0.0, 1.0))),
        Anchor::CENTER,
        Transform::from_translation(Vec3::new(
            0.0,
            world_height * 0.04,
            LAYER_RENDER_Z_STEP * 2.0,
        )),
        NoCpuCulling,
    ));
}

fn game_scene_perspective_view_size_at_z(z: f32) -> Vec2 {
    let distance = (PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN - z).abs();
    let height = 2.0 * (PRIMARY_CAMERA_FOV_RADIANS * 0.5).tan() * distance;

    Vec2::new(height * GAME_SCENE_ASPECT_RATIO, height)
}

/// HUMAN: Sizes the GameScene world backdrop to cover the safe gameplay viewport.
/// AI: Keep this tied to the GameScene camera projection so background tests match runtime framing.
fn game_scene_world_background_size() -> Vec2 {
    game_scene_perspective_view_size_at_z(GAME_SCENE_WORLD_BACKGROUND_Z)
}

/// HUMAN: Scales the deck card to fill most of the centered presentation view.
/// AI: The centered card uses world units so it remains independent of Bevy UI layout.
fn deck_centered_card_scale(card_defaults: &CardInspectionDefaults) -> f32 {
    game_scene_world_height_for_game_scene_height(
        GAME_SCENE_HEIGHT * DECK_SCENE_CARD_HEIGHT_FRACTION,
        0.0,
    ) / card_defaults.height
}

fn game_scene_world_position_from_game_scene(game_scene_position: Vec2, z: f32) -> Vec3 {
    let view_size = game_scene_perspective_view_size_at_z(z);

    Vec3::new(
        ((game_scene_position.x / GAME_SCENE_WIDTH) - 0.5) * view_size.x,
        (0.5 - (game_scene_position.y / GAME_SCENE_HEIGHT)) * view_size.y,
        z,
    )
}

fn game_scene_position_from_world_position(world_position: Vec3) -> Vec2 {
    let view_size = game_scene_perspective_view_size_at_z(world_position.z);

    Vec2::new(
        ((world_position.x / view_size.x) + 0.5) * GAME_SCENE_WIDTH,
        (0.5 - (world_position.y / view_size.y)) * GAME_SCENE_HEIGHT,
    )
}

fn game_scene_text2d_position_from_game_scene(game_scene_position: Vec2, z: f32) -> Vec3 {
    Vec3::new(
        game_scene_position.x - (GAME_SCENE_WIDTH * 0.5),
        (GAME_SCENE_HEIGHT * 0.5) - game_scene_position.y,
        z,
    )
}

fn game_scene_world_units_per_game_scene_pixel(z: f32) -> f32 {
    game_scene_perspective_view_size_at_z(z).y / GAME_SCENE_HEIGHT
}

fn game_scene_world_height_for_game_scene_height(game_scene_height: f32, z: f32) -> f32 {
    game_scene_perspective_view_size_at_z(z).y * (game_scene_height / GAME_SCENE_HEIGHT)
}

fn game_scene_world_width_for_game_scene_width(game_scene_width: f32, z: f32) -> f32 {
    game_scene_perspective_view_size_at_z(z).x * (game_scene_width / GAME_SCENE_WIDTH)
}

/// HUMAN: Refreshes visible location reveal state after round changes.
/// AI: Keep spawned GameScene location components in sync with GameLocationModel.
pub fn update_game_location_views_system(
    game_location_model: Option<Res<GameLocationModel>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut location_query: Query<&mut GameLocation>,
    mut text_queries: ParamSet<(
        Query<(&GameLocationTitleText, &mut Text)>,
        Query<(&GameLocationBodyText, &mut Text)>,
        Query<(&GameLocationTitleText, &mut Text2d)>,
        Query<(&GameLocationBodyText, &mut Text2d)>,
    )>,
    border_query: Query<(&GameLocationBorder, &MeshMaterial3d<StandardMaterial>)>,
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

    for (title, mut text) in &mut text_queries.p2() {
        if let Some(definition) = game_location_model.definition(title.location_index) {
            text.0 = definition
                .display_title(game_location_model.round)
                .to_string();
        }
    }

    for (body, mut text) in &mut text_queries.p3() {
        if let Some(definition) = game_location_model.definition(body.location_index) {
            text.0 = definition
                .display_body(game_location_model.round)
                .to_string();
        }
    }

    if let Some(materials) = materials.as_deref_mut() {
        for (border, material) in &border_query {
            if let Some(material) = materials.get_mut(&material.0) {
                let alpha = material.base_color.to_srgba().alpha;
                material.base_color = color_with_alpha(
                    game_location_model.border_color(border.location_index),
                    alpha,
                );
                material.alpha_mode = AlphaMode::Blend;
            }
        }
    }
}

/// HUMAN: Runs the start-of-game location reveal sequence.
/// AI: One system drives 3D surface opacity and safe-area overlay opacity/scale together.
pub fn location_intro_update_system(
    time: Res<Time>,
    world_query: Query<&WorldThemeTransition, With<WorldBackground>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut intro_query: Query<(
        &mut LocationBundleIntro,
        &mut Transform,
        Option<&MeshMaterial3d<StandardMaterial>>,
    )>,
    mut visual_query: Query<(
        &LocationBundleIntroVisual,
        Option<&mut ImageNode>,
        Option<&mut BackgroundColor>,
        Option<&mut BorderColor>,
        Option<&mut TextColor>,
        Option<&MeshMaterial3d<StandardMaterial>>,
    )>,
) {
    if !world_intro_sequence_complete(&world_query) {
        return;
    }

    let mut samples: [Option<LocationBundleIntroSample>; CARD_SLOT_LOCATION_COUNT] =
        [None; CARD_SLOT_LOCATION_COUNT];

    for (mut intro, mut transform, material) in &mut intro_query {
        intro.elapsed_seconds += time.delta_secs();
        let sample = intro.sample();
        transform.scale = Vec3::splat(sample.scale);

        if let Some(material) = material
            && let Some(material) = materials.get_mut(&material.0)
        {
            material.base_color = color_with_alpha(material.base_color, sample.opacity);
            material.alpha_mode = AlphaMode::Blend;
        }

        if intro.location_index < CARD_SLOT_LOCATION_COUNT {
            samples[intro.location_index] = Some(sample);
        }
    }

    for (visual, image, background, border, text, material) in &mut visual_query {
        let Some(sample) = samples.get(visual.location_index).copied().flatten() else {
            continue;
        };

        if let Some(mut image) = image {
            image.color = color_with_alpha(image.color, sample.opacity);
        }
        if let Some(mut background) = background {
            background.0 = color_with_alpha(background.0, sample.opacity);
        }
        if let Some(mut border) = border {
            border.top = color_with_alpha(border.top, sample.opacity);
            border.right = color_with_alpha(border.right, sample.opacity);
            border.bottom = color_with_alpha(border.bottom, sample.opacity);
            border.left = color_with_alpha(border.left, sample.opacity);
        }
        if let Some(mut text) = text {
            text.0 = color_with_alpha(text.0, sample.opacity);
        }
        if let Some(material) = material
            && let Some(material) = materials.get_mut(&material.0)
        {
            material.base_color = color_with_alpha(material.base_color, sample.opacity);
            material.alpha_mode = AlphaMode::Blend;
        }
    }
}

fn world_intro_sequence_complete(
    world_query: &Query<&WorldThemeTransition, With<WorldBackground>>,
) -> bool {
    world_query
        .iter()
        .all(|transition| transition.phase == WorldThemeTransitionPhase::Idle)
}

fn location_intro_sequence_complete(intro_query: &Query<&LocationBundleIntro>) -> bool {
    let mut saw_intro = false;
    let mut completed_locations = [false; CARD_SLOT_LOCATION_COUNT];
    let mut elapsed_seconds = None;
    for intro in intro_query {
        saw_intro = true;
        elapsed_seconds = Some(intro.elapsed_seconds);
        if intro.location_index < CARD_SLOT_LOCATION_COUNT && intro.sample().opacity >= 1.0 {
            completed_locations[intro.location_index] = true;
        }
    }

    if !saw_intro {
        return true;
    }

    let Some(elapsed_seconds) = elapsed_seconds else {
        return false;
    };
    let hold_gate_seconds = location_intro_hold_gate_seconds(CARD_SLOT_LOCATION_COUNT);

    completed_locations.iter().all(|is_complete| *is_complete)
        && elapsed_seconds >= hold_gate_seconds
}

/// HUMAN: Starts pending round dealing only after the active Round Start sequence completes.
/// AI: Round one waits for location intro; later rounds wait for this explicit sequence boundary.
pub fn start_pending_round_deal_after_round_start_system(
    active_view: Option<Res<ActiveView>>,
    mut pending_round_deal: ResMut<PendingRoundDealResource>,
    intro_query: Query<&LocationBundleIntro>,
    mut game_deck_model: ResMut<GameDeckModel>,
    mut game_hand_model: ResMut<GameHandModel>,
    game_round_model: Res<GameRoundModel>,
    mut match_model: ResMut<MatchModel>,
    mut card_states: ResMut<CardStateModel>,
) {
    let waits_for_location_intro = pending_round_deal.waits_for_location_intro;
    if !is_game_scene_active(active_view.as_deref())
        || !pending_round_deal.is_pending
        || (waits_for_location_intro && !location_intro_sequence_complete(&intro_query))
    {
        return;
    }

    if waits_for_location_intro {
        if let Some(elapsed_seconds) = intro_query.iter().next().map(|intro| intro.elapsed_seconds)
        {
            pending_round_deal.location_intro_completed_event_count += 1;
            pending_round_deal.last_location_intro_completed_elapsed_ms =
                (elapsed_seconds.max(0.0) * 1000.0).round() as u64;
            info!(
                "GameScene location intro sequence complete: event={}, elapsed_ms={}",
                pending_round_deal.location_intro_completed_event_count,
                pending_round_deal.last_location_intro_completed_elapsed_ms
            );
        }
    }

    let near_count_before = game_hand_model.len();
    pending_round_deal.is_round_deal_complete = false;
    start_match_round(
        &mut match_model,
        &game_round_model,
        &mut game_deck_model,
        &mut game_hand_model,
    );
    let near_count_after = game_hand_model.len();
    if near_count_after > near_count_before {
        pending_round_deal.near_deal_completed_event_count += 1;
        pending_round_deal.last_near_deal_completed_card_count =
            near_count_after - near_count_before;
        info!(
            "GameScene near deal complete: event={}, dealt_cards={}",
            pending_round_deal.near_deal_completed_event_count,
            pending_round_deal.last_near_deal_completed_card_count
        );
    }
    card_states.ensure_size(game_hand_model.len());
    pending_round_deal.is_pending = false;
    pending_round_deal.waits_for_location_intro = false;
}

/// HUMAN: Opens player interaction only after both local and CPU dealt hand visuals settle.
/// AI: Keeps the setup sequence world fade -> locations -> deal -> player input deterministic.
pub fn round_deal_completion_update_system(
    active_view: Option<Res<ActiveView>>,
    mut pending_round_deal: ResMut<PendingRoundDealResource>,
    card_defaults: Res<CardInspectionDefaults>,
    card_states: Res<CardStateModel>,
    local_hand_query: Query<(&HandCardGestureTarget, &Transform), With<CardGestureView>>,
    cpu_hand_query: Query<Option<&CardAnimation>, With<CpuHandCardView>>,
) {
    if !is_game_scene_active(active_view.as_deref())
        || pending_round_deal.is_pending
        || pending_round_deal.is_round_deal_complete
    {
        return;
    }

    if round_deal_visuals_are_complete(
        &card_defaults,
        &card_states,
        &local_hand_query,
        &cpu_hand_query,
    ) {
        pending_round_deal.is_round_deal_complete = true;
    }
}

fn round_deal_visuals_are_complete(
    card_defaults: &CardInspectionDefaults,
    card_states: &CardStateModel,
    local_hand_query: &Query<(&HandCardGestureTarget, &Transform), With<CardGestureView>>,
    cpu_hand_query: &Query<Option<&CardAnimation>, With<CpuHandCardView>>,
) -> bool {
    let hand_indices = card_states.indices_with_state(CardState::Hand);
    for (order_index, hand_index) in hand_indices.iter().enumerate() {
        let Some((_, transform)) = local_hand_query
            .iter()
            .find(|(target, _)| target.hand_index == *hand_index)
        else {
            return false;
        };
        let target_transform =
            hand_source_transform(order_index, hand_indices.len(), card_defaults);
        if transform.translation.distance(target_transform.translation) > 0.01
            || transform.scale.distance(target_transform.scale) > 0.01
            || transform.rotation.angle_between(target_transform.rotation) > 0.01
        {
            return false;
        }
    }

    cpu_hand_query.iter().all(|animation| animation.is_none())
}

fn color_with_alpha(color: Color, alpha: f32) -> Color {
    let srgba = color.to_srgba();
    if srgba.alpha <= 0.0 && srgba.red == 0.0 && srgba.green == 0.0 && srgba.blue == 0.0 {
        return Color::NONE;
    }
    Color::srgba(srgba.red, srgba.green, srgba.blue, alpha.clamp(0.0, 1.0))
}

/// HUMAN: Spawns top and bottom location power badges centered on bundle edges.
/// AI: Keeps text world-space with its location parent so the 3D camera can render it.
fn spawn_location_power_point_view(
    parent: &mut ChildSpawnerCommands,
    materials: &mut Assets<StandardMaterial>,
    model: PowerPointModel,
    location_index: usize,
    side: CardSlotSide,
    location_world_height: f32,
    circle_mesh: Handle<Mesh>,
    is_top: bool,
    initial_opacity: f32,
) {
    let point_model = PointModel::from_power_point(PointType::LocationPower, model);
    let point_y = if is_top {
        location_world_height * 0.5
    } else {
        -location_world_height * 0.5
    };
    let circle_material = flat_color_material(
        materials,
        color_with_alpha(point_model.background_color(), initial_opacity),
    );

    parent
        .spawn((
            PointViewBundle::new("PowerPointView", point_model),
            PointLocationView::new(location_index, side),
            Transform::from_translation(Vec3::new(
                0.0,
                point_y,
                PointViewLayering::SURFACE_LOCAL_Z,
            )),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("PowerPointView Circle"),
                PointViewCircle::new(VisualModificationTarget::LocationTotalPointCircle),
                Mesh3d(circle_mesh),
                MeshMaterial3d(circle_material),
                LocationBundleIntroVisual::new(location_index),
                Transform::from_translation(Vec3::new(0.0, 0.0, PointViewLayering::CIRCLE_LOCAL_Z)),
                NoCpuCulling,
                NoFrustumCulling,
            ));
            parent.spawn((
                Name::new("PowerPointView Text"),
                LocationBundleIntroVisual::new(location_index),
                Text2d::new(point_model.display_text()),
                TextFont {
                    font_size: LOCATION_POINT_TEXT_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(color_with_alpha(point_model.text_color(), initial_opacity)),
                TextLayout::new_with_justify(Justify::Center),
                Anchor::CENTER,
                Transform::from_translation(Vec3::new(
                    0.0,
                    0.0,
                    PointViewLayering::TEXT_OVERLAY_LOCAL_Z,
                )),
                NoCpuCulling,
            ));
        });
}

fn spawn_card_cost_point_view(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    model: CostPointModel,
    background_mesh: Handle<Mesh>,
    background_material: Handle<StandardMaterial>,
    outline_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
    uses_cpu_face_control: bool,
) {
    let point_model = PointModel::from_cost_point(model);
    spawn_card_point_view_world(
        parent,
        asset_server,
        "Card EnergyPointView Background",
        point_model,
        background_mesh,
        background_material,
        outline_material,
        background_translation,
        text_translation,
        is_visible,
        uses_cpu_face_control,
    );
}

fn spawn_card_power_point_view(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    model: PowerPointModel,
    background_mesh: Handle<Mesh>,
    background_material: Handle<StandardMaterial>,
    outline_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
    uses_cpu_face_control: bool,
) {
    let point_model = PointModel::from_power_point(PointType::CardPower, model);
    spawn_card_point_view_world(
        parent,
        asset_server,
        "Card PowerPointView Background",
        point_model,
        background_mesh,
        background_material,
        outline_material,
        background_translation,
        text_translation,
        is_visible,
        uses_cpu_face_control,
    );
}

/// HUMAN: Recalculates visible location power totals from runtime slot occupancy.
/// AI: This is the GameScene bridge from placed card slots to point presentation.
pub fn update_location_power_points(
    active_view: Option<Res<ActiveView>>,
    slot_board: Res<CardSlotBoardModel>,
    card_model_registry: Res<CardModelRegistry>,
    game_location_model: Option<Res<GameLocationModel>>,
    game_hand_model: Option<Res<GameHandModel>>,
    match_model: Option<Res<MatchModel>>,
    mut power_query: Query<(&PointLocationView, &mut PointView, &Children)>,
    mut text_queries: ParamSet<(Query<&mut Text>, Query<&mut Text2d>)>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

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
            match_model.as_deref(),
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
            if let Ok(mut text) = text_queries.p0().get_mut(child) {
                text.0 = display_text.clone();
            }
            if let Ok(mut text) = text_queries.p1().get_mut(child) {
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
    match_model: Option<&MatchModel>,
    location_index: usize,
    side: CardSlotSide,
) -> PowerPointModel {
    let mut counted_card_count = 0;
    let total: i32 = slot_board
        .slots()
        .filter(|slot| slot.location_index == location_index && slot.side == side)
        .filter(|slot| {
            match_model.is_none_or(|match_model| {
                let owner = match side {
                    CardSlotSide::LocalPlayer => MatchPlayerSide::Near,
                    CardSlotSide::Opponent => MatchPlayerSide::Far,
                };
                match_model.placements.iter().any(|placement| {
                    placement.owner == owner
                        && placement.location_index == slot.location_index
                        && placement.slot_index == slot.slot_index
                        && placement.visibility == PlacementVisibility::Revealed
                })
            })
        })
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
                    counted_card_count += 1;
                    card_model.base_power.value
                        + game_location_model
                            .map(|locations| {
                                locations.ability_delta_for_location(slot.location_index)
                            })
                            .unwrap_or(0)
                }),
        })
        .sum();

    let multiplier = game_location_model
        .map(|locations| {
            locations.power_multiplier_for_location_side(location_index, counted_card_count)
        })
        .unwrap_or(1);

    PowerPointModel::new(total * multiplier)
}

/// HUMAN: Applies open location power abilities to the red power point value on placed cards.
/// AI: Keep this in PointView data; presentation can render its display_text normally.
pub(crate) fn update_card_power_point_views_system(
    active_view: Option<Res<ActiveView>>,
    slot_board: Res<CardSlotBoardModel>,
    card_model_registry: Res<CardModelRegistry>,
    game_location_model: Option<Res<GameLocationModel>>,
    game_hand_model: Option<Res<GameHandModel>>,
    card_query: Query<(&HandCardGestureTarget, &Children), With<CardGestureView>>,
    mut point_query: Query<(&mut PointView, Option<&Children>)>,
    mut text_query: Query<(&CardPointTextView, &mut Text2d)>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

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
    _asset_server: &AssetServer,
    name: &str,
    point_model: PointModel,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    outline_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
    uses_cpu_face_control: bool,
) {
    let mut point_entity = parent.spawn((
        PointViewBundle::new(name, point_model),
        Transform::from_translation(background_translation),
        RenderLayers::layer(CARD_RENDER_LAYER),
        CardFaceLayer::new(CardFace::Front),
        if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
    ));
    if uses_cpu_face_control {
        point_entity.insert(CardAnimationFaceLayer);
    }
    point_entity
        .with_children(|parent| {
            if point_model.point_type == PointType::CardPower {
                parent.spawn((
                    Name::new(format!("{name} AbilityOutline")),
                    PointViewOutlineTreatment::new(VisualModifier::AbilityOutline),
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(outline_material),
                    Transform {
                        translation: Vec3::new(0.0, 0.0, -LAYER_RENDER_Z_STEP),
                        scale: Vec3::splat(VISUAL_MODIFIER_CARD_OUTLINE_SCALE),
                        ..Default::default()
                    },
                    RenderLayers::layer(CARD_RENDER_LAYER),
                    Visibility::Hidden,
                    NoCpuCulling,
                    NoFrustumCulling,
                ));
            }
            parent.spawn((
                Name::new(format!("{name} Circle")),
                PointViewCircle::new(match point_model.point_type {
                    PointType::LocationPower => VisualModificationTarget::LocationTotalPointCircle,
                    _ => VisualModificationTarget::CardPowerPointCircle,
                }),
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
                CardFaceLayer::new(CardFace::Front),
                Text2d::new(point_model.display_text()),
                TextFont {
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
                if is_visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                NoCpuCulling,
            ));
        })
        .observe(card_click_navigation);
}

/// HUMAN: Keeps point Text2d above its badge in a dedicated overlay depth band.
/// AI: Projects every PointView text child onto the dedicated overlay layer above surfaces.
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
    let game_scene_position =
        game_scene_position_from_world_position(point_transform.translation());
    let text_position =
        game_scene_text2d_position_from_game_scene(game_scene_position, CARD_POINT_TEXT_Z);
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
    let point_view_size = game_scene_perspective_view_size_at_z(point_translation.z);
    let reference_view_size = game_scene_perspective_view_size_at_z(GAME_SCENE_HAND_CARD_WORLD_Z);
    let point_pixels_per_world_unit = GAME_SCENE_HEIGHT / point_view_size.y;
    let reference_pixels_per_world_unit = GAME_SCENE_HEIGHT / reference_view_size.y;

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

/// HUMAN: Stores pre-scene-switch visibility for hidden GameScene entities.
/// AI: Restores exact card layer visibility when returning from non-game views.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct GameSceneSceneHiddenVisibility(Visibility);

fn spawn_local_player_hand(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Name::new("Local Player Hand"),
        LocalPlayerHand,
        GameSceneEntity,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(GAME_SCENE_HAND_LEFT),
            top: Val::Px(GAME_SCENE_HAND_TOP),
            width: Val::Px(GAME_SCENE_HAND_WIDTH),
            height: Val::Px(GAME_SCENE_HAND_HEIGHT),
            ..Default::default()
        },
        BorderColor::all(Color::NONE),
        BackgroundColor(Color::NONE),
        GlobalZIndex(10),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
    ));
}

fn game_scene_hand_area_min() -> Vec2 {
    Vec2::new(GAME_SCENE_HAND_LEFT, GAME_SCENE_HAND_TOP)
}

fn game_scene_hand_area_size() -> Vec2 {
    Vec2::new(GAME_SCENE_HAND_WIDTH, GAME_SCENE_HAND_HEIGHT)
}

fn game_scene_hand_card_size() -> Vec2 {
    Vec2::new(GAME_SCENE_HAND_CARD_WIDTH, GAME_SCENE_HAND_CARD_HEIGHT)
}

fn local_player_hand_deal_transform(card_defaults: &CardInspectionDefaults) -> Transform {
    let card_world_scale = game_scene_world_height_for_game_scene_height(
        GAME_SCENE_HAND_CARD_HEIGHT,
        GAME_SCENE_HAND_CARD_WORLD_Z,
    ) / card_defaults.height;

    Transform {
        translation: game_scene_world_position_from_game_scene(
            Vec2::new(GAME_SCENE_WIDTH * 0.5, GAME_SCENE_LOCAL_HAND_DEAL_SOURCE_Y),
            GAME_SCENE_HAND_CARD_WORLD_Z,
        ),
        scale: Vec3::splat(card_world_scale),
        ..Default::default()
    }
}

// HUMAN: Size and position hand cards using shared hand-area geometry.
// AI: Use a single source of truth for card height and group centering calculations.
fn spawn_game_scene_hand_cards(
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
    let deal_transform = local_player_hand_deal_transform(card_defaults);

    for (index, card_model) in card_models.into_iter().enumerate() {
        spawn_game_scene_hand_card(
            commands,
            asset_server,
            card_defaults,
            card_model,
            meshes,
            materials,
            masked_background_materials.as_deref_mut(),
            index,
            deal_transform,
        );
    }
}

fn spawn_game_scene_hand_card(
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
        false,
        transform,
    );
    commands
        .entity(card)
        .insert((
            GameSceneEntity,
            LocalPlayerHandCardPreview,
            HandCardGestureTarget::new(hand_index),
            SelectableCard::new(CardSelectionSource::LocalHand { hand_index }),
            CardGestureView,
        ))
        .observe(card_click_navigation);
}

/// HUMAN: Creates and removes rendered hand card entities as cards are dealt between rounds.
/// AI: GameHandModel is authoritative; this system reconciles spawned card roots to it.
pub fn sync_game_scene_hand_card_entities_system(
    mut commands: Commands,
    active_view: Option<Res<ActiveView>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    game_hand_model: Option<Res<GameHandModel>>,
    card_states: Option<Res<CardStateModel>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
    card_query: Query<(Entity, &HandCardGestureTarget), With<CardGestureView>>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        let _ = commands;
        return;
    }

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

    let deal_transform = local_player_hand_deal_transform(&card_defaults);

    for (hand_index, card_id) in game_hand_model.cards.iter().enumerate() {
        if existing_indices.contains(&hand_index) {
            continue;
        }
        if card_states
            .as_deref()
            .is_some_and(|states| states.state(hand_index) != Some(CardState::Hand))
        {
            continue;
        }
        let Some(card_model) = card_model_registry.card_model_for_id(card_id).cloned() else {
            continue;
        };
        spawn_game_scene_hand_card(
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
    _asset_server: &AssetServer,
    round_model: &GameRoundModel,
) {
    parent.spawn((
        Name::new("Match Status Text"),
        GameSceneEntity,
        MatchStatusText,
        Text::new("Status: Playing"),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(2.0),
            bottom: Val::Px(324.0),
            width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
            ..Default::default()
        },
        GlobalZIndex(10),
        Visibility::Visible,
    ));

    parent
        .spawn((
            Name::new("Game Control Left Menu"),
            GameSceneEntity,
            GlobalZIndex(10),
            Visibility::Visible,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(2.0),
                bottom: Val::Px(36.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                row_gap: Val::Px(14.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonUiBundle::new("Quit Game Button")
                        .with_node(Node {
                            width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                            height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                            border: UiRect::all(Val::Px(3.0)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        })
                        .with_colors(
                            END_ROUND_BUTTON_NORMAL_COLOR,
                            END_ROUND_BUTTON_NORMAL_BORDER_COLOR,
                        ),
                    GameSceneEntity,
                    GameControlButton::new(GameControlAction::QuitGame),
                    GlobalZIndex(10),
                    Visibility::Visible,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit Game"),
                        TextFont {
                            font_size: 22.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            parent
                .spawn((
                    ButtonUiBundle::new("Restart Button")
                        .with_node(Node {
                            width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                            height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                            border: UiRect::all(Val::Px(3.0)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        })
                        .with_colors(
                            END_ROUND_BUTTON_NORMAL_COLOR,
                            END_ROUND_BUTTON_NORMAL_BORDER_COLOR,
                        ),
                    GameSceneEntity,
                    GameControlButton::new(GameControlAction::Restart),
                    GlobalZIndex(10),
                    Visibility::Visible,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Restart"),
                        TextFont {
                            font_size: 22.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
    parent
        .spawn((
            Name::new("Game Control Right Menu"),
            GameSceneEntity,
            GlobalZIndex(10),
            Visibility::Visible,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Percent(2.0),
                bottom: Val::Px(36.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                row_gap: Val::Px(14.0),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonUiBundle::new("Undo Button")
                        .with_node(Node {
                            width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                            height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                            border: UiRect::all(Val::Px(3.0)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        })
                        .with_colors(
                            if round_model.has_undoable_moves() {
                                END_ROUND_BUTTON_NORMAL_COLOR
                            } else {
                                GAME_CONTROL_DISABLED_COLOR
                            },
                            if round_model.has_undoable_moves() {
                                END_ROUND_BUTTON_NORMAL_BORDER_COLOR
                            } else {
                                GAME_CONTROL_DISABLED_BORDER_COLOR
                            },
                        ),
                    GameSceneEntity,
                    GameControlButton::new(GameControlAction::Undo),
                    GlobalZIndex(10),
                    Visibility::Visible,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(round_model.energy_label()),
                        TextFont {
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        GameControlLabel::new(GameControlAction::Undo),
                    ));
                    parent.spawn((
                        Text::new("Undo"),
                        TextFont {
                            font_size: 22.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            parent
                .spawn((
                    ButtonUiBundle::new("RoundUI")
                        .with_node(Node {
                            width: Val::Px(GAME_CONTROL_BUTTON_WIDTH),
                            height: Val::Px(GAME_CONTROL_BUTTON_HEIGHT),
                            border: UiRect::all(Val::Px(3.0)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        })
                        .with_colors(
                            END_ROUND_BUTTON_NORMAL_COLOR,
                            END_ROUND_BUTTON_NORMAL_BORDER_COLOR,
                        ),
                    RoundUi,
                    GameSceneEntity,
                    GameControlButton::new(GameControlAction::EndRound),
                    GlobalZIndex(10),
                    Visibility::Visible,
                    EndRoundButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(format!(
                            "Round {}/{}",
                            round_model.round, round_model.max_rounds
                        )),
                        TextFont {
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        GameControlLabel::new(GameControlAction::EndRound),
                    ));
                    parent.spawn((
                        Text::new("Next"),
                        TextFont {
                            font_size: 24.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// HUMAN: Refreshes SettingsScreen button labels from the latest settings resource values.
/// AI: Keeps text in sync when toggles mutate MetaGameSettingsModel without rebuilding the scene.
pub fn refresh_settings_button_labels_system(
    settings: Res<MetaGameSettingsModel>,
    mut labels: Query<(&SettingsButtonLabel, &mut Text)>,
) {
    for (label, mut text) in &mut labels {
        let next_text = match label.action {
            MetaScreenButtonAction::CpuBrain => {
                format!("CPU AI Brain: {}", settings.cpu_brain_level.label())
            }
            MetaScreenButtonAction::MatchMode => {
                format!("Mode: {}", settings.selected_mode.label())
            }
            MetaScreenButtonAction::CycleFramerate => {
                format!("Framerate: {}", settings.framerate_label())
            }
            MetaScreenButtonAction::ToggleSfx => {
                format!(
                    "SFX: {}",
                    MetaGameSettingsModel::audio_label(settings.sfx_enabled)
                )
            }
            MetaScreenButtonAction::ToggleMusic => {
                format!(
                    "Music: {}",
                    MetaGameSettingsModel::audio_label(settings.music_enabled)
                )
            }
            MetaScreenButtonAction::CycleQuality => "Quality: Med".to_string(),
            _ => continue,
        };
        if text.0 != next_text {
            text.0 = next_text;
        }
    }
}

fn spawn_deck_light(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("DeckScene Key Light"),
            DeckSceneEntity,
            DirectionalLight {
                illuminance: 1500.0,
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, -0.35, 0.0)),
        ))
        .id()
}

fn spawn_debug_light(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("DebugScene Key Light"),
            DebugSceneEntity,
            DirectionalLight {
                illuminance: 1500.0,
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, -0.35, 0.0)),
        ))
        .id()
}

/// HUMAN: Spawns the deck sub-screen view.
/// AI: DeckScene keeps the centered card preview separate from the deck list UI.
pub fn setup_deck_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_defaults: Res<PrimaryCameraDefaults>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    active_card_model: Res<ActiveCardModel>,
    player_deck_collection: Option<Res<PlayerDeckCollectionModel>>,
    mut deck_screen_model: Option<ResMut<DeckScreenModel>>,
    mut top_navigation_model: Option<ResMut<TopNavigationModel>>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
) {
    let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
    let player_deck_collection = player_deck_collection
        .as_deref()
        .unwrap_or(&fallback_player_deck_collection);
    if let Some(top_navigation_model) = top_navigation_model.as_deref_mut() {
        top_navigation_model.selected = TopNavigationDestination::MyDecks;
    }
    if let Some(deck_screen_model) = deck_screen_model.as_deref_mut() {
        deck_screen_model.needs_rebuild = false;
    }
    let app_camera = app_camera_query.iter().next().unwrap_or_else(|| {
        ensure_shared_app_camera(&mut commands, app_scene_query.single().ok(), None)
    });
    spawn_deck_scene_contents(
        &mut commands,
        app_camera,
        &asset_server,
        &camera_defaults,
        &card_defaults,
        &card_model_registry,
        &active_card_model,
        &player_deck_collection,
        deck_screen_model.as_deref(),
        &mut meshes,
        &mut materials,
        masked_background_materials.map(|materials| materials.into_inner()),
        app_scene_query.single().ok(),
        CardFace::Front,
        Quat::IDENTITY,
    );
}

fn spawn_deck_scene_contents(
    commands: &mut Commands,
    app_camera: Entity,
    asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    _active_card_model: &ActiveCardModel,
    player_deck_collection: &PlayerDeckCollectionModel,
    deck_screen_model: Option<&DeckScreenModel>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mut masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    app_scene_parent: Option<Entity>,
    visible_face: CardFace,
    _initial_rotation: Quat,
) {
    let scene_root = commands.spawn(DeckScreenBundle::default()).id();
    let ui_camera = app_camera;
    let _ = camera_defaults;
    let point_text_camera = spawn_card_point_text_camera(
        commands,
        "DeckScene Card Point Text Camera",
        DeckSceneEntity,
    );
    let light = spawn_deck_light(commands);
    let mode = deck_screen_model.map_or(Default::default(), |model| model.mode);
    let tab = deck_screen_model.map_or(Default::default(), |model| model.editor_tab);
    let modal = deck_screen_model.and_then(|model| model.modal.as_ref());
    let prompt_is_open = deck_screen_model
        .as_ref()
        .is_some_and(|model| model.validation_prompt || model.coming_soon_prompt);

    commands.entity(scene_root).insert((
        UiTargetCamera(ui_camera),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
    ));
    commands.entity(scene_root).with_children(|parent| {
        spawn_top_navigation_view(
            parent,
            ui_camera,
            TopNavigationDestination::MyDecks,
            prompt_is_open,
        );
        spawn_deck_screen_content(
            parent,
            ui_camera,
            asset_server,
            card_model_registry,
            player_deck_collection,
            mode,
            tab,
            modal,
        );
        if let Some(deck_screen_model) = deck_screen_model {
            if deck_screen_model.validation_prompt {
                spawn_deck_screen_prompt(
                    parent,
                    ui_camera,
                    DECK_SCREEN_VALIDATION_TITLE,
                    DECK_SCREEN_VALIDATION_MESSAGE,
                );
            } else if deck_screen_model.coming_soon_prompt {
                spawn_deck_screen_prompt(
                    parent,
                    ui_camera,
                    DECK_SCREEN_COMING_SOON_TITLE,
                    DECK_SCREEN_COMING_SOON_MESSAGE,
                );
            }
        }
    });
    if mode == crate::runtime::resources::DeckScreenMode::Editor {
        let deck_cards = deck_screen_deck_cards(player_deck_collection);
        let library_cards = deck_screen_library_cards(&deck_cards);
        spawn_deck_screen_grid_backdrops(commands, meshes, materials, tab, scene_root);
        spawn_deck_screen_card_views(
            commands,
            asset_server,
            card_defaults,
            card_model_registry,
            &deck_cards,
            &library_cards,
            tab,
            meshes,
            materials,
            &mut masked_background_materials,
            visible_face,
            scene_root,
        );
    }
    commands.entity(scene_root).add_child(point_text_camera);
    commands.entity(scene_root).add_child(light);
    if let Some(parent) = app_scene_parent {
        commands.entity(parent).add_child(scene_root);
    }
}

fn spawn_top_navigation_view(
    parent: &mut ChildSpawnerCommands,
    ui_camera: Entity,
    selected: TopNavigationDestination,
    is_blocked: bool,
) {
    parent
        .spawn((
            TopNavigationViewBundle::default(),
            UiTargetCamera(ui_camera),
        ))
        .with_children(|parent| {
            for destination in TopNavigationDestination::all() {
                if destination == TopNavigationDestination::Settings {
                    parent.spawn((
                        Name::new("TopNav Divider"),
                        Text::new("|"),
                        TextFont {
                            font_size: 34.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.62)),
                    ));
                }
                let is_selected = destination == selected;
                parent
                    .spawn((
                        ButtonUiBundle::new(format!("TopNav {}", destination.label()))
                            .with_node(Node {
                                width: Val::Px(150.0),
                                height: Val::Px(46.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            })
                            .with_colors(
                                if is_selected {
                                    Color::srgb(0.18, 0.38, 0.58)
                                } else if is_blocked {
                                    Color::srgba(0.12, 0.14, 0.18, 0.65)
                                } else {
                                    Color::srgb(0.18, 0.22, 0.29)
                                },
                                if is_selected {
                                    Color::srgb(0.56, 0.78, 1.0)
                                } else {
                                    Color::srgb(0.43, 0.47, 0.56)
                                },
                            ),
                        TopNavigationButton::new(destination),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(destination.label()),
                            TextFont {
                                font_size: 18.0,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

fn spawn_deck_screen_content(
    parent: &mut ChildSpawnerCommands,
    ui_camera: Entity,
    asset_server: &AssetServer,
    card_model_registry: &CardModelRegistry,
    player_deck_collection: &PlayerDeckCollectionModel,
    mode: crate::runtime::resources::DeckScreenMode,
    tab: DeckEditorTabModel,
    modal: Option<&crate::runtime::resources::DeckScreenCardModalModel>,
) {
    let _ = ui_camera;
    match mode {
        crate::runtime::resources::DeckScreenMode::DeckSelection => {
            spawn_deck_selection(parent, asset_server);
        }
        crate::runtime::resources::DeckScreenMode::Editor => {
            spawn_deck_editor(
                parent,
                card_model_registry,
                player_deck_collection,
                tab,
                modal,
            );
        }
    }
}

fn spawn_deck_selection(parent: &mut ChildSpawnerCommands, asset_server: &AssetServer) {
    let deck_slots = vec![
        "+".to_string(),
        DECK_SCREEN_DECK_NAME.to_string(),
        String::new(),
        String::new(),
    ];
    spawn_grid_view_ui_bundle(
        parent,
        "My Decks",
        DeckEditableZoneModel::Deck,
        false,
        &deck_slots,
        2,
        2,
        None,
        |parent, zone, deck_name, index| match deck_name.as_str() {
            "+" => spawn_deck_selection_new_deck_tile(parent),
            name if name == DECK_SCREEN_DECK_NAME => {
                spawn_deck_selection_existing_deck_tile(parent, asset_server, zone, index)
            }
            _ => spawn_deck_selection_empty_tile(parent),
        },
    );
}

fn spawn_deck_editor(
    parent: &mut ChildSpawnerCommands,
    card_model_registry: &CardModelRegistry,
    player_deck_collection: &PlayerDeckCollectionModel,
    _tab: DeckEditorTabModel,
    _modal: Option<&crate::runtime::resources::DeckScreenCardModalModel>,
) {
    let deck_cards = deck_screen_deck_cards(player_deck_collection);
    let library_cards = deck_screen_library_cards(&deck_cards);
    spawn_grid_view_ui_bundle(
        parent,
        "Deck 01",
        DeckEditableZoneModel::Deck,
        false,
        &deck_cards,
        4,
        3,
        None,
        |parent, zone, card_id, index| {
            spawn_deck_screen_card_tile(parent, card_model_registry, card_id, zone, index);
        },
    );
    spawn_grid_view_ui_bundle(
        parent,
        "Not In Deck",
        DeckEditableZoneModel::Library,
        true,
        &library_cards,
        4,
        3,
        None,
        |parent, zone, card_id, index| {
            spawn_deck_screen_card_tile(parent, card_model_registry, card_id, zone, index);
        },
    );
}

/// HUMAN: Renders matching card-grid panel frames behind DeckScreen cards.
/// AI: These planes sit below CardViewBundle roots so panel borders cannot cover cards.
fn spawn_deck_screen_grid_backdrops(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    _tab: DeckEditorTabModel,
    scene_root: Entity,
) {
    spawn_deck_screen_grid_backdrop_for_zone(
        commands,
        meshes,
        materials,
        DeckEditableZoneModel::Deck,
        scene_root,
    );
    spawn_deck_screen_grid_backdrop_for_zone(
        commands,
        meshes,
        materials,
        DeckEditableZoneModel::Library,
        scene_root,
    );
}

fn spawn_deck_screen_grid_backdrop_for_zone(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    zone: DeckEditableZoneModel,
    scene_root: Entity,
) {
    spawn_deck_screen_grid_backdrop_rect(
        commands,
        meshes,
        materials,
        zone,
        DeckScreenGridBackdropRole::Fill,
        deck_screen_grid_backdrop_center(zone),
        Vec2::new(DECK_SCREEN_GRID_PANEL_WIDTH, DECK_SCREEN_GRID_PANEL_HEIGHT),
        Color::srgba(0.08, 0.1, 0.14, 0.55),
        scene_root,
    );

    let left = deck_screen_grid_left(zone);
    let right = left + DECK_SCREEN_GRID_PANEL_WIDTH;
    let top = DECK_SCREEN_DECK_GRID_TOP;
    let bottom = top + DECK_SCREEN_GRID_PANEL_HEIGHT;
    let center_x = left + (DECK_SCREEN_GRID_PANEL_WIDTH * 0.5);
    let center_y = top + (DECK_SCREEN_GRID_PANEL_HEIGHT * 0.5);
    let border_color = Color::srgba(0.87, 0.9, 0.95, 0.26);

    for (role, center, size) in [
        (
            DeckScreenGridBackdropRole::Top,
            Vec2::new(center_x, top),
            Vec2::new(
                DECK_SCREEN_GRID_PANEL_WIDTH,
                DECK_SCREEN_GRID_BORDER_THICKNESS,
            ),
        ),
        (
            DeckScreenGridBackdropRole::Bottom,
            Vec2::new(center_x, bottom),
            Vec2::new(
                DECK_SCREEN_GRID_PANEL_WIDTH,
                DECK_SCREEN_GRID_BORDER_THICKNESS,
            ),
        ),
        (
            DeckScreenGridBackdropRole::Left,
            Vec2::new(left, center_y),
            Vec2::new(
                DECK_SCREEN_GRID_BORDER_THICKNESS,
                DECK_SCREEN_GRID_PANEL_HEIGHT,
            ),
        ),
        (
            DeckScreenGridBackdropRole::Right,
            Vec2::new(right, center_y),
            Vec2::new(
                DECK_SCREEN_GRID_BORDER_THICKNESS,
                DECK_SCREEN_GRID_PANEL_HEIGHT,
            ),
        ),
    ] {
        spawn_deck_screen_grid_backdrop_rect(
            commands,
            meshes,
            materials,
            zone,
            role,
            center,
            size,
            border_color,
            scene_root,
        );
    }
}

fn spawn_deck_screen_grid_backdrop_rect(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    zone: DeckEditableZoneModel,
    role: DeckScreenGridBackdropRole,
    center: Vec2,
    size: Vec2,
    color: Color,
    scene_root: Entity,
) {
    let world_size = Vec2::new(
        game_scene_world_width_for_game_scene_width(size.x, DECK_SCREEN_GRID_BACKDROP_WORLD_Z),
        game_scene_world_height_for_game_scene_height(size.y, DECK_SCREEN_GRID_BACKDROP_WORLD_Z),
    );
    let entity = commands
        .spawn((
            Name::new(format!("DeckScreen {zone:?} Grid {role:?}")),
            DeckSceneEntity,
            DeckScreenGridBackdrop::new(zone, role),
            Mesh3d(meshes.add(Rectangle::new(world_size.x, world_size.y))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..Default::default()
            })),
            Transform::from_translation(game_scene_world_position_from_game_scene(
                center,
                DECK_SCREEN_GRID_BACKDROP_WORLD_Z,
            )),
            RenderLayers::layer(CARD_RENDER_LAYER),
            Visibility::Visible,
            NoCpuCulling,
            NoFrustumCulling,
            Pickable::IGNORE,
        ))
        .id();
    commands.entity(scene_root).add_child(entity);
}

fn deck_screen_grid_left(zone: DeckEditableZoneModel) -> f32 {
    match zone {
        DeckEditableZoneModel::Deck => DECK_SCREEN_DECK_GRID_LEFT,
        DeckEditableZoneModel::Library | DeckEditableZoneModel::Shop => {
            DECK_SCREEN_LIBRARY_GRID_LEFT
        }
    }
}

fn deck_screen_grid_backdrop_center(zone: DeckEditableZoneModel) -> Vec2 {
    Vec2::new(
        deck_screen_grid_left(zone) + (DECK_SCREEN_GRID_PANEL_WIDTH * 0.5),
        DECK_SCREEN_DECK_GRID_TOP + (DECK_SCREEN_GRID_PANEL_HEIGHT * 0.5),
    )
}

/// HUMAN: Renders editor cards as full CardViewBundle visuals in their grids.
/// AI: UI card tiles stay as transparent hit targets while these world entities provide presentation.
fn spawn_deck_screen_card_views(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    deck_cards: &[String],
    library_cards: &[String],
    tab: DeckEditorTabModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: &mut Option<&mut Assets<CardBackgroundMaskMaterial>>,
    visible_face: CardFace,
    scene_root: Entity,
) {
    spawn_deck_screen_card_views_for_zone(
        commands,
        asset_server,
        card_defaults,
        card_model_registry,
        deck_cards,
        DeckEditableZoneModel::Deck,
        meshes,
        materials,
        masked_background_materials,
        visible_face,
        scene_root,
    );
    if tab == DeckEditorTabModel::Library {
        spawn_deck_screen_card_views_for_zone(
            commands,
            asset_server,
            card_defaults,
            card_model_registry,
            library_cards,
            DeckEditableZoneModel::Library,
            meshes,
            materials,
            masked_background_materials,
            visible_face,
            scene_root,
        );
    }
}

fn spawn_deck_screen_card_views_for_zone(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    cards: &[String],
    zone: DeckEditableZoneModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: &mut Option<&mut Assets<CardBackgroundMaskMaterial>>,
    visible_face: CardFace,
    scene_root: Entity,
) {
    for (index, card_id) in cards.iter().enumerate() {
        let Some(card_model) = card_model_registry.card_model_for_id(card_id).cloned() else {
            continue;
        };
        let card = spawn_card_structure_for_type(
            commands,
            asset_server,
            card_defaults,
            card_model,
            meshes,
            materials,
            masked_background_materials.as_deref_mut(),
            visible_face,
            false,
            deck_screen_card_view_transform(card_defaults, zone, index),
        );
        commands.entity(card).insert((
            DeckSceneEntity,
            DeckScreenCardView::new(card_id.clone(), zone, index),
            SelectableCard::new(CardSelectionSource::ScreenCard {
                view: ActiveView::DeckScene,
            }),
        ));
        commands.entity(card).observe(card_click_selection);
        commands.entity(scene_root).add_child(card);
    }
}

fn deck_screen_card_view_transform(
    card_defaults: &CardInspectionDefaults,
    zone: DeckEditableZoneModel,
    index: usize,
) -> Transform {
    let column = (index % 4) as f32;
    let row = (index / 4) as f32;
    let grid_left = deck_screen_grid_left(zone);
    let card_center = Vec2::new(
        grid_left
            + (DECK_SCREEN_DECK_GRID_COLUMN_WIDTH * (column + 0.5))
            + (DECK_SCREEN_DECK_GRID_COLUMN_GAP * column),
        DECK_SCREEN_DECK_GRID_TOP
            + (DECK_SCREEN_DECK_GRID_ROW_HEIGHT * (row + 0.5))
            + (DECK_SCREEN_DECK_GRID_ROW_GAP * row),
    );
    let scale = game_scene_world_height_for_game_scene_height(
        DECK_SCREEN_DECK_CARD_HEIGHT,
        DECK_SCREEN_DECK_CARD_WORLD_Z,
    ) / card_defaults.height;

    Transform {
        translation: game_scene_world_position_from_game_scene(
            card_center,
            DECK_SCREEN_DECK_CARD_WORLD_Z,
        ),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(scale),
    }
}

/// HUMAN: Spawns the shared DeckScreen titled grid panel UI.
/// AI: This reuses `GridViewUiBundle` and accepts a pluggable grid item renderer.
fn spawn_grid_view_ui_bundle(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    zone: DeckEditableZoneModel,
    show_empty_state: bool,
    cards: &[String],
    grid_columns: u16,
    grid_rows: u16,
    _modal: Option<&crate::runtime::resources::DeckScreenCardModalModel>,
    mut spawn_grid_item: impl FnMut(&mut ChildSpawnerCommands, DeckEditableZoneModel, &String, usize),
) {
    parent
        .spawn(GridViewUiBundle::new(
            title,
            zone,
            deck_screen_grid_left(zone),
            DECK_SCREEN_GRID_TITLE_TOP,
            DECK_SCREEN_GRID_PANEL_WIDTH,
            Val::Px(
                DECK_SCREEN_DECK_GRID_TOP
                    - DECK_SCREEN_GRID_TITLE_TOP
                    - DECK_SCREEN_GRID_TITLE_HEIGHT,
            ),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new(format!("DeckScreen {title} Menu")),
                    GridViewMenuArea,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(-DECK_SCREEN_GRID_MENU_HEIGHT - 8.0),
                        width: Val::Px(DECK_SCREEN_GRID_PANEL_WIDTH),
                        height: Val::Px(DECK_SCREEN_GRID_MENU_HEIGHT),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..Default::default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    if zone == DeckEditableZoneModel::Deck && title == DECK_SCREEN_DECK_NAME {
                        spawn_deck_command_button(
                            parent,
                            DeckScreenDeckCommandButton::EditDeckName,
                            "Edit Deck Name",
                        );
                        spawn_deck_command_button(
                            parent,
                            DeckScreenDeckCommandButton::DeleteDeck,
                            "Delete Deck",
                        );
                    } else if zone == DeckEditableZoneModel::Library {
                        spawn_deck_library_menu_button(
                            parent,
                            DeckEditorTabModel::Library,
                            "Library",
                            true,
                        );
                        spawn_deck_library_menu_button(
                            parent,
                            DeckEditorTabModel::Shop,
                            "Shop",
                            false,
                        );
                    }
                });
            parent.spawn((
                Text::new(title.to_string()),
                GridViewTitleArea,
                TextFont {
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
                Node {
                    position_type: PositionType::Relative,
                    left: Val::Px(DECK_SCREEN_GRID_TITLE_OFFSET_X),
                    top: Val::Px(DECK_SCREEN_GRID_TITLE_OFFSET_Y),
                    width: Val::Px(DECK_SCREEN_GRID_TITLE_WIDTH),
                    height: Val::Px(DECK_SCREEN_GRID_TITLE_HEIGHT),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
            ));
            parent
                .spawn((
                    Name::new(format!("DeckScreen {title} Grid")),
                    GridViewContentArea,
                    Node {
                        width: Val::Px(DECK_SCREEN_GRID_PANEL_WIDTH),
                        height: Val::Px(DECK_SCREEN_GRID_PANEL_HEIGHT),
                        display: Display::Grid,
                        grid_template_columns: vec![RepeatedGridTrack::flex(grid_columns, 1.0)],
                        grid_template_rows: vec![RepeatedGridTrack::flex(grid_rows, 1.0)],
                        row_gap: Val::Px(DECK_SCREEN_DECK_GRID_ROW_GAP),
                        column_gap: Val::Px(DECK_SCREEN_DECK_GRID_COLUMN_GAP),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderColor::all(Color::NONE),
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    if cards.is_empty() && show_empty_state {
                        parent.spawn((
                            Text::new("Empty"),
                            TextFont {
                                font_size: 24.0,
                                ..Default::default()
                            },
                            TextColor(Color::srgb(0.68, 0.72, 0.78)),
                            Pickable::IGNORE,
                        ));
                    }
                    for (index, card_id) in cards.iter().enumerate() {
                        spawn_grid_item(parent, zone, card_id, index);
                    }
                });
        });
}

fn spawn_deck_command_button(
    parent: &mut ChildSpawnerCommands,
    command: DeckScreenDeckCommandButton,
    label: &'static str,
) {
    parent
        .spawn((
            ButtonUiBundle::new(format!("DeckScreen Command {label}")).with_node(Node {
                width: Val::Px(DECK_SCREEN_DECK_COMMAND_WIDTH),
                height: Val::Px(DECK_SCREEN_DECK_COMMAND_HEIGHT),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            }),
            command,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_deck_library_menu_button(
    parent: &mut ChildSpawnerCommands,
    tab: DeckEditorTabModel,
    label: &'static str,
    is_toggled_on: bool,
) {
    let background = if is_toggled_on {
        Color::srgb(0.32, 0.38, 0.5)
    } else {
        Color::srgb(0.18, 0.22, 0.29)
    };
    let border = if is_toggled_on {
        Color::srgb(0.82, 0.87, 0.96)
    } else {
        Color::srgb(0.43, 0.47, 0.56)
    };
    parent
        .spawn((
            ButtonUiBundle::new(format!("DeckScreen Library Command {label}"))
                .with_node(Node {
                    width: Val::Px(DECK_SCREEN_DECK_COMMAND_WIDTH),
                    height: Val::Px(DECK_SCREEN_DECK_COMMAND_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_colors(background, border),
            DeckScreenTabButton::new(tab),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_deck_selection_new_deck_tile(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            ButtonUiBundle::new("DeckScreen + Deck Tile")
                .with_node(Node {
                    width: Val::Px(DECK_VIEW_TILE_WIDTH),
                    height: Val::Px(DECK_VIEW_TILE_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..Default::default()
                })
                .with_colors(
                    Color::srgba(1.0, 1.0, 1.0, 0.06),
                    Color::srgba(0.85, 0.88, 0.94, 0.72),
                ),
            DeckScreenDeckCommandButton::EditDeckName,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("+"),
                TextFont {
                    font_size: 52.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("New Deck"),
                TextFont {
                    font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_deck_selection_existing_deck_tile(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    _zone: DeckEditableZoneModel,
    _index: usize,
) {
    parent
        .spawn((
            DeckViewBundle::new(DECK_SCREEN_DECK_NAME),
            DeckScreenDeckTileButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(asset_server.load(CARD_BACK_TEXTURE_PATH))
                    .with_mode(bevy::ui::widget::NodeImageMode::Stretch),
                Node {
                    width: Val::Percent(78.0),
                    height: Val::Percent(78.0),
                    ..Default::default()
                },
            ));
            parent.spawn((
                Text::new(DECK_SCREEN_DECK_NAME),
                TextFont {
                    font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_deck_selection_empty_tile(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Name::new("DeckScreen Empty Slot"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        BorderColor::all(Color::NONE),
        Pickable::IGNORE,
    ));
}

fn spawn_deck_screen_card_tile(
    parent: &mut ChildSpawnerCommands,
    card_model_registry: &CardModelRegistry,
    card_id: &str,
    _zone: DeckEditableZoneModel,
    _index: usize,
) {
    let card_label = card_model_registry
        .card_model_for_id(card_id)
        .map_or(card_id, |card| card.display_name);
    parent.spawn((
        Name::new(format!("DeckScreen Card {card_label}")),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(8.0)),
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        BorderColor::all(Color::NONE),
        Pickable::IGNORE,
    ));
}

/// HUMAN: Spawns the right-side action menu for the selected DeckScreen card.
/// AI: Actions are the only way to clear screen-local card selection.
fn spawn_deck_screen_selected_card_menu(
    parent: &mut ChildSpawnerCommands,
    ui_camera: Entity,
    modal: &crate::runtime::resources::DeckScreenCardModalModel,
) {
    parent
        .spawn((
            Name::new("DeckScreen Selected Card Menu"),
            DeckScreenSelectedCardMenuRoot,
            UiTargetCamera(ui_camera),
            GlobalZIndex(700),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(DECK_SCREEN_SELECTED_CARD_MENU_LEFT),
                top: Val::Px(DECK_SCREEN_SELECTED_CARD_MENU_TOP),
                width: Val::Px(DECK_SCREEN_SELECTED_CARD_MENU_WIDTH),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.06, 0.07, 0.09)),
            BorderColor::all(Color::srgb(0.58, 0.64, 0.76)),
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            spawn_deck_screen_selected_card_menu_button(
                parent,
                DeckScreenModalActionButton::MoveToLibrary,
                "Move to Library",
                modal.actions.move_to_library,
            );
            spawn_deck_screen_selected_card_menu_button(
                parent,
                DeckScreenModalActionButton::MoveToDeck,
                "Move to Deck",
                modal.actions.move_to_deck,
            );
            spawn_deck_screen_selected_card_menu_button(
                parent,
                DeckScreenModalActionButton::TransferOut,
                "Transfer",
                true,
            );
            spawn_deck_screen_selected_card_menu_button(
                parent,
                DeckScreenModalActionButton::Back,
                "Back",
                modal.actions.back,
            );
        });
}

fn spawn_deck_screen_selected_card_menu_button(
    parent: &mut ChildSpawnerCommands,
    action: DeckScreenModalActionButton,
    label: &'static str,
    is_primary_action: bool,
) {
    let background = if is_primary_action {
        Color::srgb(0.24, 0.28, 0.36)
    } else {
        Color::srgb(0.14, 0.16, 0.21)
    };
    let border = if is_primary_action {
        Color::srgb(0.68, 0.74, 0.86)
    } else {
        Color::srgb(0.36, 0.4, 0.5)
    };
    parent
        .spawn((
            ButtonUiBundle::new(format!("DeckScreen Selected Card Menu {label}"))
                .with_node(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(DECK_SCREEN_SELECTED_CARD_MENU_BUTTON_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_colors(background, border),
            action,
            Pickable::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
        });
}

fn spawn_deck_screen_prompt(
    parent: &mut ChildSpawnerCommands,
    ui_camera: Entity,
    title: &'static str,
    body: &'static str,
) {
    parent
        .spawn((
            ModalUiBundle::new("DeckScreen Prompt Modal", ui_camera),
            DeckScreenModalRoot,
            Pickable::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn(ModalPromptUiBundle::new("DeckScreen Prompt"))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(title),
                        TextFont {
                            font_size: 26.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                        Pickable::IGNORE,
                    ));
                    parent.spawn((
                        Text::new(body),
                        TextFont {
                            font_size: 18.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.86, 0.88, 0.92)),
                        Pickable::IGNORE,
                    ));
                    parent
                        .spawn(ModalMenuUiBundle::new("DeckScreen Prompt Menu"))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ModalButtonUiBundle::new("DeckScreen Prompt OK"),
                                    DeckScreenValidationOkButton,
                                    Pickable::default(),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("OK"),
                                        TextFont {
                                            font_size: 21.0,
                                            ..Default::default()
                                        },
                                        TextColor(Color::WHITE),
                                        Pickable::IGNORE,
                                    ));
                                });
                        });
                });
        });
}

/// HUMAN: Handles reusable top-navigation button activation.
/// AI: DeckScreen card action menus live inside grid panels, while validation prompts block navigation.
pub fn top_navigation_update_system(
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    mut deck_screen_model: Option<ResMut<DeckScreenModel>>,
    selected_card_modal: Option<Res<SelectedCardModalModel>>,
    player_deck_collection: Option<Res<PlayerDeckCollectionModel>>,
    settings: Res<MetaGameSettingsModel>,
    screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut top_navigation_model: ResMut<TopNavigationModel>,
    active_card_model: Res<ActiveCardModel>,
    flip_state: Res<CardFlipState>,
    mut params: ViewChangeParams,
    mut button_query: Query<(&Interaction, &TopNavigationButton), Changed<Interaction>>,
) {
    let mut screen_transition = screen_transition;
    if deck_screen_model
        .as_ref()
        .is_some_and(|model| model.validation_prompt || model.coming_soon_prompt)
    {
        return;
    }
    if *params.active_view != ActiveView::DeckScene
        && selected_card_modal
            .as_ref()
            .is_some_and(|model| model.is_active() || model.press_candidate.is_some())
    {
        return;
    }

    for (interaction, button) in &mut button_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let destination_view = match button.destination {
            TopNavigationDestination::PlayGame => ActiveView::MainMenuScene,
            TopNavigationDestination::MyDecks => ActiveView::DeckScene,
            TopNavigationDestination::Settings => ActiveView::SettingsScene,
            TopNavigationDestination::Debug => ActiveView::DebugScene,
        };
        if *params.active_view == destination_view {
            continue;
        }

        if *params.active_view == ActiveView::DeckScene
            && button.destination != TopNavigationDestination::MyDecks
            && player_deck_collection.as_ref().is_some_and(|collection| {
                deck_screen_deck_cards(collection).len() != DECK_SCREEN_CARD_COUNT
            })
        {
            if let Some(deck_screen_model) = deck_screen_model.as_deref_mut() {
                deck_screen_model.show_validation_prompt();
            }
            continue;
        }

        request_button_click(audio_manager.as_deref_mut());
        let initial_rotation =
            composed_rotation_for_face(&params.card_state, flip_state.visible_face);
        match button.destination {
            TopNavigationDestination::PlayGame => {
                top_navigation_model.selected = TopNavigationDestination::PlayGame;
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::MainMenuScene);
                } else {
                    params.transition_to_main_menu_scene();
                }
            }
            TopNavigationDestination::MyDecks => {
                top_navigation_model.selected = TopNavigationDestination::MyDecks;
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::DeckScene);
                } else {
                    params.transition_to_deck_scene(
                        &active_card_model,
                        flip_state.visible_face,
                        initial_rotation,
                    );
                }
            }
            TopNavigationDestination::Settings => {
                top_navigation_model.selected = TopNavigationDestination::Settings;
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::SettingsScene);
                } else {
                    params.transition_to_settings_scene(&settings);
                }
            }
            TopNavigationDestination::Debug => {
                top_navigation_model.selected = TopNavigationDestination::Debug;
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::DebugScene);
                } else {
                    params.transition_to_debug_scene(
                        &active_card_model,
                        flip_state.visible_face,
                        initial_rotation,
                    );
                }
            }
        }
    }
}

/// HUMAN: Handles buttons that belong to Main, Lightning, Matchmaking, and Settings screens.
/// AI: Keep Lightning integration as placeholder navigation until the real backend exists.
pub fn meta_screen_update_system(
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    mut settings: ResMut<MetaGameSettingsModel>,
    mut persistent_settings: Option<ResMut<Persistent<MetaGameSettingsModel>>>,
    mut persistent_match_mode: Option<ResMut<Persistent<MatchModePreferenceStore>>>,
    active_card_model: Res<ActiveCardModel>,
    #[cfg(not(target_arch = "wasm32"))] mut winit_settings: Option<ResMut<WinitSettings>>,
    mut params: ViewChangeParams,
    screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut button_query: Query<(&Interaction, &MetaScreenButton), Changed<Interaction>>,
) {
    let mut screen_transition = screen_transition;
    for (interaction, button) in &mut button_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        request_button_click(audio_manager.as_deref_mut());
        match button.action {
            MetaScreenButtonAction::LightningLogin => {
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::LightningScene);
                } else {
                    params.transition_to_lightning_login_scene();
                }
            }
            MetaScreenButtonAction::MatchmakingBack => {
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::MainMenuScene);
                } else {
                    params.transition_to_main_menu_scene();
                }
            }
            MetaScreenButtonAction::LearnLightning => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Err(error) = webbrowser::open("https://bitbo.io/tools/lightning-wallets/") {
                    warn!("Failed to open Lightning information page: {error}");
                }
            }
            MetaScreenButtonAction::StartGame => {
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::MatchmakingScene);
                } else {
                    params.transition_to_matchmaking_scene();
                }
            }
            MetaScreenButtonAction::CpuBrain => {
                settings.cycle_cpu_brain_level();
                save_meta_game_settings(&settings, persistent_settings.as_deref_mut());
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::SettingsScene);
                } else {
                    params.transition_to_settings_scene(&settings);
                }
            }
            MetaScreenButtonAction::MatchMode => {
                settings.toggle_mode();
                if let Some(match_model) = params.match_model.as_deref_mut() {
                    match_model.mode = settings.selected_mode;
                }
                if let Some(persistent_match_mode) = persistent_match_mode.as_deref_mut()
                    && let Err(error) = persistent_match_mode.set(MatchModePreferenceStore {
                        selected_mode: settings.selected_mode,
                    })
                {
                    warn!("Failed to save match mode preference: {error}");
                }
                save_meta_game_settings(&settings, persistent_settings.as_deref_mut());
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::SettingsScene);
                } else {
                    params.transition_to_settings_scene(&settings);
                }
            }
            MetaScreenButtonAction::CycleFramerate => {
                settings.toggle_framerate();
                if let Some(winit_settings) = winit_settings.as_deref_mut() {
                    apply_meta_game_framerate_settings(&settings, winit_settings);
                }
                save_meta_game_settings(&settings, persistent_settings.as_deref_mut());
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::SettingsScene);
                } else {
                    params.transition_to_settings_scene(&settings);
                }
            }
            MetaScreenButtonAction::CycleQuality => {}
            MetaScreenButtonAction::ToggleSfx => {
                settings.toggle_sfx();
                save_meta_game_settings(&settings, persistent_settings.as_deref_mut());
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::SettingsScene);
                } else {
                    params.transition_to_settings_scene(&settings);
                }
            }
            MetaScreenButtonAction::ToggleMusic => {
                settings.toggle_music();
                save_meta_game_settings(&settings, persistent_settings.as_deref_mut());
                if let Some(screen_transition) = screen_transition.as_deref_mut() {
                    screen_transition
                        .request_view_change(*params.active_view, ActiveView::SettingsScene);
                } else {
                    params.transition_to_settings_scene(&settings);
                }
            }
        }
        let _ = &active_card_model;
    }
}

fn save_meta_game_settings(
    settings: &MetaGameSettingsModel,
    persistent_settings: Option<&mut Persistent<MetaGameSettingsModel>>,
) {
    if let Some(persistent_settings) = persistent_settings
        && let Err(error) = persistent_settings.set(settings.clone())
    {
        warn!("Failed to save meta game settings: {error}");
    }
}

/// HUMAN: Applies stored settings UI framerate into update-loop cadence.
/// AI: Uses `WinitSettings` to avoid per-platform game-loop hacks and keeps framerate user-defined.
#[cfg(not(target_arch = "wasm32"))]
fn apply_meta_game_framerate_settings(
    settings: &MetaGameSettingsModel,
    winit_settings: &mut WinitSettings,
) {
    let frame_time = Duration::from_secs_f64(1.0 / settings.framerate as f64);
    let update_mode = UpdateMode::reactive(frame_time);
    winit_settings.focused_mode = update_mode;
    winit_settings.unfocused_mode = update_mode;
}

/// HUMAN: Advances fake matchmaking and enters GameScreen when the temporary sequence completes.
/// AI: This must remain deterministic so tests can drive it with fixed delta values.
pub fn matchmaking_update_system(
    active_card_model: Res<ActiveCardModel>,
    asset_server: Res<AssetServer>,
    card_model_registry: Res<CardModelRegistry>,
    world_model_registry: Res<WorldModelRegistry>,
    location_model_registry: Res<LocationModelRegistry>,
    time: Res<Time>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
) {
    let Some(matchmaking_phase) = params.matchmaking_model.as_deref().map(|model| model.phase)
    else {
        return;
    };
    let is_matchmaking_screen = *params.active_view == ActiveView::MatchmakingScene;
    let is_preparing_warmup = *params.active_view == ActiveView::GameScene
        && MATCH_ASSETS_PRELOAD_ENABLED
        && matchmaking_phase == MatchmakingPhaseModel::Preparing
        && params
            .matchmaking_model
            .as_deref()
            .is_some_and(MatchmakingModel::match_is_prepared);
    if !is_matchmaking_screen && !is_preparing_warmup {
        return;
    }

    if MATCH_ASSETS_PRELOAD_ENABLED && matchmaking_phase == MatchmakingPhaseModel::Loading {
        let needs_prepare = params
            .matchmaking_model
            .as_deref()
            .is_some_and(|model| !model.match_is_prepared());
        if needs_prepare {
            restart_game_model(
                params.gesture_model.as_deref_mut(),
                params.slot_board.as_deref_mut(),
                params.card_states.as_deref_mut(),
                params.game_deck_model.as_deref_mut(),
                params.game_hand_model.as_deref_mut(),
                params.game_round_model.as_deref_mut(),
                params.game_location_model.as_deref_mut(),
                Some(&location_model_registry),
                Some(&mut params.active_locations),
                Some(&mut params.active_world_model),
                Some(&world_model_registry),
                params.match_model.as_deref_mut(),
                params.player_deck_collection.as_deref(),
                params.cpu_brain_model.as_deref_mut(),
                params.pending_round_deal.as_deref_mut(),
            );
            if let Some(matchmaking_model) = params.matchmaking_model.as_deref_mut() {
                matchmaking_model.mark_match_prepared();
            }
        }
    }

    let Some(matchmaking_model) = params.matchmaking_model.as_deref_mut() else {
        return;
    };
    if matchmaking_model.phase == MatchmakingPhaseModel::Loading {
        matchmaking_model.begin_preload(
            &asset_server,
            &card_model_registry,
            &world_model_registry,
            &params.active_world_model,
            &location_model_registry,
            &params.active_locations,
        );
    }
    let loading_complete = matchmaking_model.preload_is_complete(&asset_server);
    let phase_before_tick = matchmaking_model.phase;
    let completed = matchmaking_model.tick(time.delta_secs(), loading_complete);
    let phase_after_tick = matchmaking_model.phase;
    let match_is_prepared = matchmaking_model.match_is_prepared();

    if completed {
        if is_preparing_warmup {
            params.finish_prepared_game_reveal();
        } else if MATCH_ASSETS_PRELOAD_ENABLED && match_is_prepared {
            params.begin_prepared_game_warmup(&active_card_model);
        } else {
            if let Some(screen_transition) = screen_transition.as_deref_mut() {
                screen_transition.request_view_change(*params.active_view, ActiveView::GameScene);
            } else {
                params.transition_to_game_scene(&active_card_model);
            }
        }
    } else if MATCH_ASSETS_PRELOAD_ENABLED
        && match_is_prepared
        && phase_before_tick == MatchmakingPhaseModel::Loading
        && phase_after_tick == MatchmakingPhaseModel::Preparing
    {
        params.begin_prepared_game_warmup(&active_card_model);
    } else if is_matchmaking_screen {
        params.reload_active_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
    } else {
        let _ = &active_card_model;
    }
}

/// HUMAN: Query group for DeckScreen rebuild, input, and selected-card menu sync.
/// AI: Keep deck_screen_update_system below Bevy's top-level system parameter limit.
#[derive(SystemParam)]
pub struct DeckScreenUpdateQueries<'w, 's> {
    scene_root_query: Query<'w, 's, Entity, With<DeckSceneRoot>>,
    ui_camera_query: Query<'w, 's, Entity, With<AppSceneCamera>>,
    ui_root_query: Query<
        'w,
        's,
        Entity,
        Or<(
            With<TopNavigationRoot>,
            With<CardGrid>,
            With<DeckScreenCardView>,
            With<DeckScreenGridBackdrop>,
            With<DeckScreenModalRoot>,
            With<DeckScreenSelectedCardMenuRoot>,
        )>,
    >,
    selection_menu_query: Query<'w, 's, Entity, With<DeckScreenSelectedCardMenuRoot>>,
    selected_deck_card_query: Query<'w, 's, &'static DeckScreenCardView>,
    button_query: Query<
        'w,
        's,
        (
            &'static Interaction,
            Option<&'static DeckScreenDeckTileButton>,
            Option<&'static DeckScreenDeckCommandButton>,
            Option<&'static DeckScreenTabButton>,
            Option<&'static DeckScreenModalActionButton>,
            Option<&'static DeckScreenValidationOkButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
}

pub fn deck_screen_update_system(
    mut commands: Commands,
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    mut deck_screen_model: ResMut<DeckScreenModel>,
    mut selected_card_modal: ResMut<SelectedCardModalModel>,
    mut player_deck_collection: ResMut<PlayerDeckCollectionModel>,
    mut persistent_player_decks: Option<ResMut<Persistent<PlayerDeckCollectionModel>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
    mut queries: DeckScreenUpdateQueries,
) {
    ensure_deck_screen_collection_no_auto_fill(&mut player_deck_collection);
    let mut should_persist = false;
    sync_deck_screen_modal_to_selected_card(
        &selected_card_modal,
        &mut deck_screen_model,
        &player_deck_collection,
        &queries.selected_deck_card_query,
    );

    for (interaction, deck_tile, deck_command, tab_button, modal_action, validation_ok) in
        &mut queries.button_query
    {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if validation_ok.is_some() {
            request_button_click(audio_manager.as_deref_mut());
            deck_screen_model.close_prompt();
            continue;
        }

        if deck_screen_model.validation_prompt || deck_screen_model.coming_soon_prompt {
            continue;
        }

        if deck_screen_model.modal.is_some() {
            if let Some(action) = modal_action {
                request_button_click(audio_manager.as_deref_mut());
                match action {
                    DeckScreenModalActionButton::Back => {
                        deck_screen_model.close_modal();
                        selected_card_modal.request_dismiss();
                    }
                    DeckScreenModalActionButton::MoveToLibrary => {
                        let modal = deck_screen_model.modal.clone();
                        if let Some(modal) = modal
                            && modal.actions.move_to_library
                            && move_deck_card_to_library(
                                &mut player_deck_collection,
                                modal.source_index,
                            )
                            .is_some()
                        {
                            should_persist = true;
                        }
                        deck_screen_model.close_modal();
                        selected_card_modal.request_dismiss();
                    }
                    DeckScreenModalActionButton::MoveToDeck => {
                        let modal = deck_screen_model.modal.clone();
                        if let Some(modal) = modal
                            && modal.actions.move_to_deck
                            && move_library_card_to_deck(
                                &mut player_deck_collection,
                                &modal.card_id,
                            )
                        {
                            should_persist = true;
                        }
                        deck_screen_model.close_modal();
                        selected_card_modal.request_dismiss();
                    }
                    DeckScreenModalActionButton::TransferOut => {
                        selected_card_modal.request_dismiss();
                        deck_screen_model.show_coming_soon_prompt();
                    }
                }
            }
            continue;
        }

        if deck_tile.is_some() {
            request_button_click(audio_manager.as_deref_mut());
            deck_screen_model.open_editor();
            continue;
        }
        if deck_command.is_some() {
            request_button_click(audio_manager.as_deref_mut());
            deck_screen_model.show_coming_soon_prompt();
            continue;
        }
        if let Some(tab_button) = tab_button {
            request_button_click(audio_manager.as_deref_mut());
            match tab_button.tab {
                DeckEditorTabModel::Library => {
                    deck_screen_model.select_tab(DeckEditorTabModel::Library)
                }
                DeckEditorTabModel::Shop => deck_screen_model.show_coming_soon_prompt(),
            }
            continue;
        }
    }

    if should_persist
        && let Some(persistent_player_decks) = persistent_player_decks.as_deref_mut()
        && let Err(error) = persistent_player_decks.set(player_deck_collection.clone())
    {
        warn!("Failed to save DeckScreen deck collection: {error}");
    }

    let mut should_rebuild = deck_screen_model.take_rebuild_request();
    if should_persist {
        should_rebuild = true;
    }
    if !should_rebuild {
        sync_deck_screen_selected_card_menu_view(
            &mut commands,
            &queries.scene_root_query,
            &queries.ui_camera_query,
            &queries.selection_menu_query,
            deck_screen_model.modal.as_ref(),
            deck_screen_model.validation_prompt || deck_screen_model.coming_soon_prompt,
        );
        return;
    }

    for entity in &queries.ui_root_query {
        commands.entity(entity).despawn();
    }

    let Ok(scene_root) = queries.scene_root_query.single() else {
        return;
    };
    let Ok(ui_camera) = queries.ui_camera_query.single() else {
        return;
    };
    let deck_cards = deck_screen_deck_cards(&player_deck_collection);
    commands.entity(scene_root).with_children(|parent| {
        spawn_top_navigation_view(
            parent,
            ui_camera,
            TopNavigationDestination::MyDecks,
            deck_screen_model.modal.is_some(),
        );
        spawn_deck_screen_content(
            parent,
            ui_camera,
            &asset_server,
            &card_model_registry,
            &player_deck_collection,
            deck_screen_model.mode,
            deck_screen_model.editor_tab,
            deck_screen_model.modal.as_ref(),
        );
        if deck_screen_model.validation_prompt {
            spawn_deck_screen_prompt(
                parent,
                ui_camera,
                DECK_SCREEN_VALIDATION_TITLE,
                DECK_SCREEN_VALIDATION_MESSAGE,
            );
        } else if deck_screen_model.coming_soon_prompt {
            spawn_deck_screen_prompt(
                parent,
                ui_camera,
                DECK_SCREEN_COMING_SOON_TITLE,
                DECK_SCREEN_COMING_SOON_MESSAGE,
            );
        } else if let Some(modal) = deck_screen_model.modal.as_ref() {
            spawn_deck_screen_selected_card_menu(parent, ui_camera, modal);
        }
    });
    if deck_screen_model.mode == crate::runtime::resources::DeckScreenMode::Editor {
        let mut masked_background_materials =
            masked_background_materials.map(|materials| materials.into_inner());
        let library_cards = deck_screen_library_cards(&deck_cards);
        spawn_deck_screen_grid_backdrops(
            &mut commands,
            &mut meshes,
            &mut materials,
            deck_screen_model.editor_tab,
            scene_root,
        );
        spawn_deck_screen_card_views(
            &mut commands,
            &asset_server,
            &card_defaults,
            &card_model_registry,
            &deck_cards,
            &library_cards,
            deck_screen_model.editor_tab,
            &mut meshes,
            &mut materials,
            &mut masked_background_materials,
            CardFace::Front,
            scene_root,
        );
    }
}

/// HUMAN: Mirrors the selected zoomed card into DeckScreen editor action state.
/// AI: This keeps card selection owned by SelectedCardModalModel instead of UI tile buttons.
fn sync_deck_screen_modal_to_selected_card(
    selected_card_modal: &SelectedCardModalModel,
    deck_screen_model: &mut DeckScreenModel,
    player_deck_collection: &PlayerDeckCollectionModel,
    selected_deck_card_query: &Query<&DeckScreenCardView>,
) {
    let Some(selected_entity) = selected_card_modal.selected_entity else {
        deck_screen_model.modal = None;
        return;
    };
    let Ok(selected_card) = selected_deck_card_query.get(selected_entity) else {
        return;
    };
    let deck_cards = deck_screen_deck_cards(player_deck_collection);
    let next_modal = crate::runtime::resources::DeckScreenCardModalModel {
        card_id: selected_card.card_id.clone(),
        source_zone: selected_card.zone,
        source_index: selected_card.index,
        actions: modal_actions_for(selected_card.zone, &selected_card.card_id, &deck_cards),
    };
    if deck_screen_model.modal.as_ref() != Some(&next_modal) {
        deck_screen_model.modal = Some(next_modal);
    }
}

fn sync_deck_screen_selected_card_menu_view(
    commands: &mut Commands,
    scene_root_query: &Query<Entity, With<DeckSceneRoot>>,
    ui_camera_query: &Query<Entity, With<AppSceneCamera>>,
    selection_menu_query: &Query<Entity, With<DeckScreenSelectedCardMenuRoot>>,
    modal: Option<&crate::runtime::resources::DeckScreenCardModalModel>,
    prompt_blocks_menu: bool,
) {
    let existing_menus: Vec<Entity> = selection_menu_query.iter().collect();
    if modal.is_none() || prompt_blocks_menu {
        for entity in existing_menus {
            commands.entity(entity).despawn();
        }
        return;
    }
    if !existing_menus.is_empty() {
        return;
    }
    let (Ok(scene_root), Ok(ui_camera), Some(modal)) =
        (scene_root_query.single(), ui_camera_query.single(), modal)
    else {
        return;
    };
    commands.entity(scene_root).with_children(|parent| {
        spawn_deck_screen_selected_card_menu(parent, ui_camera, modal);
    });
}

/// HUMAN: Spawns the debug sub-screen scene.
/// AI: DebugScene duplicates DeckScene presentation for debug configuration work.
pub fn setup_debug_scene(
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
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
) {
    let app_camera = app_camera_query.iter().next().unwrap_or_else(|| {
        ensure_shared_app_camera(&mut commands, app_scene_query.single().ok(), None)
    });
    spawn_debug_scene_contents(
        &mut commands,
        app_camera,
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

fn spawn_debug_scene_contents(
    commands: &mut Commands,
    app_camera: Entity,
    asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    active_card_model: &ActiveCardModel,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
    app_scene_parent: Option<Entity>,
    visible_face: CardFace,
    initial_rotation: Quat,
) {
    let scene_root = commands.spawn(DebugScreenBundle::default()).id();
    let ui_camera = app_camera;
    let _ = camera_defaults;
    let point_text_camera = spawn_card_point_text_camera(
        commands,
        "DebugScene Card Point Text Camera",
        DebugSceneEntity,
    );
    let light = spawn_debug_light(commands);
    commands.entity(scene_root).with_children(|parent| {
        spawn_top_navigation_view(parent, ui_camera, TopNavigationDestination::Debug, false);
    });
    let mut card_transform = debug_scene_initial_card_transform(card_defaults, camera_defaults);
    card_transform.rotation *= initial_rotation;
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
        false,
        card_transform,
    );
    let location =
        spawn_debug_location_sample(commands, asset_server, camera_defaults, meshes, materials);
    commands.entity(scene_root).add_child(point_text_camera);
    commands.entity(scene_root).add_child(light);
    commands.entity(scene_root).add_child(location);
    commands.entity(scene_root).add_child(card);
    commands
        .entity(card)
        .insert((
            DebugSceneEntity,
            SelectableCard::new(CardSelectionSource::ScreenCard {
                view: ActiveView::DebugScene,
            }),
        ))
        .observe(card_click_selection)
        .observe(card_click_navigation);
    if let Some(parent) = app_scene_parent {
        commands.entity(parent).add_child(scene_root);
    }
}

/// HUMAN: Adds one runtime location specimen to DebugScene beside the card specimen.
/// AI: Keep it close to GameScene's mesh/text/point composition while using fixed mock values.
fn spawn_debug_location_sample(
    commands: &mut Commands,
    asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    const DEBUG_LOCATION_INDEX: usize = 0;
    const DEBUG_LOCATION_TITLE: &str = "Debug Location";
    const DEBUG_LOCATION_BODY: &str = "Mock power totals";
    const DEBUG_LOCATION_TEXTURE_PATH: &str =
        "themes/theme_japan/locations/location_bamboo_crossing/location.png";

    let world_width = game_scene_world_width_for_game_scene_width(
        DEBUG_SCENE_LOCATION_WIDTH,
        GAME_SCENE_LOCATION_BUNDLE_Z,
    );
    let world_height = game_scene_world_height_for_game_scene_height(
        DEBUG_SCENE_LOCATION_HEIGHT,
        GAME_SCENE_LOCATION_BUNDLE_Z,
    );
    let border_material = flat_color_material(materials, Color::srgb(0.2, 0.95, 0.35));
    let point_circle_mesh = meshes.add(Circle::new(
        game_scene_world_height_for_game_scene_height(
            LOCATION_POINT_VIEW_HEIGHT,
            GAME_SCENE_LOCATION_BUNDLE_Z,
        ) * 0.5,
    ));

    commands
        .spawn((
            Name::new("DebugScene Location Sample"),
            DebugSceneEntity,
            LocationBundle::new(DEBUG_LOCATION_INDEX),
            LocationBundleSurface::new(DEBUG_LOCATION_INDEX),
            LocationBundleOverlay::new(DEBUG_LOCATION_INDEX),
            GameLocation::new(DEBUG_LOCATION_INDEX, LocationRevealState::Revealed),
            Mesh3d(meshes.add(Rectangle::new(world_width, world_height))),
            MeshMaterial3d(location_bundle_material(
                asset_server,
                materials,
                DEBUG_LOCATION_TEXTURE_PATH,
                1.0,
            )),
            debug_scene_location_transform(camera_defaults),
            Visibility::Visible,
            NoCpuCulling,
            NoFrustumCulling,
        ))
        .with_children(|parent| {
            spawn_location_border_meshes(
                parent,
                DEBUG_LOCATION_INDEX,
                world_width,
                world_height,
                border_material,
                meshes,
            );
            spawn_location_title_and_body_3d(
                parent,
                DEBUG_LOCATION_INDEX,
                DEBUG_LOCATION_TITLE,
                DEBUG_LOCATION_BODY,
                world_height,
                1.0,
            );
            spawn_location_power_point_view(
                parent,
                materials,
                PowerPointModel::new(4),
                DEBUG_LOCATION_INDEX,
                CardSlotSide::Opponent,
                world_height,
                point_circle_mesh.clone(),
                true,
                1.0,
            );
            spawn_location_power_point_view(
                parent,
                materials,
                PowerPointModel::new(9),
                DEBUG_LOCATION_INDEX,
                CardSlotSide::LocalPlayer,
                world_height,
                point_circle_mesh,
                false,
                1.0,
            );
        })
        .id()
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
        false,
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(deck_centered_card_scale(&card_defaults)),
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
    uses_cpu_face_control: bool,
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
        uses_cpu_face_control,
        transform,
    )
}

/// HUMAN: Positions the DebugScene card near the card control panel.
/// AI: Size uses Card UI width and offsets by a fixed gap so the model and UI sit beside each other.
fn debug_scene_card_transform(card_defaults: &CardInspectionDefaults, rotation: Quat) -> Transform {
    let target_card_size = game_scene_hand_card_size();
    let target_card_scale = game_scene_world_height_for_game_scene_height(
        target_card_size.y,
        GAME_SCENE_HAND_CARD_WORLD_Z,
    ) / card_defaults.height;
    let card_center = game_scene_world_position_from_game_scene(
        debug_scene_card_center_game_scene_position(),
        GAME_SCENE_HAND_CARD_WORLD_Z,
    );

    Transform {
        translation: card_center,
        rotation,
        scale: Vec3::splat(target_card_scale),
    }
}

fn debug_scene_card_center_game_scene_position() -> Vec2 {
    let target_card_size = game_scene_hand_card_size();
    Vec2::new(
        GAME_SCENE_WIDTH
            - SCREEN_PADDING_LEFT
            - DEBUG_SCENE_CARD_GAP_TO_CARD_UI
            - DEBUG_SCENE_CARD_LEFT_OFFSET_PX
            - (target_card_size.x * 1.5),
        SCREEN_PADDING_TOP
            + DEBUG_SCENE_CARD_VERTICAL_OFFSET
            + DEBUG_SCENE_CARD_EXTRA_DOWN_OFFSET_PX
            + (target_card_size.y * 0.5),
    )
}

fn debug_scene_location_transform(camera_defaults: &PrimaryCameraDefaults) -> Transform {
    let target_card_size = game_scene_hand_card_size();
    let card_center = debug_scene_card_center_game_scene_position();
    let location_center = Vec2::new(
        card_center.x
            - (target_card_size.x * 0.5)
            - DEBUG_SCENE_CARD_GAP_TO_CARD_UI
            - (DEBUG_SCENE_LOCATION_WIDTH * 0.5),
        card_center.y,
    );
    let translation =
        game_scene_world_position_from_game_scene(location_center, GAME_SCENE_LOCATION_BUNDLE_Z);

    Transform {
        translation,
        rotation: debug_scene_face_camera_rotation(translation, camera_defaults),
        scale: Vec3::ONE,
    }
}

/// HUMAN: Faces the DebugScene card squarely toward the debug 3D camera.
/// AI: Compensates for the off-center debug layout under the shared perspective camera.
fn debug_scene_initial_card_transform(
    card_defaults: &CardInspectionDefaults,
    camera_defaults: &PrimaryCameraDefaults,
) -> Transform {
    let mut transform = debug_scene_card_transform(card_defaults, Quat::IDENTITY);
    transform.rotation = debug_scene_face_camera_rotation(transform.translation, camera_defaults);
    transform
}

fn debug_scene_face_camera_rotation(
    translation: Vec3,
    camera_defaults: &PrimaryCameraDefaults,
) -> Quat {
    let mut camera_defaults = camera_defaults.clone();
    camera_defaults.position.z = DECK_SCENE_CAMERA_DISTANCE_FROM_ORIGIN;
    let face_direction = (camera_defaults.position - translation)
        .try_normalize()
        .unwrap_or(Vec3::Z);
    Quat::from_rotation_arc(Vec3::Z, face_direction)
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
    uses_cpu_face_control: bool,
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
    let ability_outline_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.74, 0.18),
        alpha_mode: AlphaMode::Opaque,
        depth_bias: POINT_DEPTH_BIAS + 1.0,
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
            uses_cpu_face_control,
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
                    uses_cpu_face_control,
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
                    uses_cpu_face_control,
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
                uses_cpu_face_control,
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
            uses_cpu_face_control,
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
            uses_cpu_face_control,
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
            uses_cpu_face_control,
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
            uses_cpu_face_control,
        );
        spawn_card_power_point_view(
            parent,
            asset_server,
            card_model.base_power,
            power_point_background_mesh,
            power_point_background_material,
            ability_outline_material.clone(),
            Vec3::new(-cost_point_x, point_y, point_background_z),
            Vec3::new(-cost_point_x, point_y, point_text_z),
            visible_face == CardFace::Front,
            uses_cpu_face_control,
        );
        spawn_card_cost_point_view(
            parent,
            asset_server,
            card_model.cost,
            cost_point_background_mesh,
            cost_point_background_material,
            ability_outline_material,
            Vec3::new(power_point_x, point_y, point_background_z),
            Vec3::new(power_point_x, point_y, point_text_z),
            visible_face == CardFace::Front,
            uses_cpu_face_control,
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

fn world_fade_overlay_material(
    materials: &mut Assets<StandardMaterial>,
    alpha: f32,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color_with_alpha(Color::BLACK, alpha),
        alpha_mode: AlphaMode::Blend,
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
    uses_cpu_face_control: bool,
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
    entity.observe(card_click_selection);
    if let Some(background_layer) = background_layer {
        entity.insert(background_layer);
    }
    if is_frame {
        entity.insert(CardFrameLayer);
    }
    if uses_cpu_face_control {
        entity.insert(CardAnimationFaceLayer);
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
    uses_cpu_face_control: bool,
) {
    let mut entity = parent.spawn((
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
    ));
    if uses_cpu_face_control {
        entity.insert(CardAnimationFaceLayer);
    }
    entity.observe(card_click_navigation);
    entity.observe(card_click_selection);
}

fn spawn_card_back_plane(
    parent: &mut ChildSpawnerCommands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    card_defaults: &CardInspectionDefaults,
    is_visible: bool,
    uses_cpu_face_control: bool,
) {
    let mut entity = parent.spawn((
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
    ));
    if uses_cpu_face_control {
        entity.insert(CardAnimationFaceLayer);
    }
    entity.observe(card_click_navigation);
    entity.observe(card_click_selection);
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
    mut card_query: Query<&mut Transform, (With<CardView>, With<DeckSceneEntity>)>,
    mut game_card_query: Query<
        &mut Transform,
        (
            With<LocalPlayerHandCardPreview>,
            With<GameSceneEntity>,
            Without<CardAnimation>,
            Without<DeckSceneEntity>,
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

pub fn log_game_scene_card_render_diagnostics(
    mut has_logged: Local<bool>,
    active_view: Res<ActiveView>,
    card_query: Query<
        (&Name, &Transform, &GlobalTransform),
        (
            With<LocalPlayerHandCardPreview>,
            With<CardView>,
            With<GameSceneEntity>,
            Without<DeckSceneEntity>,
        ),
    >,
    layer_query: Query<
        (&Name, &Visibility, &GlobalTransform),
        (
            With<CardParallaxLayer>,
            With<CardFaceLayer>,
            Without<DeckSceneEntity>,
        ),
    >,
    camera_query: Query<(&Name, &Camera, Option<&Projection>), With<GameSceneEntity>>,
) {
    if *has_logged || *active_view != ActiveView::GameScene {
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
        "GameScene 3D card render diagnostics: cards={} layers={} cameras={} card_roots=[{}] layers=[{}] cameras=[{}]",
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
                    "GameScene 3D card render diagnostics: cards={} layers={} cameras={} card_roots=[{}] layers=[{}] cameras=[{}]\n",
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
        warn!("Failed to write GameScene render diagnostics: {error}");
    }
    *has_logged = true;
}

pub fn composed_card_rotation(
    card_state: &CardInspectionState,
    flip_state: &CardFlipState,
) -> Quat {
    card_state.target_rotation * flip_state.rotation()
}

pub(crate) fn composed_rotation_for_face(card_state: &CardInspectionState, face: CardFace) -> Quat {
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
    mut face_query: Query<
        (
            &CardFaceLayer,
            Option<&CardParallaxLayer>,
            &mut Visibility,
            Option<&GameSceneSceneHiddenVisibility>,
        ),
        Without<CardAnimationFaceLayer>,
    >,
) {
    if !flip_state.is_changed() && !card_ui_state.is_changed() {
        return;
    }

    for (face_layer, parallax_layer, mut visibility, hidden_visibility) in &mut face_query {
        if hidden_visibility.is_some() {
            continue;
        }
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
        ActiveView::GameScene => {
            scene.active_world_model.toggle(&scene.world_model_registry);
            scene.request_game_scene_world_background_transition();
        }
        ActiveView::DeckScene | ActiveView::DebugScene => {
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
        ActiveView::MainMenuScene
        | ActiveView::LightningScene
        | ActiveView::MatchmakingScene
        | ActiveView::SettingsScene => {}
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

/// HUMAN: Advances GameScene world background fade-to-black theme transitions.
/// AI: Swap the world texture only while the black overlay is fully opaque.
pub fn world_theme_transition_update_system(
    time: Res<Time>,
    active_world_model: Res<ActiveWorldModel>,
    world_model_registry: Res<WorldModelRegistry>,
    mut world_query: Query<
        (
            &mut Name,
            &mut MeshMaterial3d<StandardMaterial>,
            &mut WorldThemeTransition,
        ),
        (With<WorldBackground>, Without<WorldFadeOverlay>),
    >,
    overlay_query: Query<
        &MeshMaterial3d<StandardMaterial>,
        (With<WorldFadeOverlay>, Without<WorldBackground>),
    >,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (_, _, mut transition) in &mut world_query {
        if active_world_model.index != transition.applied_world_index {
            transition.request_world_index(active_world_model.index);
        }
    }

    for (mut name, mut material, mut transition) in &mut world_query {
        advance_world_theme_transition(
            &time,
            &active_world_model,
            &world_model_registry,
            &asset_server,
            &mut materials,
            &mut name,
            &mut material,
            &mut transition,
        );
    }

    for overlay_material in &overlay_query {
        if let Some(material) = materials.get_mut(&overlay_material.0) {
            material.base_color =
                color_with_alpha(Color::BLACK, current_world_overlay_alpha(&world_query));
            material.alpha_mode = AlphaMode::Blend;
        }
    }
}

fn advance_world_theme_transition(
    time: &Time,
    active_world_model: &ActiveWorldModel,
    world_model_registry: &WorldModelRegistry,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
    name: &mut Name,
    material: &mut MeshMaterial3d<StandardMaterial>,
    transition: &mut WorldThemeTransition,
) {
    match transition.phase {
        WorldThemeTransitionPhase::StartupFadeIn => {
            transition.elapsed_seconds = (transition.elapsed_seconds + time.delta_secs().max(0.0))
                .min(WORLD_THEME_FADE_SECONDS);
            let progress = transition.elapsed_seconds / WORLD_THEME_FADE_SECONDS;
            transition.overlay_alpha = 1.0 - progress;
            if transition.elapsed_seconds >= WORLD_THEME_FADE_SECONDS {
                transition.elapsed_seconds = 0.0;
                transition.overlay_alpha = 0.0;
                transition.phase = if transition.pending_world_index.is_some() {
                    WorldThemeTransitionPhase::FadeOutToBlack
                } else {
                    WorldThemeTransitionPhase::Idle
                };
            }
        }
        WorldThemeTransitionPhase::Idle => {}
        WorldThemeTransitionPhase::FadeOutToBlack => {
            transition.elapsed_seconds = (transition.elapsed_seconds + time.delta_secs().max(0.0))
                .min(WORLD_THEME_FADE_SECONDS);
            transition.overlay_alpha = transition.elapsed_seconds / WORLD_THEME_FADE_SECONDS;
            if transition.elapsed_seconds >= WORLD_THEME_FADE_SECONDS {
                transition.overlay_alpha = 1.0;
                transition.elapsed_seconds = 0.0;
                apply_world_background_model(
                    transition
                        .pending_world_index
                        .unwrap_or(active_world_model.index),
                    world_model_registry,
                    asset_server,
                    materials,
                    name,
                    material,
                    transition,
                );
                transition.phase = WorldThemeTransitionPhase::HoldAtBlack;
            }
        }
        WorldThemeTransitionPhase::HoldAtBlack => {
            transition.elapsed_seconds = (transition.elapsed_seconds + time.delta_secs().max(0.0))
                .min(WORLD_THEME_BLACK_HOLD_SECONDS);
            transition.overlay_alpha = 1.0;
            if transition.elapsed_seconds >= WORLD_THEME_BLACK_HOLD_SECONDS {
                transition.elapsed_seconds = 0.0;
                transition.phase = WorldThemeTransitionPhase::FadeInFromBlack;
            }
        }
        WorldThemeTransitionPhase::FadeInFromBlack => {
            transition.elapsed_seconds = (transition.elapsed_seconds + time.delta_secs().max(0.0))
                .min(WORLD_THEME_FADE_SECONDS);
            let progress = transition.elapsed_seconds / WORLD_THEME_FADE_SECONDS;
            transition.overlay_alpha = 1.0 - progress;
            if transition.elapsed_seconds >= WORLD_THEME_FADE_SECONDS {
                transition.elapsed_seconds = 0.0;
                transition.overlay_alpha = 0.0;
                if active_world_model.index != transition.applied_world_index {
                    transition.request_world_index(active_world_model.index);
                } else {
                    transition.phase = WorldThemeTransitionPhase::Idle;
                }
            }
        }
    }
}

fn apply_world_background_model(
    world_index: usize,
    world_model_registry: &WorldModelRegistry,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
    name: &mut Name,
    material: &mut MeshMaterial3d<StandardMaterial>,
    transition: &mut WorldThemeTransition,
) {
    let requested_world = ActiveWorldModel { index: world_index };
    let world_model = world_model_registry.active_world_model(&requested_world);
    *name = Name::new(format!("{} World Background", world_model.display_name));
    *material = MeshMaterial3d(card_model_material(
        asset_server,
        materials,
        world_model.background_texture,
        AlphaMode::Opaque,
        BACKGROUND_DEPTH_BIAS,
    ));
    transition.applied_world_index = world_index;
    transition.pending_world_index = None;
}

fn current_world_overlay_alpha(
    world_query: &Query<
        (
            &mut Name,
            &mut MeshMaterial3d<StandardMaterial>,
            &mut WorldThemeTransition,
        ),
        (With<WorldBackground>, Without<WorldFadeOverlay>),
    >,
) -> f32 {
    world_query
        .iter()
        .map(|(_, _, transition)| transition.overlay_alpha)
        .fold(0.0, f32::max)
}

#[cfg_attr(feature = "desktop-hot-reload", hot)]
#[derive(SystemParam)]
pub struct RestartAppSceneParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    active_card_model: Res<'w, ActiveCardModel>,
    scene: ViewChangeParams<'w, 's>,
}

#[cfg_attr(feature = "desktop-hot-reload", hot)]
pub fn restart_app_scene(params: RestartAppSceneParams) {
    let RestartAppSceneParams {
        keys,
        active_card_model,
        mut scene,
    } = params;

    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    scene.restart_game(&active_card_model);
}

fn reset_game_model(
    gesture_model: Option<&mut CardGestureModel>,
    slot_board: Option<&mut CardSlotBoardModel>,
    card_states: Option<&mut CardStateModel>,
    game_deck_model: Option<&mut GameDeckModel>,
    game_hand_model: Option<&mut GameHandModel>,
    game_round_model: Option<&mut GameRoundModel>,
    game_location_model: Option<&mut GameLocationModel>,
    location_model_registry: Option<&LocationModelRegistry>,
    active_locations: Option<&mut ActiveLocations>,
    active_world_model: Option<&ActiveWorldModel>,
    match_model: Option<&mut MatchModel>,
    player_deck_collection: Option<&PlayerDeckCollectionModel>,
) {
    if let Some(gesture_model) = gesture_model {
        *gesture_model = CardGestureModel::default();
    }
    if let Some(slot_board) = slot_board {
        *slot_board = CardSlotBoardModel::default();
    }
    if let Some(card_states) = card_states {
        *card_states = CardStateModel::default();
        if let (
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(game_location_model),
            Some(location_model_registry),
            Some(active_locations),
            Some(active_world_model),
            Some(match_model),
            Some(player_deck_collection),
        ) = (
            game_deck_model,
            game_hand_model,
            game_round_model,
            game_location_model,
            location_model_registry,
            active_locations,
            active_world_model,
            match_model,
            player_deck_collection,
        ) {
            reset_two_player_match(
                match_model.mode,
                match_model,
                game_deck_model,
                game_hand_model,
                game_round_model,
                game_location_model,
                Some(location_model_registry),
                Some(active_locations),
                Some(active_world_model),
                player_deck_collection.primary_deck(),
            );
            card_states.reset_to_size(game_hand_model.len());
        }
    }
}

fn restart_game_model(
    gesture_model: Option<&mut CardGestureModel>,
    slot_board: Option<&mut CardSlotBoardModel>,
    card_states: Option<&mut CardStateModel>,
    game_deck_model: Option<&mut GameDeckModel>,
    game_hand_model: Option<&mut GameHandModel>,
    game_round_model: Option<&mut GameRoundModel>,
    game_location_model: Option<&mut GameLocationModel>,
    location_model_registry: Option<&LocationModelRegistry>,
    active_locations: Option<&mut ActiveLocations>,
    active_world_model: Option<&mut ActiveWorldModel>,
    world_model_registry: Option<&WorldModelRegistry>,
    match_model: Option<&mut MatchModel>,
    player_deck_collection: Option<&PlayerDeckCollectionModel>,
    cpu_brain_model: Option<&mut CpuBrainModel>,
    pending_round_deal: Option<&mut PendingRoundDealResource>,
) {
    let active_world_model = match (active_world_model, world_model_registry) {
        (Some(active_world_model), Some(world_model_registry)) => {
            active_world_model.randomize(world_model_registry);
            Some(&*active_world_model)
        }
        (Some(active_world_model), None) => {
            active_world_model.randomize_with_len(WORLD_MODEL_COUNT);
            Some(&*active_world_model)
        }
        (None, _) => None,
    };

    if let Some(gesture_model) = gesture_model {
        *gesture_model = CardGestureModel::default();
    }
    if let Some(slot_board) = slot_board {
        *slot_board = CardSlotBoardModel::default();
    }

    match (
        card_states,
        game_deck_model,
        game_hand_model,
        game_round_model,
        game_location_model,
        location_model_registry,
        active_locations,
        active_world_model,
        match_model,
        player_deck_collection,
        pending_round_deal,
    ) {
        (
            Some(card_states),
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(game_location_model),
            Some(location_model_registry),
            Some(active_locations),
            Some(active_world_model),
            Some(match_model),
            Some(player_deck_collection),
            Some(pending_round_deal),
        ) => {
            reset_two_player_match_without_starting_round(
                match_model.mode,
                match_model,
                game_deck_model,
                game_hand_model,
                game_round_model,
                game_location_model,
                Some(location_model_registry),
                Some(active_locations),
                Some(active_world_model),
                player_deck_collection.primary_deck(),
            );
            card_states.reset_to_size(game_hand_model.len());
            pending_round_deal.is_pending = true;
            pending_round_deal.is_round_deal_complete = false;
            pending_round_deal.waits_for_location_intro = true;
        }
        (
            card_states,
            game_deck_model,
            game_hand_model,
            game_round_model,
            game_location_model,
            location_model_registry,
            active_locations,
            active_world_model,
            match_model,
            player_deck_collection,
            _,
        ) => {
            reset_game_model(
                None,
                None,
                card_states,
                game_deck_model,
                game_hand_model,
                game_round_model,
                game_location_model,
                location_model_registry,
                active_locations,
                active_world_model,
                match_model,
                player_deck_collection,
            );
        }
    }

    if let Some(cpu_brain_model) = cpu_brain_model {
        cpu_brain_model.reset();
    }
}

#[cfg(feature = "desktop-hot-reload")]
pub fn record_desktop_hot_reload_patch_message(
    mut patches: MessageReader<HotPatched>,
    mut hot_reload_screen_model: ResMut<HotReloadScreenModel>,
) {
    for _ in patches.read() {
        info!("Desktop hot reload patch applied");
        record_desktop_hot_reload_patch();
        hot_reload_screen_model.observe_patch_count(desktop_hot_reload_patch_count());
    }
}

#[cfg(not(feature = "desktop-hot-reload"))]
pub fn record_desktop_hot_reload_patch_message() {}

#[cfg(feature = "desktop-hot-reload")]
pub fn hot_reload_auto_restart_app_scene(
    hud_state: Res<DebugHudState>,
    mut hot_reload_screen_model: ResMut<HotReloadScreenModel>,
    mut debug_drawing_model: ResMut<DebugDrawingModel>,
    active_card_model: Res<ActiveCardModel>,
    player_deck_collection: Option<Res<PlayerDeckCollectionModel>>,
    mut flip_state: ResMut<CardFlipState>,
    mut ticks: ResMut<GameTicks>,
    mut deck_screen_model: Option<ResMut<DeckScreenModel>>,
    mut scene: ViewChangeParams,
) {
    let patch_count = desktop_hot_reload_patch_count();
    hot_reload_screen_model.observe_patch_count(patch_count);
    let Some(active_view) = hot_reload_screen_model.take_screen_reset_request(
        hud_state.is_hot_reload_autorestart_enabled,
        *scene.active_view,
    ) else {
        return;
    };

    let fallback_slot_board = CardSlotBoardModel::default();
    let slot_board = scene.slot_board.as_deref().unwrap_or(&fallback_slot_board);
    debug_drawing_model.request_reference_layout(slot_board);
    reset_active_screen_model_for_hot_reload(
        active_view,
        scene.gesture_model.as_deref_mut(),
        scene.slot_board.as_deref_mut(),
        scene.card_states.as_deref_mut(),
        scene.game_deck_model.as_deref_mut(),
        scene.game_hand_model.as_deref_mut(),
        scene.game_round_model.as_deref_mut(),
        scene.game_location_model.as_deref_mut(),
        Some(&scene.location_model_registry),
        Some(&mut scene.active_locations),
        Some(&scene.active_world_model),
        scene.match_model.as_deref_mut(),
        player_deck_collection.as_deref(),
        deck_screen_model.as_deref_mut(),
        scene.matchmaking_model.as_deref_mut(),
        scene.card_state.as_mut(),
        &mut flip_state,
        &mut ticks,
    );
    scene.reload_active_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
}

#[allow(dead_code)]
pub(crate) fn reset_active_screen_model_for_hot_reload(
    active_view: ActiveView,
    gesture_model: Option<&mut CardGestureModel>,
    slot_board: Option<&mut CardSlotBoardModel>,
    card_states: Option<&mut CardStateModel>,
    game_deck_model: Option<&mut GameDeckModel>,
    game_hand_model: Option<&mut GameHandModel>,
    game_round_model: Option<&mut GameRoundModel>,
    game_location_model: Option<&mut GameLocationModel>,
    location_model_registry: Option<&LocationModelRegistry>,
    active_locations: Option<&mut ActiveLocations>,
    active_world_model: Option<&ActiveWorldModel>,
    match_model: Option<&mut MatchModel>,
    player_deck_collection: Option<&PlayerDeckCollectionModel>,
    deck_screen_model: Option<&mut DeckScreenModel>,
    matchmaking_model: Option<&mut MatchmakingModel>,
    card_state: &mut CardInspectionState,
    flip_state: &mut CardFlipState,
    ticks: &mut GameTicks,
) {
    match active_view {
        ActiveView::GameScene => reset_game_model(
            gesture_model,
            slot_board,
            card_states,
            game_deck_model,
            game_hand_model,
            game_round_model,
            game_location_model,
            location_model_registry,
            active_locations,
            active_world_model,
            match_model,
            player_deck_collection,
        ),
        ActiveView::DeckScene => {
            if let Some(deck_screen_model) = deck_screen_model {
                *deck_screen_model = DeckScreenModel::default();
            }
        }
        ActiveView::MatchmakingScene => {
            if let Some(matchmaking_model) = matchmaking_model {
                matchmaking_model.reset();
            }
        }
        ActiveView::DebugScene
        | ActiveView::MainMenuScene
        | ActiveView::LightningScene
        | ActiveView::SettingsScene => {}
    }

    *flip_state = CardFlipState::default();
    *card_state = CardInspectionState::default();
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
    game_scene_roots: Query<'w, 's, Entity, With<GameSceneRoot>>,
    game_scene_entities: Query<'w, 's, Entity, With<GameSceneEntity>>,
    child_query: Query<'w, 's, &'static Children>,
    visibility_query: Query<
        'w,
        's,
        (
            Entity,
            &'static mut Visibility,
            Option<&'static GameSceneSceneHiddenVisibility>,
        ),
    >,
    standalone_game_scene_entities: Query<
        'w,
        's,
        Entity,
        (
            With<GameSceneEntity>,
            Without<GameSceneRoot>,
            Without<ChildOf>,
        ),
    >,
    standalone_deck_scene_entities: Query<
        'w,
        's,
        Entity,
        (
            With<DeckSceneEntity>,
            Without<DeckSceneRoot>,
            Without<ChildOf>,
        ),
    >,
    standalone_debug_scene_entities: Query<
        'w,
        's,
        Entity,
        (
            With<DebugSceneEntity>,
            Without<DebugSceneRoot>,
            Without<ChildOf>,
        ),
    >,
    standalone_meta_scene_entities: Query<
        'w,
        's,
        Entity,
        (
            With<MetaSceneEntity>,
            Without<MetaSceneRoot>,
            Without<ChildOf>,
        ),
    >,
    world_background_query: Query<
        'w,
        's,
        (
            &'static mut Name,
            &'static mut MeshMaterial3d<StandardMaterial>,
            &'static mut WorldThemeTransition,
        ),
        With<WorldBackground>,
    >,
    deck_scene_roots: Query<'w, 's, Entity, With<DeckSceneRoot>>,
    debug_scene_roots: Query<'w, 's, Entity, With<DebugSceneRoot>>,
    meta_scene_roots: Query<'w, 's, Entity, With<MetaSceneRoot>>,
    primary_window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    app_camera_query: Query<
        'w,
        's,
        (Entity, &'static Camera, &'static GlobalTransform),
        (With<AppSceneCamera>, With<Camera3d>),
    >,
    deck_card_query:
        Query<'w, 's, &'static GlobalTransform, (With<CardView>, With<DeckSceneEntity>)>,
    debug_card_query:
        Query<'w, 's, &'static GlobalTransform, (With<CardView>, With<DebugSceneEntity>)>,
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
    gesture_model: Option<ResMut<'w, CardGestureModel>>,
    game_deck_model: Option<ResMut<'w, GameDeckModel>>,
    game_hand_model: Option<ResMut<'w, GameHandModel>>,
    game_round_model: Option<ResMut<'w, GameRoundModel>>,
    game_location_model: Option<ResMut<'w, GameLocationModel>>,
    match_model: Option<ResMut<'w, MatchModel>>,
    card_states: Option<ResMut<'w, CardStateModel>>,
    cpu_brain_model: Option<ResMut<'w, CpuBrainModel>>,
    pending_round_deal: Option<ResMut<'w, PendingRoundDealResource>>,
    matchmaking_model: Option<ResMut<'w, MatchmakingModel>>,
    card_state: ResMut<'w, CardInspectionState>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<'w, Assets<CardBackgroundMaskMaterial>>>,
}

impl ViewChangeParams<'_, '_> {
    fn app_camera_entity(&mut self) -> Entity {
        self.app_camera_query
            .single()
            .map(|(entity, _, _)| entity)
            .unwrap_or_else(|_| {
                ensure_shared_app_camera(
                    &mut self.commands,
                    self.app_scene_query.single().ok(),
                    None,
                )
            })
    }

    fn set_game_scene_active(&mut self, is_active: bool) -> bool {
        let mut has_game_scene = false;

        if is_active {
            for (entity, mut visibility, hidden_visibility) in &mut self.visibility_query {
                let Some(hidden_visibility) = hidden_visibility else {
                    continue;
                };
                *visibility = hidden_visibility.0;
                self.commands
                    .entity(entity)
                    .remove::<GameSceneSceneHiddenVisibility>();
            }
        } else {
            let mut visited = std::collections::HashSet::new();
            let entities: Vec<Entity> = self.game_scene_entities.iter().collect();
            for entity in entities {
                self.collect_game_scene_entity_tree(entity, &mut visited);
            }
            for entity in visited {
                let Ok((_, mut visibility, hidden_visibility)) =
                    self.visibility_query.get_mut(entity)
                else {
                    continue;
                };
                if hidden_visibility.is_none() {
                    if let Ok(mut entity_cmd) = self.commands.get_entity(entity) {
                        entity_cmd.insert(GameSceneSceneHiddenVisibility(*visibility));
                    }
                }
                *visibility = Visibility::Hidden;
            }
        }

        for _ in self.game_scene_roots.iter() {
            has_game_scene = true;
        }
        has_game_scene
    }

    fn collect_game_scene_entity_tree(
        &mut self,
        entity: Entity,
        visited: &mut std::collections::HashSet<Entity>,
    ) {
        if !visited.insert(entity) {
            return;
        }

        if let Ok(children) = self.child_query.get(entity) {
            let children: Vec<Entity> = children.iter().collect();
            for child in children {
                self.collect_game_scene_entity_tree(child, visited);
            }
        }
    }

    fn hide_game_scene(&mut self) {
        self.set_game_scene_active(false);
    }

    fn restart_game(&mut self, active_card_model: &ActiveCardModel) {
        restart_game_model(
            self.gesture_model.as_deref_mut(),
            self.slot_board.as_deref_mut(),
            self.card_states.as_deref_mut(),
            self.game_deck_model.as_deref_mut(),
            self.game_hand_model.as_deref_mut(),
            self.game_round_model.as_deref_mut(),
            self.game_location_model.as_deref_mut(),
            Some(&self.location_model_registry),
            Some(&mut self.active_locations),
            Some(&mut self.active_world_model),
            Some(&self.world_model_registry),
            self.match_model.as_deref_mut(),
            self.player_deck_collection.as_deref(),
            self.cpu_brain_model.as_deref_mut(),
            self.pending_round_deal.as_deref_mut(),
        );
        *self.card_state = CardInspectionState::default();
        self.despawn_game_scene();
        *self.active_view = ActiveView::GameScene;
        self.spawn_game_scene(active_card_model);
    }

    fn despawn_game_scene(&mut self) {
        let mut visited = std::collections::HashSet::new();
        for entity in self.game_scene_roots.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
        for entity in self.standalone_game_scene_entities.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
        for entity in self.standalone_deck_scene_entities.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
        for entity in self.standalone_debug_scene_entities.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
        for entity in self.standalone_meta_scene_entities.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
    }

    fn despawn_meta_scene(&mut self) {
        for entity in self.meta_scene_roots.iter() {
            if self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
    }

    fn despawn_deck_scene(&mut self) {
        let mut visited = std::collections::HashSet::new();
        for entity in self.deck_scene_roots.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
        for entity in self.standalone_deck_scene_entities.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
    }

    fn despawn_debug_scene(&mut self) {
        let mut visited = std::collections::HashSet::new();
        for entity in self.debug_scene_roots.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
        for entity in self.standalone_debug_scene_entities.iter() {
            if visited.insert(entity) && self.commands.get_entity(entity).is_ok() {
                self.commands.entity(entity).despawn();
            }
        }
    }

    fn spawn_game_scene(&mut self, active_card_model: &ActiveCardModel) {
        let app_camera = self.app_camera_entity();
        let fallback_slot_board = CardSlotBoardModel::default();
        let slot_board = self.slot_board.as_deref().unwrap_or(&fallback_slot_board);
        let fallback_hand_cards = fallback_starting_hand_cards();
        let game_hand_cards = self
            .game_hand_model
            .as_deref()
            .map(|hand| hand.cards.as_slice())
            .unwrap_or(fallback_hand_cards.as_slice());
        let fallback_round = GameRoundModel::default();
        let game_round_model = self.game_round_model.as_deref().unwrap_or(&fallback_round);
        let fallback_locations = GameLocationModel::default();
        let game_location_model = self
            .game_location_model
            .as_deref()
            .unwrap_or(&fallback_locations);
        spawn_game_scene_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            app_camera,
            self.hud.as_ref().map(|hud| hud.0),
            &self.asset_server,
            &self.camera_defaults,
            &self.card_defaults,
            &self.card_model_registry,
            game_hand_cards,
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
    fn spawn_deck_scene(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        let app_camera = self.app_camera_entity();
        let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
        let player_deck_collection = self
            .player_deck_collection
            .as_deref()
            .unwrap_or(&fallback_player_deck_collection);
        spawn_deck_scene_contents(
            &mut self.commands,
            app_camera,
            &self.asset_server,
            &self.camera_defaults,
            &self.card_defaults,
            &self.card_model_registry,
            active_card_model,
            player_deck_collection,
            None,
            &mut self.meshes,
            &mut self.materials,
            self.masked_background_materials.as_deref_mut(),
            self.app_scene_query.single().ok(),
            visible_face,
            initial_rotation,
        );
    }

    fn spawn_debug_scene(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        let app_camera = self.app_camera_entity();
        spawn_debug_scene_contents(
            &mut self.commands,
            app_camera,
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

    fn spawn_main_menu_scene(&mut self) {
        let app_camera = self.app_camera_entity();
        spawn_main_menu_scene_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            app_camera,
            &self.asset_server,
        );
    }

    fn spawn_lightning_login_scene(&mut self) {
        let app_camera = self.app_camera_entity();
        spawn_lightning_login_scene_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            app_camera,
            &self.asset_server,
        );
    }

    fn spawn_matchmaking_scene(&mut self) {
        let fallback_matchmaking = MatchmakingModel::default();
        let app_camera = self.app_camera_entity();
        spawn_matchmaking_scene_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            app_camera,
            self.matchmaking_model
                .as_deref()
                .unwrap_or(&fallback_matchmaking),
        );
    }

    fn spawn_settings_scene(&mut self, settings: &MetaGameSettingsModel) {
        let app_camera = self.app_camera_entity();
        spawn_settings_scene_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            app_camera,
            settings,
        );
    }

    fn leave_current_view(&mut self) {
        match *self.active_view {
            ActiveView::GameScene => self.hide_game_scene(),
            ActiveView::DeckScene => self.despawn_deck_scene(),
            ActiveView::DebugScene => self.despawn_debug_scene(),
            ActiveView::MainMenuScene
            | ActiveView::LightningScene
            | ActiveView::MatchmakingScene
            | ActiveView::SettingsScene => self.despawn_meta_scene(),
        }
    }

    fn transition_to_main_menu_scene(&mut self) {
        self.leave_current_view();
        *self.active_view = ActiveView::MainMenuScene;
        self.spawn_main_menu_scene();
    }

    fn transition_to_lightning_login_scene(&mut self) {
        self.leave_current_view();
        *self.active_view = ActiveView::LightningScene;
        self.spawn_lightning_login_scene();
    }

    fn transition_to_matchmaking_scene(&mut self) {
        self.leave_current_view();
        if let Some(matchmaking_model) = self.matchmaking_model.as_deref_mut() {
            matchmaking_model.reset();
        }
        *self.active_view = ActiveView::MatchmakingScene;
        self.spawn_matchmaking_scene();
    }

    fn transition_to_settings_scene(&mut self, settings: &MetaGameSettingsModel) {
        self.leave_current_view();
        *self.active_view = ActiveView::SettingsScene;
        self.spawn_settings_scene(settings);
    }

    fn transition_to_deck_scene(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        self.leave_current_view();
        *self.active_view = ActiveView::DeckScene;
        self.spawn_deck_scene(active_card_model, visible_face, initial_rotation);
    }

    fn transition_to_debug_scene(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        self.leave_current_view();
        *self.active_view = ActiveView::DebugScene;
        self.spawn_debug_scene(active_card_model, visible_face, initial_rotation);
    }

    fn transition_to_game_scene(&mut self, active_card_model: &ActiveCardModel) {
        self.leave_current_view();
        self.restart_game(active_card_model);
    }

    /// HUMAN: Applies one requested ActiveView transition immediately.
    /// AI: transition_update_system calls this at full black to avoid visible scene popping.
    pub fn transition_to_requested_view(
        &mut self,
        target_view: ActiveView,
        settings: &MetaGameSettingsModel,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        match target_view {
            ActiveView::MainMenuScene => self.transition_to_main_menu_scene(),
            ActiveView::LightningScene => self.transition_to_lightning_login_scene(),
            ActiveView::MatchmakingScene => self.transition_to_matchmaking_scene(),
            ActiveView::GameScene => self.transition_to_game_scene(active_card_model),
            ActiveView::DeckScene => {
                self.transition_to_deck_scene(active_card_model, visible_face, initial_rotation)
            }
            ActiveView::SettingsScene => self.transition_to_settings_scene(settings),
            ActiveView::DebugScene => {
                self.transition_to_debug_scene(active_card_model, visible_face, initial_rotation)
            }
        }
    }

    fn begin_prepared_game_warmup(&mut self, active_card_model: &ActiveCardModel) {
        self.despawn_game_scene();
        *self.active_view = ActiveView::GameScene;
        self.spawn_game_scene(active_card_model);
        self.despawn_meta_scene();
        self.spawn_matchmaking_scene();
    }

    fn finish_prepared_game_reveal(&mut self) {
        self.despawn_meta_scene();
    }

    /// HUMAN: Requests a GameScene world background transition without touching gameplay state.
    /// AI: Keep T-key theme cycling visual-only while the active match continues.
    fn request_game_scene_world_background_transition(&mut self) {
        for (_, _, mut transition) in &mut self.world_background_query {
            transition.request_world_index(self.active_world_model.index);
        }
    }

    fn reload_active_view(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        match *self.active_view {
            ActiveView::MainMenuScene => {
                self.despawn_meta_scene();
                self.spawn_main_menu_scene();
            }
            ActiveView::LightningScene => {
                self.despawn_meta_scene();
                self.spawn_lightning_login_scene();
            }
            ActiveView::MatchmakingScene => {
                self.despawn_meta_scene();
                self.spawn_matchmaking_scene();
            }
            ActiveView::GameScene => {
                self.despawn_game_scene();
                let app_camera = self.app_camera_entity();
                let fallback_slot_board = CardSlotBoardModel::default();
                let slot_board = self.slot_board.as_deref().unwrap_or(&fallback_slot_board);
                let fallback_hand_cards = fallback_starting_hand_cards();
                let game_hand_cards = self
                    .game_hand_model
                    .as_deref()
                    .map(|hand| hand.cards.as_slice())
                    .unwrap_or(fallback_hand_cards.as_slice());
                let fallback_round = GameRoundModel::default();
                let game_round_model = self.game_round_model.as_deref().unwrap_or(&fallback_round);
                let fallback_locations = GameLocationModel::default();
                let game_location_model = self
                    .game_location_model
                    .as_deref()
                    .unwrap_or(&fallback_locations);
                spawn_game_scene_contents(
                    &mut self.commands,
                    self.app_scene_query.single().ok(),
                    app_camera,
                    self.hud.as_ref().map(|hud| hud.0),
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    game_hand_cards,
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
            ActiveView::DeckScene => {
                self.despawn_deck_scene();
                let app_camera = self.app_camera_entity();
                let fallback_player_deck_collection = PlayerDeckCollectionModel::default();
                let player_deck_collection = self
                    .player_deck_collection
                    .as_deref()
                    .unwrap_or(&fallback_player_deck_collection);
                spawn_deck_scene_contents(
                    &mut self.commands,
                    app_camera,
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
                    active_card_model,
                    player_deck_collection,
                    None,
                    &mut self.meshes,
                    &mut self.materials,
                    self.masked_background_materials.as_deref_mut(),
                    self.app_scene_query.single().ok(),
                    visible_face,
                    initial_rotation,
                );
            }
            ActiveView::SettingsScene => {
                self.despawn_meta_scene();
                let fallback_settings = MetaGameSettingsModel::default();
                self.spawn_settings_scene(&fallback_settings);
            }
            ActiveView::DebugScene => {
                self.despawn_debug_scene();
                let app_camera = self.app_camera_entity();
                spawn_debug_scene_contents(
                    &mut self.commands,
                    app_camera,
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
}

/// HUMAN: Guarantees preserved GameScene entities stay invisible while another view is active.
/// AI: Runs after presentation systems that may rewrite local card visibility in the same frame.
pub fn enforce_hidden_game_scene_visibility_system(
    active_view: Res<ActiveView>,
    game_scene_entities: Query<Entity, With<GameSceneEntity>>,
    child_query: Query<&Children>,
    mut visibility_query: Query<&mut Visibility>,
    mut point_text_camera_query: Query<
        &mut Camera,
        (With<GameSceneEntity>, With<CardPointTextCamera>),
    >,
) {
    for mut camera in &mut point_text_camera_query {
        camera.is_active = *active_view == ActiveView::GameScene;
    }

    if *active_view == ActiveView::GameScene {
        return;
    }

    let mut visited = std::collections::HashSet::new();
    let entities: Vec<Entity> = game_scene_entities.iter().collect();
    for entity in entities {
        collect_entity_tree(entity, &child_query, &mut visited);
    }
    for entity in visited {
        let Ok(mut visibility) = visibility_query.get_mut(entity) else {
            continue;
        };
        *visibility = Visibility::Hidden;
    }
}

fn collect_entity_tree(
    entity: Entity,
    child_query: &Query<&Children>,
    visited: &mut std::collections::HashSet<Entity>,
) {
    if !visited.insert(entity) {
        return;
    }

    if let Ok(children) = child_query.get(entity) {
        for child in children.iter() {
            collect_entity_tree(child, child_query, visited);
        }
    }
}

/// HUMAN: BRP helper that switches the running app to DeckScene for AI runtime inspection.
/// AI: Keep this equivalent to the user-facing scene cycle path, without synthesizing input.
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub fn ai_runtime_show_deck_screen_system(
    In(_params): In<Option<serde_json::Value>>,
    active_card_model: Res<ActiveCardModel>,
    flip_state: Res<CardFlipState>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
) -> bevy::remote::BrpResult {
    if let Some(screen_transition) = screen_transition.as_deref_mut() {
        screen_transition.request_view_change(*params.active_view, ActiveView::DeckScene);
    } else {
        let initial_rotation =
            composed_rotation_for_face(&params.card_state, flip_state.visible_face);
        params.transition_to_deck_scene(
            &active_card_model,
            flip_state.visible_face,
            initial_rotation,
        );
    }

    Ok(serde_json::json!({
        "active_view": "DeckScene",
        "success": true
    }))
}

/// HUMAN: BRP helper that opens DeckScene directly to the deck editor Library tab.
/// AI: Use this for AI visual checks of the library card-grid presentation.
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub fn ai_runtime_show_deck_library_system(
    In(_params): In<Option<serde_json::Value>>,
    active_card_model: Res<ActiveCardModel>,
    flip_state: Res<CardFlipState>,
    mut deck_screen: ResMut<DeckScreenModel>,
    mut top_navigation: ResMut<TopNavigationModel>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
) -> bevy::remote::BrpResult {
    if let Some(screen_transition) = screen_transition.as_deref_mut() {
        screen_transition.request_view_change(*params.active_view, ActiveView::DeckScene);
    } else {
        let initial_rotation =
            composed_rotation_for_face(&params.card_state, flip_state.visible_face);
        params.transition_to_deck_scene(
            &active_card_model,
            flip_state.visible_face,
            initial_rotation,
        );
    }

    top_navigation.selected = TopNavigationDestination::MyDecks;
    deck_screen.open_editor();
    deck_screen.select_tab(DeckEditorTabModel::Library);

    Ok(serde_json::json!({
        "active_view": "DeckScene",
        "deck_screen_mode": "Editor",
        "editor_tab": "Library",
        "success": true
    }))
}

/// HUMAN: BRP helper that switches the running app to GameScene for AI runtime inspection.
/// AI: Keep this deterministic so card-click runtime checks do not depend on navigation UI.
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub fn ai_runtime_show_game_screen_system(
    In(_params): In<Option<serde_json::Value>>,
    active_card_model: Res<ActiveCardModel>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
) -> bevy::remote::BrpResult {
    match *params.active_view {
        ActiveView::GameScene => {
            params.set_game_scene_active(true);
        }
        ActiveView::DeckScene
        | ActiveView::DebugScene
        | ActiveView::MainMenuScene
        | ActiveView::LightningScene
        | ActiveView::MatchmakingScene
        | ActiveView::SettingsScene => {
            if let Some(screen_transition) = screen_transition.as_deref_mut() {
                screen_transition.request_view_change(*params.active_view, ActiveView::GameScene);
            } else {
                params.transition_to_game_scene(&active_card_model);
            }
        }
    }

    Ok(serde_json::json!({
        "active_view": "GameScene",
        "success": true
    }))
}

/// HUMAN: BRP helper that switches the running app to DebugScene for AI runtime inspection.
/// AI: Uses the same debug scene spawning path as in-app navigation for runtime card QA.
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub fn ai_runtime_show_debug_screen_system(
    In(_params): In<Option<serde_json::Value>>,
    active_card_model: Res<ActiveCardModel>,
    flip_state: Res<CardFlipState>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
) -> bevy::remote::BrpResult {
    if let Some(screen_transition) = screen_transition.as_deref_mut() {
        screen_transition.request_view_change(*params.active_view, ActiveView::DebugScene);
    } else {
        let initial_rotation =
            composed_rotation_for_face(&params.card_state, flip_state.visible_face);
        params.transition_to_debug_scene(
            &active_card_model,
            flip_state.visible_face,
            initial_rotation,
        );
    }

    Ok(serde_json::json!({
        "active_view": "DebugScene",
        "success": true
    }))
}

/// HUMAN: Handles pointer navigation from scene card click back to GameScene.
/// AI: Deck scene blocks restart when the click resolves to a selectable deck card.
pub fn view_input_system(
    selected_modal: Option<Res<SelectedCardModalModel>>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
    mut active_card_model: ResMut<ActiveCardModel>,
    mut flip_state: ResMut<CardFlipState>,
) {
    if selected_modal.is_some_and(|modal| modal.blocks_lower_interactions()) {
        return;
    }
    let Ok(primary_window) = params.primary_window_query.single() else {
        return;
    };
    let Some(pointer_position) =
        just_pressed_pointer_position(primary_window, &params.mouse_buttons, &params.touches)
    else {
        return;
    };

    match *params.active_view {
        ActiveView::GameScene => {
            let _ = pointer_position;
            let _ = (&mut active_card_model, &mut flip_state);
        }
        ActiveView::DeckScene => {
            let app_camera = params
                .app_camera_query
                .single()
                .ok()
                .map(|(_, camera, transform)| (camera, transform));
            let is_card_hit = is_deck_card_hit(
                pointer_position,
                app_camera,
                params.deck_card_query.single().ok(),
                &params.card_defaults,
            );
            if !is_card_hit {
                return;
            }

            if let Some(screen_transition) = screen_transition.as_deref_mut() {
                screen_transition.request_view_change(*params.active_view, ActiveView::GameScene);
            } else {
                params.despawn_deck_scene();
                params.restart_game(&active_card_model);
            }
        }
        ActiveView::DebugScene => {
            let app_camera = params
                .app_camera_query
                .single()
                .ok()
                .map(|(_, camera, transform)| (camera, transform));
            let is_card_hit = is_deck_card_hit(
                pointer_position,
                app_camera,
                params.debug_card_query.single().ok(),
                &params.card_defaults,
            );
            if !is_card_hit {
                return;
            }

            if let Some(screen_transition) = screen_transition.as_deref_mut() {
                screen_transition.request_view_change(*params.active_view, ActiveView::GameScene);
            } else {
                params.despawn_debug_scene();
                params.restart_game(&active_card_model);
            }
        }
        ActiveView::MainMenuScene
        | ActiveView::LightningScene
        | ActiveView::MatchmakingScene
        | ActiveView::SettingsScene => {}
    }
}

/// HUMAN: Stops game- and deck-scene card clicks from forcing a scene change when selecting cards.
/// AI: DeckView cards now stay in SelectedInspection until the next outside click.
fn card_click_navigation(
    click: On<Pointer<Click>>,
    selected_modal: Option<Res<SelectedCardModalModel>>,
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
    active_card_model: Res<ActiveCardModel>,
    card_query: Query<(), (With<CardView>, With<DeckSceneEntity>)>,
    parent_query: Query<&ChildOf>,
) {
    if selected_modal.is_some_and(|modal| modal.blocks_lower_interactions()) {
        return;
    }

    let clicked_entity = click.original_event_target();

    if card_click_navigation_restarts_game_for_click(
        *params.active_view,
        clicked_entity,
        &card_query,
        &parent_query,
    ) {
        if let Some(screen_transition) = screen_transition.as_deref_mut() {
            screen_transition.request_view_change(*params.active_view, ActiveView::GameScene);
        } else {
            params.despawn_deck_scene();
            params.restart_game(&active_card_model);
        }
    }
}

/// HUMAN: Handles the actual clicked-card consequence for selectable screen cards.
/// AI: Pointer picking owns the real click path; coordinate hit-testing remains only a fallback.
fn card_click_selection(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    active_view: Res<ActiveView>,
    card_defaults: Res<CardInspectionDefaults>,
    mut selected_modal: ResMut<SelectedCardModalModel>,
    gesture_model: Res<CardGestureModel>,
    card_query: Query<(Entity, &SelectableCard, &Transform, Option<&ChildOf>), With<CardView>>,
    parent_query: Query<&ChildOf>,
    parent_transform_query: Query<&GlobalTransform>,
) {
    if selected_modal.is_active()
        || matches!(
            gesture_model.state,
            CardGestureState::Dragging | CardGestureState::Returning
        )
    {
        return;
    }

    let Some((entity, selectable, source_transform, parent)) =
        clicked_selectable_card(click.original_event_target(), &card_query, &parent_query)
    else {
        return;
    };
    if !card_click_selection_source_matches_view(selectable.source, *active_view) {
        return;
    }

    let target_transform = parent
        .and_then(|parent| parent_transform_query.get(parent).ok())
        .map(|parent_global_transform| {
            selected_inspection_transform_relative_to_parent(
                &card_defaults,
                parent_global_transform,
            )
        })
        .unwrap_or_else(|| selected_inspection_transform(&card_defaults));

    commands.entity(entity).remove::<CardAnimation>();
    selected_modal.select_entity(entity, source_transform, target_transform);
}

fn clicked_selectable_card(
    clicked_entity: Entity,
    card_query: &Query<(Entity, &SelectableCard, &Transform, Option<&ChildOf>), With<CardView>>,
    parent_query: &Query<&ChildOf>,
) -> Option<(Entity, SelectableCard, Transform, Option<Entity>)> {
    let mut current = Some(clicked_entity);

    while let Some(entity) = current {
        if let Ok((entity, selectable, transform, child_of)) = card_query.get(entity) {
            return Some((
                entity,
                *selectable,
                *transform,
                child_of.map(|child_of| child_of.parent()),
            ));
        }

        current = parent_query.get(entity).ok().map(|parent| parent.parent());
    }

    None
}

fn card_click_selection_source_matches_view(
    source: CardSelectionSource,
    active_view: ActiveView,
) -> bool {
    match source {
        CardSelectionSource::ScreenCard { view } => active_view == view,
        CardSelectionSource::CardViewBundle
        | CardSelectionSource::LocalHand { .. }
        | CardSelectionSource::LocalLocation { .. }
        | CardSelectionSource::OpponentHand { .. }
        | CardSelectionSource::OpponentLocation { .. } => false,
    }
}

fn selected_inspection_transform_relative_to_parent(
    card_defaults: &CardInspectionDefaults,
    parent_global_transform: &GlobalTransform,
) -> Transform {
    let target_transform = selected_inspection_transform(card_defaults);
    let local_matrix = parent_global_transform.affine().inverse()
        * GlobalTransform::from(target_transform).affine();
    Transform::from_matrix(Mat4::from(local_matrix))
}

fn card_click_navigation_restarts_game(active_view: ActiveView) -> bool {
    active_view == ActiveView::DeckScene
}

/// HUMAN: Allows scene-return navigation only when card selection should not handle the click.
/// AI: Deck-scene selected cards stay selected; everything else returns to game view.
fn card_click_navigation_restarts_game_for_click(
    active_view: ActiveView,
    clicked_entity: Entity,
    card_query: &Query<(), (With<CardView>, With<DeckSceneEntity>)>,
    parent_query: &Query<&ChildOf>,
) -> bool {
    if !card_click_navigation_restarts_game(active_view) {
        return false;
    }

    if deck_card_click_target_is_deck_scene_card(clicked_entity, card_query, parent_query) {
        return false;
    }

    true
}

/// HUMAN: Detects whether a click target belongs to a deck-scene rendered card.
/// AI: Keeps card clicks in-place so card inspection can own the click lifecycle.
fn deck_card_click_target_is_deck_scene_card(
    clicked_entity: Entity,
    card_query: &Query<(), (With<CardView>, With<DeckSceneEntity>)>,
    parent_query: &Query<&ChildOf>,
) -> bool {
    let mut current = Some(clicked_entity);

    while let Some(entity) = current {
        if card_query.get(entity).is_ok() {
            return true;
        }

        current = parent_query.get(entity).ok().map(|parent| parent.parent());
    }

    false
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
fn is_game_scene_card_hit(pointer_position: Vec2, window_size: Vec2) -> bool {
    game_scene_card_index_at(pointer_position, window_size).is_some()
}

#[cfg(test)]
fn game_scene_card_index_at(pointer_position: Vec2, window_size: Vec2) -> Option<usize> {
    game_scene_card_index_at_for_count(pointer_position, window_size, STARTING_HAND_CARD_COUNT)
}

fn game_scene_card_index_at_for_count(
    pointer_position: Vec2,
    window_size: Vec2,
    card_count: usize,
) -> Option<usize> {
    if window_size.x <= 0.0 || window_size.y <= 0.0 {
        return None;
    }

    let Some(pointer_position) = window_pointer_to_game_scene(pointer_position, window_size) else {
        return None;
    };
    game_scene_card_hitboxes_for_count(card_count)
        .iter()
        .rposition(|(min, max)| {
            pointer_position.x >= min.x
                && pointer_position.x <= max.x
                && pointer_position.y >= min.y
                && pointer_position.y <= max.y
        })
}

fn is_deck_card_hit(
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

fn window_pointer_to_game_scene(pointer_position: Vec2, window_size: Vec2) -> Option<Vec2> {
    let game_scene_size = Vec2::new(GAME_SCENE_WIDTH, GAME_SCENE_HEIGHT);
    let scale = (window_size.x / game_scene_size.x).min(window_size.y / game_scene_size.y);
    if scale <= 0.0 {
        return None;
    }

    let scaled_game_scene_size = game_scene_size * scale;
    let offset = (window_size - scaled_game_scene_size) * 0.5;
    let pointer_position = (pointer_position - offset) / scale;

    (pointer_position.x >= 0.0
        && pointer_position.x <= GAME_SCENE_WIDTH
        && pointer_position.y >= 0.0
        && pointer_position.y <= GAME_SCENE_HEIGHT)
        .then_some(pointer_position)
}

#[cfg(test)]
fn game_scene_pointer_to_window(pointer_position: Vec2, window_size: Vec2) -> Vec2 {
    let game_scene_size = Vec2::new(GAME_SCENE_WIDTH, GAME_SCENE_HEIGHT);
    let scale = (window_size.x / game_scene_size.x).min(window_size.y / game_scene_size.y);
    let scaled_game_scene_size = game_scene_size * scale;
    let offset = (window_size - scaled_game_scene_size) * 0.5;

    offset + (pointer_position * scale)
}

#[cfg(test)]
fn game_scene_card_hitboxes() -> Vec<(Vec2, Vec2)> {
    game_scene_card_hitboxes_for_count(STARTING_HAND_CARD_COUNT)
}

// HUMAN: Builds hand-row hitboxes directly from hand area geometry for stable gestures.
// AI: Uses the hand-area card size for row centering and spacing behavior.
fn game_scene_card_hitboxes_for_count(card_count: usize) -> Vec<(Vec2, Vec2)> {
    game_scene_card_hitboxes_for_count_with_hover(card_count, None)
}

fn game_scene_card_hitboxes_for_count_with_hover(
    card_count: usize,
    hovered_index: Option<usize>,
) -> Vec<(Vec2, Vec2)> {
    if card_count == 0 {
        return Vec::new();
    }

    let hand_min = game_scene_hand_area_min();
    let hand_size = game_scene_hand_area_size();
    let card_size = game_scene_hand_card_size();
    let row_height = card_size.y;
    let centers = game_scene_hand_card_centers(card_count, hovered_index, hand_min, hand_size);
    let row_min_y = hand_min.y + ((hand_size.y - row_height) * 0.5).max(0.0);

    centers
        .into_iter()
        .map(|center_x| {
            let card_min = Vec2::new(center_x - (card_size.x * 0.5), row_min_y);
            (card_min, card_min + card_size.min(hand_size))
        })
        .collect()
}

fn game_scene_hand_card_centers(
    card_count: usize,
    hovered_index: Option<usize>,
    hand_min: Vec2,
    hand_size: Vec2,
) -> Vec<f32> {
    let card_width = game_scene_hand_card_size().x.min(hand_size.x);
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

fn game_scene_hand_card_z(card_index: usize, hovered_index: Option<usize>) -> f32 {
    if hovered_index == Some(card_index) {
        GAME_SCENE_HAND_CARD_HOVER_Z
    } else {
        GAME_SCENE_HAND_CARD_WORLD_Z + (card_index as f32 * GAME_SCENE_HAND_CARD_Z_STEP)
    }
}

fn is_game_scene_active(active_view: Option<&ActiveView>) -> bool {
    active_view.is_none_or(|active_view| *active_view == ActiveView::GameScene)
}

/// HUMAN: Clears GameScene button interactions while the selected-card modal owns the pointer.
/// AI: Run before button action systems so modal capture blocks lower UI presses at the source.
pub fn modal_block_game_control_interactions_system(
    selected_modal: Option<Res<SelectedCardModalModel>>,
    mut interaction_query: Query<&mut Interaction, With<GameControlButton>>,
) {
    if !selected_modal.is_some_and(|modal| modal.blocks_lower_interactions()) {
        return;
    }

    for mut interaction in &mut interaction_query {
        if *interaction != Interaction::None {
            *interaction = Interaction::None;
        }
    }
}

/// HUMAN: Enables DeckScreen fullscreen blur only while prompt overlays are visible.
/// AI: Prompt visibility controls camera post-process attachment so blur matches modal lifetime.
pub fn modal_backdrop_blur_update_system(
    mut commands: Commands,
    deck_screen_model: Option<Res<DeckScreenModel>>,
    camera_query: Query<
        (Entity, Option<&DeckPromptBackdropBlur>),
        (With<DeckSceneEntity>, With<Camera3d>),
    >,
) {
    let should_enable = deck_screen_model
        .as_ref()
        .is_some_and(|model| model.validation_prompt || model.coming_soon_prompt);

    for (entity, blur) in &camera_query {
        if should_enable {
            if blur.is_none() {
                commands
                    .entity(entity)
                    .insert(DeckPromptBackdropBlur::default());
            }
        } else if blur.is_some() {
            commands.entity(entity).remove::<DeckPromptBackdropBlur>();
        }
    }
}

/// HUMAN: Plays shared click feedback for accepted GameScreen control buttons.
/// AI: Keep this separate from update_end_round_button to avoid growing that system's param set.
pub fn game_control_audio_update_system(
    active_view: Option<Res<ActiveView>>,
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    game_round_model: Option<Res<GameRoundModel>>,
    match_model: Option<Res<MatchModel>>,
    pending_round_deal: Option<Res<PendingRoundDealResource>>,
    button_query: Query<(&Interaction, &GameControlButton), Changed<Interaction>>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

    for (interaction, control) in &button_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if game_control_action_is_disabled(
            control.action,
            game_round_model.as_deref(),
            match_model.as_deref(),
            pending_round_deal.as_deref(),
        ) {
            continue;
        }
        request_button_click(audio_manager.as_deref_mut());
    }
}

/// HUMAN: Applies a consistent hover treatment to all shared UI buttons.
/// AI: Hover is always 20% brighter than each button's normal/base color.
pub fn button_ui_hover_update_system(
    mut button_query: Query<
        (&Interaction, &ButtonUiBaseColor, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonUiStyle>),
    >,
) {
    for (interaction, base_color, mut background) in &mut button_query {
        background.0 = match *interaction {
            Interaction::Hovered => brighten_color(base_color.0, 1.2),
            Interaction::Pressed | Interaction::None => base_color.0,
        };
    }
}

fn brighten_color(color: Color, factor: f32) -> Color {
    let srgba = color.to_srgba();
    Color::srgba(
        (srgba.red * factor).clamp(0.0, 1.0),
        (srgba.green * factor).clamp(0.0, 1.0),
        (srgba.blue * factor).clamp(0.0, 1.0),
        srgba.alpha,
    )
}

fn request_button_click(audio_manager: Option<&mut AudioManagerModel>) {
    if let Some(audio_manager) = audio_manager {
        audio_manager.request(AudioEnum::ButtonClick);
    }
}

fn request_card_flip(audio_manager: Option<&mut AudioManagerModel>) {
    if let Some(audio_manager) = audio_manager {
        audio_manager.request(AudioEnum::CardFlip);
    }
}

#[derive(SystemParam)]
pub struct GameControlUpdateParams<'w, 's> {
    active_view: Option<Res<'w, ActiveView>>,
    button_query: Query<
        'w,
        's,
        (
            &'static Interaction,
            &'static GameControlButton,
            &'static mut BackgroundColor,
            &'static mut BorderColor,
        ),
        Changed<Interaction>,
    >,
    game_deck_model: Option<ResMut<'w, GameDeckModel>>,
    game_hand_model: Option<ResMut<'w, GameHandModel>>,
    game_round_model: Option<ResMut<'w, GameRoundModel>>,
    card_model_registry: Option<Res<'w, CardModelRegistry>>,
    game_location_model: Option<ResMut<'w, GameLocationModel>>,
    location_model_registry: Option<Res<'w, LocationModelRegistry>>,
    active_locations: Option<ResMut<'w, ActiveLocations>>,
    active_world_model: Option<ResMut<'w, ActiveWorldModel>>,
    match_model: Option<ResMut<'w, MatchModel>>,
    player_deck_collection: Option<Res<'w, PlayerDeckCollectionModel>>,
    persistent_match_mode: Option<ResMut<'w, Persistent<MatchModePreferenceStore>>>,
    slot_board: Option<ResMut<'w, CardSlotBoardModel>>,
    card_states: Option<ResMut<'w, CardStateModel>>,
    gesture_model: Option<ResMut<'w, CardGestureModel>>,
    cpu_brain_model: Option<ResMut<'w, CpuBrainModel>>,
    pending_round_deal: Option<ResMut<'w, PendingRoundDealResource>>,
}

impl GameControlUpdateParams<'_, '_> {
    fn handle_action(&mut self, action: GameControlAction) {
        match action {
            GameControlAction::QuitGame => {}
            GameControlAction::Mode => self.handle_mode_action(),
            GameControlAction::EndRound => self.handle_end_round_action(),
            GameControlAction::Restart => self.handle_restart_action(),
            GameControlAction::Undo => self.handle_undo_action(),
        }
    }

    fn handle_mode_action(&mut self) {
        let Some(match_model) = self.match_model.as_deref_mut() else {
            return;
        };
        let next_mode = match_model.mode.next();
        if let Some(persistent_match_mode) = self.persistent_match_mode.as_deref_mut()
            && let Err(error) = persistent_match_mode.set(MatchModePreferenceStore {
                selected_mode: next_mode,
            })
        {
            warn!("Failed to save match mode preference: {error}");
        }
        if let (
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(game_location_model),
            Some(slot_board),
            Some(card_states),
            Some(gesture_model),
        ) = (
            self.game_deck_model.as_deref_mut(),
            self.game_hand_model.as_deref_mut(),
            self.game_round_model.as_deref_mut(),
            self.game_location_model.as_deref_mut(),
            self.slot_board.as_deref_mut(),
            self.card_states.as_deref_mut(),
            self.gesture_model.as_deref_mut(),
        ) {
            *slot_board = CardSlotBoardModel::default();
            *gesture_model = CardGestureModel::default();
            reset_two_player_match(
                next_mode,
                match_model,
                game_deck_model,
                game_hand_model,
                game_round_model,
                game_location_model,
                self.location_model_registry.as_deref(),
                self.active_locations.as_deref_mut(),
                self.active_world_model.as_deref(),
                self.player_deck_collection
                    .as_deref()
                    .and_then(PlayerDeckCollectionModel::primary_deck),
            );
            card_states.reset_to_size(game_hand_model.len());
            if let Some(pending_round_deal) = self.pending_round_deal.as_deref_mut() {
                pending_round_deal.is_pending = false;
                pending_round_deal.is_round_deal_complete = true;
                pending_round_deal.waits_for_location_intro = false;
            }
            if let Some(cpu_brain_model) = self.cpu_brain_model.as_deref_mut() {
                cpu_brain_model.reset();
            }
        }
    }

    fn handle_end_round_action(&mut self) {
        if let (
            Some(game_deck_model),
            Some(game_hand_model),
            Some(game_round_model),
            Some(match_model),
            Some(slot_board),
        ) = (
            self.game_deck_model.as_deref_mut(),
            self.game_hand_model.as_deref_mut(),
            self.game_round_model.as_deref_mut(),
            self.match_model.as_deref_mut(),
            self.slot_board.as_deref_mut(),
        ) {
            if !game_round_model.can_end_round() {
                return;
            }
            sync_near_human_from_game_models(
                match_model,
                game_deck_model,
                game_hand_model,
                game_round_model,
            );
            let human_moves: Vec<_> = game_round_model
                .current_round_moves
                .iter()
                .map(|record| (record.location_index, record.slot_index))
                .collect();
            for (location_index, slot_index) in human_moves {
                if !match_model.placements.iter().any(|placement| {
                    placement.owner == MatchPlayerSide::Near
                        && placement.location_index == location_index
                        && placement.slot_index == slot_index
                        && placement.placement_round == match_model.round.round
                }) {
                    match_model.record_placement(MatchPlayerSide::Near, location_index, slot_index);
                }
            }
            match_model.near.ready_for_next = true;
            prepare_cpu_players_for_next_click(
                match_model,
                slot_board,
                self.card_model_registry.as_deref(),
                self.cpu_brain_model.as_deref_mut(),
            );
            resolve_match_readiness(match_model, slot_board);
        }
    }

    fn handle_restart_action(&mut self) {
        restart_game_model(
            self.gesture_model.as_deref_mut(),
            self.slot_board.as_deref_mut(),
            self.card_states.as_deref_mut(),
            self.game_deck_model.as_deref_mut(),
            self.game_hand_model.as_deref_mut(),
            self.game_round_model.as_deref_mut(),
            self.game_location_model.as_deref_mut(),
            self.location_model_registry.as_deref(),
            self.active_locations.as_deref_mut(),
            self.active_world_model.as_deref_mut(),
            None,
            self.match_model.as_deref_mut(),
            self.player_deck_collection.as_deref(),
            self.cpu_brain_model.as_deref_mut(),
            self.pending_round_deal.as_deref_mut(),
        );
    }

    fn handle_undo_action(&mut self) {
        if let (Some(game_round_model), Some(slot_board), Some(card_states), Some(gesture_model)) = (
            self.game_round_model.as_deref_mut(),
            self.slot_board.as_deref_mut(),
            self.card_states.as_deref_mut(),
            self.gesture_model.as_deref_mut(),
        ) && game_round_model.has_undoable_moves()
        {
            let moves: Vec<_> = game_round_model.current_round_moves.drain(..).collect();
            for record in moves {
                slot_board.remove_local_card(record.hand_index);
                card_states.return_to_hand(record.hand_index);
                game_round_model.restore(record.energy_cost);
            }
            *gesture_model = CardGestureModel::default();
        }
    }
}

fn prepare_cpu_players_for_next_click(
    match_model: &mut MatchModel,
    slot_board: &CardSlotBoardModel,
    card_model_registry: Option<&CardModelRegistry>,
    mut cpu_brain_model: Option<&mut CpuBrainModel>,
) {
    for side in [MatchPlayerSide::Near, MatchPlayerSide::Far] {
        if !match_model.player(side).controller.is_cpu() || match_model.player(side).ready_for_next
        {
            continue;
        }

        if !match_model.has_pending_cpu_placements()
            && let Some(card_model_registry) = card_model_registry
        {
            let selected_moves = choose_level1_moves(
                match_model,
                side,
                slot_board,
                card_model_registry,
                cpu_brain_model
                    .as_deref()
                    .map(|brain| brain.seed)
                    .unwrap_or_default(),
            );
            if !selected_moves.is_empty() {
                match_model.queue_cpu_placements(selected_moves);
            }
        }

        match_model.player_mut(side).ready_for_next = true;
        if let Some(cpu_brain_model) = cpu_brain_model.as_deref_mut() {
            cpu_brain_model.schedule_next(side);
        }
    }
}

pub fn update_end_round_button(mut params: GameControlUpdateParams) {
    if !is_game_scene_active(params.active_view.as_deref()) {
        return;
    }

    if card_gesture_blocks_game_controls(params.gesture_model.as_deref()) {
        return;
    }

    let mut actions_to_handle = Vec::new();
    for (interaction, control, mut background, mut border) in &mut params.button_query {
        let is_disabled = game_control_action_is_disabled(
            control.action,
            params.game_round_model.as_deref(),
            params.match_model.as_deref(),
            params.pending_round_deal.as_deref(),
        );
        let (background_color, border_color) = match *interaction {
            Interaction::Pressed => {
                if !is_disabled {
                    actions_to_handle.push(control.action);
                }
                (
                    END_ROUND_BUTTON_PRESSED_COLOR,
                    END_ROUND_BUTTON_PRESSED_BORDER_COLOR,
                )
            }
            Interaction::Hovered => (
                brighten_color(END_ROUND_BUTTON_NORMAL_COLOR, 1.2),
                brighten_color(END_ROUND_BUTTON_NORMAL_BORDER_COLOR, 1.2),
            ),
            Interaction::None => (
                END_ROUND_BUTTON_NORMAL_COLOR,
                END_ROUND_BUTTON_NORMAL_BORDER_COLOR,
            ),
        };
        if is_disabled {
            background.0 = GAME_CONTROL_DISABLED_COLOR;
            *border = BorderColor::all(GAME_CONTROL_DISABLED_BORDER_COLOR);
        } else {
            background.0 = background_color;
            *border = BorderColor::all(border_color);
        }
    }

    for action in actions_to_handle {
        params.handle_action(action);
    }
}

/// HUMAN: Routes the GameScreen Quit Game control back to the main menu.
/// AI: Keep this separate from gameplay action handling to avoid borrowing game-round models.
pub fn quit_game_control_update_system(
    mut screen_transition: Option<ResMut<ScreenTransitionResource>>,
    mut params: ViewChangeParams,
    mut button_query: Query<(&Interaction, &GameControlButton), Changed<Interaction>>,
) {
    if *params.active_view != ActiveView::GameScene {
        return;
    }

    for (interaction, control) in &mut button_query {
        if *interaction == Interaction::Pressed && control.action == GameControlAction::QuitGame {
            if let Some(screen_transition) = screen_transition.as_deref_mut() {
                screen_transition
                    .request_view_change(*params.active_view, ActiveView::MainMenuScene);
            } else {
                params.transition_to_main_menu_scene();
            }
        }
    }
}

/// HUMAN: Handles the Restart button using the same GameScreen entry path as matchmaking.
/// AI: Route restart controls through ViewChangeParams::restart_game for exact parity.
pub fn restart_game_control_button_system(
    button_query: Query<(&Interaction, &GameControlButton), Changed<Interaction>>,
    active_card_model: Res<ActiveCardModel>,
    selected_modal: Option<Res<SelectedCardModalModel>>,
    mut params: ViewChangeParams,
) {
    if *params.active_view != ActiveView::GameScene {
        return;
    }
    if selected_modal.is_some_and(|modal| modal.blocks_lower_interactions()) {
        return;
    }
    if !button_query.iter().any(|(interaction, control)| {
        *interaction == Interaction::Pressed && control.action == GameControlAction::Restart
    }) {
        return;
    }

    params.restart_game(&active_card_model);
}

fn card_gesture_blocks_game_controls(gesture_model: Option<&CardGestureModel>) -> bool {
    gesture_model.is_some_and(|gesture_model| {
        !matches!(
            gesture_model.state,
            CardGestureState::Idle | CardGestureState::Placed
        )
    })
}

/// HUMAN: Keeps GameScene control labels and disabled states synced to gameplay state.
/// AI: Run this separately from button interactions so round, energy, and undo text stay live.
pub fn update_game_control_ui_system(
    active_view: Option<Res<ActiveView>>,
    selected_modal: Option<Res<SelectedCardModalModel>>,
    game_round_model: Option<Res<GameRoundModel>>,
    match_model: Option<Res<MatchModel>>,
    pending_round_deal: Option<Res<PendingRoundDealResource>>,
    mut text_queries: ParamSet<(
        Query<(&GameControlLabel, &mut Text)>,
        Query<&mut Text, With<MatchStatusText>>,
    )>,
    mut button_query: Query<(
        &Interaction,
        &GameControlButton,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

    let Some(game_round_model) = game_round_model.as_deref() else {
        return;
    };

    {
        let mut label_query = text_queries.p0();
        for (label, mut text) in &mut label_query {
            match label.action {
                GameControlAction::Mode => {
                    if let Some(match_model) = match_model.as_deref() {
                        text.0 = match_model.mode.label().to_string();
                    }
                }
                GameControlAction::QuitGame => {}
                GameControlAction::Undo => {
                    text.0 = game_round_model.energy_label();
                }
                GameControlAction::EndRound => {
                    text.0 = format!(
                        "Round {}/{}",
                        game_round_model.round, game_round_model.max_rounds
                    );
                }
                GameControlAction::Restart => {}
            }
        }
    }

    if let Some(match_model) = match_model.as_deref() {
        let mut status_query = text_queries.p1();
        for mut text in &mut status_query {
            text.0 = match_model.status_text();
        }
    }

    for (interaction, control, mut background, mut border) in &mut button_query {
        if selected_modal
            .as_ref()
            .is_some_and(|modal| modal.blocks_lower_interactions())
        {
            background.0 = END_ROUND_BUTTON_NORMAL_COLOR;
            *border = BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR);
            continue;
        }
        if game_control_action_is_disabled(
            control.action,
            Some(game_round_model),
            match_model.as_deref(),
            pending_round_deal.as_deref(),
        ) {
            background.0 = GAME_CONTROL_DISABLED_COLOR;
            *border = BorderColor::all(GAME_CONTROL_DISABLED_BORDER_COLOR);
            continue;
        }

        let (background_color, border_color) = match *interaction {
            Interaction::Pressed => (
                END_ROUND_BUTTON_PRESSED_COLOR,
                END_ROUND_BUTTON_PRESSED_BORDER_COLOR,
            ),
            Interaction::Hovered => (
                brighten_color(END_ROUND_BUTTON_NORMAL_COLOR, 1.2),
                brighten_color(END_ROUND_BUTTON_NORMAL_BORDER_COLOR, 1.2),
            ),
            Interaction::None => (
                END_ROUND_BUTTON_NORMAL_COLOR,
                END_ROUND_BUTTON_NORMAL_BORDER_COLOR,
            ),
        };
        background.0 = background_color;
        *border = BorderColor::all(border_color);
    }
}

fn game_control_action_is_disabled(
    action: GameControlAction,
    game_round_model: Option<&GameRoundModel>,
    match_model: Option<&MatchModel>,
    pending_round_deal: Option<&PendingRoundDealResource>,
) -> bool {
    match action {
        GameControlAction::Mode => false,
        GameControlAction::QuitGame => false,
        GameControlAction::Restart => false,
        GameControlAction::Undo => {
            match_model.is_some_and(|model| model.near.controller.is_cpu())
                || game_round_model.is_some_and(|model| !model.has_undoable_moves())
        }
        GameControlAction::EndRound => {
            pending_round_deal.is_some_and(|deal| deal.is_pending || !deal.is_round_deal_complete)
                || match_model.is_some_and(|model| {
                    model.near.controller.is_cpu() || model.near.ready_for_next
                })
                || game_round_model.is_some_and(|model| !model.can_end_round())
        }
    }
}

/// HUMAN: Resolves round readiness for the two-player match.
/// AI: Commit hidden CPU cards first; reveal and advance run after animations settle.
pub fn resolve_match_readiness(match_model: &mut MatchModel, slot_board: &mut CardSlotBoardModel) {
    if !match_model.both_ready() || match_model.is_complete() {
        return;
    }

    if match_model.resolution_phase != MatchResolutionPhase::Planning {
        return;
    }

    if commit_pending_cpu_placements(match_model, slot_board) > 0 {
        match_model.resolution_phase = MatchResolutionPhase::CpuPlacementsMoving;
        return;
    }

    match_model.begin_current_round_reveal();
    match_model.resolution_phase = MatchResolutionPhase::CpuPlacementsRevealing;
}

fn commit_pending_cpu_placements(
    match_model: &mut MatchModel,
    slot_board: &mut CardSlotBoardModel,
) -> usize {
    let pending_moves = std::mem::take(&mut match_model.pending_cpu_placements);
    let mut committed_count = 0;
    for pending_move in pending_moves {
        let side = if match_model
            .near
            .hand_instance_ids
            .contains(&pending_move.instance_id)
        {
            MatchPlayerSide::Near
        } else {
            MatchPlayerSide::Far
        };
        let player = match_model.player_mut(side);
        if pending_move.energy_cost > player.energy_available {
            continue;
        }
        let source_hand_index = player
            .hand_instance_ids
            .iter()
            .position(|instance_id| *instance_id == pending_move.instance_id)
            .unwrap_or(pending_move.hand_index);
        let source_hand_count = player.hand.len();
        let Some((_, card_id)) = player.remove_hand_card_by_instance_id(pending_move.instance_id)
        else {
            continue;
        };
        player.energy_available -= pending_move.energy_cost;
        let slot_hand_index = cpu_slot_hand_index(side, player.next_slot_card_index());
        if slot_board.place_for_side_with_card_id(
            pending_move.location_index,
            side.slot_side(),
            pending_move.slot_index,
            slot_hand_index,
            card_id,
        ) {
            match_model.record_placement(
                side,
                pending_move.location_index,
                pending_move.slot_index,
            );
            match_model.record_cpu_placement_motion_source(CpuPlacementMotionSourceModel {
                owner: side,
                location_index: pending_move.location_index,
                slot_index: pending_move.slot_index,
                hand_index: source_hand_index,
                hand_count: source_hand_count,
            });
            committed_count += 1;
        }
    }

    committed_count
}

/// HUMAN: Advances the staged post-Next reveal flow once CPU card animations finish.
/// AI: Keeps face-down placement, reveal, scoring, and new-round setup in separate frames.
pub fn staged_match_resolution_system(
    active_view: Option<Res<ActiveView>>,
    time: Res<Time>,
    card_model_registry: Res<CardModelRegistry>,
    mut match_model: ResMut<MatchModel>,
    mut game_round_model: ResMut<GameRoundModel>,
    mut game_location_model: ResMut<GameLocationModel>,
    mut game_deck_model: ResMut<GameDeckModel>,
    mut game_hand_model: ResMut<GameHandModel>,
    mut card_states: ResMut<CardStateModel>,
    mut pending_round_deal: Option<ResMut<PendingRoundDealResource>>,
    slot_board: Res<CardSlotBoardModel>,
    animation_query: Query<&CardAnimation>,
    cpu_card_query: Query<&CpuPlacedCardView>,
) {
    if !is_game_scene_active(active_view.as_deref()) || match_model.is_complete() {
        return;
    }

    match match_model.resolution_phase {
        MatchResolutionPhase::Planning => {}
        MatchResolutionPhase::CpuPlacementsMoving => {
            let cpu_cards_still_moving = animation_query.iter().any(|animation| {
                matches!(
                    animation.phase,
                    CardAnimationPhase::MovingToHand | CardAnimationPhase::MovingToSlot
                )
            });
            if cpu_cards_still_moving {
                return;
            }
            match_model.begin_current_round_reveal();
            match_model.resolution_phase = MatchResolutionPhase::CpuPlacementsRevealing;
        }
        MatchResolutionPhase::CpuPlacementsRevealing => {
            let has_rendered_cpu_cards = cpu_card_query.iter().next().is_some();
            if animation_query.iter().next().is_some() {
                return;
            }
            if match_model.complete_revealing_current_round_placements() > 0 {
                if has_rendered_cpu_cards
                    && !match_model
                        .current_round_reveal_targets(&slot_board)
                        .is_empty()
                {
                    match_model.next_reveal_delay_seconds = CPU_CARD_REVEAL_STAGGER_SECONDS;
                }
                return;
            }
            if has_rendered_cpu_cards && match_model.tick_next_reveal_delay(time.delta_secs()) {
                return;
            }
            if match_model
                .start_next_current_round_reveal(&slot_board)
                .is_some()
            {
                return;
            }
            finish_revealed_match_round(
                &mut match_model,
                &mut game_round_model,
                &mut game_location_model,
                &mut game_deck_model,
                &mut game_hand_model,
                &mut card_states,
                pending_round_deal.as_deref_mut(),
                &slot_board,
                &card_model_registry,
            );
        }
    }
}

/// HUMAN: Turns current-round local placements face-down as soon as round resolution starts.
/// AI: Reuses passive per-card face control so reveal animation affects only moved local cards.
pub fn prepare_local_current_round_reveals_system(
    mut commands: Commands,
    active_view: Option<Res<ActiveView>>,
    match_model: Res<MatchModel>,
    game_round_model: Res<GameRoundModel>,
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    card_query: Query<(Entity, &HandCardGestureTarget), Without<CpuPlacedCardView>>,
    children_query: Query<&Children>,
    face_layer_query: Query<(), With<CardFaceLayer>>,
) {
    if !is_game_scene_active(active_view.as_deref())
        || match_model.resolution_phase == MatchResolutionPhase::Planning
        || game_round_model.current_round_moves.is_empty()
    {
        return;
    }

    let mut prepared_count = 0;
    for record in &game_round_model.current_round_moves {
        let Some((entity, target)) = card_query
            .iter()
            .find(|(_, target)| target.hand_index == record.hand_index)
        else {
            continue;
        };
        mark_card_face_layers_controlled(entity, &mut commands, &children_query, &face_layer_query);
        commands
            .entity(entity)
            .remove::<(
                CardGestureView,
                HandCardGestureTarget,
                LocalPlayerHandCardPreview,
            )>()
            .insert((
                CpuPlacedCardView::new(
                    MatchPlayerSide::Near,
                    CardSlotSide::LocalPlayer,
                    record.location_index,
                    record.slot_index,
                    record.card_id.clone(),
                    CardFace::Back,
                ),
                SelectableCard::new(CardSelectionSource::LocalLocation {
                    location_index: record.location_index,
                    slot_index: record.slot_index,
                    hand_index: target.hand_index,
                }),
            ));
        prepared_count += 1;
    }

    if prepared_count > 0 {
        request_card_flip(audio_manager.as_deref_mut());
    }
}

fn mark_card_face_layers_controlled(
    entity: Entity,
    commands: &mut Commands,
    children_query: &Query<&Children>,
    face_layer_query: &Query<(), With<CardFaceLayer>>,
) {
    if face_layer_query.get(entity).is_ok() {
        commands.entity(entity).insert(CardAnimationFaceLayer);
    }
    let Ok(children) = children_query.get(entity) else {
        return;
    };
    for child in children.iter() {
        mark_card_face_layers_controlled(child, commands, children_query, face_layer_query);
    }
}

fn finish_revealed_match_round(
    match_model: &mut MatchModel,
    game_round_model: &mut GameRoundModel,
    game_location_model: &mut GameLocationModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    card_states: &mut CardStateModel,
    pending_round_deal: Option<&mut PendingRoundDealResource>,
    slot_board: &CardSlotBoardModel,
    card_model_registry: &CardModelRegistry,
) {
    card_states.lock_location_cards();
    if game_round_model.round >= game_round_model.max_rounds {
        let winner_side =
            final_winner_from_slots(slot_board, card_model_registry, Some(game_location_model));
        match_model.round.winner = Some(MatchWinnerModel {
            side: winner_side,
            controller: match_model.controller_for_winner_side(winner_side),
        });
        game_round_model.end_round_resolved = true;
        match_model.resolution_phase = MatchResolutionPhase::Planning;
        return;
    }

    if game_round_model.advance_round() {
        match_model.round.round = game_round_model.round;
        game_location_model.set_round(game_round_model.round);
        if let Some(pending_round_deal) = pending_round_deal {
            pending_round_deal.is_pending = true;
            pending_round_deal.is_round_deal_complete = false;
            pending_round_deal.waits_for_location_intro = false;
        } else {
            start_match_round(
                match_model,
                game_round_model,
                game_deck_model,
                game_hand_model,
            );
            card_states.ensure_size(game_hand_model.len());
        }
    }
    match_model.resolution_phase = MatchResolutionPhase::Planning;
}

/// HUMAN: Runs paced CPU controller choices for any CPU-owned player.
/// AI: Applies one legal move or readiness decision per elapsed CPU timer.
pub fn cpu_brain_update_system(
    active_view: Option<Res<ActiveView>>,
    time: Res<Time>,
    card_model_registry: Res<CardModelRegistry>,
    mut match_model: ResMut<MatchModel>,
    mut cpu_brain: ResMut<CpuBrainModel>,
    mut slot_board: ResMut<CardSlotBoardModel>,
    pending_round_deal: Option<Res<PendingRoundDealResource>>,
    cpu_hand_query: Query<(&CpuHandCardView, Option<&CardAnimation>)>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

    if match_model.is_complete() {
        return;
    }
    if pending_round_deal
        .as_deref()
        .is_some_and(|deal| deal.is_pending || !deal.is_round_deal_complete)
    {
        return;
    }

    for side in [MatchPlayerSide::Near, MatchPlayerSide::Far] {
        if !match_model.player(side).controller.is_cpu() || match_model.player(side).ready_for_next
        {
            continue;
        }
        let hand_cards_are_settled =
            cpu_hand_cards_are_settled_for_planning(side, &match_model, &cpu_hand_query);
        if cpu_brain.wait_for_settled_hand_pause(
            side,
            match_model.round.round,
            match_model.player(side).hand.len(),
            hand_cards_are_settled,
            time.delta_secs(),
            CPU_HAND_SETTLED_PAUSE_SECONDS,
        ) {
            continue;
        }
        if !cpu_brain.tick(side, time.delta_secs()) {
            continue;
        }

        let selected_moves = choose_level1_moves(
            &match_model,
            side,
            &slot_board,
            &card_model_registry,
            cpu_brain.seed,
        );
        if selected_moves.is_empty() {
            match_model.player_mut(side).ready_for_next = true;
        } else {
            match_model.queue_cpu_placements(selected_moves);
            match_model.player_mut(side).ready_for_next = true;
        }
        cpu_brain.schedule_next(side);
    }

    resolve_match_readiness(&mut match_model, &mut slot_board);
}

fn cpu_hand_cards_are_settled_for_planning(
    side: MatchPlayerSide,
    match_model: &MatchModel,
    cpu_hand_query: &Query<(&CpuHandCardView, Option<&CardAnimation>)>,
) -> bool {
    let player = match_model.player(side);
    if player.hand.is_empty() {
        return true;
    }

    let expected_instance_ids: std::collections::HashSet<u64> =
        player.hand_instance_ids.iter().copied().collect();
    if expected_instance_ids.is_empty() {
        return true;
    }

    let mut seen_instance_ids = std::collections::HashSet::new();
    for (view, animation) in cpu_hand_query.iter() {
        if view.owner != side || !expected_instance_ids.contains(&view.instance_id) {
            continue;
        }
        if animation.is_some() {
            return false;
        }
        seen_instance_ids.insert(view.instance_id);
    }

    if seen_instance_ids.is_empty() || seen_instance_ids == expected_instance_ids {
        return true;
    }

    // HUMAN: Treat partial passive-hand visibility as settled when no matching card is animating.
    // AI: Prevent readiness deadlocks when a CPU hand card view fails to render this frame.
    true
}

/// HUMAN: Keeps passive CPU hand cards visible while CPU players prepare moves.
/// AI: This is presentation-only; MatchModel remains the hand authority.
pub fn sync_cpu_hand_card_entities_system(
    mut commands: Commands,
    active_view: Option<Res<ActiveView>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    match_model: Res<MatchModel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
    mut hand_query: Query<(Entity, &mut CpuHandCardView)>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        let _ = commands;
        let _ = hand_query;
        return;
    }

    let mut expected = std::collections::HashMap::new();
    for owner in [MatchPlayerSide::Near, MatchPlayerSide::Far] {
        let player = match_model.player(owner);
        if !player.controller.is_cpu() {
            continue;
        }
        let visible_face = cpu_card_hand_visible_face(owner);
        for (hand_index, card_id) in player.hand.iter().enumerate() {
            let Some(instance_id) = player.hand_instance_id(hand_index) else {
                continue;
            };
            expected.insert(
                (owner, instance_id, card_id.clone()),
                (hand_index, visible_face),
            );
        }
    }

    for (entity, view) in &mut hand_query {
        let key = (view.owner, view.instance_id, view.card_id.clone());
        if expected.remove(&key).is_none() {
            commands.entity(entity).despawn();
        }
    }

    for ((owner, instance_id, card_id), (hand_index, visible_face)) in expected {
        let Some(card_model) = card_model_registry.card_model_for_id(&card_id).cloned() else {
            continue;
        };
        let hand_count = match_model.player(owner).hand.len();
        let hand_transform = cpu_card_hand_transform(owner, hand_index, hand_count, &card_defaults);
        let source_transform = cpu_card_deck_transform(owner, hand_transform);
        let card = spawn_card_structure_for_type(
            &mut commands,
            &asset_server,
            &card_defaults,
            card_model,
            &mut meshes,
            &mut materials,
            masked_background_materials.as_deref_mut(),
            visible_face,
            true,
            source_transform,
        );
        commands.entity(card).insert((
            GameSceneEntity,
            CpuHandCardView::new(owner, instance_id, hand_index, card_id, visible_face),
            SelectableCard::new(CardSelectionSource::OpponentHand { owner, hand_index }),
            CardAnimation::move_to_hand(source_transform, hand_transform, visible_face),
        ));
    }
}

/// HUMAN: Keeps rendered CPU-placed card entities in sync with populated CPU slots.
/// AI: CPU cards are passive and intentionally lack gesture/hover markers.
pub fn sync_cpu_placed_card_entities_system(
    mut commands: Commands,
    active_view: Option<Res<ActiveView>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    mut match_model: ResMut<MatchModel>,
    slot_board: Res<CardSlotBoardModel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    mut card_query: Query<(
        Entity,
        &mut CpuPlacedCardView,
        &Transform,
        Option<&CardAnimation>,
        Option<&CardAnimationMarker>,
    )>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        let _ = commands;
        let _ = card_query;
        return;
    }

    let mut expected = std::collections::HashMap::new();
    for slot in slot_board.slots() {
        let owner = match slot.side {
            CardSlotSide::LocalPlayer => MatchPlayerSide::Near,
            CardSlotSide::Opponent => MatchPlayerSide::Far,
        };
        let CardSlotState::Populated { card_id, .. } = &slot.state else {
            continue;
        };
        let visible_face = cpu_card_slot_visible_face(
            owner,
            match_model.placement_visibility(owner, slot.location_index, slot.slot_index),
        );
        expected.insert(
            (
                slot.side,
                slot.location_index,
                slot.slot_index,
                card_id.clone(),
            ),
            (owner, visible_face),
        );
    }

    for (entity, mut view, _, animation, animation_marker) in &mut card_query {
        let key = (
            view.side,
            view.location_index,
            view.slot_index,
            view.card_id.clone(),
        );
        let Some((_, visible_face)) = expected.remove(&key) else {
            commands.entity(entity).despawn();
            continue;
        };
        if view.visible_face != visible_face && visible_face == CardFace::Front {
            if let Some(animation) = animation {
                if animation.phase == CardAnimationPhase::Revealing {
                    view.visible_face = visible_face;
                }
                continue;
            }
            if animation_marker.is_some_and(|marker| marker.phase == CardAnimationPhase::Revealing)
            {
                view.visible_face = visible_face;
                continue;
            }
            view.visible_face = visible_face;
            let slot_transform = slot_transform(
                view.location_index,
                view.slot_index,
                view.side,
                &slot_board,
                &card_defaults,
            );
            commands
                .entity(entity)
                .insert(CardAnimation::swan_flip_to_front(slot_transform, 0.0));
            request_card_flip(audio_manager.as_deref_mut());
        }
    }

    for ((side, location_index, slot_index, card_id), (owner, visible_face)) in expected {
        if !match_model.player(owner).controller.is_cpu() {
            continue;
        }
        let Some(card_model) = card_model_registry.card_model_for_id(&card_id).cloned() else {
            continue;
        };
        let target_transform = slot_transform(
            location_index,
            slot_index,
            side,
            &slot_board,
            &card_defaults,
        );
        let hand_transform = match_model
            .take_cpu_placement_motion_source(owner, location_index, slot_index)
            .map(|source| {
                cpu_card_hand_transform(owner, source.hand_index, source.hand_count, &card_defaults)
            })
            .unwrap_or_else(|| cpu_card_move_source_hand_transform(owner, target_transform));
        let source_transform = hand_transform;
        let card = spawn_card_structure_for_type(
            &mut commands,
            &asset_server,
            &card_defaults,
            card_model,
            &mut meshes,
            &mut materials,
            masked_background_materials.as_deref_mut(),
            cpu_card_hand_visible_face(owner),
            true,
            source_transform,
        );
        commands.entity(card).insert((
            GameSceneEntity,
            CpuPlacedCardView::new(
                owner,
                side,
                location_index,
                slot_index,
                card_id,
                cpu_card_hand_visible_face(owner),
            ),
            SelectableCard::new(CardSelectionSource::OpponentLocation {
                owner,
                side,
                location_index,
                slot_index,
            }),
            CardAnimation::move_hand_to_slot(hand_transform, target_transform, visible_face),
        ));
    }
}

fn cpu_card_deck_transform(owner: MatchPlayerSide, target_transform: Transform) -> Transform {
    let source_y = match owner {
        MatchPlayerSide::Near => GAME_SCENE_LOCAL_HAND_DEAL_SOURCE_Y,
        MatchPlayerSide::Far => -120.0,
    };
    let source_position = game_scene_world_position_from_game_scene(
        Vec2::new(GAME_SCENE_WIDTH * 0.5, source_y),
        target_transform.translation.z,
    );
    Transform {
        translation: source_position,
        rotation: target_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI),
        scale: target_transform.scale,
    }
}

fn cpu_card_hand_visible_face(owner: MatchPlayerSide) -> CardFace {
    let _ = owner;
    CardFace::Back
}

fn cpu_card_slot_visible_face(
    _owner: MatchPlayerSide,
    placement_visibility: PlacementVisibility,
) -> CardFace {
    match placement_visibility {
        PlacementVisibility::CurrentRoundHidden => CardFace::Back,
        PlacementVisibility::Revealing | PlacementVisibility::Revealed => CardFace::Front,
    }
}

fn cpu_card_hand_transform(
    owner: MatchPlayerSide,
    hand_index: usize,
    hand_count: usize,
    card_defaults: &CardInspectionDefaults,
) -> Transform {
    let card_size = game_scene_hand_card_size();
    let hand_z = game_scene_hand_card_z(hand_index, None);
    let card_world_scale =
        game_scene_world_height_for_game_scene_height(card_size.y, hand_z) / card_defaults.height;
    let hitboxes = game_scene_card_hitboxes_for_count(hand_count);
    let hand_position = if owner == MatchPlayerSide::Near {
        let (card_min, card_max) = hitboxes[hand_index];
        game_scene_world_position_from_game_scene((card_min + card_max) * 0.5, hand_z)
    } else {
        let (card_min, card_max) = hitboxes[hand_index];
        game_scene_world_position_from_game_scene(
            Vec2::new((card_min.x + card_max.x) * 0.5, GAME_SCENE_FAR_HAND_Y),
            hand_z,
        )
    };
    Transform {
        translation: hand_position,
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(card_world_scale),
    }
}

fn cpu_card_move_source_hand_transform(
    owner: MatchPlayerSide,
    target_transform: Transform,
) -> Transform {
    let hand_y = match owner {
        MatchPlayerSide::Near => GAME_SCENE_HAND_TOP + (GAME_SCENE_HAND_HEIGHT * 0.5),
        MatchPlayerSide::Far => GAME_SCENE_FAR_HAND_Y,
    };
    let hand_position = game_scene_world_position_from_game_scene(
        Vec2::new(GAME_SCENE_WIDTH * 0.5, hand_y),
        target_transform.translation.z,
    );
    Transform {
        translation: hand_position,
        rotation: target_transform.rotation,
        scale: target_transform.scale,
    }
}

/// HUMAN: Marks card roots with their current animation owner while tweens are active.
/// AI: Sync systems can trust this marker instead of reapplying in-flight tweens.
pub fn card_animation_marker_system(
    mut commands: Commands,
    animation_query: Query<(Entity, &CardAnimation), Changed<CardAnimation>>,
    stale_marker_query: Query<Entity, (With<CardAnimationMarker>, Without<CardAnimation>)>,
) {
    for (entity, animation) in &animation_query {
        commands
            .entity(entity)
            .insert(CardAnimationMarker::from(*animation));
    }

    for entity in &stale_marker_query {
        commands.entity(entity).remove::<CardAnimationMarker>();
    }
}

/// HUMAN: Animates card roots as they move into slots and reveal.
/// AI: Presentation-only tweening; ownership and winner logic stay in resources.
pub fn card_animation_system(
    active_view: Option<Res<ActiveView>>,
    time: Res<Time>,
    mut commands: Commands,
    mut audio_manager: Option<ResMut<AudioManagerModel>>,
    mut card_query: Query<(Entity, &mut Transform, &mut CardAnimation)>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

    for (entity, mut transform, mut animation) in &mut card_query {
        if advance_card_animation(
            time.delta_secs(),
            &mut transform,
            &mut animation,
            audio_manager.as_deref_mut(),
        ) {
            commands.entity(entity).remove::<CardAnimation>();
        }
    }
}

fn advance_card_animation(
    delta_seconds: f32,
    transform: &mut Transform,
    animation: &mut CardAnimation,
    mut audio_manager: Option<&mut AudioManagerModel>,
) -> bool {
    let mut active_delta_seconds = delta_seconds.max(0.0);
    if animation.start_delay_seconds > 0.0 {
        if active_delta_seconds < animation.start_delay_seconds {
            animation.start_delay_seconds -= active_delta_seconds;
            return false;
        }
        active_delta_seconds -= animation.start_delay_seconds;
        animation.start_delay_seconds = 0.0;
    }

    animation.phase_elapsed_seconds += active_delta_seconds;

    if animation.phase == CardAnimationPhase::Revealing {
        let preset = match animation.flip_style {
            CardAnimationFlipStyle::Standard => GameTweenPreset::Flip,
            CardAnimationFlipStyle::Swan => GameTweenPreset::SwanFlip,
        };
        let reveal_duration_seconds = match animation.flip_style {
            CardAnimationFlipStyle::Standard => GAME_TWEEN_FLIP_SECONDS,
            CardAnimationFlipStyle::Swan => GAME_TWEEN_SWAN_FLIP_SECONDS,
        };
        let reveal_duration_complete = animation.phase_elapsed_seconds >= reveal_duration_seconds;
        animation.current_y_rotation = sample_flip_y_rotation(
            std::f32::consts::PI,
            animation.target_y_rotation,
            animation.phase_elapsed_seconds,
            preset,
        )
        .0;
        transform.translation = animation.target_transform.translation;
        transform.scale = match animation.flip_style {
            CardAnimationFlipStyle::Standard => animation.target_transform.scale,
            CardAnimationFlipStyle::Swan => {
                let swan_elapsed = animation.phase_elapsed_seconds.max(0.0);
                if swan_elapsed >= GAME_TWEEN_SWAN_SCALE_UP_SECONDS
                    && !animation.swan_peak_sfx_played
                {
                    if let Some(audio_manager) = audio_manager.as_deref_mut() {
                        audio_manager.request(AudioEnum::CardSwanPeak);
                    }
                    animation.swan_peak_sfx_played = true;
                }
                let scale_multiplier = sample_swan_scale_multiplier(swan_elapsed);
                animation.target_transform.scale * scale_multiplier
            }
        };
        let target_game_scene_position =
            game_scene_position_from_world_position(animation.target_transform.translation);
        if animation.flip_style == CardAnimationFlipStyle::Swan && !reveal_duration_complete {
            transform.translation.z = CPU_CARD_MOVING_FRONT_Z;
            transform.translation = game_scene_world_position_from_game_scene(
                target_game_scene_position,
                transform.translation.z,
            );
        } else {
            transform.translation = animation.target_transform.translation;
        }
    } else {
        let move_seconds = match animation.phase {
            CardAnimationPhase::MovingToHand => GAME_TWEEN_DEAL_SLIDE_SECONDS,
            CardAnimationPhase::MovingToSlot => GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS,
            CardAnimationPhase::Revealing => GAME_TWEEN_CARD_MOVE_TO_SLOT_SECONDS,
        };
        let progress = (animation.phase_elapsed_seconds / move_seconds).clamp(0.0, 1.0);
        let eased_progress = ease_out_cubic(progress);
        let translation_z = if progress < 1.0 {
            CPU_CARD_MOVING_FRONT_Z
        } else {
            animation.target_transform.translation.z
        };
        transform.translation = cpu_card_move_translation(
            animation.phase_start_transform.translation,
            animation.target_transform.translation,
            translation_z,
            eased_progress,
        );
        transform.scale = if progress < 1.0 {
            let scale_multiplier = match animation.phase {
                CardAnimationPhase::MovingToHand => 1.0,
                CardAnimationPhase::MovingToSlot => sample_card_move_scale_multiplier(progress),
                CardAnimationPhase::Revealing => 1.0,
            };
            cpu_card_move_scale(
                animation.phase_start_transform,
                animation.target_transform,
                translation_z,
                eased_progress,
                scale_multiplier,
            )
        } else {
            animation.target_transform.scale
        };
    }
    transform.rotation =
        animation.target_transform.rotation * Quat::from_rotation_y(animation.current_y_rotation);

    let reveal_duration_complete = animation.phase != CardAnimationPhase::Revealing
        || animation.phase_elapsed_seconds
            >= match animation.flip_style {
                CardAnimationFlipStyle::Standard => GAME_TWEEN_FLIP_SECONDS,
                CardAnimationFlipStyle::Swan => GAME_TWEEN_SWAN_FLIP_SECONDS,
            };
    let is_settled = reveal_duration_complete
        && transform
            .translation
            .distance(animation.target_transform.translation)
            <= CPU_CARD_ANIMATION_SETTLE_EPSILON
        && transform.scale.distance(animation.target_transform.scale)
            <= CPU_CARD_ANIMATION_SETTLE_EPSILON
        && (animation.target_y_rotation - animation.current_y_rotation).abs()
            <= CPU_CARD_ANIMATION_SETTLE_EPSILON;
    if is_settled {
        if animation.phase == CardAnimationPhase::Revealing
            && animation.flip_style == CardAnimationFlipStyle::Swan
            && !animation.swan_land_sfx_played
        {
            if let Some(audio_manager) = audio_manager.as_deref_mut() {
                audio_manager.request(AudioEnum::CardSwanLand);
            }
            animation.swan_land_sfx_played = true;
        }
        *transform = animation.target_transform;
        transform.rotation = animation.target_transform.rotation
            * Quat::from_rotation_y(animation.target_y_rotation);
        if animation.phase == CardAnimationPhase::MovingToHand {
            animation.phase_start_transform = *transform;
            animation.target_transform = animation.slot_transform;
            animation.phase_elapsed_seconds = 0.0;
            animation.phase = CardAnimationPhase::MovingToSlot;
            return false;
        }
    }
    is_settled
}

fn cpu_card_move_translation(
    start_translation: Vec3,
    target_translation: Vec3,
    current_z: f32,
    eased_progress: f32,
) -> Vec3 {
    let start_game_scene_position = game_scene_position_from_world_position(start_translation);
    let target_game_scene_position = game_scene_position_from_world_position(target_translation);
    game_scene_world_position_from_game_scene(
        start_game_scene_position.lerp(target_game_scene_position, eased_progress),
        current_z,
    )
}

#[cfg(test)]
fn cpu_card_move_scale_multiplier(progress: f32) -> f32 {
    sample_card_move_scale_multiplier(progress)
}

fn cpu_card_move_scale(
    start_transform: Transform,
    target_transform: Transform,
    current_z: f32,
    eased_progress: f32,
    scale_multiplier: f32,
) -> Vec3 {
    let start_world_units_per_pixel =
        game_scene_world_units_per_game_scene_pixel(start_transform.translation.z);
    let target_world_units_per_pixel =
        game_scene_world_units_per_game_scene_pixel(target_transform.translation.z);
    let current_world_units_per_pixel = game_scene_world_units_per_game_scene_pixel(current_z);
    sample_card_move_scale(
        start_transform,
        target_transform,
        start_world_units_per_pixel,
        target_world_units_per_pixel,
        current_world_units_per_pixel,
        eased_progress,
        scale_multiplier,
    )
}

/// HUMAN: Shows card fronts or backs according to each card's own reveal tween.
/// AI: This prevents per-card reveal from depending on the global debug card flip state.
pub fn update_card_animation_face_visibility_system(
    active_view: Option<Res<ActiveView>>,
    card_ui_state: Res<CardUiState>,
    cpu_card_query: Query<(&CpuPlacedCardView, Option<&CardAnimation>)>,
    cpu_hand_query: Query<(&CpuHandCardView, Option<&CardAnimation>)>,
    mut face_query: Query<
        (
            &ChildOf,
            &CardFaceLayer,
            Option<&CardParallaxLayer>,
            &mut Visibility,
        ),
        With<CardAnimationFaceLayer>,
    >,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

    for (child_of, face_layer, parallax_layer, mut visibility) in &mut face_query {
        let visible_face = if let Ok((view, animation)) = cpu_card_query.get(child_of.parent()) {
            animation
                .map(|animation| animation.current_face())
                .unwrap_or(view.visible_face)
        } else if let Ok((view, animation)) = cpu_hand_query.get(child_of.parent()) {
            animation
                .map(|animation| animation.current_face())
                .unwrap_or(view.visible_face)
        } else {
            continue;
        };
        let is_hidden_safe_area = parallax_layer
            .is_some_and(|layer| layer.role == CardLayerRole::SafeArea)
            && !card_ui_state.show_safe_area;
        *visibility = if face_layer.face == visible_face && !is_hidden_safe_area {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
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
    location_model_registry: Res<LocationModelRegistry>,
    mut active_locations: ResMut<ActiveLocations>,
    active_world_model: Res<ActiveWorldModel>,
    mut match_model: ResMut<MatchModel>,
    mut card_states: ResMut<CardStateModel>,
) {
    reset_two_player_match(
        match_model.mode,
        &mut match_model,
        &mut game_deck_model,
        &mut game_hand_model,
        &mut game_round_model,
        &mut game_location_model,
        Some(&location_model_registry),
        Some(&mut active_locations),
        Some(&active_world_model),
        player_deck_collection.primary_deck(),
    );
    card_states.reset_to_size(game_hand_model.len());
}

/// HUMAN: Loads the persisted match mode preference into transient match state.
/// AI: Persistence owns only selected mode; game state is rebuilt separately.
pub fn load_saved_match_mode_preference(
    mut match_model: ResMut<MatchModel>,
    persistent_match_mode: Option<Res<Persistent<MatchModePreferenceStore>>>,
) {
    if let Some(store) = persistent_match_mode {
        match_model.mode = store.selected_mode;
    }
}

/// HUMAN: Loads persisted pre-game settings into the runtime settings resource.
/// AI: Keep match mode mirrored to MatchModel for existing game setup code.
pub fn load_saved_meta_game_settings(
    mut settings: ResMut<MetaGameSettingsModel>,
    mut match_model: ResMut<MatchModel>,
    persistent_settings: Option<Res<Persistent<MetaGameSettingsModel>>>,
    #[cfg(not(target_arch = "wasm32"))] mut winit_settings: Option<ResMut<WinitSettings>>,
) {
    if let Some(store) = persistent_settings {
        *settings = (**store).clone();
    }
    settings.normalize_framerate();
    match_model.mode = settings.selected_mode;
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(winit_settings) = winit_settings.as_deref_mut() {
        apply_meta_game_framerate_settings(&settings, winit_settings);
    }
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
            Text::new("Screen: GameScreen\nFrame: 0\nKEYS: "),
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
            spawn_key_span(parent, "T", KeyCode::KeyT, false);
            parent.spawn((TextSpan::new("\nKEYS: "), debug_hud_text_font()));
            spawn_key_span(parent, "[D]", KeyCode::KeyD, true);
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
    visibility_query: Query<'w, 's, &'static mut Visibility, With<DebugHudText>>,
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
                KeyCode::KeyD => params.hud_state.is_debug_drawing_visible(),
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
        ActiveView::MainMenuScene => "MainMenuScreen",
        ActiveView::LightningScene => "LightningScreen",
        ActiveView::MatchmakingScene => "MatchmakingScreen",
        ActiveView::GameScene => "GameScreen",
        ActiveView::DeckScene => "DeckScreen",
        ActiveView::SettingsScene => "SettingsScreen",
        ActiveView::DebugScene => "DebugScreen",
    };
    let full_text = format!("Screen: {scene_name}\nFrame: {}\nKEYS: ", params.ticks.0);
    for mut text in &mut params.text_query {
        *text = Text::new(full_text.clone());
    }
    for mut visibility in &mut params.visibility_query {
        *visibility = if *params.active_view == ActiveView::DeckScene {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
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

/// HUMAN: Keeps the persistent DebugHUD rendered by the shared AppScene camera.
/// AI: UiTargetCamera is only honored on root UI nodes, so update the DebugHUD root ancestor.
pub fn sync_debug_hud_ui_camera_system(
    mut commands: Commands,
    child_of_query: Query<&ChildOf>,
    app_camera_query: Query<Entity, With<AppSceneCamera>>,
    hud_query: Query<Entity, With<DebugHudText>>,
    target_camera_query: Query<&UiTargetCamera>,
) {
    let Ok(app_camera) = app_camera_query.single() else {
        return;
    };

    commands.entity(app_camera).insert(PrimaryEguiContext);

    for hud in &hud_query {
        let root = ui_root_for_entity(hud, &child_of_query);
        if target_camera_query
            .get(root)
            .is_ok_and(|target_camera| target_camera.0 == app_camera)
        {
            continue;
        }
        commands.entity(root).insert(UiTargetCamera(app_camera));
    }
}

fn ui_root_for_entity(entity: Entity, child_of_query: &Query<&ChildOf>) -> Entity {
    let mut root = entity;
    while let Ok(child_of) = child_of_query.get(root) {
        root = child_of.parent();
    }
    root
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
        let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        hud_state.debug_draw_mode = if shift_pressed {
            hud_state.debug_draw_mode.toggle_solo()
        } else {
            hud_state.debug_draw_mode.toggle_standard()
        };
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
    if world
        .get_resource::<SelectedCardModalModel>()
        .is_some_and(SelectedCardModalModel::blocks_lower_interactions)
    {
        return;
    }
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
                ui.heading("deck");
                bevy_inspector::ui_for_entities_filtered(world, ui, true, &InspectorEntityFilter);
                ui.allocate_space(ui.available_size());
            });
        });
}

pub fn card_ui(world: &mut World) {
    if world
        .get_resource::<SelectedCardModalModel>()
        .is_some_and(SelectedCardModalModel::blocks_lower_interactions)
    {
        return;
    }
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
            .unwrap_or(Vec2::new(GAME_SCENE_WIDTH, GAME_SCENE_HEIGHT)),
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
        if let Some(mut audio_manager) = world.get_resource_mut::<AudioManagerModel>() {
            audio_manager.request(AudioEnum::CardFlip);
        }
    }
}

pub fn transition_egui_overlay_ui(world: &mut World) {
    let Some((phase, overlay_alpha, color)) =
        world
            .get_resource::<ScreenTransitionResource>()
            .map(|transition| {
                (
                    transition.phase,
                    transition.overlay_alpha.clamp(0.0, 1.0),
                    transition.color,
                )
            })
    else {
        return;
    };
    if phase == ScreenTransitionPhase::Idle && overlay_alpha <= 0.0 {
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
    let painter = egui_context.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("screen_transition_overlay"),
    ));
    painter.rect_filled(
        egui_context.content_rect(),
        0.0,
        egui_color_from_bevy(color, overlay_alpha),
    );
}

fn should_show_card_ui(active_view: ActiveView) -> bool {
    matches!(active_view, ActiveView::DebugScene)
}

fn egui_color_from_bevy(color: Color, alpha_multiplier: f32) -> egui::Color32 {
    let srgba = color.to_srgba();
    egui::Color32::from_rgba_unmultiplied(
        (srgba.red.clamp(0.0, 1.0) * 255.0).round() as u8,
        (srgba.green.clamp(0.0, 1.0) * 255.0).round() as u8,
        (srgba.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
        ((srgba.alpha * alpha_multiplier).clamp(0.0, 1.0) * 255.0).round() as u8,
    )
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
    let Some((safe_area_margin, scale)) = game_scene_layout(window_size) else {
        return egui::vec2(
            -SCREEN_PADDING_LEFT,
            SCREEN_PADDING_TOP + DEBUG_SCENE_CARD_VERTICAL_OFFSET,
        );
    };

    egui::vec2(
        -(safe_area_margin.x + (SCREEN_PADDING_LEFT * scale)),
        safe_area_margin.y + ((SCREEN_PADDING_TOP + DEBUG_SCENE_CARD_VERTICAL_OFFSET) * scale),
    )
}

fn game_scene_layout(window_size: Vec2) -> Option<(Vec2, f32)> {
    if window_size.x <= 0.0 || window_size.y <= 0.0 {
        return None;
    }

    let game_scene_size = Vec2::new(GAME_SCENE_WIDTH, GAME_SCENE_HEIGHT);
    let scale = (window_size.x / game_scene_size.x).min(window_size.y / game_scene_size.y);
    if scale <= 0.0 {
        return None;
    }

    let safe_area_margin = ((window_size - (game_scene_size * scale)) * 0.5).max(Vec2::ZERO);
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
#[path = "../../tests/runtime/systems/systems_mod_tests.rs"]
mod systems_mod_tests;
