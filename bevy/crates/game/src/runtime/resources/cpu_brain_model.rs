use bevy::prelude::*;

use super::{
    CardModelRegistry, CardSlotBoardModel, MatchPlayerSide, OpponentMatchModel,
    maximum_cpu_decision_delay_seconds, minimum_cpu_decision_delay_seconds,
};

/// HUMAN: One legal card placement choice selected by CPU Brain.
/// AI: Keep this data-only so systems can dispatch it through shared placement helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuBrainMoveModel {
    pub hand_index: usize,
    pub card_id: String,
    pub location_index: usize,
    pub slot_index: usize,
    pub energy_cost: i32,
    pub score: i32,
}

/// HUMAN: Runtime pacing and seeded randomness for CPU player decisions.
/// AI: This is authored gameplay logic and never calls a runtime generative model.
#[derive(Resource, Debug)]
pub struct CpuBrainModel {
    pub seed: u64,
    pub near_next_decision_seconds: f32,
    pub far_next_decision_seconds: f32,
}

impl Default for CpuBrainModel {
    fn default() -> Self {
        Self {
            seed: 14,
            near_next_decision_seconds: minimum_cpu_decision_delay_seconds(),
            far_next_decision_seconds: minimum_cpu_decision_delay_seconds(),
        }
    }
}

impl CpuBrainModel {
    pub fn reset(&mut self) {
        self.near_next_decision_seconds = minimum_cpu_decision_delay_seconds();
        self.far_next_decision_seconds = minimum_cpu_decision_delay_seconds();
    }

    pub fn tick(&mut self, side: MatchPlayerSide, delta_seconds: f32) -> bool {
        let timer = self.timer_mut(side);
        *timer -= delta_seconds.max(0.0);
        *timer <= 0.0
    }

    pub fn schedule_next(&mut self, side: MatchPlayerSide) {
        let salt = match side {
            MatchPlayerSide::Near => 0xA11C_E001,
            MatchPlayerSide::Far => 0xFACE_1001,
        };
        let mut rng = fastrand::Rng::with_seed(self.seed ^ salt);
        let delay = rng.f32()
            * (maximum_cpu_decision_delay_seconds() - minimum_cpu_decision_delay_seconds())
            + minimum_cpu_decision_delay_seconds();
        *self.timer_mut(side) = delay;
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    }

    fn timer_mut(&mut self, side: MatchPlayerSide) -> &mut f32 {
        match side {
            MatchPlayerSide::Near => &mut self.near_next_decision_seconds,
            MatchPlayerSide::Far => &mut self.far_next_decision_seconds,
        }
    }
}

pub fn choose_level1_move(
    match_model: &OpponentMatchModel,
    side: MatchPlayerSide,
    slot_board: &CardSlotBoardModel,
    card_registry: &CardModelRegistry,
    seed: u64,
) -> Option<CpuBrainMoveModel> {
    let player = match_model.player(side);
    let mut moves = Vec::new();
    for (hand_index, card_id) in player.hand.iter().enumerate() {
        let Some(card_model) = card_registry.card_model_for_id(card_id) else {
            continue;
        };
        if card_model.cost.value > player.energy_available {
            continue;
        }
        for location_index in 0..super::CARD_SLOT_LOCATION_COUNT {
            let Some(slot_index) = slot_board.next_available_slot(location_index, side.slot_side())
            else {
                continue;
            };
            let score = card_model.base_power.value
                + card_model.cost.value
                + location_index as i32
                + if side == MatchPlayerSide::Near { 1 } else { 0 };
            moves.push(CpuBrainMoveModel {
                hand_index,
                card_id: card_id.clone(),
                location_index,
                slot_index,
                energy_cost: card_model.cost.value,
                score,
            });
        }
    }

    let best_score = moves.iter().map(|candidate| candidate.score).max()?;
    let mut best_moves: Vec<_> = moves
        .into_iter()
        .filter(|candidate| candidate.score == best_score)
        .collect();
    let mut rng = fastrand::Rng::with_seed(seed ^ ((side.player_number() as u64) << 32));
    let index = rng.usize(0..best_moves.len());
    Some(best_moves.swap_remove(index))
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/cpu_brain_model_tests.rs"]
mod cpu_brain_model_tests;
