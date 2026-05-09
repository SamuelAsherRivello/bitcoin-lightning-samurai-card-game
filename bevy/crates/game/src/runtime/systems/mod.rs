use bevy::{
    asset::RenderAssetUsages,
    ecs::system::SystemParam,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::view::NoIndirectDrawing,
    text::{Underline, UnderlineColor},
    ui::UiScale,
    window::{
        Monitor, PrimaryWindow, WindowCloseRequested, WindowMoved, WindowResized, WindowResolution,
    },
};
use bevy_card_game_shared::{
    GameTitle,
    window::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH},
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
    bevy_inspector::EntityFilter,
};
use bevy_persistent::prelude::Persistent;

use crate::runtime::components::{
    AppSceneEntity, AppSceneRoot, CardBrowserSceneEntity, CardBrowserSceneRoot, CardFrameLayer,
    CardLayerRole, CardParallaxLayer, CardPlaceholder, DebugHudFpsText, DebugHudKeyText,
    DebugHudText, InspectorState, Player, PrimarySceneCamera,
};
use crate::runtime::resources::{
    ActiveCardTheme, CARD_DEPTH_FACTOR_MAX, CARD_DEPTH_FACTOR_MIN, CardInspectionDefaults,
    CardInspectionState, CardTheme, CardThemeRegistry, CardUiState, DebugHudInputStore,
    DebugHudState, GameTicks, PrimaryCameraDefaults, WindowPlacement, WindowPlacementState,
    WindowPlacementStore, load_window_placement, valid_window_placement,
};

#[cfg(feature = "desktop-hot-reload")]
use crate::runtime::resources::desktop_hot_reload_patch_count;

#[cfg(test)]
use bevy::mesh::VertexAttributeValues;

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;
const SCREEN_PADDING_TOP: f32 = 24.0;
const SCREEN_PADDING_LEFT: f32 = 24.0;
const TARGET_WIDTH: f32 = DEFAULT_WINDOW_WIDTH as f32;
const TARGET_HEIGHT: f32 = DEFAULT_WINDOW_HEIGHT as f32;
const DEBUG_HUD_FONT_SIZE: f32 = 22.0;
const DEBUG_WINDOW_FONT_SIZE: f32 = 14.0;
const BACKGROUND_APPARENT_DEPTH: f32 = -1.0;
const FRAME_APPARENT_DEPTH: f32 = 0.0;
const FOREGROUND_APPARENT_DEPTH: f32 = 1.0;
const TITLE_APPARENT_DEPTH: f32 = 2.0;
const LAYER_RENDER_Z_STEP: f32 = 0.0001;
const BACKGROUND_DEPTH_BIAS: f32 = 0.0;
const FRAME_DEPTH_BIAS: f32 = 8.0;
const FOREGROUND_DEPTH_BIAS: f32 = 16.0;
const TITLE_DEPTH_BIAS: f32 = 24.0;
const PARALLAX_OFFSET_RATIO: f32 = 0.065;
const FRAME_THICKNESS_RATIO: f32 = 0.05;
const BACKGROUND_APERTURE_SCALE: f32 = 1.5;
const FOREGROUND_WIDTH_RATIO: f32 = 0.72;
const TITLE_WIDTH_RATIO: f32 = 0.92;
const TITLE_HEIGHT_RATIO: f32 = 0.22;
const FRAME_SHINE_STRENGTH: f32 = 0.22;

pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn setup_primary_camera(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    spawn_primary_camera(&mut commands, &camera_defaults);
}

fn spawn_primary_camera(commands: &mut Commands, camera_defaults: &PrimaryCameraDefaults) {
    commands.spawn((
        Name::new("Primary 3D Camera"),
        PrimarySceneCamera,
        AppSceneEntity,
        Camera3d::default(),
        NoIndirectDrawing,
        Projection::Perspective(PerspectiveProjection {
            fov: camera_defaults.fov_radians,
            near: camera_defaults.near,
            far: camera_defaults.far,
            ..Default::default()
        }),
        camera_defaults.transform(),
    ));
}

pub fn setup_app_scene(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    commands.spawn((
        Name::new("AppScene"),
        AppSceneRoot,
        AppSceneEntity,
        Transform::default(),
        Visibility::default(),
    ));
    spawn_primary_camera(&mut commands, &camera_defaults);
    spawn_app_light(&mut commands);
    spawn_debug_hud(&mut commands);
}

fn spawn_app_light(commands: &mut Commands) {
    commands.spawn((
        Name::new("AppScene Key Light"),
        AppSceneEntity,
        DirectionalLight {
            illuminance: 1500.0,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.45, -0.35, 0.0)),
    ));
}

pub fn setup_card_browser_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    theme_registry: Res<CardThemeRegistry>,
    active_theme: Res<ActiveCardTheme>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_card_structure(
        &mut commands,
        &asset_server,
        &card_defaults,
        &theme_registry,
        &active_theme,
        &mut meshes,
        &mut materials,
    );
}

pub fn setup_card_placeholder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    theme_registry: Res<CardThemeRegistry>,
    active_theme: Res<ActiveCardTheme>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_card_structure(
        &mut commands,
        &asset_server,
        &card_defaults,
        &theme_registry,
        &active_theme,
        &mut meshes,
        &mut materials,
    );
}

fn spawn_card_structure(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    theme_registry: &CardThemeRegistry,
    active_theme: &ActiveCardTheme,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let theme = theme_registry
        .active_theme(&active_theme)
        .cloned()
        .unwrap_or_else(CardTheme::skybolt);
    let background_material = themed_material(
        asset_server,
        materials,
        theme.background_texture,
        AlphaMode::Opaque,
        BACKGROUND_DEPTH_BIAS,
    );
    let frame_material = themed_material(
        asset_server,
        materials,
        theme.frame_texture,
        AlphaMode::Opaque,
        FRAME_DEPTH_BIAS,
    );
    let foreground_material = themed_material(
        asset_server,
        materials,
        theme.foreground_texture,
        AlphaMode::AlphaToCoverage,
        FOREGROUND_DEPTH_BIAS,
    );
    let title_material = themed_material(
        asset_server,
        materials,
        theme.title_texture,
        AlphaMode::AlphaToCoverage,
        TITLE_DEPTH_BIAS,
    );

    let frame_dimensions = frame_dimensions(&card_defaults);
    let card_front_z = (card_defaults.thickness * 0.5) + LAYER_RENDER_Z_STEP;
    let background_z = card_front_z;
    let frame_z = card_front_z + (LAYER_RENDER_Z_STEP * 3.0);
    let foreground_z = card_front_z + (LAYER_RENDER_Z_STEP * 5.0);
    let title_z = card_front_z + (LAYER_RENDER_Z_STEP * 7.0);

    let background_mesh = meshes.add(background_aperture_mesh(&frame_dimensions, Vec2::ZERO));
    let frame_mesh = meshes.add(frame_cutout_mesh(card_defaults, &frame_dimensions));
    let foreground_width = card_defaults.width * FOREGROUND_WIDTH_RATIO;
    let foreground_height = card_defaults.height * theme.foreground_height_ratio;
    let foreground_mesh = meshes.add(Rectangle::new(foreground_width, foreground_height));
    let title_mesh = meshes.add(Rectangle::new(
        card_defaults.width * TITLE_WIDTH_RATIO,
        card_defaults.height * TITLE_HEIGHT_RATIO,
    ));

    commands
        .spawn((
            Name::new(format!("CardStructure {}", theme.display_name)),
            CardPlaceholder,
            CardBrowserSceneRoot,
            CardBrowserSceneEntity,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            spawn_parallax_plane(
                parent,
                Name::new(format!("Card Background {}", theme.display_name)),
                background_mesh,
                background_material,
                CardLayerRole::Background,
                BACKGROUND_APPARENT_DEPTH,
                Vec3::new(0.0, 0.0, background_z),
                false,
            );

            spawn_parallax_plane(
                parent,
                Name::new("Card Frame Cutout"),
                frame_mesh,
                frame_material.clone(),
                CardLayerRole::Frame,
                FRAME_APPARENT_DEPTH,
                Vec3::new(0.0, 0.0, frame_z),
                true,
            );

            spawn_parallax_plane(
                parent,
                Name::new(format!("Card Foreground {} Character", theme.display_name)),
                foreground_mesh,
                foreground_material,
                CardLayerRole::Foreground,
                FOREGROUND_APPARENT_DEPTH,
                Vec3::new(
                    card_defaults.width * theme.foreground_x_ratio,
                    card_defaults.height * theme.foreground_y_ratio,
                    foreground_z,
                ),
                false,
            );
            spawn_parallax_plane(
                parent,
                Name::new(format!("Card Title {}", theme.display_name)),
                title_mesh,
                title_material,
                CardLayerRole::Title,
                TITLE_APPARENT_DEPTH,
                Vec3::new(0.0, card_defaults.height * theme.title_y_ratio, title_z),
                false,
            );
        });
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
    let half_width = frame_dimensions.hole_width * 0.5;
    let half_height = frame_dimensions.hole_height * 0.5;
    let positions = vec![
        [-half_width, -half_height, 0.0],
        [half_width, -half_height, 0.0],
        [half_width, half_height, 0.0],
        [-half_width, half_height, 0.0],
    ];
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = background_aperture_uvs(uv_offset);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn background_aperture_uvs(uv_offset: Vec2) -> Vec<[f32; 2]> {
    let visible_uv_size = 1.0 / BACKGROUND_APERTURE_SCALE;
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

fn themed_material(
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
    is_frame: bool,
) {
    let mut entity = parent.spawn((
        name,
        Mesh3d(mesh),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(neutral_translation),
        CardParallaxLayer::new(role, apparent_depth, neutral_translation),
    ));
    if is_frame {
        entity.insert(CardFrameLayer);
    }
}

pub fn track_card_pointer_target(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    card_defaults: Res<CardInspectionDefaults>,
    mut card_state: ResMut<CardInspectionState>,
) {
    let Ok(primary_window) = primary_window_query.single() else {
        return;
    };
    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };

    let window_size = Vec2::new(
        primary_window.resolution.width(),
        primary_window.resolution.height(),
    );
    update_card_target_from_pointer(
        cursor_position,
        window_size,
        &card_defaults,
        &mut card_state,
    );
}

pub fn smooth_card_rotation(
    time: Res<Time>,
    card_defaults: Res<CardInspectionDefaults>,
    card_state: Res<CardInspectionState>,
    mut card_query: Query<&mut Transform, With<CardPlaceholder>>,
) {
    let Ok(mut transform) = card_query.single_mut() else {
        return;
    };

    let response_seconds = card_defaults.smoothing_response_seconds.max(f32::EPSILON);
    let blend = 1.0 - 0.01_f32.powf(time.delta_secs() / response_seconds);
    transform.rotation = transform.rotation.slerp(card_state.target_rotation, blend);
    transform.translation = Vec3::ZERO;
}

pub fn update_card_parallax_layers(
    card_defaults: Res<CardInspectionDefaults>,
    card_ui_state: Res<CardUiState>,
    card_query: Query<&Transform, (With<CardPlaceholder>, Without<CardParallaxLayer>)>,
    mut layer_query: Query<(&CardParallaxLayer, &mut Transform, Option<&Mesh3d>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(card_transform) = card_query.single() else {
        return;
    };

    let (yaw, pitch, _) = card_transform.rotation.to_euler(EulerRot::YXZ);
    let max_tilt = card_defaults.max_tilt_radians.max(f32::EPSILON);
    let tilt =
        Vec2::new(yaw / max_tilt, -pitch / max_tilt).clamp(Vec2::splat(-1.0), Vec2::splat(1.0));
    let max_offset = Vec2::new(
        card_defaults.width * PARALLAX_OFFSET_RATIO,
        card_defaults.height * PARALLAX_OFFSET_RATIO,
    );
    let depth_multiplier = card_ui_state.depth_multiplier();

    let frame_dimensions = frame_dimensions(&card_defaults);
    let background_virtual_size = Vec2::new(
        frame_dimensions.hole_width * BACKGROUND_APERTURE_SCALE,
        frame_dimensions.hole_height * BACKGROUND_APERTURE_SCALE,
    );

    for (layer, mut transform, mesh_handle) in &mut layer_query {
        let offset = tilt * max_offset * layer.apparent_depth * depth_multiplier;
        if layer.role == CardLayerRole::Background {
            transform.translation = layer.neutral_translation;
            if let Some(mesh_handle) = mesh_handle {
                if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                    let uv_offset = Vec2::new(
                        -offset.x / background_virtual_size.x,
                        offset.y / background_virtual_size.y,
                    );
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, background_aperture_uvs(uv_offset));
                }
            }
        } else {
            transform.translation = layer.neutral_translation + Vec3::new(offset.x, offset.y, 0.0);
        }
    }
}

pub fn update_card_frame_shine(
    card_defaults: Res<CardInspectionDefaults>,
    card_query: Query<&Transform, (With<CardPlaceholder>, Without<CardFrameLayer>)>,
    frame_query: Query<&MeshMaterial3d<StandardMaterial>, With<CardFrameLayer>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(card_transform) = card_query.single() else {
        return;
    };

    let (yaw, pitch, _) = card_transform.rotation.to_euler(EulerRot::YXZ);
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

pub fn toggle_card_theme(
    keys: Res<ButtonInput<KeyCode>>,
    registry: Res<CardThemeRegistry>,
    mut active_theme: ResMut<ActiveCardTheme>,
    mut commands: Commands,
    scene_entities: Query<Entity, With<CardBrowserSceneEntity>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut card_state: ResMut<CardInspectionState>,
    mut ticks: ResMut<GameTicks>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        let previous_index = active_theme.index;
        active_theme.toggle(&registry);
        if active_theme.index != previous_index {
            reload_card_browser_scene(
                &mut commands,
                &scene_entities,
                &asset_server,
                &card_defaults,
                &registry,
                &active_theme,
                &mut meshes,
                &mut materials,
                &mut card_state,
                &mut ticks,
            );
        }
    }
}

pub fn restart_card_browser_scene(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    scene_entities: Query<Entity, With<CardBrowserSceneEntity>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    theme_registry: Res<CardThemeRegistry>,
    active_theme: Res<ActiveCardTheme>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut card_state: ResMut<CardInspectionState>,
    mut ticks: ResMut<GameTicks>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    reload_card_browser_scene(
        &mut commands,
        &scene_entities,
        &asset_server,
        &card_defaults,
        &theme_registry,
        &active_theme,
        &mut meshes,
        &mut materials,
        &mut card_state,
        &mut ticks,
    );
}

#[cfg(feature = "desktop-hot-reload")]
pub fn hot_reload_auto_restart_card_browser_scene(
    mut last_seen_patch_count: Local<u64>,
    hud_state: Res<DebugHudState>,
    mut commands: Commands,
    scene_entities: Query<Entity, With<CardBrowserSceneEntity>>,
    asset_server: Res<AssetServer>,
    card_defaults: Res<CardInspectionDefaults>,
    theme_registry: Res<CardThemeRegistry>,
    active_theme: Res<ActiveCardTheme>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut card_state: ResMut<CardInspectionState>,
    mut ticks: ResMut<GameTicks>,
) {
    let patch_count = desktop_hot_reload_patch_count();
    if patch_count == *last_seen_patch_count {
        return;
    }

    *last_seen_patch_count = patch_count;

    if !hud_state.is_hot_reload_autorestart_enabled {
        return;
    }

    reload_card_browser_scene(
        &mut commands,
        &scene_entities,
        &asset_server,
        &card_defaults,
        &theme_registry,
        &active_theme,
        &mut meshes,
        &mut materials,
        &mut card_state,
        &mut ticks,
    );
}

#[cfg(not(feature = "desktop-hot-reload"))]
pub fn hot_reload_auto_restart_card_browser_scene() {}

fn reload_card_browser_scene(
    commands: &mut Commands,
    scene_entities: &Query<Entity, With<CardBrowserSceneEntity>>,
    asset_server: &AssetServer,
    card_defaults: &CardInspectionDefaults,
    theme_registry: &CardThemeRegistry,
    active_theme: &ActiveCardTheme,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    card_state: &mut CardInspectionState,
    ticks: &mut GameTicks,
) {
    for entity in scene_entities.iter() {
        commands.entity(entity).despawn();
    }

    *card_state = CardInspectionState::default();
    ticks.0 = 0;
    spawn_card_structure(
        commands,
        asset_server,
        card_defaults,
        theme_registry,
        active_theme,
        meshes,
        materials,
    );
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
    persistent_input: Option<Res<Persistent<DebugHudInputStore>>>,
) {
    if let Some(persistent_input) = persistent_input {
        persistent_input.apply_to_state(&mut hud_state);
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

fn spawn_debug_hud(commands: &mut Commands) {
    commands
        .spawn((
            Text::new("Card Browser\nFrame: 0\nKEYS: "),
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
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
            AppSceneEntity,
            DebugHudText,
        ))
        .with_children(|parent| {
            spawn_key_span(parent, "W", KeyCode::KeyW, false);
            spawn_key_span(parent, "A", KeyCode::KeyA, false);
            spawn_key_span(parent, "S", KeyCode::KeyS, false);
            spawn_key_span(parent, "D", KeyCode::KeyD, false);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "R", KeyCode::KeyR, false);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "T", KeyCode::KeyT, false);
            parent.spawn((TextSpan::new("\nKEYS: "), debug_hud_text_font()));
            spawn_key_span(parent, "F", KeyCode::KeyF, true);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "I", KeyCode::KeyI, true);
            parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
            spawn_key_span(parent, "H", KeyCode::KeyH, true);
            parent.spawn((TextSpan::new(""), debug_hud_text_font(), DebugHudFpsText));
        });
}

#[derive(SystemParam)]
pub struct DebugHudUpdateParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    ticks: Res<'w, GameTicks>,
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
                KeyCode::KeyF => fps_on,
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

    let full_text = format!("Card Browser\nFrame: {}\nKEYS: ", params.ticks.0);
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
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
) {
    let mut changed = false;

    if keys.just_pressed(KeyCode::KeyF) {
        hud_state.is_fps_visible = !hud_state.is_fps_visible;
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
}

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

pub fn scale_debug_hud(
    mut window_resized_events: Option<MessageReader<WindowResized>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut ui_scale: Option<ResMut<UiScale>>,
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Some(ref mut ui_scale) = ui_scale else {
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

    let width_scale = primary_window.resolution.width() / TARGET_WIDTH;
    let height_scale = primary_window.resolution.height() / TARGET_HEIGHT;
    ui_scale.0 = width_scale.min(height_scale).max(0.1);
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

    let current_window_placement = match window.position {
        WindowPosition::At(position) => placement_for_window(
            position,
            logical_window_size(window),
            window.resolution.physical_size(),
            &monitor_query,
        ),
        WindowPosition::Automatic | WindowPosition::Centered(_) => None,
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
    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    let egui_context = egui_context.get_mut();
    use_matching_debug_window_text_style(egui_context);

    let Some(mut card_ui_state) = world.get_resource_mut::<CardUiState>() else {
        return;
    };

    egui::Window::new("Card UI")
        .anchor(egui::Align2::RIGHT_CENTER, egui::vec2(-24.0, 0.0))
        .default_width(260.0)
        .resizable(false)
        .show(egui_context, |ui| {
            ui.add(
                egui::Slider::new(
                    &mut card_ui_state.depth_factor,
                    CARD_DEPTH_FACTOR_MIN..=CARD_DEPTH_FACTOR_MAX,
                )
                .text("DepthFactor"),
            );
        });
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
    fn debug_hud_title_is_card_browser_without_theme_status() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut hud_query = app
            .world_mut()
            .query_filtered::<&Text, With<DebugHudText>>();
        let hud_text = hud_query.single(app.world()).unwrap();

        assert!(hud_text.0.starts_with("Card Browser\nFrame: 0"));
        assert!(!hud_text.0.contains("Theme:"));
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
    fn app_scene_owns_camera_light_and_hud_without_card_browser_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PrimaryCameraDefaults>()
            .add_systems(Startup, setup_app_scene);

        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimarySceneCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 1);

        let mut light_query = app
            .world_mut()
            .query_filtered::<Entity, With<DirectionalLight>>();
        assert_eq!(light_query.iter(app.world()).count(), 1);

        let mut hud_query = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudText>>();
        assert_eq!(hud_query.iter(app.world()).count(), 1);

        let mut card_query = app
            .world_mut()
            .query_filtered::<Entity, With<CardPlaceholder>>();
        assert_eq!(card_query.iter(app.world()).count(), 0);
    }

    #[test]
    fn polished_layers_use_flat_artwork_with_four_apparent_depth_offsets() {
        let card_defaults = CardInspectionDefaults::default();
        let frame_dimensions = frame_dimensions(&card_defaults);

        assert_eq!(BACKGROUND_APPARENT_DEPTH, -1.0);
        assert_eq!(FRAME_APPARENT_DEPTH, 0.0);
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
        assert!(BACKGROUND_APERTURE_SCALE > 1.0);
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
            .init_resource::<CardThemeRegistry>()
            .init_resource::<ActiveCardTheme>()
            .add_systems(Startup, setup_card_browser_scene);

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
    fn background_geometry_is_clipped_to_frame_hole() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardThemeRegistry>()
            .init_resource::<ActiveCardTheme>()
            .add_systems(Startup, setup_card_browser_scene);

        app.update();

        let card_defaults = CardInspectionDefaults::default();
        let frame_dimensions = frame_dimensions(&card_defaults);
        let mut background_query = app.world_mut().query::<(&CardParallaxLayer, &Mesh3d)>();
        let background_mesh_handle = background_query
            .iter(app.world())
            .find_map(|(layer, mesh_handle)| {
                (layer.role == CardLayerRole::Background).then_some(mesh_handle)
            })
            .unwrap();
        let mesh = app
            .world()
            .resource::<Assets<Mesh>>()
            .get(&background_mesh_handle.0)
            .unwrap();

        let (width, height) = mesh_bounds(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        let (uv_width, uv_height) = mesh_uv_bounds(mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap());

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
            .init_resource::<CardThemeRegistry>()
            .init_resource::<ActiveCardTheme>()
            .add_systems(Startup, setup_card_browser_scene);

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
    fn debug_hud_includes_theme_toggle_key() {
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
    fn debug_hud_theme_key_is_not_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let theme_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyT)
            .unwrap();

        assert!(!theme_key.is_toggle);
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
    fn restart_key_reloads_card_browser_scene_and_keeps_app_scene() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_asset::<Image>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<GameTicks>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<CardThemeRegistry>()
            .init_resource::<ActiveCardTheme>()
            .add_systems(Startup, setup_app_scene)
            .add_systems(Startup, setup_card_browser_scene)
            .add_systems(Update, restart_card_browser_scene);

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
            .query_filtered::<Entity, With<PrimarySceneCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 1);

        let mut hud_query = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudText>>();
        assert_eq!(hud_query.iter(app.world()).count(), 1);

        let mut card_query = app
            .world_mut()
            .query_filtered::<Entity, With<CardPlaceholder>>();
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
