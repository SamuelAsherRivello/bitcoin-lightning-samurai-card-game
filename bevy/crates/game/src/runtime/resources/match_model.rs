use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};

use super::CpuBrainMoveModel;
use super::{
    ACTIVE_LOCATION_COUNT, ActiveLocations, ActiveWorldModel, CARD_SLOT_LOCATION_COUNT,
    CardInstanceId, CardModelRegistry, CardOwnerModel, CardSlotBoardModel, CardSlotSide,
    CardSlotState, DeckModel, GameDeckModel, GameHandModel, GameLocationModel, GameRoundModel,
    LocationModelRegistry, PlayerSide, PowerPointModel, STARTING_DECK_CARD_COUNT,
    STARTING_HAND_CARD_COUNT, random_shuffled_default_deck_cards,
};

/// HUMAN: User-facing match mode selection for two-player gameplay.
/// AI: Keep mode labels stable and hide CPU Brain implementation details.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Resource, Serialize)]
pub enum MatchModeModel {
    #[default]
    HumanVersusCpu,
    CpuVersusCpu,
}

impl MatchModeModel {
    pub const fn label(self) -> &'static str {
        match self {
            Self::HumanVersusCpu => "Human versus CPU",
            Self::CpuVersusCpu => "CPU versus CPU",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::HumanVersusCpu => Self::CpuVersusCpu,
            Self::CpuVersusCpu => Self::HumanVersusCpu,
        }
    }
}

/// HUMAN: One side of the two-player board.
/// AI: Maps owner semantics onto existing local/opponent slot sides.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MatchPlayerSide {
    Near,
    Far,
}

impl MatchPlayerSide {
    pub const fn slot_side(self) -> CardSlotSide {
        match self {
            Self::Near => CardSlotSide::LocalPlayer,
            Self::Far => CardSlotSide::Opponent,
        }
    }

    pub const fn player_number(self) -> u8 {
        match self {
            Self::Near => 1,
            Self::Far => 2,
        }
    }

    pub const fn point_side(self) -> PlayerSide {
        match self {
            Self::Near => PlayerSide::Local,
            Self::Far => PlayerSide::Opponent,
        }
    }
}

/// HUMAN: Human controller marker for local input owned choices.
/// AI: Keep controller identity separate from player state and CPU Brain state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerController;

/// HUMAN: CPU controller marker for authored game-code choices.
/// AI: This dispatches CPU Brain choices to shared game logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuController {
    pub brain_level: CpuBrainLevel,
}

/// HUMAN: Controller assigned to a player in the active match.
/// AI: UI labels should expose only Human/CPU, never Brain internals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerControllerModel {
    Human(PlayerController),
    Cpu(CpuController),
}

impl PlayerControllerModel {
    pub const fn human() -> Self {
        Self::Human(PlayerController)
    }

    pub const fn cpu() -> Self {
        Self::Cpu(CpuController {
            brain_level: CpuBrainLevel::Level1,
        })
    }

    pub const fn is_cpu(self) -> bool {
        matches!(self, Self::Cpu(_))
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Human(_) => "Human",
            Self::Cpu(_) => "CPU",
        }
    }
}

/// HUMAN: CPU Brain difficulty level for authored CPU play.
/// AI: Only Level1 exists; use the enum to leave future levels explicit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CpuBrainLevel {
    Level1,
}

/// HUMAN: Current hidden/revealed state for a placed card.
/// AI: Opposing controllers cannot inspect current-round hidden card identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlacementVisibility {
    CurrentRoundHidden,
    Revealing,
    Revealed,
}

/// HUMAN: Placement visibility data for one card on the board.
/// AI: Keep reveal state deterministic and independent of render face choice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementVisibilityModel {
    pub owner: MatchPlayerSide,
    pub location_index: usize,
    pub slot_index: usize,
    pub placement_round: u8,
    pub visibility: PlacementVisibility,
}

/// HUMAN: One occupied current-round card waiting for reveal resolution.
/// AI: Keeps reveal order explicit so empty slots never add animation delay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlacementRevealTarget {
    pub owner: MatchPlayerSide,
    pub location_index: usize,
    pub slot_index: usize,
}

/// HUMAN: Per-player transient match state for one game.
/// AI: This is reset on restart/mode change and is not persisted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchPlayerModel {
    pub side: MatchPlayerSide,
    pub controller: PlayerControllerModel,
    pub deck: Vec<String>,
    pub deck_instance_ids: Vec<u64>,
    pub hand: Vec<String>,
    pub hand_instance_ids: Vec<u64>,
    pub energy_available: i32,
    pub ready_for_next: bool,
    next_slot_card_index: usize,
}

impl MatchPlayerModel {
    pub fn new(
        side: MatchPlayerSide,
        controller: PlayerControllerModel,
        deck: Vec<String>,
    ) -> Self {
        Self {
            side,
            controller,
            deck_instance_ids: (0..deck.len())
                .map(|index| CardInstanceId::from_owner_index(CardOwnerModel::new(side), index).0)
                .collect(),
            deck,
            hand: Vec::new(),
            hand_instance_ids: Vec::new(),
            energy_available: 0,
            ready_for_next: false,
            next_slot_card_index: 0,
        }
    }

    pub fn draw(&mut self, count: usize) -> Vec<String> {
        let draw_count = count.min(self.deck.len());
        let cards: Vec<String> = self.deck.drain(0..draw_count).collect();
        let instance_ids: Vec<u64> = self.deck_instance_ids.drain(0..draw_count).collect();
        self.hand.extend(cards.iter().cloned());
        self.hand_instance_ids.extend(instance_ids);
        cards
    }

    pub fn hand_instance_id(&self, hand_index: usize) -> Option<u64> {
        self.hand_instance_ids.get(hand_index).copied()
    }

    pub fn remove_hand_card(&mut self, hand_index: usize) -> Option<(u64, String)> {
        if hand_index >= self.hand.len() || hand_index >= self.hand_instance_ids.len() {
            return None;
        }

        let instance_id = self.hand_instance_ids.remove(hand_index);
        let card_id = self.hand.remove(hand_index);
        Some((instance_id, card_id))
    }

    pub fn remove_hand_card_by_instance_id(&mut self, instance_id: u64) -> Option<(usize, String)> {
        let hand_index = self
            .hand_instance_ids
            .iter()
            .position(|candidate| *candidate == instance_id)?;
        let (_, card_id) = self.remove_hand_card(hand_index)?;
        Some((hand_index, card_id))
    }

    pub fn next_slot_card_index(&mut self) -> usize {
        let index = self.next_slot_card_index;
        self.next_slot_card_index += 1;
        index
    }
}

/// HUMAN: Match winner state after round six resolves.
/// AI: Store presentation-ready data without exposing CPU Brain labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchWinnerModel {
    pub side: MatchPlayerSide,
    pub controller: PlayerControllerModel,
}

impl MatchWinnerModel {
    pub fn status_text(self) -> String {
        format!(
            "Status: Winner is Player {} ({})",
            self.side.player_number(),
            self.controller.label()
        )
    }
}

/// HUMAN: Two-player round and winner state for the active match.
/// AI: Keep this as the source for readiness gating and final status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchRoundModel {
    pub round: u8,
    pub max_rounds: u8,
    pub winner: Option<MatchWinnerModel>,
}

/// HUMAN: Current round resolution stage after both players press Next.
/// AI: Separates hidden CPU placement motion from reveal and next-round setup.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MatchResolutionPhase {
    #[default]
    Planning,
    CpuPlacementsMoving,
    CpuPlacementsRevealing,
}

/// HUMAN: Render source for a hidden CPU card that has just left hand.
/// AI: Lets presentation tween from the real hand slot after gameplay removes the card.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuPlacementMotionSourceModel {
    pub owner: MatchPlayerSide,
    pub location_index: usize,
    pub slot_index: usize,
    pub hand_index: usize,
    pub hand_count: usize,
}

impl Default for MatchRoundModel {
    fn default() -> Self {
        Self {
            round: 1,
            max_rounds: 6,
            winner: None,
        }
    }
}

/// HUMAN: World selected for one active match.
/// AI: Store the registry index here so reset and replay can keep a stable match context.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MatchWorldModel {
    pub world_index: usize,
}

/// HUMAN: Three locations selected for one active match.
/// AI: This mirrors ActiveLocations at match start without owning location definitions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchLocationSelectionModel {
    pub indices: [usize; ACTIVE_LOCATION_COUNT],
}

impl Default for MatchLocationSelectionModel {
    fn default() -> Self {
        Self {
            indices: ActiveLocations::default().indices,
        }
    }
}

/// HUMAN: Runtime two-player match model for one complete game.
/// AI: Own match context, players, round state, hidden placements, and CPU planning queues.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct MatchModel {
    pub mode: MatchModeModel,
    pub world: MatchWorldModel,
    pub locations: MatchLocationSelectionModel,
    pub near: MatchPlayerModel,
    pub far: MatchPlayerModel,
    pub round: MatchRoundModel,
    pub placements: Vec<PlacementVisibilityModel>,
    pub pending_cpu_placements: Vec<CpuBrainMoveModel>,
    pub cpu_placement_motion_sources: Vec<CpuPlacementMotionSourceModel>,
    pub resolution_phase: MatchResolutionPhase,
    pub next_reveal_delay_seconds: f32,
}

impl Default for MatchModel {
    fn default() -> Self {
        Self::new(MatchModeModel::default(), default_master_deck())
    }
}

impl MatchModel {
    pub fn new(mode: MatchModeModel, master_deck: Vec<String>) -> Self {
        let near_controller = match mode {
            MatchModeModel::HumanVersusCpu => PlayerControllerModel::human(),
            MatchModeModel::CpuVersusCpu => PlayerControllerModel::cpu(),
        };
        let far_controller = PlayerControllerModel::cpu();
        Self {
            mode,
            world: MatchWorldModel::default(),
            locations: MatchLocationSelectionModel::default(),
            near: MatchPlayerModel::new(
                MatchPlayerSide::Near,
                near_controller,
                master_deck.clone(),
            ),
            far: MatchPlayerModel::new(MatchPlayerSide::Far, far_controller, master_deck),
            round: MatchRoundModel::default(),
            placements: Vec::new(),
            pending_cpu_placements: Vec::new(),
            cpu_placement_motion_sources: Vec::new(),
            resolution_phase: MatchResolutionPhase::Planning,
            next_reveal_delay_seconds: 0.0,
        }
    }

    pub fn reset_for_mode(&mut self, mode: MatchModeModel, master_deck: Vec<String>) {
        *self = Self::new(mode, master_deck);
    }

    pub fn set_context(
        &mut self,
        active_world_model: &ActiveWorldModel,
        active_locations: &ActiveLocations,
    ) {
        self.world = MatchWorldModel {
            world_index: active_world_model.index,
        };
        self.locations = MatchLocationSelectionModel {
            indices: active_locations.indices,
        };
    }

    pub fn player(&self, side: MatchPlayerSide) -> &MatchPlayerModel {
        match side {
            MatchPlayerSide::Near => &self.near,
            MatchPlayerSide::Far => &self.far,
        }
    }

    pub fn player_mut(&mut self, side: MatchPlayerSide) -> &mut MatchPlayerModel {
        match side {
            MatchPlayerSide::Near => &mut self.near,
            MatchPlayerSide::Far => &mut self.far,
        }
    }

    pub fn both_ready(&self) -> bool {
        self.near.ready_for_next && self.far.ready_for_next
    }

    pub fn queue_cpu_placements(&mut self, moves: Vec<CpuBrainMoveModel>) {
        self.pending_cpu_placements.extend(moves);
    }

    pub fn has_pending_cpu_placements(&self) -> bool {
        !self.pending_cpu_placements.is_empty()
    }

    pub fn is_complete(&self) -> bool {
        self.round.winner.is_some()
    }

    pub fn status_text(&self) -> String {
        self.round
            .winner
            .map(MatchWinnerModel::status_text)
            .unwrap_or_else(|| "Status: Playing".to_string())
    }

    pub fn record_placement(
        &mut self,
        owner: MatchPlayerSide,
        location_index: usize,
        slot_index: usize,
    ) {
        self.placements.push(PlacementVisibilityModel {
            owner,
            location_index,
            slot_index,
            placement_round: self.round.round,
            visibility: PlacementVisibility::CurrentRoundHidden,
        });
    }

    pub fn record_cpu_placement_motion_source(&mut self, source: CpuPlacementMotionSourceModel) {
        self.cpu_placement_motion_sources.push(source);
    }

    pub fn take_cpu_placement_motion_source(
        &mut self,
        owner: MatchPlayerSide,
        location_index: usize,
        slot_index: usize,
    ) -> Option<CpuPlacementMotionSourceModel> {
        let source_index = self
            .cpu_placement_motion_sources
            .iter()
            .position(|source| {
                source.owner == owner
                    && source.location_index == location_index
                    && source.slot_index == slot_index
            })?;
        Some(self.cpu_placement_motion_sources.remove(source_index))
    }

    pub fn reveal_current_round_placements(&mut self) {
        for placement in &mut self.placements {
            if placement.placement_round == self.round.round {
                placement.visibility = PlacementVisibility::Revealed;
            }
        }
    }

    pub fn begin_current_round_reveal(&mut self) {
        self.next_reveal_delay_seconds = 0.0;
    }

    pub fn tick_next_reveal_delay(&mut self, delta_seconds: f32) -> bool {
        if self.next_reveal_delay_seconds <= 0.0 {
            return false;
        }

        self.next_reveal_delay_seconds =
            (self.next_reveal_delay_seconds - delta_seconds.max(0.0)).max(0.0);
        self.next_reveal_delay_seconds > 0.0
    }

    pub fn current_round_reveal_targets(
        &self,
        slot_board: &CardSlotBoardModel,
    ) -> Vec<PlacementRevealTarget> {
        let mut targets = self
            .placements
            .iter()
            .filter(|placement| {
                placement.placement_round == self.round.round
                    && placement.visibility == PlacementVisibility::CurrentRoundHidden
                    && slot_board
                        .slot(
                            placement.location_index,
                            placement.owner.slot_side(),
                            placement.slot_index,
                        )
                        .is_some_and(|slot| !slot.state.is_empty())
            })
            .map(|placement| PlacementRevealTarget {
                owner: placement.owner,
                location_index: placement.location_index,
                slot_index: placement.slot_index,
            })
            .collect::<Vec<_>>();
        targets.sort_by_key(|target| {
            (
                reveal_side_order(target.owner),
                target.location_index,
                reveal_slot_order(target.owner, target.slot_index),
            )
        });
        targets
    }

    pub fn start_next_current_round_reveal(
        &mut self,
        slot_board: &CardSlotBoardModel,
    ) -> Option<PlacementRevealTarget> {
        let target = self
            .current_round_reveal_targets(slot_board)
            .into_iter()
            .next()?;
        if let Some(placement) = self.placements.iter_mut().find(|placement| {
            placement.owner == target.owner
                && placement.location_index == target.location_index
                && placement.slot_index == target.slot_index
                && placement.placement_round == self.round.round
        }) {
            placement.visibility = PlacementVisibility::Revealing;
        }
        Some(target)
    }

    pub fn complete_revealing_current_round_placements(&mut self) -> usize {
        let mut completed_count = 0;
        for placement in &mut self.placements {
            if placement.placement_round == self.round.round
                && placement.visibility == PlacementVisibility::Revealing
            {
                placement.visibility = PlacementVisibility::Revealed;
                completed_count += 1;
            }
        }
        completed_count
    }

    pub fn revealed_to_controller(
        &self,
        viewer: MatchPlayerSide,
        owner: MatchPlayerSide,
        location_index: usize,
        slot_index: usize,
    ) -> bool {
        if viewer == owner {
            return true;
        }
        self.placements
            .iter()
            .find(|placement| {
                placement.owner == owner
                    && placement.location_index == location_index
                    && placement.slot_index == slot_index
            })
            .is_some_and(|placement| placement.visibility == PlacementVisibility::Revealed)
    }

    pub fn placement_visibility(
        &self,
        owner: MatchPlayerSide,
        location_index: usize,
        slot_index: usize,
    ) -> PlacementVisibility {
        self.placements
            .iter()
            .find(|placement| {
                placement.owner == owner
                    && placement.location_index == location_index
                    && placement.slot_index == slot_index
            })
            .map(|placement| placement.visibility)
            .unwrap_or(PlacementVisibility::Revealed)
    }

    pub fn controller_for_winner_side(&self, side: MatchPlayerSide) -> PlayerControllerModel {
        self.player(side).controller
    }
}

fn reveal_side_order(owner: MatchPlayerSide) -> usize {
    match owner {
        MatchPlayerSide::Near => 0,
        MatchPlayerSide::Far => 1,
    }
}

fn reveal_slot_order(owner: MatchPlayerSide, slot_index: usize) -> usize {
    match (owner, slot_index) {
        (MatchPlayerSide::Near, 0) => 0,
        (MatchPlayerSide::Near, 1) => 1,
        (MatchPlayerSide::Near, 2) => 2,
        (MatchPlayerSide::Near, 3) => 3,
        (MatchPlayerSide::Far, 2) => 0,
        (MatchPlayerSide::Far, 3) => 1,
        (MatchPlayerSide::Far, 0) => 2,
        (MatchPlayerSide::Far, 1) => 3,
        (_, _) => usize::MAX,
    }
}

/// HUMAN: Persisted user preference for last selected match mode.
/// AI: Store only the mode, not transient match state or CPU Brain state.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct MatchModePreferenceStore {
    pub selected_mode: MatchModeModel,
}

pub fn match_mode_preference_path() -> std::path::PathBuf {
    super::workspace_root_path_for_game()
        .join("data")
        .join("local_storage")
        .join("match-mode-preference.json")
}

pub fn create_match_mode_preference_store()
-> Result<Persistent<MatchModePreferenceStore>, PersistenceError> {
    Persistent::<MatchModePreferenceStore>::builder()
        .name("match mode preference")
        .format(StorageFormat::JsonPretty)
        .path(match_mode_preference_path())
        .default(MatchModePreferenceStore::default())
        .revertible(true)
        .revert_to_default_on_deserialization_errors(true)
        .build()
}

pub fn default_master_deck() -> Vec<String> {
    random_shuffled_default_deck_cards()
        .into_iter()
        .take(STARTING_DECK_CARD_COUNT)
        .collect()
}

pub fn master_deck_from_deck_model(deck: Option<&DeckModel>) -> Vec<String> {
    let mut cards = deck
        .map(|deck| deck.cards.clone())
        .filter(|cards| !cards.is_empty())
        .unwrap_or_else(default_master_deck);
    cards.truncate(STARTING_DECK_CARD_COUNT);
    if cards.is_empty() {
        default_master_deck()
    } else {
        fastrand::shuffle(&mut cards);
        cards
    }
}

pub fn start_match_round(
    match_model: &mut MatchModel,
    game_round_model: &GameRoundModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
) {
    match_model.near.ready_for_next = false;
    match_model.far.ready_for_next = false;
    match_model.pending_cpu_placements.clear();
    match_model.cpu_placement_motion_sources.clear();
    match_model.resolution_phase = MatchResolutionPhase::Planning;
    match_model.near.energy_available = game_round_model.energy_available;
    match_model.far.energy_available = game_round_model.energy_available;
    match_model
        .near
        .draw(game_round_model.requested_cards_to_deal);
    match_model
        .far
        .draw(game_round_model.requested_cards_to_deal);

    if match_model.near.controller.is_cpu() {
        game_hand_model.cards.clear();
        game_deck_model.cards.clear();
    } else {
        game_hand_model.cards = match_model.near.hand.clone();
        game_deck_model.cards = match_model.near.deck.clone();
    }
}

pub fn reset_two_player_match(
    mode: MatchModeModel,
    match_model: &mut MatchModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    game_round_model: &mut GameRoundModel,
    game_location_model: &mut GameLocationModel,
    location_model_registry: Option<&LocationModelRegistry>,
    active_locations: Option<&mut ActiveLocations>,
    active_world_model: Option<&ActiveWorldModel>,
    player_deck: Option<&DeckModel>,
) {
    reset_two_player_match_to_round_one(
        mode,
        match_model,
        game_deck_model,
        game_hand_model,
        game_round_model,
        game_location_model,
        location_model_registry,
        active_locations,
        active_world_model,
        player_deck,
    );
    start_match_round(
        match_model,
        game_round_model,
        game_deck_model,
        game_hand_model,
    );
}

/// HUMAN: Resets a match to round one without dealing the first cards yet.
/// AI: GameScene intro uses this so the first deal can happen after the reveal sequence.
pub fn reset_two_player_match_without_starting_round(
    mode: MatchModeModel,
    match_model: &mut MatchModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    game_round_model: &mut GameRoundModel,
    game_location_model: &mut GameLocationModel,
    location_model_registry: Option<&LocationModelRegistry>,
    active_locations: Option<&mut ActiveLocations>,
    active_world_model: Option<&ActiveWorldModel>,
    player_deck: Option<&DeckModel>,
) {
    reset_two_player_match_to_round_one(
        mode,
        match_model,
        game_deck_model,
        game_hand_model,
        game_round_model,
        game_location_model,
        location_model_registry,
        active_locations,
        active_world_model,
        player_deck,
    );
}

fn reset_two_player_match_to_round_one(
    mode: MatchModeModel,
    match_model: &mut MatchModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    game_round_model: &mut GameRoundModel,
    game_location_model: &mut GameLocationModel,
    location_model_registry: Option<&LocationModelRegistry>,
    active_locations: Option<&mut ActiveLocations>,
    active_world_model: Option<&ActiveWorldModel>,
    player_deck: Option<&DeckModel>,
) {
    let master_deck = master_deck_from_deck_model(player_deck);
    match_model.reset_for_mode(mode, master_deck);
    game_round_model.reset();
    if let (Some(location_model_registry), Some(active_locations), Some(active_world_model)) = (
        location_model_registry,
        active_locations,
        active_world_model,
    ) {
        active_locations.reroll(location_model_registry, active_world_model);
        match_model.set_context(active_world_model, active_locations);
        game_location_model.reset_with_active_location_indices(&active_locations.indices);
    } else {
        game_location_model.reset();
    }
    game_hand_model.cards.clear();
    game_deck_model.cards.clear();
}

pub fn sync_near_human_from_game_models(
    match_model: &mut MatchModel,
    game_deck_model: &GameDeckModel,
    game_hand_model: &GameHandModel,
    game_round_model: &GameRoundModel,
) {
    if match_model.near.controller.is_cpu() {
        return;
    }
    match_model.near.deck = game_deck_model.cards.clone();
    match_model.near.hand = game_hand_model.cards.clone();
    match_model.near.energy_available = game_round_model.energy_available;
}

pub fn final_winner_from_slots(
    slot_board: &CardSlotBoardModel,
    card_registry: &CardModelRegistry,
    game_location_model: Option<&GameLocationModel>,
) -> MatchPlayerSide {
    let mut near_wins = 0;
    let mut far_wins = 0;
    let mut near_total = 0;
    let mut far_total = 0;
    for location_index in 0..CARD_SLOT_LOCATION_COUNT {
        let near = side_power_total(
            slot_board,
            card_registry,
            game_location_model,
            location_index,
            MatchPlayerSide::Near,
        );
        let far = side_power_total(
            slot_board,
            card_registry,
            game_location_model,
            location_index,
            MatchPlayerSide::Far,
        );
        near_total += near.value;
        far_total += far.value;
        if near.value > far.value {
            near_wins += 1;
        } else if far.value > near.value {
            far_wins += 1;
        }
    }

    if near_wins > far_wins {
        MatchPlayerSide::Near
    } else if far_wins > near_wins {
        MatchPlayerSide::Far
    } else if far_total > near_total {
        MatchPlayerSide::Far
    } else {
        MatchPlayerSide::Near
    }
}

fn side_power_total(
    slot_board: &CardSlotBoardModel,
    card_registry: &CardModelRegistry,
    game_location_model: Option<&GameLocationModel>,
    location_index: usize,
    side: MatchPlayerSide,
) -> PowerPointModel {
    let mut counted_card_count = 0;
    let total: i32 = slot_board
        .slots()
        .filter(|slot| slot.location_index == location_index && slot.side == side.slot_side())
        .filter_map(|slot| match &slot.state {
            CardSlotState::Empty => None,
            CardSlotState::Populated { card_id, .. } => {
                card_registry.card_model_for_id(card_id).map(|card_model| {
                    counted_card_count += 1;
                    card_model.base_power.value
                        + game_location_model
                            .map(|locations| locations.ability_delta_for_location(location_index))
                            .unwrap_or(0)
                })
            }
        })
        .sum();
    let multiplier = game_location_model
        .map(|locations| {
            locations.power_multiplier_for_location_side(location_index, counted_card_count)
        })
        .unwrap_or(1);
    PowerPointModel::new(total * multiplier)
}

pub const fn minimum_cpu_decision_delay_seconds() -> f32 {
    0.5
}

pub const fn maximum_cpu_decision_delay_seconds() -> f32 {
    0.75
}

pub fn cpu_slot_hand_index(side: MatchPlayerSide, sequence: usize) -> usize {
    match side {
        MatchPlayerSide::Near => 10_000 + sequence,
        MatchPlayerSide::Far => 20_000 + sequence,
    }
}

pub fn default_round_hand_size() -> usize {
    STARTING_HAND_CARD_COUNT
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/match_model_tests.rs"]
mod match_model_tests;
