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
pub const CARD_WIDTH_WORLD_UNITS: f32 = 9.0 / 16.0;
pub const CARD_HEIGHT_WORLD_UNITS: f32 = 1.0;
pub const CARD_THICKNESS_WORLD_UNITS: f32 = 0.02;
pub const CARD_MAX_TILT_DEGREES: f32 = 20.0;
pub const CARD_SMOOTHING_RESPONSE_SECONDS: f32 = 0.1;
pub const CARD_TYPE_SLOT_COUNT: usize = 4;
pub const WORLD_THEME_COUNT: usize = 2;
pub const TACTICAL_LOCATION_COUNT: usize = 6;
pub const ACTIVE_LOCATION_COUNT: usize = 3;
pub const CARD_DEPTH_FACTOR_DEFAULT: f32 = 10.0;
pub const CARD_DEPTH_FACTOR_MIN: f32 = 0.0;
pub const CARD_DEPTH_FACTOR_MAX: f32 = 20.0;
pub const CARD_LAYER_SCALE_DEFAULT: f32 = 1.0;
pub const CARD_LAYER_SCALE_MIN: f32 = 0.0;
pub const CARD_LAYER_SCALE_MAX: f32 = 2.0;
pub const CARD_FLIP_DURATION_SECONDS: f32 = 0.45;
pub const CARD_BACK_TEXTURE_PATH: &str = "cards/card_structure/card_back_japan_realism.png";
pub const KAGE_REN_CARD_TYPE_ID: &str = "kage_ren";
pub const KAGE_REN_CARD_TYPE_NAME: &str = "KAGE REN";
pub const LORD_DAICHI_CARD_TYPE_ID: &str = "lord_daichi";
pub const LORD_DAICHI_CARD_TYPE_NAME: &str = "LORD DAICHI";
pub const SISTER_HOTARU_CARD_TYPE_ID: &str = "sister_hotaru";
pub const SISTER_HOTARU_CARD_TYPE_NAME: &str = "SISTER HOTARU";
pub const YOKAI_PLACEHOLDER_CARD_TYPE_ID: &str = "yokai_placeholder";
pub const YOKAI_PLACEHOLDER_CARD_TYPE_NAME: &str = "YOKAI TEST";
pub const BAMBOO_FOREST_WORLD_ID: &str = "bamboo_forest";
pub const COASTAL_HARBOR_WORLD_ID: &str = "coastal_harbor";

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Resource)]
pub enum ActiveScene {
    #[default]
    Game,
    CardBrowser,
}

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
pub struct CardType {
    pub id: &'static str,
    pub display_name: &'static str,
    pub background_texture: &'static str,
    pub frame_texture: &'static str,
    pub foreground_texture: &'static str,
    pub title_texture: &'static str,
    pub background_uses_frame_mask: bool,
    pub foreground_x_ratio: f32,
    pub foreground_y_ratio: f32,
    pub foreground_height_ratio: f32,
    pub title_y_ratio: f32,
}

impl CardType {
    pub const fn kage_ren() -> Self {
        Self {
            id: KAGE_REN_CARD_TYPE_ID,
            display_name: KAGE_REN_CARD_TYPE_NAME,
            background_texture: "cards/card_types/card_type_kage_ren/background.png",
            frame_texture: "cards/card_types/card_type_kage_ren/frame.png",
            foreground_texture: "cards/card_types/card_type_kage_ren/foreground_character.png",
            title_texture: "cards/card_types/card_type_kage_ren/title.png",
            background_uses_frame_mask: true,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: -0.02,
            foreground_height_ratio: 0.78,
            title_y_ratio: -0.32,
        }
    }

    pub const fn lord_daichi() -> Self {
        Self {
            id: LORD_DAICHI_CARD_TYPE_ID,
            display_name: LORD_DAICHI_CARD_TYPE_NAME,
            background_texture: "cards/card_types/card_type_lord_daichi/background.png",
            frame_texture: "cards/card_types/card_type_lord_daichi/frame.png",
            foreground_texture: "cards/card_types/card_type_lord_daichi/foreground_character.png",
            title_texture: "cards/card_types/card_type_lord_daichi/title.png",
            background_uses_frame_mask: false,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: -0.02,
            foreground_height_ratio: 0.82,
            title_y_ratio: -0.32,
        }
    }

    pub const fn sister_hotaru() -> Self {
        Self {
            id: SISTER_HOTARU_CARD_TYPE_ID,
            display_name: SISTER_HOTARU_CARD_TYPE_NAME,
            background_texture: "cards/card_types/card_type_sister_hotaru/background.png",
            frame_texture: "cards/card_types/card_type_sister_hotaru/frame.png",
            foreground_texture: "cards/card_types/card_type_sister_hotaru/foreground_character.png",
            title_texture: "cards/card_types/card_type_sister_hotaru/title.png",
            background_uses_frame_mask: false,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: -0.02,
            foreground_height_ratio: 0.78,
            title_y_ratio: -0.32,
        }
    }

    pub const fn yokai_placeholder() -> Self {
        Self {
            id: YOKAI_PLACEHOLDER_CARD_TYPE_ID,
            display_name: YOKAI_PLACEHOLDER_CARD_TYPE_NAME,
            background_texture: "cards/card_types/card_type_yokai_placeholder/background.png",
            frame_texture: "cards/card_types/card_type_yokai_placeholder/frame.png",
            foreground_texture: "cards/card_types/card_type_yokai_placeholder/foreground_character.png",
            title_texture: "cards/card_types/card_type_yokai_placeholder/title.png",
            background_uses_frame_mask: false,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: -0.02,
            foreground_height_ratio: 0.8,
            title_y_ratio: -0.32,
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct CardTypeRegistry {
    slots: Vec<Option<CardType>>,
}

impl Default for CardTypeRegistry {
    fn default() -> Self {
        Self {
            slots: vec![
                Some(CardType::kage_ren()),
                Some(CardType::lord_daichi()),
                Some(CardType::sister_hotaru()),
                Some(CardType::yokai_placeholder()),
            ],
        }
    }
}

impl CardTypeRegistry {
    pub fn card_types(&self) -> impl Iterator<Item = &CardType> {
        self.slots.iter().flatten()
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn available_count(&self) -> usize {
        self.slots.iter().flatten().count()
    }

    pub fn active_card_type(&self, active_card_type: &ActiveCardType) -> Option<&CardType> {
        self.slots
            .get(active_card_type.index)
            .and_then(Option::as_ref)
            .or_else(|| self.slots.iter().flatten().next())
    }

    pub fn next_available_index(&self, current_index: usize) -> usize {
        let available_indices: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, card_type)| card_type.as_ref().map(|_| index))
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

#[derive(Clone, Debug, PartialEq)]
pub struct WorldTheme {
    pub id: &'static str,
    pub display_name: &'static str,
    pub background_texture: &'static str,
}

impl WorldTheme {
    pub const fn bamboo_forest() -> Self {
        Self {
            id: BAMBOO_FOREST_WORLD_ID,
            display_name: "Bamboo Forest",
            background_texture: "worlds/bamboo_forest/world_background.png",
        }
    }

    pub const fn coastal_harbor() -> Self {
        Self {
            id: COASTAL_HARBOR_WORLD_ID,
            display_name: "Coastal Harbor",
            background_texture: "worlds/coastal_harbor/world_background.png",
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct WorldThemeRegistry {
    themes: Vec<WorldTheme>,
}

impl Default for WorldThemeRegistry {
    fn default() -> Self {
        Self {
            themes: vec![WorldTheme::bamboo_forest(), WorldTheme::coastal_harbor()],
        }
    }
}

impl WorldThemeRegistry {
    pub fn active_world_theme(&self, active_world_theme: &ActiveWorldTheme) -> &WorldTheme {
        self.themes
            .get(active_world_theme.index)
            .or_else(|| self.themes.first())
            .expect("world theme registry must contain at least one theme")
    }

    pub fn next_index(&self, current_index: usize) -> usize {
        if self.themes.is_empty() {
            return 0;
        }

        (current_index + 1) % self.themes.len()
    }

    pub fn len(&self) -> usize {
        self.themes.len()
    }
}

#[derive(Debug, Resource)]
pub struct ActiveWorldTheme {
    pub index: usize,
}

impl Default for ActiveWorldTheme {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl ActiveWorldTheme {
    pub fn toggle(&mut self, registry: &WorldThemeRegistry) {
        self.index = registry.next_index(self.index);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TacticalLocation {
    pub id: &'static str,
    pub display_name: &'static str,
    pub texture: &'static str,
}

impl TacticalLocation {
    pub const fn new(id: &'static str, display_name: &'static str, texture: &'static str) -> Self {
        Self {
            id,
            display_name,
            texture,
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct TacticalLocationRegistry {
    locations: Vec<TacticalLocation>,
}

impl Default for TacticalLocationRegistry {
    fn default() -> Self {
        Self {
            locations: vec![
                TacticalLocation::new(
                    "fortress_gate",
                    "Fortress Gate",
                    "locations/fortress_gate/location.png",
                ),
                TacticalLocation::new(
                    "bamboo_crossing",
                    "Bamboo Crossing",
                    "locations/bamboo_crossing/location.png",
                ),
                TacticalLocation::new(
                    "shrine_ruins",
                    "Shrine Ruins",
                    "locations/shrine_ruins/location.png",
                ),
                TacticalLocation::new(
                    "battlefield",
                    "Battlefield",
                    "locations/battlefield/location.png",
                ),
                TacticalLocation::new(
                    "spirit_well",
                    "Spirit Well",
                    "locations/spirit_well/location.png",
                ),
                TacticalLocation::new(
                    "market_square",
                    "Market Square",
                    "locations/market_square/location.png",
                ),
            ],
        }
    }
}

impl TacticalLocationRegistry {
    pub fn selected_locations(&self, active_locations: &ActiveLocations) -> Vec<&TacticalLocation> {
        active_locations
            .indices
            .iter()
            .filter_map(|index| self.locations.get(*index))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.locations.len()
    }
}

#[derive(Clone, Debug, Resource)]
pub struct ActiveLocations {
    pub indices: [usize; ACTIVE_LOCATION_COUNT],
    generation: usize,
}

impl Default for ActiveLocations {
    fn default() -> Self {
        Self {
            indices: [0, 1, 2],
            generation: 0,
        }
    }
}

impl ActiveLocations {
    pub fn reroll(
        &mut self,
        registry: &TacticalLocationRegistry,
        active_world_theme: &ActiveWorldTheme,
    ) {
        let count = registry.len().max(1);
        self.generation = self.generation.wrapping_add(1);
        let start = (active_world_theme.index + self.generation) % count;
        for (offset, index) in self.indices.iter_mut().enumerate() {
            *index = (start + offset) % count;
        }
    }
}

#[derive(Debug, Resource)]
pub struct ActiveCardType {
    pub index: usize,
}

impl Default for ActiveCardType {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl ActiveCardType {
    pub fn toggle(&mut self, registry: &CardTypeRegistry) {
        self.index = registry.next_available_index(self.index);
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CardFace {
    #[default]
    Front,
    Back,
}

#[derive(Debug, Resource)]
pub struct CardFlipState {
    pub current_y_rotation: f32,
    pub target_y_rotation: f32,
    pub visible_face: CardFace,
}

impl Default for CardFlipState {
    fn default() -> Self {
        Self {
            current_y_rotation: 0.0,
            target_y_rotation: 0.0,
            visible_face: CardFace::Front,
        }
    }
}

impl CardFlipState {
    pub const fn is_animating(&self) -> bool {
        (self.target_y_rotation - self.current_y_rotation).abs() > f32::EPSILON
    }

    pub fn request_flip(&mut self) {
        if self.is_animating() {
            if self.target_y_rotation > self.current_y_rotation {
                self.target_y_rotation -= std::f32::consts::PI;
            } else {
                self.target_y_rotation += std::f32::consts::PI;
            }
            return;
        }

        match Self::face_for_angle(self.target_y_rotation) {
            CardFace::Front => self.target_y_rotation += std::f32::consts::PI,
            CardFace::Back => self.target_y_rotation -= std::f32::consts::PI,
        }
    }

    pub fn advance(&mut self, delta_seconds: f32) {
        let remaining = self.target_y_rotation - self.current_y_rotation;
        if remaining.abs() <= f32::EPSILON {
            self.current_y_rotation = self.target_y_rotation;
            self.visible_face = Self::face_for_angle(self.current_y_rotation);
            return;
        }

        let max_step = (std::f32::consts::PI / CARD_FLIP_DURATION_SECONDS) * delta_seconds.max(0.0);
        if remaining.abs() <= max_step {
            self.current_y_rotation = self.target_y_rotation;
        } else {
            self.current_y_rotation += max_step * remaining.signum();
        }
        self.visible_face = Self::face_for_angle(self.current_y_rotation);
    }

    pub fn face_for_angle(angle: f32) -> CardFace {
        if angle.cos() >= 0.0 {
            CardFace::Front
        } else {
            CardFace::Back
        }
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.current_y_rotation)
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
    pub background_layer_scale: f32,
    pub frame_layer_scale: f32,
    pub foreground_layer_scale: f32,
    pub title_layer_scale: f32,
}

impl Default for CardUiState {
    fn default() -> Self {
        Self {
            depth_factor: CARD_DEPTH_FACTOR_DEFAULT,
            background_layer_scale: CARD_LAYER_SCALE_DEFAULT,
            frame_layer_scale: CARD_LAYER_SCALE_DEFAULT,
            foreground_layer_scale: CARD_LAYER_SCALE_DEFAULT,
            title_layer_scale: CARD_LAYER_SCALE_DEFAULT,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Resource, Serialize)]
pub struct CardSettingsStore {
    pub depth_factor: f32,
    #[serde(default = "default_card_layer_scale")]
    pub background_layer_scale: f32,
    #[serde(default = "default_card_layer_scale")]
    pub frame_layer_scale: f32,
    #[serde(default = "default_card_layer_scale")]
    pub foreground_layer_scale: f32,
    #[serde(default = "default_card_layer_scale")]
    pub title_layer_scale: f32,
}

impl Default for CardSettingsStore {
    fn default() -> Self {
        Self {
            depth_factor: CARD_DEPTH_FACTOR_DEFAULT,
            background_layer_scale: CARD_LAYER_SCALE_DEFAULT,
            frame_layer_scale: CARD_LAYER_SCALE_DEFAULT,
            foreground_layer_scale: CARD_LAYER_SCALE_DEFAULT,
            title_layer_scale: CARD_LAYER_SCALE_DEFAULT,
        }
    }
}

const fn default_card_layer_scale() -> f32 {
    CARD_LAYER_SCALE_DEFAULT
}

impl CardSettingsStore {
    pub fn from_state(state: &CardUiState) -> Self {
        Self {
            depth_factor: state.depth_factor,
            background_layer_scale: state.background_layer_scale,
            frame_layer_scale: state.frame_layer_scale,
            foreground_layer_scale: state.foreground_layer_scale,
            title_layer_scale: state.title_layer_scale,
        }
    }

    pub fn apply_to_state(&self, state: &mut CardUiState) {
        state.depth_factor = self
            .depth_factor
            .clamp(CARD_DEPTH_FACTOR_MIN, CARD_DEPTH_FACTOR_MAX);
        state.background_layer_scale = self
            .background_layer_scale
            .clamp(CARD_LAYER_SCALE_MIN, CARD_LAYER_SCALE_MAX);
        state.frame_layer_scale = self
            .frame_layer_scale
            .clamp(CARD_LAYER_SCALE_MIN, CARD_LAYER_SCALE_MAX);
        state.foreground_layer_scale = self
            .foreground_layer_scale
            .clamp(CARD_LAYER_SCALE_MIN, CARD_LAYER_SCALE_MAX);
        state.title_layer_scale = self
            .title_layer_scale
            .clamp(CARD_LAYER_SCALE_MIN, CARD_LAYER_SCALE_MAX);
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub is_fullscreen: bool,
    pub is_inspector_visible: bool,
    pub is_hot_reload_autorestart_enabled: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct DebugHudInputStore {
    #[serde(default)]
    pub is_fps_visible: bool,
    #[serde(default)]
    pub is_fullscreen: bool,
    #[serde(default)]
    pub is_inspector_visible: bool,
    #[serde(default)]
    pub is_hot_reload_autorestart_enabled: bool,
}

impl DebugHudInputStore {
    pub fn from_state(state: &DebugHudState) -> Self {
        Self {
            is_fps_visible: state.is_fps_visible,
            is_fullscreen: state.is_fullscreen,
            is_inspector_visible: state.is_inspector_visible,
            is_hot_reload_autorestart_enabled: state.is_hot_reload_autorestart_enabled,
        }
    }

    pub fn apply_to_state(&self, state: &mut DebugHudState) {
        state.is_fps_visible = self.is_fps_visible;
        state.is_fullscreen = self.is_fullscreen;
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

pub fn card_settings_path() -> PathBuf {
    workspace_root_path()
        .join("data")
        .join("local_storage")
        .join("card-settings.json")
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

pub fn create_card_settings_store() -> Result<Persistent<CardSettingsStore>, PersistenceError> {
    Persistent::<CardSettingsStore>::builder()
        .name("card settings")
        .format(StorageFormat::JsonPretty)
        .path(card_settings_path())
        .default(CardSettingsStore::default())
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
    fn card_settings_uses_workspace_local_storage() {
        let path = card_settings_path();
        assert!(
            path.ends_with(
                Path::new("data")
                    .join("local_storage")
                    .join("card-settings.json")
            )
        );
    }

    #[test]
    fn debug_hud_input_defaults_all_toggles_off() {
        let store = DebugHudInputStore::default();

        assert!(!store.is_fps_visible);
        assert!(!store.is_fullscreen);
        assert!(!store.is_inspector_visible);
        assert!(!store.is_hot_reload_autorestart_enabled);
    }

    #[test]
    fn card_defaults_match_japan_realism_card_ratio() {
        let defaults = CardInspectionDefaults::default();
        let expected_ratio = 16.0 / 9.0;
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
    fn card_type_registry_has_japan_realism_characters() {
        let registry = CardTypeRegistry::default();
        let active_card_type = ActiveCardType::default();

        assert_eq!(registry.slot_count(), CARD_TYPE_SLOT_COUNT);
        assert_eq!(registry.available_count(), 4);
        assert_eq!(
            registry
                .active_card_type(&active_card_type)
                .map(|card_type| card_type.id),
            Some(KAGE_REN_CARD_TYPE_ID)
        );
    }

    #[test]
    fn card_type_textures_match_asset_directory_casing() {
        let registry = CardTypeRegistry::default();
        let asset_root = workspace_root_path()
            .join("bevy")
            .join("crates")
            .join("game")
            .join("assets");

        for card_type in registry.slots.iter().flatten() {
            for texture_path in [
                card_type.background_texture,
                card_type.frame_texture,
                card_type.foreground_texture,
                card_type.title_texture,
            ] {
                assert!(
                    asset_root.join(texture_path).is_file(),
                    "missing card type texture at {}",
                    texture_path
                );
            }
        }
    }

    #[test]
    fn card_type_toggle_cycles_between_four_japan_realism_cards() {
        let registry = CardTypeRegistry::default();
        let mut active_card_type = ActiveCardType::default();

        active_card_type.toggle(&registry);
        assert_eq!(active_card_type.index, 1);
        assert_eq!(
            registry
                .active_card_type(&active_card_type)
                .map(|card_type| card_type.display_name),
            Some(LORD_DAICHI_CARD_TYPE_NAME)
        );

        active_card_type.toggle(&registry);
        assert_eq!(active_card_type.index, 2);
        assert_eq!(
            registry
                .active_card_type(&active_card_type)
                .map(|card_type| card_type.display_name),
            Some(SISTER_HOTARU_CARD_TYPE_NAME)
        );

        active_card_type.toggle(&registry);
        assert_eq!(active_card_type.index, 3);
        assert_eq!(
            registry
                .active_card_type(&active_card_type)
                .map(|card_type| card_type.display_name),
            Some(YOKAI_PLACEHOLDER_CARD_TYPE_NAME)
        );

        active_card_type.toggle(&registry);

        assert_eq!(active_card_type.index, 0);
        assert_eq!(
            registry
                .active_card_type(&active_card_type)
                .map(|card_type| card_type.display_name),
            Some(KAGE_REN_CARD_TYPE_NAME)
        );
    }

    #[test]
    fn card_flip_state_defaults_to_front_idle() {
        let state = CardFlipState::default();

        assert_eq!(state.current_y_rotation, 0.0);
        assert_eq!(state.target_y_rotation, 0.0);
        assert_eq!(state.visible_face, CardFace::Front);
        assert!(!state.is_animating());
    }

    #[test]
    fn card_flip_state_targets_180_degrees_per_request() {
        let mut state = CardFlipState::default();

        state.request_flip();

        assert_eq!(state.target_y_rotation, std::f32::consts::PI);
        assert!(state.is_animating());
    }

    #[test]
    fn card_flip_state_switches_face_after_midpoint() {
        assert_eq!(CardFlipState::face_for_angle(0.0), CardFace::Front);
        assert_eq!(
            CardFlipState::face_for_angle(std::f32::consts::FRAC_PI_2 - 0.01),
            CardFace::Front
        );
        assert_eq!(
            CardFlipState::face_for_angle(std::f32::consts::FRAC_PI_2 + 0.01),
            CardFace::Back
        );
        assert_eq!(
            CardFlipState::face_for_angle(std::f32::consts::PI),
            CardFace::Back
        );
    }

    #[test]
    fn card_flip_state_reverses_mid_animation_from_current_progress() {
        let mut state = CardFlipState::default();

        state.request_flip();
        state.current_y_rotation = std::f32::consts::FRAC_PI_2;
        state.request_flip();

        assert_eq!(state.target_y_rotation, 0.0);
    }

    #[test]
    fn card_back_texture_uses_card_structure_asset_path() {
        assert_eq!(
            CARD_BACK_TEXTURE_PATH,
            "cards/card_structure/card_back_japan_realism.png"
        );
        assert!(!CARD_BACK_TEXTURE_PATH.contains("card_type_"));
    }

    #[test]
    fn world_theme_registry_cycles_between_bamboo_forest_and_coastal_harbor() {
        let registry = WorldThemeRegistry::default();
        let mut active_world_theme = ActiveWorldTheme::default();

        assert_eq!(registry.len(), WORLD_THEME_COUNT);
        assert_eq!(
            registry.active_world_theme(&active_world_theme).id,
            BAMBOO_FOREST_WORLD_ID
        );

        active_world_theme.toggle(&registry);
        assert_eq!(
            registry.active_world_theme(&active_world_theme).id,
            COASTAL_HARBOR_WORLD_ID
        );

        active_world_theme.toggle(&registry);
        assert_eq!(
            registry.active_world_theme(&active_world_theme).id,
            BAMBOO_FOREST_WORLD_ID
        );
    }

    #[test]
    fn active_locations_selects_three_locations_from_six() {
        let registry = TacticalLocationRegistry::default();
        let mut active_locations = ActiveLocations::default();
        let active_world_theme = ActiveWorldTheme::default();

        assert_eq!(registry.len(), TACTICAL_LOCATION_COUNT);
        assert_eq!(
            registry.selected_locations(&active_locations).len(),
            ACTIVE_LOCATION_COUNT
        );

        active_locations.reroll(&registry, &active_world_theme);

        assert_eq!(
            registry.selected_locations(&active_locations).len(),
            ACTIVE_LOCATION_COUNT
        );
        assert!(
            active_locations
                .indices
                .iter()
                .all(|index| *index < TACTICAL_LOCATION_COUNT)
        );
    }

    #[test]
    fn card_ui_depth_factor_defaults_to_current_parallax_strength() {
        let state = CardUiState::default();

        assert_eq!(state.depth_factor, CARD_DEPTH_FACTOR_DEFAULT);
        assert_eq!(state.depth_multiplier(), 1.0);
        assert_eq!(state.background_layer_scale, CARD_LAYER_SCALE_DEFAULT);
        assert_eq!(state.frame_layer_scale, CARD_LAYER_SCALE_DEFAULT);
        assert_eq!(state.foreground_layer_scale, CARD_LAYER_SCALE_DEFAULT);
        assert_eq!(state.title_layer_scale, CARD_LAYER_SCALE_DEFAULT);
    }

    #[test]
    fn card_ui_depth_factor_scales_from_coplanar_to_double_strength() {
        let mut state = CardUiState {
            depth_factor: CARD_DEPTH_FACTOR_MIN,
            ..Default::default()
        };

        assert_eq!(state.depth_multiplier(), 0.0);

        state.depth_factor = CARD_DEPTH_FACTOR_MAX;

        assert_eq!(state.depth_multiplier(), 2.0);
    }

    #[test]
    fn card_settings_applies_depth_factor_to_card_ui_state() {
        let settings = CardSettingsStore {
            depth_factor: 7.5,
            background_layer_scale: 0.5,
            frame_layer_scale: 0.75,
            foreground_layer_scale: 1.25,
            title_layer_scale: 1.5,
        };
        let mut state = CardUiState::default();

        settings.apply_to_state(&mut state);

        assert_eq!(state.depth_factor, 7.5);
        assert_eq!(state.background_layer_scale, 0.5);
        assert_eq!(state.frame_layer_scale, 0.75);
        assert_eq!(state.foreground_layer_scale, 1.25);
        assert_eq!(state.title_layer_scale, 1.5);
    }

    #[test]
    fn card_settings_clamps_depth_factor_to_supported_range() {
        let settings = CardSettingsStore {
            depth_factor: CARD_DEPTH_FACTOR_MAX + 1.0,
            background_layer_scale: CARD_LAYER_SCALE_MIN - 1.0,
            frame_layer_scale: CARD_LAYER_SCALE_MAX + 1.0,
            foreground_layer_scale: CARD_LAYER_SCALE_MAX + 1.0,
            title_layer_scale: CARD_LAYER_SCALE_MIN - 1.0,
        };
        let mut state = CardUiState::default();

        settings.apply_to_state(&mut state);

        assert_eq!(state.depth_factor, CARD_DEPTH_FACTOR_MAX);
        assert_eq!(state.background_layer_scale, CARD_LAYER_SCALE_MIN);
        assert_eq!(state.frame_layer_scale, CARD_LAYER_SCALE_MAX);
        assert_eq!(state.foreground_layer_scale, CARD_LAYER_SCALE_MAX);
        assert_eq!(state.title_layer_scale, CARD_LAYER_SCALE_MIN);
    }
}
