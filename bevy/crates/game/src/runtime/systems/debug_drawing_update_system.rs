use bevy::prelude::*;
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::hot;

use crate::runtime::components::{DebugDrawing, GameViewRoot};
use crate::runtime::resources::{ActiveView, CardSlotBoardModel, DebugDrawingModel, DebugHudState};

#[cfg_attr(feature = "desktop-hot-reload", hot)]
/// HUMAN: Synchronizes requested debug drawings with visible game-scene annotations.
/// AI: Keep annotations removable and aspect-ratio-safe by parenting them to existing HUD areas.
pub fn debug_drawing_update_system(
    mut commands: Commands,
    model: Res<DebugDrawingModel>,
    slot_board: Res<CardSlotBoardModel>,
    active_view: Res<ActiveView>,
    hud_state: Res<DebugHudState>,
    game_view_query: Query<Entity, With<GameViewRoot>>,
    drawing_query: Query<(Entity, &DebugDrawing)>,
) {
    if !hud_state.is_debug_drawing_visible || *active_view != ActiveView::GameView {
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

    let Ok(game_view_entity) = game_view_query.single() else {
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
                GlobalZIndex(90),
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
        commands.entity(game_view_entity).add_child(drawing);
    }
}

fn despawn_debug_drawing(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn_children().despawn();
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/debug_drawing_update_system_tests.rs"]
mod debug_drawing_update_system_tests;
