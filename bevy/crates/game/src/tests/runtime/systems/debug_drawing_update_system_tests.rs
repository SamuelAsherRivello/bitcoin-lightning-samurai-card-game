use super::*;
use crate::runtime::resources::{DebugDrawMode, DebugDrawingTarget};

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
        .debug_draw_mode = DebugDrawMode::On;

    app.update();

    let drawings: Vec<Entity> = app
        .world_mut()
        .query_filtered::<Entity, With<DebugDrawing>>()
        .iter(app.world())
        .collect();
    assert_eq!(drawings.len(), 11);
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
        .debug_draw_mode = DebugDrawMode::On;
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
        .debug_draw_mode = DebugDrawMode::On;
    app.update();

    app.world_mut()
        .resource_mut::<DebugHudState>()
        .debug_draw_mode = DebugDrawMode::Off;
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
        .debug_draw_mode = DebugDrawMode::On;
    app.update();

    assert!(
        app.world_mut()
            .query::<&Text>()
            .iter(app.world())
            .any(|text| text.0.contains("game area"))
    );

    app.world_mut()
        .resource_mut::<DebugHudState>()
        .debug_draw_mode = DebugDrawMode::Off;
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
        .debug_draw_mode = DebugDrawMode::On;
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

#[test]
fn solo_debug_drawing_hides_game_view_content_but_keeps_ui_camera_active() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveView>()
        .init_resource::<DebugHudState>()
        .add_systems(Update, debug_draw_solo_update_system);
    let app_scene = app.world_mut().spawn(AppSceneRoot).id();
    let game_view = app.world_mut().spawn(GameViewRoot).id();
    app.world_mut().entity_mut(app_scene).add_child(game_view);
    let game_view_ui = app
        .world_mut()
        .spawn((GameViewEntity, Node::default(), Visibility::Visible))
        .id();
    let untagged_render_child = app.world_mut().spawn(Visibility::Visible).id();
    let card_render_root = app
        .world_mut()
        .spawn((Name::new("Solo Test Card"), Visibility::Visible))
        .id();
    let card_render_mesh = app
        .world_mut()
        .spawn((Name::new("Solo Test Card Mesh"), Visibility::Visible))
        .id();
    app.world_mut()
        .entity_mut(game_view)
        .add_child(game_view_ui);
    app.world_mut()
        .entity_mut(game_view)
        .add_child(card_render_root);
    app.world_mut()
        .entity_mut(game_view_ui)
        .add_child(untagged_render_child);
    app.world_mut()
        .entity_mut(card_render_root)
        .add_child(card_render_mesh);
    let debug_drawing = app
        .world_mut()
        .spawn((
            DebugDrawing::new(DebugDrawingTarget::GameArea, 1),
            GlobalZIndex(DEBUG_DRAWING_Z_INDEX),
            Visibility::Visible,
        ))
        .id();
    let debug_drawing_label = app.world_mut().spawn(Visibility::Visible).id();
    app.world_mut()
        .entity_mut(game_view)
        .add_child(debug_drawing);
    app.world_mut()
        .entity_mut(debug_drawing)
        .add_child(debug_drawing_label);
    let debug_hud = app
        .world_mut()
        .spawn((
            DebugHudText,
            GlobalZIndex(DEBUG_DRAW_SOLO_OVERLAY_Z_INDEX + 1),
            Visibility::Visible,
        ))
        .id();
    let scene_camera = app
        .world_mut()
        .spawn((GameViewEntity, Camera::default()))
        .id();
    let ui_camera = app
        .world_mut()
        .spawn((GameViewEntity, Camera::default(), IsDefaultUiCamera))
        .id();

    app.world_mut()
        .resource_mut::<DebugHudState>()
        .debug_draw_mode = DebugDrawMode::OnSolo;
    app.update();

    assert_eq!(
        app.world().get::<Visibility>(game_view_ui),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(debug_hud),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(untagged_render_child),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(card_render_root),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(card_render_mesh),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().get::<Visibility>(debug_drawing),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(debug_drawing_label),
        Some(&Visibility::Visible)
    );
    let solo_overlay = app
        .world_mut()
        .query_filtered::<Entity, With<DebugDrawSoloOverlay>>()
        .single(app.world())
        .unwrap();
    assert!(
        app.world()
            .entity(app_scene)
            .get::<Children>()
            .unwrap()
            .contains(&solo_overlay)
    );
    assert_eq!(
        app.world().get::<BackgroundColor>(solo_overlay).unwrap().0,
        Color::BLACK
    );
    assert_eq!(
        *app.world().get::<GlobalZIndex>(solo_overlay).unwrap(),
        GlobalZIndex(DEBUG_DRAW_SOLO_OVERLAY_Z_INDEX)
    );
    assert_eq!(
        *app.world().get::<GlobalZIndex>(debug_drawing).unwrap(),
        GlobalZIndex(DEBUG_DRAWING_Z_INDEX)
    );
    assert!(
        app.world().get::<GlobalZIndex>(debug_hud).unwrap().0
            > app.world().get::<GlobalZIndex>(solo_overlay).unwrap().0
    );
    assert!(!app.world().get::<Camera>(scene_camera).unwrap().is_active);
    assert!(app.world().get::<Camera>(ui_camera).unwrap().is_active);

    app.world_mut()
        .resource_mut::<DebugHudState>()
        .debug_draw_mode = DebugDrawMode::Off;
    app.update();

    assert_eq!(
        app.world().get::<Visibility>(game_view_ui),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(debug_hud),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(untagged_render_child),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(card_render_root),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(card_render_mesh),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(debug_drawing),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(debug_drawing_label),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DebugDrawSoloOverlay>>()
            .iter(app.world())
            .count(),
        0
    );
    assert!(app.world().get::<Camera>(scene_camera).unwrap().is_active);
    assert!(app.world().get::<Camera>(ui_camera).unwrap().is_active);
}
