use bevy::prelude::*;

pub const GAME_LOCATION_COUNT: usize = 3;

/// HUMAN: Current round visibility state for one GameView location.
/// AI: Only open locations apply location abilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameLocationState {
    Closed,
    Open,
}

/// HUMAN: Initial location ability set for the local gameplay loop.
/// AI: Keep this narrow until a later general ability system exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocationAbility {
    NoAbility,
    EnergyDelta(i32),
}

impl LocationAbility {
    pub const fn energy_delta(self) -> i32 {
        match self {
            Self::NoAbility => 0,
            Self::EnergyDelta(delta) => delta,
        }
    }
}

/// HUMAN: Static location display and rule definition.
/// AI: Runtime state derives from current round; do not mutate these values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationDefinitionModel {
    pub location_index: usize,
    pub opens_on_round: u8,
    pub title: &'static str,
    pub body: &'static str,
    pub ability: LocationAbility,
}

impl LocationDefinitionModel {
    pub const fn display_title(self, round: u8) -> &'static str {
        if self.is_open(round) {
            self.title
        } else {
            match self.opens_on_round {
                1 => "Closed Until Round 1",
                2 => "Closed Until Round 2",
                3 => "Closed Until Round 3",
                _ => "Closed",
            }
        }
    }

    pub const fn display_body(self, round: u8) -> &'static str {
        if self.is_open(round) { self.body } else { "" }
    }

    pub const fn state(self, round: u8) -> GameLocationState {
        if self.is_open(round) {
            GameLocationState::Open
        } else {
            GameLocationState::Closed
        }
    }

    pub const fn is_open(self, round: u8) -> bool {
        round >= self.opens_on_round
    }
}

/// HUMAN: Runtime location model derived from the current round.
/// AI: This resource feeds UI text and ability application.
#[derive(Resource, Clone, Debug, Eq, PartialEq)]
pub struct GameLocationModel {
    pub round: u8,
    pub definitions: [LocationDefinitionModel; GAME_LOCATION_COUNT],
}

impl Default for GameLocationModel {
    fn default() -> Self {
        Self {
            round: 1,
            definitions: location_definitions(),
        }
    }
}

impl GameLocationModel {
    pub fn reset(&mut self) {
        self.round = 1;
    }

    pub fn set_round(&mut self, round: u8) {
        self.round = round.clamp(1, 6);
    }

    pub fn definition(&self, location_index: usize) -> Option<LocationDefinitionModel> {
        self.definitions.get(location_index).copied()
    }

    pub fn ability_delta_for_location(&self, location_index: usize) -> i32 {
        self.definition(location_index)
            .filter(|definition| definition.is_open(self.round))
            .map(|definition| definition.ability.energy_delta())
            .unwrap_or(0)
    }

    pub fn border_color(&self, location_index: usize) -> Color {
        match self
            .definition(location_index)
            .map(|definition| definition.state(self.round))
        {
            Some(GameLocationState::Open) => Color::srgb(0.2, 0.95, 0.35),
            _ => Color::srgb(0.95, 0.18, 0.18),
        }
    }
}

pub const fn location_definitions() -> [LocationDefinitionModel; GAME_LOCATION_COUNT] {
    [
        LocationDefinitionModel {
            location_index: 0,
            opens_on_round: 1,
            title: "Fortress Gate",
            body: "+2 Power to each card here",
            ability: LocationAbility::EnergyDelta(2),
        },
        LocationDefinitionModel {
            location_index: 1,
            opens_on_round: 2,
            title: "Bamboo Crossing",
            body: "-2 Power to each card here",
            ability: LocationAbility::EnergyDelta(-2),
        },
        LocationDefinitionModel {
            location_index: 2,
            opens_on_round: 3,
            title: "Normal",
            body: "(No Ability)",
            ability: LocationAbility::NoAbility,
        },
    ]
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/game_location_model_tests.rs"]
mod game_location_model_tests;
