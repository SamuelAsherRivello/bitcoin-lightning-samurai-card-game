use bevy::prelude::*;

pub const CARD_SLOT_LOCATION_COUNT: usize = 3;
pub const CARD_SLOT_ROW_COUNT: usize = 4;
pub const CARD_SLOT_TOTAL_COUNT: usize = CARD_SLOT_LOCATION_COUNT * CARD_SLOT_ROW_COUNT * 2;
pub const CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT: usize =
    CARD_SLOT_LOCATION_COUNT * CARD_SLOT_ROW_COUNT;
pub const CARD_SLOT_GAME_SCENE_WIDTH: f32 = 92.0;
pub const CARD_SLOT_GAME_SCENE_HEIGHT: f32 = 90.0;
pub const CARD_SLOT_LOCATION_AREA_WIDTH: f32 = 184.0;
pub const CARD_SLOT_LOCATION_AREA_HEIGHT: f32 = 208.0;
const CARD_SLOT_LOCATION_LEFTS: [f32; CARD_SLOT_LOCATION_COUNT] = [364.0, 548.0, 732.0];
const CARD_SLOT_COLUMN_OFFSET: f32 = 92.0;
const CARD_SLOT_OPPONENT_TOP_Y: f32 = 44.0;
const CARD_SLOT_LOCAL_TOP_Y: f32 = 432.0;
const CARD_SLOT_ROW_OFFSET: f32 = 90.0;
const CARD_SLOT_LOCATION_AREA_TOP_Y: f32 = 224.0;

/// HUMAN: Player side for a board slot around a shared location.
/// AI: LocalPlayer slots accept direct human drag placement; Opponent slots do not.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CardSlotSide {
    LocalPlayer,
    Opponent,
}

/// HUMAN: Current gameplay state of one playable card.
/// AI: Gesture code may start drags from Hand and current-round Location; locked cards stay on board.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CardState {
    #[default]
    Hand,
    Dragging,
    Location,
    LocationLocked,
}

/// HUMAN: Aspect-ratio-safe GameScene rectangle for a location card slot.
/// AI: This is shared by debug drawing, drop hit testing, and placement transforms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardSlotRect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

impl CardSlotRect {
    pub const fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    pub fn center(self) -> Vec2 {
        Vec2::new(self.left + self.width * 0.5, self.top + self.height * 0.5)
    }

    pub fn size(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.left
            && point.x <= self.left + self.width
            && point.y >= self.top
            && point.y <= self.top + self.height
    }

    pub fn union(self, other: Self) -> Self {
        let left = self.left.min(other.left);
        let top = self.top.min(other.top);
        let right = (self.left + self.width).max(other.left + other.width);
        let bottom = (self.top + self.height).max(other.top + other.height);

        Self::new(left, top, right - left, bottom - top)
    }
}

/// HUMAN: Occupancy state for one location-area card slot.
/// AI: Store only lightweight card identity here; full card data remains in card models.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CardSlotState {
    Empty,
    Populated { hand_index: usize, card_id: String },
}

impl CardSlotState {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn hand_index(&self) -> Option<usize> {
        match self {
            Self::Empty => None,
            Self::Populated { hand_index, .. } => Some(*hand_index),
        }
    }

    pub fn card_id(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::Populated { card_id, .. } => Some(card_id.as_str()),
        }
    }
}

/// HUMAN: One card placement slot near a shared location.
/// AI: Keep legality deterministic for tests and separate from rendered slot entities.
#[derive(Clone, Debug, PartialEq)]
pub struct CardSlotModel {
    pub location_index: usize,
    pub side: CardSlotSide,
    pub slot_index: usize,
    pub rect: CardSlotRect,
    pub state: CardSlotState,
}

impl CardSlotModel {
    pub fn new(location_index: usize, side: CardSlotSide, slot_index: usize) -> Self {
        Self {
            location_index,
            side,
            slot_index,
            rect: card_slot_rect(location_index, side, slot_index),
            state: CardSlotState::Empty,
        }
    }

    pub fn accepts_local_direct_placement(&self) -> bool {
        matches!(self.side, CardSlotSide::LocalPlayer) && self.state.is_empty()
    }
}

/// HUMAN: Runtime board-slot state for the three shared locations.
/// AI: Use this for gesture legality before later gameplay round rules are introduced.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct CardSlotBoardModel {
    slots: Vec<CardSlotModel>,
}

impl Default for CardSlotBoardModel {
    fn default() -> Self {
        let mut slots = Vec::with_capacity(CARD_SLOT_TOTAL_COUNT);
        for location_index in 0..CARD_SLOT_LOCATION_COUNT {
            for slot_index in 0..CARD_SLOT_ROW_COUNT {
                slots.push(CardSlotModel::new(
                    location_index,
                    CardSlotSide::Opponent,
                    slot_index,
                ));
            }
            for slot_index in 0..CARD_SLOT_ROW_COUNT {
                slots.push(CardSlotModel::new(
                    location_index,
                    CardSlotSide::LocalPlayer,
                    slot_index,
                ));
            }
        }

        Self { slots }
    }
}

impl CardSlotBoardModel {
    pub fn slots(&self) -> impl Iterator<Item = &CardSlotModel> {
        self.slots.iter()
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn slot_rect(
        &self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
    ) -> Option<CardSlotRect> {
        self.slot(location_index, side, slot_index)
            .map(|slot| slot.rect)
    }

    pub fn local_slots_area_rect(&self, location_index: usize) -> Option<CardSlotRect> {
        let mut rects = (0..CARD_SLOT_ROW_COUNT).filter_map(|slot_index| {
            self.slot_rect(location_index, CardSlotSide::LocalPlayer, slot_index)
        });
        let first = rects.next()?;

        Some(rects.fold(first, |area, rect| area.union(rect)))
    }

    /// HUMAN: Runtime rectangle for the shared location area for drag/drop and overlays.
    /// AI: Keep this as a single source for area-dependent systems and debug drawing.
    pub fn location_area_rect(&self, location_index: usize) -> Option<CardSlotRect> {
        let left = card_slot_location_left(location_index)?;
        Some(CardSlotRect::new(
            left,
            CARD_SLOT_LOCATION_AREA_TOP_Y,
            CARD_SLOT_LOCATION_AREA_WIDTH,
            CARD_SLOT_LOCATION_AREA_HEIGHT,
        ))
    }

    pub fn local_slots_area_hit_target(&self, game_scene_position: Vec2) -> Option<usize> {
        (0..CARD_SLOT_LOCATION_COUNT).find(|location_index| {
            self.local_slots_area_rect(*location_index)
                .is_some_and(|rect| rect.contains(game_scene_position))
        })
    }

    pub fn local_direct_placement_count(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| matches!(slot.side, CardSlotSide::LocalPlayer))
            .count()
    }

    pub fn slot(
        &self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
    ) -> Option<&CardSlotModel> {
        self.slots.iter().find(|slot| {
            slot.location_index == location_index
                && slot.side == side
                && slot.slot_index == slot_index
        })
    }

    pub fn can_place_local(
        &self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
    ) -> bool {
        self.slot(location_index, side, slot_index)
            .is_some_and(|slot| slot.accepts_local_direct_placement())
    }

    pub fn can_place_for_side(
        &self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
    ) -> bool {
        self.slot(location_index, side, slot_index)
            .is_some_and(|slot| slot.state.is_empty())
    }

    pub fn place_local(
        &mut self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
        hand_index: usize,
    ) -> bool {
        self.place_local_with_card_id(location_index, side, slot_index, hand_index, "")
    }

    pub fn place_local_with_card_id(
        &mut self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
        hand_index: usize,
        card_id: impl Into<String>,
    ) -> bool {
        let Some(target_index) = self.slots.iter().position(|slot| {
            slot.location_index == location_index
                && slot.side == side
                && slot.slot_index == slot_index
                && slot.accepts_local_direct_placement()
        }) else {
            return false;
        };

        for slot in &mut self.slots {
            if slot.side == CardSlotSide::LocalPlayer && slot.state.hand_index() == Some(hand_index)
            {
                slot.state = CardSlotState::Empty;
            }
        }

        self.slots[target_index].state = CardSlotState::Populated {
            hand_index,
            card_id: card_id.into(),
        };
        true
    }

    pub fn place_for_side_with_card_id(
        &mut self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
        hand_index: usize,
        card_id: impl Into<String>,
    ) -> bool {
        let Some(target_index) = self.slots.iter().position(|slot| {
            slot.location_index == location_index
                && slot.side == side
                && slot.slot_index == slot_index
                && slot.state.is_empty()
        }) else {
            return false;
        };

        for slot in &mut self.slots {
            if slot.side == side && slot.state.hand_index() == Some(hand_index) {
                slot.state = CardSlotState::Empty;
            }
        }

        self.slots[target_index].state = CardSlotState::Populated {
            hand_index,
            card_id: card_id.into(),
        };
        true
    }

    pub fn next_available_slot(&self, location_index: usize, side: CardSlotSide) -> Option<usize> {
        (0..CARD_SLOT_ROW_COUNT)
            .find(|slot_index| self.can_place_for_side(location_index, side, *slot_index))
    }

    pub fn next_available_local_slot(&self, location_index: usize) -> Option<usize> {
        (0..CARD_SLOT_ROW_COUNT).find(|slot_index| {
            self.can_place_local(location_index, CardSlotSide::LocalPlayer, *slot_index)
        })
    }

    pub fn place_next_local(&mut self, location_index: usize, hand_index: usize) -> Option<usize> {
        self.place_next_local_with_card_id(location_index, hand_index, "")
    }

    pub fn place_next_local_with_card_id(
        &mut self,
        location_index: usize,
        hand_index: usize,
        card_id: impl Into<String>,
    ) -> Option<usize> {
        let slot_index = self.next_available_local_slot(location_index)?;
        self.place_local_with_card_id(
            location_index,
            CardSlotSide::LocalPlayer,
            slot_index,
            hand_index,
            card_id,
        )
        .then_some(slot_index)
    }

    pub fn location_has_available_local_slot(&self, location_index: usize) -> bool {
        self.next_available_local_slot(location_index).is_some()
    }

    pub fn populated_count(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| !slot.state.is_empty())
            .count()
    }

    pub fn remove_local_card(&mut self, hand_index: usize) -> Option<(usize, usize)> {
        for slot in &mut self.slots {
            if slot.side == CardSlotSide::LocalPlayer && slot.state.hand_index() == Some(hand_index)
            {
                slot.state = CardSlotState::Empty;
                return Some((slot.location_index, slot.slot_index));
            }
        }

        None
    }

    pub fn local_slot_for_card(&self, hand_index: usize) -> Option<(usize, usize)> {
        self.slots.iter().find_map(|slot| {
            (slot.side == CardSlotSide::LocalPlayer && slot.state.hand_index() == Some(hand_index))
                .then_some((slot.location_index, slot.slot_index))
        })
    }
}

/// HUMAN: Runtime gameplay state for hand cards as they move between hand and board.
/// AI: This is intentionally small until full deck/round models exist.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct CardStateModel {
    states: Vec<CardState>,
    hand_order: Vec<usize>,
}

impl Default for CardStateModel {
    fn default() -> Self {
        let states = vec![CardState::Hand; super::STARTING_HAND_CARD_COUNT];
        Self {
            hand_order: (0..states.len()).collect(),
            states,
        }
    }
}

impl CardStateModel {
    pub fn with_size(card_count: usize) -> Self {
        Self {
            states: vec![CardState::Hand; card_count],
            hand_order: (0..card_count).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn reset_to_size(&mut self, card_count: usize) {
        *self = Self::with_size(card_count);
    }

    pub fn ensure_size(&mut self, card_count: usize) {
        if self.states.len() < card_count {
            let old_len = self.states.len();
            self.states.resize(card_count, CardState::Hand);
            self.hand_order.extend(old_len..card_count);
        }
    }

    pub fn state(&self, hand_index: usize) -> Option<CardState> {
        self.states.get(hand_index).copied()
    }

    pub fn indices_with_state(&self, state: CardState) -> Vec<usize> {
        if state == CardState::Hand {
            return self
                .hand_order
                .iter()
                .copied()
                .filter(|index| self.state(*index) == Some(CardState::Hand))
                .collect();
        }

        self.states
            .iter()
            .enumerate()
            .filter_map(|(index, card_state)| (*card_state == state).then_some(index))
            .collect()
    }

    pub fn hand_index_at_order(&self, order_index: usize) -> Option<usize> {
        self.indices_with_state(CardState::Hand)
            .get(order_index)
            .copied()
    }

    pub fn is_draggable(&self, hand_index: usize) -> bool {
        matches!(
            self.state(hand_index),
            Some(CardState::Hand | CardState::Location)
        )
    }

    pub fn is_selectable(&self, hand_index: usize) -> bool {
        matches!(
            self.state(hand_index),
            Some(CardState::Hand | CardState::Location | CardState::LocationLocked)
        )
    }

    pub fn begin_drag(&mut self, hand_index: usize) -> bool {
        if !self.is_draggable(hand_index) {
            return false;
        }

        self.set_state(hand_index, CardState::Dragging)
    }

    pub fn return_to_hand(&mut self, hand_index: usize) -> bool {
        self.set_state(hand_index, CardState::Hand)
    }

    pub fn return_to_hand_at_order(&mut self, hand_index: usize, order_index: usize) -> bool {
        self.reorder_hand_index(hand_index, order_index);
        self.return_to_hand(hand_index)
    }

    pub fn place_in_location(&mut self, hand_index: usize) -> bool {
        self.set_state(hand_index, CardState::Location)
    }

    pub fn lock_location_cards(&mut self) {
        for state in &mut self.states {
            if *state == CardState::Location {
                *state = CardState::LocationLocked;
            }
        }
    }

    fn set_state(&mut self, hand_index: usize, state: CardState) -> bool {
        let Some(card_state) = self.states.get_mut(hand_index) else {
            return false;
        };
        *card_state = state;
        true
    }

    fn reorder_hand_index(&mut self, hand_index: usize, order_index: usize) {
        self.hand_order.retain(|index| *index != hand_index);
        let index = order_index.min(self.hand_order.len());
        self.hand_order.insert(index, hand_index);
    }
}

pub fn card_slot_rect(
    location_index: usize,
    side: CardSlotSide,
    slot_index: usize,
) -> CardSlotRect {
    let location_left =
        card_slot_location_left(location_index).unwrap_or(CARD_SLOT_LOCATION_LEFTS[0]);
    let column = slot_index % 2;
    let row = slot_index / 2;
    let top = match side {
        CardSlotSide::LocalPlayer => CARD_SLOT_LOCAL_TOP_Y,
        CardSlotSide::Opponent => CARD_SLOT_OPPONENT_TOP_Y,
    };

    CardSlotRect::new(
        location_left + column as f32 * CARD_SLOT_COLUMN_OFFSET,
        top + row as f32 * CARD_SLOT_ROW_OFFSET,
        CARD_SLOT_GAME_SCENE_WIDTH,
        CARD_SLOT_GAME_SCENE_HEIGHT,
    )
}

fn card_slot_location_left(location_index: usize) -> Option<f32> {
    CARD_SLOT_LOCATION_LEFTS.get(location_index).copied()
}

#[cfg(test)]
#[path = "../../tests/runtime/resources/card_slot_model_tests.rs"]
mod card_slot_model_tests;
