/// HUMAN: Six-round local gameplay state for the near player.
/// AI: Keep round, energy, and undo history here; rendering and gestures consume this state.
#[derive(Clone, Debug, PartialEq)]
pub struct CurrentRoundMoveRecord {
    pub hand_index: usize,
    pub card_id: String,
    pub location_index: usize,
    pub slot_index: usize,
    pub energy_cost: i32,
    pub location_energy_delta: i32,
}

/// HUMAN: Runtime round and energy model for the active GameView loop.
/// AI: This is reset-only local state, not persisted deck data.
#[derive(bevy::prelude::Resource, Clone, Debug, PartialEq)]
pub struct GameRoundModel {
    pub round: u8,
    pub max_rounds: u8,
    pub energy_available: i32,
    pub energy_maximum: i32,
    pub requested_cards_to_deal: usize,
    pub current_round_moves: Vec<CurrentRoundMoveRecord>,
    pub end_round_resolved: bool,
}

impl Default for GameRoundModel {
    fn default() -> Self {
        Self::for_round(1)
    }
}

impl GameRoundModel {
    pub fn for_round(round: u8) -> Self {
        let round = round.clamp(1, 6);
        let energy = energy_for_round(round);
        Self {
            round,
            max_rounds: 6,
            energy_available: energy,
            energy_maximum: energy,
            requested_cards_to_deal: requested_cards_for_round(round),
            current_round_moves: Vec::new(),
            end_round_resolved: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn can_spend(&self, cost: i32) -> bool {
        cost <= self.energy_available
    }

    pub fn spend(&mut self, cost: i32) -> bool {
        if !self.can_spend(cost) {
            return false;
        }
        self.energy_available -= cost;
        true
    }

    pub fn restore(&mut self, cost: i32) {
        self.energy_available = (self.energy_available + cost).min(self.energy_maximum);
    }

    pub fn record_move(&mut self, record: CurrentRoundMoveRecord) {
        self.current_round_moves.push(record);
    }

    pub fn remove_move_for_hand_index(
        &mut self,
        hand_index: usize,
    ) -> Option<CurrentRoundMoveRecord> {
        let index = self
            .current_round_moves
            .iter()
            .position(|record| record.hand_index == hand_index)?;
        Some(self.current_round_moves.remove(index))
    }

    pub fn has_undoable_moves(&self) -> bool {
        !self.current_round_moves.is_empty()
    }

    pub fn can_end_round(&self) -> bool {
        !self.end_round_resolved
    }

    pub fn advance_round(&mut self) -> bool {
        if self.end_round_resolved {
            return false;
        }
        self.current_round_moves.clear();
        if self.round >= self.max_rounds {
            self.end_round_resolved = true;
            return false;
        }

        *self = Self::for_round(self.round + 1);
        true
    }

    pub fn energy_label(&self) -> String {
        format!("Energy {}/{}", self.energy_available, self.energy_maximum)
    }
}

pub const fn requested_cards_for_round(round: u8) -> usize {
    match round {
        1 => 1,
        2 => 2,
        3 => 3,
        4..=6 => 1,
        _ => 0,
    }
}

pub const fn energy_for_round(round: u8) -> i32 {
    match round {
        1..=6 => round as i32,
        _ => 0,
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/game_round_model_tests.rs"]
mod game_round_model_tests;
