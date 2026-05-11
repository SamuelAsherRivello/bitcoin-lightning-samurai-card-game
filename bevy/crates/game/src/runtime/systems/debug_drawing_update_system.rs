use bevy::prelude::*;
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::hot;

use crate::runtime::components::{DebugDrawing, GameViewRoot};
use crate::runtime::resources::{ActiveView, DebugDrawingModel, DebugHudState};

#[cfg_attr(feature = "desktop-hot-reload", hot)]
/// HUMAN: Synchronizes requested debug drawings with visible game-scene annotations.
/// AI: Keep annotations removable and aspect-ratio-safe by parenting them to existing HUD areas.
pub fn debug_drawing_update_system(
    mut commands: Commands,
    model: Res<DebugDrawingModel>,
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

        let drawing = commands
            .spawn((
                Name::new(format!("Debug Drawing: {}", request.label)),
                DebugDrawing::new(request.target, request.generation),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(request.rect.left),
                    top: Val::Px(request.rect.top),
                    width: Val::Px(request.rect.width),
                    height: Val::Px(request.rect.height),
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
                            left: Val::Px(-10.0),
                            top: Val::Px(-18.0),
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
mod tests {
    use super::*;

    #[test]
    fn reference_debug_drawings_spawn_under_game_view() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugDrawingModel>()
            .init_resource::<ActiveView>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, debug_drawing_update_system);
        let game_view = app.world_mut().spawn(GameViewRoot).id();
        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = true;

        app.update();

        let drawings: Vec<Entity> = app
            .world_mut()
            .query_filtered::<Entity, With<DebugDrawing>>()
            .iter(app.world())
            .collect();
        assert_eq!(drawings.len(), 30);
        assert!(
            app.world()
                .entity(game_view)
                .get::<Children>()
                .unwrap()
                .contains(&drawings[0])
        );
    }

    #[test]
    fn removing_all_requests_removes_debug_drawings() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugDrawingModel>()
            .init_resource::<ActiveView>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, debug_drawing_update_system);
        app.world_mut().spawn(GameViewRoot);
        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = true;
        app.update();

        let targets: Vec<_> = app
            .world()
            .resource::<DebugDrawingModel>()
            .requests()
            .iter()
            .map(|request| request.target)
            .collect();
        for target in targets {
            app.world_mut()
                .resource_mut::<DebugDrawingModel>()
                .remove(target);
        }
        app.update();

        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<DebugDrawing>>()
                .iter(app.world())
                .count(),
            0
        );
    }

    #[test]
    fn hidden_debug_drawing_state_despawns_drawings() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugDrawingModel>()
            .init_resource::<ActiveView>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, debug_drawing_update_system);
        app.world_mut().spawn(GameViewRoot);
        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = true;
        app.update();

        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = false;
        app.update();

        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<DebugDrawing>>()
                .iter(app.world())
                .count(),
            0
        );
    }

    #[test]
    fn hidden_debug_drawing_state_despawns_label_children() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugDrawingModel>()
            .init_resource::<ActiveView>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, debug_drawing_update_system);
        app.world_mut().spawn(GameViewRoot);
        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = true;
        app.update();

        assert!(
            app.world_mut()
                .query::<&Text>()
                .iter(app.world())
                .any(|text| text.0.contains("game area"))
        );

        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = false;
        app.update();

        assert_eq!(
            app.world_mut().query::<&Text>().iter(app.world()).count(),
            0
        );
    }

    #[test]
    fn card_browser_view_hides_game_scene_debug_drawings() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<DebugDrawingModel>()
            .init_resource::<ActiveView>()
            .init_resource::<DebugHudState>()
            .add_systems(Update, debug_drawing_update_system);
        app.world_mut().spawn(GameViewRoot);
        app.world_mut()
            .resource_mut::<DebugHudState>()
            .is_debug_drawing_visible = true;
        app.update();

        *app.world_mut().resource_mut::<ActiveView>() = ActiveView::CardBrowserView;
        app.update();

        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<DebugDrawing>>()
                .iter(app.world())
                .count(),
            0
        );
    }
}
