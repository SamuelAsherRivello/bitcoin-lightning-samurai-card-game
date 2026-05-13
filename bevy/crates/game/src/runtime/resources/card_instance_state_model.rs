use bevy::prelude::*;

use crate::runtime::components::{CpuHandCardView, CpuPlacedCardView};

use super::{
    CardFace, CardGestureState, CardSlotBoardModel, CardSlotSide, CardState, CardStateModel,
    GameHandModel, MatchPlayerSide, PlacementVisibility, PlacementVisibilityModel,
};

/// HUMAN: Documents the current runtime owners for each card state axis.
/// AI: Keep this inventory descriptive; durable gameplay state lives in CardInstanceStateModel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardStateAxisModel {
    pub axis: &'static str,
    pub owner: &'static str,
    pub values: &'static [&'static str],
}

impl CardStateAxisModel {
    pub const fn current_inventory() -> &'static [Self] {
        &[
            Self {
                axis: "visual_root",
                owner: "CardViewBundle/CardView",
                values: &["spawned_root", "transform", "visibility"],
            },
            Self {
                axis: "face",
                owner: "CardFace/CardFaceLayer/CardFlipState",
                values: &["Front", "Back"],
            },
            Self {
                axis: "local_zone",
                owner: "CardStateModel/CardState",
                values: &["Hand", "Dragging", "Location", "LocationLocked"],
            },
            Self {
                axis: "interaction",
                owner: "CardGestureModel/CardGestureState",
                values: &[
                    "Idle",
                    "Pressed",
                    "SelectedInspecting",
                    "Dragging",
                    "Returning",
                    "Placed",
                ],
            },
            Self {
                axis: "slot_occupancy",
                owner: "CardSlotBoardModel/CardSlotState",
                values: &["Empty", "Populated"],
            },
            Self {
                axis: "reveal",
                owner: "PlacementVisibilityModel/PlacementVisibility",
                values: &["CurrentRoundHidden", "Revealed"],
            },
            Self {
                axis: "cpu_presentation",
                owner: "CpuHandCardView/CpuPlacedCardView/CpuPlacedCardAnimation",
                values: &["passive_hand", "passive_placed", "moving", "revealing"],
            },
        ]
    }

    pub fn by_axis(axis: &str) -> Option<&'static Self> {
        Self::current_inventory()
            .iter()
            .find(|state_axis| state_axis.axis == axis)
    }
}

/// HUMAN: Stable identity for one runtime card instance in an active match.
/// AI: Prefer this over hand_index when connecting decks, hands, slots, gestures, and views.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardInstanceId(pub u64);

impl CardInstanceId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn from_owner_index(owner: CardOwnerModel, index: usize) -> Self {
        let owner_offset = match owner.side {
            MatchPlayerSide::Near => 0_u64,
            MatchPlayerSide::Far => 1_000_000_u64,
        };
        Self(owner_offset + index as u64)
    }
}

/// HUMAN: Owner side for one runtime card instance.
/// AI: Controller behavior is derived elsewhere; this records board ownership only.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardOwnerModel {
    pub side: MatchPlayerSide,
}

impl CardOwnerModel {
    pub const fn new(side: MatchPlayerSide) -> Self {
        Self { side }
    }

    pub const fn near() -> Self {
        Self::new(MatchPlayerSide::Near)
    }

    pub const fn far() -> Self {
        Self::new(MatchPlayerSide::Far)
    }
}

/// HUMAN: Movement lock state for a card already assigned to a location slot.
/// AI: CurrentRoundMovable maps the existing CardState::Location behavior before end round.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LocationLockState {
    CurrentRoundMovable,
    Locked,
}

/// HUMAN: Durable zone for one card instance.
/// AI: Selection, dragging, and animation are overlays; they do not change this source zone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardZoneModel {
    Deck {
        deck_index: usize,
    },
    Hand {
        order_index: usize,
    },
    Location {
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
        lock_state: LocationLockState,
    },
    OutOfPlay,
}

impl CardZoneModel {
    pub const fn is_draggable_source(self) -> bool {
        matches!(
            self,
            Self::Hand { .. }
                | Self::Location {
                    lock_state: LocationLockState::CurrentRoundMovable,
                    ..
                }
        )
    }

    pub const fn is_selectable_source(self) -> bool {
        matches!(self, Self::Hand { .. } | Self::Location { .. })
    }

    pub const fn slot_identity(self) -> Option<(usize, CardSlotSide, usize)> {
        match self {
            Self::Location {
                location_index,
                side,
                slot_index,
                ..
            } => Some((location_index, side, slot_index)),
            _ => None,
        }
    }
}

/// HUMAN: Gameplay reveal policy for one card instance.
/// AI: This answers controller knowledge; CardFace remains the render-facing mesh side.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardRevealPolicy {
    OwnerVisible,
    CurrentRoundHiddenToOpponent,
    RevealedToAll,
}

impl CardRevealPolicy {
    pub fn from_placement_visibility(visibility: PlacementVisibility) -> Self {
        match visibility {
            PlacementVisibility::CurrentRoundHidden | PlacementVisibility::Revealing => {
                Self::CurrentRoundHiddenToOpponent
            }
            PlacementVisibility::Revealed => Self::RevealedToAll,
        }
    }

    pub fn visible_face(
        self,
        viewer: MatchPlayerSide,
        owner: MatchPlayerSide,
        front_face: CardFace,
    ) -> CardFace {
        let may_see_front = match self {
            Self::OwnerVisible | Self::CurrentRoundHiddenToOpponent => viewer == owner,
            Self::RevealedToAll => true,
        };
        if may_see_front {
            front_face
        } else {
            CardFace::Back
        }
    }
}

/// HUMAN: Durable state for one runtime card instance.
/// AI: Keep one instance in exactly one zone; derive views and interactions from this state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardInstanceStateModel {
    pub instance_id: CardInstanceId,
    pub card_model_id: String,
    pub owner: CardOwnerModel,
    pub zone: CardZoneModel,
    pub reveal_policy: CardRevealPolicy,
}

impl CardInstanceStateModel {
    pub fn new(
        instance_id: CardInstanceId,
        card_model_id: impl Into<String>,
        owner: CardOwnerModel,
        zone: CardZoneModel,
        reveal_policy: CardRevealPolicy,
    ) -> Self {
        Self {
            instance_id,
            card_model_id: card_model_id.into(),
            owner,
            zone,
            reveal_policy,
        }
    }

    pub fn validate(&self) -> Result<(), CardStateValidationError> {
        if self.card_model_id.is_empty() {
            return Err(CardStateValidationError::MissingCardModelId {
                instance_id: self.instance_id,
            });
        }
        if self.reveal_policy == CardRevealPolicy::CurrentRoundHiddenToOpponent
            && !matches!(self.zone, CardZoneModel::Location { .. })
        {
            return Err(CardStateValidationError::HiddenCardOutsideLocation {
                instance_id: self.instance_id,
            });
        }
        Ok(())
    }
}

/// HUMAN: Pose source for a rendered CardView.
/// AI: This is derived from durable zone plus interaction, never used for legality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardViewPoseModel {
    Deck,
    Hand,
    LocationSlot,
    SelectedInspection,
    DragPreview,
    ReturnTarget,
    SettlingPlaced,
    OutOfPlay,
}

/// HUMAN: Relative render band for card view presentation.
/// AI: Keep z intent explicit so selected and dragged views can sit above base hand/slot views.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardViewZBand {
    Deck,
    Hand,
    LocationSlot,
    Selected,
    Drag,
    Hidden,
}

/// HUMAN: Local interaction affordance exposed by a rendered card view.
/// AI: Derive this from owner, controller, zone, lock state, and active interaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardInputAffordance {
    None,
    Passive,
    Selectable,
    Draggable,
}

/// HUMAN: Render-facing view state derived for a CardView entity.
/// AI: This mirrors what systems need to apply without duplicating durable gameplay truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardViewStateModel {
    pub instance_id: CardInstanceId,
    pub visible_face: CardFace,
    pub pose: CardViewPoseModel,
    pub z_band: CardViewZBand,
    pub input_affordance: CardInputAffordance,
}

impl CardViewStateModel {
    pub fn derive_for_viewer(
        card: &CardInstanceStateModel,
        viewer: MatchPlayerSide,
        interaction: Option<&CardInteractionModel>,
    ) -> Self {
        let active_interaction = interaction
            .filter(|interaction| interaction.active_instance_id == Some(card.instance_id));
        let pose = active_interaction
            .map(|interaction| interaction.pose_override())
            .unwrap_or_else(|| pose_for_zone(card.zone));
        let z_band = z_band_for_pose(pose);
        Self {
            instance_id: card.instance_id,
            visible_face: card
                .reveal_policy
                .visible_face(viewer, card.owner.side, CardFace::Front),
            pose,
            z_band,
            input_affordance: input_affordance_for(card, viewer, active_interaction),
        }
    }
}

/// HUMAN: Transient interaction state for one active card instance.
/// AI: This replaces hand-index focus while preserving the existing gesture state vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardInteractionState {
    Idle,
    Pressed,
    SelectedInspecting,
    Dragging,
    Returning,
    SettlingPlaced,
}

impl From<CardGestureState> for CardInteractionState {
    fn from(value: CardGestureState) -> Self {
        match value {
            CardGestureState::Idle => Self::Idle,
            CardGestureState::Pressed => Self::Pressed,
            CardGestureState::SelectedInspecting => Self::SelectedInspecting,
            CardGestureState::Dragging => Self::Dragging,
            CardGestureState::Returning => Self::Returning,
            CardGestureState::Placed => Self::SettlingPlaced,
        }
    }
}

/// HUMAN: One active card interaction overlay.
/// AI: Keep selection and dragging separate from CardZoneModel so returns know their source.
#[derive(Clone, Debug, PartialEq)]
pub struct CardInteractionModel {
    pub state: CardInteractionState,
    pub active_instance_id: Option<CardInstanceId>,
    pub source_zone: Option<CardZoneModel>,
    pub target_zone: Option<CardZoneModel>,
}

impl Default for CardInteractionModel {
    fn default() -> Self {
        Self {
            state: CardInteractionState::Idle,
            active_instance_id: None,
            source_zone: None,
            target_zone: None,
        }
    }
}

impl CardInteractionModel {
    pub fn active(
        state: CardInteractionState,
        instance_id: CardInstanceId,
        source_zone: CardZoneModel,
    ) -> Self {
        Self {
            state,
            active_instance_id: Some(instance_id),
            source_zone: Some(source_zone),
            target_zone: None,
        }
    }

    pub const fn is_active(&self) -> bool {
        !matches!(self.state, CardInteractionState::Idle) && self.active_instance_id.is_some()
    }

    pub fn validate_for_card(
        &self,
        card: &CardInstanceStateModel,
    ) -> Result<(), CardStateValidationError> {
        if self.active_instance_id != Some(card.instance_id) {
            return Err(CardStateValidationError::MissingInteractionInstance {
                instance_id: self.active_instance_id,
            });
        }
        match self.state {
            CardInteractionState::Idle => Ok(()),
            CardInteractionState::Dragging if !card.zone.is_draggable_source() => {
                Err(CardStateValidationError::IllegalInteractionForZone {
                    instance_id: card.instance_id,
                    state: self.state,
                    zone: card.zone,
                })
            }
            CardInteractionState::Pressed | CardInteractionState::SelectedInspecting
                if !card.zone.is_selectable_source() =>
            {
                Err(CardStateValidationError::IllegalInteractionForZone {
                    instance_id: card.instance_id,
                    state: self.state,
                    zone: card.zone,
                })
            }
            _ => Ok(()),
        }
    }

    fn pose_override(&self) -> CardViewPoseModel {
        match self.state {
            CardInteractionState::Idle => CardViewPoseModel::Hand,
            CardInteractionState::Pressed => self
                .source_zone
                .map(pose_for_zone)
                .unwrap_or(CardViewPoseModel::Hand),
            CardInteractionState::SelectedInspecting => CardViewPoseModel::SelectedInspection,
            CardInteractionState::Dragging => CardViewPoseModel::DragPreview,
            CardInteractionState::Returning => CardViewPoseModel::ReturnTarget,
            CardInteractionState::SettlingPlaced => CardViewPoseModel::SettlingPlaced,
        }
    }
}

/// HUMAN: Query-friendly collection of card instance states for one active match.
/// AI: Use helpers as the lookup/index strategy before replacing existing hand-index state.
#[derive(Resource, Clone, Debug, Default, Eq, PartialEq)]
pub struct CardInstanceStateCollectionModel {
    pub cards: Vec<CardInstanceStateModel>,
}

impl CardInstanceStateCollectionModel {
    pub fn new(cards: Vec<CardInstanceStateModel>) -> Self {
        Self { cards }
    }

    pub fn validate(&self) -> Result<(), CardStateValidationError> {
        for (index, card) in self.cards.iter().enumerate() {
            card.validate()?;
            if self.cards[index + 1..]
                .iter()
                .any(|other| other.instance_id == card.instance_id)
            {
                return Err(CardStateValidationError::DuplicateInstanceId {
                    instance_id: card.instance_id,
                });
            }
        }
        Ok(())
    }

    pub fn by_instance_id(&self, instance_id: CardInstanceId) -> Option<&CardInstanceStateModel> {
        self.cards
            .iter()
            .find(|card| card.instance_id == instance_id)
    }

    pub fn by_owner(&self, owner: CardOwnerModel) -> Vec<&CardInstanceStateModel> {
        self.cards
            .iter()
            .filter(|card| card.owner == owner)
            .collect()
    }

    pub fn by_zone_kind(&self, zone_kind: CardZoneKind) -> Vec<&CardInstanceStateModel> {
        self.cards
            .iter()
            .filter(|card| CardZoneKind::from(card.zone) == zone_kind)
            .collect()
    }

    pub fn at_slot(
        &self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
    ) -> Option<&CardInstanceStateModel> {
        self.cards
            .iter()
            .find(|card| card.zone.slot_identity() == Some((location_index, side, slot_index)))
    }

    pub fn validate_slot_occupancy(
        &self,
        placement: &CardPlacementModel,
    ) -> Result<(), CardStateValidationError> {
        let Some(card) = self.by_instance_id(placement.instance_id) else {
            return Err(CardStateValidationError::MissingPlacedInstance {
                instance_id: placement.instance_id,
            });
        };
        if card.zone.slot_identity()
            != Some((
                placement.location_index,
                placement.side,
                placement.slot_index,
            ))
        {
            return Err(CardStateValidationError::SlotMismatch {
                instance_id: placement.instance_id,
            });
        }
        Ok(())
    }
}

/// HUMAN: Coarse zone category for instance lookups.
/// AI: Keep indexes independent from zone details such as order or slot identity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardZoneKind {
    Deck,
    Hand,
    Location,
    OutOfPlay,
}

impl From<CardZoneModel> for CardZoneKind {
    fn from(value: CardZoneModel) -> Self {
        match value {
            CardZoneModel::Deck { .. } => Self::Deck,
            CardZoneModel::Hand { .. } => Self::Hand,
            CardZoneModel::Location { .. } => Self::Location,
            CardZoneModel::OutOfPlay => Self::OutOfPlay,
        }
    }
}

/// HUMAN: Location-slot placement identity for one card instance.
/// AI: Use this to validate slot occupancy while CardSlotBoardModel still stores hand_index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardPlacementModel {
    pub instance_id: CardInstanceId,
    pub location_index: usize,
    pub side: CardSlotSide,
    pub slot_index: usize,
    pub placed_round: u8,
}

/// HUMAN: Validation failures for proposed card instance state.
/// AI: Prefer explicit variants so tests can pin illegal state combinations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardStateValidationError {
    DuplicateInstanceId {
        instance_id: CardInstanceId,
    },
    MissingCardModelId {
        instance_id: CardInstanceId,
    },
    HiddenCardOutsideLocation {
        instance_id: CardInstanceId,
    },
    IllegalInteractionForZone {
        instance_id: CardInstanceId,
        state: CardInteractionState,
        zone: CardZoneModel,
    },
    MissingInteractionInstance {
        instance_id: Option<CardInstanceId>,
    },
    MissingPlacedInstance {
        instance_id: CardInstanceId,
    },
    SlotMismatch {
        instance_id: CardInstanceId,
    },
}

pub fn local_instances_from_existing_state(
    hand_model: &GameHandModel,
    card_states: &CardStateModel,
    slot_board: &CardSlotBoardModel,
) -> CardInstanceStateCollectionModel {
    let owner = CardOwnerModel::near();
    let hand_order = card_states.indices_with_state(CardState::Hand);
    let cards = hand_model
        .cards
        .iter()
        .enumerate()
        .map(|(hand_index, card_id)| {
            let instance_id = CardInstanceId::from_owner_index(owner, hand_index);
            let zone = local_zone_from_existing_state(hand_index, card_states, slot_board)
                .unwrap_or_else(|| {
                    let order_index = hand_order
                        .iter()
                        .position(|ordered_index| *ordered_index == hand_index)
                        .unwrap_or(hand_index);
                    CardZoneModel::Hand { order_index }
                });
            CardInstanceStateModel::new(
                instance_id,
                card_id.clone(),
                owner,
                zone,
                CardRevealPolicy::OwnerVisible,
            )
        })
        .collect();
    CardInstanceStateCollectionModel::new(cards)
}

pub fn instance_from_cpu_hand_view(view: &CpuHandCardView) -> CardInstanceStateModel {
    let owner = CardOwnerModel::new(view.owner);
    CardInstanceStateModel::new(
        CardInstanceId::from_owner_index(owner, view.hand_index),
        view.card_id.clone(),
        owner,
        CardZoneModel::Hand {
            order_index: view.hand_index,
        },
        CardRevealPolicy::OwnerVisible,
    )
}

pub fn instance_from_cpu_placed_view(
    view: &CpuPlacedCardView,
    placement_visibility: Option<PlacementVisibility>,
) -> CardInstanceStateModel {
    let owner = CardOwnerModel::new(view.owner);
    let reveal_policy = placement_visibility
        .map(CardRevealPolicy::from_placement_visibility)
        .unwrap_or(CardRevealPolicy::RevealedToAll);
    let lock_state = match placement_visibility {
        Some(PlacementVisibility::CurrentRoundHidden) => LocationLockState::CurrentRoundMovable,
        _ => LocationLockState::Locked,
    };
    CardInstanceStateModel::new(
        CardInstanceId::from_owner_index(owner, view.slot_index),
        view.card_id.clone(),
        owner,
        CardZoneModel::Location {
            location_index: view.location_index,
            side: view.side,
            slot_index: view.slot_index,
            lock_state,
        },
        reveal_policy,
    )
}

pub fn reveal_policy_from_placement(placement: &PlacementVisibilityModel) -> CardRevealPolicy {
    CardRevealPolicy::from_placement_visibility(placement.visibility)
}

fn local_zone_from_existing_state(
    hand_index: usize,
    card_states: &CardStateModel,
    slot_board: &CardSlotBoardModel,
) -> Option<CardZoneModel> {
    match card_states.state(hand_index)? {
        CardState::Hand | CardState::Dragging => None,
        CardState::Location | CardState::LocationLocked => {
            let (location_index, slot_index) = slot_board.local_slot_for_card(hand_index)?;
            Some(CardZoneModel::Location {
                location_index,
                side: CardSlotSide::LocalPlayer,
                slot_index,
                lock_state: match card_states.state(hand_index)? {
                    CardState::Location => LocationLockState::CurrentRoundMovable,
                    CardState::LocationLocked => LocationLockState::Locked,
                    _ => return None,
                },
            })
        }
    }
}

fn pose_for_zone(zone: CardZoneModel) -> CardViewPoseModel {
    match zone {
        CardZoneModel::Deck { .. } => CardViewPoseModel::Deck,
        CardZoneModel::Hand { .. } => CardViewPoseModel::Hand,
        CardZoneModel::Location { .. } => CardViewPoseModel::LocationSlot,
        CardZoneModel::OutOfPlay => CardViewPoseModel::OutOfPlay,
    }
}

fn z_band_for_pose(pose: CardViewPoseModel) -> CardViewZBand {
    match pose {
        CardViewPoseModel::Deck => CardViewZBand::Deck,
        CardViewPoseModel::Hand | CardViewPoseModel::ReturnTarget => CardViewZBand::Hand,
        CardViewPoseModel::LocationSlot | CardViewPoseModel::SettlingPlaced => {
            CardViewZBand::LocationSlot
        }
        CardViewPoseModel::SelectedInspection => CardViewZBand::Selected,
        CardViewPoseModel::DragPreview => CardViewZBand::Drag,
        CardViewPoseModel::OutOfPlay => CardViewZBand::Hidden,
    }
}

fn input_affordance_for(
    card: &CardInstanceStateModel,
    viewer: MatchPlayerSide,
    interaction: Option<&CardInteractionModel>,
) -> CardInputAffordance {
    if interaction.is_some_and(CardInteractionModel::is_active) {
        return CardInputAffordance::None;
    }
    if viewer != card.owner.side {
        return CardInputAffordance::Passive;
    }
    if card.zone.is_draggable_source() {
        CardInputAffordance::Draggable
    } else if card.zone.is_selectable_source() {
        CardInputAffordance::Selectable
    } else {
        CardInputAffordance::None
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/card_instance_state_model_tests.rs"]
mod card_instance_state_model_tests;
