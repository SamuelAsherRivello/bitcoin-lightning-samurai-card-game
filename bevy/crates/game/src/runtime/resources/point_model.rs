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
/// AI: This is the stable integration point for later turn, CPU, and end-game flows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatchOutcomeModel {
    pub result: MatchOutcome,
    pub local_controlled_count: usize,
    pub opponent_controlled_count: usize,
    pub local_total_power: PowerPointModel,
    pub opponent_total_power: PowerPointModel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_models_render_display_contract_values() {
        for value in [POINT_VIEW_DISPLAY_MIN, 0, POINT_VIEW_DISPLAY_MAX] {
            let cost = CostPointModel::new(value);
            let power = PowerPointModel::new(value);

            assert!(cost.is_in_display_contract());
            assert!(power.is_in_display_contract());
            assert_eq!(cost.display_text(), value.to_string());
            assert_eq!(power.display_text(), value.to_string());
        }

        assert!(!CostPointModel::new(POINT_VIEW_DISPLAY_MIN - 1).is_in_display_contract());
        assert!(!PowerPointModel::new(POINT_VIEW_DISPLAY_MAX + 1).is_in_display_contract());
    }

    #[test]
    fn random_point_models_stay_inside_display_contract() {
        for _ in 0..256 {
            let cost = CostPointModel::random();
            let power = PowerPointModel::random();

            assert!(cost.is_in_display_contract());
            assert!(power.is_in_display_contract());
        }
    }

    #[test]
    fn card_instance_effective_power_keeps_base_power_separate() {
        let card = CardInstanceModel::new(
            "sample",
            PlayerSide::Local,
            0,
            true,
            PowerPointModel::new(3),
        )
        .with_effective_power_delta(PowerPointModel::new(2));

        assert_eq!(card.base_power, PowerPointModel::new(3));
        assert_eq!(card.effective_power(), PowerPointModel::new(5));
    }

    #[test]
    fn card_cost_never_contributes_to_location_total() {
        let cost = CostPointModel::new(9);
        let card = CardInstanceModel::new(
            "sample",
            PlayerSide::Local,
            0,
            true,
            PowerPointModel::new(2),
        );

        let location = LocationScoreModel::from_cards(
            0,
            &[card],
            PowerPointModel::new(0),
            PowerPointModel::new(0),
        )
        .unwrap();

        assert_eq!(cost.value, 9);
        assert_eq!(location.local_total, PowerPointModel::new(2));
    }

    #[test]
    fn location_total_uses_revealed_effective_power_and_modifiers() {
        let cards = [
            CardInstanceModel::new(
                "local_revealed",
                PlayerSide::Local,
                0,
                true,
                PowerPointModel::new(3),
            )
            .with_effective_power_delta(PowerPointModel::new(2)),
            CardInstanceModel::new(
                "local_hidden",
                PlayerSide::Local,
                0,
                false,
                PowerPointModel::new(99),
            ),
            CardInstanceModel::new(
                "opponent_revealed",
                PlayerSide::Opponent,
                0,
                true,
                PowerPointModel::new(4),
            ),
        ];

        let location = LocationScoreModel::from_cards(
            0,
            &cards,
            PowerPointModel::new(-1),
            PowerPointModel::new(3),
        )
        .unwrap();

        assert_eq!(location.local_total, PowerPointModel::new(4));
        assert_eq!(location.opponent_total, PowerPointModel::new(7));
    }

    #[test]
    fn moved_card_contributes_only_to_current_location() {
        let moved_card =
            CardInstanceModel::new("moved", PlayerSide::Local, 1, true, PowerPointModel::new(5));

        let old_location = LocationScoreModel::from_cards(
            0,
            std::slice::from_ref(&moved_card),
            PowerPointModel::new(0),
            PowerPointModel::new(0),
        )
        .unwrap();
        let new_location = LocationScoreModel::from_cards(
            1,
            &[moved_card],
            PowerPointModel::new(0),
            PowerPointModel::new(0),
        )
        .unwrap();

        assert_eq!(old_location.local_total, PowerPointModel::new(0));
        assert_eq!(new_location.local_total, PowerPointModel::new(5));
    }

    #[test]
    fn location_capacity_rejects_more_than_four_cards_per_player() {
        let cards = [
            CardInstanceModel::new("a", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
            CardInstanceModel::new("b", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
            CardInstanceModel::new("c", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
            CardInstanceModel::new("d", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
            CardInstanceModel::new("e", PlayerSide::Local, 0, true, PowerPointModel::new(1)),
        ];

        assert_eq!(
            LocationScoreModel::from_cards(
                0,
                &cards,
                PowerPointModel::new(0),
                PowerPointModel::new(0),
            ),
            Err(LocationScoreError::TooManyCards {
                owner: PlayerSide::Local,
                location_index: 0,
                count: 5,
                capacity: DEFAULT_LOCATION_CARD_CAPACITY_PER_PLAYER,
            })
        );
    }

    #[test]
    fn location_control_covers_leads_ties_and_empty_locations() {
        let local_lead = LocationScoreModel {
            local_total: PowerPointModel::new(5),
            opponent_total: PowerPointModel::new(3),
            ..LocationScoreModel::empty(0)
        };
        let opponent_lead = LocationScoreModel {
            local_total: PowerPointModel::new(2),
            opponent_total: PowerPointModel::new(7),
            ..LocationScoreModel::empty(1)
        };
        let tied = LocationScoreModel {
            local_total: PowerPointModel::new(4),
            opponent_total: PowerPointModel::new(4),
            ..LocationScoreModel::empty(2)
        };

        assert_eq!(local_lead.control().controller, LocationController::Local);
        assert_eq!(
            opponent_lead.control().controller,
            LocationController::Opponent
        );
        assert_eq!(tied.control().controller, LocationController::None);
        assert_eq!(
            LocationScoreModel::empty(0).control().controller,
            LocationController::None
        );
    }

    #[test]
    fn match_outcome_uses_location_count_before_total_power() {
        let match_score = MatchScoreModel::new([
            LocationScoreModel {
                local_total: PowerPointModel::new(1),
                opponent_total: PowerPointModel::new(0),
                ..LocationScoreModel::empty(0)
            },
            LocationScoreModel {
                local_total: PowerPointModel::new(1),
                opponent_total: PowerPointModel::new(0),
                ..LocationScoreModel::empty(1)
            },
            LocationScoreModel {
                local_total: PowerPointModel::new(0),
                opponent_total: PowerPointModel::new(20),
                ..LocationScoreModel::empty(2)
            },
        ]);

        let outcome = match_score.outcome();

        assert_eq!(outcome.result, MatchOutcome::LocalWin);
        assert_eq!(outcome.local_controlled_count, 2);
        assert_eq!(outcome.opponent_controlled_count, 1);
    }

    #[test]
    fn match_outcome_uses_total_power_tiebreaker_then_draw() {
        let local_tiebreak = MatchScoreModel::new([
            LocationScoreModel {
                local_total: PowerPointModel::new(8),
                opponent_total: PowerPointModel::new(1),
                ..LocationScoreModel::empty(0)
            },
            LocationScoreModel {
                local_total: PowerPointModel::new(1),
                opponent_total: PowerPointModel::new(6),
                ..LocationScoreModel::empty(1)
            },
            LocationScoreModel {
                local_total: PowerPointModel::new(0),
                opponent_total: PowerPointModel::new(0),
                ..LocationScoreModel::empty(2)
            },
        ]);
        let draw = MatchScoreModel::new([
            LocationScoreModel {
                local_total: PowerPointModel::new(5),
                opponent_total: PowerPointModel::new(3),
                ..LocationScoreModel::empty(0)
            },
            LocationScoreModel {
                local_total: PowerPointModel::new(1),
                opponent_total: PowerPointModel::new(3),
                ..LocationScoreModel::empty(1)
            },
            LocationScoreModel {
                local_total: PowerPointModel::new(0),
                opponent_total: PowerPointModel::new(0),
                ..LocationScoreModel::empty(2)
            },
        ]);

        assert_eq!(local_tiebreak.outcome().result, MatchOutcome::LocalWin);
        assert_eq!(draw.outcome().result, MatchOutcome::Draw);
    }
}
