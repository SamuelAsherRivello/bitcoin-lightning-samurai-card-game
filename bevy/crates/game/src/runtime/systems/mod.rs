use bevy::{
    app::AppExit,
    asset::RenderAssetUsages,
    camera::{
        ClearColorConfig, ScalingMode, Viewport,
        visibility::{NoCpuCulling, RenderLayers},
    },
    ecs::system::SystemParam,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::view::NoIndirectDrawing,
    text::{Underline, UnderlineColor},
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

pub mod debug_drawing_update_system;

pub use debug_drawing_update_system::*;

use crate::runtime::components::{
    AppSceneEntity, AppSceneRoot, CardBackgroundLayer, CardBrowserViewEntity, CardBrowserViewRoot,
    CardFaceLayer, CardFrameLayer, CardLayerRole, CardParallaxLayer, CardView, CardViewBundle,
    CostPointView, DebugHudFpsText, DebugHudKeyText, DebugHudText, EndTurnButton, GameLocation,
    GameViewEntity, GameViewRoot, InspectorState, LocalPlayerHand, LocalPlayerHandCardPreview,
    LocationRevealState, Player, PowerPointView, PrimaryViewCamera, TurnUi, WorldBackground,
};
use crate::runtime::materials::CardBackgroundMaskMaterial;
use crate::runtime::resources::{
    ActiveCardModel, ActiveLocations, ActiveView, ActiveWorldModel, CARD_BACK_TEXTURE_PATH,
    CARD_DEPTH_FACTOR_DEFAULT, CARD_DEPTH_FACTOR_MAX, CARD_DEPTH_FACTOR_MIN, CARD_LAYER_SCALE_MAX,
    CARD_LAYER_SCALE_MIN, CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT, CARD_SAFE_AREA_TEXTURE_PATH,
    CardFace, CardFlipState, CardInspectionDefaults, CardInspectionState, CardModel,
    CardModelRegistry, CardSettingsStore, CardUiState, CostPointModel, DebugHudInputStore,
    DebugHudState, GameTicks, LocationModelRegistry, LocationScoreModel,
    PRIMARY_CAMERA_FOV_RADIANS, PowerPointModel, PrimaryCameraDefaults, WindowPlacement,
    WindowPlacementState, WindowPlacementStore, WorldModelRegistry, load_window_placement,
    valid_window_placement,
};

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
const GAME_ROUND_CURRENT: u32 = 1;
const GAME_ROUND_TOTAL: u32 = 6;
const GAME_VIEW_ASPECT_RATIO: f32 = GAME_VIEW_WIDTH / GAME_VIEW_HEIGHT;
const GAME_SCENE_HAND_LEFT_PERCENT: f32 = 24.0;
const GAME_SCENE_HAND_BOTTOM_PERCENT: f32 = 2.5;
const GAME_SCENE_HAND_WIDTH_PERCENT: f32 = 52.0;
const GAME_SCENE_HAND_HEIGHT_PERCENT: f32 = 25.0;
const GAME_SCENE_HAND_CARD_HEIGHT: f32 = 192.0;
const GAME_SCENE_HAND_CARD_WIDTH: f32 =
    GAME_SCENE_HAND_CARD_HEIGHT * CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT;
const GAME_SCENE_HAND_CARD_GAP: f32 = 16.0;
const GAME_SCENE_HAND_CARD_WORLD_Z: f32 = 0.32;
const GAME_SCENE_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.33;
const GAME_SCENE_WORLD_BACKGROUND_BLEED: f32 = 1.18;
const GAME_SCENE_WORLD_BACKGROUND_Z: f32 = -0.16;
const CARD_RENDER_LAYER: usize = 1;
const GAME_SCENE_CARD_TILT_RADIANS: f32 = 0.07;
const CARD_BROWSER_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.33;
const CARD_BROWSER_CARD_HEIGHT_FRACTION: f32 = 0.9;
const DEBUG_HUD_Z_INDEX: i32 = 100;
const END_TURN_BUTTON_NORMAL_COLOR: Color = Color::srgba(0.22, 0.04, 0.44, 0.82);
const END_TURN_BUTTON_HOVER_COLOR: Color = Color::srgba(0.36, 0.08, 0.68, 0.9);
const END_TURN_BUTTON_PRESSED_COLOR: Color = Color::srgba(0.12, 0.02, 0.28, 0.95);
const END_TURN_BUTTON_NORMAL_BORDER_COLOR: Color = Color::srgb(0.45, 0.18, 0.9);
const END_TURN_BUTTON_HOVER_BORDER_COLOR: Color = Color::srgb(0.7, 0.42, 1.0);
const END_TURN_BUTTON_PRESSED_BORDER_COLOR: Color = Color::srgb(0.95, 0.82, 1.0);
const POINT_VIEW_WIDTH: f32 = 46.0;
const POINT_VIEW_HEIGHT: f32 = 36.0;
const CARD_POINT_BADGE_SIZE: f32 = 0.17;
const CARD_POINT_BADGE_INSET_RATIO: f32 = 0.16;
const CARD_POINT_DIGIT_WIDTH: f32 = 0.04;
const CARD_POINT_DIGIT_HEIGHT: f32 = 0.076;
const CARD_POINT_DIGIT_STROKE: f32 = 0.01;
const CARD_POINT_DIGIT_GAP: f32 = 0.004;

/// HUMAN: Spawns the local player entity for the app.
/// AI: Startup system; keep player setup separate from AppScene and view setup.
pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn setup_primary_camera(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    spawn_primary_camera(&mut commands, &camera_defaults);
}

pub fn constrain_card_browser_camera_to_safe_area(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<
        &mut Camera,
        (
            With<PrimaryViewCamera>,
            With<CardBrowserViewEntity>,
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

/// HUMAN: Keeps GameView 3D cameras aligned with the aspect-ratio-safe area.
/// AI: Avoid native fullscreen scissor validation by using the surface-sized default viewport there.
pub fn constrain_game_view_3d_cameras_to_safe_area(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Camera, (With<GameViewEntity>, With<Camera3d>)>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    let safe_area_viewport = game_view_safe_area_viewport_for_window(window);

    for mut camera in &mut camera_query {
        camera.viewport = safe_area_viewport.clone();
    }
}

fn game_view_safe_area_viewport_for_window(window: &Window) -> Option<Viewport> {
    if should_use_default_camera_viewport(window) {
        return None;
    }

    game_view_safe_area_viewport(window.resolution.physical_size())
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
    camera_transform.position.z = CARD_BROWSER_CAMERA_DISTANCE_FROM_ORIGIN;

    commands
        .spawn((
            Name::new("Primary 3D Camera"),
            PrimaryViewCamera,
            CardBrowserViewEntity,
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

fn spawn_card_browser_ui_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("CardBrowserView UI Camera"),
            CardBrowserViewEntity,
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

#[cfg_attr(feature = "desktop-hot-reload", hot)]
/// HUMAN: Spawns the persistent AppScene and debug HUD.
/// AI: AppScene remains present while GameView and CardBrowserView swap on top.
pub fn setup_app_scene(mut commands: Commands, hud: Option<Res<Hud>>) {
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
pub fn setup_game_view(
    mut commands: Commands,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    hud: Option<Res<Hud>>,
    asset_server: Res<AssetServer>,
    camera_defaults: Option<Res<PrimaryCameraDefaults>>,
    card_defaults: Res<CardInspectionDefaults>,
    card_model_registry: Res<CardModelRegistry>,
    active_card_model: Res<ActiveCardModel>,
    world_model_registry: Res<WorldModelRegistry>,
    active_world_model: Res<ActiveWorldModel>,
    location_model_registry: Res<LocationModelRegistry>,
    active_locations: Res<ActiveLocations>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    masked_background_materials: Option<ResMut<Assets<CardBackgroundMaskMaterial>>>,
) {
    let fallback_camera_defaults = PrimaryCameraDefaults::default();
    let camera_defaults = camera_defaults
        .as_deref()
        .unwrap_or(&fallback_camera_defaults);
    spawn_game_view_contents(
        &mut commands,
        app_scene_query.single().ok(),
        hud.as_ref().map(|hud| hud.0),
        &asset_server,
        camera_defaults,
        &card_defaults,
        &card_model_registry,
        &active_card_model,
        &world_model_registry,
        &active_world_model,
        &location_model_registry,
        &active_locations,
        &mut meshes,
        &mut materials,
        masked_background_materials.map(|materials| materials.into_inner()),
    );
}

fn spawn_game_view_contents(
    commands: &mut Commands,
    app_scene_parent: Option<Entity>,
    hud_parent: Option<Entity>,
    asset_server: &AssetServer,
    camera_defaults: &PrimaryCameraDefaults,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
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
        );
    });
    let scene_entity = scene.id();
    spawn_game_view_camera(commands);
    spawn_game_view_card_camera(commands, camera_defaults);
    spawn_game_view_card_overlay_camera(commands, camera_defaults);
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
        meshes,
        materials,
        masked_background_materials.as_deref_mut(),
    );

    if let Some(parent) = hud_parent.or(app_scene_parent) {
        commands.entity(parent).add_child(scene_entity);
    }
    let _ = active_card_model;
}

fn spawn_game_view_ui(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    location_model_registry: &LocationModelRegistry,
    active_locations: &ActiveLocations,
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
            spawn_location_row(
                parent,
                asset_server,
                location_model_registry,
                active_locations,
            );
            spawn_local_player_hand(parent);
            spawn_turn_ui(parent);
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

fn spawn_location_row(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    location_model_registry: &LocationModelRegistry,
    active_locations: &ActiveLocations,
) {
    parent
        .spawn((
            Name::new("GameView Location Row"),
            GameViewEntity,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(28.0),
                top: Val::Percent(28.0),
                width: Val::Percent(44.0),
                height: Val::Percent(30.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            GlobalZIndex(10),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            for (index, location) in location_model_registry
                .selected_locations(active_locations)
                .into_iter()
                .enumerate()
            {
                spawn_location_ui(
                    parent,
                    asset_server,
                    index,
                    location.display_name,
                    location.texture,
                );
            }
        });
}

fn spawn_location_ui(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    index: usize,
    display_name: &'static str,
    texture: &'static str,
) {
    let mut location = parent.spawn((
        Name::new(format!("Game Location {}", index + 1)),
        GameLocation::new(index, LocationRevealState::Revealed),
        ImageNode::new(asset_server.load(texture)),
        Node {
            width: Val::Px(168.0),
            height: Val::Px(216.0),
            border: UiRect::all(Val::Px(4.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..Default::default()
        },
        Visibility::Visible,
    ));
    location.insert(BorderColor::all(Color::srgb(0.58, 0.47, 0.31)));
    location.with_children(|parent| {
        let location_score = LocationScoreModel::empty(index);
        spawn_power_point_view(parent, location_score.opponent_total);
        spawn_location_text(parent, display_name, 18.0);
        spawn_power_point_view(parent, location_score.local_total);
    });
}

fn game_view_perspective_view_size_at_z(z: f32) -> Vec2 {
    let distance = (GAME_SCENE_CAMERA_DISTANCE_FROM_ORIGIN - z).abs();
    let height = 2.0 * (PRIMARY_CAMERA_FOV_RADIANS * 0.5).tan() * distance;

    Vec2::new(height * GAME_VIEW_ASPECT_RATIO, height)
}

fn game_view_world_position_from_game_view(game_view_position: Vec2, z: f32) -> Vec3 {
    let view_size = game_view_perspective_view_size_at_z(z);

    Vec3::new(
        ((game_view_position.x / GAME_VIEW_WIDTH) - 0.5) * view_size.x,
        (0.5 - (game_view_position.y / GAME_VIEW_HEIGHT)) * view_size.y,
        z,
    )
}

fn game_view_world_height_for_game_view_height(game_view_height: f32, z: f32) -> f32 {
    game_view_perspective_view_size_at_z(z).y * (game_view_height / GAME_VIEW_HEIGHT)
}

fn game_view_world_background_size() -> Vec2 {
    game_view_perspective_view_size_at_z(GAME_SCENE_WORLD_BACKGROUND_Z)
        * GAME_SCENE_WORLD_BACKGROUND_BLEED
}

fn card_browser_perspective_view_height_at_z(z: f32) -> f32 {
    let distance = (CARD_BROWSER_CAMERA_DISTANCE_FROM_ORIGIN - z).abs();
    2.0 * (PRIMARY_CAMERA_FOV_RADIANS * 0.5).tan() * distance
}

fn card_browser_centered_card_scale(card_defaults: &CardInspectionDefaults) -> f32 {
    (card_browser_perspective_view_height_at_z(0.0) * CARD_BROWSER_CARD_HEIGHT_FRACTION)
        / card_defaults.height
}

fn spawn_location_text(parent: &mut ChildSpawnerCommands, text: &'static str, font_size: f32) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    ));
}

fn spawn_power_point_view(parent: &mut ChildSpawnerCommands, model: PowerPointModel) {
    parent
        .spawn((
            Name::new("PowerPointView"),
            PowerPointView::new(model),
            Node {
                width: Val::Px(POINT_VIEW_WIDTH),
                height: Val::Px(POINT_VIEW_HEIGHT),
                border: UiRect::all(Val::Px(2.0)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..Default::default()
            },
            BorderColor::all(Color::srgb(0.82, 0.62, 0.2)),
            BackgroundColor(Color::srgba(0.08, 0.02, 0.02, 0.82)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(model.display_text()),
                TextFont {
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_card_cost_point_view(
    parent: &mut ChildSpawnerCommands,
    model: CostPointModel,
    background_mesh: Handle<Mesh>,
    background_material: Handle<StandardMaterial>,
    horizontal_digit_mesh: Handle<Mesh>,
    vertical_digit_mesh: Handle<Mesh>,
    digit_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
) {
    spawn_card_point_background(
        parent,
        Name::new("Card CostPointView Background"),
        CostPointView::new(model),
        background_mesh,
        background_material,
        background_translation,
        is_visible,
    );
    spawn_card_point_text(
        parent,
        Name::new("Card CostPointView Text"),
        model.display_text(),
        horizontal_digit_mesh,
        vertical_digit_mesh,
        digit_material,
        text_translation,
        is_visible,
    );
}

fn spawn_card_power_point_view(
    parent: &mut ChildSpawnerCommands,
    model: PowerPointModel,
    background_mesh: Handle<Mesh>,
    background_material: Handle<StandardMaterial>,
    horizontal_digit_mesh: Handle<Mesh>,
    vertical_digit_mesh: Handle<Mesh>,
    digit_material: Handle<StandardMaterial>,
    background_translation: Vec3,
    text_translation: Vec3,
    is_visible: bool,
) {
    spawn_card_point_background(
        parent,
        Name::new("Card PowerPointView Background"),
        PowerPointView::new(model),
        background_mesh,
        background_material,
        background_translation,
        is_visible,
    );
    spawn_card_point_text(
        parent,
        Name::new("Card PowerPointView Text"),
        model.display_text(),
        horizontal_digit_mesh,
        vertical_digit_mesh,
        digit_material,
        text_translation,
        is_visible,
    );
}

fn spawn_card_point_background<M: Component>(
    parent: &mut ChildSpawnerCommands,
    name: Name,
    marker: M,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    translation: Vec3,
    is_visible: bool,
) {
    parent
        .spawn((
            name,
            marker,
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(translation),
            RenderLayers::layer(CARD_RENDER_LAYER),
            NoCpuCulling,
            CardFaceLayer::new(CardFace::Front),
            if is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
        ))
        .observe(card_click_navigation);
}

fn spawn_card_point_text(
    parent: &mut ChildSpawnerCommands,
    name: Name,
    text: String,
    horizontal_digit_mesh: Handle<Mesh>,
    vertical_digit_mesh: Handle<Mesh>,
    digit_material: Handle<StandardMaterial>,
    translation: Vec3,
    is_visible: bool,
) {
    let characters: Vec<char> = text.chars().collect();
    let total_width = (characters.len() as f32 * CARD_POINT_DIGIT_WIDTH)
        + (characters.len().saturating_sub(1) as f32 * CARD_POINT_DIGIT_GAP);
    let start_x = -total_width * 0.5 + (CARD_POINT_DIGIT_WIDTH * 0.5);

    for (index, character) in characters.into_iter().enumerate() {
        let character_translation = translation
            + Vec3::new(
                start_x + (index as f32 * (CARD_POINT_DIGIT_WIDTH + CARD_POINT_DIGIT_GAP)),
                0.0,
                0.0,
            );
        spawn_card_point_glyph(
            parent,
            name.clone(),
            character,
            horizontal_digit_mesh.clone(),
            vertical_digit_mesh.clone(),
            digit_material.clone(),
            character_translation,
            is_visible,
        );
    }
}

fn spawn_card_point_glyph(
    parent: &mut ChildSpawnerCommands,
    name: Name,
    character: char,
    horizontal_digit_mesh: Handle<Mesh>,
    vertical_digit_mesh: Handle<Mesh>,
    digit_material: Handle<StandardMaterial>,
    translation: Vec3,
    is_visible: bool,
) {
    for segment in point_glyph_segments(character) {
        let (mesh, offset) = match segment {
            PointGlyphSegment::Top => (
                horizontal_digit_mesh.clone(),
                Vec2::new(
                    0.0,
                    (CARD_POINT_DIGIT_HEIGHT - CARD_POINT_DIGIT_STROKE) * 0.5,
                ),
            ),
            PointGlyphSegment::Middle => (horizontal_digit_mesh.clone(), Vec2::ZERO),
            PointGlyphSegment::Bottom => (
                horizontal_digit_mesh.clone(),
                Vec2::new(
                    0.0,
                    -(CARD_POINT_DIGIT_HEIGHT - CARD_POINT_DIGIT_STROKE) * 0.5,
                ),
            ),
            PointGlyphSegment::UpperLeft => (
                vertical_digit_mesh.clone(),
                Vec2::new(
                    -(CARD_POINT_DIGIT_WIDTH - CARD_POINT_DIGIT_STROKE) * 0.5,
                    CARD_POINT_DIGIT_HEIGHT * 0.25,
                ),
            ),
            PointGlyphSegment::UpperRight => (
                vertical_digit_mesh.clone(),
                Vec2::new(
                    (CARD_POINT_DIGIT_WIDTH - CARD_POINT_DIGIT_STROKE) * 0.5,
                    CARD_POINT_DIGIT_HEIGHT * 0.25,
                ),
            ),
            PointGlyphSegment::LowerLeft => (
                vertical_digit_mesh.clone(),
                Vec2::new(
                    -(CARD_POINT_DIGIT_WIDTH - CARD_POINT_DIGIT_STROKE) * 0.5,
                    -CARD_POINT_DIGIT_HEIGHT * 0.25,
                ),
            ),
            PointGlyphSegment::LowerRight => (
                vertical_digit_mesh.clone(),
                Vec2::new(
                    (CARD_POINT_DIGIT_WIDTH - CARD_POINT_DIGIT_STROKE) * 0.5,
                    -CARD_POINT_DIGIT_HEIGHT * 0.25,
                ),
            ),
        };

        parent
            .spawn((
                Name::new(format!("{name} Glyph")),
                Mesh3d(mesh),
                MeshMaterial3d(digit_material.clone()),
                Transform::from_translation(translation + offset.extend(0.0)),
                RenderLayers::layer(CARD_RENDER_LAYER),
                NoCpuCulling,
                CardFaceLayer::new(CardFace::Front),
                if is_visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
            ))
            .observe(card_click_navigation);
    }
}

#[derive(Clone, Copy)]
enum PointGlyphSegment {
    Top,
    UpperLeft,
    UpperRight,
    Middle,
    LowerLeft,
    LowerRight,
    Bottom,
}

fn point_glyph_segments(character: char) -> &'static [PointGlyphSegment] {
    match character {
        '0' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperLeft,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::LowerLeft,
            PointGlyphSegment::LowerRight,
            PointGlyphSegment::Bottom,
        ],
        '1' => &[PointGlyphSegment::UpperRight, PointGlyphSegment::LowerRight],
        '2' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerLeft,
            PointGlyphSegment::Bottom,
        ],
        '3' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerRight,
            PointGlyphSegment::Bottom,
        ],
        '4' => &[
            PointGlyphSegment::UpperLeft,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerRight,
        ],
        '5' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperLeft,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerRight,
            PointGlyphSegment::Bottom,
        ],
        '6' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperLeft,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerLeft,
            PointGlyphSegment::LowerRight,
            PointGlyphSegment::Bottom,
        ],
        '7' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::LowerRight,
        ],
        '8' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperLeft,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerLeft,
            PointGlyphSegment::LowerRight,
            PointGlyphSegment::Bottom,
        ],
        '9' => &[
            PointGlyphSegment::Top,
            PointGlyphSegment::UpperLeft,
            PointGlyphSegment::UpperRight,
            PointGlyphSegment::Middle,
            PointGlyphSegment::LowerRight,
            PointGlyphSegment::Bottom,
        ],
        '-' => &[PointGlyphSegment::Middle],
        _ => &[],
    }
}

#[cfg(test)]
fn point_glyph_segment_count(text: &str) -> usize {
    text.chars()
        .map(|character| point_glyph_segments(character).len())
        .sum()
}

fn spawn_local_player_hand(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Name::new("Local Player Hand"),
        LocalPlayerHand,
        GameViewEntity,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(GAME_SCENE_HAND_LEFT_PERCENT),
            bottom: Val::Percent(GAME_SCENE_HAND_BOTTOM_PERCENT),
            width: Val::Percent(GAME_SCENE_HAND_WIDTH_PERCENT),
            height: Val::Percent(GAME_SCENE_HAND_HEIGHT_PERCENT),
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

fn spawn_game_view_hand_cards(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    card_model_registry: &CardModelRegistry,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mut masked_background_materials: Option<&mut Assets<CardBackgroundMaskMaterial>>,
) {
    let cards: Vec<CardModel> = card_model_registry.card_models().cloned().collect();
    let hitboxes = game_view_card_hitboxes();
    let card_world_scale = game_view_world_height_for_game_view_height(
        GAME_SCENE_HAND_CARD_HEIGHT,
        GAME_SCENE_HAND_CARD_WORLD_Z,
    ) / card_defaults.height;

    for (index, card_model) in cards.into_iter().enumerate() {
        let (card_min, card_max) = hitboxes[index];
        let card_position = game_view_world_position_from_game_view(
            (card_min + card_max) * 0.5,
            GAME_SCENE_HAND_CARD_WORLD_Z,
        );
        let card = spawn_card_structure_for_type(
            commands,
            asset_server,
            card_defaults,
            card_model,
            meshes,
            materials,
            masked_background_materials.as_deref_mut(),
            CardFace::Front,
            Transform {
                translation: card_position,
                scale: Vec3::splat(card_world_scale),
                ..Default::default()
            },
        );
        commands
            .entity(card)
            .insert((GameViewEntity, LocalPlayerHandCardPreview))
            .observe(card_click_navigation);
    }
}

fn spawn_turn_ui(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Name::new("TurnUI"),
            TurnUi,
            GameViewEntity,
            Button,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Percent(2.0),
                bottom: Val::Percent(3.0),
                width: Val::Px(220.0),
                height: Val::Px(88.0),
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
            spawn_location_text(parent, "End Turn", 24.0);
            parent.spawn((
                Text::new(format!("{GAME_ROUND_CURRENT}/{GAME_ROUND_TOTAL}")),
                TextFont {
                    font_size: 22.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_card_browser_light(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("CardBrowserView Key Light"),
            CardBrowserViewEntity,
            DirectionalLight {
                illuminance: 1500.0,
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, -0.35, 0.0)),
        ))
        .id()
}

/// HUMAN: Spawns the card browser sub-screen view.
/// AI: CardBrowserView presents a CardView built from the active CardModel.
pub fn setup_card_browser_view(
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
    spawn_card_browser_view_contents(
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

fn spawn_card_browser_view_contents(
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
            Name::new("CardBrowserView"),
            CardBrowserViewRoot,
            CardBrowserViewEntity,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id();
    let camera = spawn_primary_camera(commands, camera_defaults);
    let ui_camera = spawn_card_browser_ui_camera(commands);
    let light = spawn_card_browser_light(commands);
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
        initial_rotation,
    );
    // Keep 3D content out of the UI node hierarchy so resize-driven UI layout
    // transforms cannot move or scale the card browser presentation.
    commands.entity(scene_root).add_child(camera);
    commands.entity(scene_root).add_child(ui_camera);
    commands.entity(scene_root).add_child(light);
    commands.entity(scene_root).add_child(card);
    commands
        .entity(card)
        .insert(CardBrowserViewEntity)
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
        Quat::IDENTITY,
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
    initial_rotation: Quat,
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
        Transform {
            translation: Vec3::ZERO,
            rotation: initial_rotation,
            scale: Vec3::splat(card_browser_centered_card_scale(card_defaults)),
        },
    )
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
    let horizontal_digit_mesh = meshes.add(Rectangle::new(
        CARD_POINT_DIGIT_WIDTH,
        CARD_POINT_DIGIT_STROKE,
    ));
    let vertical_digit_mesh = meshes.add(Rectangle::new(
        CARD_POINT_DIGIT_STROKE,
        (CARD_POINT_DIGIT_HEIGHT - CARD_POINT_DIGIT_STROKE) * 0.5,
    ));
    let cost_point_background_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.06, 0.22, 0.78),
        alpha_mode: AlphaMode::Opaque,
        depth_bias: POINT_DEPTH_BIAS,
        unlit: true,
        ..Default::default()
    });
    let power_point_background_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.62, 0.05, 0.08),
        alpha_mode: AlphaMode::Opaque,
        depth_bias: POINT_DEPTH_BIAS,
        unlit: true,
        ..Default::default()
    });
    let point_digit_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Opaque,
        depth_bias: POINT_DEPTH_BIAS + 8.0,
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
        spawn_card_cost_point_view(
            parent,
            card_model.cost,
            cost_point_background_mesh,
            cost_point_background_material,
            horizontal_digit_mesh.clone(),
            vertical_digit_mesh.clone(),
            point_digit_material.clone(),
            Vec3::new(-cost_point_x, point_y, point_background_z),
            Vec3::new(-cost_point_x, point_y, point_text_z),
            visible_face == CardFace::Front,
        );
        spawn_card_power_point_view(
            parent,
            card_model.base_power,
            power_point_background_mesh,
            power_point_background_material,
            horizontal_digit_mesh,
            vertical_digit_mesh,
            point_digit_material,
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
    mut card_query: Query<&mut Transform, (With<CardView>, With<CardBrowserViewEntity>)>,
    mut game_card_query: Query<
        &mut Transform,
        (
            With<LocalPlayerHandCardPreview>,
            With<GameViewEntity>,
            Without<CardBrowserViewEntity>,
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
            Without<CardBrowserViewEntity>,
        ),
    >,
    layer_query: Query<
        (&Name, &Visibility, &GlobalTransform),
        (
            With<CardParallaxLayer>,
            With<CardFaceLayer>,
            Without<CardBrowserViewEntity>,
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
/// AI: Uses domain_schedule_system naming; GameView toggles world models, CardBrowserView toggles card UI depth.
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
        ActiveView::CardBrowserView => {
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
pub fn restart_app_scene(
    keys: Res<ButtonInput<KeyCode>>,
    active_card_model: Res<ActiveCardModel>,
    mut flip_state: ResMut<CardFlipState>,
    mut ticks: ResMut<GameTicks>,
    mut scene: ViewChangeParams,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    scene.reload_app_scene_and_active_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
    *flip_state = CardFlipState::default();
    *scene.card_state = CardInspectionState::default();
    ticks.0 = 0;
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

    debug_drawing_model.request_reference_layout();
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
    standalone_card_browser_view_entities: Query<
        'w,
        's,
        Entity,
        (
            With<CardBrowserViewEntity>,
            Without<CardBrowserViewRoot>,
            Without<ChildOf>,
        ),
    >,
    card_browser_view_roots: Query<'w, 's, Entity, With<CardBrowserViewRoot>>,
    primary_window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    card_browser_camera_query: Query<
        'w,
        's,
        (&'static Camera, &'static GlobalTransform),
        (
            With<PrimaryViewCamera>,
            With<CardBrowserViewEntity>,
            With<Camera3d>,
        ),
    >,
    card_browser_card_query:
        Query<'w, 's, &'static GlobalTransform, (With<CardView>, With<CardBrowserViewEntity>)>,
    mouse_buttons: Res<'w, ButtonInput<MouseButton>>,
    touches: Res<'w, Touches>,
    asset_server: Res<'w, AssetServer>,
    camera_defaults: Res<'w, PrimaryCameraDefaults>,
    card_defaults: Res<'w, CardInspectionDefaults>,
    card_model_registry: Res<'w, CardModelRegistry>,
    world_model_registry: Res<'w, WorldModelRegistry>,
    active_world_model: ResMut<'w, ActiveWorldModel>,
    location_model_registry: Res<'w, LocationModelRegistry>,
    active_locations: ResMut<'w, ActiveLocations>,
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
        for entity in self.standalone_card_browser_view_entities.iter() {
            self.commands.entity(entity).despawn();
        }
    }

    fn despawn_card_browser_view(&mut self) {
        for entity in self.card_browser_view_roots.iter() {
            self.commands.entity(entity).despawn();
        }
    }

    fn spawn_game_view(&mut self, active_card_model: &ActiveCardModel) {
        spawn_game_view_contents(
            &mut self.commands,
            self.app_scene_query.single().ok(),
            self.hud.as_ref().map(|hud| hud.0),
            &self.asset_server,
            &self.camera_defaults,
            &self.card_defaults,
            &self.card_model_registry,
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

    fn spawn_card_browser_view(
        &mut self,
        active_card_model: &ActiveCardModel,
        visible_face: CardFace,
        initial_rotation: Quat,
    ) {
        spawn_card_browser_view_contents(
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
                spawn_game_view_contents(
                    &mut self.commands,
                    self.app_scene_query.single().ok(),
                    self.hud.as_ref().map(|hud| hud.0),
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
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
            ActiveView::CardBrowserView => {
                self.despawn_card_browser_view();
                spawn_card_browser_view_contents(
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
        self.despawn_card_browser_view();
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
                spawn_game_view_contents(
                    &mut self.commands,
                    Some(app_scene),
                    self.hud.as_ref().map(|hud| hud.0),
                    &self.asset_server,
                    &self.camera_defaults,
                    &self.card_defaults,
                    &self.card_model_registry,
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
            ActiveView::CardBrowserView => {
                spawn_card_browser_view_contents(
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

/// HUMAN: Handles pointer navigation between GameView and CardBrowserView.
/// AI: Uses domain_schedule_system naming; this is the only input path that swaps active views.
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
            let window_size = Vec2::new(
                primary_window.resolution.width(),
                primary_window.resolution.height(),
            );
            let Some(card_index) = game_view_card_index_at(pointer_position, window_size) else {
                return;
            };

            active_card_model.index = card_index;
            *flip_state = CardFlipState::default();
            params.despawn_game_view();
            params.spawn_card_browser_view(&active_card_model, CardFace::Front, Quat::IDENTITY);
            *params.active_view = ActiveView::CardBrowserView;
        }
        ActiveView::CardBrowserView => {
            if !is_card_browser_card_hit(
                pointer_position,
                params.card_browser_camera_query.single().ok(),
                params.card_browser_card_query.single().ok(),
                &params.card_defaults,
            ) {
                return;
            }

            params.despawn_card_browser_view();
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
        ActiveView::CardBrowserView => {
            params.despawn_card_browser_view();
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

fn game_view_card_index_at(pointer_position: Vec2, window_size: Vec2) -> Option<usize> {
    if window_size.x <= 0.0 || window_size.y <= 0.0 {
        return None;
    }

    let Some(pointer_position) = window_pointer_to_game_view(pointer_position, window_size) else {
        return None;
    };
    game_view_card_hitboxes().iter().position(|(min, max)| {
        pointer_position.x >= min.x
            && pointer_position.x <= max.x
            && pointer_position.y >= min.y
            && pointer_position.y <= max.y
    })
}

fn is_card_browser_card_hit(
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

fn game_view_card_hitboxes() -> Vec<(Vec2, Vec2)> {
    let hand_min = Vec2::new(
        GAME_VIEW_WIDTH * (GAME_SCENE_HAND_LEFT_PERCENT * 0.01),
        GAME_VIEW_HEIGHT
            * ((100.0 - GAME_SCENE_HAND_BOTTOM_PERCENT - GAME_SCENE_HAND_HEIGHT_PERCENT) * 0.01),
    );
    let hand_size = Vec2::new(
        GAME_VIEW_WIDTH * (GAME_SCENE_HAND_WIDTH_PERCENT * 0.01),
        GAME_VIEW_HEIGHT * (GAME_SCENE_HAND_HEIGHT_PERCENT * 0.01),
    );
    let card_size = Vec2::new(GAME_SCENE_HAND_CARD_WIDTH, GAME_SCENE_HAND_CARD_HEIGHT);
    let row_width = (GAME_SCENE_HAND_CARD_WIDTH * 4.0) + (GAME_SCENE_HAND_CARD_GAP * 3.0);
    let row_height = GAME_SCENE_HAND_CARD_HEIGHT;
    let row_min = hand_min + ((hand_size - Vec2::new(row_width, row_height)) * 0.5).max(Vec2::ZERO);

    (0..4)
        .map(|index| {
            let card_min = row_min
                + Vec2::new(
                    index as f32 * (GAME_SCENE_HAND_CARD_WIDTH + GAME_SCENE_HAND_CARD_GAP),
                    0.0,
                );
            (card_min, card_min + card_size.min(hand_size))
        })
        .collect()
}

pub fn update_end_turn_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EndTurnButton>),
    >,
) {
    for (interaction, mut background, mut border) in &mut button_query {
        let (background_color, border_color) = match *interaction {
            Interaction::Pressed => {
                info!("End Turn clicked");
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
        background.0 = background_color;
        *border = BorderColor::all(border_color);
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
        ActiveView::CardBrowserView => "CardBrowserView",
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
    mut placement_state: Option<ResMut<WindowPlacementState>>,
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
    mut persistent_placement: Option<ResMut<Persistent<WindowPlacementStore>>>,
) {
    let mut changed = false;
    let mut placement_to_save = None;

    if keys.just_pressed(KeyCode::KeyF) {
        hud_state.is_fullscreen = !hud_state.is_fullscreen;
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
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
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

pub fn restore_window_placement_to_current_monitors(
    mut placement_state: ResMut<WindowPlacementState>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
) {
    if placement_state.restored {
        return;
    }
    if monitor_query.iter().next().is_none() {
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
) {
    let Some(ref mut window_moved_events) = window_moved_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };
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
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };
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

    let current_window_placement = if is_windowed(window) {
        current_windowed_placement(window, placement_state.current.as_ref(), &monitor_query)
    } else {
        placement_state.current.clone()
    };

    let placement_with_current_size = placement_state.current.as_ref().map(|placement| {
        placement_with_current_window_size(
            placement,
            logical_window_size(window),
            window.resolution.physical_size(),
            &monitor_query,
        )
    });
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
                ui.heading("Card Browser");
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
    active_view == ActiveView::CardBrowserView
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
mod tests {
    use super::*;

    fn mesh_bounds(attribute: &VertexAttributeValues) -> (f32, f32) {
        let VertexAttributeValues::Float32x3(positions) = attribute else {
            panic!("expected Float32x3 mesh positions");
        };

        let (min_x, max_x) = positions
            .iter()
            .map(|position| position[0])
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| {
                (min.min(x), max.max(x))
            });
        let (min_y, max_y) = positions
            .iter()
            .map(|position| position[1])
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), y| {
                (min.min(y), max.max(y))
            });

        (max_x - min_x, max_y - min_y)
    }

    fn mesh_uv_bounds(attribute: &VertexAttributeValues) -> (f32, f32) {
        let VertexAttributeValues::Float32x2(uvs) = attribute else {
            panic!("expected Float32x2 mesh uvs");
        };

        let (min_u, max_u) = uvs
            .iter()
            .map(|uv| uv[0])
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), u| {
                (min.min(u), max.max(u))
            });
        let (min_v, max_v) = uvs
            .iter()
            .map(|uv| uv[1])
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), v| {
                (min.min(v), max.max(v))
            });

        (max_u - min_u, max_v - min_v)
    }

    fn assert_close(left: f32, right: f32) {
        assert!((left - right).abs() < 0.000_001, "{left} != {right}");
    }

    fn active_child_scene_root_count(app: &mut App) -> usize {
        let mut scene_query = app
            .world_mut()
            .query_filtered::<Entity, Or<(With<GameViewRoot>, With<CardBrowserViewRoot>)>>();
        scene_query.iter(app.world()).count()
    }

    fn spawn_test_primary_window(app: &mut App) -> Entity {
        app.world_mut()
            .spawn((
                Window {
                    resolution: WindowResolution::new(
                        DEFAULT_WINDOW_WIDTH as u32,
                        DEFAULT_WINDOW_HEIGHT as u32,
                    ),
                    ..Default::default()
                },
                PrimaryWindow,
            ))
            .id()
    }

    fn test_monitor(name: &str, position: IVec2, size: UVec2) -> Monitor {
        Monitor {
            name: Some(name.to_string()),
            physical_height: size.y,
            physical_width: size.x,
            physical_position: position,
            refresh_rate_millihertz: Some(60_000),
            scale_factor: 1.0,
            video_modes: Vec::new(),
        }
    }

    #[test]
    fn debug_window_text_styles_use_matching_font_face_and_size() {
        let context = egui::Context::default();

        use_matching_debug_window_text_style(&context);

        let style = context.style();
        let expected_font_id = egui::FontId::proportional(DEBUG_WINDOW_FONT_SIZE);

        assert!(
            style
                .text_styles
                .values()
                .all(|font_id| font_id.family == expected_font_id.family
                    && font_id.size == expected_font_id.size)
        );
    }

    #[test]
    fn debug_hud_text_spans_use_matching_font_size() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut hud_query = app
            .world_mut()
            .query_filtered::<(Entity, &TextFont), With<DebugHudText>>();
        let (hud_entity, hud_font) = hud_query.single(app.world()).unwrap();
        assert_eq!(hud_font.font_size, DEBUG_HUD_FONT_SIZE);

        let children = app.world().get::<Children>(hud_entity).unwrap();
        assert!(!children.is_empty());

        for child in children.iter() {
            let child_font = app.world().get::<TextFont>(child).unwrap();
            assert_eq!(child_font.font_size, DEBUG_HUD_FONT_SIZE);
        }
    }

    #[test]
    fn debug_hud_title_shows_active_view_without_card_model_status() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ActiveView>()
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut hud_query = app
            .world_mut()
            .query_filtered::<&Text, With<DebugHudText>>();
        let hud_text = hud_query.single(app.world()).unwrap();

        assert!(hud_text.0.starts_with("Scene: GameView\nFrame: 0"));
        assert!(!hud_text.0.contains("CardModel:"));
    }

    #[test]
    fn inspector_defaults_are_compact_and_below_hud() {
        let inspector = InspectorState::default();

        assert_eq!(inspector.x, 24.0);
        assert_eq!(inspector.y, 132.0);
        assert_eq!(inspector.width, 338.0);
        assert_eq!(inspector.height, 310.0);
    }

    #[test]
    fn app_scene_owns_debug_hud_without_card_browser_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PrimaryCameraDefaults>()
            .add_systems(Startup, setup_app_scene);

        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimaryViewCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 0);

        let mut light_query = app
            .world_mut()
            .query_filtered::<Entity, With<DirectionalLight>>();
        assert_eq!(light_query.iter(app.world()).count(), 0);

        let mut hud_query = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudText>>();
        let hud_entities: Vec<Entity> = hud_query.iter(app.world()).collect();
        assert_eq!(hud_entities.len(), 1);

        let mut app_scene_query = app
            .world_mut()
            .query_filtered::<Entity, With<AppSceneRoot>>();
        let app_scene_entity = app_scene_query.single(app.world()).unwrap();
        let app_scene_node = app.world().get::<Node>(app_scene_entity).unwrap();
        assert_eq!(app_scene_node.width, Val::Percent(100.0));
        assert_eq!(app_scene_node.height, Val::Percent(100.0));
        let app_scene_children = app.world().get::<Children>(app_scene_entity).unwrap();
        assert!(app_scene_children.contains(&hud_entities[0]));

        let mut card_query = app.world_mut().query_filtered::<Entity, With<CardView>>();
        assert_eq!(card_query.iter(app.world()).count(), 0);
    }

    #[test]
    fn card_browser_view_owns_camera_light_and_card() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<CardBackgroundMaskMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<&Transform, (With<PrimaryViewCamera>, With<CardBrowserViewEntity>)>();
        let camera_transform = camera_query.single(app.world()).unwrap();
        assert_eq!(
            camera_transform.translation.z,
            CARD_BROWSER_CAMERA_DISTANCE_FROM_ORIGIN
        );

        let mut light_query = app
            .world_mut()
            .query_filtered::<Entity, (With<DirectionalLight>, With<CardBrowserViewEntity>)>();
        assert_eq!(light_query.iter(app.world()).count(), 1);

        let mut ui_camera_query = app
            .world_mut()
            .query_filtered::<(&Camera, Option<&PrimaryEguiContext>), (
                With<Camera2d>,
                With<CardBrowserViewEntity>,
            )>();
        let (ui_camera, egui_context) = ui_camera_query.single(app.world()).unwrap();
        assert_eq!(ui_camera.order, 1);
        assert!(matches!(ui_camera.clear_color, ClearColorConfig::None));
        assert!(egui_context.is_some());

        let mut card_query = app
            .world_mut()
            .query_filtered::<&Transform, (With<CardView>, With<CardBrowserViewEntity>)>();
        let card_transform = card_query.single(app.world()).unwrap();
        let expected_card_scale =
            card_browser_centered_card_scale(app.world().resource::<CardInspectionDefaults>());
        assert_eq!(card_transform.translation, Vec3::ZERO);
        assert_close(card_transform.scale.x, expected_card_scale);
        assert_close(card_transform.scale.y, expected_card_scale);
        assert_close(card_transform.scale.z, expected_card_scale);
    }

    #[test]
    fn card_structure_spawns_visible_cost_and_power_point_views() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let active_card = {
            let registry = app.world().resource::<CardModelRegistry>();
            let active_card_model = app.world().resource::<ActiveCardModel>();
            registry
                .active_card_model(active_card_model)
                .expect("active card model should exist")
                .clone()
        };

        let mut cost_query = app
            .world_mut()
            .query::<(&Name, &CostPointView, &Visibility)>();
        let cost_views: Vec<(String, CostPointModel, Visibility)> = cost_query
            .iter(app.world())
            .map(|(name, view, visibility)| (name.to_string(), view.model, *visibility))
            .collect();
        assert_eq!(
            cost_views,
            vec![(
                "Card CostPointView Background".to_string(),
                active_card.cost,
                Visibility::Visible,
            )]
        );

        let mut power_query = app
            .world_mut()
            .query_filtered::<(&Name, &PowerPointView, &Visibility), Without<GameLocation>>();
        let power_views: Vec<(String, PowerPointModel, Visibility)> = power_query
            .iter(app.world())
            .map(|(name, view, visibility)| (name.to_string(), view.model, *visibility))
            .collect();
        assert_eq!(
            power_views,
            vec![(
                "Card PowerPointView Background".to_string(),
                active_card.base_power,
                Visibility::Visible,
            )]
        );

        let mut glyph_query = app.world_mut().query::<(&Name, &Visibility)>();
        let cost_glyph_count = glyph_query
            .iter(app.world())
            .filter(|(name, visibility)| {
                name.as_str() == "Card CostPointView Text Glyph"
                    && **visibility == Visibility::Visible
            })
            .count();
        let power_glyph_count = glyph_query
            .iter(app.world())
            .filter(|(name, visibility)| {
                name.as_str() == "Card PowerPointView Text Glyph"
                    && **visibility == Visibility::Visible
            })
            .count();

        assert_eq!(
            cost_glyph_count,
            point_glyph_segment_count(&active_card.cost.display_text())
        );
        assert_eq!(
            power_glyph_count,
            point_glyph_segment_count(&active_card.base_power.display_text())
        );
    }

    #[test]
    fn card_browser_view_root_does_not_inherit_ui_layout_transform() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<CardBackgroundMaskMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_app_scene)
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut root_query = app
            .world_mut()
            .query_filtered::<(Option<&ChildOf>, &Transform), With<CardBrowserViewRoot>>();
        let (parent, transform) = root_query.single(app.world()).unwrap();
        assert!(parent.is_none());
        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(transform.scale, Vec3::ONE);
    }

    #[test]
    fn card_browser_camera_viewport_matches_centered_safe_area() {
        let wide_viewport = game_view_safe_area_viewport(UVec2::new(1600, 800)).unwrap();
        assert_eq!(wide_viewport.physical_position, UVec2::new(160, 0));
        assert_eq!(wide_viewport.physical_size, UVec2::new(1280, 800));

        let tall_viewport = game_view_safe_area_viewport(UVec2::new(1280, 1000)).unwrap();
        assert_eq!(tall_viewport.physical_position, UVec2::new(0, 100));
        assert_eq!(tall_viewport.physical_size, UVec2::new(1280, 800));

        let default_viewport =
            game_view_safe_area_viewport(UVec2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
                .unwrap();
        assert_eq!(default_viewport.physical_position, UVec2::new(0, 64));
        assert_eq!(default_viewport.physical_size, UVec2::new(1024, 640));
    }

    #[test]
    fn game_view_3d_cameras_use_centered_safe_area_viewport() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .add_systems(Startup, setup_game_view)
            .add_systems(Update, constrain_game_view_3d_cameras_to_safe_area);
        app.world_mut().spawn((
            Window {
                resolution: WindowResolution::new(1280, 1536),
                ..Default::default()
            },
            PrimaryWindow,
        ));

        app.update();
        app.update();

        let expected_viewport = game_view_safe_area_viewport(UVec2::new(1280, 1536)).unwrap();
        let mut camera_query = app
            .world_mut()
            .query_filtered::<&Camera, (With<GameViewEntity>, With<Camera3d>)>();
        let cameras: Vec<&Camera> = camera_query.iter(app.world()).collect();
        assert_eq!(cameras.len(), 2);
        for camera in cameras {
            let viewport = camera.viewport.as_ref().unwrap();
            assert_eq!(
                viewport.physical_position,
                expected_viewport.physical_position
            );
            assert_eq!(viewport.physical_size, expected_viewport.physical_size);
            assert_eq!(viewport.depth, expected_viewport.depth);
        }
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn game_view_3d_cameras_use_default_viewport_in_native_fullscreen() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .add_systems(Startup, setup_game_view)
            .add_systems(Update, constrain_game_view_3d_cameras_to_safe_area);
        app.world_mut().spawn((
            Window {
                resolution: WindowResolution::new(2560, 1600),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..Default::default()
            },
            PrimaryWindow,
        ));

        app.update();
        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<&Camera, (With<GameViewEntity>, With<Camera3d>)>();
        let cameras: Vec<&Camera> = camera_query.iter(app.world()).collect();
        assert_eq!(cameras.len(), 2);
        for camera in cameras {
            assert!(camera.viewport.is_none());
        }
    }

    #[test]
    fn game_view_owns_camera_world_background_and_three_locations() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<CardBackgroundMaskMaterial>>()
            .init_asset::<Image>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .add_systems(Startup, setup_game_view);

        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, (With<PrimaryViewCamera>, With<GameViewEntity>)>();
        assert_eq!(camera_query.iter(app.world()).count(), 2);

        let mut light_query = app
            .world_mut()
            .query_filtered::<Entity, (With<DirectionalLight>, With<GameViewEntity>)>();
        assert_eq!(light_query.iter(app.world()).count(), 0);

        let mut background_query = app.world_mut().query_filtered::<(
            &Name,
            &Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
        ), With<WorldBackground>>();
        let (background_name, background_mesh, _background_material) =
            background_query.single(app.world()).unwrap();

        assert_eq!(background_name.as_str(), "Bamboo Forest World Background");
        let background_mesh = app
            .world()
            .resource::<Assets<Mesh>>()
            .get(&background_mesh.0)
            .unwrap();
        let (background_width, background_height) =
            mesh_bounds(background_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        let expected_background_size = game_view_world_background_size();
        assert_close(background_width, expected_background_size.x);
        assert_close(background_height, expected_background_size.y);

        let mut location_query = app.world_mut().query::<&GameLocation>();
        let mut locations: Vec<(usize, LocationRevealState)> = location_query
            .iter(app.world())
            .map(|location| (location.index, location.reveal_state))
            .collect();
        locations.sort_by_key(|(index, _)| *index);

        assert_eq!(
            locations,
            vec![
                (0, LocationRevealState::Revealed),
                (1, LocationRevealState::Revealed),
                (2, LocationRevealState::Revealed)
            ]
        );
        let mut point_view_query = app.world_mut().query::<(&Name, &PowerPointView)>();
        let point_view_values: Vec<i32> = point_view_query
            .iter(app.world())
            .filter(|(name, _)| name.as_str() == "PowerPointView")
            .map(|(_, point_view)| point_view.model.value)
            .collect();
        assert_eq!(point_view_values, vec![0, 0, 0, 0, 0, 0]);

        let mut hand_query = app
            .world_mut()
            .query_filtered::<Entity, With<LocalPlayerHand>>();
        assert_eq!(hand_query.iter(app.world()).count(), 1);

        let mut turn_ui_query = app.world_mut().query_filtered::<Entity, With<TurnUi>>();
        assert_eq!(turn_ui_query.iter(app.world()).count(), 1);

        let mut end_turn_button_query =
            app.world_mut()
                .query_filtered::<Entity, (With<TurnUi>, With<EndTurnButton>, With<Button>)>();
        assert_eq!(end_turn_button_query.iter(app.world()).count(), 1);

        let mut preview_query = app.world_mut().query_filtered::<&Transform, (
            With<LocalPlayerHandCardPreview>,
            With<CardView>,
            With<GameViewEntity>,
            Without<CardBrowserViewEntity>,
        )>();
        let mut preview_transforms: Vec<Transform> =
            preview_query.iter(app.world()).copied().collect();
        preview_transforms.sort_by(|left, right| {
            left.translation
                .x
                .partial_cmp(&right.translation.x)
                .unwrap()
        });
        assert_eq!(preview_transforms.len(), 4);

        let expected_scale = game_view_world_height_for_game_view_height(
            GAME_SCENE_HAND_CARD_HEIGHT,
            GAME_SCENE_HAND_CARD_WORLD_Z,
        ) / app.world().resource::<CardInspectionDefaults>().height;
        for (index, transform) in preview_transforms.iter().enumerate() {
            let (card_min, card_max) = game_view_card_hitboxes()[index];
            let expected_translation = game_view_world_position_from_game_view(
                (card_min + card_max) * 0.5,
                GAME_SCENE_HAND_CARD_WORLD_Z,
            );
            assert_close(transform.translation.x, expected_translation.x);
            assert_close(transform.translation.y, expected_translation.y);
            assert_close(transform.translation.z, expected_translation.z);
            assert_close(transform.scale.x, expected_scale);
            assert_close(transform.scale.y, expected_scale);
            assert_close(transform.scale.z, expected_scale);
        }

        let mut preview_layer_query = app.world_mut().query_filtered::<Entity, (
            With<CardParallaxLayer>,
            With<CardFaceLayer>,
            Without<CardBrowserViewEntity>,
        )>();
        assert_eq!(preview_layer_query.iter(app.world()).count(), 20);

        let mut preview_camera_query = app
            .world_mut()
            .query_filtered::<&Camera, (With<Camera3d>, With<GameViewEntity>)>();
        let mut preview_camera_orders: Vec<isize> = preview_camera_query
            .iter(app.world())
            .map(|camera| camera.order)
            .collect();
        preview_camera_orders.sort();
        assert_eq!(preview_camera_orders, vec![0, 2]);

        let mut ui_camera_query = app
            .world_mut()
            .query_filtered::<&Camera, (With<Camera2d>, With<GameViewEntity>)>();
        let ui_camera = ui_camera_query.single(app.world()).unwrap();
        assert_eq!(ui_camera.order, 1);
        assert!(matches!(ui_camera.clear_color, ClearColorConfig::None));
    }

    #[test]
    fn game_view_hand_preview_transform_chain_has_global_transforms() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .add_systems(Startup, setup_app_scene)
            .add_systems(Startup, setup_game_view);

        app.update();

        let mut transform_parent_query =
            app.world_mut()
                .query_filtered::<(&Name, &Transform, &GlobalTransform), Or<(
                    With<GameViewRoot>,
                    With<LocalPlayerHand>,
                )>>();
        let transform_parent_names: Vec<&str> = transform_parent_query
            .iter(app.world())
            .map(|(name, _, _)| name.as_str())
            .collect();
        assert!(transform_parent_names.contains(&"GameView"));
        assert!(transform_parent_names.contains(&"Local Player Hand"));

        let mut game_view_ui_query =
            app.world_mut()
                .query::<(&Name, &Transform, &GlobalTransform, &GameViewEntity)>();
        assert!(
            game_view_ui_query
                .iter(app.world())
                .any(|(name, _, _, _)| name.as_str() == "GameView UI")
        );

        let mut preview_query = app.world_mut().query_filtered::<
            (&Name, &Transform, &GlobalTransform),
            With<LocalPlayerHandCardPreview>,
        >();
        assert_eq!(preview_query.iter(app.world()).count(), 4);
    }

    #[test]
    fn browser_card_rotation_system_does_not_recenter_game_view_hand_card() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .add_systems(Startup, setup_game_view)
            .add_systems(Update, smooth_card_rotation);

        app.update();
        app.update();

        let mut preview_query = app.world_mut().query_filtered::<&Transform, (
            With<LocalPlayerHandCardPreview>,
            With<GameViewEntity>,
            Without<CardBrowserViewEntity>,
        )>();
        let initial_transforms: Vec<Transform> = preview_query.iter(app.world()).copied().collect();
        assert_eq!(initial_transforms.len(), 4);

        app.update();

        let updated_transforms: Vec<Transform> = preview_query.iter(app.world()).copied().collect();
        assert_eq!(updated_transforms.len(), initial_transforms.len());
        for (initial_transform, updated_transform) in
            initial_transforms.iter().zip(updated_transforms.iter())
        {
            assert_eq!(updated_transform.translation, initial_transform.translation);
            assert_eq!(updated_transform.scale, initial_transform.scale);
        }
    }

    #[test]
    fn game_view_card_hitbox_accepts_only_lower_center_card_area() {
        let window_size = Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32);
        assert_close(
            GAME_SCENE_HAND_CARD_WIDTH / GAME_SCENE_HAND_CARD_HEIGHT,
            CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT,
        );

        let hitboxes = game_view_card_hitboxes();
        assert_eq!(hitboxes.len(), 4);
        let (card_min, card_max) = hitboxes[0];
        let card_center = (card_min + card_max) * 0.5;
        let window_card_center = game_view_pointer_to_window(card_center, window_size);

        assert!(is_game_view_card_hit(window_card_center, window_size));
        assert!(is_game_view_card_hit(
            game_view_pointer_to_window(card_min + Vec2::splat(0.5), window_size),
            window_size
        ));
        assert!(is_game_view_card_hit(
            game_view_pointer_to_window(card_max - Vec2::splat(0.5), window_size),
            window_size
        ));
        assert!(!is_game_view_card_hit(
            game_view_pointer_to_window(
                Vec2::new(GAME_VIEW_WIDTH * 0.5, GAME_VIEW_HEIGHT * 0.5),
                window_size
            ),
            window_size
        ));
        assert!(!is_game_view_card_hit(
            game_view_pointer_to_window(card_min - Vec2::splat(1.0), window_size),
            window_size
        ));

        let last_center = (hitboxes[3].0 + hitboxes[3].1) * 0.5;
        assert_eq!(
            game_view_card_index_at(
                game_view_pointer_to_window(last_center, window_size),
                window_size
            ),
            Some(3)
        );
    }

    #[test]
    fn clicking_game_card_opens_browser() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<Touches>()
            .init_resource::<ActiveView>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .add_systems(Startup, (setup_app_scene, setup_game_view).chain())
            .add_systems(Update, view_input_system);
        let window = spawn_test_primary_window(&mut app);

        app.update();
        assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameView);
        assert_eq!(active_child_scene_root_count(&mut app), 1);

        app.world_mut()
            .get_mut::<Window>(window)
            .unwrap()
            .set_cursor_position(Some(game_view_pointer_to_window(
                (game_view_card_hitboxes()[2].0 + game_view_card_hitboxes()[2].1) * 0.5,
                Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32),
            )));
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);
        app.update();

        assert_eq!(
            *app.world().resource::<ActiveView>(),
            ActiveView::CardBrowserView
        );
        assert_eq!(app.world().resource::<ActiveCardModel>().index, 2);
        assert_eq!(active_child_scene_root_count(&mut app), 1);
        let mut game_view_query = app
            .world_mut()
            .query_filtered::<Entity, With<GameViewRoot>>();
        assert_eq!(game_view_query.iter(app.world()).count(), 0);
        let mut card_scene_query = app
            .world_mut()
            .query_filtered::<Entity, With<CardBrowserViewEntity>>();
        assert!(card_scene_query.iter(app.world()).count() > 0);
        let mut game_view_entity_query = app
            .world_mut()
            .query_filtered::<Entity, With<GameViewEntity>>();
        assert_eq!(game_view_entity_query.iter(app.world()).count(), 0);
        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimaryViewCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 1);
    }

    #[test]
    fn card_ui_visibility_follows_card_browser_view() {
        assert!(!should_show_card_ui(ActiveView::GameView));
        assert!(should_show_card_ui(ActiveView::CardBrowserView));
    }

    #[test]
    fn card_ui_anchor_accounts_for_wide_window_safe_area() {
        let offset = card_ui_safe_area_anchor_offset(Vec2::new(1600.0, 800.0));

        assert_eq!(offset.x, -(160.0 + SCREEN_PADDING_LEFT));
        assert_eq!(offset.y, SCREEN_PADDING_TOP);
    }

    #[test]
    fn card_ui_anchor_accounts_for_tall_window_safe_area() {
        let offset = card_ui_safe_area_anchor_offset(Vec2::new(1280.0, 1000.0));

        assert_eq!(offset.x, -SCREEN_PADDING_LEFT);
        assert_eq!(offset.y, 100.0 + SCREEN_PADDING_TOP);
    }

    #[test]
    fn card_ui_anchor_padding_scales_with_debug_hud() {
        let offset = card_ui_safe_area_anchor_offset(Vec2::new(1024.0, 768.0));

        assert_close(offset.x, -(SCREEN_PADDING_LEFT * 0.8));
        assert_close(offset.y, 64.0 + (SCREEN_PADDING_TOP * 0.8));
    }

    #[test]
    fn end_turn_button_updates_visual_state_from_interaction() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Update, update_end_turn_button);
        let button = app
            .world_mut()
            .spawn((
                EndTurnButton,
                Interaction::Hovered,
                BackgroundColor(END_TURN_BUTTON_NORMAL_COLOR),
                BorderColor::all(END_TURN_BUTTON_NORMAL_BORDER_COLOR),
            ))
            .id();

        app.update();
        assert_eq!(
            app.world().get::<BackgroundColor>(button).unwrap().0,
            END_TURN_BUTTON_HOVER_COLOR
        );
        assert_eq!(
            *app.world().get::<BorderColor>(button).unwrap(),
            BorderColor::all(END_TURN_BUTTON_HOVER_BORDER_COLOR)
        );

        *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::Pressed;
        app.update();
        assert_eq!(
            app.world().get::<BackgroundColor>(button).unwrap().0,
            END_TURN_BUTTON_PRESSED_COLOR
        );
        assert_eq!(
            *app.world().get::<BorderColor>(button).unwrap(),
            BorderColor::all(END_TURN_BUTTON_PRESSED_BORDER_COLOR)
        );

        *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::None;
        app.update();
        assert_eq!(
            app.world().get::<BackgroundColor>(button).unwrap().0,
            END_TURN_BUTTON_NORMAL_COLOR
        );
        assert_eq!(
            *app.world().get::<BorderColor>(button).unwrap(),
            BorderColor::all(END_TURN_BUTTON_NORMAL_BORDER_COLOR)
        );
    }

    #[test]
    fn polished_layers_use_flat_artwork_with_apparent_depth_offsets() {
        let card_defaults = CardInspectionDefaults::default();
        let frame_dimensions = frame_dimensions(&card_defaults);

        assert_eq!(BACKGROUND_APPARENT_DEPTH, -1.0);
        assert_eq!(FRAME_APPARENT_DEPTH, 0.0);
        assert_eq!(SAFE_AREA_APPARENT_DEPTH, FRAME_APPARENT_DEPTH);
        assert_eq!(FOREGROUND_APPARENT_DEPTH, 1.0);
        assert_eq!(TITLE_APPARENT_DEPTH, 2.0);
        assert!(LAYER_RENDER_Z_STEP < card_defaults.thickness * 0.01);
        assert!(PARALLAX_OFFSET_RATIO > 0.0);
        assert_eq!(
            frame_dimensions.frame_thickness_x,
            card_defaults.width * FRAME_THICKNESS_RATIO
        );
        assert_eq!(
            frame_dimensions.frame_thickness_y,
            frame_dimensions.frame_thickness_x
        );
        assert_eq!(
            frame_dimensions.hole_width + (frame_dimensions.frame_thickness_x * 2.0),
            card_defaults.width
        );
        assert_eq!(
            frame_dimensions.hole_height + (frame_dimensions.frame_thickness_y * 2.0),
            card_defaults.height
        );
        assert_eq!(BACKGROUND_APERTURE_SCALE, 1.0);
    }

    #[test]
    fn card_structure_uses_one_cutout_frame_entity() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut frame_query = app
            .world_mut()
            .query_filtered::<(&Name, &CardParallaxLayer), With<CardFrameLayer>>();
        let frames: Vec<(String, CardLayerRole)> = frame_query
            .iter(app.world())
            .map(|(name, layer)| (name.to_string(), layer.role))
            .collect();

        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].0, "Card Frame Cutout");
        assert_eq!(frames[0].1, CardLayerRole::Frame);
    }

    #[test]
    fn card_structure_spawns_one_card_back_and_one_card_root() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut card_query = app.world_mut().query_filtered::<Entity, With<CardView>>();
        assert_eq!(card_query.iter(app.world()).count(), 1);

        let mut back_query = app
            .world_mut()
            .query_filtered::<(&Name, &CardFaceLayer), Without<CardParallaxLayer>>();
        let backs: Vec<String> = back_query
            .iter(app.world())
            .filter_map(|(name, face_layer)| {
                (face_layer.face == CardFace::Back).then_some(name.to_string())
            })
            .collect();

        assert_eq!(backs, vec!["Card Back CardSeries Pattern"]);
    }

    #[test]
    fn card_faces_default_to_front_visible_and_back_hidden() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut face_query = app.world_mut().query::<(&CardFaceLayer, &Visibility)>();
        let states: Vec<(CardFace, Visibility)> = face_query
            .iter(app.world())
            .map(|(face_layer, visibility)| (face_layer.face, *visibility))
            .collect();

        assert!(
            states
                .iter()
                .any(|(face, visibility)| *face == CardFace::Back
                    && *visibility == Visibility::Hidden)
        );
        assert!(states.iter().any(
            |(face, visibility)| *face == CardFace::Front && *visibility == Visibility::Visible
        ));
    }

    #[test]
    fn face_visibility_follows_flip_state_midpoint() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<CardFlipState>()
            .init_resource::<CardUiState>()
            .add_systems(Update, update_card_face_visibility);

        app.world_mut()
            .spawn((CardFaceLayer::new(CardFace::Front), Visibility::Visible));
        app.world_mut()
            .spawn((CardFaceLayer::new(CardFace::Back), Visibility::Hidden));

        app.update();
        {
            let mut flip_state = app.world_mut().resource_mut::<CardFlipState>();
            flip_state.current_y_rotation = std::f32::consts::PI;
            flip_state.visible_face = CardFace::Back;
        }
        app.update();

        let mut face_query = app.world_mut().query::<(&CardFaceLayer, &Visibility)>();
        for (face_layer, visibility) in face_query.iter(app.world()) {
            match face_layer.face {
                CardFace::Front => assert_eq!(*visibility, Visibility::Hidden),
                CardFace::Back => assert_eq!(*visibility, Visibility::Visible),
            }
        }
    }

    #[test]
    fn composed_card_rotation_layers_flip_over_pointer_rotation() {
        let card_state = CardInspectionState {
            last_pointer_normalized: Vec2::ZERO,
            target_rotation: Quat::from_euler(EulerRot::XYZ, 0.2, -0.1, 0.0),
        };
        let flip_state = CardFlipState {
            current_y_rotation: std::f32::consts::PI,
            target_y_rotation: std::f32::consts::PI,
            visible_face: CardFace::Back,
        };

        let rotation = composed_card_rotation(&card_state, &flip_state);

        assert_ne!(rotation, card_state.target_rotation);
        assert_eq!(rotation, card_state.target_rotation * flip_state.rotation());
    }

    #[test]
    fn flip_from_non_neutral_pointer_rotation_does_not_snap_to_neutral() {
        let card_state = CardInspectionState {
            last_pointer_normalized: Vec2::ONE,
            target_rotation: target_rotation_for_pointer(
                Vec2::new(0.6, -0.4),
                &CardInspectionDefaults::default(),
            ),
        };
        let mut flip_state = CardFlipState::default();

        flip_state.request_flip();
        flip_state.advance(crate::runtime::resources::CARD_FLIP_DURATION_SECONDS * 0.5);
        let rotation = composed_card_rotation(&card_state, &flip_state);

        assert_ne!(rotation, Quat::IDENTITY);
        assert_ne!(rotation, flip_state.rotation());
    }

    #[test]
    fn card_ui_toggle_while_back_visible_keeps_card_back_visible() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<Touches>()
            .init_resource::<GameTicks>()
            .init_resource::<ActiveView>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .init_resource::<CardUiState>()
            .add_systems(Startup, setup_card_browser_view)
            .add_systems(Update, card_model_input_system);

        app.update();
        *app.world_mut().resource_mut::<ActiveView>() = ActiveView::CardBrowserView;
        {
            app.world_mut()
                .resource_mut::<CardInspectionState>()
                .target_rotation = Quat::from_euler(EulerRot::XYZ, 0.15, -0.12, 0.0);
            let mut flip_state = app.world_mut().resource_mut::<CardFlipState>();
            flip_state.current_y_rotation = std::f32::consts::PI;
            flip_state.target_y_rotation = std::f32::consts::PI;
            flip_state.visible_face = CardFace::Back;
        }
        let expected_rotation = composed_rotation_for_face(
            app.world().resource::<CardInspectionState>(),
            CardFace::Back,
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyT);
        app.update();

        assert_eq!(app.world().resource::<ActiveCardModel>().index, 0);
        assert_eq!(
            app.world().resource::<CardUiState>().depth_factor,
            CARD_DEPTH_FACTOR_MAX
        );
        let mut face_query = app.world_mut().query::<(&CardFaceLayer, &Visibility)>();
        let back_visible = face_query
            .iter(app.world())
            .any(|(face_layer, visibility)| {
                face_layer.face == CardFace::Back && *visibility == Visibility::Visible
            });

        assert!(back_visible);
        let mut card_query = app
            .world_mut()
            .query_filtered::<&Transform, With<CardView>>();
        let card_transform = card_query.single(app.world()).unwrap();
        assert!(card_transform.rotation.angle_between(expected_rotation) < 0.000_1);
        assert!(
            app.world()
                .resource::<CardInspectionState>()
                .target_rotation
                .angle_between(Quat::from_euler(EulerRot::XYZ, 0.15, -0.12, 0.0))
                < 0.000_1
        );
    }

    #[test]
    fn card_ui_toggle_while_front_visible_changes_global_card_settings() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<Touches>()
            .init_resource::<GameTicks>()
            .init_resource::<ActiveView>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .init_resource::<CardUiState>()
            .add_systems(Startup, setup_card_browser_view)
            .add_systems(Update, card_model_input_system);

        app.update();
        *app.world_mut().resource_mut::<ActiveView>() = ActiveView::CardBrowserView;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyT);
        app.update();

        assert_eq!(app.world().resource::<ActiveCardModel>().index, 0);
        assert_eq!(
            app.world().resource::<CardUiState>().depth_factor,
            CARD_DEPTH_FACTOR_MAX
        );
        let mut name_query = app.world_mut().query::<&Name>();
        assert!(
            name_query
                .iter(app.world())
                .any(|name| name.as_str().contains("KAGE REN"))
        );
    }

    #[test]
    fn card_browser_card_layers_use_shared_render_aspect_ratio() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<CardBackgroundMaskMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut layer_query =
            app.world_mut()
                .query::<(&CardParallaxLayer, &Mesh3d, Option<&CardBackgroundLayer>)>();
        for (layer, mesh_handle, background_layer) in layer_query.iter(app.world()) {
            if layer.role == CardLayerRole::Background
                && !background_layer.is_some_and(|layer| layer.uses_frame_mask)
            {
                continue;
            }

            let mesh = app
                .world()
                .resource::<Assets<Mesh>>()
                .get(&mesh_handle.0)
                .unwrap();
            let (width, height) = mesh_bounds(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
            assert_close(width / height, CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT);
        }
    }

    #[test]
    fn kage_ren_background_uses_frame_mask_full_card_space() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<CardBackgroundMaskMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let card_defaults = CardInspectionDefaults::default();
        let frame_dimensions = frame_dimensions(&card_defaults);
        let mut background_query =
            app.world_mut()
                .query::<(&CardParallaxLayer, &CardBackgroundLayer, &Mesh3d)>();
        let (background_layer, background_mesh_handle) = background_query
            .iter(app.world())
            .find_map(|(parallax_layer, background_layer, mesh_handle)| {
                (parallax_layer.role == CardLayerRole::Background)
                    .then_some((background_layer, mesh_handle))
            })
            .unwrap();
        let mesh = app
            .world()
            .resource::<Assets<Mesh>>()
            .get(&background_mesh_handle.0)
            .unwrap();

        let (width, height) = mesh_bounds(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        let (uv_width, uv_height) = mesh_uv_bounds(mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap());
        let (mask_uv_width, mask_uv_height) =
            mesh_uv_bounds(mesh.attribute(Mesh::ATTRIBUTE_UV_1).unwrap());

        assert!(background_layer.uses_frame_mask);
        assert_close(width, card_defaults.width);
        assert_close(height, card_defaults.height);
        assert_close(uv_width, 1.0);
        assert_close(uv_height, 1.0);
        assert_close(mask_uv_width, 1.0);
        assert_close(mask_uv_height, 1.0);

        let mut material_query = app.world_mut().query::<(
            &CardParallaxLayer,
            &MeshMaterial3d<CardBackgroundMaskMaterial>,
        )>();
        let masked_material_handle = material_query
            .iter(app.world())
            .find_map(|(parallax_layer, material)| {
                (parallax_layer.role == CardLayerRole::Background).then_some(material)
            })
            .unwrap();
        let masked_material = app
            .world()
            .resource::<Assets<CardBackgroundMaskMaterial>>()
            .get(&masked_material_handle.0)
            .unwrap();
        assert_eq!(
            masked_material.inner_aperture,
            frame_mask_inner_aperture(&card_defaults, &frame_dimensions)
        );
        assert_eq!(masked_material.alpha_mode, AlphaMode::Blend);
    }

    #[test]
    fn unmasked_background_geometry_is_clipped_to_rectangular_frame_hole() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .insert_resource(ActiveCardModel { index: 1 })
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let card_defaults = CardInspectionDefaults::default();
        let frame_dimensions = frame_dimensions(&card_defaults);
        let mut background_query =
            app.world_mut()
                .query::<(&CardParallaxLayer, &CardBackgroundLayer, &Mesh3d)>();
        let (background_layer, background_mesh_handle) = background_query
            .iter(app.world())
            .find_map(|(parallax_layer, background_layer, mesh_handle)| {
                (parallax_layer.role == CardLayerRole::Background)
                    .then_some((background_layer, mesh_handle))
            })
            .unwrap();
        let mesh = app
            .world()
            .resource::<Assets<Mesh>>()
            .get(&background_mesh_handle.0)
            .unwrap();

        let (width, height) = mesh_bounds(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        let (uv_width, uv_height) = mesh_uv_bounds(mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap());

        assert!(!background_layer.uses_frame_mask);
        assert_close(width, frame_dimensions.hole_width);
        assert_close(height, frame_dimensions.hole_height);
        assert_close(uv_width, 1.0 / BACKGROUND_APERTURE_SCALE);
        assert_close(uv_height, 1.0 / BACKGROUND_APERTURE_SCALE);
    }

    #[test]
    fn layer_materials_have_stable_front_to_back_depth_biases() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view);

        app.update();

        let mut layer_query = app
            .world_mut()
            .query::<(&CardParallaxLayer, &MeshMaterial3d<StandardMaterial>)>();
        let mut layer_biases: Vec<(CardLayerRole, f32, AlphaMode)> = layer_query
            .iter(app.world())
            .map(|(layer, material_handle)| {
                let material = app
                    .world()
                    .resource::<Assets<StandardMaterial>>()
                    .get(&material_handle.0)
                    .unwrap();
                (layer.role, material.depth_bias, material.alpha_mode)
            })
            .collect();

        layer_biases.sort_by(|left, right| left.1.total_cmp(&right.1));

        assert_eq!(
            layer_biases,
            vec![
                (
                    CardLayerRole::Background,
                    BACKGROUND_DEPTH_BIAS,
                    AlphaMode::Opaque
                ),
                (CardLayerRole::Frame, FRAME_DEPTH_BIAS, AlphaMode::Opaque),
                (
                    CardLayerRole::SafeArea,
                    SAFE_AREA_DEPTH_BIAS,
                    AlphaMode::Blend
                ),
                (
                    CardLayerRole::Foreground,
                    FOREGROUND_DEPTH_BIAS,
                    AlphaMode::AlphaToCoverage
                ),
                (
                    CardLayerRole::Title,
                    TITLE_DEPTH_BIAS,
                    AlphaMode::AlphaToCoverage
                ),
            ]
        );
    }

    #[test]
    fn card_ui_layer_scales_apply_without_moving_layer_centers() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardUiState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .add_systems(Startup, setup_card_browser_view)
            .add_systems(Update, update_card_parallax_layers);

        app.update();
        {
            let mut card_ui_state = app.world_mut().resource_mut::<CardUiState>();
            card_ui_state.background_layer_scale = 0.5;
            card_ui_state.frame_layer_scale = 0.75;
            card_ui_state.foreground_layer_scale = 1.25;
            card_ui_state.title_layer_scale = 1.5;
        }
        app.update();

        let mut layer_query = app.world_mut().query::<(
            &CardParallaxLayer,
            &Transform,
            Option<&CardBackgroundLayer>,
            Option<&Mesh3d>,
        )>();
        for (layer, transform, background_layer, mesh_handle) in layer_query.iter(app.world()) {
            let expected_scale = match layer.role {
                CardLayerRole::Background
                    if background_layer.is_some_and(|layer| layer.uses_frame_mask) =>
                {
                    0.75
                }
                CardLayerRole::Background => 0.5,
                CardLayerRole::Frame => 0.75,
                CardLayerRole::SafeArea => 1.0,
                CardLayerRole::Foreground => 1.25,
                CardLayerRole::Title => 1.5,
            };
            assert_eq!(
                transform.scale,
                Vec3::new(expected_scale, expected_scale, 1.0)
            );
            assert_eq!(transform.translation, layer.neutral_translation);

            if background_layer.is_some_and(|layer| layer.uses_frame_mask) {
                let mesh_handle = mesh_handle.unwrap();
                let mesh = app
                    .world()
                    .resource::<Assets<Mesh>>()
                    .get(&mesh_handle.0)
                    .unwrap();
                let (background_uv_width, background_uv_height) =
                    mesh_uv_bounds(mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap());
                let (mask_uv_width, mask_uv_height) =
                    mesh_uv_bounds(mesh.attribute(Mesh::ATTRIBUTE_UV_1).unwrap());

                assert_close(background_uv_width, 2.0);
                assert_close(background_uv_height, 2.0);
                assert_close(mask_uv_width, 1.0);
                assert_close(mask_uv_height, 1.0);
            }
        }
    }

    #[test]
    fn debug_hud_includes_card_model_toggle_key() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let key_codes: Vec<KeyCode> = app
            .world_mut()
            .query::<&DebugHudKeyText>()
            .iter(app.world())
            .map(|key_text| key_text.key_code)
            .collect();

        assert!(key_codes.contains(&KeyCode::KeyT));
    }

    #[test]
    fn debug_hud_excludes_removed_scene_browser_toggle_key() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let key_codes: Vec<KeyCode> = app
            .world_mut()
            .query::<&DebugHudKeyText>()
            .iter(app.world())
            .map(|key_text| key_text.key_code)
            .collect();

        assert!(!key_codes.contains(&KeyCode::KeyB));
    }

    #[test]
    fn debug_hud_excludes_invisible_escape_quit_key() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let key_codes: Vec<KeyCode> = app
            .world_mut()
            .query::<&DebugHudKeyText>()
            .iter(app.world())
            .map(|key_text| key_text.key_code)
            .collect();

        assert!(!key_codes.contains(&KeyCode::Escape));
    }

    #[test]
    fn debug_hud_restart_key_is_not_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let restart_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyR)
            .unwrap();

        assert!(!restart_key.is_toggle);
    }

    #[test]
    fn debug_hud_card_model_key_is_not_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let card_model_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyT)
            .unwrap();

        assert!(!card_model_key.is_toggle);
    }

    #[test]
    fn debug_hud_hot_reload_key_is_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let hot_reload_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyH)
            .unwrap();

        assert!(hot_reload_key.is_toggle);
    }

    #[test]
    fn debug_hud_debug_drawing_key_is_d_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let debug_drawing_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyD)
            .unwrap();

        assert!(debug_drawing_key.is_toggle);
    }

    #[test]
    fn debug_hud_removes_unused_was_keys() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let key_codes: Vec<KeyCode> = app
            .world_mut()
            .query::<&DebugHudKeyText>()
            .iter(app.world())
            .map(|key_text| key_text.key_code)
            .collect();

        assert!(!key_codes.contains(&KeyCode::KeyW));
        assert!(!key_codes.contains(&KeyCode::KeyA));
        assert!(!key_codes.contains(&KeyCode::KeyS));
    }

    #[test]
    fn debug_hud_fps_key_is_p_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let fps_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyP)
            .unwrap();

        assert!(fps_key.is_toggle);
    }

    #[test]
    fn p_key_toggles_fps_counter() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, toggle_debug_hud_inputs);

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyP);
        app.update();

        assert!(app.world().resource::<DebugHudState>().is_fps_visible);

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyP);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyP);
        app.update();

        assert!(!app.world().resource::<DebugHudState>().is_fps_visible);
    }

    #[test]
    fn d_key_toggles_debug_drawing() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, toggle_debug_hud_inputs);

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();

        assert!(
            app.world()
                .resource::<DebugHudState>()
                .is_debug_drawing_visible
        );

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyD);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();

        assert!(
            !app.world()
                .resource::<DebugHudState>()
                .is_debug_drawing_visible
        );
    }

    #[test]
    fn escape_key_requests_primary_window_close() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .add_message::<WindowCloseRequested>()
            .add_message::<AppExit>()
            .add_systems(Update, quit_app_on_escape);
        let primary_window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
        app.update();

        let close_requests: Vec<Entity> = app
            .world()
            .resource::<Messages<WindowCloseRequested>>()
            .iter_current_update_messages()
            .map(|event| event.window)
            .collect();

        assert_eq!(close_requests, vec![primary_window]);
        assert!(
            app.world()
                .resource::<Messages<AppExit>>()
                .iter_current_update_messages()
                .next()
                .is_none()
        );
    }

    #[test]
    fn f_key_toggles_fullscreen_window_mode() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, toggle_debug_hud_inputs);
        app.world_mut().spawn((Window::default(), PrimaryWindow));

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        assert!(app.world().resource::<DebugHudState>().is_fullscreen);
        assert!(!app.world().resource::<DebugHudState>().is_fps_visible);
        let window = app
            .world_mut()
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(app.world())
            .unwrap();
        assert_eq!(
            window.mode,
            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        );

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyF);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        assert!(!app.world().resource::<DebugHudState>().is_fullscreen);
        let window_mode = app
            .world_mut()
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(app.world())
            .unwrap()
            .mode
            .clone();
        assert_eq!(window_mode, WindowMode::Windowed);
    }

    #[test]
    fn f_key_fullscreens_on_current_monitor_and_restores_windowed_placement() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .init_resource::<WindowPlacementState>()
            .add_systems(Update, toggle_debug_hud_inputs);
        app.world_mut()
            .spawn(test_monitor("Primary", IVec2::ZERO, UVec2::new(1920, 1080)));
        let secondary_monitor = app
            .world_mut()
            .spawn(test_monitor(
                "Secondary",
                IVec2::new(1920, 0),
                UVec2::new(1920, 1080),
            ))
            .id();
        app.world_mut().spawn((
            Window {
                position: WindowPosition::At(IVec2::new(2020, 80)),
                resolution: WindowResolution::new(800, 600),
                ..Default::default()
            },
            PrimaryWindow,
        ));

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        let window = app
            .world_mut()
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(app.world())
            .unwrap();
        assert_eq!(
            window.mode,
            WindowMode::BorderlessFullscreen(MonitorSelection::Entity(secondary_monitor))
        );
        assert_eq!(
            app.world()
                .resource::<WindowPlacementState>()
                .current
                .as_ref()
                .map(|placement| (
                    placement.window_position,
                    placement.window_size,
                    placement.monitor_name.clone()
                )),
            Some((
                IVec2::new(2020, 80),
                UVec2::new(800, 600),
                Some("Secondary".to_string())
            ))
        );

        {
            let mut window = app
                .world_mut()
                .query_filtered::<&mut Window, With<PrimaryWindow>>()
                .single_mut(app.world_mut())
                .unwrap();
            window.position = WindowPosition::At(IVec2::new(1920, 0));
            window.resolution = WindowResolution::new(1920, 1080);
        }
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyF);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        let window = app
            .world_mut()
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(app.world())
            .unwrap();
        assert_eq!(window.mode, WindowMode::Windowed);
        assert_eq!(window.position, WindowPosition::At(IVec2::new(2020, 80)));
        assert_eq!(logical_window_size(window), UVec2::new(800, 600));
    }

    #[test]
    fn h_key_toggles_hot_reload_autorestart() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, toggle_debug_hud_inputs);

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyH);
        app.update();

        assert!(
            app.world()
                .resource::<DebugHudState>()
                .is_hot_reload_autorestart_enabled
        );

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyH);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyH);
        app.update();

        assert!(
            !app.world()
                .resource::<DebugHudState>()
                .is_hot_reload_autorestart_enabled
        );
    }

    #[test]
    fn restart_key_reloads_card_browser_view_and_keeps_app_scene() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<Touches>()
            .init_resource::<GameTicks>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardFlipState>()
            .init_resource::<CardModelRegistry>()
            .init_resource::<ActiveCardModel>()
            .init_resource::<WorldModelRegistry>()
            .init_resource::<ActiveWorldModel>()
            .init_resource::<LocationModelRegistry>()
            .init_resource::<ActiveLocations>()
            .init_resource::<ActiveView>()
            .add_systems(Startup, setup_app_scene)
            .add_systems(Startup, setup_card_browser_view)
            .add_systems(Update, restart_app_scene);

        *app.world_mut().resource_mut::<ActiveView>() = ActiveView::CardBrowserView;

        app.update();

        app.world_mut().resource_mut::<GameTicks>().0 = 42;
        app.world_mut()
            .resource_mut::<CardInspectionState>()
            .last_pointer_normalized = Vec2::ONE;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimaryViewCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 1);

        let mut hud_query = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudText>>();
        assert_eq!(hud_query.iter(app.world()).count(), 1);

        let mut card_query = app.world_mut().query_filtered::<Entity, With<CardView>>();
        assert_eq!(card_query.iter(app.world()).count(), 1);
        assert_eq!(app.world().resource::<GameTicks>().0, 0);
        assert_eq!(
            app.world()
                .resource::<CardInspectionState>()
                .last_pointer_normalized,
            Vec2::ZERO
        );
    }

    #[test]
    fn restored_resolution_applies_saved_size_as_logical_units() {
        let mut current_resolution = WindowResolution::new(1024, 768);
        current_resolution.set_scale_factor(1.5);

        let restored = restored_window_resolution(&current_resolution, UVec2::new(512, 384));

        assert_eq!(restored.width(), 512.0);
        assert_eq!(restored.height(), 384.0);
        assert_eq!(restored.physical_width(), 768);
        assert_eq!(restored.physical_height(), 576);
    }
}
