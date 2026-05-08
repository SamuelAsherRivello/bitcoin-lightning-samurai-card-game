use bevy::{
    ecs::system::SystemParam,
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
    CardParallaxLayer, CardPlaceholder, DebugHudFpsText, DebugHudKeyText, DebugHudText,
    InspectorState, Player, PrimarySceneCamera,
};
use crate::runtime::resources::{
    CardInspectionDefaults, CardInspectionState, DebugHudState, GameTicks, PrimaryCameraDefaults,
    WindowPlacement, WindowPlacementState, WindowPlacementStore, load_window_placement,
    valid_window_placement,
};

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;
const SCREEN_PADDING_TOP: f32 = 24.0;
const SCREEN_PADDING_LEFT: f32 = 24.0;
const TARGET_WIDTH: f32 = DEFAULT_WINDOW_WIDTH as f32;
const TARGET_HEIGHT: f32 = DEFAULT_WINDOW_HEIGHT as f32;
const DEBUG_HUD_FONT_SIZE: f32 = 22.0;
const DEBUG_WINDOW_FONT_SIZE: f32 = 14.0;
const BACKGROUND_DOT_COLUMNS: u32 = 10;
const BACKGROUND_DOT_ROWS: u32 = 14;
const FOREGROUND_DOT_COLUMNS: u32 = 6;
const FOREGROUND_DOT_ROWS: u32 = 8;
const BACKGROUND_DOT_SIZE_RATIO: f32 = 0.035;
const FOREGROUND_DOT_SIZE_RATIO: f32 = 0.052;
const BACKGROUND_APPARENT_DEPTH: f32 = -1.0;
const FRAME_APPARENT_DEPTH: f32 = 0.0;
const FOREGROUND_APPARENT_DEPTH: f32 = 1.0;
const LAYER_RENDER_Z_STEP: f32 = 0.0001;
const PARALLAX_OFFSET_RATIO: f32 = 0.065;

pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn setup_primary_camera(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    commands.spawn((
        Name::new("Primary 3D Camera"),
        PrimarySceneCamera,
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

pub fn setup_card_placeholder(
    mut commands: Commands,
    card_defaults: Res<CardInspectionDefaults>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let background_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.46, 0.58),
        unlit: true,
        ..Default::default()
    });
    let frame_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });
    let foreground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.62, 0.16),
        unlit: true,
        ..Default::default()
    });
    let background_dot_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.90, 0.95),
        unlit: true,
        ..Default::default()
    });
    let foreground_dot_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.19, 0.04),
        unlit: true,
        ..Default::default()
    });

    let frame_thickness_x = card_defaults.width * 0.1;
    let frame_thickness_y = card_defaults.height * 0.1;
    let hole_width = card_defaults.width - (frame_thickness_x * 2.0);
    let hole_height = card_defaults.height - (frame_thickness_y * 2.0);
    let card_front_z = (card_defaults.thickness * 0.5) + LAYER_RENDER_Z_STEP;
    let background_z = card_front_z;
    let background_detail_z = card_front_z + LAYER_RENDER_Z_STEP;
    let frame_z = card_front_z + (LAYER_RENDER_Z_STEP * 3.0);
    let foreground_z = card_front_z + (LAYER_RENDER_Z_STEP * 5.0);
    let background_width = hole_width;
    let background_height = hole_height;
    let background_dot_size = background_width * BACKGROUND_DOT_SIZE_RATIO;
    let background_dot_field_width = background_width;
    let background_dot_field_height = background_height;

    let background_mesh = meshes.add(Rectangle::new(background_width, background_height));
    let vertical_frame_mesh = meshes.add(Rectangle::new(frame_thickness_x, card_defaults.height));
    let horizontal_frame_mesh = meshes.add(Rectangle::new(hole_width, frame_thickness_y));
    let foreground_width = card_defaults.width * 0.5;
    let foreground_height = card_defaults.height * 0.5;
    let foreground_mesh = meshes.add(Rectangle::new(foreground_width, foreground_height));
    let background_dot_mesh = meshes.add(Rectangle::new(background_dot_size, background_dot_size));
    let foreground_dot_mesh = meshes.add(Rectangle::new(
        foreground_width * FOREGROUND_DOT_SIZE_RATIO,
        foreground_width * FOREGROUND_DOT_SIZE_RATIO,
    ));

    commands
        .spawn((
            Name::new("Poker Card Placeholder"),
            CardPlaceholder,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            spawn_parallax_plane(
                parent,
                Name::new("Card Background Aperture Fill"),
                background_mesh,
                background_material,
                FRAME_APPARENT_DEPTH,
                Vec3::new(0.0, 0.0, background_z),
            );
            spawn_dot_pattern(
                parent,
                "Card Background Dot",
                background_dot_field_width,
                background_dot_field_height,
                BACKGROUND_DOT_COLUMNS,
                BACKGROUND_DOT_ROWS,
                background_detail_z,
                background_dot_mesh,
                background_dot_material,
                BACKGROUND_APPARENT_DEPTH,
            );

            spawn_parallax_plane(
                parent,
                Name::new("Card Frame Left"),
                vertical_frame_mesh.clone(),
                frame_material.clone(),
                FRAME_APPARENT_DEPTH,
                Vec3::new(
                    -(card_defaults.width * 0.5) + (frame_thickness_x * 0.5),
                    0.0,
                    frame_z,
                ),
            );
            spawn_parallax_plane(
                parent,
                Name::new("Card Frame Right"),
                vertical_frame_mesh,
                frame_material.clone(),
                FRAME_APPARENT_DEPTH,
                Vec3::new(
                    (card_defaults.width * 0.5) - (frame_thickness_x * 0.5),
                    0.0,
                    frame_z,
                ),
            );
            spawn_parallax_plane(
                parent,
                Name::new("Card Frame Top"),
                horizontal_frame_mesh.clone(),
                frame_material.clone(),
                FRAME_APPARENT_DEPTH,
                Vec3::new(
                    0.0,
                    (card_defaults.height * 0.5) - (frame_thickness_y * 0.5),
                    frame_z,
                ),
            );
            spawn_parallax_plane(
                parent,
                Name::new("Card Frame Bottom"),
                horizontal_frame_mesh,
                frame_material,
                FRAME_APPARENT_DEPTH,
                Vec3::new(
                    0.0,
                    -(card_defaults.height * 0.5) + (frame_thickness_y * 0.5),
                    frame_z,
                ),
            );

            spawn_parallax_plane(
                parent,
                Name::new("Card Foreground Rectangle"),
                foreground_mesh,
                foreground_material,
                FOREGROUND_APPARENT_DEPTH,
                Vec3::new(0.0, 0.0, foreground_z),
            );
            spawn_dot_pattern(
                parent,
                "Card Foreground Dot",
                foreground_width,
                foreground_height,
                FOREGROUND_DOT_COLUMNS,
                FOREGROUND_DOT_ROWS,
                foreground_z + LAYER_RENDER_Z_STEP,
                foreground_dot_mesh,
                foreground_dot_material,
                FOREGROUND_APPARENT_DEPTH,
            );
        });
}

fn spawn_parallax_plane(
    parent: &mut ChildSpawnerCommands,
    name: Name,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    apparent_depth: f32,
    neutral_translation: Vec3,
) {
    parent.spawn((
        name,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(neutral_translation),
        CardParallaxLayer::new(apparent_depth, neutral_translation),
    ));
}

fn spawn_dot_pattern(
    parent: &mut ChildSpawnerCommands,
    name_prefix: &'static str,
    width: f32,
    height: f32,
    columns: u32,
    rows: u32,
    z: f32,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    apparent_depth: f32,
) {
    for (index, position) in dot_pattern_positions(width, height, columns, rows)
        .into_iter()
        .enumerate()
    {
        let neutral_translation = Vec3::new(position.x, position.y, z);
        parent.spawn((
            Name::new(format!("{name_prefix} {}", index + 1)),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(neutral_translation),
            CardParallaxLayer::new(apparent_depth, neutral_translation),
        ));
    }
}

fn dot_pattern_positions(width: f32, height: f32, columns: u32, rows: u32) -> Vec<Vec2> {
    if columns == 0 || rows == 0 {
        return Vec::new();
    }

    let x_step = width / columns as f32;
    let y_step = height / rows as f32;
    let x_start = (width * -0.5) + (x_step * 0.5);
    let y_start = (height * -0.5) + (y_step * 0.5);
    let mut positions = Vec::with_capacity((columns * rows) as usize);

    for row in 0..rows {
        for column in 0..columns {
            positions.push(Vec2::new(
                x_start + (column as f32 * x_step),
                y_start + (row as f32 * y_step),
            ));
        }
    }

    positions
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
    card_query: Query<&Transform, (With<CardPlaceholder>, Without<CardParallaxLayer>)>,
    mut layer_query: Query<(&CardParallaxLayer, &mut Transform)>,
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

    for (layer, mut transform) in &mut layer_query {
        let offset = tilt * max_offset * layer.apparent_depth;
        transform.translation = layer.neutral_translation + Vec3::new(offset.x, offset.y, 0.0);
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

pub fn advance_ticks(mut ticks: ResMut<GameTicks>) {
    ticks.0 += 1;
}

pub fn setup_inspector(mut commands: Commands) {
    commands.spawn((Name::new("Bevy Inspector"), InspectorState::default()));
}

pub fn setup_debug_hud(mut commands: Commands) {
    commands
        .spawn((
            Text::new("Card Inspection POC\nFrame: 0\nKEYS: "),
            TextFont {
                font_size: DEBUG_HUD_FONT_SIZE,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SCREEN_PADDING_LEFT),
                top: Val::Px(SCREEN_PADDING_TOP),
                width: Val::Px(210.0),
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
            DebugHudText,
        ))
        .with_children(|parent| {
            spawn_key_span(parent, "W", KeyCode::KeyW, false);
            spawn_key_span(parent, "A", KeyCode::KeyA, false);
            spawn_key_span(parent, "S", KeyCode::KeyS, false);
            spawn_key_span(parent, "D", KeyCode::KeyD, false);
            parent.spawn((TextSpan::new("\nKEYS: "), debug_hud_text_font()));
            spawn_key_span(parent, "F", KeyCode::KeyF, true);
            parent.spawn((TextSpan::new(" "), debug_hud_text_font()));
            spawn_key_span(parent, "I", KeyCode::KeyI, true);
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
    if params.keys.just_pressed(KeyCode::KeyF) {
        params.hud_state.is_fps_visible = !params.hud_state.is_fps_visible;
    }

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

    let full_text = format!("Card Inspection POC\nFrame: {}\nKEYS: ", params.ticks.0);
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

pub fn toggle_inspector(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector_query: Query<&mut InspectorState>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    let Ok(mut inspector) = inspector_query.single_mut() else {
        return;
    };

    inspector.is_visible = !inspector.is_visible;
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
                ui.heading("Card Inspection POC");
                bevy_inspector::ui_for_entities_filtered(world, ui, true, &InspectorEntityFilter);
                ui.allocate_space(ui.available_size());
            });
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
    fn dot_pattern_positions_fill_bounds_without_touching_edges() {
        let positions = dot_pattern_positions(1.0, 2.0, 2, 4);

        assert_eq!(positions.len(), 8);
        assert_eq!(positions.first(), Some(&Vec2::new(-0.25, -0.75)));
        assert_eq!(positions.last(), Some(&Vec2::new(0.25, 0.75)));
    }

    #[test]
    fn polished_layers_use_flat_artwork_with_apparent_depth_offsets() {
        let card_defaults = CardInspectionDefaults::default();
        let frame_thickness_x = card_defaults.width * 0.1;
        let frame_thickness_y = card_defaults.height * 0.1;
        let hole_width = card_defaults.width - (frame_thickness_x * 2.0);
        let hole_height = card_defaults.height - (frame_thickness_y * 2.0);
        let max_parallax_offset = Vec2::new(
            card_defaults.width * PARALLAX_OFFSET_RATIO,
            card_defaults.height * PARALLAX_OFFSET_RATIO,
        );
        let background_dot_size = hole_width * BACKGROUND_DOT_SIZE_RATIO;
        let background_dot_field_width = hole_width;
        let background_dot_field_height = hole_height;

        assert_eq!(BACKGROUND_APPARENT_DEPTH, -1.0);
        assert_eq!(FRAME_APPARENT_DEPTH, 0.0);
        assert_eq!(FOREGROUND_APPARENT_DEPTH, 1.0);
        assert!(LAYER_RENDER_Z_STEP < card_defaults.thickness * 0.01);
        assert!(PARALLAX_OFFSET_RATIO > 0.0);
        assert_eq!(hole_width + (frame_thickness_x * 2.0), card_defaults.width);
        assert_eq!(hole_height + (frame_thickness_y * 2.0), card_defaults.height);
        assert_eq!(background_dot_field_width, hole_width);
        assert_eq!(background_dot_field_height, hole_height);
        assert!(
            background_dot_field_width + (max_parallax_offset.x * 2.0) + background_dot_size
                <= card_defaults.width
        );
        assert!(
            background_dot_field_height + (max_parallax_offset.y * 2.0) + background_dot_size
                <= card_defaults.height
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
