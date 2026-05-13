use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};

use super::{
    CARD_SLOT_LOCATION_COUNT, CardModelRegistry, CardSlotBoardModel, CardSlotSide, CardSlotState,
    DeckModel, GameDeckModel, GameHandModel, GameLocationModel, GameRoundModel, PlayerSide,
    PowerPointModel, STARTING_DECK_CARD_COUNT, STARTING_HAND_CARD_COUNT,
    random_shuffled_default_deck_cards,
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
/// AI: Opposing controllers cannot inspect current-turn hidden card identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlacementVisibility {
    CurrentTurnHidden,
    Revealed,
}

/// HUMAN: Placement visibility data for one card on the board.
/// AI: Keep reveal state deterministic and independent of render face choice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementVisibilityModel {
    pub owner: MatchPlayerSide,
    pub location_index: usize,
    pub slot_index: usize,
    pub placement_turn: u8,
    pub visibility: PlacementVisibility,
}

/// HUMAN: Per-player transient match state for one game.
/// AI: This is reset on restart/mode change and is not persisted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchPlayerModel {
    pub side: MatchPlayerSide,
    pub controller: PlayerControllerModel,
    pub deck: Vec<String>,
    pub hand: Vec<String>,
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
            deck,
            hand: Vec::new(),
            energy_available: 0,
            ready_for_next: false,
            next_slot_card_index: 0,
        }
    }

    pub fn draw(&mut self, count: usize) -> Vec<String> {
        let draw_count = count.min(self.deck.len());
        let cards: Vec<String> = self.deck.drain(0..draw_count).collect();
        self.hand.extend(cards.iter().cloned());
        cards
    }

    pub fn next_slot_card_index(&mut self) -> usize {
        let index = self.next_slot_card_index;
        self.next_slot_card_index += 1;
        index
    }
}

/// HUMAN: Match winner state after turn six resolves.
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

/// HUMAN: Two-player turn and winner state for the active match.
/// AI: Keep this as the source for readiness gating and final status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchTurnModel {
    pub turn: u8,
    pub max_turns: u8,
    pub winner: Option<MatchWinnerModel>,
}

impl Default for MatchTurnModel {
    fn default() -> Self {
        Self {
            turn: 1,
            max_turns: 6,
            winner: None,
        }
    }
}

/// HUMAN: Runtime two-player match model for opponent modes.
/// AI: Bridge existing single-player hand/slot state into two-controller gameplay.
#[derive(Resource, Clone, Debug, Eq, PartialEq)]
pub struct OpponentMatchModel {
    pub mode: MatchModeModel,
    pub near: MatchPlayerModel,
    pub far: MatchPlayerModel,
    pub turn: MatchTurnModel,
    pub placements: Vec<PlacementVisibilityModel>,
}

impl Default for OpponentMatchModel {
    fn default() -> Self {
        Self::new(MatchModeModel::default(), default_master_deck())
    }
}

impl OpponentMatchModel {
    pub fn new(mode: MatchModeModel, master_deck: Vec<String>) -> Self {
        let near_controller = match mode {
            MatchModeModel::HumanVersusCpu => PlayerControllerModel::human(),
            MatchModeModel::CpuVersusCpu => PlayerControllerModel::cpu(),
        };
        let far_controller = PlayerControllerModel::cpu();
        Self {
            mode,
            near: MatchPlayerModel::new(
                MatchPlayerSide::Near,
                near_controller,
                master_deck.clone(),
            ),
            far: MatchPlayerModel::new(MatchPlayerSide::Far, far_controller, master_deck),
            turn: MatchTurnModel::default(),
            placements: Vec::new(),
        }
    }

    pub fn reset_for_mode(&mut self, mode: MatchModeModel, master_deck: Vec<String>) {
        *self = Self::new(mode, master_deck);
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

    pub fn is_complete(&self) -> bool {
        self.turn.winner.is_some()
    }

    pub fn status_text(&self) -> String {
        self.turn
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
            placement_turn: self.turn.turn,
            visibility: PlacementVisibility::CurrentTurnHidden,
        });
    }

    pub fn reveal_current_turn_placements(&mut self) {
        for placement in &mut self.placements {
            if placement.placement_turn == self.turn.turn {
                placement.visibility = PlacementVisibility::Revealed;
            }
        }
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
        cards
    }
}

pub fn start_match_turn(
    match_model: &mut OpponentMatchModel,
    game_round_model: &GameRoundModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
) {
    match_model.near.ready_for_next = false;
    match_model.far.ready_for_next = false;
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
    match_model: &mut OpponentMatchModel,
    game_deck_model: &mut GameDeckModel,
    game_hand_model: &mut GameHandModel,
    game_round_model: &mut GameRoundModel,
    game_location_model: &mut GameLocationModel,
    player_deck: Option<&DeckModel>,
) {
    let master_deck = master_deck_from_deck_model(player_deck);
    match_model.reset_for_mode(mode, master_deck);
    game_round_model.reset();
    game_location_model.reset();
    game_hand_model.cards.clear();
    game_deck_model.cards.clear();
    start_match_turn(
        match_model,
        game_round_model,
        game_deck_model,
        game_hand_model,
    );
}

pub fn sync_near_human_from_game_models(
    match_model: &mut OpponentMatchModel,
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
        if near.value >= far.value {
            near_wins += 1;
        } else {
            far_wins += 1;
        }
    }

    if near_wins >= far_wins {
        MatchPlayerSide::Near
    } else {
        MatchPlayerSide::Far
    }
}

fn side_power_total(
    slot_board: &CardSlotBoardModel,
    card_registry: &CardModelRegistry,
    game_location_model: Option<&GameLocationModel>,
    location_index: usize,
    side: MatchPlayerSide,
) -> PowerPointModel {
    let total = slot_board
        .slots()
        .filter(|slot| slot.location_index == location_index && slot.side == side.slot_side())
        .filter_map(|slot| match &slot.state {
            CardSlotState::Empty => None,
            CardSlotState::Populated { card_id, .. } => {
                card_registry.card_model_for_id(card_id).map(|card_model| {
                    card_model.base_power.value
                        + game_location_model
                            .map(|locations| locations.ability_delta_for_location(location_index))
                            .unwrap_or(0)
                })
            }
        })
        .sum();
    PowerPointModel::new(total)
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

pub fn default_turn_hand_size() -> usize {
    STARTING_HAND_CARD_COUNT
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/opponent_match_model_tests.rs"]
mod opponent_match_model_tests;
