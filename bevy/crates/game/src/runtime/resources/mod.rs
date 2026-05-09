use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[cfg(feature = "desktop-hot-reload")]
use std::sync::atomic::{AtomicU64, Ordering};

const WORKSPACE_RELATIVE_FROM_GAME_CRATE: [&str; 3] = ["..", "..", ".."];
#[cfg(feature = "desktop-hot-reload")]
static DESKTOP_HOT_RELOAD_PATCH_COUNT: AtomicU64 = AtomicU64::new(0);

pub const PRIMARY_CAMERA_FOV_RADIANS: f32 = std::f32::consts::FRAC_PI_4;
pub const PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.5;
pub const PRIMARY_CAMERA_NEAR: f32 = 0.1;
pub const PRIMARY_CAMERA_FAR: f32 = 1000.0;
pub const CARD_WIDTH_WORLD_UNITS: f32 = 63.0 / 88.0;
pub const CARD_HEIGHT_WORLD_UNITS: f32 = 1.0;
pub const CARD_THICKNESS_WORLD_UNITS: f32 = 0.02;
pub const CARD_MAX_TILT_DEGREES: f32 = 20.0;
pub const CARD_SMOOTHING_RESPONSE_SECONDS: f32 = 0.1;
pub const CARD_THEME_SLOT_COUNT: usize = 2;
pub const CARD_DEPTH_FACTOR_DEFAULT: f32 = 10.0;
pub const CARD_DEPTH_FACTOR_MIN: f32 = 0.0;
pub const CARD_DEPTH_FACTOR_MAX: f32 = 20.0;
pub const SKYBOLT_THEME_ID: &str = "skybolt";
pub const SKYBOLT_THEME_NAME: &str = "SKYBOLT";
pub const TAR_THEME_ID: &str = "tar";
pub const TAR_THEME_NAME: &str = "TAR";

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

#[derive(Clone, Debug, Resource)]
pub struct PrimaryCameraDefaults {
    pub position: Vec3,
    pub target: Vec3,
    pub fov_radians: f32,
    pub near: f32,
    pub far: f32,
    pub clear_color: Color,
}

impl Default for PrimaryCameraDefaults {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN),
            target: Vec3::ZERO,
            fov_radians: PRIMARY_CAMERA_FOV_RADIANS,
            near: PRIMARY_CAMERA_NEAR,
            far: PRIMARY_CAMERA_FAR,
            clear_color: Color::srgb(0.08, 0.08, 0.08),
        }
    }
}

impl PrimaryCameraDefaults {
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position).looking_at(self.target, Vec3::Y)
    }
}

#[derive(Clone, Debug, Resource)]
pub struct CardInspectionDefaults {
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub max_tilt_radians: f32,
    pub smoothing_response_seconds: f32,
    pub material_color: Color,
}

impl Default for CardInspectionDefaults {
    fn default() -> Self {
        Self {
            width: CARD_WIDTH_WORLD_UNITS,
            height: CARD_HEIGHT_WORLD_UNITS,
            thickness: CARD_THICKNESS_WORLD_UNITS,
            max_tilt_radians: CARD_MAX_TILT_DEGREES.to_radians(),
            smoothing_response_seconds: CARD_SMOOTHING_RESPONSE_SECONDS,
            material_color: Color::WHITE,
        }
    }
}

impl CardInspectionDefaults {
    pub fn height_width_ratio(&self) -> f32 {
        self.height / self.width
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CardTheme {
    pub id: &'static str,
    pub display_name: &'static str,
    pub background_texture: &'static str,
    pub frame_texture: &'static str,
    pub foreground_texture: &'static str,
    pub title_texture: &'static str,
    pub foreground_x_ratio: f32,
    pub foreground_y_ratio: f32,
    pub foreground_height_ratio: f32,
    pub title_y_ratio: f32,
}

impl CardTheme {
    pub const fn skybolt() -> Self {
        Self {
            id: SKYBOLT_THEME_ID,
            display_name: SKYBOLT_THEME_NAME,
            background_texture: "Cards/CardThemes/CardTheme_SkyBolt/background_clouds.png",
            frame_texture: "Cards/CardThemes/CardTheme_SkyBolt/frame_pinstripe.png",
            foreground_texture: "Cards/CardThemes/CardTheme_SkyBolt/foreground_character.png",
            title_texture: "Cards/CardThemes/CardTheme_SkyBolt/title_skybolt.png",
            foreground_x_ratio: 0.02,
            foreground_y_ratio: 0.05,
            foreground_height_ratio: 0.82,
            title_y_ratio: -0.32,
        }
    }

    pub const fn tar() -> Self {
        Self {
            id: TAR_THEME_ID,
            display_name: TAR_THEME_NAME,
            background_texture: "Cards/CardThemes/CardTheme_Tar/background_cafe.png",
            frame_texture: "Cards/CardThemes/CardTheme_Tar/frame_tar.png",
            foreground_texture: "Cards/CardThemes/CardTheme_Tar/foreground_minotaur.png",
            title_texture: "Cards/CardThemes/CardTheme_Tar/title_tar.png",
            foreground_x_ratio: 0.0,
            foreground_y_ratio: -0.03,
            foreground_height_ratio: 0.98,
            title_y_ratio: 0.43,
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct CardThemeRegistry {
    slots: Vec<Option<CardTheme>>,
}

impl Default for CardThemeRegistry {
    fn default() -> Self {
        Self {
            slots: vec![Some(CardTheme::skybolt()), Some(CardTheme::tar())],
        }
    }
}

impl CardThemeRegistry {
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn available_count(&self) -> usize {
        self.slots.iter().flatten().count()
    }

    pub fn active_theme(&self, active_theme: &ActiveCardTheme) -> Option<&CardTheme> {
        self.slots
            .get(active_theme.index)
            .and_then(Option::as_ref)
            .or_else(|| self.slots.iter().flatten().next())
    }

    pub fn next_available_index(&self, current_index: usize) -> usize {
        let available_indices: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, theme)| theme.as_ref().map(|_| index))
            .collect();

        if available_indices.len() <= 1 {
            return available_indices.first().copied().unwrap_or(0);
        }

        available_indices
            .iter()
            .position(|index| *index == current_index)
            .map(|position| available_indices[(position + 1) % available_indices.len()])
            .unwrap_or(available_indices[0])
    }
}

#[derive(Debug, Resource)]
pub struct ActiveCardTheme {
    pub index: usize,
}

impl Default for ActiveCardTheme {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl ActiveCardTheme {
    pub fn toggle(&mut self, registry: &CardThemeRegistry) {
        self.index = registry.next_available_index(self.index);
    }
}

#[derive(Debug, Resource)]
pub struct CardInspectionState {
    pub last_pointer_normalized: Vec2,
    pub target_rotation: Quat,
}

impl Default for CardInspectionState {
    fn default() -> Self {
        Self {
            last_pointer_normalized: Vec2::ZERO,
            target_rotation: Quat::IDENTITY,
        }
    }
}

#[derive(Debug, Resource)]
pub struct CardUiState {
    pub depth_factor: f32,
}

impl Default for CardUiState {
    fn default() -> Self {
        Self {
            depth_factor: CARD_DEPTH_FACTOR_DEFAULT,
        }
    }
}

impl CardUiState {
    pub fn depth_multiplier(&self) -> f32 {
        (self.depth_factor / CARD_DEPTH_FACTOR_DEFAULT).clamp(
            CARD_DEPTH_FACTOR_MIN / CARD_DEPTH_FACTOR_DEFAULT,
            CARD_DEPTH_FACTOR_MAX / CARD_DEPTH_FACTOR_DEFAULT,
        )
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub is_inspector_visible: bool,
    pub is_hot_reload_autorestart_enabled: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct DebugHudInputStore {
    pub is_fps_visible: bool,
    pub is_inspector_visible: bool,
    pub is_hot_reload_autorestart_enabled: bool,
}

impl DebugHudInputStore {
    pub fn from_state(state: &DebugHudState) -> Self {
        Self {
            is_fps_visible: state.is_fps_visible,
            is_inspector_visible: state.is_inspector_visible,
            is_hot_reload_autorestart_enabled: state.is_hot_reload_autorestart_enabled,
        }
    }

    pub fn apply_to_state(&self, state: &mut DebugHudState) {
        state.is_fps_visible = self.is_fps_visible;
        state.is_inspector_visible = self.is_inspector_visible;
        state.is_hot_reload_autorestart_enabled = self.is_hot_reload_autorestart_enabled;
    }
}

#[derive(Resource, Debug, Default)]
pub struct WindowPlacementState {
    pub current: Option<WindowPlacement>,
    pub restored: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct WindowPlacementStore {
    pub current: Option<WindowPlacement>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WindowPlacement {
    pub window_position: IVec2,
    pub window_size: UVec2,
    pub monitor_name: Option<String>,
    pub monitor_position: IVec2,
    pub monitor_size: UVec2,
    pub relative_position: IVec2,
}

pub fn window_placement_path() -> PathBuf {
    workspace_root_path()
        .join("data")
        .join("local_storage")
        .join("window-placement.json")
}

pub fn debug_hud_input_path() -> PathBuf {
    workspace_root_path()
        .join("data")
        .join("local_storage")
        .join("debug-hud-input.json")
}

pub fn create_window_placement_store() -> Result<Persistent<WindowPlacementStore>, PersistenceError>
{
    Persistent::<WindowPlacementStore>::builder()
        .name("window placement")
        .format(StorageFormat::JsonPretty)
        .path(window_placement_path())
        .default(WindowPlacementStore::default())
        .revertible(true)
        .revert_to_default_on_deserialization_errors(true)
        .build()
}

pub fn create_debug_hud_input_store() -> Result<Persistent<DebugHudInputStore>, PersistenceError> {
    Persistent::<DebugHudInputStore>::builder()
        .name("debug hud input")
        .format(StorageFormat::JsonPretty)
        .path(debug_hud_input_path())
        .default(DebugHudInputStore::default())
        .revertible(true)
        .revert_to_default_on_deserialization_errors(true)
        .build()
}

pub fn load_window_placement() -> Option<WindowPlacement> {
    #[cfg(target_arch = "wasm32")]
    {
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        valid_window_placement(create_window_placement_store().ok()?.current.clone())
    }
}

#[cfg(feature = "desktop-hot-reload")]
pub fn record_desktop_hot_reload_patch() {
    DESKTOP_HOT_RELOAD_PATCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[cfg(feature = "desktop-hot-reload")]
pub fn desktop_hot_reload_patch_count() -> u64 {
    DESKTOP_HOT_RELOAD_PATCH_COUNT.load(Ordering::Relaxed)
}

pub fn valid_window_placement(placement: Option<WindowPlacement>) -> Option<WindowPlacement> {
    placement.filter(is_valid_window_placement)
}

fn workspace_root_path() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    for component in WORKSPACE_RELATIVE_FROM_GAME_CRATE {
        path.push(component);
    }
    path
}

fn is_valid_window_placement(placement: &WindowPlacement) -> bool {
    placement.window_size.x > 0 && placement.window_size.y > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_placement_serializes_position_size_and_screen_identity() {
        let placement = WindowPlacement {
            window_position: IVec2::new(100, 200),
            window_size: UVec2::new(800, 600),
            monitor_name: Some("Display 1".to_string()),
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(100, 200),
        };

        let serialized = serde_json::to_string(&placement).unwrap();
        let deserialized: WindowPlacement = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, placement);
    }

    #[test]
    fn window_placement_requires_positive_size() {
        let placement = WindowPlacement {
            window_position: IVec2::new(100, 200),
            window_size: UVec2::ZERO,
            monitor_name: None,
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(100, 200),
        };

        assert_eq!(valid_window_placement(Some(placement)), None);
    }

    #[test]
    fn window_placement_uses_workspace_local_storage() {
        let path = window_placement_path();
        assert!(
            path.ends_with(
                Path::new("data")
                    .join("local_storage")
                    .join("window-placement.json")
            )
        );
        assert!(!path.components().any(|component| {
            component.as_os_str() == "game" && path.to_string_lossy().contains("game\\data")
        }));
    }

    #[test]
    fn debug_hud_input_uses_workspace_local_storage() {
        let path = debug_hud_input_path();
        assert!(
            path.ends_with(
                Path::new("data")
                    .join("local_storage")
                    .join("debug-hud-input.json")
            )
        );
    }

    #[test]
    fn debug_hud_input_defaults_all_toggles_off() {
        let store = DebugHudInputStore::default();

        assert!(!store.is_fps_visible);
        assert!(!store.is_inspector_visible);
        assert!(!store.is_hot_reload_autorestart_enabled);
    }

    #[test]
    fn card_defaults_match_poker_card_ratio() {
        let defaults = CardInspectionDefaults::default();
        let expected_ratio = 88.0 / 63.0;
        let tolerance = expected_ratio * 0.02;

        assert!((defaults.height_width_ratio() - expected_ratio).abs() <= tolerance);
        assert_eq!(
            defaults.max_tilt_radians,
            CARD_MAX_TILT_DEGREES.to_radians()
        );
        assert_eq!(
            defaults.smoothing_response_seconds,
            CARD_SMOOTHING_RESPONSE_SECONDS
        );
    }

    #[test]
    fn card_defaults_fit_inside_unit_bounds() {
        let defaults = CardInspectionDefaults::default();

        assert!(defaults.width <= 1.0);
        assert!(defaults.height <= 1.0);
        assert!(defaults.thickness <= 1.0);
        assert_eq!(defaults.height, 1.0);
    }

    #[test]
    fn card_theme_registry_has_skybolt_and_tar() {
        let registry = CardThemeRegistry::default();
        let active_theme = ActiveCardTheme::default();

        assert_eq!(registry.slot_count(), CARD_THEME_SLOT_COUNT);
        assert_eq!(registry.available_count(), 2);
        assert_eq!(
            registry.active_theme(&active_theme).map(|theme| theme.id),
            Some(SKYBOLT_THEME_ID)
        );
    }

    #[test]
    fn theme_toggle_cycles_between_skybolt_and_tar() {
        let registry = CardThemeRegistry::default();
        let mut active_theme = ActiveCardTheme::default();

        active_theme.toggle(&registry);
        assert_eq!(active_theme.index, 1);
        assert_eq!(
            registry
                .active_theme(&active_theme)
                .map(|theme| theme.display_name),
            Some(TAR_THEME_NAME)
        );

        active_theme.toggle(&registry);

        assert_eq!(active_theme.index, 0);
        assert_eq!(
            registry
                .active_theme(&active_theme)
                .map(|theme| theme.display_name),
            Some(SKYBOLT_THEME_NAME)
        );
    }

    #[test]
    fn card_ui_depth_factor_defaults_to_current_parallax_strength() {
        let state = CardUiState::default();

        assert_eq!(state.depth_factor, CARD_DEPTH_FACTOR_DEFAULT);
        assert_eq!(state.depth_multiplier(), 1.0);
    }

    #[test]
    fn card_ui_depth_factor_scales_from_coplanar_to_double_strength() {
        let mut state = CardUiState {
            depth_factor: CARD_DEPTH_FACTOR_MIN,
        };

        assert_eq!(state.depth_multiplier(), 0.0);

        state.depth_factor = CARD_DEPTH_FACTOR_MAX;

        assert_eq!(state.depth_multiplier(), 2.0);
    }
}
