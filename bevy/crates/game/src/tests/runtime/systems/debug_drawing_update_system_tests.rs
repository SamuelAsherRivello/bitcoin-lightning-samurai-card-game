use super::*;

#[test]
fn reference_debug_drawings_spawn_under_game_view() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<DebugDrawingModel>()
        .init_resource::<CardSlotBoardModel>()
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
    assert_eq!(drawings.len(), 29);
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
        .init_resource::<CardSlotBoardModel>()
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
        .init_resource::<CardSlotBoardModel>()
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
        .init_resource::<CardSlotBoardModel>()
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
fn deck_builder_scene_hides_game_scene_debug_drawings() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<DebugDrawingModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<ActiveView>()
        .init_resource::<DebugHudState>()
        .add_systems(Update, debug_drawing_update_system);
    app.world_mut().spawn(GameViewRoot);
    app.world_mut()
        .resource_mut::<DebugHudState>()
        .is_debug_drawing_visible = true;
    app.update();

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DeckBuilderScene;
    app.update();

    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DebugDrawing>>()
            .iter(app.world())
            .count(),
        0
    );
}
