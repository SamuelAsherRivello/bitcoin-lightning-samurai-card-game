use bevy::prelude::*;
use bevy_aspect_ratio_mask::Hud;
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::hot;
use std::collections::HashSet;

use crate::runtime::components::{
    AppSceneRoot, DebugDrawing, DebugHudText, GameSceneEntity, GameSceneRoot,
};
use crate::runtime::resources::{ActiveView, CardSlotBoardModel, DebugDrawingModel, DebugHudState};

const DEBUG_DRAWING_Z_INDEX: i32 = 1_100;
const DEBUG_DRAW_SOLO_OVERLAY_Z_INDEX: i32 = 1_000;

#[cfg_attr(feature = "desktop-hot-reload", hot)]
/// HUMAN: Synchronizes requested debug drawings with visible game-scene annotations.
/// AI: Keep annotations removable and aspect-ratio-safe by parenting them to existing HUD areas.
pub fn debug_drawing_update_system(
    mut commands: Commands,
    model: Res<DebugDrawingModel>,
    slot_board: Res<CardSlotBoardModel>,
    active_view: Res<ActiveView>,
    hud_state: Res<DebugHudState>,
    game_scene_query: Query<Entity, With<GameSceneRoot>>,
    drawing_query: Query<(Entity, &DebugDrawing)>,
) {
    if !hud_state.is_debug_drawing_visible() || *active_view != ActiveView::GameScene {
        for (entity, _) in &drawing_query {
            despawn_debug_drawing(&mut commands, entity);
        }
        return;
    }

    for (entity, drawing) in &drawing_query {
        let stale = match model.request_for(drawing.target) {
            Some(request) => request.generation != drawing.generation,
            None => true,
        };
        if stale {
            despawn_debug_drawing(&mut commands, entity);
        }
    }

    let Ok(game_scene_entity) = game_scene_query.single() else {
        return;
    };

    for request in model.requests() {
        let already_drawn = drawing_query.iter().any(|(_, drawing)| {
            drawing.target == request.target && drawing.generation == request.generation
        });
        if already_drawn {
            continue;
        }

        let rect = request
            .target
            .runtime_rect(&slot_board)
            .unwrap_or(request.rect);
        let drawing = commands
            .spawn((
                Name::new(format!("Debug Drawing: {}", request.label)),
                DebugDrawing::new(request.target, request.generation),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(rect.left),
                    top: Val::Px(rect.top),
                    width: Val::Px(rect.width),
                    height: Val::Px(rect.height),
                    border: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                BorderColor::all(request.color.border_color()),
                BackgroundColor(request.color.fill_color()),
                GlobalZIndex(DEBUG_DRAWING_Z_INDEX),
                Visibility::Visible,
            ))
            .with_children(|parent| {
                if !request.label.is_empty() {
                    parent.spawn((
                        Text::new(request.label.clone()),
                        TextFont {
                            font_size: 14.0,
                            ..Default::default()
                        },
                        TextColor(request.color.border_color()),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(4.0),
                            top: Val::Px(4.0),
                            ..Default::default()
                        },
                    ));
                }
            })
            .id();
        commands.entity(game_scene_entity).add_child(drawing);
    }
}

/// HUMAN: Shows debug drawing by itself for visual inspection.
/// AI: Suppress GameScene content while keeping the UI camera path available for debug annotations.
pub(crate) fn debug_draw_solo_update_system(
    active_view: Res<ActiveView>,
    hud_state: Res<DebugHudState>,
    mut commands: Commands,
    hud: Option<Res<Hud>>,
    app_scene_query: Query<Entity, With<AppSceneRoot>>,
    game_scene_root_query: Query<Entity, With<GameSceneRoot>>,
    solo_overlay_query: Query<Entity, With<DebugDrawSoloOverlay>>,
    children_query: Query<&Children>,
    debug_drawing_query: Query<(), With<DebugDrawing>>,
    mut visibility_query: Query<
        (
            Entity,
            &mut Visibility,
            Option<&DebugDrawSoloHiddenVisibility>,
        ),
        Without<DebugHudText>,
    >,
    mut camera_query: Query<(&mut Camera, Option<&IsDefaultUiCamera>), With<GameSceneEntity>>,
) {
    let is_game_scene = *active_view == ActiveView::GameScene;
    let is_solo = is_game_scene && hud_state.is_debug_drawing_solo();

    sync_debug_draw_solo_overlay(
        is_solo,
        &mut commands,
        hud.as_ref().map(|hud| hud.0),
        &app_scene_query,
        &game_scene_root_query,
        &solo_overlay_query,
    );

    let solo_hidden_entities = if is_solo {
        collect_debug_draw_solo_hidden_entities(
            &game_scene_root_query,
            &children_query,
            &debug_drawing_query,
        )
    } else {
        HashSet::new()
    };

    for (entity, mut visibility, solo_hidden_visibility) in &mut visibility_query {
        if solo_hidden_entities.contains(&entity) {
            if solo_hidden_visibility.is_none() {
                commands
                    .entity(entity)
                    .insert(DebugDrawSoloHiddenVisibility(*visibility));
            }
            *visibility = Visibility::Hidden;
        } else if let Some(solo_hidden_visibility) = solo_hidden_visibility {
            *visibility = solo_hidden_visibility.0;
            commands
                .entity(entity)
                .remove::<DebugDrawSoloHiddenVisibility>();
        }
    }

    if !is_game_scene {
        return;
    }

    for (mut camera, ui_camera) in &mut camera_query {
        camera.is_active = ui_camera.is_some() || !is_solo;
    }
}

fn sync_debug_draw_solo_overlay(
    is_solo: bool,
    commands: &mut Commands,
    hud_parent: Option<Entity>,
    app_scene_query: &Query<Entity, With<AppSceneRoot>>,
    game_scene_root_query: &Query<Entity, With<GameSceneRoot>>,
    solo_overlay_query: &Query<Entity, With<DebugDrawSoloOverlay>>,
) {
    let existing_overlay = solo_overlay_query.single().ok();
    if !is_solo {
        if let Some(entity) = existing_overlay {
            commands.entity(entity).despawn();
        }
        return;
    }

    if existing_overlay.is_some() {
        return;
    }

    let Some(parent) = hud_parent
        .or_else(|| app_scene_query.single().ok())
        .or_else(|| game_scene_root_query.single().ok())
    else {
        return;
    };

    let overlay = commands
        .spawn((
            Name::new("Debug Draw Solo Overlay"),
            DebugDrawSoloOverlay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
            GlobalZIndex(DEBUG_DRAW_SOLO_OVERLAY_Z_INDEX),
            Visibility::Visible,
        ))
        .id();
    commands.entity(parent).add_child(overlay);
}

fn collect_debug_draw_solo_hidden_entities(
    game_scene_roots: &Query<Entity, With<GameSceneRoot>>,
    children_query: &Query<&Children>,
    debug_drawing_query: &Query<(), With<DebugDrawing>>,
) -> HashSet<Entity> {
    let mut hidden_entities = HashSet::new();
    for root in game_scene_roots.iter() {
        if let Ok(children) = children_query.get(root) {
            for child in children.iter() {
                collect_debug_draw_solo_hidden_entity(
                    child,
                    false,
                    &mut hidden_entities,
                    children_query,
                    debug_drawing_query,
                );
            }
        }
    }
    hidden_entities
}

fn collect_debug_draw_solo_hidden_entity(
    entity: Entity,
    is_debug_drawing_descendant: bool,
    hidden_entities: &mut HashSet<Entity>,
    children_query: &Query<&Children>,
    debug_drawing_query: &Query<(), With<DebugDrawing>>,
) {
    let is_debug_drawing_subtree =
        is_debug_drawing_descendant || debug_drawing_query.contains(entity);
    if is_debug_drawing_subtree {
        return;
    }

    hidden_entities.insert(entity);
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            collect_debug_draw_solo_hidden_entity(
                child,
                is_debug_drawing_subtree,
                hidden_entities,
                children_query,
                debug_drawing_query,
            );
        }
    }
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub(crate) struct DebugDrawSoloHiddenVisibility(Visibility);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub(crate) struct DebugDrawSoloOverlay;

fn despawn_debug_drawing(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn_children().despawn();
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/debug_drawing_update_system_tests.rs"]
mod debug_drawing_update_system_tests;
