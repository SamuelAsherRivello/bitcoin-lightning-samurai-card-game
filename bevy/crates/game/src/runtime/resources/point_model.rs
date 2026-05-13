pub const POINT_VIEW_DISPLAY_MIN: i32 = -99;
pub const POINT_VIEW_DISPLAY_MAX: i32 = 99;
pub const SHARED_LOCATION_COUNT: usize = 3;
pub const DEFAULT_LOCATION_CARD_CAPACITY_PER_PLAYER: usize = 4;

pub fn random_point_value() -> i32 {
    fastrand::i32(POINT_VIEW_DISPLAY_MIN..=POINT_VIEW_DISPLAY_MAX)
}

/// HUMAN: Cost point data for cards and future cost-bearing gameplay entities.
/// AI: Keep cost distinct from scoring power; cost never contributes to totals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CostPointModel {
    pub value: i32,
}

impl CostPointModel {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn random() -> Self {
        Self::new(random_point_value())
    }

    pub fn display_text(self) -> String {
        self.value.to_string()
    }

    pub const fn is_in_display_contract(self) -> bool {
        self.value >= POINT_VIEW_DISPLAY_MIN && self.value <= POINT_VIEW_DISPLAY_MAX
    }
}

/// HUMAN: Power point data for card power and shared location totals.
/// AI: Use this for scoring values only; negative values are valid match state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PowerPointModel {
    pub value: i32,
}

impl PowerPointModel {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn random() -> Self {
        Self::new(random_point_value())
    }

    pub fn display_text(self) -> String {
        self.value.to_string()
    }

    pub const fn is_in_display_contract(self) -> bool {
        self.value >= POINT_VIEW_DISPLAY_MIN && self.value <= POINT_VIEW_DISPLAY_MAX
    }
}

impl std::ops::Add for PowerPointModel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerSide {
    Local,
    Opponent,
}

/// HUMAN: Runtime card copy used by point scoring.
/// AI: Keep effective power separate from CardModel base power and source data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardInstanceModel {
    pub card_id: &'static str,
    pub owner: PlayerSide,
    pub location_index: usize,
    pub is_revealed: bool,
    pub base_power: PowerPointModel,
    pub effective_power_delta: PowerPointModel,
}

impl CardInstanceModel {
    pub const fn new(
        card_id: &'static str,
        owner: PlayerSide,
        location_index: usize,
        is_revealed: bool,
        base_power: PowerPointModel,
    ) -> Self {
        Self {
            card_id,
            owner,
            location_index,
            is_revealed,
            base_power,
            effective_power_delta: PowerPointModel::new(0),
        }
    }

    pub const fn with_effective_power_delta(mut self, delta: PowerPointModel) -> Self {
        self.effective_power_delta = delta;
        self
    }

    pub fn effective_power(&self) -> PowerPointModel {
        self.base_power + self.effective_power_delta
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocationScoreError {
    TooManyCards {
        owner: PlayerSide,
        location_index: usize,
        count: usize,
        capacity: usize,
    },
}

/// HUMAN: One shared location's resolved scoring state.
/// AI: Derive this from card instances and modifiers; do not store visual-only text here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationScoreModel {
    pub location_index: usize,
    pub local_total: PowerPointModel,
    pub opponent_total: PowerPointModel,
    pub local_modifier: PowerPointModel,
    pub opponent_modifier: PowerPointModel,
    pub capacity_per_player: usize,
}

impl LocationScoreModel {
    pub const fn empty(location_index: usize) -> Self {
        Self {
            location_index,
            local_total: PowerPointModel::new(0),
            opponent_total: PowerPointModel::new(0),
            local_modifier: PowerPointModel::new(0),
            opponent_modifier: PowerPointModel::new(0),
            capacity_per_player: DEFAULT_LOCATION_CARD_CAPACITY_PER_PLAYER,
        }
    }

    pub fn from_cards(
        location_index: usize,
        cards: &[CardInstanceModel],
        local_modifier: PowerPointModel,
        opponent_modifier: PowerPointModel,
    ) -> Result<Self, LocationScoreError> {
        Self::from_cards_with_capacity(
            location_index,
            cards,
            local_modifier,
            opponent_modifier,
            DEFAULT_LOCATION_CARD_CAPACITY_PER_PLAYER,
        )
    }

    pub fn from_cards_with_capacity(
        location_index: usize,
        cards: &[CardInstanceModel],
        local_modifier: PowerPointModel,
        opponent_modifier: PowerPointModel,
        capacity_per_player: usize,
    ) -> Result<Self, LocationScoreError> {
        let mut local_count = 0;
        let mut opponent_count = 0;
        let mut local_total = local_modifier;
        let mut opponent_total = opponent_modifier;

        for card in cards
            .iter()
            .filter(|card| card.location_index == location_index && card.is_revealed)
        {
            match card.owner {
                PlayerSide::Local => {
                    local_count += 1;
                    if local_count > capacity_per_player {
                        return Err(LocationScoreError::TooManyCards {
                            owner: PlayerSide::Local,
                            location_index,
                            count: local_count,
                            capacity: capacity_per_player,
                        });
                    }
                    local_total = local_total + card.effective_power();
                }
                PlayerSide::Opponent => {
                    opponent_count += 1;
                    if opponent_count > capacity_per_player {
                        return Err(LocationScoreError::TooManyCards {
                            owner: PlayerSide::Opponent,
                            location_index,
                            count: opponent_count,
                            capacity: capacity_per_player,
                        });
                    }
                    opponent_total = opponent_total + card.effective_power();
                }
            }
        }

        Ok(Self {
            location_index,
            local_total,
            opponent_total,
            local_modifier,
            opponent_modifier,
            capacity_per_player,
        })
    }

    pub fn control(self) -> LocationControlModel {
        LocationControlModel {
            location_index: self.location_index,
            controller: if self.local_total.value > self.opponent_total.value {
                LocationController::Local
            } else if self.opponent_total.value > self.local_total.value {
                LocationController::Opponent
            } else {
                LocationController::None
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocationController {
    Local,
    Opponent,
    None,
}

/// HUMAN: Controller result for one shared location.
/// AI: Use None for tied and empty equal-zero locations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationControlModel {
    pub location_index: usize,
    pub controller: LocationController,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchOutcome {
    LocalWin,
    OpponentWin,
    Draw,
}

/// HUMAN: Aggregate scoring state for the three shared locations.
/// AI: Keep final outcome calculation deterministic and independent of rendering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchScoreModel {
    pub locations: [LocationScoreModel; SHARED_LOCATION_COUNT],
}

impl MatchScoreModel {
    pub const fn new(locations: [LocationScoreModel; SHARED_LOCATION_COUNT]) -> Self {
        Self { locations }
    }

    pub fn local_total_power(self) -> PowerPointModel {
        PowerPointModel::new(
            self.locations
                .iter()
                .map(|location| location.local_total.value)
                .sum(),
        )
    }

    pub fn opponent_total_power(self) -> PowerPointModel {
        PowerPointModel::new(
            self.locations
                .iter()
                .map(|location| location.opponent_total.value)
                .sum(),
        )
    }

    pub fn controlled_counts(self) -> (usize, usize) {
        self.locations
            .iter()
            .map(|location| location.control().controller)
            .fold((0, 0), |(local, opponent), controller| match controller {
                LocationController::Local => (local + 1, opponent),
                LocationController::Opponent => (local, opponent + 1),
                LocationController::None => (local, opponent),
            })
    }

    pub fn outcome(self) -> MatchOutcomeModel {
        let (local_controlled_count, opponent_controlled_count) = self.controlled_counts();
        let local_total_power = self.local_total_power();
        let opponent_total_power = self.opponent_total_power();

        let result = if local_controlled_count > opponent_controlled_count {
            MatchOutcome::LocalWin
        } else if opponent_controlled_count > local_controlled_count {
            MatchOutcome::OpponentWin
        } else if local_total_power.value > opponent_total_power.value {
            MatchOutcome::LocalWin
        } else if opponent_total_power.value > local_total_power.value {
            MatchOutcome::OpponentWin
        } else {
            MatchOutcome::Draw
        };

        MatchOutcomeModel {
            result,
            local_controlled_count,
            opponent_controlled_count,
            local_total_power,
            opponent_total_power,
        }
    }
}

/// HUMAN: Final match result after location control and total-power tiebreaking.
/// AI: This is the stable integration point for later round, CPU, and end-game flows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchOutcomeModel {
    pub result: MatchOutcome,
    pub local_controlled_count: usize,
    pub opponent_controlled_count: usize,
    pub local_total_power: PowerPointModel,
    pub opponent_total_power: PowerPointModel,
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/point_model_tests.rs"]
mod point_model_tests;
