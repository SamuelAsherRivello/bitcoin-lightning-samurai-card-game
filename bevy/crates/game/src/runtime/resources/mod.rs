use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[cfg(feature = "desktop-hot-reload")]
use std::sync::atomic::{AtomicU64, Ordering};

pub mod card_gesture_model;
pub mod card_slot_model;
pub mod debug_drawing_model;
pub mod point_model;

pub use card_gesture_model::*;
pub use card_slot_model::*;
pub use debug_drawing_model::*;
pub use point_model::*;

const WORKSPACE_RELATIVE_FROM_GAME_CRATE: [&str; 3] = ["..", "..", ".."];
#[cfg(feature = "desktop-hot-reload")]
static DESKTOP_HOT_RELOAD_PATCH_COUNT: AtomicU64 = AtomicU64::new(0);

pub const PRIMARY_CAMERA_FOV_RADIANS: f32 = std::f32::consts::FRAC_PI_4;
pub const PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 1.5;
pub const PRIMARY_CAMERA_NEAR: f32 = 0.1;
pub const PRIMARY_CAMERA_FAR: f32 = 1000.0;
pub const CARD_HEIGHT_WORLD_UNITS: f32 = 1.0;
pub const CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT: f32 = 2.0 / 3.0;
pub const CARD_WIDTH_WORLD_UNITS: f32 =
    CARD_HEIGHT_WORLD_UNITS * CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT;
pub const CARD_THICKNESS_WORLD_UNITS: f32 = 0.02;
pub const CARD_MAX_TILT_DEGREES: f32 = 20.0;
pub const CARD_SMOOTHING_RESPONSE_SECONDS: f32 = 0.1;
pub const CARD_MODEL_SLOT_COUNT: usize = 4;
pub const STARTING_DECK_CARD_COUNT: usize = 12;
pub const STARTING_HAND_CARD_COUNT: usize = 5;
pub const STARTING_HAND_REPEATS_PER_CARD: usize = 3;
pub const DEFAULT_PLAYER_NAME: &str = "Player 01";
pub const DEFAULT_DECK_NAME: &str = "Deck01";
pub const WORLD_MODEL_COUNT: usize = 2;
pub const LOCATION_MODEL_COUNT: usize = 6;
pub const ACTIVE_LOCATION_COUNT: usize = 3;
pub const CARD_DEPTH_FACTOR_DEFAULT: f32 = 10.0;
pub const CARD_DEPTH_FACTOR_MIN: f32 = 0.0;
pub const CARD_DEPTH_FACTOR_MAX: f32 = 20.0;
pub const CARD_LAYER_SCALE_DEFAULT: f32 = 1.0;
pub const CARD_LAYER_SCALE_MIN: f32 = 0.0;
pub const CARD_LAYER_SCALE_MAX: f32 = 2.0;
pub const CARD_FLIP_DURATION_SECONDS: f32 = 0.45;
pub const CARD_BACK_TEXTURE_PATH: &str = "themes/theme_japan/cards/card_back.png";
pub const CARD_SAFE_AREA_TEXTURE_PATH: &str = "themes/theme_japan/cards/safe_area.png";
pub const KAGE_REN_CARD_MODEL_ID: &str = "kage_ren";
pub const KAGE_REN_CARD_MODEL_NAME: &str = "KAGE REN";
pub const LORD_DAICHI_CARD_MODEL_ID: &str = "lord_daichi";
pub const LORD_DAICHI_CARD_MODEL_NAME: &str = "LORD DAICHI";
pub const SISTER_HOTARU_CARD_MODEL_ID: &str = "sister_hotaru";
pub const SISTER_HOTARU_CARD_MODEL_NAME: &str = "SISTER HOTARU";
pub const YOKAI_PLACEHOLDER_CARD_MODEL_ID: &str = "yokai_placeholder";
pub const YOKAI_PLACEHOLDER_CARD_MODEL_NAME: &str = "YOKAI TEST";
pub const BAMBOO_FOREST_WORLD_ID: &str = "bamboo_forest";
pub const COASTAL_HARBOR_WORLD_ID: &str = "coastal_harbor";

/// HUMAN: Frame counter shared by runtime systems.
/// AI: Keep as the lightweight app tick resource; do not mix with gameplay turn state.
#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

/// HUMAN: Selects the single sub-screen view loaded on top of the persistent AppScene.
/// AI: Variants are views, not scenes; AppScene remains always present.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Resource)]
pub enum ActiveView {
    #[default]
    GameView,
    DeckBuilderScene,
    DebugSettingsScene,
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

/// HUMAN: Card data model used to build a rendered CardView.
/// AI: Keep asset paths here as data; rendering belongs in CardViewBundle and systems.
#[derive(Clone, Debug, PartialEq)]
pub struct CardModel {
    pub id: &'static str,
    pub display_name: &'static str,
    pub cost: CostPointModel,
    pub base_power: PowerPointModel,
    pub background_texture: &'static str,
    pub frame_texture: &'static str,
    pub foreground_texture: &'static str,
    pub title_texture: &'static str,
    pub background_uses_frame_mask: bool,
    pub foreground_width_ratio: f32,
    pub foreground_x_ratio: f32,
    pub foreground_y_ratio: f32,
    pub foreground_height_ratio: f32,
    pub title_y_ratio: f32,
}

impl CardModel {
    pub fn kage_ren() -> Self {
        Self {
            id: KAGE_REN_CARD_MODEL_ID,
            display_name: KAGE_REN_CARD_MODEL_NAME,
            cost: CostPointModel::random(),
            base_power: PowerPointModel::random(),
            background_texture: "themes/theme_japan/cards/card_kage_ren/background.png",
            frame_texture: "themes/theme_japan/cards/card_kage_ren/frame.png",
            foreground_texture: "themes/theme_japan/cards/card_kage_ren/foreground_character.png",
            title_texture: "themes/theme_japan/cards/card_kage_ren/title.png",
            background_uses_frame_mask: true,
            foreground_width_ratio: 1.0,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: 0.0,
            foreground_height_ratio: 1.0,
            title_y_ratio: -0.32,
        }
    }

    pub fn lord_daichi() -> Self {
        Self {
            id: LORD_DAICHI_CARD_MODEL_ID,
            display_name: LORD_DAICHI_CARD_MODEL_NAME,
            cost: CostPointModel::random(),
            base_power: PowerPointModel::random(),
            background_texture: "themes/theme_japan/cards/card_lord_daichi/background.png",
            frame_texture: "themes/theme_japan/cards/card_lord_daichi/frame.png",
            foreground_texture: "themes/theme_japan/cards/card_lord_daichi/foreground_character.png",
            title_texture: "themes/theme_japan/cards/card_lord_daichi/title.png",
            background_uses_frame_mask: false,
            foreground_width_ratio: 1.0,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: 0.0,
            foreground_height_ratio: 1.0,
            title_y_ratio: -0.32,
        }
    }

    pub fn sister_hotaru() -> Self {
        Self {
            id: SISTER_HOTARU_CARD_MODEL_ID,
            display_name: SISTER_HOTARU_CARD_MODEL_NAME,
            cost: CostPointModel::random(),
            base_power: PowerPointModel::random(),
            background_texture: "themes/theme_japan/cards/card_sister_hotaru/background.png",
            frame_texture: "themes/theme_japan/cards/card_sister_hotaru/frame.png",
            foreground_texture: "themes/theme_japan/cards/card_sister_hotaru/foreground_character.png",
            title_texture: "themes/theme_japan/cards/card_sister_hotaru/title.png",
            background_uses_frame_mask: false,
            foreground_width_ratio: 1.0,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: 0.0,
            foreground_height_ratio: 1.0,
            title_y_ratio: -0.32,
        }
    }

    pub fn yokai_placeholder() -> Self {
        Self {
            id: YOKAI_PLACEHOLDER_CARD_MODEL_ID,
            display_name: YOKAI_PLACEHOLDER_CARD_MODEL_NAME,
            cost: CostPointModel::random(),
            base_power: PowerPointModel::random(),
            background_texture: "themes/theme_japan/cards/card_yokai_placeholder/background.png",
            frame_texture: "themes/theme_japan/cards/card_yokai_placeholder/frame.png",
            foreground_texture: "themes/theme_japan/cards/card_yokai_placeholder/foreground_character.png",
            title_texture: "themes/theme_japan/cards/card_yokai_placeholder/title.png",
            background_uses_frame_mask: false,
            foreground_width_ratio: 1.0,
            foreground_x_ratio: 0.0,
            foreground_y_ratio: 0.0,
            foreground_height_ratio: 1.0,
            title_y_ratio: -0.32,
        }
    }
}

/// HUMAN: Registry of available card data models.
/// AI: This owns CardModel data only; avoid adding render handles or ECS entities here.
#[derive(Clone, Debug, Resource)]
pub struct CardModelRegistry {
    slots: Vec<Option<CardModel>>,
}

impl Default for CardModelRegistry {
    fn default() -> Self {
        Self {
            slots: vec![
                Some(CardModel::kage_ren()),
                Some(CardModel::lord_daichi()),
                Some(CardModel::sister_hotaru()),
                Some(CardModel::yokai_placeholder()),
            ],
        }
    }
}

impl CardModelRegistry {
    pub fn card_models(&self) -> impl Iterator<Item = &CardModel> {
        self.slots.iter().flatten()
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn available_count(&self) -> usize {
        self.slots.iter().flatten().count()
    }

    pub fn active_card_model(&self, active_card_model: &ActiveCardModel) -> Option<&CardModel> {
        self.slots
            .get(active_card_model.index)
            .and_then(Option::as_ref)
            .or_else(|| self.slots.iter().flatten().next())
    }

    pub fn next_available_index(&self, current_index: usize) -> usize {
        let available_indices: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, card_model)| card_model.as_ref().map(|_| index))
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

    pub fn card_model_for_id(&self, card_id: &str) -> Option<&CardModel> {
        self.slots
            .iter()
            .flatten()
            .find(|card_model| card_model.id == card_id)
    }
}

/// HUMAN: Runtime deck list definition for a single player.
/// AI: Deck data is serializable and decoupled from board/hand/runtime state.
#[derive(Clone, Debug, Deserialize, PartialEq, Resource, Serialize)]
pub struct DeckModel {
    pub name: String,
    pub cards: Vec<String>,
}

impl Default for DeckModel {
    fn default() -> Self {
        Self::with_name_and_cards(DEFAULT_DECK_NAME, random_shuffled_default_deck_cards())
    }
}

impl DeckModel {
    pub fn with_name_and_cards(name: &str, cards: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            cards,
        }
    }

    pub fn has_cards(&self) -> bool {
        !self.cards.is_empty()
    }
}

/// HUMAN: Player deck bucket for persisted deck collections.
/// AI: Keep one model per player, each with an ordered list of deck IDs.
#[derive(Clone, Debug, Deserialize, PartialEq, Resource, Serialize)]
pub struct PlayerModel {
    pub name: String,
    pub decks: Vec<DeckModel>,
}

impl Default for PlayerModel {
    fn default() -> Self {
        Self {
            name: DEFAULT_PLAYER_NAME.to_string(),
            decks: vec![DeckModel::default()],
        }
    }
}

impl PlayerModel {
    pub fn primary_deck(&self) -> Option<&DeckModel> {
        self.decks.first()
    }
}

/// HUMAN: Persisted deck collection owning all user-defined players.
/// AI: This is the long-lived source of deck data loaded once during startup.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct PlayerDeckCollectionModel {
    pub players: Vec<PlayerModel>,
}

impl PlayerDeckCollectionModel {
    pub fn primary_player(&self) -> Option<&PlayerModel> {
        self.players.first()
    }

    pub fn primary_deck(&self) -> Option<&DeckModel> {
        self.primary_player().and_then(PlayerModel::primary_deck)
    }
}

/// HUMAN: Runtime deck state for the active game instance.
/// AI: Cards in this model are removed when drawn into the active hand.
#[derive(Clone, Debug, Default, PartialEq, Resource)]
pub struct GameDeckModel {
    pub cards: Vec<String>,
}

impl GameDeckModel {
    pub fn draw_to_hand(
        &mut self,
        hand_size: usize,
        hand_model: &mut GameHandModel,
    ) -> Vec<String> {
        let draw_count = hand_size.min(self.cards.len());
        if draw_count == 0 {
            return Vec::new();
        }

        let drawn: Vec<String> = self.cards.drain(0..draw_count).collect();
        hand_model.cards.extend(drawn.iter().cloned());
        drawn
    }
}

/// HUMAN: Runtime active hand state for gameplay rendering and gesture targeting.
/// AI: This mirrors cards drawn from GameDeckModel and is the source of hand entities.
#[derive(Clone, Debug, Default, PartialEq, Resource)]
pub struct GameHandModel {
    pub cards: Vec<String>,
}

impl GameHandModel {
    pub fn new(cards: Vec<String>) -> Self {
        Self { cards }
    }

    pub fn reset_from_deck(&mut self, deck: &DeckModel) {
        self.cards = deck.cards.clone();
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

pub fn random_shuffled_default_deck_cards() -> Vec<String> {
    let mut cards: Vec<String> = vec![
        KAGE_REN_CARD_MODEL_ID.to_string(),
        LORD_DAICHI_CARD_MODEL_ID.to_string(),
        SISTER_HOTARU_CARD_MODEL_ID.to_string(),
        YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
    ]
    .into_iter()
    .cycle()
    .take(STARTING_DECK_CARD_COUNT)
    .collect();

    fastrand::shuffle(&mut cards);
    cards
}

pub fn ensure_player_deck_collection_model(
    mut model: PlayerDeckCollectionModel,
) -> PlayerDeckCollectionModel {
    if model.players.is_empty() {
        model.players.push(PlayerModel::default());
    } else {
        for player in &mut model.players {
            if player.name.is_empty() {
                player.name = DEFAULT_PLAYER_NAME.to_string();
            }
            if player.decks.is_empty() {
                player.decks.push(DeckModel::default());
            }
            if let Some(first_deck) = player.decks.first_mut() {
                if first_deck.cards.is_empty() {
                    *first_deck = DeckModel::default();
                }
            }
        }
    }

    model
}

/// HUMAN: World data model used by GameView background rendering.
/// AI: Keep the model separate from spawned world background entities.
#[derive(Clone, Debug, PartialEq)]
pub struct WorldModel {
    pub id: &'static str,
    pub display_name: &'static str,
    pub background_texture: &'static str,
}

impl WorldModel {
    pub const fn bamboo_forest() -> Self {
        Self {
            id: BAMBOO_FOREST_WORLD_ID,
            display_name: "Bamboo Forest",
            background_texture: "themes/theme_japan/worlds/world_bamboo_forest/world_background.png",
        }
    }

    pub const fn coastal_harbor() -> Self {
        Self {
            id: COASTAL_HARBOR_WORLD_ID,
            display_name: "Coastal Harbor",
            background_texture: "themes/theme_japan/worlds/world_coastal_harbor/world_background.png",
        }
    }
}

/// HUMAN: Registry of available world data models.
/// AI: Preserve current ordering because keyboard toggles and tests depend on it.
#[derive(Clone, Debug, Resource)]
pub struct WorldModelRegistry {
    themes: Vec<WorldModel>,
}

impl Default for WorldModelRegistry {
    fn default() -> Self {
        Self {
            themes: vec![WorldModel::bamboo_forest(), WorldModel::coastal_harbor()],
        }
    }
}

impl WorldModelRegistry {
    pub fn active_world_model(&self, active_world_model: &ActiveWorldModel) -> &WorldModel {
        self.themes
            .get(active_world_model.index)
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

/// HUMAN: Active world model selection.
/// AI: Indexes WorldModelRegistry and should not store texture handles.
#[derive(Debug, Resource)]
pub struct ActiveWorldModel {
    pub index: usize,
}

impl Default for ActiveWorldModel {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl ActiveWorldModel {
    pub fn toggle(&mut self, registry: &WorldModelRegistry) {
        self.index = registry.next_index(self.index);
    }
}

/// HUMAN: Location data model used by GameView location cards.
/// AI: Keep location texture paths model-owned and theme-rooted.
#[derive(Clone, Debug, PartialEq)]
pub struct LocationModel {
    pub id: &'static str,
    pub display_name: &'static str,
    pub texture: &'static str,
}

impl LocationModel {
    pub const fn new(id: &'static str, display_name: &'static str, texture: &'static str) -> Self {
        Self {
            id,
            display_name,
            texture,
        }
    }
}

/// HUMAN: Registry of available location data models.
/// AI: ActiveLocations chooses indices from this registry for the current view.
#[derive(Clone, Debug, Resource)]
pub struct LocationModelRegistry {
    locations: Vec<LocationModel>,
}

impl Default for LocationModelRegistry {
    fn default() -> Self {
        Self {
            locations: vec![
                LocationModel::new(
                    "fortress_gate",
                    "Fortress Gate",
                    "themes/theme_japan/locations/location_fortress_gate/location.png",
                ),
                LocationModel::new(
                    "bamboo_crossing",
                    "Bamboo Crossing",
                    "themes/theme_japan/locations/location_bamboo_crossing/location.png",
                ),
                LocationModel::new(
                    "shrine_ruins",
                    "Shrine Ruins",
                    "themes/theme_japan/locations/location_shrine_ruins/location.png",
                ),
                LocationModel::new(
                    "battlefield",
                    "Battlefield",
                    "themes/theme_japan/locations/location_battlefield/location.png",
                ),
                LocationModel::new(
                    "spirit_well",
                    "Spirit Well",
                    "themes/theme_japan/locations/location_spirit_well/location.png",
                ),
                LocationModel::new(
                    "market_square",
                    "Market Square",
                    "themes/theme_japan/locations/location_market_square/location.png",
                ),
            ],
        }
    }
}

impl LocationModelRegistry {
    pub fn selected_locations(&self, active_locations: &ActiveLocations) -> Vec<&LocationModel> {
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
        registry: &LocationModelRegistry,
        active_world_model: &ActiveWorldModel,
    ) {
        let count = registry.len().max(1);
        self.generation = self.generation.wrapping_add(1);
        let start = (active_world_model.index + self.generation) % count;
        for (offset, index) in self.indices.iter_mut().enumerate() {
            *index = (start + offset) % count;
        }
    }
}

/// HUMAN: Active card model selection.
/// AI: Indexes CardModelRegistry and drives CardViewBundle creation.
#[derive(Debug, Resource)]
pub struct ActiveCardModel {
    pub index: usize,
}

impl Default for ActiveCardModel {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl ActiveCardModel {
    pub fn toggle(&mut self, registry: &CardModelRegistry) {
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
    pub show_safe_area: bool,
    pub background_layer_scale: f32,
    pub frame_layer_scale: f32,
    pub foreground_layer_scale: f32,
    pub title_layer_scale: f32,
}

impl Default for CardUiState {
    fn default() -> Self {
        Self {
            depth_factor: CARD_DEPTH_FACTOR_DEFAULT,
            show_safe_area: true,
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
    #[serde(default = "default_show_safe_area")]
    pub show_safe_area: bool,
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
            show_safe_area: true,
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

const fn default_show_safe_area() -> bool {
    true
}

impl CardSettingsStore {
    pub fn from_state(state: &CardUiState) -> Self {
        Self {
            depth_factor: state.depth_factor,
            show_safe_area: state.show_safe_area,
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
        state.show_safe_area = self.show_safe_area;
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
    pub is_debug_drawing_visible: bool,
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
    #[serde(default)]
    pub is_debug_drawing_visible: bool,
}

impl DebugHudInputStore {
    pub fn from_state(state: &DebugHudState) -> Self {
        Self {
            is_fps_visible: state.is_fps_visible,
            is_fullscreen: state.is_fullscreen,
            is_inspector_visible: state.is_inspector_visible,
            is_hot_reload_autorestart_enabled: state.is_hot_reload_autorestart_enabled,
            is_debug_drawing_visible: state.is_debug_drawing_visible,
        }
    }

    pub fn apply_to_state(&self, state: &mut DebugHudState) {
        state.is_fps_visible = self.is_fps_visible;
        state.is_fullscreen = self.is_fullscreen;
        state.is_inspector_visible = self.is_inspector_visible;
        state.is_hot_reload_autorestart_enabled = self.is_hot_reload_autorestart_enabled;
        state.is_debug_drawing_visible = self.is_debug_drawing_visible;
    }
}

#[derive(Resource, Debug, Default)]
pub struct WindowPlacementState {
    pub current: Option<WindowPlacement>,
    pub restored: bool,
}

/// HUMAN: Tracks native fullscreen transition frames that need the default camera viewport.
/// AI: Keeps Bevy depth and color attachments aligned while winit surface sizes settle.
#[derive(Resource, Debug, Default)]
pub struct FullscreenViewportTransitionState {
    pub frames_remaining: u8,
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

pub fn player_deck_collection_path() -> PathBuf {
    workspace_root_path()
        .join("data")
        .join("local_storage")
        .join("player-deck-collection.json")
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

pub fn create_player_deck_collection_store()
-> Result<Persistent<PlayerDeckCollectionModel>, PersistenceError> {
    Persistent::<PlayerDeckCollectionModel>::builder()
        .name("player deck collection")
        .format(StorageFormat::JsonPretty)
        .path(player_deck_collection_path())
        .default(PlayerDeckCollectionModel {
            players: vec![PlayerModel::default()],
        })
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
        assert!(!store.is_debug_drawing_visible);
    }

    #[test]
    fn debug_hud_input_store_persists_debug_drawing_toggle() {
        let state = DebugHudState {
            is_debug_drawing_visible: true,
            ..Default::default()
        };

        let store = DebugHudInputStore::from_state(&state);
        let mut restored_state = DebugHudState::default();
        store.apply_to_state(&mut restored_state);

        assert!(store.is_debug_drawing_visible);
        assert!(restored_state.is_debug_drawing_visible);
    }

    #[test]
    fn card_defaults_match_japan_realism_card_ratio() {
        let defaults = CardInspectionDefaults::default();
        let expected_ratio = 1.0 / CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT;
        let tolerance = expected_ratio * 0.02;

        assert!((defaults.height_width_ratio() - expected_ratio).abs() <= tolerance);
        assert_eq!(
            defaults.width / defaults.height,
            CARD_RENDER_ASPECT_RATIO_WIDTH_OVER_HEIGHT
        );
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
    fn card_model_registry_has_japan_realism_characters() {
        let registry = CardModelRegistry::default();
        let active_card_model = ActiveCardModel::default();

        assert_eq!(registry.slot_count(), CARD_MODEL_SLOT_COUNT);
        assert_eq!(registry.available_count(), 4);
        assert_eq!(
            registry
                .active_card_model(&active_card_model)
                .map(|card_model| card_model.id),
            Some(KAGE_REN_CARD_MODEL_ID)
        );
    }

    #[test]
    fn card_model_registry_exposes_cost_and_base_power_for_every_card() {
        let registry = CardModelRegistry::default();
        let card_ids: Vec<&str> = registry
            .card_models()
            .map(|card_model| card_model.id)
            .collect();

        assert_eq!(
            card_ids,
            vec![
                KAGE_REN_CARD_MODEL_ID,
                LORD_DAICHI_CARD_MODEL_ID,
                SISTER_HOTARU_CARD_MODEL_ID,
                YOKAI_PLACEHOLDER_CARD_MODEL_ID,
            ]
        );
        assert!(registry.card_models().all(|card_model| {
            card_model.cost.is_in_display_contract()
                && card_model.base_power.is_in_display_contract()
        }));
    }

    #[test]
    fn card_model_creation_assigns_in_range_cost_and_base_power() {
        let generated_values: Vec<(i32, i32)> = (0..64)
            .map(|_| {
                let card_model = CardModel::kage_ren();
                (card_model.cost.value, card_model.base_power.value)
            })
            .collect();

        assert!(generated_values.iter().all(|(cost, base_power)| {
            (POINT_VIEW_DISPLAY_MIN..=POINT_VIEW_DISPLAY_MAX).contains(cost)
                && (POINT_VIEW_DISPLAY_MIN..=POINT_VIEW_DISPLAY_MAX).contains(base_power)
        }));
    }

    #[test]
    fn card_model_textures_match_asset_directory_casing() {
        let registry = CardModelRegistry::default();
        let asset_root = game_asset_root_path();

        for card_model in registry.slots.iter().flatten() {
            for texture_path in [
                card_model.background_texture,
                card_model.frame_texture,
                card_model.foreground_texture,
                card_model.title_texture,
            ] {
                assert!(
                    asset_root.join(texture_path).is_file(),
                    "missing card model texture at {}",
                    texture_path
                );
            }
        }
    }

    #[test]
    fn theme_asset_root_contains_current_japan_cards_locations_and_worlds() {
        let asset_root = game_asset_root_path();
        for relative_path in [
            "themes/theme_japan/cards/card_kage_ren/background.png",
            "themes/theme_japan/cards/card_lord_daichi/background.png",
            "themes/theme_japan/cards/card_sister_hotaru/background.png",
            "themes/theme_japan/cards/card_yokai_placeholder/background.png",
            "themes/theme_japan/cards/card_back.png",
            "themes/theme_japan/cards/safe_area.png",
            "themes/theme_japan/locations/location_fortress_gate/location.png",
            "themes/theme_japan/locations/location_bamboo_crossing/location.png",
            "themes/theme_japan/locations/location_shrine_ruins/location.png",
            "themes/theme_japan/locations/location_battlefield/location.png",
            "themes/theme_japan/locations/location_spirit_well/location.png",
            "themes/theme_japan/locations/location_market_square/location.png",
            "themes/theme_japan/worlds/world_bamboo_forest/world_background.png",
            "themes/theme_japan/worlds/world_coastal_harbor/world_background.png",
        ] {
            assert!(
                asset_root.join(relative_path).is_file(),
                "missing theme asset at {relative_path}"
            );
        }
    }

    #[test]
    fn runtime_model_paths_start_with_theme_root() {
        let card_registry = CardModelRegistry::default();
        for card_model in card_registry.card_models() {
            for texture_path in [
                card_model.background_texture,
                card_model.frame_texture,
                card_model.foreground_texture,
                card_model.title_texture,
            ] {
                assert!(texture_path.starts_with("themes/theme_japan/cards/"));
            }
        }

        let world_registry = WorldModelRegistry::default();
        for world_model in &world_registry.themes {
            assert!(
                world_model
                    .background_texture
                    .starts_with("themes/theme_japan/worlds/")
            );
        }

        let location_registry = LocationModelRegistry::default();
        for location_model in &location_registry.locations {
            assert!(
                location_model
                    .texture
                    .starts_with("themes/theme_japan/locations/")
            );
        }
    }

    #[test]
    fn theme_owned_folder_names_use_category_prefixes() {
        let card_registry = CardModelRegistry::default();
        for card_model in card_registry.card_models() {
            assert!(theme_owned_name(card_model.background_texture).starts_with("card_"));
        }

        let world_registry = WorldModelRegistry::default();
        for world_model in &world_registry.themes {
            assert!(theme_owned_name(world_model.background_texture).starts_with("world_"));
        }

        let location_registry = LocationModelRegistry::default();
        for location_model in &location_registry.locations {
            assert!(theme_owned_name(location_model.texture).starts_with("location_"));
        }
    }

    #[test]
    fn theme_owned_paths_do_not_repeat_japan_after_theme_root() {
        for path in theme_owned_runtime_paths() {
            let after_root = path
                .strip_prefix("themes/theme_japan/")
                .expect("theme-owned path should start with theme root");
            assert!(
                !after_root.contains("japan"),
                "theme-owned path repeats theme name: {path}"
            );
        }
    }

    #[test]
    fn card_model_registry_paths_cover_card_view_bundle_presentation_assets() {
        let registry = CardModelRegistry::default();
        for card_model in registry.card_models() {
            assert!(card_model.background_texture.ends_with("/background.png"));
            assert!(card_model.frame_texture.ends_with("/frame.png"));
            assert!(
                card_model
                    .foreground_texture
                    .ends_with("/foreground_character.png")
            );
            assert!(card_model.title_texture.ends_with("/title.png"));
            assert!(CARD_BACK_TEXTURE_PATH.ends_with("/card_back.png"));
            assert!(CARD_SAFE_AREA_TEXTURE_PATH.ends_with("/safe_area.png"));
        }
    }

    #[test]
    fn card_model_toggle_cycles_between_four_japan_realism_cards() {
        let registry = CardModelRegistry::default();
        let mut active_card_model = ActiveCardModel::default();

        active_card_model.toggle(&registry);
        assert_eq!(active_card_model.index, 1);
        assert_eq!(
            registry
                .active_card_model(&active_card_model)
                .map(|card_model| card_model.display_name),
            Some(LORD_DAICHI_CARD_MODEL_NAME)
        );

        active_card_model.toggle(&registry);
        assert_eq!(active_card_model.index, 2);
        assert_eq!(
            registry
                .active_card_model(&active_card_model)
                .map(|card_model| card_model.display_name),
            Some(SISTER_HOTARU_CARD_MODEL_NAME)
        );

        active_card_model.toggle(&registry);
        assert_eq!(active_card_model.index, 3);
        assert_eq!(
            registry
                .active_card_model(&active_card_model)
                .map(|card_model| card_model.display_name),
            Some(YOKAI_PLACEHOLDER_CARD_MODEL_NAME)
        );

        active_card_model.toggle(&registry);

        assert_eq!(active_card_model.index, 0);
        assert_eq!(
            registry
                .active_card_model(&active_card_model)
                .map(|card_model| card_model.display_name),
            Some(KAGE_REN_CARD_MODEL_NAME)
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
    fn card_back_texture_uses_theme_card_asset_path() {
        assert_eq!(
            CARD_BACK_TEXTURE_PATH,
            "themes/theme_japan/cards/card_back.png"
        );
        assert!(!CARD_BACK_TEXTURE_PATH.contains("card_model_"));
    }

    #[test]
    fn world_model_registry_cycles_between_bamboo_forest_and_coastal_harbor() {
        let registry = WorldModelRegistry::default();
        let mut active_world_model = ActiveWorldModel::default();

        assert_eq!(registry.len(), WORLD_MODEL_COUNT);
        assert_eq!(
            registry.active_world_model(&active_world_model).id,
            BAMBOO_FOREST_WORLD_ID
        );

        active_world_model.toggle(&registry);
        assert_eq!(
            registry.active_world_model(&active_world_model).id,
            COASTAL_HARBOR_WORLD_ID
        );

        active_world_model.toggle(&registry);
        assert_eq!(
            registry.active_world_model(&active_world_model).id,
            BAMBOO_FOREST_WORLD_ID
        );
    }

    #[test]
    fn active_locations_selects_three_locations_from_six() {
        let registry = LocationModelRegistry::default();
        let mut active_locations = ActiveLocations::default();
        let active_world_model = ActiveWorldModel::default();

        assert_eq!(registry.len(), LOCATION_MODEL_COUNT);
        assert_eq!(
            registry.selected_locations(&active_locations).len(),
            ACTIVE_LOCATION_COUNT
        );

        active_locations.reroll(&registry, &active_world_model);

        assert_eq!(
            registry.selected_locations(&active_locations).len(),
            ACTIVE_LOCATION_COUNT
        );
        assert!(
            active_locations
                .indices
                .iter()
                .all(|index| *index < LOCATION_MODEL_COUNT)
        );
    }

    #[test]
    fn card_ui_depth_factor_defaults_to_current_parallax_strength() {
        let state = CardUiState::default();

        assert_eq!(state.depth_factor, CARD_DEPTH_FACTOR_DEFAULT);
        assert_eq!(state.depth_multiplier(), 1.0);
        assert!(state.show_safe_area);
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
            show_safe_area: false,
            background_layer_scale: 0.5,
            frame_layer_scale: 0.75,
            foreground_layer_scale: 1.25,
            title_layer_scale: 1.5,
        };
        let mut state = CardUiState::default();

        settings.apply_to_state(&mut state);

        assert_eq!(state.depth_factor, 7.5);
        assert!(!state.show_safe_area);
        assert_eq!(state.background_layer_scale, 0.5);
        assert_eq!(state.frame_layer_scale, 0.75);
        assert_eq!(state.foreground_layer_scale, 1.25);
        assert_eq!(state.title_layer_scale, 1.5);
    }

    #[test]
    fn card_settings_clamps_depth_factor_to_supported_range() {
        let settings = CardSettingsStore {
            depth_factor: CARD_DEPTH_FACTOR_MAX + 1.0,
            show_safe_area: false,
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

    fn game_asset_root_path() -> PathBuf {
        workspace_root_path()
            .join("bevy")
            .join("crates")
            .join("game")
            .join("assets")
    }

    fn theme_owned_name(path: &str) -> &str {
        path.split('/')
            .nth(3)
            .expect("theme-owned path should include category-owned folder")
    }

    fn theme_owned_runtime_paths() -> Vec<&'static str> {
        let card_registry = CardModelRegistry::default();
        let world_registry = WorldModelRegistry::default();
        let location_registry = LocationModelRegistry::default();
        let mut paths = vec![CARD_BACK_TEXTURE_PATH, CARD_SAFE_AREA_TEXTURE_PATH];
        for card_model in card_registry.card_models() {
            paths.extend([
                card_model.background_texture,
                card_model.frame_texture,
                card_model.foreground_texture,
                card_model.title_texture,
            ]);
        }
        for world_model in &world_registry.themes {
            paths.push(world_model.background_texture);
        }
        for location_model in &location_registry.locations {
            paths.push(location_model.texture);
        }
        paths
    }
}
