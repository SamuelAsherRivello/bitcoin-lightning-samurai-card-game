use bevy::prelude::*;

use super::{
    CardModelRegistry, CardSlotBoardModel, MatchPlayerSide, OpponentMatchModel,
    cpu_slot_hand_index, maximum_cpu_decision_delay_seconds, minimum_cpu_decision_delay_seconds,
};

/// HUMAN: One legal card placement choice selected by CPU Brain.
/// AI: Keep this data-only so systems can dispatch it through shared placement helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuBrainMoveModel {
    pub instance_id: u64,
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
    near_presented_round: u8,
    far_presented_round: u8,
    near_hand_presentation_seconds: f32,
    far_hand_presentation_seconds: f32,
}

impl Default for CpuBrainModel {
    fn default() -> Self {
        Self {
            seed: 14,
            near_next_decision_seconds: minimum_cpu_decision_delay_seconds(),
            far_next_decision_seconds: minimum_cpu_decision_delay_seconds(),
            near_presented_round: 0,
            far_presented_round: 0,
            near_hand_presentation_seconds: 0.0,
            far_hand_presentation_seconds: 0.0,
        }
    }
}

impl CpuBrainModel {
    pub fn reset(&mut self) {
        self.near_next_decision_seconds = minimum_cpu_decision_delay_seconds();
        self.far_next_decision_seconds = minimum_cpu_decision_delay_seconds();
        self.near_presented_round = 0;
        self.far_presented_round = 0;
        self.near_hand_presentation_seconds = 0.0;
        self.far_hand_presentation_seconds = 0.0;
    }

    pub fn wait_for_hand_presentation(
        &mut self,
        side: MatchPlayerSide,
        round: u8,
        hand_card_count: usize,
        delta_seconds: f32,
        duration_seconds: f32,
    ) -> bool {
        let (presented_round, timer) = self.presentation_state_mut(side);
        if *presented_round != round {
            *presented_round = round;
            *timer = if hand_card_count == 0 {
                0.0
            } else {
                duration_seconds.max(0.0)
            };
            return *timer > 0.0;
        }
        if *timer <= 0.0 {
            return false;
        }
        *timer = (*timer - delta_seconds.max(0.0)).max(0.0);
        *timer > 0.0
    }

    pub fn wait_for_settled_hand_pause(
        &mut self,
        side: MatchPlayerSide,
        round: u8,
        hand_card_count: usize,
        hand_cards_are_settled: bool,
        delta_seconds: f32,
        pause_seconds: f32,
    ) -> bool {
        let (presented_round, timer) = self.presentation_state_mut(side);
        if *presented_round != round {
            *presented_round = round;
            *timer = if hand_card_count == 0 {
                0.0
            } else {
                pause_seconds.max(0.0)
            };
        }
        if hand_card_count == 0 {
            *timer = 0.0;
            return false;
        }
        if !hand_cards_are_settled {
            *timer = pause_seconds.max(0.0);
            return true;
        }
        if *timer <= 0.0 {
            return false;
        }
        *timer = (*timer - delta_seconds.max(0.0)).max(0.0);
        *timer > 0.0
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

    fn presentation_state_mut(&mut self, side: MatchPlayerSide) -> (&mut u8, &mut f32) {
        match side {
            MatchPlayerSide::Near => (
                &mut self.near_presented_round,
                &mut self.near_hand_presentation_seconds,
            ),
            MatchPlayerSide::Far => (
                &mut self.far_presented_round,
                &mut self.far_hand_presentation_seconds,
            ),
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
                instance_id: player
                    .hand_instance_id(hand_index)
                    .unwrap_or(((side.player_number() as u64) << 32) | hand_index as u64),
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

pub fn choose_level1_moves(
    match_model: &OpponentMatchModel,
    side: MatchPlayerSide,
    slot_board: &CardSlotBoardModel,
    card_registry: &CardModelRegistry,
    seed: u64,
) -> Vec<CpuBrainMoveModel> {
    let mut simulated_match = match_model.clone();
    let mut simulated_slots = slot_board.clone();
    let mut moves = Vec::new();

    loop {
        let Some(selected_move) = choose_level1_move(
            &simulated_match,
            side,
            &simulated_slots,
            card_registry,
            seed.wrapping_add(moves.len() as u64),
        ) else {
            break;
        };
        let player = simulated_match.player_mut(side);
        if selected_move.hand_index >= player.hand.len()
            || selected_move.energy_cost > player.energy_available
        {
            break;
        }
        player.energy_available -= selected_move.energy_cost;
        player.remove_hand_card(selected_move.hand_index);
        let slot_hand_index = cpu_slot_hand_index(side, moves.len());
        if !simulated_slots.place_for_side_with_card_id(
            selected_move.location_index,
            side.slot_side(),
            selected_move.slot_index,
            slot_hand_index,
            selected_move.card_id.clone(),
        ) {
            break;
        }
        moves.push(selected_move);
    }

    moves
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/cpu_brain_model_tests.rs"]
mod cpu_brain_model_tests;
