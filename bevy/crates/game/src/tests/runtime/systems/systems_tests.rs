use super::*;
use crate::runtime::resources::{
    CardGestureModel, CardGestureState, CardSlotBoardModel, CpuBrainMoveModel,
    DECK_SCREEN_CARD_COUNT, DebugDrawMode, DeckScreenMode,
};
use bevy::ecs::system::RunSystemOnce;
use bevy::text::Font;
use bevy_persistent::prelude::StorageFormat;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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
    assert!((left - right).abs() < 0.000_1, "{left} != {right}");
}

fn assert_vec2_close(left: Vec2, right: Vec2) {
    assert!(
        left.distance(right) < 0.000_1,
        "left={left:?} right={right:?}"
    );
}

fn apparent_scale_at_z(scale: f32, z: f32) -> f32 {
    scale / game_scene_world_units_per_game_scene_pixel(z)
}

fn active_child_scene_root_count(app: &mut App) -> usize {
    let mut scene_query = app.world_mut().query_filtered::<Entity, Or<(
        With<GameSceneRoot>,
        With<DeckSceneRoot>,
        With<DebugSceneRoot>,
    )>>();
    scene_query.iter(app.world()).count()
}

fn assert_debug_hud_targets_active_ui_camera(app: &mut App, active_view: ActiveView) {
    let target_camera = {
        let hud = {
            let mut hud_query = app
                .world_mut()
                .query_filtered::<Entity, With<DebugHudText>>();
            hud_query
                .single(app.world())
                .expect("DebugHUD should exist")
        };
        let root = {
            let mut child_of_query = app.world_mut().query::<&ChildOf>();
            let mut root = hud;
            while let Ok(child_of) = child_of_query.get(app.world(), root) {
                root = child_of.parent();
            }
            root
        };
        let mut hud_query = app
            .world_mut()
            .query_filtered::<&UiTargetCamera, With<Node>>();
        hud_query
            .get(app.world(), root)
            .expect("DebugHUD root should target a UI camera")
            .0
    };
    let camera_entity = app.world().entity(target_camera);
    let camera = camera_entity
        .get::<Camera>()
        .expect("DebugHUD target should have a Camera");
    assert!(camera.is_active);

    match active_view {
        ActiveView::GameScene => assert!(camera_entity.get::<GameSceneEntity>().is_some()),
        ActiveView::DeckScene => {
            assert!(camera_entity.get::<DeckSceneEntity>().is_some())
        }
        ActiveView::DebugScene => {
            assert!(camera_entity.get::<DebugSceneEntity>().is_some())
        }
    }
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

fn test_persistent_path(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-local-storage")
        .join(format!("{name}-{timestamp}"));
    std::fs::create_dir_all(&directory).expect("test persistent directory should be created");
    directory.join(format!("{name}.json"))
}

fn test_debug_hud_input_store(name: &str) -> Persistent<DebugHudInputStore> {
    Persistent::<DebugHudInputStore>::builder()
        .name(name)
        .format(StorageFormat::JsonPretty)
        .path(test_persistent_path(name))
        .default(DebugHudInputStore::default())
        .build()
        .expect("test debug hud input store should be created")
}

fn test_window_placement_store(name: &str) -> Persistent<WindowPlacementStore> {
    Persistent::<WindowPlacementStore>::builder()
        .name(name)
        .format(StorageFormat::JsonPretty)
        .path(test_persistent_path(name))
        .default(WindowPlacementStore::default())
        .build()
        .expect("test window placement store should be created")
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

    assert!(hud_text.0.starts_with("Screen: GameScreen\nFrame: 0"));
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
fn app_scene_owns_debug_hud_without_deck_entities() {
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
fn deck_scene_owns_camera_light_and_deck_screen_ui() {
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
        .add_systems(Startup, setup_deck_scene);

    app.update();

    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Transform, (With<PrimaryViewCamera>, With<DeckSceneEntity>)>();
    let camera_transform = camera_query.single(app.world()).unwrap();
    assert_eq!(
        camera_transform.translation.z,
        DECK_SCENE_CAMERA_DISTANCE_FROM_ORIGIN
    );

    let mut light_query = app
        .world_mut()
        .query_filtered::<Entity, (With<DirectionalLight>, With<DeckSceneEntity>)>();
    assert_eq!(light_query.iter(app.world()).count(), 1);

    let (ui_camera_entity, ui_camera, egui_context) = app
        .world_mut()
        .query_filtered::<(Entity, &Camera, Option<&PrimaryEguiContext>), (
            With<Camera2d>,
            With<DeckSceneEntity>,
        )>()
        .single(app.world())
        .unwrap();
    assert_eq!(ui_camera.order, 1);
    assert!(matches!(ui_camera.clear_color, ClearColorConfig::None));
    assert!(egui_context.is_some());

    let mut root_query = app
        .world_mut()
        .query_filtered::<(Entity, &Node, &UiTargetCamera), With<DeckSceneRoot>>();
    let (deck_root, root_node, root_target_camera) = root_query.single(app.world()).unwrap();
    assert_eq!(root_node.width, Val::Percent(100.0));
    assert_eq!(root_node.height, Val::Percent(100.0));
    assert_eq!(*root_target_camera, UiTargetCamera(ui_camera_entity));

    let mut top_nav_query = app
        .world_mut()
        .query_filtered::<Entity, With<TopNavigationRoot>>();
    let top_nav_entity = top_nav_query.single(app.world()).unwrap();
    assert_eq!(
        app.world()
            .get::<ChildOf>(top_nav_entity)
            .map(ChildOf::parent),
        Some(deck_root)
    );

    let mut content_query = app
        .world_mut()
        .query_filtered::<Entity, With<DeckScreenContentRoot>>();
    let content_entity = content_query.single(app.world()).unwrap();
    assert_eq!(
        app.world().get::<UiTargetCamera>(content_entity),
        Some(&UiTargetCamera(ui_camera_entity))
    );

    let mut deck_tile_query = app
        .world_mut()
        .query_filtered::<Entity, With<DeckScreenDeckTileButton>>();
    assert_eq!(deck_tile_query.iter(app.world()).count(), 1);

    let mut text_query = app.world_mut().query::<&Text>();
    let labels: Vec<String> = text_query
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect();
    assert!(labels.iter().any(|text| text == DECK_SCREEN_DECK_NAME));
    assert!(labels.iter().any(|text| text == "New Deck"));
}

#[test]
fn debug_scene_owns_camera_light_and_card() {
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
        .add_systems(Startup, setup_debug_scene);

    app.update();

    let mut root_query = app
        .world_mut()
        .query_filtered::<Entity, With<DebugSceneRoot>>();
    assert_eq!(root_query.iter(app.world()).count(), 1);

    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Transform, (With<PrimaryViewCamera>, With<DebugSceneEntity>)>();
    let camera_transform = camera_query.single(app.world()).unwrap();
    assert_eq!(
        camera_transform.translation.z,
        DECK_SCENE_CAMERA_DISTANCE_FROM_ORIGIN
    );

    let mut light_query = app
        .world_mut()
        .query_filtered::<Entity, (With<DirectionalLight>, With<DebugSceneEntity>)>();
    assert_eq!(light_query.iter(app.world()).count(), 1);

    let mut card_query = app
        .world_mut()
        .query_filtered::<&Transform, (With<CardView>, With<DebugSceneEntity>)>();
    let card_transform = card_query.single(app.world()).unwrap();
    let expected_transform = debug_scene_card_transform(
        app.world().resource::<CardInspectionDefaults>(),
        Quat::IDENTITY,
    );
    assert_close(
        card_transform.translation.x,
        expected_transform.translation.x,
    );
    assert_close(
        card_transform.translation.y,
        expected_transform.translation.y,
    );
    assert_close(
        card_transform.translation.z,
        expected_transform.translation.z,
    );
    assert_close(card_transform.scale.x, expected_transform.scale.x);
    assert_close(card_transform.scale.y, expected_transform.scale.y);
    assert_close(card_transform.scale.z, expected_transform.scale.z);
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
        .add_systems(Startup, setup_debug_scene);

    app.update();

    let active_card = {
        let registry = app.world().resource::<CardModelRegistry>();
        let active_card_model = app.world().resource::<ActiveCardModel>();
        registry
            .active_card_model(active_card_model)
            .expect("active card model should exist")
            .clone()
    };

    let mut energy_query = app.world_mut().query::<(&Name, &PointView, &Visibility)>();
    let energy_views: Vec<(String, i32, Visibility)> = energy_query
        .iter(app.world())
        .filter_map(|(name, view, visibility)| {
            (view.model.point_type == PointType::CardEnergy).then_some((
                name.to_string(),
                view.model.value,
                *visibility,
            ))
        })
        .collect();
    assert_eq!(
        energy_views,
        vec![(
            "Card EnergyPointView Background".to_string(),
            active_card.cost.value,
            Visibility::Visible,
        )]
    );

    let mut power_query = app
        .world_mut()
        .query_filtered::<(&Name, &PointView, &Visibility), Without<GameLocation>>();
    let power_views: Vec<(String, i32, Visibility)> = power_query
        .iter(app.world())
        .filter_map(|(name, view, visibility)| {
            (view.model.point_type == PointType::CardPower).then_some((
                name.to_string(),
                view.model.value,
                *visibility,
            ))
        })
        .collect();
    assert_eq!(
        power_views,
        vec![(
            "Card PowerPointView Background".to_string(),
            active_card.base_power.value,
            Visibility::Visible,
        )]
    );

    let mut text_query = app.world_mut().query::<(
        &Name,
        &CardPointTextView,
        &Text2d,
        &TextFont,
        &TextColor,
        &RenderLayers,
        &Visibility,
    )>();
    let mut point_text: Vec<(
        String,
        PointType,
        String,
        f32,
        Color,
        RenderLayers,
        Visibility,
    )> = text_query
        .iter(app.world())
        .map(
            |(name, view, text, text_font, text_color, render_layers, visibility)| {
                (
                    name.to_string(),
                    view.point_type,
                    text.0.clone(),
                    text_font.font_size,
                    text_color.0,
                    render_layers.clone(),
                    *visibility,
                )
            },
        )
        .collect();
    point_text.sort_by(|left, right| left.0.cmp(&right.0));

    assert_eq!(
        point_text,
        vec![
            (
                "Card EnergyPointView Background Text".to_string(),
                PointType::CardEnergy,
                active_card.cost.display_text(),
                POINT_VIEW_BASE_TEXT_FONT_SIZE,
                PointModel::card_energy(active_card.cost.value).text_color(),
                RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER),
                Visibility::Visible,
            ),
            (
                "Card PowerPointView Background Text".to_string(),
                PointType::CardPower,
                active_card.base_power.display_text(),
                POINT_VIEW_BASE_TEXT_FONT_SIZE,
                PointModel::card_power(active_card.base_power.value).text_color(),
                RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER),
                Visibility::Visible,
            )
        ]
    );
}

#[test]
fn deck_scene_root_does_not_inherit_ui_layout_transform() {
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
        .add_systems(Startup, setup_deck_scene);

    app.update();

    let mut root_query = app
        .world_mut()
        .query_filtered::<(Option<&ChildOf>, &Transform), With<DeckSceneRoot>>();
    let (parent, transform) = {
        let (parent, transform) = root_query.single(app.world()).unwrap();
        (parent.map(ChildOf::parent), *transform)
    };
    let mut app_scene_query = app
        .world_mut()
        .query_filtered::<Entity, With<AppSceneRoot>>();
    let app_scene = app_scene_query.single(app.world()).unwrap();
    assert!(parent.is_none() || parent == Some(app_scene));
    assert_eq!(transform.translation, Vec3::ZERO);
    assert_eq!(transform.scale, Vec3::ONE);
}

#[test]
fn deck_screen_cards_select_as_enlarged_card_views_without_preview_modal() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<PrimaryCameraDefaults>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<DeckScreenModel>()
        .init_resource::<SelectedCardModalModel>()
        .init_resource::<TopNavigationModel>()
        .init_resource::<PlayerDeckCollectionModel>()
        .add_systems(Startup, setup_deck_scene)
        .add_systems(Update, deck_screen_update_system);

    app.update();

    let deck_tile = app
        .world_mut()
        .query_filtered::<Entity, With<DeckScreenDeckTileButton>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(deck_tile)
        .insert(Interaction::Pressed);
    app.update();

    assert_eq!(
        app.world().resource::<DeckScreenModel>().mode,
        DeckScreenMode::Editor
    );
    assert_eq!(
        deck_screen_deck_cards(app.world().resource::<PlayerDeckCollectionModel>()).len(),
        DECK_SCREEN_CARD_COUNT
    );
    let deck_card_view_count = app
        .world_mut()
        .query_filtered::<Entity, (With<CardView>, With<DeckScreenCardView>)>()
        .iter(app.world())
        .count();
    assert_eq!(deck_card_view_count, DECK_SCREEN_CARD_COUNT);
    let deck_card_view_metadata_count = app
        .world_mut()
        .query::<&DeckScreenCardView>()
        .iter(app.world())
        .filter(|view| view.zone == DeckEditableZoneModel::Deck)
        .count();
    assert_eq!(deck_card_view_metadata_count, DECK_SCREEN_CARD_COUNT);
    let deck_column_card_view_count = deck_screen_card_views_right_of(&mut app, 600.0);
    assert_eq!(deck_column_card_view_count, 0);
    assert_matching_deck_screen_grid_backdrops(&mut app);
    assert_eq!(
        deck_screen_library_cards(&deck_screen_deck_cards(
            app.world().resource::<PlayerDeckCollectionModel>()
        ))
        .len(),
        0
    );

    let first_deck_card = app
        .world_mut()
        .query_filtered::<(Entity, &DeckScreenCardTileButton), With<Button>>()
        .iter(app.world())
        .find_map(|(entity, tile)| (tile.zone == DeckEditableZoneModel::Deck).then_some(entity))
        .unwrap();
    app.world_mut()
        .entity_mut(first_deck_card)
        .insert(Interaction::Pressed);
    app.update();

    assert!(app.world().resource::<DeckScreenModel>().modal.is_none());

    {
        let mut collection = app.world_mut().resource_mut::<PlayerDeckCollectionModel>();
        assert!(move_deck_card_to_library(&mut collection, 0).is_some());
    }
    app.world_mut()
        .resource_mut::<DeckScreenModel>()
        .needs_rebuild = true;
    app.update();

    let deck_cards = deck_screen_deck_cards(app.world().resource::<PlayerDeckCollectionModel>());
    let library_cards = deck_screen_library_cards(&deck_cards);
    assert_eq!(deck_cards.len(), DECK_SCREEN_CARD_COUNT - 1);
    assert_eq!(library_cards.len(), 1);
    assert!(app.world().resource::<DeckScreenModel>().modal.is_none());
    let library_column_card_view_count = deck_screen_card_views_right_of(&mut app, 600.0);
    assert_eq!(library_column_card_view_count, 1);
    assert_matching_deck_screen_grid_backdrops(&mut app);
    let library_card_view_metadata_count = app
        .world_mut()
        .query::<&DeckScreenCardView>()
        .iter(app.world())
        .filter(|view| view.zone == DeckEditableZoneModel::Library)
        .count();
    assert_eq!(library_card_view_metadata_count, 1);

    let library_card = app
        .world_mut()
        .query_filtered::<(Entity, &DeckScreenCardTileButton), With<Button>>()
        .iter(app.world())
        .find_map(|(entity, tile)| (tile.zone == DeckEditableZoneModel::Library).then_some(entity))
        .unwrap();
    app.world_mut()
        .entity_mut(library_card)
        .insert(Interaction::Pressed);
    app.update();

    assert!(app.world().resource::<DeckScreenModel>().modal.is_none());
}

fn deck_screen_card_views_right_of(app: &mut App, x_threshold: f32) -> usize {
    app.world_mut()
        .query_filtered::<&Transform, (With<CardView>, With<DeckScreenCardView>)>()
        .iter(app.world())
        .map(|transform| game_scene_position_from_world_position(transform.translation).x)
        .filter(|x| *x > x_threshold)
        .count()
}

fn assert_matching_deck_screen_grid_backdrops(app: &mut App) {
    let mut backdrop_query = app
        .world_mut()
        .query::<(&DeckScreenGridBackdrop, &Transform, &Mesh3d)>();
    let backdrops: Vec<(DeckScreenGridBackdrop, Transform, Mesh3d)> = backdrop_query
        .iter(app.world())
        .map(|(backdrop, transform, mesh)| (*backdrop, *transform, mesh.clone()))
        .collect();

    assert_eq!(backdrops.len(), 10);

    for role in [
        DeckScreenGridBackdropRole::Fill,
        DeckScreenGridBackdropRole::Top,
        DeckScreenGridBackdropRole::Bottom,
        DeckScreenGridBackdropRole::Left,
        DeckScreenGridBackdropRole::Right,
    ] {
        let deck = backdrops
            .iter()
            .find(|(backdrop, _, _)| {
                backdrop.zone == DeckEditableZoneModel::Deck && backdrop.role == role
            })
            .unwrap();
        let library = backdrops
            .iter()
            .find(|(backdrop, _, _)| {
                backdrop.zone == DeckEditableZoneModel::Library && backdrop.role == role
            })
            .unwrap();

        let deck_position = game_scene_position_from_world_position(deck.1.translation);
        let library_position = game_scene_position_from_world_position(library.1.translation);
        assert_close(deck_position.y, library_position.y);

        let meshes = app.world().resource::<Assets<Mesh>>();
        let deck_mesh = meshes.get(&deck.2.0).unwrap();
        let library_mesh = meshes.get(&library.2.0).unwrap();
        let deck_size = mesh_bounds(deck_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        let library_size = mesh_bounds(library_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        assert_close(deck_size.0, library_size.0);
        assert_close(deck_size.1, library_size.1);
    }
}

#[test]
fn deck_camera_viewport_matches_centered_safe_area() {
    let wide_viewport = game_scene_safe_area_viewport(UVec2::new(1600, 800)).unwrap();
    assert_eq!(wide_viewport.physical_position, UVec2::new(160, 0));
    assert_eq!(wide_viewport.physical_size, UVec2::new(1280, 800));

    let tall_viewport = game_scene_safe_area_viewport(UVec2::new(1280, 1000)).unwrap();
    assert_eq!(tall_viewport.physical_position, UVec2::new(0, 100));
    assert_eq!(tall_viewport.physical_size, UVec2::new(1280, 800));

    let default_viewport =
        game_scene_safe_area_viewport(UVec2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
            .unwrap();
    assert_eq!(default_viewport.physical_position, UVec2::new(0, 64));
    assert_eq!(default_viewport.physical_size, UVec2::new(1024, 640));
}

#[test]
fn game_scene_3d_cameras_use_centered_safe_area_viewport() {
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
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, constrain_game_scene_3d_cameras_to_safe_area);
    app.world_mut().spawn((
        Window {
            resolution: WindowResolution::new(1280, 1536),
            ..Default::default()
        },
        PrimaryWindow,
    ));

    app.update();
    app.update();

    let expected_viewport = game_scene_safe_area_viewport(UVec2::new(1280, 1536)).unwrap();
    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Camera, (With<GameSceneEntity>, With<Camera3d>)>();
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
fn game_scene_card_point_text_camera_uses_centered_safe_area_viewport() {
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
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, constrain_game_scene_3d_cameras_to_safe_area);
    app.world_mut().spawn((
        Window {
            resolution: WindowResolution::new(1280, 1536),
            ..Default::default()
        },
        PrimaryWindow,
    ));

    app.update();
    app.update();

    let expected_viewport = game_scene_safe_area_viewport(UVec2::new(1280, 1536)).unwrap();
    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Camera, (With<GameSceneEntity>, With<CardPointTextCamera>)>();
    let camera = camera_query.single(app.world()).unwrap();
    let viewport = camera.viewport.as_ref().unwrap();

    assert_eq!(
        viewport.physical_position,
        expected_viewport.physical_position
    );
    assert_eq!(viewport.physical_size, expected_viewport.physical_size);
    assert_eq!(viewport.depth, expected_viewport.depth);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn game_scene_3d_cameras_use_default_viewport_in_native_fullscreen() {
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
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, constrain_game_scene_3d_cameras_to_safe_area);
    app.world_mut().spawn((
        Window {
            resolution: WindowResolution::new(2560, 1600),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            ..Default::default()
        },
        PrimaryWindow,
    ));

    app.update();

    let mut camera_query = app
        .world_mut()
        .query_filtered::<&Camera, (With<GameSceneEntity>, With<Camera3d>)>();
    let cameras: Vec<&Camera> = camera_query.iter(app.world()).collect();
    assert_eq!(cameras.len(), 2);
    for camera in cameras {
        assert!(camera.viewport.is_none());
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn game_scene_uses_default_viewport_during_fullscreen_transition() {
    let window = Window {
        resolution: WindowResolution::new(2560, 1600),
        mode: WindowMode::Windowed,
        ..Default::default()
    };
    let transition = FullscreenViewportTransitionState {
        frames_remaining: 1,
    };

    assert!(
        game_scene_safe_area_viewport_for_window_transition(&window, Some(&transition)).is_none()
    );
    assert!(
        game_scene_safe_area_viewport_for_window_transition(&window, None)
            .is_some_and(|viewport| viewport.physical_size == UVec2::new(2560, 1600))
    );
}

#[test]
fn game_scene_owns_camera_world_background_and_three_locations() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<Touches>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .add_systems(Startup, setup_game_scene);

    app.update();

    let mut camera_query = app
        .world_mut()
        .query_filtered::<Entity, (With<PrimaryViewCamera>, With<GameSceneEntity>)>();
    assert_eq!(camera_query.iter(app.world()).count(), 2);

    let mut light_query = app
        .world_mut()
        .query_filtered::<Entity, (With<DirectionalLight>, With<GameSceneEntity>)>();
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
    let expected_background_size = game_scene_world_background_size();
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
            (1, LocationRevealState::Unrevealed),
            (2, LocationRevealState::Unrevealed)
        ]
    );

    let slot_board = CardSlotBoardModel::default();
    let mut location_node_query = app.world_mut().query::<(&GameLocation, &Node, &Children)>();
    let mut location_layouts: Vec<(usize, Val, Val, Val, Val, Vec<Entity>)> = location_node_query
        .iter(app.world())
        .map(|(location, node, children)| {
            (
                location.index,
                node.left,
                node.top,
                node.width,
                node.height,
                children.iter().collect(),
            )
        })
        .collect();
    location_layouts.sort_by_key(|(index, ..)| *index);
    assert_eq!(location_layouts.len(), 3);

    for (location_index, left, top, width, height, children) in location_layouts {
        let area_rect = slot_board.location_area_rect(location_index).unwrap();
        let bundle_size = LocationViewBundle::scaled_size(area_rect);
        assert_eq!(
            left,
            Val::Px(area_rect.left + (area_rect.width - bundle_size.x) / 2.0)
        );
        assert_eq!(
            top,
            Val::Px(area_rect.top + (area_rect.height - bundle_size.y) / 2.0)
        );
        assert_eq!(width, Val::Px(bundle_size.x));
        assert_eq!(height, Val::Px(bundle_size.y));

        let background_name = format!("Game Location Background {location_index}");
        let border_name = format!("Game Location Border {location_index}");
        let mut background_node = None;
        let mut border_node = None;
        for child in children {
            let entity = app.world().entity(child);
            let Some(name) = entity.get::<Name>() else {
                continue;
            };
            if name.as_str() == background_name {
                background_node = entity.get::<Node>();
            }
            if name.as_str() == border_name {
                border_node = entity.get::<Node>();
            }
        }

        for node in [background_node.unwrap(), border_node.unwrap()] {
            assert_eq!(node.left, Val::Px(0.0));
            assert_eq!(node.top, Val::Px(0.0));
            assert_eq!(node.width, Val::Px(bundle_size.x));
            assert_eq!(node.height, Val::Px(bundle_size.y));
        }
    }

    let mut slot_target_query = app.world_mut().query::<&CardSlotGestureTarget>();
    let slot_targets: Vec<CardSlotGestureTarget> =
        slot_target_query.iter(app.world()).copied().collect();
    assert_eq!(slot_targets.len(), 24);
    assert_eq!(
        slot_targets
            .iter()
            .filter(|target| target.side == CardSlotSide::LocalPlayer)
            .count(),
        12
    );
    assert_eq!(
        slot_targets
            .iter()
            .filter(|target| target.side == CardSlotSide::Opponent)
            .count(),
        12
    );
    let mut drop_target_hint_query = app.world_mut().query::<&DropTargetHint>();
    let drop_target_hints: Vec<usize> = drop_target_hint_query
        .iter(app.world())
        .map(|hint| hint.location_index)
        .collect();
    assert_eq!(drop_target_hints, vec![0, 1, 2]);
    let mut point_view_query = app.world_mut().query::<(&Name, &PointView)>();
    let point_view_values: Vec<i32> = point_view_query
        .iter(app.world())
        .filter(|(_, point_view)| point_view.model.point_type == PointType::LocationPower)
        .map(|(_, point_view)| point_view.model.value)
        .collect();
    assert_eq!(point_view_values, vec![0, 0, 0, 0, 0, 0]);
    let mut location_power_query = app.world_mut().query::<(&PointLocationView, &PointView)>();
    let location_power_views: Vec<(usize, CardSlotSide, i32)> = location_power_query
        .iter(app.world())
        .map(|(location_power_view, point_view)| {
            (
                location_power_view.location_index,
                location_power_view.side,
                point_view.model.value,
            )
        })
        .collect();
    assert_eq!(location_power_views.len(), 6);

    let mut location_power_node_query = app
        .world_mut()
        .query::<(&PointLocationView, &Node, &Children)>();
    for (location_power_view, node, children) in location_power_node_query.iter(app.world()) {
        let area_rect = slot_board
            .location_area_rect(location_power_view.location_index)
            .unwrap();
        let bundle_size = LocationViewBundle::scaled_size(area_rect);
        let expected_left = (bundle_size.x - LOCATION_POINT_VIEW_WIDTH) / 2.0;

        assert_eq!(node.left, Val::Px(expected_left));
        assert_eq!(node.width, Val::Px(LOCATION_POINT_VIEW_WIDTH));
        assert_eq!(node.height, Val::Px(LOCATION_POINT_VIEW_HEIGHT));
        match location_power_view.side {
            CardSlotSide::Opponent => {
                assert_eq!(node.top, Val::Px(-LOCATION_POINT_VIEW_HALF_HEIGHT));
            }
            CardSlotSide::LocalPlayer => {
                assert_eq!(node.bottom, Val::Px(-LOCATION_POINT_VIEW_HALF_HEIGHT));
            }
        }

        let mut circle_node = None;
        let mut circle_color = None;
        let mut has_text = false;
        for child in children {
            let entity = app.world().entity(*child);
            let Some(name) = entity.get::<Name>() else {
                continue;
            };
            if name.as_str() == "PowerPointView Circle" {
                circle_node = entity.get::<Node>();
                circle_color = entity.get::<BackgroundColor>();
            }
            if name.as_str() == "PowerPointView Text" {
                has_text = true;
            }
        }

        let circle_node = circle_node.unwrap();
        assert_eq!(circle_node.width, Val::Px(LOCATION_POINT_VIEW_WIDTH));
        assert_eq!(circle_node.height, Val::Px(LOCATION_POINT_VIEW_HEIGHT));
        assert_eq!(
            circle_node.border_radius,
            BorderRadius::all(Val::Px(LOCATION_POINT_VIEW_HALF_HEIGHT))
        );
        assert_eq!(
            circle_color.unwrap().0,
            PointModel::location_power(0).background_color()
        );
        assert!(has_text);
    }

    let mut hand_query = app
        .world_mut()
        .query_filtered::<Entity, With<LocalPlayerHand>>();
    assert_eq!(hand_query.iter(app.world()).count(), 1);

    let mut round_ui_query = app.world_mut().query_filtered::<Entity, With<RoundUi>>();
    assert_eq!(round_ui_query.iter(app.world()).count(), 1);

    let mut end_round_button_query =
        app.world_mut()
            .query_filtered::<Entity, (With<RoundUi>, With<EndRoundButton>, With<Button>)>();
    assert_eq!(end_round_button_query.iter(app.world()).count(), 1);

    let mut preview_query = app.world_mut().query_filtered::<&Transform, (
        With<LocalPlayerHandCardPreview>,
        With<CardView>,
        With<GameSceneEntity>,
        Without<DeckSceneEntity>,
    )>();
    let mut preview_transforms: Vec<Transform> = preview_query.iter(app.world()).copied().collect();
    preview_transforms.sort_by(|left, right| {
        left.translation
            .x
            .partial_cmp(&right.translation.x)
            .unwrap()
    });
    assert_eq!(preview_transforms.len(), STARTING_HAND_CARD_COUNT);

    let deal_transform =
        local_player_hand_deal_transform(app.world().resource::<CardInspectionDefaults>());
    for transform in preview_transforms.iter() {
        assert_close(transform.translation.x, deal_transform.translation.x);
        assert_close(transform.translation.y, deal_transform.translation.y);
        assert_close(transform.translation.z, deal_transform.translation.z);
        assert_close(transform.scale.x, deal_transform.scale.x);
        assert_close(transform.scale.y, deal_transform.scale.y);
        assert_close(transform.scale.z, deal_transform.scale.z);
    }

    let mut card_states = CardStateModel::default();
    card_states.reset_to_size(STARTING_HAND_CARD_COUNT);
    let final_hand_transforms: Vec<Transform> = (0..STARTING_HAND_CARD_COUNT)
        .map(|index| {
            hand_source_transform(
                index,
                STARTING_HAND_CARD_COUNT,
                app.world().resource::<CardInspectionDefaults>(),
            )
        })
        .collect();
    assert!(
        preview_transforms
            .iter()
            .zip(final_hand_transforms.iter())
            .all(|(initial_transform, final_transform)| initial_transform
                .translation
                .distance(final_transform.translation)
                > 0.01)
    );

    app.insert_resource(CardGestureModel::default())
        .insert_resource(card_states)
        .add_systems(Update, card_gesture_animation_system);
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_millis(125));
    app.update();

    preview_transforms = preview_query.iter(app.world()).copied().collect();
    for transform in preview_transforms.iter() {
        assert!(transform.translation.y > deal_transform.translation.y);
        assert!(transform.translation.y < final_hand_transforms[0].translation.y);
    }

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(1));
    app.update();

    preview_transforms = preview_query.iter(app.world()).copied().collect();
    preview_transforms.sort_by(|left, right| {
        left.translation
            .x
            .partial_cmp(&right.translation.x)
            .unwrap()
    });
    assert_eq!(preview_transforms.len(), final_hand_transforms.len());

    let mut preview_layer_query = app.world_mut().query_filtered::<Entity, (
        With<CardParallaxLayer>,
        With<CardFaceLayer>,
        Without<DeckSceneEntity>,
    )>();
    assert_eq!(preview_layer_query.iter(app.world()).count(), 25);

    let mut preview_camera_query = app
        .world_mut()
        .query_filtered::<&Camera, (With<Camera3d>, With<GameSceneEntity>)>();
    let mut preview_camera_orders: Vec<isize> = preview_camera_query
        .iter(app.world())
        .map(|camera| camera.order)
        .collect();
    preview_camera_orders.sort();
    assert_eq!(preview_camera_orders, vec![0, 2]);

    let mut ui_camera_query = app.world_mut().query_filtered::<&Camera, (
        With<Camera2d>,
        With<GameSceneEntity>,
        With<IsDefaultUiCamera>,
    )>();
    let ui_camera = ui_camera_query.single(app.world()).unwrap();
    assert_eq!(ui_camera.order, 1);
    assert!(matches!(ui_camera.clear_color, ClearColorConfig::None));
}

#[test]
fn location_views_open_second_and_third_locations_when_round_changes() {
    let mut app = App::new();
    app.init_resource::<GameLocationModel>()
        .add_systems(Update, update_game_location_views_system);

    for location_index in 0..3 {
        app.world_mut().spawn(GameLocation::new(
            location_index,
            LocationRevealState::Unrevealed,
        ));
        app.world_mut()
            .spawn((GameLocationTitleText::new(location_index), Text::new("")));
        app.world_mut()
            .spawn((GameLocationBodyText::new(location_index), Text::new("")));
        app.world_mut().spawn((
            GameLocationBorder::new(location_index),
            BorderColor::all(Color::WHITE),
        ));
    }

    app.update();
    assert_location_reveal_states(
        &mut app,
        [
            LocationRevealState::Revealed,
            LocationRevealState::Unrevealed,
            LocationRevealState::Unrevealed,
        ],
    );

    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(2);
    app.update();
    assert_location_reveal_states(
        &mut app,
        [
            LocationRevealState::Revealed,
            LocationRevealState::Revealed,
            LocationRevealState::Unrevealed,
        ],
    );
    assert_location_text(&mut app, 1, "Bamboo Crossing", "-2 Power to each card here");

    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(3);
    app.update();
    assert_location_reveal_states(
        &mut app,
        [
            LocationRevealState::Revealed,
            LocationRevealState::Revealed,
            LocationRevealState::Revealed,
        ],
    );
    assert_location_text(&mut app, 2, "Shrine Ruins", "(No Ability)");
}

fn assert_location_reveal_states(app: &mut App, expected: [LocationRevealState; 3]) {
    let mut location_query = app.world_mut().query::<&GameLocation>();
    let mut locations: Vec<(usize, LocationRevealState)> = location_query
        .iter(app.world())
        .map(|location| (location.index, location.reveal_state))
        .collect();
    locations.sort_by_key(|(index, _)| *index);

    assert_eq!(
        locations,
        expected
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, LocationRevealState)>>()
    );
}

fn assert_location_text(
    app: &mut App,
    location_index: usize,
    expected_title: &str,
    expected_body: &str,
) {
    let mut title_query = app.world_mut().query::<(&GameLocationTitleText, &Text)>();
    let title = title_query
        .iter(app.world())
        .find_map(|(title, text)| {
            (title.location_index == location_index).then_some(text.0.clone())
        })
        .unwrap();
    assert_eq!(title, expected_title);

    let mut body_query = app.world_mut().query::<(&GameLocationBodyText, &Text)>();
    let body = body_query
        .iter(app.world())
        .find_map(|(body, text)| (body.location_index == location_index).then_some(text.0.clone()))
        .unwrap();
    assert_eq!(body, expected_body);
}

#[test]
fn location_power_points_update_from_populated_card_slots() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .add_systems(Update, update_location_power_points);
    let power_view = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::location_power(0)),
            PointLocationView::new(1, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("0"));
        })
        .id();
    let placed_card_ids: Vec<String> = app
        .world()
        .resource::<CardModelRegistry>()
        .card_models()
        .take(2)
        .map(|card_model| card_model.id.to_string())
        .collect();
    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(1, 0, placed_card_ids[0].clone()),
            Some(0)
        );
        assert_eq!(
            slots.place_next_local_with_card_id(1, 1, placed_card_ids[1].clone()),
            Some(1)
        );
    }
    let expected_total: i32 = app
        .world()
        .resource::<CardModelRegistry>()
        .card_models()
        .take(2)
        .map(|card_model| card_model.base_power.value)
        .sum();

    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(expected_total)
    );
    let text_child = app
        .world()
        .entity(power_view)
        .get::<Children>()
        .unwrap()
        .first()
        .copied()
        .unwrap();
    assert_eq!(
        app.world().entity(text_child).get::<Text>().unwrap().0,
        expected_total.to_string()
    );
}

#[test]
fn location_power_points_wait_for_current_round_reveal_state() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(OpponentMatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec!["a".to_string(); crate::runtime::resources::STARTING_DECK_CARD_COUNT],
        ))
        .add_systems(Update, update_location_power_points);
    let power_view = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::location_power(0)),
            PointLocationView::new(1, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("0"));
        })
        .id();
    let placed_card_id = app
        .world()
        .resource::<CardModelRegistry>()
        .card_models()
        .next()
        .unwrap()
        .id
        .to_string();
    let expected_total = app
        .world()
        .resource::<CardModelRegistry>()
        .card_model_for_id(&placed_card_id)
        .unwrap()
        .base_power;
    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(1, 0, placed_card_id),
            Some(0)
        );
    }
    app.world_mut()
        .resource_mut::<OpponentMatchModel>()
        .record_placement(MatchPlayerSide::Near, 1, 0);

    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(0)
    );

    let slots = app.world().resource::<CardSlotBoardModel>().clone();
    {
        let mut match_model = app.world_mut().resource_mut::<OpponentMatchModel>();
        match_model.start_next_current_round_reveal(&slots);
        match_model.complete_revealing_current_round_placements();
    }
    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(expected_total.value)
    );
}

#[test]
fn fortress_gate_location_power_total_uses_effective_card_power() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameLocationModel>()
        .add_systems(Update, update_location_power_points);
    let power_view = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::location_power(0)),
            PointLocationView::new(0, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("0"));
        })
        .id();

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(
                0,
                0,
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID,
            ),
            Some(0)
        );
    }

    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(3)
    );
    let text_child = app
        .world()
        .entity(power_view)
        .get::<Children>()
        .unwrap()
        .first()
        .copied()
        .unwrap();
    assert_eq!(app.world().entity(text_child).get::<Text>().unwrap().0, "3");
}

#[test]
fn bamboo_crossing_location_power_total_uses_effective_card_power() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameLocationModel>()
        .add_systems(Update, update_location_power_points);
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(2);
    let power_view = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::location_power(0)),
            PointLocationView::new(1, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("0"));
        })
        .id();

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(
                1,
                0,
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID,
            ),
            Some(0)
        );
    }

    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(1)
    );
    let text_child = app
        .world()
        .entity(power_view)
        .get::<Children>()
        .unwrap()
        .first()
        .copied()
        .unwrap();
    assert_eq!(app.world().entity(text_child).get::<Text>().unwrap().0, "1");
}

#[test]
fn market_square_location_power_doubles_side_total_at_four_cards() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameLocationModel>()
        .add_systems(Update, update_location_power_points);
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .reset_with_active_location_indices(&[5, 0, 1]);
    let power_view = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::location_power(0)),
            PointLocationView::new(0, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("0"));
        })
        .id();

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        for hand_index in 0..4 {
            assert_eq!(
                slots.place_next_local_with_card_id(
                    0,
                    hand_index,
                    crate::runtime::resources::KAGE_REN_CARD_MODEL_ID,
                ),
                Some(hand_index)
            );
        }
    }

    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(8)
    );
    let text_child = app
        .world()
        .entity(power_view)
        .get::<Children>()
        .unwrap()
        .first()
        .copied()
        .unwrap();
    assert_eq!(app.world().entity(text_child).get::<Text>().unwrap().0, "8");

    app.world_mut()
        .resource_mut::<CardSlotBoardModel>()
        .remove_local_card(3);
    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(3)
    );
    assert_eq!(app.world().entity(text_child).get::<Text>().unwrap().0, "3");
}

#[test]
fn location_power_update_ignores_non_location_point_types() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .add_systems(Update, update_location_power_points);

    let non_location_point = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::card_power(11)),
            PointLocationView::new(0, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("11"));
        })
        .id();

    let location_power_point = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::location_power(0)),
            PointLocationView::new(0, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("0"));
        })
        .id();

    let placed_card_id = app
        .world()
        .resource::<CardModelRegistry>()
        .card_models()
        .next()
        .unwrap()
        .id
        .to_string();
    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(0, 0, placed_card_id),
            Some(0)
        );
    }

    let expected_total = app
        .world()
        .resource::<CardModelRegistry>()
        .card_models()
        .next()
        .unwrap()
        .base_power
        .value;

    app.update();

    let non_location_view = app
        .world()
        .entity(non_location_point)
        .get::<PointView>()
        .unwrap();
    assert_eq!(non_location_view.model, PointModel::card_power(11));
    let non_location_text = app
        .world()
        .entity(non_location_point)
        .get::<Children>()
        .unwrap()
        .first()
        .copied()
        .unwrap();
    assert_eq!(
        app.world()
            .entity(non_location_text)
            .get::<Text>()
            .unwrap()
            .0,
        "11".to_string()
    );

    let location_power_view = app
        .world()
        .entity(location_power_point)
        .get::<PointView>()
        .unwrap();
    assert_eq!(
        location_power_view.model,
        PointModel::location_power(expected_total)
    );
}

#[test]
fn card_power_text_applies_and_removes_location_power_delta() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameLocationModel>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ]))
        .add_systems(Update, update_card_power_point_views_system);

    let power_text = app
        .world_mut()
        .spawn((
            CardPointTextView::new(PointType::CardPower),
            Text2d::new("1"),
        ))
        .id();
    let power_background = app
        .world_mut()
        .spawn(PointView::new(PointModel::card_power(1)))
        .id();
    app.world_mut()
        .entity_mut(power_background)
        .add_children(&[power_text]);
    let card = app
        .world_mut()
        .spawn((HandCardGestureTarget::new(0), CardGestureView))
        .id();
    app.world_mut()
        .entity_mut(card)
        .add_children(&[power_background]);

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(
                0,
                0,
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID
            ),
            Some(0)
        );
    }

    app.update();

    assert_eq!(
        app.world()
            .entity(power_background)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::card_power(3)
    );
    assert_eq!(
        app.world().entity(power_text).get::<Text2d>().unwrap().0,
        "3"
    );

    app.world_mut()
        .resource_mut::<CardSlotBoardModel>()
        .remove_local_card(0);
    app.update();

    assert_eq!(
        app.world()
            .entity(power_background)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::card_power(1)
    );
    assert_eq!(
        app.world().entity(power_text).get::<Text2d>().unwrap().0,
        "1"
    );
}

#[test]
fn card_power_text_applies_bamboo_crossing_power_delta() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameLocationModel>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID.to_string(),
        ]))
        .add_systems(Update, update_card_power_point_views_system);
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(2);

    let power_text = app
        .world_mut()
        .spawn((
            CardPointTextView::new(PointType::CardPower),
            Text2d::new("3"),
        ))
        .id();
    let power_background = app
        .world_mut()
        .spawn(PointView::new(PointModel::card_power(3)))
        .id();
    app.world_mut()
        .entity_mut(power_background)
        .add_children(&[power_text]);
    let card = app
        .world_mut()
        .spawn((HandCardGestureTarget::new(0), CardGestureView))
        .id();
    app.world_mut()
        .entity_mut(card)
        .add_children(&[power_background]);

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(
                1,
                0,
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID,
            ),
            Some(0)
        );
    }

    app.update();

    assert_eq!(
        app.world()
            .entity(power_background)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::card_power(1)
    );
    assert_eq!(
        app.world().entity(power_text).get::<Text2d>().unwrap().0,
        "1"
    );
}

#[test]
fn moved_card_power_text_uses_current_location_delta() {
    let mut app = App::new();
    app.init_resource::<CardSlotBoardModel>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameLocationModel>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID.to_string(),
        ]))
        .add_systems(Update, update_card_power_point_views_system);
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(2);

    let power_text = app
        .world_mut()
        .spawn((
            CardPointTextView::new(PointType::CardPower),
            Text2d::new("3"),
        ))
        .id();
    let power_background = app
        .world_mut()
        .spawn(PointView::new(PointModel::card_power(3)))
        .id();
    app.world_mut()
        .entity_mut(power_background)
        .add_children(&[power_text]);
    let card = app
        .world_mut()
        .spawn((HandCardGestureTarget::new(0), CardGestureView))
        .id();
    app.world_mut()
        .entity_mut(card)
        .add_children(&[power_background]);

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(
                0,
                0,
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID,
            ),
            Some(0)
        );
    }
    app.update();
    assert_eq!(
        app.world()
            .entity(power_background)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::card_power(5)
    );

    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert_eq!(
            slots.place_next_local_with_card_id(
                1,
                0,
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID,
            ),
            Some(0)
        );
    }
    app.update();

    assert_eq!(
        app.world()
            .entity(power_background)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::card_power(1)
    );
    assert_eq!(
        app.world().entity(power_text).get::<Text2d>().unwrap().0,
        "1"
    );
}

#[test]
fn cpu_versus_cpu_autoplay_reaches_winner_status_within_thirty_seconds() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CpuBrainModel>()
        .insert_resource(OpponentMatchModel::new(
            MatchModeModel::CpuVersusCpu,
            vec![
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::SISTER_HOTARU_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::SISTER_HOTARU_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::SISTER_HOTARU_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
            ],
        ))
        .add_systems(
            Update,
            (
                cpu_brain_update_system,
                staged_match_resolution_system.after(cpu_brain_update_system),
            ),
        );
    {
        let mut match_model = app.world_mut().resource_mut::<OpponentMatchModel>();
        match_model.near.draw(1);
        match_model.far.draw(1);
        match_model.near.energy_available = 1;
        match_model.far.energy_available = 1;
    }

    let mut elapsed = 0.0;
    while elapsed < 30.0 {
        elapsed += 0.5;
        {
            let match_model = app.world().resource::<OpponentMatchModel>();
            let round = match_model.round.round;
            let near_hand_count = match_model.near.hand.len();
            let far_hand_count = match_model.far.hand.len();
            let mut brain = app.world_mut().resource_mut::<CpuBrainModel>();
            brain.near_next_decision_seconds = 0.0;
            brain.far_next_decision_seconds = 0.0;
            brain.wait_for_hand_presentation(
                MatchPlayerSide::Near,
                round,
                near_hand_count,
                0.0,
                CPU_CARD_MOVE_SECONDS,
            );
            brain.wait_for_hand_presentation(
                MatchPlayerSide::Near,
                round,
                near_hand_count,
                CPU_CARD_MOVE_SECONDS,
                CPU_CARD_MOVE_SECONDS,
            );
            brain.wait_for_hand_presentation(
                MatchPlayerSide::Far,
                round,
                far_hand_count,
                0.0,
                CPU_CARD_MOVE_SECONDS,
            );
            brain.wait_for_hand_presentation(
                MatchPlayerSide::Far,
                round,
                far_hand_count,
                CPU_CARD_MOVE_SECONDS,
                CPU_CARD_MOVE_SECONDS,
            );
        }
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_millis(500));
        app.update();
        if app
            .world()
            .resource::<OpponentMatchModel>()
            .round
            .winner
            .is_some()
        {
            break;
        }
    }

    let match_model = app.world().resource::<OpponentMatchModel>();
    assert!(
        match_model.round.winner.is_some(),
        "CPU-vs-CPU did not finish within 30 seconds; status={} round={} near_ready={} far_ready={} near_hand={} far_hand={}",
        match_model.status_text(),
        match_model.round.round,
        match_model.near.ready_for_next,
        match_model.far.ready_for_next,
        match_model.near.hand.len(),
        match_model.far.hand.len()
    );
    assert!(
        match_model
            .status_text()
            .starts_with("Status: Winner is Player ")
    );
}

#[test]
fn cpu_gameplay_pauses_outside_game_scene_and_resumes_on_return() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CpuBrainModel>()
        .insert_resource(ActiveView::DeckScene)
        .insert_resource(OpponentMatchModel::new(
            MatchModeModel::CpuVersusCpu,
            vec![crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string()],
        ))
        .add_systems(
            Update,
            (
                cpu_brain_update_system,
                staged_match_resolution_system.after(cpu_brain_update_system),
            ),
        );
    {
        let mut match_model = app.world_mut().resource_mut::<OpponentMatchModel>();
        match_model.near.draw(1);
        match_model.near.energy_available = 1;
    }
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(1));
    app.update();

    assert_eq!(
        app.world().resource::<OpponentMatchModel>().near.hand.len(),
        1
    );
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        0
    );

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    {
        let match_model = app.world().resource::<OpponentMatchModel>();
        let round = match_model.round.round;
        let near_hand_count = match_model.near.hand.len();
        let mut brain = app.world_mut().resource_mut::<CpuBrainModel>();
        brain.near_next_decision_seconds = 0.0;
        brain.far_next_decision_seconds = 0.0;
        brain.wait_for_hand_presentation(MatchPlayerSide::Near, round, near_hand_count, 0.0, 0.0);
    }
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(1));
    app.update();

    assert_eq!(
        app.world().resource::<OpponentMatchModel>().near.hand.len(),
        0
    );
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        1
    );
}

#[test]
fn cpu_brain_plans_moves_without_populating_slots_until_both_players_are_ready() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CpuBrainModel>()
        .insert_resource(OpponentMatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec![crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string()],
        ))
        .add_systems(Update, cpu_brain_update_system);
    let (round, far_hand_count) = {
        let mut match_model = app.world_mut().resource_mut::<OpponentMatchModel>();
        match_model.far.draw(1);
        match_model.far.energy_available = 10;
        (match_model.round.round, match_model.far.hand.len())
    };
    {
        let mut brain = app.world_mut().resource_mut::<CpuBrainModel>();
        brain.far_next_decision_seconds = 0.0;
        brain.wait_for_hand_presentation(MatchPlayerSide::Far, round, far_hand_count, 0.0, 0.0);
    }
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(1));
    app.update();

    let match_model = app.world().resource::<OpponentMatchModel>();
    assert_eq!(match_model.far.hand.len(), 1);
    assert!(match_model.has_pending_cpu_placements());
    assert!(match_model.far.ready_for_next);
    assert!(!match_model.near.ready_for_next);
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        0
    );
}

#[test]
fn cpu_brain_waits_for_hand_card_to_settle_and_pause_before_planning() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CpuBrainModel>()
        .insert_resource(OpponentMatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec![crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string()],
        ))
        .add_systems(Update, cpu_brain_update_system);
    let (instance_id, card_id) = {
        let mut match_model = app.world_mut().resource_mut::<OpponentMatchModel>();
        match_model.far.draw(1);
        match_model.far.energy_available = 10;
        (
            match_model.far.hand_instance_id(0).unwrap(),
            match_model.far.hand[0].clone(),
        )
    };
    app.world_mut().spawn((
        CpuHandCardView::new(
            MatchPlayerSide::Far,
            instance_id,
            0,
            card_id,
            CardFace::Back,
        ),
        CpuPlacedCardAnimation::move_deck_to_hand_to_slot(
            Transform::default(),
            Transform::default(),
            Transform::default(),
            CardFace::Back,
        ),
    ));
    app.world_mut()
        .resource_mut::<CpuBrainModel>()
        .far_next_decision_seconds = 0.0;

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(2));
    app.update();
    assert!(
        !app.world()
            .resource::<OpponentMatchModel>()
            .has_pending_cpu_placements()
    );

    let animated_hand_cards: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, (With<CpuHandCardView>, With<CpuPlacedCardAnimation>)>()
        .iter(app.world())
        .collect();
    for entity in animated_hand_cards {
        app.world_mut()
            .entity_mut(entity)
            .remove::<CpuPlacedCardAnimation>();
    }
    let remaining_animated_hand_cards = app
        .world_mut()
        .query_filtered::<Entity, (With<CpuHandCardView>, With<CpuPlacedCardAnimation>)>()
        .iter(app.world())
        .count();
    assert_eq!(remaining_animated_hand_cards, 0);

    {
        let (round, far_hand_count) = {
            let match_model = app.world().resource::<OpponentMatchModel>();
            (match_model.round.round, match_model.far.hand.len())
        };
        let mut brain = app.world_mut().resource_mut::<CpuBrainModel>();
        assert!(!brain.wait_for_settled_hand_pause(
            MatchPlayerSide::Far,
            round,
            far_hand_count,
            true,
            0.6,
            CPU_HAND_SETTLED_PAUSE_SECONDS,
        ));
        brain.far_next_decision_seconds = 0.0;
    }
    app.update();
    let match_model = app.world().resource::<OpponentMatchModel>();
    assert!(
        match_model.has_pending_cpu_placements(),
        "far_ready={} far_hand={} far_energy={} pending={} populated={}",
        match_model.far.ready_for_next,
        match_model.far.hand.len(),
        match_model.far.energy_available,
        match_model.pending_cpu_placements.len(),
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count()
    );
}

#[test]
fn game_scene_card_sync_does_not_render_outside_game_scene() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(ActiveView::DeckScene)
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ]))
        .add_systems(Update, sync_game_scene_hand_card_entities_system);

    app.update();

    let mut card_query = app
        .world_mut()
        .query_filtered::<Entity, (With<CardView>, With<LocalPlayerHandCardPreview>)>();
    assert_eq!(card_query.iter(app.world()).count(), 0);
}

#[test]
fn hidden_game_scene_enforcement_hides_local_card_descendants() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(ActiveView::DeckScene)
        .add_systems(Update, enforce_hidden_game_scene_visibility_system);
    let card = app
        .world_mut()
        .spawn((GameSceneEntity, CardGestureView, Visibility::Visible))
        .id();
    let front_layer = app
        .world_mut()
        .spawn((
            CardFaceLayer::new(CardFace::Front),
            Visibility::Visible,
            GlobalTransform::default(),
        ))
        .id();
    app.world_mut().entity_mut(card).add_child(front_layer);

    app.update();

    assert_eq!(
        app.world().entity(card).get::<Visibility>(),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().entity(front_layer).get::<Visibility>(),
        Some(&Visibility::Hidden)
    );
}

#[test]
fn cpu_placed_card_animation_moves_deck_to_hand_then_slot_face_down() {
    let hand_transform = Transform {
        translation: Vec3::new(0.0, -3.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let slot_transform = Transform {
        translation: Vec3::new(4.0, 2.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let source_transform = Transform {
        translation: Vec3::new(-4.0, -2.0, 0.52),
        rotation: Quat::from_rotation_y(std::f32::consts::PI),
        scale: Vec3::splat(0.25),
    };
    let mut transform = source_transform;
    let mut animation = CpuPlacedCardAnimation::move_deck_to_hand_to_slot(
        source_transform,
        hand_transform,
        slot_transform,
        CardFace::Back,
    );

    let is_settled = advance_cpu_placed_card_animation(10.0, &mut transform, &mut animation);

    assert!(!is_settled);
    assert_eq!(
        animation.phase,
        crate::runtime::components::CpuPlacedCardAnimationPhase::MovingToSlot
    );
    assert_close(transform.translation.x, hand_transform.translation.x);
    assert_close(transform.translation.y, hand_transform.translation.y);
    assert!(
        transform
            .rotation
            .angle_between(hand_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI))
            < 0.000_1
    );

    let is_settled = advance_cpu_placed_card_animation(10.0, &mut transform, &mut animation);

    assert!(is_settled);
    assert_close(transform.translation.x, slot_transform.translation.x);
    assert_close(transform.translation.y, slot_transform.translation.y);
    assert!(
        transform
            .rotation
            .angle_between(slot_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI))
            < 0.000_1
    );
    assert_eq!(animation.current_face(), CardFace::Back);
}

#[test]
fn cpu_deck_to_hand_animation_lifts_without_scale_pulse() {
    let hand_transform = Transform {
        translation: Vec3::new(0.0, -3.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let slot_transform = Transform {
        translation: Vec3::new(4.0, 2.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let source_transform = Transform {
        translation: Vec3::new(-4.0, -2.0, 0.52),
        rotation: Quat::from_rotation_y(std::f32::consts::PI),
        scale: Vec3::splat(0.25),
    };
    let mut transform = source_transform;
    let mut animation = CpuPlacedCardAnimation::move_deck_to_hand_to_slot(
        source_transform,
        hand_transform,
        slot_transform,
        CardFace::Back,
    );

    assert!(!advance_cpu_placed_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.5,
        &mut transform,
        &mut animation
    ));

    assert_close(transform.translation.z, CPU_CARD_MOVING_FRONT_Z);
    assert_close(
        apparent_scale_at_z(transform.scale.x, transform.translation.z),
        apparent_scale_at_z(source_transform.scale.x, source_transform.translation.z),
    );
}

#[test]
fn cpu_placed_card_position_move_takes_half_second() {
    let hand_transform = Transform {
        translation: Vec3::new(0.0, -3.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let slot_transform = Transform {
        translation: Vec3::new(4.0, 2.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let mut transform = hand_transform;
    let mut animation =
        CpuPlacedCardAnimation::move_hand_to_slot(hand_transform, slot_transform, CardFace::Back);

    assert!(!advance_cpu_placed_card_animation(
        CPU_CARD_MOVE_SECONDS - 0.01,
        &mut transform,
        &mut animation
    ));
    assert!(transform.translation.distance(slot_transform.translation) > 0.0);
    let apparent_scale = apparent_scale_at_z(transform.scale.x, transform.translation.z);
    let target_apparent_scale =
        apparent_scale_at_z(slot_transform.scale.x, slot_transform.translation.z);
    assert!(apparent_scale > target_apparent_scale);
    assert!(apparent_scale < target_apparent_scale * CPU_CARD_MOVE_SCALE_MULTIPLIER);

    assert!(advance_cpu_placed_card_animation(
        0.01,
        &mut transform,
        &mut animation
    ));
    assert_close(transform.translation.x, slot_transform.translation.x);
    assert_close(transform.translation.y, slot_transform.translation.y);
    assert_close(transform.translation.z, slot_transform.translation.z);
    assert_close(transform.scale.x, slot_transform.scale.x);
}

#[test]
fn cpu_placed_card_scale_tweens_up_before_returning_to_slot_scale() {
    let hand_transform = Transform {
        translation: Vec3::new(0.0, -3.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let slot_transform = Transform {
        translation: Vec3::new(4.0, 2.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let mut transform = hand_transform;
    let mut animation =
        CpuPlacedCardAnimation::move_hand_to_slot(hand_transform, slot_transform, CardFace::Back);

    assert!(!advance_cpu_placed_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.1,
        &mut transform,
        &mut animation
    ));
    let apparent_scale = apparent_scale_at_z(transform.scale.x, transform.translation.z);
    let hand_apparent_scale =
        apparent_scale_at_z(hand_transform.scale.x, hand_transform.translation.z);
    assert!(apparent_scale > hand_apparent_scale);
    assert!(apparent_scale < hand_apparent_scale * CPU_CARD_MOVE_SCALE_MULTIPLIER);

    assert!(!advance_cpu_placed_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.4,
        &mut transform,
        &mut animation
    ));
    assert_close(
        apparent_scale_at_z(transform.scale.x, transform.translation.z),
        hand_apparent_scale * CPU_CARD_MOVE_SCALE_MULTIPLIER,
    );
}

#[test]
fn cpu_placed_card_scale_multiplier_uses_equal_half_move_timing() {
    assert_close(cpu_card_move_scale_multiplier(0.0), 1.0);
    assert_close(cpu_card_move_scale_multiplier(0.25), 1.25);
    assert_close(cpu_card_move_scale_multiplier(0.5), 1.5);
    assert_close(cpu_card_move_scale_multiplier(0.75), 1.25);
    assert_close(cpu_card_move_scale_multiplier(1.0), 1.0);
}

#[test]
fn cpu_placed_card_move_preserves_tweened_game_scene_path_while_lifted_forward() {
    let hand_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(260.0, 720.0), 0.3),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let slot_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(840.0, 260.0), 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let mut transform = hand_transform;
    let mut animation =
        CpuPlacedCardAnimation::move_hand_to_slot(hand_transform, slot_transform, CardFace::Back);

    assert!(!advance_cpu_placed_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.5,
        &mut transform,
        &mut animation
    ));

    assert_close(transform.translation.z, CPU_CARD_MOVING_FRONT_Z);
    assert_vec2_close(
        game_scene_position_from_world_position(transform.translation),
        game_scene_position_from_world_position(hand_transform.translation).lerp(
            game_scene_position_from_world_position(slot_transform.translation),
            0.875,
        ),
    );
}

#[test]
fn committed_cpu_placement_records_original_visible_hand_source() {
    let mut match_model = OpponentMatchModel::new(
        MatchModeModel::HumanVersusCpu,
        vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ],
    );
    match_model.far.draw(3);
    match_model.far.energy_available = 10;
    match_model.near.ready_for_next = true;
    match_model.far.ready_for_next = true;
    let instance_id = match_model.far.hand_instance_id(1).unwrap();
    let card_id = match_model.far.hand[1].clone();
    match_model.queue_cpu_placements(vec![CpuBrainMoveModel {
        instance_id,
        hand_index: 1,
        card_id,
        location_index: 0,
        slot_index: 0,
        energy_cost: 0,
        score: 0,
    }]);
    let mut slot_board = CardSlotBoardModel::default();

    assert_eq!(
        commit_pending_cpu_placements(&mut match_model, &mut slot_board),
        1
    );

    assert_eq!(
        match_model.take_cpu_placement_motion_source(MatchPlayerSide::Far, 0, 0),
        Some(CpuPlacementMotionSourceModel {
            owner: MatchPlayerSide::Far,
            location_index: 0,
            slot_index: 0,
            hand_index: 1,
            hand_count: 3,
        })
    );
}

#[test]
fn cpu_card_faces_keep_near_front_and_far_hidden_until_revealed() {
    assert_eq!(
        cpu_card_hand_visible_face(MatchPlayerSide::Near),
        CardFace::Front
    );
    assert_eq!(
        cpu_card_hand_visible_face(MatchPlayerSide::Far),
        CardFace::Back
    );
    assert_eq!(
        cpu_card_slot_visible_face(
            MatchPlayerSide::Near,
            PlacementVisibility::CurrentRoundHidden
        ),
        CardFace::Front
    );
    assert_eq!(
        cpu_card_slot_visible_face(
            MatchPlayerSide::Far,
            PlacementVisibility::CurrentRoundHidden
        ),
        CardFace::Back
    );
    assert_eq!(
        cpu_card_slot_visible_face(MatchPlayerSide::Far, PlacementVisibility::Revealed),
        CardFace::Front
    );
}

#[test]
fn far_cpu_hand_sits_above_the_game_scene() {
    let card_defaults = CardInspectionDefaults::default();
    let hand_transform = cpu_card_hand_transform(MatchPlayerSide::Far, 0, 1, &card_defaults);
    let (card_min, card_max) = game_scene_card_hitboxes_for_count(1)[0];
    let expected_position = game_scene_world_position_from_game_scene(
        Vec2::new((card_min.x + card_max.x) * 0.5, GAME_SCENE_FAR_HAND_Y),
        hand_transform.translation.z,
    );

    assert_close(hand_transform.translation.x, expected_position.x);
    assert_close(hand_transform.translation.y, expected_position.y);
    assert_close(hand_transform.translation.z, expected_position.z);

    let slot_transform = Transform {
        translation: Vec3::new(1.0, 2.0, 0.52),
        scale: Vec3::splat(0.25),
        ..Default::default()
    };
    let source_transform =
        cpu_card_move_source_hand_transform(MatchPlayerSide::Far, slot_transform);
    let expected_source_position = game_scene_world_position_from_game_scene(
        Vec2::new(GAME_SCENE_WIDTH * 0.5, GAME_SCENE_FAR_HAND_Y),
        slot_transform.translation.z,
    );

    assert_close(source_transform.translation.x, expected_source_position.x);
    assert_close(source_transform.translation.y, expected_source_position.y);
    assert_close(source_transform.translation.z, expected_source_position.z);
}

#[test]
fn cpu_brain_waits_for_hand_presentation_before_first_move() {
    let mut brain = CpuBrainModel::default();

    assert!(brain.wait_for_hand_presentation(
        MatchPlayerSide::Near,
        1,
        2,
        10.0,
        CPU_CARD_MOVE_SECONDS,
    ));
    assert!(brain.wait_for_hand_presentation(
        MatchPlayerSide::Near,
        1,
        2,
        CPU_CARD_MOVE_SECONDS * 0.5,
        CPU_CARD_MOVE_SECONDS,
    ));
    assert!(!brain.wait_for_hand_presentation(
        MatchPlayerSide::Near,
        1,
        2,
        CPU_CARD_MOVE_SECONDS * 0.5,
        CPU_CARD_MOVE_SECONDS,
    ));
}

#[test]
fn cpu_placed_card_reveal_waits_for_delay_without_destination_scale_pulse() {
    let slot_transform = Transform {
        translation: Vec3::new(4.0, 2.0, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let mut transform = slot_transform;
    transform.rotation = slot_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);
    let mut animation = CpuPlacedCardAnimation::flip_to_front(slot_transform, 0.25);

    assert!(!advance_cpu_placed_card_animation(
        0.24,
        &mut transform,
        &mut animation
    ));
    assert_eq!(animation.current_face(), CardFace::Back);
    assert_close(animation.start_delay_seconds, 0.01);
    assert_eq!(transform.scale, slot_transform.scale);

    assert!(!advance_cpu_placed_card_animation(
        0.25,
        &mut transform,
        &mut animation
    ));
    assert_eq!(animation.current_face(), CardFace::Front);
    assert_close(animation.current_y_rotation, 0.0);
    assert_eq!(transform.translation, slot_transform.translation);
    assert_eq!(transform.scale, slot_transform.scale);
    assert_close(
        transform.rotation.angle_between(slot_transform.rotation),
        0.0,
    );

    assert!(advance_cpu_placed_card_animation(
        0.26,
        &mut transform,
        &mut animation
    ));
    assert_eq!(animation.current_face(), CardFace::Front);
    assert_close(animation.current_y_rotation, 0.0);
    assert_eq!(transform.scale, slot_transform.scale);
}

#[test]
fn staged_match_resolution_reveals_only_occupied_cards_one_at_a_time() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<CardStateModel>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(OpponentMatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec!["a".to_string(); crate::runtime::resources::STARTING_DECK_CARD_COUNT],
        ))
        .add_systems(Update, staged_match_resolution_system);
    {
        let mut slots = app.world_mut().resource_mut::<CardSlotBoardModel>();
        assert!(slots.place_for_side_with_card_id(0, CardSlotSide::Opponent, 1, 20, "far_first"));
        assert!(slots.place_for_side_with_card_id(
            2,
            CardSlotSide::LocalPlayer,
            0,
            10,
            "near_last"
        ));
    }
    {
        let mut match_model = app.world_mut().resource_mut::<OpponentMatchModel>();
        match_model.record_placement(MatchPlayerSide::Far, 0, 1);
        match_model.record_placement(MatchPlayerSide::Near, 0, 0);
        match_model.record_placement(MatchPlayerSide::Near, 2, 0);
        match_model.begin_current_round_reveal();
        match_model.resolution_phase = MatchResolutionPhase::CpuPlacementsRevealing;
    }
    app.world_mut().spawn(CpuPlacedCardView::new(
        MatchPlayerSide::Far,
        CardSlotSide::Opponent,
        0,
        1,
        "far_first",
        CardFace::Back,
    ));

    app.update();

    let match_model = app.world().resource::<OpponentMatchModel>();
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Far, 0, 1),
        PlacementVisibility::Revealing
    );
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Near, 0, 0),
        PlacementVisibility::CurrentRoundHidden
    );
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Near, 2, 0),
        PlacementVisibility::CurrentRoundHidden
    );
    assert_close(match_model.next_reveal_delay_seconds, 0.0);

    app.update();

    let match_model = app.world().resource::<OpponentMatchModel>();
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Far, 0, 1),
        PlacementVisibility::Revealed
    );
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Near, 2, 0),
        PlacementVisibility::CurrentRoundHidden
    );
    assert_close(
        match_model.next_reveal_delay_seconds,
        CPU_CARD_REVEAL_STAGGER_SECONDS,
    );

    app.world_mut()
        .resource_mut::<OpponentMatchModel>()
        .next_reveal_delay_seconds = 0.0;
    app.update();

    assert_eq!(
        app.world()
            .resource::<OpponentMatchModel>()
            .placement_visibility(MatchPlayerSide::Near, 2, 0),
        PlacementVisibility::Revealing
    );
}

#[test]
fn cpu_placed_card_face_visibility_uses_per_card_reveal_state() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardUiState>()
        .add_systems(Update, update_cpu_placed_card_face_visibility_system);

    let root = app
        .world_mut()
        .spawn((
            CpuPlacedCardView::new(
                MatchPlayerSide::Far,
                CardSlotSide::Opponent,
                0,
                0,
                "test-card",
                CardFace::Back,
            ),
            CpuPlacedCardAnimation::move_deck_to_hand_to_slot(
                Transform::default(),
                Transform::default(),
                Transform::default(),
                CardFace::Back,
            ),
        ))
        .id();
    let front = app
        .world_mut()
        .spawn((
            CardFaceLayer::new(CardFace::Front),
            CpuPlacedCardFaceLayer,
            Visibility::Hidden,
        ))
        .id();
    let back = app
        .world_mut()
        .spawn((
            CardFaceLayer::new(CardFace::Back),
            CpuPlacedCardFaceLayer,
            Visibility::Hidden,
        ))
        .id();
    app.world_mut()
        .entity_mut(root)
        .add_children(&[front, back]);

    app.update();
    assert_eq!(
        app.world().get::<Visibility>(back),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(front),
        Some(&Visibility::Hidden)
    );

    app.world_mut()
        .entity_mut(root)
        .remove::<CpuPlacedCardAnimation>();
    app.world_mut()
        .get_mut::<CpuPlacedCardView>(root)
        .unwrap()
        .visible_face = CardFace::Front;
    app.update();

    assert_eq!(
        app.world().get::<Visibility>(front),
        Some(&Visibility::Visible)
    );
    assert_eq!(
        app.world().get::<Visibility>(back),
        Some(&Visibility::Hidden)
    );
}

#[test]
fn card_point_text2d_overlay_projects_point_root_into_2d_text_space() {
    let point_world_position = game_scene_world_position_from_game_scene(
        Vec2::new(444.0, 612.0),
        GAME_SCENE_HAND_CARD_WORLD_Z,
    );
    let point_transform = GlobalTransform::from(Transform::from_translation(point_world_position));

    let text_transform = card_point_text2d_local_transform(&point_transform);
    let text_global = point_transform * GlobalTransform::from(text_transform);
    let expected_translation =
        game_scene_text2d_position_from_game_scene(Vec2::new(444.0, 612.0), CARD_POINT_TEXT_Z);

    assert_close(text_global.translation().x, expected_translation.x);
    assert_close(text_global.translation().y, expected_translation.y);
    assert_close(text_global.translation().z, expected_translation.z);
    assert_close(text_global.to_scale_rotation_translation().0.x, 1.0);
    assert_close(text_global.to_scale_rotation_translation().0.y, 1.0);
}

#[test]
fn card_point_text2d_overlay_scales_with_point_root() {
    let point_world_position = game_scene_world_position_from_game_scene(
        Vec2::new(444.0, 612.0),
        GAME_SCENE_HAND_CARD_WORLD_Z,
    );
    let point_transform = GlobalTransform::from(Transform {
        translation: point_world_position,
        scale: Vec3::splat(1.75),
        ..Default::default()
    });

    let text_transform = card_point_text2d_local_transform(&point_transform);
    let text_global = point_transform * GlobalTransform::from(text_transform);
    let text_scale = text_global.to_scale_rotation_translation().0;

    assert_close(text_scale.x, 1.75);
    assert_close(text_scale.y, 1.75);
}

#[test]
fn game_scene_world_position_round_trips_to_game_scene_position() {
    let game_scene_position = Vec2::new(724.0, 533.0);
    let world_position = game_scene_world_position_from_game_scene(
        game_scene_position,
        GAME_SCENE_HAND_CARD_WORLD_Z,
    );

    let round_trip_position = game_scene_position_from_world_position(world_position);

    assert_close(round_trip_position.x, game_scene_position.x);
    assert_close(round_trip_position.y, game_scene_position.y);
}

#[test]
fn hand_card_entity_sync_spawns_newly_dealt_cards() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
            crate::runtime::resources::SISTER_HOTARU_CARD_MODEL_ID.to_string(),
            crate::runtime::resources::LORD_DAICHI_CARD_MODEL_ID.to_string(),
        ]))
        .add_systems(Update, sync_game_scene_hand_card_entities_system);

    app.update();

    let mut card_query = app.world_mut().query::<&HandCardGestureTarget>();
    let mut indices: Vec<usize> = card_query
        .iter(app.world())
        .map(|target| target.hand_index)
        .collect();
    indices.sort();
    assert_eq!(indices, vec![0, 1, 2]);

    app.world_mut()
        .resource_mut::<GameHandModel>()
        .cards
        .truncate(1);
    app.update();

    let mut card_query = app.world_mut().query::<&HandCardGestureTarget>();
    let indices: Vec<usize> = card_query
        .iter(app.world())
        .map(|target| target.hand_index)
        .collect();
    assert_eq!(indices, vec![0]);
}

#[test]
fn game_scene_hand_preview_transform_chain_has_global_transforms() {
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
        .add_systems(Startup, setup_game_scene);

    app.update();

    let mut transform_parent_query =
            app.world_mut()
                .query_filtered::<(&Name, &Transform, &GlobalTransform), Or<(
                    With<GameSceneRoot>,
                    With<LocalPlayerHand>,
                )>>();
    let transform_parent_names: Vec<&str> = transform_parent_query
        .iter(app.world())
        .map(|(name, _, _)| name.as_str())
        .collect();
    assert!(transform_parent_names.contains(&"GameScene"));
    assert!(transform_parent_names.contains(&"Local Player Hand"));

    let mut game_scene_ui_query =
        app.world_mut()
            .query::<(&Name, &Transform, &GlobalTransform, &GameSceneEntity)>();
    assert!(
        game_scene_ui_query
            .iter(app.world())
            .any(|(name, _, _, _)| name.as_str() == "GameScene UI")
    );

    let mut preview_query = app
        .world_mut()
        .query_filtered::<(&Name, &Transform, &GlobalTransform), With<LocalPlayerHandCardPreview>>(
        );
    assert_eq!(
        preview_query.iter(app.world()).count(),
        STARTING_HAND_CARD_COUNT
    );
}

#[test]
fn deck_rotation_system_does_not_recenter_game_scene_hand_card() {
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
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, smooth_card_rotation);

    app.update();
    app.update();

    let mut preview_query = app.world_mut().query_filtered::<&Transform, (
        With<LocalPlayerHandCardPreview>,
        With<GameSceneEntity>,
        Without<DeckSceneEntity>,
    )>();
    let initial_transforms: Vec<Transform> = preview_query.iter(app.world()).copied().collect();
    assert_eq!(initial_transforms.len(), STARTING_HAND_CARD_COUNT);

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
fn initial_local_player_hand_tweens_from_offscreen_deal_source() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<Touches>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .add_systems(Startup, setup_game_scene);

    app.update();

    let deal_transform =
        local_player_hand_deal_transform(app.world().resource::<CardInspectionDefaults>());
    let mut preview_query = app.world_mut().query_filtered::<&Transform, (
        With<LocalPlayerHandCardPreview>,
        With<CardView>,
        With<GameSceneEntity>,
        Without<DeckSceneEntity>,
    )>();
    let initial_transforms: Vec<Transform> = preview_query.iter(app.world()).copied().collect();
    assert_eq!(initial_transforms.len(), STARTING_HAND_CARD_COUNT);
    assert!(initial_transforms.iter().all(|transform| {
        (transform.translation - deal_transform.translation).length() < 0.000_1
            && (transform.scale - deal_transform.scale).length() < 0.000_1
    }));

    let mut card_states = CardStateModel::default();
    card_states.reset_to_size(STARTING_HAND_CARD_COUNT);
    app.insert_resource(CardGestureModel::default())
        .insert_resource(card_states);

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_millis(125));
    app.world_mut()
        .run_system_once(card_gesture_animation_system)
        .unwrap();

    let tweened_transforms: Vec<Transform> = preview_query.iter(app.world()).copied().collect();
    assert_eq!(tweened_transforms.len(), STARTING_HAND_CARD_COUNT);
    assert!(tweened_transforms.iter().all(|transform| {
        let game_scene_position = game_scene_position_from_world_position(transform.translation);
        game_scene_position.y < GAME_SCENE_LOCAL_HAND_DEAL_SOURCE_Y
            && transform.translation.distance(deal_transform.translation) > 0.01
    }));
}

#[test]
fn game_scene_card_hitbox_accepts_only_lower_center_card_area() {
    let window_size = Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32);
    assert_close(
        GAME_SCENE_HAND_CARD_WIDTH / GAME_SCENE_HAND_CARD_HEIGHT,
        CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT,
    );

    let hitboxes = game_scene_card_hitboxes();
    assert_eq!(hitboxes.len(), STARTING_HAND_CARD_COUNT);
    let (card_min, card_max) = hitboxes[0];
    let card_center = (card_min + card_max) * 0.5;
    let window_card_center = game_scene_pointer_to_window(card_center, window_size);

    assert!(is_game_scene_card_hit(window_card_center, window_size));
    assert!(is_game_scene_card_hit(
        game_scene_pointer_to_window(card_min + Vec2::splat(0.5), window_size),
        window_size
    ));
    assert!(is_game_scene_card_hit(
        game_scene_pointer_to_window(card_max - Vec2::splat(0.5), window_size),
        window_size
    ));
    assert!(!is_game_scene_card_hit(
        game_scene_pointer_to_window(
            Vec2::new(GAME_SCENE_WIDTH * 0.5, GAME_SCENE_HEIGHT * 0.5),
            window_size
        ),
        window_size
    ));
    assert!(!is_game_scene_card_hit(
        game_scene_pointer_to_window(card_min - Vec2::splat(1.0), window_size),
        window_size
    ));

    let last_index = STARTING_HAND_CARD_COUNT - 1;
    let last_center = (hitboxes[last_index].0 + hitboxes[last_index].1) * 0.5;
    assert_eq!(
        game_scene_card_index_at(
            game_scene_pointer_to_window(last_center, window_size),
            window_size
        ),
        Some(last_index)
    );
}

#[test]
fn hand_cards_are_centered_in_aligned_hand_area_for_variable_counts() {
    let four_hitboxes = game_scene_card_hitboxes_for_count(4);
    let first_min = four_hitboxes.first().map(|(min, _)| *min).unwrap();
    let last_max = four_hitboxes.last().map(|(_, max)| *max).unwrap();
    let group_center = (first_min + last_max) * 0.5;
    let hand_center = Vec2::new(
        GAME_SCENE_HAND_LEFT + GAME_SCENE_HAND_WIDTH * 0.5,
        GAME_SCENE_HAND_TOP + GAME_SCENE_HAND_HEIGHT * 0.5,
    );

    assert_close(
        GAME_SCENE_HAND_TOP + GAME_SCENE_HAND_HEIGHT,
        GAME_SCENE_HEIGHT,
    );
    assert_close(group_center.x, hand_center.x);
    assert_close(group_center.y, hand_center.y);
    assert_close(last_max.y - first_min.y, GAME_SCENE_HAND_CARD_HEIGHT);

    let one_hitbox = game_scene_card_hitboxes_for_count(1);
    let (single_min, single_max) = one_hitbox[0];
    assert_close(((single_min + single_max) * 0.5).x, hand_center.x);
    assert_close(((single_min + single_max) * 0.5).y, hand_center.y);
    assert_close(single_max.y - single_min.y, GAME_SCENE_HAND_CARD_HEIGHT);
}

#[test]
fn hand_cards_fit_inside_hand_area_when_count_exceeds_available_gap_space() {
    let hitboxes = game_scene_card_hitboxes_for_count(12);
    let hand_min = game_scene_hand_area_min();
    let hand_max = hand_min + game_scene_hand_area_size();

    assert_eq!(hitboxes.len(), 12);
    for (min, max) in hitboxes {
        assert!(min.x >= hand_min.x);
        assert!(max.x <= hand_max.x);
        assert!(min.y >= hand_min.y);
        assert!(max.y <= hand_max.y);
    }
}

#[test]
fn hand_hover_layout_clears_space_around_hovered_card_without_leaving_hand_area() {
    let hovered_index = 5;
    let hitboxes = game_scene_card_hitboxes_for_count_with_hover(12, Some(hovered_index));
    let (hover_min, hover_max) = hitboxes[hovered_index];
    let hand_min = game_scene_hand_area_min();
    let hand_max = hand_min + game_scene_hand_area_size();

    for (index, (min, max)) in hitboxes.iter().enumerate() {
        assert!(min.x >= hand_min.x);
        assert!(max.x <= hand_max.x);
        if index < hovered_index {
            assert!(max.x <= hover_min.x + 0.000_1);
        } else if index > hovered_index {
            assert!(min.x + 0.000_1 >= hover_max.x);
        }
    }
}

#[test]
fn hand_overlap_hit_testing_prefers_rightmost_visible_card() {
    let hitboxes = game_scene_card_hitboxes_for_count(12);
    let overlap_pair = hitboxes
        .windows(2)
        .enumerate()
        .find(|(_, pair)| pair[0].1.x > pair[1].0.x)
        .map(|(index, pair)| {
            let overlap_center_x = (pair[0].1.x + pair[1].0.x) * 0.5;
            let center_y = (pair[0].0.y + pair[0].1.y) * 0.5;
            (index, Vec2::new(overlap_center_x, center_y))
        })
        .unwrap();
    let window_size = Vec2::new(1280.0, 800.0);

    assert_eq!(
        game_scene_card_index_at_for_count(
            game_scene_pointer_to_window(overlap_pair.1, window_size),
            window_size,
            12
        ),
        Some(overlap_pair.0 + 1)
    );
}

#[test]
fn clicking_game_card_selects_in_game_without_opening_deck() {
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
        .init_resource::<CardGestureModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CardFlipState>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .add_systems(Startup, (setup_app_scene, setup_game_scene).chain())
        .add_systems(
            Update,
            (view_input_system, card_gesture_update_system).chain(),
        );
    let window = spawn_test_primary_window(&mut app);

    app.update();
    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameScene);
    assert_eq!(active_child_scene_root_count(&mut app), 1);

    app.world_mut()
        .get_mut::<Window>(window)
        .unwrap()
        .set_cursor_position(Some(game_scene_pointer_to_window(
            (game_scene_card_hitboxes()[2].0 + game_scene_card_hitboxes()[2].1) * 0.5,
            Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32),
        )));
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .press(MouseButton::Left);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .release(MouseButton::Left);
    app.update();

    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameScene);
    assert_eq!(
        app.world().resource::<CardGestureModel>().active_hand_index,
        Some(2)
    );
    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::SelectedInspecting
    );
    assert_eq!(active_child_scene_root_count(&mut app), 1);
    let mut game_scene_query = app
        .world_mut()
        .query_filtered::<Entity, With<GameSceneRoot>>();
    assert_eq!(game_scene_query.iter(app.world()).count(), 1);
    let mut card_scene_query = app
        .world_mut()
        .query_filtered::<Entity, With<DeckSceneEntity>>();
    assert_eq!(card_scene_query.iter(app.world()).count(), 0);
    let mut game_scene_entity_query = app
        .world_mut()
        .query_filtered::<Entity, With<GameSceneEntity>>();
    assert!(game_scene_entity_query.iter(app.world()).count() > 0);
    let mut camera_query = app
        .world_mut()
        .query_filtered::<Entity, With<PrimaryViewCamera>>();
    assert_eq!(camera_query.iter(app.world()).count(), 2);
}

#[test]
fn card_ui_visibility_follows_active_view() {
    assert!(!should_show_card_ui(ActiveView::GameScene));
    assert!(!should_show_card_ui(ActiveView::DeckScene));
    assert!(should_show_card_ui(ActiveView::DebugScene));
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
fn end_round_button_updates_visual_state_from_interaction() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Update, update_end_round_button);
    let button = app
        .world_mut()
        .spawn((
            EndRoundButton,
            GameControlButton::new(GameControlAction::EndRound),
            Interaction::Hovered,
            BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
            BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
        ))
        .id();

    app.update();
    assert_eq!(
        app.world().get::<BackgroundColor>(button).unwrap().0,
        END_ROUND_BUTTON_HOVER_COLOR
    );
    assert_eq!(
        *app.world().get::<BorderColor>(button).unwrap(),
        BorderColor::all(END_ROUND_BUTTON_HOVER_BORDER_COLOR)
    );

    *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::Pressed;
    app.update();
    assert_eq!(
        app.world().get::<BackgroundColor>(button).unwrap().0,
        END_ROUND_BUTTON_PRESSED_COLOR
    );
    assert_eq!(
        *app.world().get::<BorderColor>(button).unwrap(),
        BorderColor::all(END_ROUND_BUTTON_PRESSED_BORDER_COLOR)
    );

    *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::None;
    app.update();
    assert_eq!(
        app.world().get::<BackgroundColor>(button).unwrap().0,
        END_ROUND_BUTTON_NORMAL_COLOR
    );
    assert_eq!(
        *app.world().get::<BorderColor>(button).unwrap(),
        BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR)
    );
}

#[test]
fn selected_card_modal_blocks_game_control_interactions() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardGestureModel>()
        .add_systems(Update, update_end_round_button);
    let button = app
        .world_mut()
        .spawn((
            GameControlButton::new(GameControlAction::Restart),
            Interaction::Pressed,
            BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
            BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
        ))
        .id();
    {
        let mut gesture = app.world_mut().resource_mut::<CardGestureModel>();
        assert!(gesture.press(0, Vec2::ZERO, Vec2::ZERO, Transform::default()));
        gesture.select(Transform::from_scale(Vec3::splat(2.0)));
    }

    app.update();

    assert_eq!(
        app.world().get::<BackgroundColor>(button).unwrap().0,
        END_ROUND_BUTTON_NORMAL_COLOR
    );
    assert_eq!(
        *app.world().get::<BorderColor>(button).unwrap(),
        BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR)
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
        .add_systems(Startup, setup_debug_scene);

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
        .add_systems(Startup, setup_debug_scene);

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
        .add_systems(Startup, setup_debug_scene);

    app.update();

    let mut face_query = app.world_mut().query::<(&CardFaceLayer, &Visibility)>();
    let states: Vec<(CardFace, Visibility)> = face_query
        .iter(app.world())
        .map(|(face_layer, visibility)| (face_layer.face, *visibility))
        .collect();

    assert!(
        states
            .iter()
            .any(|(face, visibility)| *face == CardFace::Back && *visibility == Visibility::Hidden)
    );
    assert!(
        states.iter().any(
            |(face, visibility)| *face == CardFace::Front && *visibility == Visibility::Visible
        )
    );
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
fn face_visibility_toggles_world_space_point_text() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardFlipState>()
        .init_resource::<CardUiState>()
        .add_systems(Update, update_card_face_visibility);

    app.world_mut().spawn((
        CardFaceLayer::new(CardFace::Front),
        CardPointTextView::new(PointType::CardEnergy),
        Text2d::new("1"),
        Visibility::Hidden,
    ));

    app.world_mut().resource_mut::<CardUiState>().show_safe_area = true;
    app.update();

    let point_text = app
        .world_mut()
        .query::<(&CardPointTextView, &Text2d, &Visibility)>()
        .iter(app.world())
        .next()
        .map(|(view, text, visibility)| (view.point_type, text.0.clone(), *visibility))
        .unwrap();
    assert_eq!(
        point_text,
        (PointType::CardEnergy, "1".to_string(), Visibility::Visible)
    );
}

#[test]
fn card_back_hides_card_point_overlay_text_after_selection_overlay_update() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardFlipState>()
        .init_resource::<CardUiState>()
        .init_resource::<SelectedCardModalModel>()
        .add_systems(
            Update,
            (
                update_card_face_visibility,
                card_point_overlay_selection_update_system.after(update_card_face_visibility),
            ),
        );

    let card = app.world_mut().spawn_empty().id();
    let point = app
        .world_mut()
        .spawn((
            PointView::new(PointModel::card_power(1)),
            CardFaceLayer::new(CardFace::Front),
            Visibility::Visible,
        ))
        .id();
    let point_text = app
        .world_mut()
        .spawn((
            CardPointTextView::new(PointType::CardPower),
            Text2d::new("1"),
            Visibility::Visible,
        ))
        .id();
    app.world_mut().entity_mut(card).add_child(point);
    app.world_mut().entity_mut(point).add_child(point_text);

    app.update();
    assert_eq!(
        app.world().entity(point_text).get::<Visibility>(),
        Some(&Visibility::Visible)
    );

    {
        let mut flip_state = app.world_mut().resource_mut::<CardFlipState>();
        flip_state.current_y_rotation = std::f32::consts::PI;
        flip_state.visible_face = CardFace::Back;
    }
    app.update();

    assert_eq!(
        app.world().entity(point).get::<Visibility>(),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().entity(point_text).get::<Visibility>(),
        Some(&Visibility::Hidden)
    );
}

#[test]
fn composed_card_rotation_layers_flip_over_pointer_rotation() {
    let card_state = CardInspectionState {
        last_pointer_normalized: Vec2::ZERO,
        target_rotation: Quat::from_euler(EulerRot::XYZ, 0.2, -0.1, 0.0),
    };
    let flip_state = CardFlipState {
        start_y_rotation: std::f32::consts::PI,
        current_y_rotation: std::f32::consts::PI,
        target_y_rotation: std::f32::consts::PI,
        elapsed_seconds: 0.0,
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
        .init_asset::<Font>()
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
        .add_systems(Startup, setup_deck_scene)
        .add_systems(Update, card_model_input_system);

    app.update();
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DebugScene;
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
        .init_asset::<Font>()
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
        .add_systems(Startup, setup_deck_scene)
        .add_systems(Update, card_model_input_system);

    app.update();
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DebugScene;
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
fn deck_card_layers_use_shared_render_aspect_ratio() {
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
        .add_systems(Startup, setup_debug_scene);

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
        .add_systems(Startup, setup_debug_scene);

    app.update();

    let card_defaults = CardInspectionDefaults::default();
    let frame_dimensions = frame_dimensions(&card_defaults);
    let mut background_query = app
        .world_mut()
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
        .add_systems(Startup, setup_debug_scene);

    app.update();

    let card_defaults = CardInspectionDefaults::default();
    let frame_dimensions = frame_dimensions(&card_defaults);
    let mut background_query = app
        .world_mut()
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
        .add_systems(Startup, setup_debug_scene);

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
        .add_systems(Startup, setup_deck_scene)
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
fn debug_hud_excludes_removed_deck_toggle_key() {
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
fn debug_hud_debug_drawing_key_label_hints_shift_mode() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_debug_hud);

    app.update();

    let label = app
        .world_mut()
        .query::<(&DebugHudKeyText, &TextSpan)>()
        .iter(app.world())
        .find_map(|(key_text, text_span)| {
            (key_text.key_code == KeyCode::KeyD).then_some(text_span.0.as_str())
        })
        .unwrap();

    assert_eq!(label, "[D]");
}

#[test]
fn debug_hud_removes_unused_wa_keys() {
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
}

#[test]
fn debug_hud_scene_cycle_key_is_not_toggle() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_debug_hud);

    app.update();

    let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
    let scene_key = key_query
        .iter(app.world())
        .find(|key_text| key_text.key_code == KeyCode::KeyS)
        .unwrap();

    assert!(!scene_key.is_toggle);
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
            .is_debug_drawing_visible()
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
            .is_debug_drawing_visible()
    );
}

#[test]
fn shift_d_toggles_solo_debug_drawing() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<DebugHudState>()
        .add_systems(Update, toggle_debug_hud_inputs);

    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.press(KeyCode::ShiftLeft);
        keys.press(KeyCode::KeyD);
    }
    app.update();

    assert_eq!(
        app.world().resource::<DebugHudState>().debug_draw_mode,
        DebugDrawMode::OnSolo
    );

    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.reset(KeyCode::KeyD);
        keys.press(KeyCode::KeyD);
    }
    app.update();

    assert_eq!(
        app.world().resource::<DebugHudState>().debug_draw_mode,
        DebugDrawMode::Off
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
fn escape_key_saves_fullscreen_preference_before_close() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ButtonInput<KeyCode>>()
        .insert_resource(DebugHudState {
            is_fullscreen: true,
            ..Default::default()
        })
        .insert_resource(test_debug_hud_input_store("escape-debug-hud-input"))
        .add_message::<WindowCloseRequested>()
        .add_message::<AppExit>()
        .add_systems(Update, quit_app_on_escape);
    app.world_mut().spawn((Window::default(), PrimaryWindow));

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::Escape);
    app.update();

    assert!(
        app.world()
            .resource::<Persistent<DebugHudInputStore>>()
            .is_fullscreen
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
fn f_key_on_saves_fullscreen_and_windowed_placement() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<DebugHudState>()
        .init_resource::<WindowPlacementState>()
        .insert_resource(test_debug_hud_input_store("f-on-debug-hud-input"))
        .insert_resource(test_window_placement_store("f-on-window-placement"))
        .add_systems(Update, toggle_debug_hud_inputs);
    app.world_mut()
        .spawn(test_monitor("Primary", IVec2::ZERO, UVec2::new(1920, 1080)));
    app.world_mut().spawn((
        Window {
            position: WindowPosition::At(IVec2::new(240, 120)),
            resolution: WindowResolution::new(1024, 768),
            ..Default::default()
        },
        PrimaryWindow,
    ));

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyF);
    app.update();

    assert!(
        app.world()
            .resource::<Persistent<DebugHudInputStore>>()
            .is_fullscreen
    );
    let saved_placement = app
        .world()
        .resource::<Persistent<WindowPlacementStore>>()
        .current
        .as_ref()
        .expect("window placement should be saved");
    assert_eq!(saved_placement.window_position, IVec2::new(240, 120));
    assert_eq!(saved_placement.window_size, UVec2::new(1024, 768));
}

#[test]
fn f_key_off_saves_windowed_state_and_restored_placement() {
    let saved_windowed_placement = WindowPlacement {
        window_position: IVec2::new(320, 180),
        window_size: UVec2::new(900, 700),
        monitor_name: Some("Primary".to_string()),
        monitor_position: IVec2::ZERO,
        monitor_size: UVec2::new(1920, 1080),
        relative_position: IVec2::new(320, 180),
    };
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ButtonInput<KeyCode>>()
        .insert_resource(DebugHudState {
            is_fullscreen: true,
            ..Default::default()
        })
        .insert_resource(WindowPlacementState {
            current: Some(saved_windowed_placement),
            restored: true,
        })
        .insert_resource(test_debug_hud_input_store("f-off-debug-hud-input"))
        .insert_resource(test_window_placement_store("f-off-window-placement"))
        .add_systems(Update, toggle_debug_hud_inputs);
    app.world_mut()
        .spawn(test_monitor("Primary", IVec2::ZERO, UVec2::new(1920, 1080)));
    app.world_mut().spawn((
        Window {
            position: WindowPosition::At(IVec2::ZERO),
            resolution: WindowResolution::new(1920, 1080),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            ..Default::default()
        },
        PrimaryWindow,
    ));

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyF);
    app.update();

    assert!(
        !app.world()
            .resource::<Persistent<DebugHudInputStore>>()
            .is_fullscreen
    );
    let window = app
        .world_mut()
        .query_filtered::<&Window, With<PrimaryWindow>>()
        .single(app.world())
        .unwrap();
    assert_eq!(window.mode, WindowMode::Windowed);
    assert_eq!(window.position, WindowPosition::At(IVec2::new(320, 180)));
    assert_eq!(logical_window_size(window), UVec2::new(900, 700));

    let saved_placement = app
        .world()
        .resource::<Persistent<WindowPlacementStore>>()
        .current
        .as_ref()
        .expect("window placement should be saved");
    assert_eq!(saved_placement.window_position, IVec2::new(320, 180));
    assert_eq!(saved_placement.window_size, UVec2::new(900, 700));
}

#[test]
fn fullscreen_window_resize_does_not_replace_saved_windowed_placement() {
    let saved_windowed_placement = WindowPlacement {
        window_position: IVec2::new(400, 200),
        window_size: UVec2::new(960, 540),
        monitor_name: Some("Primary".to_string()),
        monitor_position: IVec2::ZERO,
        monitor_size: UVec2::new(1920, 1080),
        relative_position: IVec2::new(400, 200),
    };
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<WindowResized>()
        .insert_resource(DebugHudState {
            is_fullscreen: true,
            ..Default::default()
        })
        .insert_resource(WindowPlacementState {
            current: Some(saved_windowed_placement),
            restored: true,
        })
        .add_systems(Update, track_window_size);
    app.world_mut()
        .spawn(test_monitor("Primary", IVec2::ZERO, UVec2::new(1920, 1080)));
    let primary_window = app
        .world_mut()
        .spawn((
            Window {
                position: WindowPosition::At(IVec2::new(0, 0)),
                resolution: WindowResolution::new(1280, 800),
                mode: WindowMode::Windowed,
                ..Default::default()
            },
            PrimaryWindow,
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<WindowResized>>()
        .write(WindowResized {
            window: primary_window,
            width: 1280.0,
            height: 800.0,
        });
    app.update();

    let placement = app
        .world()
        .resource::<WindowPlacementState>()
        .current
        .as_ref()
        .expect("saved windowed placement should remain available");
    assert_eq!(placement.window_position, IVec2::new(400, 200));
    assert_eq!(placement.window_size, UVec2::new(960, 540));
}

#[test]
fn fullscreen_startup_does_not_restore_windowed_placement() {
    let saved_windowed_placement = WindowPlacement {
        window_position: IVec2::new(2120, 160),
        window_size: UVec2::new(320, 180),
        monitor_name: Some("Secondary".to_string()),
        monitor_position: IVec2::new(1920, 0),
        monitor_size: UVec2::new(1920, 1080),
        relative_position: IVec2::new(200, 160),
    };
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(DebugHudState {
            is_fullscreen: true,
            ..Default::default()
        })
        .insert_resource(WindowPlacementState {
            current: Some(saved_windowed_placement),
            restored: false,
        })
        .add_systems(Update, restore_window_placement_to_current_monitors);
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
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            ..Default::default()
        },
        PrimaryWindow,
    ));

    app.update();

    let window = app
        .world_mut()
        .query_filtered::<&Window, With<PrimaryWindow>>()
        .single(app.world())
        .unwrap();
    assert_eq!(
        window.position,
        WindowPosition::Centered(MonitorSelection::Primary)
    );
    assert_eq!(
        logical_window_size(window),
        UVec2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
    );
    assert_eq!(
        window.mode,
        WindowMode::BorderlessFullscreen(MonitorSelection::Entity(secondary_monitor))
    );
    assert!(app.world().resource::<WindowPlacementState>().restored);
}

#[test]
fn fullscreen_window_close_saves_f_on_and_preserves_windowed_placement() {
    let saved_windowed_placement = WindowPlacement {
        window_position: IVec2::new(440, 220),
        window_size: UVec2::new(1000, 700),
        monitor_name: Some("Primary".to_string()),
        monitor_position: IVec2::ZERO,
        monitor_size: UVec2::new(1920, 1080),
        relative_position: IVec2::new(440, 220),
    };
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<WindowCloseRequested>()
        .insert_resource(DebugHudState {
            is_fullscreen: true,
            ..Default::default()
        })
        .insert_resource(WindowPlacementState {
            current: Some(saved_windowed_placement),
            restored: true,
        })
        .insert_resource(test_debug_hud_input_store(
            "fullscreen-close-debug-hud-input",
        ))
        .insert_resource(test_window_placement_store(
            "fullscreen-close-window-placement",
        ))
        .add_systems(Update, save_window_placement_on_close);
    app.world_mut()
        .spawn(test_monitor("Primary", IVec2::ZERO, UVec2::new(1920, 1080)));
    let primary_window = app
        .world_mut()
        .spawn((
            Window {
                position: WindowPosition::At(IVec2::ZERO),
                resolution: WindowResolution::new(1280, 800),
                mode: WindowMode::Windowed,
                ..Default::default()
            },
            PrimaryWindow,
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<WindowCloseRequested>>()
        .write(WindowCloseRequested {
            window: primary_window,
        });
    app.update();

    assert!(
        app.world()
            .resource::<Persistent<DebugHudInputStore>>()
            .is_fullscreen
    );
    let saved_placement = app
        .world()
        .resource::<Persistent<WindowPlacementStore>>()
        .current
        .as_ref()
        .expect("window placement should be saved");
    assert_eq!(saved_placement.window_position, IVec2::new(440, 220));
    assert_eq!(saved_placement.window_size, UVec2::new(1000, 700));
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
fn s_key_cycles_game_to_deck_to_debug_and_wraps() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<Touches>()
        .init_resource::<PrimaryCameraDefaults>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardInspectionState>()
        .init_resource::<CardFlipState>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .init_resource::<ActiveView>()
        .add_systems(Startup, (setup_app_scene, setup_game_scene).chain())
        .add_systems(
            Startup,
            sync_debug_hud_ui_camera_system.after(setup_game_scene),
        )
        .add_systems(
            Update,
            (
                scene_input_system,
                sync_debug_hud_ui_camera_system.after(scene_input_system),
            ),
        );

    app.update();
    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameScene);
    assert_debug_hud_targets_active_ui_camera(&mut app, ActiveView::GameScene);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    app.update();

    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::DeckScene);
    assert_debug_hud_targets_active_ui_camera(&mut app, ActiveView::DeckScene);
    let mut hud_query = app
        .world_mut()
        .query_filtered::<&Visibility, With<DebugHudText>>();
    assert!(matches!(
        hud_query.single(app.world()),
        Ok(Visibility::Visible | Visibility::Inherited)
    ));
    assert_eq!(active_child_scene_root_count(&mut app), 2);
    let mut game_query = app
        .world_mut()
        .query_filtered::<&Visibility, With<GameSceneRoot>>();
    assert!(matches!(
        game_query.single(app.world()),
        Ok(Visibility::Hidden)
    ));
    let mut deck_query = app
        .world_mut()
        .query_filtered::<Entity, With<DeckSceneRoot>>();
    assert_eq!(deck_query.iter(app.world()).count(), 1);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset(KeyCode::KeyS);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    app.update();

    assert_eq!(
        *app.world().resource::<ActiveView>(),
        ActiveView::DebugScene
    );
    assert_debug_hud_targets_active_ui_camera(&mut app, ActiveView::DebugScene);
    let mut hud_query = app
        .world_mut()
        .query_filtered::<&Visibility, With<DebugHudText>>();
    assert!(matches!(
        hud_query.single(app.world()),
        Ok(Visibility::Visible | Visibility::Inherited)
    ));
    assert_eq!(active_child_scene_root_count(&mut app), 2);
    let mut debug_query = app
        .world_mut()
        .query_filtered::<Entity, With<DebugSceneRoot>>();
    assert_eq!(debug_query.iter(app.world()).count(), 1);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset(KeyCode::KeyS);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    app.update();

    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameScene);
    assert_debug_hud_targets_active_ui_camera(&mut app, ActiveView::GameScene);
    assert_eq!(active_child_scene_root_count(&mut app), 1);
    let mut game_query = app
        .world_mut()
        .query_filtered::<&Visibility, With<GameSceneRoot>>();
    assert!(matches!(
        game_query.single(app.world()),
        Ok(Visibility::Visible | Visibility::Inherited)
    ));
}

#[test]
fn s_key_return_preserves_existing_game_scene_entities() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<Touches>()
        .init_resource::<PrimaryCameraDefaults>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardInspectionState>()
        .init_resource::<CardFlipState>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .init_resource::<ActiveView>()
        .add_systems(Startup, (setup_app_scene, setup_game_scene).chain())
        .add_systems(Update, scene_input_system);

    app.update();

    let game_scene_root = {
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<GameSceneRoot>>();
        query
            .single(app.world())
            .expect("GameScene root should spawn")
    };
    let expected_transform = Transform::from_xyz(12.0, 34.0, 56.0);
    let preserved_entity = app
        .world_mut()
        .spawn((
            GameSceneEntity,
            expected_transform,
            GlobalTransform::default(),
            Visibility::Visible,
        ))
        .id();
    let preserved_child = app
        .world_mut()
        .spawn((GlobalTransform::default(), Visibility::Visible))
        .id();
    app.world_mut()
        .entity_mut(preserved_entity)
        .add_child(preserved_child);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset(KeyCode::KeyS);
    app.update();

    assert_eq!(
        app.world().entity(preserved_entity).get::<Visibility>(),
        Some(&Visibility::Hidden)
    );
    assert_eq!(
        app.world().entity(preserved_child).get::<Visibility>(),
        Some(&Visibility::Hidden)
    );

    for _ in 0..2 {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyS);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyS);
        app.update();
    }

    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameScene);
    let mut root_query = app
        .world_mut()
        .query_filtered::<Entity, With<GameSceneRoot>>();
    assert_eq!(
        root_query
            .single(app.world())
            .expect("GameScene root should be reused"),
        game_scene_root
    );
    let transform = app
        .world()
        .entity(preserved_entity)
        .get::<Transform>()
        .expect("preserved GameScene entity should survive scene cycling");
    assert_eq!(*transform, expected_transform);
    let visibility = app
        .world()
        .entity(preserved_entity)
        .get::<Visibility>()
        .expect("preserved GameScene entity should keep visibility");
    assert!(matches!(
        visibility,
        Visibility::Visible | Visibility::Inherited
    ));
    let child_visibility = app
        .world()
        .entity(preserved_child)
        .get::<Visibility>()
        .expect("preserved GameScene child should restore visibility");
    assert!(matches!(
        child_visibility,
        Visibility::Visible | Visibility::Inherited
    ));
}

#[test]
fn restart_key_reloads_game_scene_and_clears_game_model() {
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
        .init_resource::<CardGestureModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .init_resource::<ActiveView>()
        .add_systems(Startup, setup_app_scene)
        .add_systems(Startup, setup_deck_scene)
        .add_systems(Update, restart_app_scene);

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DeckScene;

    app.update();

    app.world_mut().resource_mut::<GameTicks>().0 = 42;
    assert_eq!(
        app.world_mut()
            .resource_mut::<CardSlotBoardModel>()
            .place_next_local(1, 0),
        Some(0)
    );
    assert!(
        app.world_mut()
            .resource_mut::<CardStateModel>()
            .place_in_location(0)
    );
    app.world_mut().resource_mut::<CardGestureModel>().state = CardGestureState::Dragging;
    app.world_mut()
        .resource_mut::<CardInspectionState>()
        .last_pointer_normalized = Vec2::ONE;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyR);
    app.update();

    assert_eq!(*app.world().resource::<ActiveView>(), ActiveView::GameScene);

    let mut hud_query = app
        .world_mut()
        .query_filtered::<Entity, With<DebugHudText>>();
    assert_eq!(hud_query.iter(app.world()).count(), 1);

    let mut card_query = app
        .world_mut()
        .query_filtered::<Entity, (With<CardView>, With<LocalPlayerHandCardPreview>)>();
    assert_eq!(
        card_query.iter(app.world()).count(),
        STARTING_HAND_CARD_COUNT
    );
    assert_eq!(app.world().resource::<GameTicks>().0, 0);
    assert_eq!(
        app.world()
            .resource::<CardInspectionState>()
            .last_pointer_normalized,
        Vec2::ZERO
    );
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        0
    );
    assert_eq!(
        app.world().resource::<CardStateModel>().state(0),
        Some(CardState::Hand)
    );
    assert_eq!(
        app.world().resource::<CardGestureModel>().state,
        CardGestureState::Idle
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
