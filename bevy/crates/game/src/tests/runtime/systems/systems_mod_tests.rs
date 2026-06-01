use super::*;
use crate::runtime::bundles::AppCameraBundle;
use crate::runtime::components::{
    AppSceneCamera, CardAnimationMarker, CardGrid, DeckScreenContentRoot,
    DeckScreenDeckCommandButton, DeckScreenModalActionButton, DeckScreenSelectedCardMenuRoot,
    DeckScreenTabButton, DeckView, PrimaryViewCamera, ScreenTransitionOverlay, SelectableCard,
    WorldThemeTransition, WorldThemeTransitionPhase,
};
use crate::runtime::resources::deck_screen_model::DECK_SCREEN_VISIBLE_CARD_COUNT;
use crate::runtime::resources::{
    AppCameraModel, CardGestureModel, CardGestureState, CardSlotBoardModel, CpuBrainMoveModel,
    CurrentRoundMoveRecord, DECK_SCREEN_COMING_SOON_MESSAGE, DECK_SCREEN_COMING_SOON_TITLE,
    DebugDrawMode, DeckEditorTabModel, DeckScreenMode, MATCHMAKING_PREPARING_SECONDS,
    MatchmakingPhaseModel, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN,
    SCREEN_TRANSITION_BLACK_HOLD_SECONDS, ScreenTransitionPhase,
};
use bevy::camera::RenderTargetInfo;
use bevy::ecs::system::RunSystemOnce;
use bevy::text::Font;
use bevy::time::TimeUpdateStrategy;
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

fn update_until_world_transition_idle(app: &mut App) {
    for _ in 0..32 {
        app.update();
        let transition = app
            .world_mut()
            .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
            .single(app.world())
            .unwrap();
        if transition.phase == WorldThemeTransitionPhase::Idle {
            return;
        }
    }

    panic!("world transition did not become idle");
}

fn apparent_scale_at_z(scale: f32, z: f32) -> f32 {
    scale / game_scene_world_units_per_game_scene_pixel(z)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TestRgbaPixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl TestRgbaPixel {
    fn from_color(color: Color) -> Self {
        let color = color.to_srgba();
        Self {
            red: color_component_to_u8(color.red),
            green: color_component_to_u8(color.green),
            blue: color_component_to_u8(color.blue),
            alpha: color_component_to_u8(color.alpha),
        }
    }

    const fn transparent() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }

    const fn is_visible_white(self) -> bool {
        self.red >= 245 && self.green >= 245 && self.blue >= 245 && self.alpha >= 245
    }
}

fn color_component_to_u8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

struct PointViewVisualTestImage {
    pixels: Vec<TestRgbaPixel>,
}

impl PointViewVisualTestImage {
    fn white_pixel_count(&self) -> usize {
        self.pixels
            .iter()
            .filter(|pixel| pixel.is_visible_white())
            .count()
    }
}

fn render_point_view_visual_test_image(
    point_model: PointModel,
    text: &str,
    text_color: Color,
    font_size: f32,
) -> PointViewVisualTestImage {
    const IMAGE_SIZE: usize = 96;
    const CENTER: i32 = (IMAGE_SIZE / 2) as i32;
    const RADIUS: i32 = 42;
    const GLYPH_WIDTH: usize = 3;
    const GLYPH_HEIGHT: usize = 5;
    const GLYPH_GAP: usize = 1;

    let glyph_scale = ((font_size / POINT_VIEW_BASE_TEXT_FONT_SIZE) * 8.0).round() as usize;
    let glyph_scale = glyph_scale.max(1);
    let mut pixels = vec![TestRgbaPixel::transparent(); IMAGE_SIZE * IMAGE_SIZE];
    let background = TestRgbaPixel::from_color(point_model.background_color());
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let dx = x as i32 - CENTER;
            let dy = y as i32 - CENTER;
            if (dx * dx) + (dy * dy) <= RADIUS * RADIUS {
                pixels[(y * IMAGE_SIZE) + x] = background;
            }
        }
    }

    let glyph_count = text.chars().count();
    let text_width = glyph_count
        .saturating_mul(GLYPH_WIDTH * glyph_scale)
        .saturating_add(glyph_count.saturating_sub(1) * GLYPH_GAP * glyph_scale);
    let start_x = (IMAGE_SIZE - text_width) / 2;
    let start_y = (IMAGE_SIZE - (GLYPH_HEIGHT * glyph_scale)) / 2;
    let text_pixel = TestRgbaPixel::from_color(text_color);

    for (index, character) in text.chars().enumerate() {
        let glyph_x = start_x + index * ((GLYPH_WIDTH + GLYPH_GAP) * glyph_scale);
        draw_test_digit_glyph(
            &mut pixels,
            IMAGE_SIZE,
            glyph_x,
            start_y,
            glyph_scale,
            character,
            text_pixel,
        );
    }

    PointViewVisualTestImage { pixels }
}

fn draw_test_digit_glyph(
    pixels: &mut [TestRgbaPixel],
    image_size: usize,
    x: usize,
    y: usize,
    scale: usize,
    character: char,
    color: TestRgbaPixel,
) {
    for (row, pattern) in test_digit_glyph(character).iter().enumerate() {
        for (column, value) in pattern.chars().enumerate() {
            if value != '#' {
                continue;
            }
            for pixel_y in 0..scale {
                for pixel_x in 0..scale {
                    let target_x = x + (column * scale) + pixel_x;
                    let target_y = y + (row * scale) + pixel_y;
                    if target_x < image_size && target_y < image_size {
                        pixels[(target_y * image_size) + target_x] = color;
                    }
                }
            }
        }
    }
}

fn test_digit_glyph(character: char) -> [&'static str; 5] {
    match character {
        '-' => ["...", "...", "###", "...", "..."],
        '0' => ["###", "#.#", "#.#", "#.#", "###"],
        '1' => [".#.", "##.", ".#.", ".#.", "###"],
        '2' => ["###", "..#", "###", "#..", "###"],
        '3' => ["###", "..#", "###", "..#", "###"],
        '4' => ["#.#", "#.#", "###", "..#", "..#"],
        '5' => ["###", "#..", "###", "..#", "###"],
        '6' => ["###", "#..", "###", "#.#", "###"],
        '7' => ["###", "..#", ".#.", ".#.", ".#."],
        '8' => ["###", "#.#", "###", "#.#", "###"],
        '9' => ["###", "#.#", "###", "..#", "###"],
        _ => ["...", ".#.", "...", ".#.", "..."],
    }
}

#[test]
fn shared_app_camera_bundle_can_target_ui_and_3d_content() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>();

    let camera = app
        .world_mut()
        .spawn(AppCameraBundle::new(&AppCameraModel::active()))
        .id();
    let mesh = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(Rectangle::new(1.0, 1.0));
    let material = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial::default());
    app.world_mut().spawn((
        Name::new("Shared Camera 3D Proof Mesh"),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::default(),
    ));
    app.world_mut().spawn((
        Name::new("Shared Camera UI Proof Root"),
        Node::default(),
        Button,
        Text::new("Shared Camera UI"),
        UiTargetCamera(camera),
    ));

    let mut camera_query = app
        .world_mut()
        .query_filtered::<(&Camera, &Projection), (With<AppSceneCamera>, With<Camera3d>)>();
    let (camera_component, projection) = camera_query
        .single(app.world())
        .expect("shared AppScene camera should exist");
    assert!(camera_component.is_active);
    assert!(matches!(projection, Projection::Perspective(_)));

    let mut ui_query = app
        .world_mut()
        .query_filtered::<&UiTargetCamera, (With<Button>, With<Text>)>();
    assert_eq!(
        ui_query
            .single(app.world())
            .expect("UI proof root should target the shared camera")
            .0,
        camera
    );

    let mut mesh_query = app
        .world_mut()
        .query::<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>();
    assert_eq!(mesh_query.iter(app.world()).count(), 1);
}

fn active_child_scene_root_count(app: &mut App) -> usize {
    let mut scene_query = app.world_mut().query_filtered::<Entity, Or<(
        With<GameSceneRoot>,
        With<DeckSceneRoot>,
        With<DebugSceneRoot>,
    )>>();
    scene_query.iter(app.world()).count()
}

#[allow(dead_code)]
fn assert_debug_hud_targets_active_ui_camera(app: &mut App, active_view: ActiveView) {
    let _ = active_view;
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

    assert!(camera_entity.get::<AppSceneCamera>().is_some());
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

fn sync_debug_scene_global_transforms(app: &mut App) {
    let transforms: Vec<(Entity, Transform)> = app
        .world_mut()
        .query_filtered::<(Entity, &Transform), With<DebugSceneEntity>>()
        .iter(app.world())
        .map(|(entity, transform)| (entity, *transform))
        .collect();
    for (entity, transform) in transforms {
        if let Some(mut global_transform) = app.world_mut().get_mut::<GlobalTransform>(entity) {
            *global_transform = GlobalTransform::from(transform);
        }
    }
}

fn window_pointer_for_debug_card_center(app: &mut App, card: Entity) -> Vec2 {
    let card_world_position = app
        .world()
        .get::<Transform>(card)
        .expect("debug card should have a Transform")
        .translation;
    let mut camera_query = app
        .world_mut()
        .query_filtered::<(&Camera, &Transform), (With<AppSceneCamera>, With<Camera3d>)>();
    let (camera, camera_transform) = camera_query
        .single(app.world())
        .expect("debug scene should have one primary 3D camera");
    let camera_global_transform = GlobalTransform::from(*camera_transform);
    let mut ndc = camera
        .world_to_ndc(&camera_global_transform, card_world_position)
        .expect("debug card center should project into NDC");
    ndc.y = -ndc.y;
    let mut window_query = app
        .world_mut()
        .query_filtered::<&Window, With<PrimaryWindow>>();
    let window = window_query
        .single(app.world())
        .expect("test should have one primary window");
    let window_size = Vec2::new(window.resolution.width(), window.resolution.height());

    (ndc.truncate() + Vec2::ONE) * 0.5 * window_size
}

fn prepare_debug_camera_for_test_viewport(app: &mut App, window: Entity) {
    let _ = app
        .world()
        .get::<Window>(window)
        .expect("test should have a primary window");
    let physical_size = UVec2::new(DEFAULT_WINDOW_WIDTH as u32, DEFAULT_WINDOW_HEIGHT as u32);
    let logical_size = Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32);
    let mut camera_query = app
        .world_mut()
        .query_filtered::<(&mut Camera, &mut Projection), (With<AppSceneCamera>, With<Camera3d>)>();
    let (mut camera, mut projection) = camera_query
        .single_mut(app.world_mut())
        .expect("debug scene should have one primary 3D camera");
    projection.update(logical_size.x, logical_size.y);
    camera.computed.target_info = Some(RenderTargetInfo {
        physical_size,
        scale_factor: 1.0,
    });
    camera.computed.clip_from_view = projection.get_clip_from_view();
    sync_debug_scene_global_transforms(app);
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

fn nav_destination_target_view(destination: TopNavigationDestination) -> ActiveView {
    match destination {
        TopNavigationDestination::PlayGame => ActiveView::MainMenuScene,
        TopNavigationDestination::MyDecks => ActiveView::DeckScene,
        TopNavigationDestination::Settings => ActiveView::SettingsScene,
        TopNavigationDestination::Debug => ActiveView::DebugScene,
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TransitionTraceFrame {
    phase: ScreenTransitionPhase,
    alpha: f32,
}

fn create_nav_transition_test_app_with_start(start_view: ActiveView) -> App {
    let mut app = App::new();
    let frame_step = std::time::Duration::from_secs_f32(1.0 / 120.0);
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(frame_step))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_resource::<ActiveView>()
        .init_resource::<MetaGameSettingsModel>()
        .init_resource::<TopNavigationModel>()
        .init_resource::<ScreenTransitionResource>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<PrimaryCameraDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<CardFlipState>()
        .init_resource::<CardInspectionState>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .init_resource::<MatchmakingModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<Touches>()
        .init_resource::<CardGestureModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<MatchModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CpuBrainModel>()
        .add_systems(
            Startup,
            (
                setup_app_scene,
                setup_initial_meta_scene.after(setup_app_scene),
            ),
        )
        .add_systems(
            Update,
            (
                transition_overlay_target_camera_update_system,
                sync_debug_hud_ui_camera_system,
                top_navigation_update_system,
                meta_screen_update_system.after(top_navigation_update_system),
                transition_update_system
                    .after(top_navigation_update_system)
                    .after(meta_screen_update_system),
            ),
        );
    let _window = spawn_test_primary_window(&mut app);
    *app.world_mut().resource_mut::<ActiveView>() = start_view;
    app.update();
    app
}

fn create_nav_transition_test_app() -> App {
    create_nav_transition_test_app_with_start(ActiveView::MainMenuScene)
}

fn collect_top_nav_transition_trace(
    destination: TopNavigationDestination,
) -> Vec<TransitionTraceFrame> {
    let mut app = create_nav_transition_test_app_with_start(ActiveView::LightningScene);
    let target_view = nav_destination_target_view(destination);
    {
        let mut transition = app.world_mut().resource_mut::<ScreenTransitionResource>();
        transition.phase = ScreenTransitionPhase::Idle;
        transition.phase_elapsed_seconds = 0.0;
        transition.overlay_alpha = 0.0;
    }

    let button_entity = app
        .world_mut()
        .query::<(Entity, &TopNavigationButton)>()
        .iter(app.world())
        .find_map(|(entity, button)| (button.destination == destination).then_some(entity))
        .expect("top nav button should exist for destination");
    *app.world_mut()
        .get_mut::<Interaction>(button_entity)
        .unwrap() = Interaction::Pressed;

    let mut trace = Vec::new();
    for _ in 0..960 {
        app.update();
        let transition = app.world().resource::<ScreenTransitionResource>();
        trace.push(TransitionTraceFrame {
            phase: transition.phase,
            alpha: transition.overlay_alpha,
        });
        if transition.phase == ScreenTransitionPhase::Idle
            && *app.world().resource::<ActiveView>() == target_view
        {
            break;
        }
    }

    trace
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
fn debug_hud_is_hidden_on_game_and_deck_screens() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<GameTicks>()
        .init_resource::<ActiveView>()
        .init_resource::<DebugHudState>()
        .add_systems(Startup, setup_debug_hud)
        .add_systems(Update, update_debug_hud);

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    app.update();

    let game_visibility = *app
        .world_mut()
        .query_filtered::<&Visibility, With<DebugHudText>>()
        .single(app.world())
        .expect("debug hud should exist");
    assert_eq!(game_visibility, Visibility::Hidden);

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DeckScene;
    app.update();

    let deck_visibility = *app
        .world_mut()
        .query_filtered::<&Visibility, With<DebugHudText>>()
        .single(app.world())
        .expect("debug hud should exist");
    assert_eq!(deck_visibility, Visibility::Hidden);

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::SettingsScene;
    app.update();

    let settings_visibility = *app
        .world_mut()
        .query_filtered::<&Visibility, With<DebugHudText>>()
        .single(app.world())
        .expect("debug hud should exist");
    assert_eq!(settings_visibility, Visibility::Visible);
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

    let (app_camera_entity, app_camera) = app
        .world_mut()
        .query_filtered::<(Entity, &Camera), (With<AppSceneCamera>, With<Camera3d>)>()
        .single(app.world())
        .expect("shared AppScene camera should exist");
    assert!(app_camera.is_active);

    let (transition_overlay_entity, transition_target) = app
        .world_mut()
        .query_filtered::<(Entity, &UiTargetCamera), With<ScreenTransitionOverlay>>()
        .single(app.world())
        .expect("transition overlay should exist");
    assert_eq!(transition_target.0, app_camera_entity);
    assert!(
        app.world()
            .get::<ChildOf>(transition_overlay_entity)
            .is_none(),
        "transition overlay must be a root UI node so its UiTargetCamera is authoritative"
    );

    let mut card_query = app.world_mut().query_filtered::<Entity, With<CardView>>();
    assert_eq!(card_query.iter(app.world()).count(), 0);
}

#[test]
fn active_ui_camera_sync_keeps_single_global_owner_when_stale_meta_cameras_exist() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(ActiveView::MainMenuScene)
        .add_systems(Update, sync_debug_hud_ui_camera_system);

    app.world_mut().spawn((
        Name::new("Current Meta UI Camera"),
        MetaSceneEntity,
        Camera2d,
        Camera::default(),
    ));
    app.world_mut().spawn((
        Name::new("Stale Meta UI Camera"),
        MetaSceneEntity,
        Camera2d,
        Camera::default(),
        IsDefaultUiCamera,
        PrimaryEguiContext,
    ));

    app.update();

    let primary_count = app
        .world_mut()
        .query_filtered::<(), With<PrimaryEguiContext>>()
        .iter(app.world())
        .count();
    let default_count = app
        .world_mut()
        .query_filtered::<(), With<IsDefaultUiCamera>>()
        .iter(app.world())
        .count();

    assert_eq!(primary_count, 1);
    assert_eq!(default_count, 1);
}

#[test]
fn settings_scene_root_targets_shared_ui_camera_and_spawns_buttons() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .insert_resource(ActiveView::SettingsScene)
        .init_resource::<MatchmakingModel>()
        .init_resource::<MetaGameSettingsModel>()
        .add_systems(Startup, setup_initial_meta_scene);

    app.update();

    let app_camera = app
        .world_mut()
        .query_filtered::<Entity, With<AppSceneCamera>>()
        .single(app.world())
        .expect("settings should reuse the shared app camera");
    let (_, target_camera) = app
        .world_mut()
        .query_filtered::<(Entity, &UiTargetCamera), With<MetaSceneRoot>>()
        .single(app.world())
        .expect("settings root should target the shared UI camera");
    assert_eq!(*target_camera, UiTargetCamera(app_camera));

    let button_count = app
        .world_mut()
        .query_filtered::<Entity, With<MetaScreenButton>>()
        .iter(app.world())
        .count();
    assert_eq!(button_count, 6);
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

    let mut app_camera_query = app
        .world_mut()
        .query_filtered::<(&Camera, &Transform), (With<AppSceneCamera>, With<Camera3d>)>();
    let (app_camera, camera_transform) = app_camera_query.single(app.world()).unwrap();
    assert!(app_camera.is_active);
    assert_eq!(
        camera_transform.translation.z,
        PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN
    );

    let mut light_query = app
        .world_mut()
        .query_filtered::<Entity, (With<DirectionalLight>, With<DeckSceneEntity>)>();
    assert_eq!(light_query.iter(app.world()).count(), 1);

    let (ui_camera_entity, ui_camera, egui_context, default_marker) = app
        .world_mut()
        .query_filtered::<(
            Entity,
            &Camera,
            Option<&PrimaryEguiContext>,
            Option<&IsDefaultUiCamera>,
        ), With<AppSceneCamera>>()
        .single(app.world())
        .unwrap();
    assert!(ui_camera.is_active);
    assert!(egui_context.is_none());
    assert!(default_marker.is_none());

    let point_text_camera_count = app
        .world_mut()
        .query_filtered::<Entity, (With<CardPointTextCamera>, With<DeckSceneEntity>)>()
        .iter(app.world())
        .count();
    assert_eq!(point_text_camera_count, 1);

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
        .query_filtered::<(Entity, &CardGrid), With<CardGrid>>();
    let (content_entity, content_grid) = content_query.single(app.world()).unwrap();
    assert_eq!(content_grid.title, "My Decks");
    assert_eq!(
        app.world()
            .get::<ChildOf>(content_entity)
            .map(ChildOf::parent),
        Some(deck_root)
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

    let mut deck_tile_node_query = app.world_mut().query::<(
        &Name,
        &Node,
        Option<&DeckView>,
        Option<&DeckScreenDeckCommandButton>,
    )>();
    let mut existing_deck_size = None;
    let mut new_deck_size = None;
    let mut new_deck_command = None;
    for (name, node, deck_view, deck_command) in deck_tile_node_query.iter(app.world()) {
        if deck_view.is_some() {
            existing_deck_size = Some((node.width, node.height));
        } else if name.as_str() == "DeckScreen + Deck Tile" {
            new_deck_size = Some((node.width, node.height));
            new_deck_command = deck_command.copied();
        }
    }
    assert_eq!(new_deck_size, existing_deck_size);
    assert_eq!(
        new_deck_command,
        Some(DeckScreenDeckCommandButton::EditDeckName)
    );
}

#[test]
fn new_deck_tile_uses_edit_name_coming_soon_prompt() {
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

    let new_deck_tile = app
        .world_mut()
        .query::<(Entity, &Name)>()
        .iter(app.world())
        .find_map(|(entity, name)| (name.as_str() == "DeckScreen + Deck Tile").then_some(entity))
        .unwrap();
    app.world_mut()
        .entity_mut(new_deck_tile)
        .insert(Interaction::Pressed);
    app.update();

    assert_eq!(
        app.world().resource::<DeckScreenModel>().mode,
        DeckScreenMode::DeckSelection
    );
    assert!(app.world().resource::<DeckScreenModel>().coming_soon_prompt);
    let prompt_labels: Vec<String> = app
        .world_mut()
        .query::<&Text>()
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect();
    assert!(
        prompt_labels
            .iter()
            .any(|text| text == DECK_SCREEN_COMING_SOON_TITLE)
    );
    assert!(
        prompt_labels
            .iter()
            .any(|text| text == DECK_SCREEN_COMING_SOON_MESSAGE)
    );
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
        .query_filtered::<(&Camera, &Transform), (With<AppSceneCamera>, With<Camera3d>)>();
    let (camera, camera_transform) = camera_query.single(app.world()).unwrap();
    assert!(camera.is_active);
    assert_eq!(
        camera_transform.translation.z,
        PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN
    );

    let mut light_query = app
        .world_mut()
        .query_filtered::<Entity, (With<DirectionalLight>, With<DebugSceneEntity>)>();
    assert_eq!(light_query.iter(app.world()).count(), 1);

    let point_text_camera_count = app
        .world_mut()
        .query_filtered::<Entity, (With<CardPointTextCamera>, With<DebugSceneEntity>)>()
        .iter(app.world())
        .count();
    assert_eq!(point_text_camera_count, 1);

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
    let card_translation = card_transform.translation;

    let mut location_query = app.world_mut().query_filtered::<(&Transform, &Children), (
        With<GameLocation>,
        With<DebugSceneEntity>,
        Without<CardView>,
    )>();
    let debug_locations: Vec<_> = location_query.iter(app.world()).collect();
    assert_eq!(debug_locations.len(), 1);
    let (location_transform, children) = debug_locations[0];
    assert!(
        location_transform.translation.x < card_translation.x,
        "debug location should sit to the left of the debug card"
    );
    assert!(children.iter().any(|child| {
        app.world()
            .entity(child)
            .get::<PointLocationView>()
            .is_some()
    }));
}

#[test]
fn debug_scene_card_matches_game_screen_card_size() {
    let card_defaults = CardInspectionDefaults::default();
    let transform = debug_scene_card_transform(&card_defaults, Quat::IDENTITY);
    let apparent_width =
        card_defaults.width * apparent_scale_at_z(transform.scale.x, transform.translation.z);
    let apparent_height =
        card_defaults.height * apparent_scale_at_z(transform.scale.y, transform.translation.z);

    assert_close(transform.translation.z, GAME_SCENE_HAND_CARD_WORLD_Z);
    assert_close(apparent_width, GAME_SCENE_HAND_CARD_WIDTH);
    assert_close(apparent_height, GAME_SCENE_HAND_CARD_HEIGHT);
}

#[test]
fn debug_scene_card_faces_debug_camera() {
    let card_defaults = CardInspectionDefaults::default();
    let camera_defaults = PrimaryCameraDefaults::default();
    let transform = debug_scene_initial_card_transform(&card_defaults, &camera_defaults);
    let mut camera_defaults = camera_defaults;
    camera_defaults.position.z = DECK_SCENE_CAMERA_DISTANCE_FROM_ORIGIN;
    let expected_normal = (camera_defaults.position - transform.translation).normalize();
    let actual_normal = transform.rotation * Vec3::Z;

    assert!(
        actual_normal.distance(expected_normal) < 0.000_1,
        "actual_normal={actual_normal:?} expected_normal={expected_normal:?}"
    );
}

#[test]
fn game_scene_card_depth_bands_are_closer_than_locations() {
    let card_depths = [
        GAME_SCENE_HAND_CARD_WORLD_Z,
        GAME_SCENE_HAND_CARD_WORLD_Z
            + (STARTING_HAND_CARD_COUNT as f32 * GAME_SCENE_HAND_CARD_Z_STEP),
        GAME_SCENE_HAND_CARD_HOVER_Z,
        card_gesture_animation_system::CARD_GESTURE_SLOT_Z,
        card_gesture_animation_system::CARD_GESTURE_SELECTED_Z,
        card_gesture_animation_system::CARD_GESTURE_DRAG_Z,
        CPU_CARD_MOVING_FRONT_Z,
    ];

    assert!(GAME_SCENE_WORLD_BACKGROUND_Z < GAME_SCENE_LOCATION_BUNDLE_Z);
    for depth in card_depths {
        assert!(
            GAME_SCENE_LOCATION_BUNDLE_Z < depth,
            "card depth {depth} must be closer to the camera than location depth {GAME_SCENE_LOCATION_BUNDLE_Z}"
        );
    }
}

#[test]
fn card_click_navigation_does_not_restart_from_debug_scene() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    assert!(card_click_navigation_restarts_game(ActiveView::DeckScene));
    assert!(!card_click_navigation_restarts_game(ActiveView::DebugScene));
    assert!(!card_click_navigation_restarts_game(ActiveView::GameScene));
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
    let card_defaults = app.world().resource::<CardInspectionDefaults>();
    let point_x =
        (card_defaults.width * 0.5) - (card_defaults.width * CARD_POINT_BADGE_INSET_RATIO);
    let point_center_x = point_x + (CARD_POINT_BADGE_SIZE * 0.5);
    let point_center_y =
        (card_defaults.height * 0.5) - (card_defaults.height * CARD_POINT_BADGE_INSET_RATIO);

    let mut energy_query = app
        .world_mut()
        .query::<(&Name, &PointView, &Visibility, &Transform)>();
    let energy_views: Vec<(String, i32, Visibility, Vec3)> = energy_query
        .iter(app.world())
        .filter_map(|(name, view, visibility, transform)| {
            (view.model.point_type == PointType::CardEnergy).then_some((
                name.to_string(),
                view.model.value,
                *visibility,
                transform.translation,
            ))
        })
        .collect();
    assert_eq!(
        energy_views,
        vec![(
            "Card EnergyPointView Background".to_string(),
            active_card.cost.value,
            Visibility::Visible,
            Vec3::new(point_center_x, point_center_y, energy_views[0].3.z),
        )]
    );

    let mut power_query = app
        .world_mut()
        .query_filtered::<(&Name, &PointView, &Visibility, &Transform), Without<GameLocation>>();
    let power_views: Vec<(String, i32, Visibility, Vec3)> = power_query
        .iter(app.world())
        .filter_map(|(name, view, visibility, transform)| {
            (view.model.point_type == PointType::CardPower).then_some((
                name.to_string(),
                view.model.value,
                *visibility,
                transform.translation,
            ))
        })
        .collect();
    assert_eq!(
        power_views,
        vec![(
            "Card PowerPointView Background".to_string(),
            active_card.base_power.value,
            Visibility::Visible,
            Vec3::new(-point_center_x, point_center_y, power_views[0].3.z),
        )]
    );

    let mut text_query = app.world_mut().query::<(
        &Name,
        &ChildOf,
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
        .filter_map(
            |(name, child_of, view, text, text_font, text_color, render_layers, visibility)| {
                if app
                    .world()
                    .entity(child_of.parent())
                    .get::<PointLocationView>()
                    .is_some()
                {
                    return None;
                }
                Some((
                    name.to_string(),
                    view.point_type,
                    text.0.clone(),
                    text_font.font_size,
                    text_color.0,
                    render_layers.clone(),
                    *visibility,
                ))
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
fn point_view_visual_raster_contains_visible_white_text_pixels() {
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

    let point_text_camera_layers = app
        .world_mut()
        .query_filtered::<&RenderLayers, (With<DebugSceneEntity>, With<CardPointTextCamera>)>()
        .single(app.world())
        .expect("DebugScene point text camera should render point text");
    assert_eq!(
        *point_text_camera_layers,
        RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER)
    );

    let (point_entity, text, text_font, text_color, text_layers, text_visibility) = {
        let mut text_query = app.world_mut().query::<(
            &Name,
            &ChildOf,
            &Text2d,
            &TextFont,
            &TextColor,
            &RenderLayers,
            &Visibility,
        )>();
        text_query
            .iter(app.world())
            .find_map(
                |(name, child_of, text, text_font, text_color, render_layers, visibility)| {
                    (name.as_str() == "Card PowerPointView Background Text").then_some((
                        child_of.parent(),
                        text.0.clone(),
                        text_font.clone(),
                        text_color.0,
                        render_layers.clone(),
                        *visibility,
                    ))
                },
            )
            .expect("card power point text should exist")
    };
    assert_eq!(
        text_layers,
        RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER)
    );
    assert_ne!(text_visibility, Visibility::Hidden);

    let point_view = app
        .world()
        .get::<PointView>(point_entity)
        .expect("point text should be parented under a point view");
    let point_visibility = app
        .world()
        .get::<Visibility>(point_entity)
        .expect("point view should have visibility");
    assert_eq!(*point_visibility, Visibility::Visible);
    let visible_text_color = TestRgbaPixel::from_color(text_color);
    assert!(visible_text_color.is_visible_white());

    let visual_image = render_point_view_visual_test_image(
        point_view.model,
        &text,
        text_color,
        text_font.font_size,
    );
    let white_pixel_count = visual_image.white_pixel_count();
    assert!(
        white_pixel_count >= 100,
        "expected visible white text pixels, found {white_pixel_count}"
    );
}

#[test]
fn location_point_view_visual_raster_contains_visible_white_text_pixels() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_resource::<Touches>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .add_systems(Startup, setup_game_scene)
        .add_systems(
            Update,
            (
                location_intro_update_system,
                update_card_point_text2d_overlay_system,
            )
                .chain(),
        );

    app.update();
    for mut transition in app
        .world_mut()
        .query::<&mut WorldThemeTransition>()
        .iter_mut(app.world_mut())
    {
        transition.phase = WorldThemeTransitionPhase::Idle;
    }
    for mut intro in app
        .world_mut()
        .query::<&mut LocationBundleIntro>()
        .iter_mut(app.world_mut())
    {
        intro.elapsed_seconds = 4.0;
    }
    app.update();

    let point_text_camera_layers = app
        .world_mut()
        .query_filtered::<&RenderLayers, (With<GameSceneEntity>, With<CardPointTextCamera>)>()
        .single(app.world())
        .expect("GameScene point text camera should render location point text");
    assert_eq!(
        *point_text_camera_layers,
        RenderLayers::layer(CARD_POINT_TEXT_RENDER_LAYER)
    );

    let (
        point_entity,
        text,
        text_font,
        text_color,
        render_layers,
        text_visibility,
        point_text_view,
    ) = {
        let mut text_query = app.world_mut().query::<(
            &Name,
            &ChildOf,
            Option<&CardPointTextView>,
            &Text2d,
            &TextFont,
            &TextColor,
            Option<&RenderLayers>,
            &Visibility,
        )>();
        text_query
            .iter(app.world())
            .find_map(
                |(
                    name,
                    child_of,
                    point_text_view,
                    text,
                    text_font,
                    text_color,
                    render_layers,
                    visibility,
                )| {
                    if name.as_str() != "PowerPointView Text" {
                        return None;
                    }
                    let point_entity = child_of.parent();
                    app.world()
                        .get::<PointLocationView>(point_entity)
                        .is_some()
                        .then_some((
                            point_entity,
                            text.0.clone(),
                            text_font.clone(),
                            text_color.0,
                            render_layers.cloned(),
                            *visibility,
                            point_text_view.copied(),
                        ))
                },
            )
            .expect("location point text should exist")
    };
    assert!(render_layers.is_none());
    assert_ne!(text_visibility, Visibility::Hidden);
    assert_eq!(point_text_view, None);

    let point_view = app
        .world()
        .get::<PointView>(point_entity)
        .expect("location point text should be parented under a point view");
    assert_eq!(point_view.model.point_type, PointType::LocationPower);
    assert_eq!(text_font.font_size, LOCATION_POINT_TEXT_FONT_SIZE);
    let visible_text_color = TestRgbaPixel::from_color(text_color);
    assert!(visible_text_color.is_visible_white());

    let visual_image = render_point_view_visual_test_image(
        point_view.model,
        &text,
        text_color,
        text_font.font_size,
    );
    let white_pixel_count = visual_image.white_pixel_count();
    assert!(
        white_pixel_count >= 40,
        "expected visible white location point text pixels, found {white_pixel_count}"
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
fn deck_screen_editor_selects_card_tiles_and_shows_action_menu() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<Touches>()
        .init_resource::<ActiveView>()
        .init_resource::<PrimaryCameraDefaults>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<DeckScreenModel>()
        .init_resource::<SelectedCardModalModel>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CardFlipState>()
        .init_resource::<TopNavigationModel>()
        .init_resource::<PlayerDeckCollectionModel>()
        .add_systems(Startup, setup_deck_scene)
        .add_systems(
            Update,
            (card_selection_update_system, deck_screen_update_system).chain(),
        );
    let _window = spawn_test_primary_window(&mut app);

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
    app.update();

    assert_eq!(
        app.world().resource::<DeckScreenModel>().mode,
        DeckScreenMode::Editor
    );
    assert_eq!(
        deck_screen_deck_cards(app.world().resource::<PlayerDeckCollectionModel>()).len(),
        DECK_SCREEN_VISIBLE_CARD_COUNT
    );
    let deck_card_view_count = app
        .world_mut()
        .query_filtered::<Entity, (With<CardView>, With<DeckScreenCardView>)>()
        .iter(app.world())
        .count();
    assert_eq!(deck_card_view_count, DECK_SCREEN_VISIBLE_CARD_COUNT + 3);
    let deck_card_view_metadata_count = app
        .world_mut()
        .query::<&DeckScreenCardView>()
        .iter(app.world())
        .filter(|view| view.zone == DeckEditableZoneModel::Deck)
        .count();
    assert_eq!(
        deck_card_view_metadata_count,
        DECK_SCREEN_VISIBLE_CARD_COUNT
    );
    let deck_column_card_view_count = deck_screen_card_views_right_of(&mut app, 600.0);
    assert_eq!(deck_column_card_view_count, 3);
    assert_matching_deck_screen_grid_backdrops(&mut app);
    assert_deck_screen_grid_backdrops_are_behind_cards(&mut app);
    let grid_titles: Vec<String> = app
        .world_mut()
        .query::<&CardGrid>()
        .iter(app.world())
        .map(|grid| grid.title.clone())
        .collect();
    assert_eq!(
        grid_titles,
        vec!["Deck 01".to_string(), "Not In Deck".to_string()]
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenContentRoot>>()
            .iter(app.world())
            .count(),
        0
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<GridViewMenuArea>>()
            .iter(app.world())
            .count(),
        2
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenDeckCommandButton>>()
            .iter(app.world())
            .count(),
        2
    );
    for command in [
        DeckScreenDeckCommandButton::EditDeckName,
        DeckScreenDeckCommandButton::DeleteDeck,
    ] {
        let command_entity = app
            .world_mut()
            .query::<(Entity, &DeckScreenDeckCommandButton)>()
            .iter(app.world())
            .find_map(|(entity, button)| (*button == command).then_some(entity))
            .unwrap();
        app.world_mut()
            .entity_mut(command_entity)
            .insert(Interaction::Pressed);
        app.update();

        assert!(app.world().resource::<DeckScreenModel>().coming_soon_prompt);
        let prompt_labels: Vec<String> = app
            .world_mut()
            .query::<&Text>()
            .iter(app.world())
            .map(|text| text.0.clone())
            .collect();
        assert!(
            prompt_labels
                .iter()
                .any(|text| text == DECK_SCREEN_COMING_SOON_TITLE)
        );
        assert!(
            prompt_labels
                .iter()
                .any(|text| text == DECK_SCREEN_COMING_SOON_MESSAGE)
        );

        let ok_entity = app
            .world_mut()
            .query_filtered::<Entity, With<DeckScreenValidationOkButton>>()
            .single(app.world())
            .unwrap();
        app.world_mut()
            .entity_mut(ok_entity)
            .insert(Interaction::Pressed);
        app.update();

        assert!(!app.world().resource::<DeckScreenModel>().coming_soon_prompt);
    }
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenTabButton>>()
            .iter(app.world())
            .count(),
        2
    );
    let tab_button_styles: Vec<(DeckEditorTabModel, BackgroundColor, BorderColor)> = app
        .world_mut()
        .query::<(&DeckScreenTabButton, &BackgroundColor, &BorderColor)>()
        .iter(app.world())
        .map(|(button, background, border)| (button.tab, *background, *border))
        .collect();
    let library_style = tab_button_styles
        .iter()
        .find(|(tab, _, _)| *tab == DeckEditorTabModel::Library)
        .unwrap();
    let shop_style = tab_button_styles
        .iter()
        .find(|(tab, _, _)| *tab == DeckEditorTabModel::Shop)
        .unwrap();
    assert_ne!(library_style.1, shop_style.1);
    assert_ne!(library_style.2, shop_style.2);

    let shop_entity = app
        .world_mut()
        .query::<(Entity, &DeckScreenTabButton)>()
        .iter(app.world())
        .find_map(|(entity, button)| (button.tab == DeckEditorTabModel::Shop).then_some(entity))
        .unwrap();
    app.world_mut()
        .entity_mut(shop_entity)
        .insert(Interaction::Pressed);
    app.update();

    assert_eq!(
        app.world().resource::<DeckScreenModel>().editor_tab,
        DeckEditorTabModel::Library
    );
    assert!(app.world().resource::<DeckScreenModel>().coming_soon_prompt);
    let ok_entity = app
        .world_mut()
        .query_filtered::<Entity, With<DeckScreenValidationOkButton>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(ok_entity)
        .insert(Interaction::Pressed);
    app.update();
    assert!(!app.world().resource::<DeckScreenModel>().coming_soon_prompt);
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, (With<DeckScreenCardView>, With<SelectableCard>)>()
            .iter(app.world())
            .count(),
        DECK_SCREEN_VISIBLE_CARD_COUNT + 3
    );
    assert!(
        app.world_mut()
            .query_filtered::<&Pickable, With<GridViewContentArea>>()
            .iter(app.world())
            .all(|pickable| *pickable == Pickable::IGNORE)
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenModalActionButton>>()
            .iter(app.world())
            .count(),
        0
    );
    assert_eq!(
        deck_screen_library_cards(&deck_screen_deck_cards(
            app.world().resource::<PlayerDeckCollectionModel>()
        ))
        .len(),
        3
    );
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DeckScene;
    app.update();

    let (first_deck_card, source_transform) = app
        .world_mut()
        .query_filtered::<(Entity, &DeckScreenCardView, &Transform), With<CardView>>()
        .iter(app.world())
        .find_map(|(entity, view, transform)| {
            (view.zone == DeckEditableZoneModel::Deck).then_some((entity, *transform))
        })
        .unwrap();
    let target_transform =
        selected_inspection_transform(app.world().resource::<CardInspectionDefaults>());
    app.world_mut()
        .resource_mut::<SelectedCardModalModel>()
        .select_entity(first_deck_card, source_transform, target_transform);
    app.update();

    assert!(app.world().resource::<SelectedCardModalModel>().is_active());
    assert!(app.world().resource::<DeckScreenModel>().modal.is_some());
    assert_eq!(
        app.world()
            .resource::<SelectedCardModalModel>()
            .selected_entity,
        Some(first_deck_card)
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenSelectedCardMenuRoot>>()
            .iter(app.world())
            .count(),
        1
    );
    let menu_labels: Vec<String> = app
        .world_mut()
        .query::<&Text>()
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect();
    for label in ["Move to Library", "Move to Deck", "Transfer", "Back"] {
        assert!(menu_labels.iter().any(|text| text == label));
    }

    let transfer_entity = app
        .world_mut()
        .query::<(Entity, &DeckScreenModalActionButton)>()
        .iter(app.world())
        .find_map(|(entity, action)| {
            (*action == DeckScreenModalActionButton::TransferOut).then_some(entity)
        })
        .unwrap();
    app.world_mut()
        .entity_mut(transfer_entity)
        .insert(Interaction::Pressed);
    app.update();

    assert!(app.world().resource::<DeckScreenModel>().modal.is_none());
    assert!(app.world().resource::<DeckScreenModel>().coming_soon_prompt);
    let ok_entity = app
        .world_mut()
        .query_filtered::<Entity, With<DeckScreenValidationOkButton>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(ok_entity)
        .insert(Interaction::Pressed);
    app.update();
    assert!(!app.world().resource::<DeckScreenModel>().coming_soon_prompt);
    app.world_mut()
        .resource_mut::<SelectedCardModalModel>()
        .clear();
    app.update();

    assert!(app.world().resource::<DeckScreenModel>().modal.is_none());
    assert!(!app.world().resource::<SelectedCardModalModel>().is_active());

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
    assert_eq!(deck_cards.len(), DECK_SCREEN_VISIBLE_CARD_COUNT - 1);
    assert_eq!(library_cards.len(), 4);
    assert!(app.world().resource::<DeckScreenModel>().modal.is_none());
    let library_column_card_view_count = deck_screen_card_views_right_of(&mut app, 600.0);
    assert_eq!(library_column_card_view_count, 4);
    assert_matching_deck_screen_grid_backdrops(&mut app);
    let library_card_view_metadata_count = app
        .world_mut()
        .query::<&DeckScreenCardView>()
        .iter(app.world())
        .filter(|view| view.zone == DeckEditableZoneModel::Library)
        .count();
    assert_eq!(library_card_view_metadata_count, 4);
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

fn assert_deck_screen_grid_backdrops_are_behind_cards(app: &mut App) {
    let min_card_z = app
        .world_mut()
        .query_filtered::<&Transform, (With<CardView>, With<DeckScreenCardView>)>()
        .iter(app.world())
        .map(|transform| transform.translation.z)
        .fold(f32::INFINITY, f32::min);
    let max_backdrop_z = app
        .world_mut()
        .query_filtered::<&Transform, With<DeckScreenGridBackdrop>>()
        .iter(app.world())
        .map(|transform| transform.translation.z)
        .fold(f32::NEG_INFINITY, f32::max);

    assert!(max_backdrop_z < min_card_z);
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
fn game_scene_ui_camera_waits_for_active_view_sync_before_global_ui_ownership() {
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
        .add_systems(Startup, setup_game_scene);

    app.update();

    let (_, ui_camera, egui_context, default_marker) = app
        .world_mut()
        .query_filtered::<(
            Entity,
            &Camera,
            Option<&PrimaryEguiContext>,
            Option<&IsDefaultUiCamera>,
        ), With<AppSceneCamera>>()
        .single(app.world())
        .unwrap();
    assert!(ui_camera.is_active);
    assert!(egui_context.is_none());
    assert!(default_marker.is_none());
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
        .add_systems(Update, app_camera_update_system);
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
    let cameras: Vec<&Camera> = camera_query.iter(app.world()).collect();
    assert_eq!(cameras.len(), 1);
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
        .add_systems(Update, app_camera_update_system);
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
fn deck_scene_card_point_text_camera_uses_centered_safe_area_viewport() {
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
        .add_systems(Startup, setup_deck_scene)
        .add_systems(Update, app_camera_update_system);
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
        .query_filtered::<&Camera, (With<DeckSceneEntity>, With<CardPointTextCamera>)>();
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
fn debug_scene_card_point_text_camera_uses_centered_safe_area_viewport() {
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
        .add_systems(Startup, setup_debug_scene)
        .add_systems(Update, app_camera_update_system);
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
        .query_filtered::<&Camera, (With<DebugSceneEntity>, With<CardPointTextCamera>)>();
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
        .add_systems(Update, app_camera_update_system);
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
        .query_filtered::<&Camera, (With<AppSceneCamera>, With<Camera3d>)>();
    let cameras: Vec<&Camera> = camera_query.iter(app.world()).collect();
    assert_eq!(cameras.len(), 1);
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
        .query_filtered::<Entity, (With<AppSceneCamera>, With<Camera3d>)>();
    let app_camera_entities: Vec<Entity> = camera_query.iter(app.world()).collect();
    assert_eq!(app_camera_entities.len(), 1);
    let app_camera_entity = app_camera_entities[0];

    let mut game_root_query = app
        .world_mut()
        .query_filtered::<&UiTargetCamera, With<GameSceneRoot>>();
    assert_eq!(
        *game_root_query.single(app.world()).unwrap(),
        UiTargetCamera(app_camera_entity)
    );

    let mut top_nav_query = app
        .world_mut()
        .query_filtered::<Entity, With<TopNavigationRoot>>();
    assert_eq!(top_nav_query.iter(app.world()).count(), 0);

    let mut light_query = app
        .world_mut()
        .query_filtered::<Entity, (With<DirectionalLight>, With<GameSceneEntity>)>();
    assert_eq!(light_query.iter(app.world()).count(), 0);

    let mut background_query = app.world_mut().query_filtered::<(
        &Name,
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &Transform,
    ), With<WorldBackground>>();
    let (background_name, background_mesh, background_material, background_transform) =
        background_query.single(app.world()).unwrap();
    let background_name = background_name.to_string();
    let background_mesh = background_mesh.0.clone();
    let background_material = background_material.0.clone();
    let background_z = background_transform.translation.z;

    let active_world_name = app
        .world()
        .resource::<WorldModelRegistry>()
        .active_world_model(app.world().resource::<ActiveWorldModel>())
        .display_name;
    assert_eq!(
        background_name,
        format!("{active_world_name} World Background")
    );
    let background_mesh = app
        .world()
        .resource::<Assets<Mesh>>()
        .get(&background_mesh)
        .unwrap();
    let (background_width, background_height) =
        mesh_bounds(background_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
    let expected_background_size = game_scene_world_background_size();
    let full_camera_view_height = 2.0
        * (PRIMARY_CAMERA_FOV_RADIANS * 0.5).tan()
        * (PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN - GAME_SCENE_WORLD_BACKGROUND_Z).abs();
    let full_camera_view_size = Vec2::new(
        full_camera_view_height * GAME_SCENE_ASPECT_RATIO,
        full_camera_view_height,
    );
    assert_close(background_width, expected_background_size.x);
    assert_close(background_height, expected_background_size.y);
    assert_eq!(
        expected_background_size,
        game_scene_perspective_view_size_at_z(GAME_SCENE_WORLD_BACKGROUND_Z)
    );
    assert_vec2_close(expected_background_size, full_camera_view_size);
    assert_close(background_z, GAME_SCENE_WORLD_BACKGROUND_Z);
    let background_material = app
        .world()
        .resource::<Assets<StandardMaterial>>()
        .get(&background_material)
        .unwrap();
    assert!(background_material.unlit);

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
    let mut location_surface_query = app.world_mut().query::<(
        &LocationBundleSurface,
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &Transform,
    )>();
    let mut location_surfaces: Vec<(usize, Handle<Mesh>, Handle<StandardMaterial>, Transform)> =
        location_surface_query
            .iter(app.world())
            .map(|(surface, mesh, material, transform)| {
                (
                    surface.location_index,
                    mesh.0.clone(),
                    material.0.clone(),
                    *transform,
                )
            })
            .collect();
    location_surfaces.sort_by_key(|(index, ..)| *index);
    assert!(location_surfaces.iter().all(|(_, _, material, transform)| {
        transform.translation.z > background_z
            && app
                .world()
                .resource::<Assets<StandardMaterial>>()
                .get(material)
                .is_some_and(|material| material.unlit)
    }));
    assert_eq!(location_surfaces.len(), 3);
    for (location_index, mesh, material, transform) in location_surfaces {
        let area_rect = slot_board.location_area_rect(location_index).unwrap();
        let bundle_size = LocationViewBundle::scaled_size(area_rect);
        let mesh = app.world().resource::<Assets<Mesh>>().get(&mesh).unwrap();
        let (surface_width, surface_height) =
            mesh_bounds(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        assert_close(
            surface_width,
            game_scene_world_width_for_game_scene_width(
                bundle_size.x,
                GAME_SCENE_LOCATION_BUNDLE_Z,
            ),
        );
        assert_close(
            surface_height,
            game_scene_world_height_for_game_scene_height(
                bundle_size.y,
                GAME_SCENE_LOCATION_BUNDLE_Z,
            ),
        );
        assert_vec2_close(
            game_scene_position_from_world_position(transform.translation),
            Vec2::new(
                area_rect.left + (area_rect.width * 0.5),
                area_rect.top + (area_rect.height * 0.5),
            ),
        );
        assert_close(transform.translation.z, GAME_SCENE_LOCATION_BUNDLE_Z);
        assert!(GAME_SCENE_WORLD_BACKGROUND_Z < transform.translation.z);
        assert!(transform.translation.z < GAME_SCENE_HAND_CARD_WORLD_Z);
        assert_close(transform.scale.x, 1.5);
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material)
                .unwrap()
                .base_color
                .to_srgba()
                .alpha,
            0.0
        );
    }

    let mut location_mesh_query = app.world_mut().query::<(
        &GameLocation,
        &LocationBundleSurface,
        &LocationBundleOverlay,
        &Mesh3d,
        &Transform,
        &Children,
    )>();
    let mut location_layouts: Vec<(usize, Vec3, Vec<Entity>)> = location_mesh_query
        .iter(app.world())
        .map(|(location, surface, overlay, _, transform, children)| {
            assert_eq!(location.index, surface.location_index);
            assert_eq!(location.index, overlay.location_index);
            (location.index, transform.scale, children.iter().collect())
        })
        .collect();
    location_layouts.sort_by_key(|(index, ..)| *index);
    assert_eq!(location_layouts.len(), 3);

    for (location_index, scale, children) in location_layouts {
        assert_eq!(scale, Vec3::splat(1.5));

        let mut border_mesh_count = 0;
        for child in children {
            let entity = app.world().entity(child);
            let Some(name) = entity.get::<Name>() else {
                continue;
            };
            if name
                .as_str()
                .starts_with(&format!("Game Location Border {location_index}"))
            {
                assert!(entity.get::<Mesh3d>().is_some());
                assert!(entity.get::<MeshMaterial3d<StandardMaterial>>().is_some());
                assert!(entity.get::<Transform>().is_some());
                assert!(entity.get::<Node>().is_none());
                border_mesh_count += 1;
            }
        }

        assert_eq!(border_mesh_count, 4);
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

    let mut location_power_mesh_query =
        app.world_mut()
            .query::<(&PointLocationView, &Transform, Option<&Node>, &Children)>();
    for (location_power_view, transform, node, children) in
        location_power_mesh_query.iter(app.world())
    {
        let area_rect = slot_board
            .location_area_rect(location_power_view.location_index)
            .unwrap();
        let bundle_size = LocationViewBundle::scaled_size(area_rect);
        let location_world_height = game_scene_world_height_for_game_scene_height(
            bundle_size.y,
            GAME_SCENE_LOCATION_BUNDLE_Z,
        );
        let point_world_height = game_scene_world_height_for_game_scene_height(
            LOCATION_POINT_VIEW_HEIGHT,
            GAME_SCENE_LOCATION_BUNDLE_Z,
        );

        assert!(node.is_none());
        assert_close(transform.translation.x, 0.0);
        match location_power_view.side {
            CardSlotSide::Opponent => {
                assert_close(transform.translation.y, location_world_height * 0.5);
            }
            CardSlotSide::LocalPlayer => {
                assert_close(transform.translation.y, -location_world_height * 0.5);
            }
        }
        assert_close(transform.translation.z, LAYER_RENDER_Z_STEP);

        let mut circle_mesh = None;
        let mut circle_material = None;
        let mut has_text2d = false;
        for child in children {
            let entity = app.world().entity(*child);
            let Some(name) = entity.get::<Name>() else {
                continue;
            };
            if name.as_str() == "PowerPointView Circle" {
                circle_mesh = entity.get::<Mesh3d>();
                circle_material = entity.get::<MeshMaterial3d<StandardMaterial>>();
                assert!(entity.get::<Node>().is_none());
                assert!(entity.get::<BackgroundColor>().is_none());
            }
            if name.as_str() == "PowerPointView Text" {
                assert!(entity.get::<Text2d>().is_some());
                assert!(entity.get::<Text>().is_none());
                has_text2d = true;
            }
        }

        let circle_mesh = app
            .world()
            .resource::<Assets<Mesh>>()
            .get(&circle_mesh.unwrap().0)
            .unwrap();
        let (circle_width, circle_height) =
            mesh_bounds(circle_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap());
        assert_close(
            circle_width,
            game_scene_world_width_for_game_scene_width(
                LOCATION_POINT_VIEW_WIDTH,
                GAME_SCENE_LOCATION_BUNDLE_Z,
            ),
        );
        assert_close(circle_height, point_world_height);
        let circle_material = app
            .world()
            .resource::<Assets<StandardMaterial>>()
            .get(&circle_material.unwrap().0)
            .unwrap();
        assert_eq!(
            circle_material.base_color,
            color_with_alpha(PointModel::location_power(0).background_color(), 0.0)
        );
        assert!(has_text2d);
    }

    let mut hand_query = app
        .world_mut()
        .query_filtered::<(Entity, &Node), With<LocalPlayerHand>>();
    let hands: Vec<(Entity, &Node)> = hand_query.iter(app.world()).collect();
    assert_eq!(hands.len(), 1);
    let (hand_entity, hand_node) = hands[0];
    assert_eq!(hand_node.border, UiRect::ZERO);
    assert_eq!(
        app.world().get::<BorderColor>(hand_entity),
        Some(&BorderColor::all(Color::NONE))
    );
    assert_eq!(
        app.world().get::<BackgroundColor>(hand_entity),
        Some(&BackgroundColor(Color::NONE))
    );

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
        .query_filtered::<&Camera, (With<AppSceneCamera>, With<Camera3d>)>();
    let mut preview_camera_orders: Vec<isize> = preview_camera_query
        .iter(app.world())
        .map(|camera| camera.order)
        .collect();
    preview_camera_orders.sort();
    assert_eq!(preview_camera_orders, vec![0]);

    let ui_camera_count = app
        .world_mut()
        .query_filtered::<Entity, (
            With<Camera2d>,
            With<GameSceneEntity>,
            Without<CardPointTextCamera>,
        )>()
        .iter(app.world())
        .count();
    assert_eq!(ui_camera_count, 0);
}

#[test]
fn location_intro_reveals_locations_in_sequence() {
    let location_01_start = LocationBundleIntroSample::at(0, 0.0);
    assert_close(location_01_start.opacity, 0.0);
    assert_close(location_01_start.scale, 1.5);

    let location_01_mid = LocationBundleIntroSample::at(0, 0.25);
    assert!(location_01_mid.opacity > 0.5);
    assert!(location_01_mid.scale < 1.25);

    let location_01_end = LocationBundleIntroSample::at(0, 0.5);
    assert_close(location_01_end.opacity, 1.0);
    assert_close(location_01_end.scale, 1.0);

    let location_02_before_wait = LocationBundleIntroSample::at(1, 0.99);
    assert_close(location_02_before_wait.opacity, 0.0);
    assert_close(location_02_before_wait.scale, 1.5);

    let location_02_start = LocationBundleIntroSample::at(1, 1.0);
    assert_close(location_02_start.opacity, 0.0);
    assert_close(location_02_start.scale, 1.5);

    let location_02_end = LocationBundleIntroSample::at(1, 1.5);
    assert_close(location_02_end.opacity, 1.0);
    assert_close(location_02_end.scale, 1.0);

    let location_03_before_wait = LocationBundleIntroSample::at(2, 1.99);
    assert_close(location_03_before_wait.opacity, 0.0);
    assert_close(location_03_before_wait.scale, 1.5);

    let location_03_start = LocationBundleIntroSample::at(2, 2.0);
    assert_close(location_03_start.opacity, 0.0);
    assert_close(location_03_start.scale, 1.5);

    let location_03_end = LocationBundleIntroSample::at(2, 2.5);
    assert_close(location_03_end.opacity, 1.0);
    assert_close(location_03_end.scale, 1.0);
}

#[test]
fn location_intro_update_system_finishes_surface_overlay_and_visual_opacity() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(Update, location_intro_update_system);

    let mut material_handles = Vec::new();
    for location_index in 0..3 {
        let material = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            });
        material_handles.push(material.clone());
        let completed_intro = LocationBundleIntro {
            location_index,
            elapsed_seconds: 4.0,
        };
        app.world_mut().spawn((
            LocationBundleSurface::new(location_index),
            completed_intro,
            MeshMaterial3d(material),
            Transform::from_scale(Vec3::splat(1.5)),
        ));
        app.world_mut().spawn((
            LocationBundleOverlay::new(location_index),
            completed_intro,
            Transform::from_scale(Vec3::splat(1.5)),
        ));
        app.world_mut().spawn((
            LocationBundleIntroVisual::new(location_index),
            BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.0)),
            BorderColor::all(Color::srgba(0.0, 1.0, 0.0, 0.0)),
            TextColor(Color::srgba(0.0, 0.0, 1.0, 0.0)),
        ));
    }

    app.update();

    for material in material_handles {
        assert_close(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material)
                .unwrap()
                .base_color
                .to_srgba()
                .alpha,
            1.0,
        );
    }

    let mut intro_query = app
        .world_mut()
        .query::<(&LocationBundleIntro, &Transform)>();
    for (intro, transform) in intro_query.iter(app.world()) {
        assert_close(intro.sample().opacity, 1.0);
        assert_eq!(transform.scale, Vec3::ONE);
    }

    let mut visual_query = app.world_mut().query::<(
        &BackgroundColor,
        &BorderColor,
        &TextColor,
        &LocationBundleIntroVisual,
    )>();
    for (background, border, text, _) in visual_query.iter(app.world()) {
        assert_close(background.0.to_srgba().alpha, 1.0);
        assert_close(border.top.to_srgba().alpha, 1.0);
        assert_close(border.right.to_srgba().alpha, 1.0);
        assert_close(border.bottom.to_srgba().alpha, 1.0);
        assert_close(border.left.to_srgba().alpha, 1.0);
        assert_close(text.0.to_srgba().alpha, 1.0);
    }
}

#[test]
fn location_intro_does_not_turn_border_overlay_background_black() {
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
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, location_intro_update_system);

    app.update();
    for mut intro in app
        .world_mut()
        .query::<&mut LocationBundleIntro>()
        .iter_mut(app.world_mut())
    {
        intro.elapsed_seconds = 4.0;
    }
    app.update();

    let mut border_query = app.world_mut().query::<(
        &GameLocationBorder,
        Option<&BackgroundColor>,
        &MeshMaterial3d<StandardMaterial>,
    )>();
    let borders: Vec<_> = border_query.iter(app.world()).collect();
    assert_eq!(borders.len(), 12);
    for (_, background, material) in borders {
        if let Some(background) = background {
            assert_eq!(background.0, Color::NONE);
        }
        let material = app
            .world()
            .resource::<Assets<StandardMaterial>>()
            .get(&material.0)
            .unwrap();
        assert_ne!(material.base_color, Color::BLACK);
    }

    let mut point_circle_query = app.world_mut().query::<(
        &PointViewCircle,
        &MeshMaterial3d<StandardMaterial>,
        &LocationBundleIntroVisual,
    )>();
    let circles: Vec<_> = point_circle_query.iter(app.world()).collect();
    assert_eq!(circles.len(), 6);
    for (_, material, _) in circles {
        let material = app
            .world()
            .resource::<Assets<StandardMaterial>>()
            .get(&material.0)
            .unwrap();
        let actual = material.base_color.to_srgba();
        let expected = PointModel::location_power(0).background_color().to_srgba();
        assert_close(actual.red, expected.red);
        assert_close(actual.green, expected.green);
        assert_close(actual.blue, expected.blue);
        assert_ne!(material.base_color, Color::BLACK);
    }
}

#[test]
fn first_round_hand_waits_until_location_intro_completes() {
    let mut app = App::new();
    let frame_step = std::time::Duration::from_millis(100);
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(frame_step))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<Touches>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardInspectionState>()
        .init_resource::<CardFlipState>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .init_resource::<WorldModelRegistry>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<MatchModel>()
        .init_resource::<PendingRoundDealResource>()
        .init_resource::<CardStateModel>()
        .init_resource::<CardGestureModel>()
        .add_systems(Startup, setup_game_scene)
        .add_systems(
            Update,
            (
                world_theme_transition_update_system,
                location_intro_update_system,
                start_pending_round_deal_after_round_start_system
                    .after(location_intro_update_system),
                sync_game_scene_hand_card_entities_system
                    .after(start_pending_round_deal_after_round_start_system),
                card_gesture_animation_system.after(sync_game_scene_hand_card_entities_system),
                round_deal_completion_update_system.after(card_gesture_animation_system),
            ),
        );

    app.update();
    assert_eq!(app.world().resource::<GameHandModel>().len(), 0);
    let mut preview_query = app.world_mut().query_filtered::<Entity, (
        With<LocalPlayerHandCardPreview>,
        With<CardView>,
        With<GameSceneEntity>,
        Without<DeckSceneEntity>,
    )>();
    assert_eq!(preview_query.iter(app.world()).count(), 0);

    let mut intro_query = app.world_mut().query::<&mut LocationBundleIntro>();
    for mut intro in intro_query.iter_mut(app.world_mut()) {
        intro.elapsed_seconds = 2.99;
    }
    app.update();
    for intro in app
        .world_mut()
        .query::<&LocationBundleIntro>()
        .iter(app.world())
    {
        assert_eq!(intro.elapsed_seconds, 2.99);
    }
    assert_eq!(app.world().resource::<GameHandModel>().len(), 0);
    assert_eq!(preview_query.iter(app.world()).count(), 0);
    assert_eq!(
        app.world()
            .resource::<PendingRoundDealResource>()
            .location_intro_completed_event_count,
        0
    );
    assert_eq!(
        app.world()
            .resource::<PendingRoundDealResource>()
            .near_deal_completed_event_count,
        0
    );

    let mut intro_query = app.world_mut().query::<&mut LocationBundleIntro>();
    for mut intro in intro_query.iter_mut(app.world_mut()) {
        intro.elapsed_seconds = 0.0;
    }
    update_until_world_transition_idle(&mut app);

    let mut intro_query = app.world_mut().query::<&mut LocationBundleIntro>();
    for mut intro in intro_query.iter_mut(app.world_mut()) {
        intro.elapsed_seconds = 3.01;
    }
    app.update();
    let first_round_deal_count = GameRoundModel::default().requested_cards_to_deal;
    assert_eq!(
        app.world().resource::<GameHandModel>().len(),
        first_round_deal_count
    );
    assert_eq!(
        preview_query.iter(app.world()).count(),
        first_round_deal_count
    );
    let pending_round_deal = app.world().resource::<PendingRoundDealResource>();
    assert_eq!(pending_round_deal.location_intro_completed_event_count, 1);
    assert!(pending_round_deal.last_location_intro_completed_elapsed_ms >= 3000);
    assert!(!pending_round_deal.is_round_deal_complete);
    assert_eq!(pending_round_deal.near_deal_completed_event_count, 1);
    assert_eq!(
        pending_round_deal.last_near_deal_completed_card_count,
        first_round_deal_count
    );

    for _ in 0..20 {
        app.update();
        if app
            .world()
            .resource::<PendingRoundDealResource>()
            .is_round_deal_complete
        {
            return;
        }
    }

    panic!("round deal visuals did not complete");
}

#[test]
fn game_scene_restart_does_not_deal_round_one_before_location_intro() {
    let mut gesture_model = CardGestureModel::default();
    let mut slot_board = CardSlotBoardModel::default();
    let mut card_states = CardStateModel::default();
    let mut game_deck_model = GameDeckModel::default();
    let mut game_hand_model = GameHandModel::default();
    let mut game_round_model = GameRoundModel::default();
    let mut game_location_model = GameLocationModel::default();
    let location_model_registry = LocationModelRegistry::default();
    let mut active_locations = ActiveLocations::default();
    let mut active_world_model = ActiveWorldModel::default();
    let world_model_registry = WorldModelRegistry::default();
    let mut match_model = MatchModel::default();
    let player_deck_collection = PlayerDeckCollectionModel::default();
    let mut cpu_brain_model = CpuBrainModel::default();
    let mut pending_round_deal = PendingRoundDealResource::default();

    restart_game_model(
        Some(&mut gesture_model),
        Some(&mut slot_board),
        Some(&mut card_states),
        Some(&mut game_deck_model),
        Some(&mut game_hand_model),
        Some(&mut game_round_model),
        Some(&mut game_location_model),
        Some(&location_model_registry),
        Some(&mut active_locations),
        Some(&mut active_world_model),
        Some(&world_model_registry),
        Some(&mut match_model),
        Some(&player_deck_collection),
        Some(&mut cpu_brain_model),
        Some(&mut pending_round_deal),
    );

    assert_eq!(
        game_hand_model.len(),
        0,
        "near hand must stay empty until location intro sequence completes"
    );
    assert_eq!(
        card_states.len(),
        0,
        "card state must not be sized for an undealt round-one near hand"
    );
    assert!(pending_round_deal.is_pending);
    assert!(pending_round_deal.waits_for_location_intro);
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
        .insert_resource(MatchModel::new(
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
    app.update();

    assert_eq!(
        app.world()
            .entity(power_view)
            .get::<PointView>()
            .unwrap()
            .model,
        PointModel::location_power(0)
    );

    app.world_mut()
        .resource_mut::<MatchModel>()
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
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
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
        .insert_resource(MatchModel::new(
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
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
        match_model.near.draw(1);
        match_model.far.draw(1);
        match_model.near.energy_available = 1;
        match_model.far.energy_available = 1;
    }

    let mut elapsed = 0.0;
    while elapsed < 30.0 {
        elapsed += 0.5;
        {
            let match_model = app.world().resource::<MatchModel>();
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
        if app.world().resource::<MatchModel>().round.winner.is_some() {
            break;
        }
    }

    let match_model = app.world().resource::<MatchModel>();
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
        .insert_resource(MatchModel::new(
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
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
        match_model.near.draw(1);
        match_model.near.energy_available = 1;
    }
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(1));
    app.update();

    assert_eq!(app.world().resource::<MatchModel>().near.hand.len(), 1);
    assert_eq!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        0
    );

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    {
        let match_model = app.world().resource::<MatchModel>();
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

    assert_eq!(app.world().resource::<MatchModel>().near.hand.len(), 0);
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
        .insert_resource(MatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec![crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string()],
        ))
        .add_systems(Update, cpu_brain_update_system);
    let (round, far_hand_count) = {
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
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

    let match_model = app.world().resource::<MatchModel>();
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
        .insert_resource(MatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec![crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string()],
        ))
        .add_systems(Update, cpu_brain_update_system);
    let (instance_id, card_id) = {
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
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
        CardAnimation::move_deck_to_hand_to_slot(
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
            .resource::<MatchModel>()
            .has_pending_cpu_placements()
    );

    let animated_hand_cards: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, (With<CpuHandCardView>, With<CardAnimation>)>()
        .iter(app.world())
        .collect();
    for entity in animated_hand_cards {
        app.world_mut().entity_mut(entity).remove::<CardAnimation>();
    }
    let remaining_animated_hand_cards = app
        .world_mut()
        .query_filtered::<Entity, (With<CpuHandCardView>, With<CardAnimation>)>()
        .iter(app.world())
        .count();
    assert_eq!(remaining_animated_hand_cards, 0);

    {
        let (round, far_hand_count) = {
            let match_model = app.world().resource::<MatchModel>();
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
    let match_model = app.world().resource::<MatchModel>();
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
fn top_nav_transitions_share_consistent_overlay_alpha_history() {
    let destinations = [
        TopNavigationDestination::PlayGame,
        TopNavigationDestination::MyDecks,
        TopNavigationDestination::Settings,
        TopNavigationDestination::Debug,
    ];
    let reference_destination = TopNavigationDestination::PlayGame;
    let reference_trace = collect_top_nav_transition_trace(reference_destination);

    assert!(!reference_trace.is_empty());
    for destination in destinations {
        let trace = collect_top_nav_transition_trace(destination);
        assert_eq!(
            reference_trace.len(),
            trace.len(),
            "{destination:?} transition frame count diverged from {reference_destination:?}"
        );
        for (index, (reference, frame)) in reference_trace.iter().zip(trace.iter()).enumerate() {
            assert_eq!(
                reference.phase, frame.phase,
                "{destination:?} phase mismatch at frame {index}: reference={:?} actual={:?}",
                reference.phase, frame.phase
            );
            assert!(
                (reference.alpha - frame.alpha).abs() < 0.000_1,
                "{destination:?} alpha mismatch at frame {index}: reference={} actual={}",
                reference.alpha,
                frame.alpha
            );
        }
    }

    let black_frames = reference_trace
        .iter()
        .filter(|frame| (frame.alpha - 1.0).abs() < 0.000_1)
        .count();
    let expected_hold_frames =
        (SCREEN_TRANSITION_BLACK_HOLD_SECONDS / (1.0 / 60.0)).floor() as usize;
    assert!(
        black_frames >= expected_hold_frames,
        "expected hold-at-black plateau, got {black_frames} full-black frames"
    );
}

#[test]
fn top_nav_ignores_press_for_currently_selected_destination() {
    let mut app = create_nav_transition_test_app();
    app.world_mut().init_resource::<AudioManagerModel>();
    {
        let mut transition = app.world_mut().resource_mut::<ScreenTransitionResource>();
        transition.phase = ScreenTransitionPhase::Idle;
        transition.phase_elapsed_seconds = 0.0;
        transition.overlay_alpha = 0.0;
    }

    let play_game_button = app
        .world_mut()
        .query::<(Entity, &TopNavigationButton)>()
        .iter(app.world())
        .find_map(|(entity, button)| {
            (button.destination == TopNavigationDestination::PlayGame).then_some(entity)
        })
        .expect("play game top nav button should exist");
    *app.world_mut()
        .get_mut::<Interaction>(play_game_button)
        .unwrap() = Interaction::Pressed;
    app.update();

    assert_eq!(
        *app.world().resource::<ActiveView>(),
        ActiveView::MainMenuScene
    );
    assert_eq!(
        app.world().resource::<ScreenTransitionResource>().phase,
        ScreenTransitionPhase::Idle
    );
    assert!(
        app.world()
            .resource::<AudioManagerModel>()
            .requests
            .is_empty(),
        "selected top nav press should not enqueue click audio"
    );
}

#[test]
fn cpu_brain_does_not_stall_when_cpu_hand_view_is_partially_missing() {
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
        .insert_resource(MatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec![
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
            ],
        ))
        .add_systems(Update, cpu_brain_update_system);

    let (round, far_hand_count, first_instance_id, first_card_id) = {
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
        match_model.far.draw(2);
        match_model.far.energy_available = 10;
        (
            match_model.round.round,
            match_model.far.hand.len(),
            match_model.far.hand_instance_id(0).unwrap(),
            match_model.far.hand[0].clone(),
        )
    };

    app.world_mut().spawn(CpuHandCardView::new(
        MatchPlayerSide::Far,
        first_instance_id,
        0,
        first_card_id,
        CardFace::Back,
    ));

    {
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

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs(1));
    app.update();

    let match_model = app.world().resource::<MatchModel>();
    assert!(match_model.far.ready_for_next);
    assert!(match_model.has_pending_cpu_placements());
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
fn hidden_game_scene_enforcement_disables_game_scene_point_text_camera() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(ActiveView::DebugScene)
        .add_systems(Update, enforce_hidden_game_scene_visibility_system);
    let camera = app
        .world_mut()
        .spawn((
            GameSceneEntity,
            CardPointTextCamera,
            Camera::default(),
            Visibility::Visible,
        ))
        .id();

    app.update();

    assert!(
        !app.world()
            .entity(camera)
            .get::<Camera>()
            .unwrap()
            .is_active
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
    let mut animation = CardAnimation::move_deck_to_hand_to_slot(
        source_transform,
        hand_transform,
        slot_transform,
        CardFace::Back,
    );

    let is_settled = advance_card_animation(10.0, &mut transform, &mut animation, None);

    assert!(!is_settled);
    assert_eq!(
        animation.phase,
        crate::runtime::components::CardAnimationPhase::MovingToSlot
    );
    assert_close(transform.translation.x, hand_transform.translation.x);
    assert_close(transform.translation.y, hand_transform.translation.y);
    assert!(
        transform
            .rotation
            .angle_between(hand_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI))
            < 0.000_1
    );

    let is_settled = advance_card_animation(10.0, &mut transform, &mut animation, None);

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
    let mut animation = CardAnimation::move_deck_to_hand_to_slot(
        source_transform,
        hand_transform,
        slot_transform,
        CardFace::Back,
    );

    assert!(!advance_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.5,
        &mut transform,
        &mut animation,
        None
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
        CardAnimation::move_hand_to_slot(hand_transform, slot_transform, CardFace::Back);

    assert!(!advance_card_animation(
        CPU_CARD_MOVE_SECONDS - 0.01,
        &mut transform,
        &mut animation,
        None
    ));
    assert!(transform.translation.distance(slot_transform.translation) > 0.0);
    let apparent_scale = apparent_scale_at_z(transform.scale.x, transform.translation.z);
    let target_apparent_scale =
        apparent_scale_at_z(slot_transform.scale.x, slot_transform.translation.z);
    assert!(apparent_scale > target_apparent_scale);
    assert!(apparent_scale < target_apparent_scale * CPU_CARD_MOVE_SCALE_MULTIPLIER);

    assert!(advance_card_animation(
        0.01,
        &mut transform,
        &mut animation,
        None
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
        CardAnimation::move_hand_to_slot(hand_transform, slot_transform, CardFace::Back);

    assert!(!advance_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.1,
        &mut transform,
        &mut animation,
        None
    ));
    let apparent_scale = apparent_scale_at_z(transform.scale.x, transform.translation.z);
    let hand_apparent_scale =
        apparent_scale_at_z(hand_transform.scale.x, hand_transform.translation.z);
    assert!(apparent_scale > hand_apparent_scale);
    assert!(apparent_scale < hand_apparent_scale * CPU_CARD_MOVE_SCALE_MULTIPLIER);

    assert!(!advance_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.4,
        &mut transform,
        &mut animation,
        None
    ));
    assert_close(
        apparent_scale_at_z(transform.scale.x, transform.translation.z),
        hand_apparent_scale * CPU_CARD_MOVE_SCALE_MULTIPLIER,
    );
}

#[test]
fn cpu_placed_card_scale_multiplier_uses_equal_half_move_timing() {
    assert_close(cpu_card_move_scale_multiplier(0.0), 1.0);
    assert_close(cpu_card_move_scale_multiplier(0.25), 1.05);
    assert_close(cpu_card_move_scale_multiplier(0.5), 1.1);
    assert_close(cpu_card_move_scale_multiplier(0.75), 1.05);
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
        CardAnimation::move_hand_to_slot(hand_transform, slot_transform, CardFace::Back);

    assert!(!advance_card_animation(
        CPU_CARD_MOVE_SECONDS * 0.5,
        &mut transform,
        &mut animation,
        None
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

fn assert_cpu_card_tween_path_stable_for_fps(frames_per_second: f32) {
    #[derive(Clone, Copy)]
    struct FrameSample {
        phase: crate::runtime::components::CardAnimationPhase,
        phase_elapsed_seconds: f32,
        phase_start_transform: Transform,
        phase_target_transform: Transform,
        game_scene_xy: Vec2,
        z: f32,
        scale: Vec3,
        apparent_width: f32,
        apparent_height: f32,
    }

    let card_defaults = CardInspectionDefaults::default();
    let slot_target_xy = Vec2::new(640.0, 260.0);
    let source_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(220.0, 730.0), 0.52),
        rotation: Quat::from_rotation_y(std::f32::consts::PI),
        scale: Vec3::splat(0.25),
    };
    let hand_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(610.0, 675.0), 0.3),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let slot_transform = Transform {
        translation: game_scene_world_position_from_game_scene(slot_target_xy, 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let mut transform = source_transform;
    let mut animation = CardAnimation::move_deck_to_hand_to_slot(
        source_transform,
        hand_transform,
        slot_transform,
        CardFace::Back,
    );

    let delta_seconds = 1.0 / frames_per_second;
    let mut samples: Vec<FrameSample> = Vec::new();
    let mut settled = false;
    for _ in 0..(frames_per_second as usize * 4) {
        settled = advance_card_animation(delta_seconds, &mut transform, &mut animation, None);
        let apparent_scale = apparent_scale_at_z(transform.scale.x, transform.translation.z);
        samples.push(FrameSample {
            phase: animation.phase,
            phase_elapsed_seconds: animation.phase_elapsed_seconds,
            phase_start_transform: animation.phase_start_transform,
            phase_target_transform: animation.target_transform,
            game_scene_xy: game_scene_position_from_world_position(transform.translation),
            z: transform.translation.z,
            scale: transform.scale,
            apparent_width: card_defaults.width * apparent_scale,
            apparent_height: card_defaults.height * apparent_scale,
        });
        if settled {
            break;
        }
    }

    assert!(
        settled,
        "animation should settle within {frames_per_second}fps sample budget"
    );
    assert!(samples.len() > 10);

    let expected_aspect = card_defaults.width / card_defaults.height;
    let start_to_hand = game_scene_position_from_world_position(hand_transform.translation)
        - game_scene_position_from_world_position(source_transform.translation);
    let hand_to_slot = game_scene_position_from_world_position(slot_transform.translation)
        - game_scene_position_from_world_position(hand_transform.translation);
    let max_step_deck_to_hand =
        start_to_hand.length() * ((delta_seconds / CPU_CARD_MOVE_SECONDS) * 3.1);
    let max_step_hand_to_slot =
        hand_to_slot.length() * ((delta_seconds / CPU_CARD_MOVE_SECONDS) * 3.1);
    let slot_apparent_width = card_defaults.width
        * apparent_scale_at_z(slot_transform.scale.x, slot_transform.translation.z);
    let slot_apparent_height = card_defaults.height
        * apparent_scale_at_z(slot_transform.scale.y, slot_transform.translation.z);
    let expected_apparent_size = |sample: FrameSample| -> (f32, f32) {
        if sample.phase_elapsed_seconds <= 0.0 {
            return (sample.apparent_width, sample.apparent_height);
        }

        let progress = (sample.phase_elapsed_seconds / CPU_CARD_MOVE_SECONDS).clamp(0.0, 1.0);
        let eased_progress = ease_out_cubic(progress);
        let expected_z = if progress < 1.0 {
            CPU_CARD_MOVING_FRONT_Z
        } else {
            sample.phase_target_transform.translation.z
        };
        let scale_multiplier = match sample.phase {
            crate::runtime::components::CardAnimationPhase::MovingToHand => 1.0,
            crate::runtime::components::CardAnimationPhase::MovingToSlot => {
                cpu_card_move_scale_multiplier(progress)
            }
            crate::runtime::components::CardAnimationPhase::Revealing => 1.0,
        };
        let expected_scale = if progress < 1.0 {
            cpu_card_move_scale(
                sample.phase_start_transform,
                sample.phase_target_transform,
                expected_z,
                eased_progress,
                scale_multiplier,
            )
        } else {
            sample.phase_target_transform.scale
        };
        (
            card_defaults.width * apparent_scale_at_z(expected_scale.x, expected_z),
            card_defaults.height * apparent_scale_at_z(expected_scale.y, expected_z),
        )
    };

    for sample in &samples {
        assert!(sample.game_scene_xy.x.is_finite());
        assert!(sample.game_scene_xy.y.is_finite());
        assert!(sample.z.is_finite());
        assert!(sample.apparent_width.is_finite());
        assert!(sample.apparent_height.is_finite());
        assert!(sample.apparent_width > 0.0);
        assert!(sample.apparent_height > 0.0);
        assert!((sample.apparent_width / sample.apparent_height - expected_aspect).abs() < 0.0001);
        assert!(sample.z <= CPU_CARD_MOVING_FRONT_Z + 0.0001);
        assert!(
            sample.z
                >= source_transform
                    .translation
                    .z
                    .min(hand_transform.translation.z)
                    - 0.0001
        );
    }

    for window in samples.windows(2) {
        let prev = window[0];
        let next = window[1];
        let max_step = match prev.phase {
            crate::runtime::components::CardAnimationPhase::MovingToHand => max_step_deck_to_hand,
            crate::runtime::components::CardAnimationPhase::MovingToSlot => max_step_hand_to_slot,
            crate::runtime::components::CardAnimationPhase::Revealing => 0.0,
        };
        let step = next.game_scene_xy.distance(prev.game_scene_xy);
        assert!(
            step <= max_step + 0.01,
            "unexpected tween jump: step={step} max_step={max_step}"
        );

        let width_step = (next.apparent_width - prev.apparent_width).abs();
        let (prev_expected_width, prev_expected_height) = expected_apparent_size(prev);
        let (next_expected_width, next_expected_height) = expected_apparent_size(next);
        let expected_width_step = (next_expected_width - prev_expected_width).abs();
        assert!(
            width_step <= expected_width_step + 0.01,
            "unexpected apparent width jump at {frames_per_second}fps: step={width_step} expected_step={expected_width_step}"
        );
        let height_step = (next.apparent_height - prev.apparent_height).abs();
        let expected_height_step = (next_expected_height - prev_expected_height).abs();
        assert!(
            height_step <= expected_height_step + 0.01,
            "unexpected apparent height jump at {frames_per_second}fps: step={height_step} expected_step={expected_height_step}"
        );

        if prev.phase == next.phase {
            let phase_target_xy =
                game_scene_position_from_world_position(prev.phase_target_transform.translation);
            let prev_distance = prev.game_scene_xy.distance(phase_target_xy);
            let next_distance = next.game_scene_xy.distance(phase_target_xy);
            assert!(
                next_distance <= prev_distance + 0.000_1,
                "phase target distance increased at {frames_per_second}fps: prev={prev_distance} next={next_distance}"
            );
        }
    }

    for sample in &samples {
        let progress = (sample.phase_elapsed_seconds / CPU_CARD_MOVE_SECONDS).clamp(0.0, 1.0);
        let eased_progress = ease_out_cubic(progress);
        let expected_z = if progress < 1.0 {
            CPU_CARD_MOVING_FRONT_Z
        } else {
            sample.phase_target_transform.translation.z
        };

        if sample.phase_elapsed_seconds <= 0.0 {
            continue;
        }

        let expected_translation = cpu_card_move_translation(
            sample.phase_start_transform.translation,
            sample.phase_target_transform.translation,
            expected_z,
            eased_progress,
        );
        assert_vec2_close(
            sample.game_scene_xy,
            game_scene_position_from_world_position(expected_translation),
        );
        assert_close(sample.z, expected_translation.z);

        let scale_multiplier = match sample.phase {
            crate::runtime::components::CardAnimationPhase::MovingToHand => 1.0,
            crate::runtime::components::CardAnimationPhase::MovingToSlot => {
                cpu_card_move_scale_multiplier(progress)
            }
            crate::runtime::components::CardAnimationPhase::Revealing => 1.0,
        };
        let expected_scale = if progress < 1.0 {
            cpu_card_move_scale(
                sample.phase_start_transform,
                sample.phase_target_transform,
                expected_z,
                eased_progress,
                scale_multiplier,
            )
        } else {
            sample.phase_target_transform.scale
        };
        assert_close(sample.scale.x, expected_scale.x);
        assert_close(sample.scale.y, expected_scale.y);
        assert_close(sample.scale.z, expected_scale.z);
    }

    let final_sample = samples[samples.len() - 1];
    let final_target_xy = game_scene_position_from_world_position(slot_transform.translation);
    assert_vec2_close(final_sample.game_scene_xy, final_target_xy);
    assert_close(final_sample.z, slot_transform.translation.z);
    assert_close(final_sample.apparent_width, slot_apparent_width);
    assert_close(final_sample.apparent_height, slot_apparent_height);
}

#[test]
fn cpu_card_tween_path_is_stable_at_30fps() {
    assert_cpu_card_tween_path_stable_for_fps(30.0);
}

#[test]
fn cpu_card_tween_path_is_stable_at_60fps() {
    assert_cpu_card_tween_path_stable_for_fps(60.0);
}

#[test]
fn cpu_card_tween_path_is_stable_at_120fps() {
    assert_cpu_card_tween_path_stable_for_fps(120.0);
}

#[test]
fn committed_cpu_placement_records_original_visible_hand_source() {
    let mut match_model = MatchModel::new(
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
        CardFace::Back
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
        CardFace::Back
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
    let mut animation = CardAnimation::flip_to_front(slot_transform, 0.25);

    assert!(!advance_card_animation(
        0.24,
        &mut transform,
        &mut animation,
        None
    ));
    assert_eq!(animation.current_face(), CardFace::Back);
    assert_close(animation.start_delay_seconds, 0.01);
    assert_eq!(transform.scale, slot_transform.scale);

    assert!(!advance_card_animation(
        0.25,
        &mut transform,
        &mut animation,
        None
    ));
    assert_eq!(animation.current_face(), CardFace::Front);
    assert!(animation.current_y_rotation > 0.0);
    assert_eq!(transform.translation, slot_transform.translation);
    assert_eq!(transform.scale, slot_transform.scale);
    assert!(transform.rotation.angle_between(slot_transform.rotation) > 0.0);

    assert!(advance_card_animation(
        0.76,
        &mut transform,
        &mut animation,
        None
    ));
    assert_eq!(animation.current_face(), CardFace::Front);
    assert_close(animation.current_y_rotation, 0.0);
    assert_eq!(transform.scale, slot_transform.scale);
}

#[test]
fn cpu_swan_flip_keeps_game_scene_anchor_while_scale_and_depth_tween() {
    let slot_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(880.0, 275.0), 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let anchor = game_scene_position_from_world_position(slot_transform.translation);
    let mut transform = slot_transform;
    transform.rotation = slot_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);
    let mut animation = CardAnimation::swan_flip_to_front(slot_transform, 0.0);

    assert!(!advance_card_animation(
        0.30,
        &mut transform,
        &mut animation,
        None
    ));
    assert_eq!(animation.phase, CardAnimationPhase::Revealing);
    assert!(transform.translation.z > slot_transform.translation.z);
    assert!(transform.scale.x > slot_transform.scale.x);
    assert_vec2_close(
        game_scene_position_from_world_position(transform.translation),
        anchor,
    );

    assert!(!advance_card_animation(
        0.30,
        &mut transform,
        &mut animation,
        None
    ));
    assert_vec2_close(
        game_scene_position_from_world_position(transform.translation),
        anchor,
    );
}

#[test]
fn swan_flip_keeps_front_depth_until_scale_down_completes() {
    let slot_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(880.0, 275.0), 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let mut transform = slot_transform;
    transform.rotation = slot_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);
    let mut animation = CardAnimation::swan_flip_to_front(slot_transform, 0.0);

    assert!(!advance_card_animation(
        GAME_TWEEN_SWAN_FLIP_SECONDS - 0.001,
        &mut transform,
        &mut animation,
        None
    ));
    assert_eq!(animation.phase, CardAnimationPhase::Revealing);
    assert_close(transform.translation.z, CPU_CARD_MOVING_FRONT_Z);
}

#[test]
fn card_animation_marker_tracks_active_tween_lifecycle() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Update, card_animation_marker_system);
    let slot_transform = Transform {
        translation: game_scene_world_position_from_game_scene(Vec2::new(880.0, 275.0), 0.52),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.25),
    };
    let entity = app
        .world_mut()
        .spawn(CardAnimation::swan_flip_to_front(slot_transform, 0.0))
        .id();

    app.update();

    assert_eq!(
        app.world().get::<CardAnimationMarker>(entity),
        Some(&CardAnimationMarker {
            phase: CardAnimationPhase::Revealing,
            flip_style: CardAnimationFlipStyle::Swan,
        })
    );

    app.world_mut().entity_mut(entity).remove::<CardAnimation>();
    app.update();

    assert!(app.world().get::<CardAnimationMarker>(entity).is_none());
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
        .insert_resource(MatchModel::new(
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
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
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

    let match_model = app.world().resource::<MatchModel>();
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Near, 2, 0),
        PlacementVisibility::Revealing
    );
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Far, 0, 1),
        PlacementVisibility::CurrentRoundHidden
    );
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Near, 0, 0),
        PlacementVisibility::CurrentRoundHidden
    );
    assert_close(match_model.next_reveal_delay_seconds, 0.0);

    app.update();

    let match_model = app.world().resource::<MatchModel>();
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Near, 2, 0),
        PlacementVisibility::Revealed
    );
    assert_eq!(
        match_model.placement_visibility(MatchPlayerSide::Far, 0, 1),
        PlacementVisibility::CurrentRoundHidden
    );
    assert_close(
        match_model.next_reveal_delay_seconds,
        CPU_CARD_REVEAL_STAGGER_SECONDS,
    );

    app.world_mut()
        .resource_mut::<MatchModel>()
        .next_reveal_delay_seconds = 0.0;
    app.update();

    assert_eq!(
        app.world()
            .resource::<MatchModel>()
            .placement_visibility(MatchPlayerSide::Far, 0, 1),
        PlacementVisibility::Revealing
    );
}

#[test]
fn round_finish_keeps_human_location_cards_assigned_while_next_deal_waits() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<GameDeckModel>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ]))
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .insert_resource(CardStateModel::with_size(1))
        .init_resource::<PendingRoundDealResource>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(MatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec![
                crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string();
                crate::runtime::resources::STARTING_DECK_CARD_COUNT
            ],
        ))
        .add_systems(Update, staged_match_resolution_system);

    assert!(
        app.world_mut()
            .resource_mut::<CardSlotBoardModel>()
            .place_next_local_with_card_id(0, 0, crate::runtime::resources::KAGE_REN_CARD_MODEL_ID,)
            .is_some()
    );
    app.world_mut()
        .resource_mut::<CardStateModel>()
        .place_in_location(0);
    {
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
        match_model.record_placement(MatchPlayerSide::Near, 0, 0);
        match_model.begin_current_round_reveal();
        match_model.reveal_current_round_placements();
        match_model.resolution_phase = MatchResolutionPhase::CpuPlacementsRevealing;
    }
    let placed_card = app
        .world_mut()
        .spawn((HandCardGestureTarget::new(0), CardGestureView))
        .id();

    app.update();

    assert_eq!(
        app.world().resource::<CardStateModel>().state(0),
        Some(CardState::LocationLocked)
    );
    assert!(
        app.world()
            .resource::<CardSlotBoardModel>()
            .slot(0, CardSlotSide::LocalPlayer, 0)
            .is_some_and(|slot| !slot.state.is_empty())
    );
    assert!(
        app.world().get_entity(placed_card).is_ok(),
        "placed human card entity should remain in its assigned location"
    );
    assert!(
        app.world()
            .resource::<PendingRoundDealResource>()
            .is_pending
    );
}

#[test]
fn hand_sync_does_not_respawn_first_near_card_after_reveal_prep_removes_hand_markers() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ]))
        .insert_resource(CardStateModel::with_size(1))
        .add_systems(Update, sync_game_scene_hand_card_entities_system);

    app.world_mut()
        .resource_mut::<CardStateModel>()
        .place_in_location(0);

    app.update();

    let hand_card_count = app
        .world_mut()
        .query_filtered::<Entity, (With<CardGestureView>, With<HandCardGestureTarget>)>()
        .iter(app.world())
        .count();
    assert_eq!(
        hand_card_count, 0,
        "placed near hand index 0 must not be recreated as a new hand card during reveal"
    );
}

#[test]
fn hand_sync_keeps_existing_near_card_while_dragging() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ]))
        .insert_resource(CardStateModel::with_size(1))
        .add_systems(Update, sync_game_scene_hand_card_entities_system);
    let card = app
        .world_mut()
        .spawn((
            HandCardGestureTarget::new(0),
            CardGestureView,
            LocalPlayerHandCardPreview,
            Transform::default(),
            Visibility::Visible,
        ))
        .id();
    assert!(
        app.world_mut()
            .resource_mut::<CardStateModel>()
            .begin_drag(0)
    );

    app.update();

    assert!(
        app.world().get_entity(card).is_ok(),
        "hand sync must not despawn the active card entity while it is being dragged"
    );
}

#[test]
fn round_deal_completion_allows_settled_hand_card_hover_rotation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardInspectionDefaults>()
        .insert_resource(CardStateModel::with_size(1));
    let target_transform =
        hand_source_transform(0, 1, app.world().resource::<CardInspectionDefaults>());
    app.world_mut().spawn((
        HandCardGestureTarget::new(0),
        CardGestureView,
        Transform {
            rotation: Quat::from_rotation_z(0.05),
            ..target_transform
        },
    ));

    let is_complete = app
        .world_mut()
        .run_system_once(
            |defaults: Res<CardInspectionDefaults>,
             states: Res<CardStateModel>,
             local_hand_query: Query<
                (&HandCardGestureTarget, &Transform),
                With<CardGestureView>,
            >,
             cpu_hand_query: Query<Option<&CardAnimation>, With<CpuHandCardView>>| {
                round_deal_visuals_are_complete(
                    &defaults,
                    &states,
                    &local_hand_query,
                    &cpu_hand_query,
                )
            },
        )
        .unwrap();

    assert!(
        is_complete,
        "hover tilt must not keep the round-deal gate closed after hand cards settle"
    );
}

#[test]
fn local_current_round_cards_insta_flip_to_back_when_resolution_starts() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<AudioManagerModel>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(GameRoundModel {
            current_round_moves: vec![CurrentRoundMoveRecord {
                hand_index: 2,
                card_id: "kage_ren".to_string(),
                location_index: 1,
                slot_index: 0,
                energy_cost: 1,
                location_energy_delta: 0,
            }],
            ..GameRoundModel::default()
        })
        .insert_resource({
            let mut match_model = MatchModel::new(
                MatchModeModel::HumanVersusCpu,
                vec!["kage_ren".to_string(); crate::runtime::resources::STARTING_DECK_CARD_COUNT],
            );
            match_model.resolution_phase = MatchResolutionPhase::CpuPlacementsRevealing;
            match_model
        })
        .add_systems(Update, prepare_local_current_round_reveals_system);

    let root = app
        .world_mut()
        .spawn((
            CardGestureView,
            HandCardGestureTarget::new(2),
            LocalPlayerHandCardPreview,
            SelectableCard::new(CardSelectionSource::LocalHand { hand_index: 2 }),
            Transform::default(),
        ))
        .id();
    let front = app
        .world_mut()
        .spawn((CardFaceLayer::new(CardFace::Front), Visibility::Visible))
        .id();
    let back = app
        .world_mut()
        .spawn((CardFaceLayer::new(CardFace::Back), Visibility::Hidden))
        .id();
    app.world_mut()
        .entity_mut(root)
        .add_children(&[front, back]);

    app.update();

    let reveal_view = app
        .world()
        .get::<CpuPlacedCardView>(root)
        .expect("local placed card should use per-card reveal control");
    assert_eq!(reveal_view.owner, MatchPlayerSide::Near);
    assert_eq!(reveal_view.side, CardSlotSide::LocalPlayer);
    assert_eq!(reveal_view.location_index, 1);
    assert_eq!(reveal_view.slot_index, 0);
    assert_eq!(reveal_view.visible_face, CardFace::Back);
    assert!(
        app.world()
            .get::<LocalPlayerHandCardPreview>(root)
            .is_none()
    );
    assert!(app.world().get::<CardGestureView>(root).is_none());
    assert!(app.world().get::<HandCardGestureTarget>(root).is_none());
    assert_eq!(
        app.world()
            .get::<SelectableCard>(root)
            .expect("prepared local card should remain selectable after reveal")
            .source,
        CardSelectionSource::LocalLocation {
            location_index: 1,
            slot_index: 0,
            hand_index: 2,
        }
    );
    assert!(app.world().get::<CardAnimationFaceLayer>(front).is_some());
    assert!(app.world().get::<CardAnimationFaceLayer>(back).is_some());
    assert!(
        app.world()
            .resource::<AudioManagerModel>()
            .requests
            .iter()
            .any(|request| request.cue == AudioEnum::CardFlip)
    );
}

#[test]
fn local_revealing_card_uses_swan_flip_before_cpu_reveals() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_resource::<AudioManagerModel>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(MatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec!["kage_ren".to_string(); crate::runtime::resources::STARTING_DECK_CARD_COUNT],
        ))
        .add_systems(Update, sync_cpu_placed_card_entities_system);

    assert!(
        app.world_mut()
            .resource_mut::<CardSlotBoardModel>()
            .place_for_side_with_card_id(0, CardSlotSide::LocalPlayer, 0, 2, "kage_ren")
    );
    let slot_board = app.world().resource::<CardSlotBoardModel>().clone();
    let mut match_model = app.world_mut().resource_mut::<MatchModel>();
    match_model.record_placement(MatchPlayerSide::Near, 0, 0);
    match_model.start_next_current_round_reveal(&slot_board);
    let root = app
        .world_mut()
        .spawn((
            CpuPlacedCardView::new(
                MatchPlayerSide::Near,
                CardSlotSide::LocalPlayer,
                0,
                0,
                "kage_ren",
                CardFace::Back,
            ),
            Transform::default(),
        ))
        .id();

    app.update();

    let animation = app
        .world()
        .get::<CardAnimation>(root)
        .expect("local revealing card should receive reveal animation");
    assert_eq!(animation.phase, CardAnimationPhase::Revealing);
    assert_eq!(animation.flip_style, CardAnimationFlipStyle::Swan);
    assert!(
        app.world()
            .resource::<AudioManagerModel>()
            .requests
            .iter()
            .any(|request| request.cue == AudioEnum::CardFlip)
    );
}

#[test]
fn local_swan_reveal_in_progress_is_not_restarted_by_sync() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_resource::<AudioManagerModel>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<CardSlotBoardModel>()
        .insert_resource(ActiveView::GameScene)
        .insert_resource(MatchModel::new(
            MatchModeModel::HumanVersusCpu,
            vec!["kage_ren".to_string(); crate::runtime::resources::STARTING_DECK_CARD_COUNT],
        ))
        .add_systems(Update, sync_cpu_placed_card_entities_system);

    assert!(
        app.world_mut()
            .resource_mut::<CardSlotBoardModel>()
            .place_for_side_with_card_id(0, CardSlotSide::LocalPlayer, 0, 2, "kage_ren")
    );
    let slot_board = app.world().resource::<CardSlotBoardModel>().clone();
    {
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
        match_model.record_placement(MatchPlayerSide::Near, 0, 0);
        match_model.start_next_current_round_reveal(&slot_board);
    }
    let slot_transform = slot_transform(
        0,
        0,
        CardSlotSide::LocalPlayer,
        &slot_board,
        app.world().resource::<CardInspectionDefaults>(),
    );
    let mut reveal_animation = CardAnimation::swan_flip_to_front(slot_transform, 0.0);
    reveal_animation.phase_elapsed_seconds = 0.42;
    let root = app
        .world_mut()
        .spawn((
            CpuPlacedCardView::new(
                MatchPlayerSide::Near,
                CardSlotSide::LocalPlayer,
                0,
                0,
                "kage_ren",
                CardFace::Back,
            ),
            reveal_animation,
            Transform::default(),
        ))
        .id();

    app.update();

    let animation = app
        .world()
        .get::<CardAnimation>(root)
        .expect("in-progress swan reveal should remain active");
    assert_close(animation.phase_elapsed_seconds, 0.42);
    assert_eq!(
        app.world()
            .get::<CpuPlacedCardView>(root)
            .expect("near placed card view should remain present")
            .visible_face,
        CardFace::Front
    );
}

#[test]
fn local_swan_reveal_is_not_overwritten_by_hand_preview_tilt() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f32(1.0 / 60.0),
        ))
        .insert_resource(ActiveView::GameScene)
        .insert_resource(CardInspectionDefaults::default())
        .insert_resource(CardInspectionState {
            last_pointer_normalized: Vec2::new(1.0, 0.0),
            target_rotation: Quat::IDENTITY,
        })
        .insert_resource(CardFlipState::default())
        .add_systems(
            Update,
            (
                card_animation_system,
                smooth_card_rotation.after(card_animation_system),
            ),
        );

    let slot_transform = Transform::from_translation(Vec3::new(0.0, 0.0, CARD_GESTURE_SLOT_Z))
        .with_scale(Vec3::splat(0.25));
    let root = app
        .world_mut()
        .spawn((
            CardView,
            GameSceneEntity,
            LocalPlayerHandCardPreview,
            CardAnimation::swan_flip_to_front(slot_transform, 0.0),
            slot_transform,
        ))
        .id();

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    app.update();
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    app.update();
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
    app.update();

    let actual_rotation = app
        .world()
        .get::<Transform>(root)
        .expect("local revealing card should still exist")
        .rotation;
    let animation = app
        .world()
        .get::<CardAnimation>(root)
        .expect("swan reveal should still be active");
    let expected_rotation =
        slot_transform.rotation * Quat::from_rotation_y(animation.current_y_rotation);

    assert!(
        actual_rotation.angle_between(expected_rotation) < 0.0001,
        "local reveal rotation should be owned by swan animation while active"
    );
}

#[test]
fn cpu_placed_card_face_visibility_uses_per_card_reveal_state() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CardUiState>()
        .add_systems(Update, update_card_animation_face_visibility_system);

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
            CardAnimation::move_deck_to_hand_to_slot(
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
            CardAnimationFaceLayer,
            Visibility::Hidden,
        ))
        .id();
    let back = app
        .world_mut()
        .spawn((
            CardFaceLayer::new(CardFace::Back),
            CardAnimationFaceLayer,
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

    app.world_mut().entity_mut(root).remove::<CardAnimation>();
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
fn hand_card_entity_sync_repairs_missing_hand_states_before_spawning_dealt_cards() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardModelRegistry>()
        .insert_resource(GameHandModel::new(vec![
            crate::runtime::resources::KAGE_REN_CARD_MODEL_ID.to_string(),
        ]))
        .insert_resource(CardStateModel::default())
        .add_systems(Update, sync_game_scene_hand_card_entities_system);

    app.update();

    assert_eq!(
        app.world().resource::<CardStateModel>().state(0),
        Some(CardState::Hand)
    );
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
        .init_resource::<SelectedCardModalModel>()
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
    assert!(app.world().resource::<SelectedCardModalModel>().is_active());
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenSelectedCardMenuRoot>>()
            .iter(app.world())
            .count(),
        0
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
        .query_filtered::<Entity, (With<AppSceneCamera>, With<Camera3d>)>();
    assert_eq!(camera_query.iter(app.world()).count(), 1);
}

#[test]
fn clicking_debug_card_selects_card_without_selected_card_menu() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_asset::<Image>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<Touches>()
        .init_resource::<ActiveView>()
        .init_resource::<PrimaryCameraDefaults>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<CardGestureModel>()
        .init_resource::<SelectedCardModalModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CardFlipState>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<ActiveCardModel>()
        .add_systems(Startup, setup_debug_scene)
        .add_systems(Update, card_selection_update_system);
    let window = spawn_test_primary_window(&mut app);
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::DebugScene;

    app.update();

    let debug_card = app
        .world_mut()
        .query_filtered::<Entity, (With<CardView>, With<DebugSceneEntity>, With<SelectableCard>)>()
        .single(app.world())
        .unwrap();
    prepare_debug_camera_for_test_viewport(&mut app, window);
    let pointer_position = window_pointer_for_debug_card_center(&mut app, debug_card);
    app.world_mut()
        .get_mut::<Window>(window)
        .unwrap()
        .set_cursor_position(Some(pointer_position));
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .press(MouseButton::Left);
    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .release(MouseButton::Left);
    app.update();

    assert_eq!(
        app.world()
            .resource::<SelectedCardModalModel>()
            .selected_entity,
        Some(debug_card)
    );
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<DeckScreenSelectedCardMenuRoot>>()
            .iter(app.world())
            .count(),
        0
    );
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
    assert_eq!(
        offset.y,
        SCREEN_PADDING_TOP + DEBUG_SCENE_CARD_VERTICAL_OFFSET
    );
}

#[test]
fn card_ui_anchor_accounts_for_tall_window_safe_area() {
    let offset = card_ui_safe_area_anchor_offset(Vec2::new(1280.0, 1000.0));

    assert_eq!(offset.x, -SCREEN_PADDING_LEFT);
    assert_eq!(
        offset.y,
        100.0 + SCREEN_PADDING_TOP + DEBUG_SCENE_CARD_VERTICAL_OFFSET
    );
}

#[test]
fn card_ui_anchor_padding_scales_with_debug_hud() {
    let offset = card_ui_safe_area_anchor_offset(Vec2::new(1024.0, 768.0));

    assert_close(offset.x, -(SCREEN_PADDING_LEFT * 0.8));
    assert_close(
        offset.y,
        64.0 + ((SCREEN_PADDING_TOP + DEBUG_SCENE_CARD_VERTICAL_OFFSET) * 0.8),
    );
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
        brighten_color(END_ROUND_BUTTON_NORMAL_COLOR, 1.2)
    );
    assert_eq!(
        *app.world().get::<BorderColor>(button).unwrap(),
        BorderColor::all(brighten_color(END_ROUND_BUTTON_NORMAL_BORDER_COLOR, 1.2))
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
fn next_button_waits_for_round_deal_completion_before_accepting_click() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveView>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<MatchModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<PendingRoundDealResource>()
        .add_systems(Update, update_end_round_button);
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;

    let button = app
        .world_mut()
        .spawn((
            EndRoundButton,
            GameControlButton::new(GameControlAction::EndRound),
            Interaction::Pressed,
            BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
            BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
        ))
        .id();

    app.update();
    assert!(!app.world().resource::<MatchModel>().near.ready_for_next);
    assert_eq!(
        app.world().get::<BackgroundColor>(button).unwrap().0,
        GAME_CONTROL_DISABLED_COLOR
    );

    *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::None;
    {
        let mut pending_round_deal = app.world_mut().resource_mut::<PendingRoundDealResource>();
        pending_round_deal.is_round_deal_complete = true;
    }
    app.update();
    *app.world_mut().get_mut::<Interaction>(button).unwrap() = Interaction::Pressed;
    app.update();

    assert!(app.world().resource::<MatchModel>().near.ready_for_next);
}

#[test]
fn next_button_enables_in_same_frame_round_deal_completes() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveView>()
        .init_resource::<CardInspectionDefaults>()
        .init_resource::<GameRoundModel>()
        .init_resource::<MatchModel>()
        .init_resource::<PendingRoundDealResource>()
        .insert_resource(CardStateModel::with_size(1))
        .add_systems(
            Update,
            (
                round_deal_completion_update_system,
                update_game_control_ui_system.after(round_deal_completion_update_system),
            ),
        );
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    app.world_mut()
        .resource_mut::<PendingRoundDealResource>()
        .is_round_deal_complete = false;

    let target_transform =
        hand_source_transform(0, 1, app.world().resource::<CardInspectionDefaults>());
    app.world_mut().spawn((
        HandCardGestureTarget::new(0),
        CardGestureView,
        target_transform,
        Visibility::Visible,
    ));
    let button = app
        .world_mut()
        .spawn((
            EndRoundButton,
            GameControlButton::new(GameControlAction::EndRound),
            Interaction::None,
            BackgroundColor(GAME_CONTROL_DISABLED_COLOR),
            BorderColor::all(GAME_CONTROL_DISABLED_BORDER_COLOR),
        ))
        .id();

    app.update();

    assert!(
        app.world()
            .resource::<PendingRoundDealResource>()
            .is_round_deal_complete
    );
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
fn next_button_resolves_round_one_when_local_player_plays_no_card() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveView>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<MatchModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<PendingRoundDealResource>()
        .init_resource::<CpuBrainModel>()
        .add_systems(Update, update_end_round_button);
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    {
        let mut pending_round_deal = app.world_mut().resource_mut::<PendingRoundDealResource>();
        pending_round_deal.is_round_deal_complete = true;
    }
    {
        let mut match_model = app.world_mut().resource_mut::<MatchModel>();
        match_model.far.draw(1);
        match_model.far.energy_available = 1;
        assert!(!match_model.near.ready_for_next);
        assert!(!match_model.far.ready_for_next);
    }

    app.world_mut().spawn((
        EndRoundButton,
        GameControlButton::new(GameControlAction::EndRound),
        Interaction::Pressed,
        BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
        BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
    ));

    app.update();

    let match_model = app.world().resource::<MatchModel>();
    assert!(match_model.near.ready_for_next);
    assert!(match_model.far.ready_for_next);
    assert!(matches!(
        match_model.resolution_phase,
        MatchResolutionPhase::CpuPlacementsMoving | MatchResolutionPhase::CpuPlacementsRevealing
    ));
    let populated_count = app
        .world()
        .resource::<CardSlotBoardModel>()
        .populated_count();
    assert!(populated_count <= 1);
}

#[test]
fn next_button_resolves_when_cpu_has_no_legal_move() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveView>()
        .init_resource::<CardModelRegistry>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<MatchModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<PendingRoundDealResource>()
        .init_resource::<CpuBrainModel>()
        .add_systems(Update, update_end_round_button);
    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    app.world_mut()
        .resource_mut::<PendingRoundDealResource>()
        .is_round_deal_complete = true;
    app.world_mut()
        .resource_mut::<MatchModel>()
        .far
        .energy_available = 0;

    app.world_mut().spawn((
        EndRoundButton,
        GameControlButton::new(GameControlAction::EndRound),
        Interaction::Pressed,
        BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
        BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
    ));

    app.update();

    let match_model = app.world().resource::<MatchModel>();
    assert!(match_model.near.ready_for_next);
    assert!(match_model.far.ready_for_next);
    assert_eq!(
        match_model.resolution_phase,
        MatchResolutionPhase::CpuPlacementsRevealing
    );
}

#[test]
fn shared_button_hover_is_twenty_percent_brighter_than_normal() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Update, button_ui_hover_update_system);
    let normal = Color::srgb(0.20, 0.24, 0.32);
    let button = app
        .world_mut()
        .spawn((
            ButtonUiBundle::new("Hover Test").with_colors(normal, Color::WHITE),
            Interaction::Hovered,
        ))
        .id();

    app.update();

    assert_eq!(
        app.world().get::<BackgroundColor>(button).unwrap().0,
        brighten_color(normal, 1.2)
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
fn restart_button_restarts_game_model_and_randomizes_world() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveView>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<LocationModelRegistry>()
        .init_resource::<ActiveLocations>()
        .init_resource::<ActiveWorldModel>()
        .init_resource::<MatchModel>()
        .init_resource::<PlayerDeckCollectionModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CpuBrainModel>()
        .add_systems(Update, update_end_round_button);

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    app.world_mut().resource_mut::<ActiveWorldModel>().index = 0;
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
    app.world_mut().spawn((
        GameControlButton::new(GameControlAction::Restart),
        Interaction::Pressed,
        BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
        BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
    ));

    app.update();

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
    assert_ne!(app.world().resource::<ActiveWorldModel>().index, 0);
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
    let _ = expected_rotation;
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
fn game_scene_theme_key_only_updates_world_background() {
    let mut app = App::new();
    let frame_step = std::time::Duration::from_secs_f32(0.0625);
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(frame_step))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
        .init_resource::<ButtonInput<KeyCode>>()
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
        .init_resource::<PlayerDeckCollectionModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<MatchModel>()
        .init_resource::<CardStateModel>()
        .init_resource::<CardSlotBoardModel>()
        .init_resource::<CardGestureModel>()
        .init_resource::<CpuBrainModel>()
        .init_resource::<CardUiState>()
        .add_systems(Startup, setup_game_scene)
        .add_systems(
            Update,
            (
                card_model_input_system,
                world_theme_transition_update_system,
            )
                .chain(),
        );

    *app.world_mut().resource_mut::<ActiveView>() = ActiveView::GameScene;
    app.world_mut().resource_mut::<ActiveWorldModel>().index = 0;
    app.update();
    let startup_transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(
        startup_transition.phase,
        WorldThemeTransitionPhase::StartupFadeIn
    );
    assert_close(startup_transition.overlay_alpha, 1.0);
    app.update();
    let startup_transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(
        startup_transition.phase,
        WorldThemeTransitionPhase::StartupFadeIn
    );
    assert_close(startup_transition.overlay_alpha, 0.5);
    update_until_world_transition_idle(&mut app);
    let applied_world_index = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap()
        .applied_world_index;
    app.world_mut().resource_mut::<ActiveWorldModel>().index = applied_world_index;
    app.world_mut().resource_mut::<ActiveLocations>().indices = [5, 0, 1];
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .reset_with_active_location_indices(&[5, 0, 1]);
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(4);
    app.world_mut().resource_mut::<GameRoundModel>().round = 4;
    app.world_mut()
        .resource_mut::<GameRoundModel>()
        .energy_available = 2;
    let starting_world_index = app.world().resource::<ActiveWorldModel>().index;
    let expected_world_index = app
        .world()
        .resource::<WorldModelRegistry>()
        .next_index(starting_world_index);
    let expected_world_name = app
        .world()
        .resource::<WorldModelRegistry>()
        .active_world_model(&ActiveWorldModel {
            index: expected_world_index,
        })
        .display_name;
    let expected_background_name = format!("{expected_world_name} World Background");

    let (background_entity, starting_background_name) = app
        .world_mut()
        .query_filtered::<(Entity, &Name), With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    let starting_background_name = starting_background_name.to_string();
    let locations_before = app.world().resource::<ActiveLocations>().indices;
    let game_locations_before = app.world().resource::<GameLocationModel>().clone();
    let game_round_before = app.world().resource::<GameRoundModel>().clone();
    let game_hand_before = app.world().resource::<GameHandModel>().clone();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyT);
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset(KeyCode::KeyT);

    assert_eq!(
        app.world().resource::<ActiveWorldModel>().index,
        expected_world_index
    );
    assert_eq!(
        app.world()
            .resource::<WorldModelRegistry>()
            .active_world_model(app.world().resource::<ActiveWorldModel>())
            .display_name,
        expected_world_name
    );
    assert_eq!(
        app.world().resource::<ActiveLocations>().indices,
        locations_before
    );
    assert_eq!(
        app.world().resource::<GameLocationModel>(),
        &game_locations_before
    );
    assert_eq!(app.world().resource::<GameRoundModel>(), &game_round_before);
    assert_eq!(app.world().resource::<GameHandModel>(), &game_hand_before);
    let transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(transition.phase, WorldThemeTransitionPhase::FadeOutToBlack);
    assert_close(transition.overlay_alpha, 0.5);
    let (updated_background_entity, updated_background_name) = app
        .world_mut()
        .query_filtered::<(Entity, &Name), With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(updated_background_entity, background_entity);
    assert_eq!(updated_background_name.as_str(), starting_background_name);

    app.update();
    let transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(transition.phase, WorldThemeTransitionPhase::HoldAtBlack);
    assert_close(transition.overlay_alpha, 1.0);
    let updated_background_name = app
        .world_mut()
        .query_filtered::<&Name, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(updated_background_name.as_str(), expected_background_name);

    app.update();
    let transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(transition.phase, WorldThemeTransitionPhase::HoldAtBlack);
    assert_close(transition.overlay_alpha, 1.0);

    app.update();
    let transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(transition.phase, WorldThemeTransitionPhase::FadeInFromBlack);
    assert_close(transition.overlay_alpha, 1.0);

    app.update();
    let transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(transition.phase, WorldThemeTransitionPhase::FadeInFromBlack);
    assert_close(transition.overlay_alpha, 0.5);

    app.update();
    let transition = app
        .world_mut()
        .query_filtered::<&WorldThemeTransition, With<WorldBackground>>()
        .single(app.world())
        .unwrap();
    assert_eq!(transition.phase, WorldThemeTransitionPhase::Idle);
    assert_close(transition.overlay_alpha, 0.0);
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
fn card_layer_materials_do_not_escape_their_card_depth_band() {
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

    layer_biases.sort_by(|left, right| {
        card_layer_role_test_order(left.0)
            .cmp(&card_layer_role_test_order(right.0))
            .then_with(|| left.1.total_cmp(&right.1))
    });

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

    assert!(layer_biases.iter().all(|(_, bias, _)| *bias == 0.0));
}

#[test]
fn card_point_badge_materials_do_not_escape_their_card_depth_band() {
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

    let mut point_query = app
        .world_mut()
        .query_filtered::<(&ChildOf, &MeshMaterial3d<StandardMaterial>), With<PointViewCircle>>();
    let point_depth_biases: Vec<f32> = point_query
        .iter(app.world())
        .filter_map(|(child_of, material_handle)| {
            if app
                .world()
                .entity(child_of.parent())
                .get::<PointLocationView>()
                .is_some()
            {
                return None;
            }
            Some(
                app.world()
                    .resource::<Assets<StandardMaterial>>()
                    .get(&material_handle.0)
                    .unwrap()
                    .depth_bias,
            )
        })
        .collect();

    assert_eq!(point_depth_biases, vec![POINT_DEPTH_BIAS, POINT_DEPTH_BIAS]);
    assert!(point_depth_biases.iter().all(|bias| *bias == 0.0));
}

fn card_layer_role_test_order(role: CardLayerRole) -> u8 {
    match role {
        CardLayerRole::Background => 0,
        CardLayerRole::Frame => 1,
        CardLayerRole::SafeArea => 2,
        CardLayerRole::Foreground => 3,
        CardLayerRole::Title => 4,
    }
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
fn debug_hud_excludes_removed_scene_cycle_key() {
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
fn hot_reload_game_screen_reset_loses_screen_local_state() {
    let mut gesture_model = CardGestureModel {
        state: CardGestureState::Dragging,
        ..Default::default()
    };
    let mut slot_board = CardSlotBoardModel::default();
    let mut card_state_model = CardStateModel::default();
    let mut card_state = CardInspectionState {
        last_pointer_normalized: Vec2::ONE,
        target_rotation: Quat::from_rotation_y(1.0),
    };
    let mut flip_state = CardFlipState {
        target_y_rotation: 1.0,
        visible_face: CardFace::Back,
        ..Default::default()
    };
    let mut ticks = GameTicks(7);

    assert_eq!(slot_board.place_next_local(1, 0), Some(0));
    assert!(card_state_model.place_in_location(0));

    reset_active_screen_model_for_hot_reload(
        ActiveView::GameScene,
        Some(&mut gesture_model),
        Some(&mut slot_board),
        Some(&mut card_state_model),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &mut card_state,
        &mut flip_state,
        &mut ticks,
    );

    assert_eq!(gesture_model.state, CardGestureState::Idle);
    assert_eq!(slot_board.populated_count(), 0);
    assert_eq!(card_state_model.state(0), Some(CardState::Hand));
    assert_eq!(card_state.last_pointer_normalized, Vec2::ZERO);
    assert_eq!(flip_state.visible_face, CardFace::Front);
    assert_eq!(ticks.0, 0);
}

#[test]
fn hot_reload_deck_and_debug_screen_reset_screen_local_state() {
    let mut deck_screen_model = DeckScreenModel::default();
    deck_screen_model.open_editor();
    let mut card_state = CardInspectionState {
        last_pointer_normalized: Vec2::ONE,
        target_rotation: Quat::from_rotation_y(1.0),
    };
    let mut flip_state = CardFlipState {
        target_y_rotation: 1.0,
        visible_face: CardFace::Back,
        ..Default::default()
    };
    let mut ticks = GameTicks(11);

    reset_active_screen_model_for_hot_reload(
        ActiveView::DeckScene,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&mut deck_screen_model),
        None,
        &mut card_state,
        &mut flip_state,
        &mut ticks,
    );

    assert_eq!(deck_screen_model, DeckScreenModel::default());
    assert_eq!(card_state.last_pointer_normalized, Vec2::ZERO);
    assert_eq!(flip_state.visible_face, CardFace::Front);
    assert_eq!(ticks.0, 0);

    card_state.last_pointer_normalized = Vec2::ONE;
    flip_state.visible_face = CardFace::Back;
    ticks.0 = 5;

    reset_active_screen_model_for_hot_reload(
        ActiveView::DebugScene,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &mut card_state,
        &mut flip_state,
        &mut ticks,
    );

    assert_eq!(card_state.last_pointer_normalized, Vec2::ZERO);
    assert_eq!(flip_state.visible_face, CardFace::Front);
    assert_eq!(ticks.0, 0);
}

#[derive(Debug, PartialEq)]
struct RestartGameSnapshot {
    active_view: ActiveView,
    game_scene_root_count: usize,
    deck_scene_root_count: usize,
    debug_scene_root_count: usize,
    world_background_count: usize,
    hand_card_count: usize,
    game_round: u8,
    game_energy_available: i32,
    game_location_round: u8,
    game_hand_len: usize,
    slot_board_populated_count: usize,
    first_card_state: Option<CardState>,
    gesture_state: CardGestureState,
    card_last_pointer_normalized: Vec2,
    card_target_rotation: Quat,
}

fn add_restart_parity_resources(app: &mut App) {
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_asset::<Image>()
        .init_asset::<Font>()
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
        .init_resource::<PlayerDeckCollectionModel>()
        .init_resource::<GameDeckModel>()
        .init_resource::<GameHandModel>()
        .init_resource::<GameRoundModel>()
        .init_resource::<GameLocationModel>()
        .init_resource::<MatchModel>()
        .init_resource::<CpuBrainModel>()
        .init_resource::<CardUiState>()
        .init_resource::<MatchmakingModel>()
        .init_resource::<MetaGameSettingsModel>()
        .add_systems(Startup, setup_app_scene);
}

fn dirty_restart_parity_state(app: &mut App) {
    app.world_mut().resource_mut::<ActiveWorldModel>().index = 0;
    app.world_mut().resource_mut::<ActiveLocations>().indices = [5, 0, 1];
    app.world_mut().resource_mut::<GameRoundModel>().round = 4;
    app.world_mut()
        .resource_mut::<GameRoundModel>()
        .energy_available = 2;
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .reset_with_active_location_indices(&[5, 0, 1]);
    app.world_mut()
        .resource_mut::<GameLocationModel>()
        .set_round(4);
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
}

fn restart_game_snapshot(app: &mut App) -> RestartGameSnapshot {
    RestartGameSnapshot {
        active_view: *app.world().resource::<ActiveView>(),
        game_scene_root_count: app
            .world_mut()
            .query_filtered::<Entity, With<GameSceneRoot>>()
            .iter(app.world())
            .count(),
        deck_scene_root_count: app
            .world_mut()
            .query_filtered::<Entity, With<DeckSceneRoot>>()
            .iter(app.world())
            .count(),
        debug_scene_root_count: app
            .world_mut()
            .query_filtered::<Entity, With<DebugSceneRoot>>()
            .iter(app.world())
            .count(),
        world_background_count: app
            .world_mut()
            .query_filtered::<Entity, With<WorldBackground>>()
            .iter(app.world())
            .count(),
        hand_card_count: app
            .world_mut()
            .query_filtered::<Entity, (With<CardView>, With<LocalPlayerHandCardPreview>)>()
            .iter(app.world())
            .count(),
        game_round: app.world().resource::<GameRoundModel>().round,
        game_energy_available: app.world().resource::<GameRoundModel>().energy_available,
        game_location_round: app.world().resource::<GameLocationModel>().round,
        game_hand_len: app.world().resource::<GameHandModel>().len(),
        slot_board_populated_count: app
            .world()
            .resource::<CardSlotBoardModel>()
            .populated_count(),
        first_card_state: app.world().resource::<CardStateModel>().state(0),
        gesture_state: app.world().resource::<CardGestureModel>().state,
        card_last_pointer_normalized: app
            .world()
            .resource::<CardInspectionState>()
            .last_pointer_normalized,
        card_target_rotation: app
            .world()
            .resource::<CardInspectionState>()
            .target_rotation,
    }
}

fn restart_snapshot_from_matchmaking_entry() -> RestartGameSnapshot {
    let mut app = App::new();
    add_restart_parity_resources(&mut app);
    app.insert_resource(ActiveView::MatchmakingScene)
        .add_systems(Update, matchmaking_update_system);
    app.update();
    dirty_restart_parity_state(&mut app);
    {
        let mut matchmaking = app.world_mut().resource_mut::<MatchmakingModel>();
        matchmaking.phase = MatchmakingPhaseModel::Preparing;
        matchmaking.elapsed_seconds = MATCHMAKING_PREPARING_SECONDS;
    }
    fastrand::seed(7);
    app.update();
    restart_game_snapshot(&mut app)
}

fn restart_snapshot_from_r_shortcut() -> RestartGameSnapshot {
    let mut app = App::new();
    add_restart_parity_resources(&mut app);
    app.insert_resource(ActiveView::GameScene)
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, restart_app_scene);
    app.update();
    dirty_restart_parity_state(&mut app);
    fastrand::seed(7);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyR);
    app.update();
    restart_game_snapshot(&mut app)
}

fn restart_snapshot_from_restart_button() -> RestartGameSnapshot {
    let mut app = App::new();
    add_restart_parity_resources(&mut app);
    app.insert_resource(ActiveView::GameScene)
        .add_systems(Startup, setup_game_scene)
        .add_systems(Update, restart_game_control_button_system);
    app.update();
    dirty_restart_parity_state(&mut app);
    fastrand::seed(7);
    app.world_mut().spawn((
        GameControlButton::new(GameControlAction::Restart),
        Interaction::Pressed,
        BackgroundColor(END_ROUND_BUTTON_NORMAL_COLOR),
        BorderColor::all(END_ROUND_BUTTON_NORMAL_BORDER_COLOR),
    ));
    app.update();
    restart_game_snapshot(&mut app)
}

#[test]
fn game_restart_entry_points_land_in_identical_game_state() {
    let matchmaking_entry = restart_snapshot_from_matchmaking_entry();
    let r_shortcut = restart_snapshot_from_r_shortcut();
    let restart_button = restart_snapshot_from_restart_button();

    assert_eq!(r_shortcut, matchmaking_entry);
    assert_eq!(restart_button, matchmaking_entry);
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
    app.world_mut().resource_mut::<ActiveWorldModel>().index = 0;
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
    assert_eq!(app.world().resource::<GameTicks>().0, 42);
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
    assert_ne!(app.world().resource::<ActiveWorldModel>().index, 0);
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
