use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[cfg(feature = "desktop-hot-reload")]
use std::sync::atomic::{AtomicU64, Ordering};

pub mod app_camera_model;
pub mod audio_manager_model;
pub mod card_gesture_model;
pub mod card_instance_state_model;
pub mod card_slot_model;
pub mod cpu_brain_model;
pub mod debug_drawing_model;
pub mod deck_screen_model;
pub mod font_model;
pub mod game_location_model;
pub mod game_round_model;
pub mod hot_reload_screen_model;
pub mod match_model;
pub mod matchmaking_model;
pub mod meta_game_settings_model;
pub mod pending_round_deal_resource;
pub mod point_model;
pub mod screen_transition_resource;
pub mod selected_card_modal_model;
pub mod top_navigation_model;

pub use app_camera_model::*;
pub use audio_manager_model::*;
pub use card_gesture_model::*;
pub use card_instance_state_model::*;
pub use card_slot_model::*;
pub use cpu_brain_model::*;
pub use debug_drawing_model::*;
pub use deck_screen_model::*;
pub use font_model::*;
pub use game_location_model::*;
pub use game_round_model::*;
pub use hot_reload_screen_model::*;
pub use match_model::*;
pub use matchmaking_model::*;
pub use meta_game_settings_model::*;
pub use pending_round_deal_resource::*;
pub use point_model::*;
pub use screen_transition_resource::*;
pub use selected_card_modal_model::*;
pub use top_navigation_model::*;

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
pub const CARD_MODEL_SLOT_COUNT: usize = 5;
pub const GAME_MASTER_DECK_CARD_COUNT: usize = 15;
pub const STARTING_DECK_CARD_COUNT: usize = 12;
pub const STARTING_HAND_CARD_COUNT: usize = 5;
pub const STARTING_HAND_REPEATS_PER_CARD: usize = 3;
pub const DEFAULT_PLAYER_NAME: &str = "Player 01";
pub const DEFAULT_DECK_NAME: &str = "Deck01";
pub const WORLD_MODEL_COUNT: usize = 3;
pub const LOCATION_MODEL_COUNT: usize = 6;
pub const ACTIVE_LOCATION_COUNT: usize = 3;
pub const CARD_DEPTH_FACTOR_DEFAULT: f32 = 10.0;
pub const CARD_DEPTH_FACTOR_MIN: f32 = 0.0;
pub const CARD_DEPTH_FACTOR_MAX: f32 = 20.0;
pub const CARD_LAYER_SCALE_DEFAULT: f32 = 1.0;
pub const CARD_LAYER_SCALE_MIN: f32 = 0.0;
pub const CARD_LAYER_SCALE_MAX: f32 = 2.0;
pub const CARD_FLIP_DURATION_SECONDS: f32 = 0.5;
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
pub const GORO_TAKESHI_CARD_MODEL_ID: &str = "goro_takeshi";
pub const GORO_TAKESHI_CARD_MODEL_NAME: &str = "GORO TAKESHI";
pub const BAMBOO_FOREST_WORLD_ID: &str = "bamboo_forest";
pub const COASTAL_HARBOR_WORLD_ID: &str = "coastal_harbor";
pub const SUJI_SWAMP_WORLD_ID: &str = "suji_swamp";

/// HUMAN: Frame counter shared by runtime systems.
/// AI: Keep as the lightweight app tick resource; do not mix with gameplay round state.
#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

/// HUMAN: Selects the single sub-screen view loaded on top of the persistent AppScene.
/// AI: Variants are views, not scenes; AppScene remains always present.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Resource)]
pub enum ActiveView {
    MainMenuScene,
    LightningScene,
    MatchmakingScene,
    #[default]
    GameScene,
    DeckScene,
    SettingsScene,
    DebugScene,
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
            cost: CostPointModel::new(1),
            base_power: PowerPointModel::new(1),
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
            cost: CostPointModel::new(2),
            base_power: PowerPointModel::new(3),
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
            cost: CostPointModel::new(1),
            base_power: PowerPointModel::new(2),
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
            cost: CostPointModel::new(3),
            base_power: PowerPointModel::new(4),
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

    pub fn goro_takeshi() -> Self {
        Self {
            id: GORO_TAKESHI_CARD_MODEL_ID,
            display_name: GORO_TAKESHI_CARD_MODEL_NAME,
            cost: CostPointModel::new(5),
            base_power: PowerPointModel::new(5),
            background_texture: "themes/theme_japan/cards/card_goro_takeshi/background.png",
            frame_texture: "themes/theme_japan/cards/card_goro_takeshi/frame.png",
            foreground_texture: "themes/theme_japan/cards/card_goro_takeshi/foreground_character.png",
            title_texture: "themes/theme_japan/cards/card_goro_takeshi/title.png",
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
                Some(CardModel::goro_takeshi()),
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
    pub fn reset_randomized(&mut self) {
        self.cards = random_shuffled_default_deck_cards();
    }

    pub fn draw_to_hand(
        &mut self,
        hand_size: usize,
        hand_model: &mut GameHandModel,
    ) -> Vec<String> {
        self.deal_to_hand(hand_size, hand_model)
    }

    pub fn deal_to_hand(
        &mut self,
        requested_count: usize,
        hand_model: &mut GameHandModel,
    ) -> Vec<String> {
        let deal_count = requested_count.min(self.cards.len());
        if deal_count == 0 {
            return Vec::new();
        }

        let dealt: Vec<String> = self.cards.drain(0..deal_count).collect();
        hand_model.cards.extend(dealt.iter().cloned());
        dealt
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

    pub fn remove_index(&mut self, hand_index: usize) -> Option<String> {
        (hand_index < self.cards.len()).then(|| self.cards.remove(hand_index))
    }

    pub fn insert_at(&mut self, hand_index: usize, card_id: String) {
        let index = hand_index.min(self.cards.len());
        self.cards.insert(index, card_id);
    }

    pub fn push_card(&mut self, card_id: String) {
        self.cards.push(card_id);
    }
}

/// HUMAN: Hidden card library that future player decks choose from.
/// AI: This is card-entry data, so repeated IDs represent separate deck-building copies.
pub fn game_master_deck_cards() -> Vec<String> {
    vec![
        KAGE_REN_CARD_MODEL_ID.to_string(),
        LORD_DAICHI_CARD_MODEL_ID.to_string(),
        SISTER_HOTARU_CARD_MODEL_ID.to_string(),
        YOKAI_PLACEHOLDER_CARD_MODEL_ID.to_string(),
        GORO_TAKESHI_CARD_MODEL_ID.to_string(),
    ]
    .into_iter()
    .cycle()
    .take(GAME_MASTER_DECK_CARD_COUNT)
    .collect()
}

/// HUMAN: Default Deck 01 card-entry list used by both players during gameplay.
/// AI: Keep this exactly STARTING_DECK_CARD_COUNT entries and derive it from the hidden master deck.
pub fn default_deck_01_cards() -> Vec<String> {
    game_master_deck_cards()
        .into_iter()
        .take(STARTING_DECK_CARD_COUNT)
        .collect()
}

pub fn random_shuffled_default_deck_cards() -> Vec<String> {
    let mut cards = default_deck_01_cards();

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

/// HUMAN: World data model used by GameScene background rendering.
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

    pub const fn suji_swamp() -> Self {
        Self {
            id: SUJI_SWAMP_WORLD_ID,
            display_name: "Suji Swamp",
            background_texture: "themes/theme_japan/worlds/world_suji_swamp/world_background.png",
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
            themes: vec![
                WorldModel::bamboo_forest(),
                WorldModel::coastal_harbor(),
                WorldModel::suji_swamp(),
            ],
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

    pub fn randomize(&mut self, registry: &WorldModelRegistry) {
        self.randomize_with_len(registry.len());
    }

    pub fn randomize_with_len(&mut self, len: usize) {
        if len <= 1 {
            self.index = 0;
            return;
        }

        let mut next_index = fastrand::usize(..len);
        if next_index == self.index {
            next_index = (next_index + 1) % len;
        }
        self.index = next_index;
    }
}

/// HUMAN: Location data model used by GameScene location cards.
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

/// HUMAN: Three location registry indices currently active in the match.
/// AI: Keep this aligned with GameLocationModel definitions whenever a match resets.
#[derive(Clone, Debug, Resource)]
pub struct ActiveLocations {
    pub indices: [usize; ACTIVE_LOCATION_COUNT],
}

impl Default for ActiveLocations {
    fn default() -> Self {
        Self { indices: [0, 1, 2] }
    }
}

impl ActiveLocations {
    pub fn reroll(
        &mut self,
        registry: &LocationModelRegistry,
        _active_world_model: &ActiveWorldModel,
    ) {
        let count = registry.len().max(1);
        let mut available_indices: Vec<usize> = (0..count).collect();
        fastrand::shuffle(&mut available_indices);
        for (slot_index, index) in self.indices.iter_mut().enumerate() {
            *index = available_indices
                .get(slot_index)
                .copied()
                .unwrap_or(slot_index % count);
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CardFace {
    #[default]
    Front,
    Back,
}

#[derive(Debug, Resource)]
pub struct CardFlipState {
    pub start_y_rotation: f32,
    pub current_y_rotation: f32,
    pub target_y_rotation: f32,
    pub elapsed_seconds: f32,
    pub visible_face: CardFace,
}

impl Default for CardFlipState {
    fn default() -> Self {
        Self {
            start_y_rotation: 0.0,
            current_y_rotation: 0.0,
            target_y_rotation: 0.0,
            elapsed_seconds: 0.0,
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
            self.start_y_rotation = self.current_y_rotation;
            self.elapsed_seconds = 0.0;
            if self.target_y_rotation > self.current_y_rotation {
                self.target_y_rotation -= std::f32::consts::PI;
            } else {
                self.target_y_rotation += std::f32::consts::PI;
            }
            return;
        }

        self.start_y_rotation = self.current_y_rotation;
        self.elapsed_seconds = 0.0;
        match Self::face_for_angle(self.target_y_rotation) {
            CardFace::Front => self.target_y_rotation += std::f32::consts::PI,
            CardFace::Back => self.target_y_rotation -= std::f32::consts::PI,
        }
    }

    pub fn advance(&mut self, delta_seconds: f32) {
        let remaining = self.target_y_rotation - self.current_y_rotation;
        if remaining.abs() <= f32::EPSILON {
            self.current_y_rotation = self.target_y_rotation;
            self.start_y_rotation = self.target_y_rotation;
            self.elapsed_seconds = 0.0;
            self.visible_face = Self::face_for_angle(self.current_y_rotation);
            return;
        }

        self.elapsed_seconds += delta_seconds.max(0.0);
        let progress = (self.elapsed_seconds / CARD_FLIP_DURATION_SECONDS).clamp(0.0, 1.0);
        if progress >= 1.0 {
            self.current_y_rotation = self.target_y_rotation;
        } else {
            let eased_progress = ease_out_cubic(progress);
            self.current_y_rotation = self
                .start_y_rotation
                .lerp(self.target_y_rotation, eased_progress);
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

fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress.clamp(0.0, 1.0)).powi(3)
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

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DebugDrawMode {
    #[default]
    Off,
    On,
    OnSolo,
}

impl DebugDrawMode {
    pub const fn is_visible(self) -> bool {
        !matches!(self, Self::Off)
    }

    pub const fn is_solo(self) -> bool {
        matches!(self, Self::OnSolo)
    }

    pub const fn toggle_standard(self) -> Self {
        if matches!(self, Self::On) {
            Self::Off
        } else {
            Self::On
        }
    }

    pub const fn toggle_solo(self) -> Self {
        if matches!(self, Self::OnSolo) {
            Self::Off
        } else {
            Self::OnSolo
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub is_fullscreen: bool,
    pub is_inspector_visible: bool,
    pub is_hot_reload_autorestart_enabled: bool,
    pub debug_draw_mode: DebugDrawMode,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}

impl DebugHudState {
    pub const fn is_debug_drawing_visible(&self) -> bool {
        self.debug_draw_mode.is_visible()
    }

    pub const fn is_debug_drawing_solo(&self) -> bool {
        self.debug_draw_mode.is_solo()
    }
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
    pub debug_draw_mode: DebugDrawMode,
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
            debug_draw_mode: state.debug_draw_mode,
            is_debug_drawing_visible: state.is_debug_drawing_visible(),
        }
    }

    pub fn apply_to_state(&self, state: &mut DebugHudState) {
        state.is_fps_visible = self.is_fps_visible;
        state.is_fullscreen = self.is_fullscreen;
        state.is_inspector_visible = self.is_inspector_visible;
        state.is_hot_reload_autorestart_enabled = self.is_hot_reload_autorestart_enabled;
        state.debug_draw_mode =
            if self.debug_draw_mode == DebugDrawMode::Off && self.is_debug_drawing_visible {
                DebugDrawMode::On
            } else {
                self.debug_draw_mode
            };
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
    workspace_root_path_for_game()
}

pub fn workspace_root_path_for_game() -> PathBuf {
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
#[path = "../../tests/runtime/resources/resources_mod_tests.rs"]
mod resources_mod_tests;
