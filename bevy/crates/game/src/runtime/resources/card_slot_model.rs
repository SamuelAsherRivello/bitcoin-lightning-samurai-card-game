use bevy::prelude::*;

pub const CARD_SLOT_LOCATION_COUNT: usize = 3;
pub const CARD_SLOT_ROW_COUNT: usize = 4;
pub const CARD_SLOT_TOTAL_COUNT: usize = CARD_SLOT_LOCATION_COUNT * CARD_SLOT_ROW_COUNT * 2;
pub const CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT: usize =
    CARD_SLOT_LOCATION_COUNT * CARD_SLOT_ROW_COUNT;
pub const CARD_SLOT_GAME_VIEW_WIDTH: f32 = 92.0;
pub const CARD_SLOT_GAME_VIEW_HEIGHT: f32 = 90.0;
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
/// AI: Gesture code may only start drags from Hand; Location cards stay on board.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CardState {
    #[default]
    Hand,
    Dragging,
    Location,
}

/// HUMAN: Aspect-ratio-safe GameView rectangle for a location card slot.
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardSlotState {
    Empty,
    Populated { hand_index: usize },
}

impl CardSlotState {
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// HUMAN: One card placement slot near a shared location.
/// AI: Keep legality deterministic for tests and separate from rendered slot entities.
#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub const fn accepts_local_direct_placement(self) -> bool {
        matches!(self.side, CardSlotSide::LocalPlayer) && self.state.is_empty()
    }
}

/// HUMAN: Runtime board-slot state for the three shared locations.
/// AI: Use this for gesture legality before later gameplay turn rules are introduced.
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

    pub fn local_slots_area_hit_target(&self, game_view_position: Vec2) -> Option<usize> {
        (0..CARD_SLOT_LOCATION_COUNT).find(|location_index| {
            self.local_slots_area_rect(*location_index)
                .is_some_and(|rect| rect.contains(game_view_position))
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

    pub fn place_local(
        &mut self,
        location_index: usize,
        side: CardSlotSide,
        slot_index: usize,
        hand_index: usize,
    ) -> bool {
        if !self.can_place_local(location_index, side, slot_index) {
            return false;
        }

        if let Some(slot) = self.slots.iter_mut().find(|slot| {
            slot.location_index == location_index
                && slot.side == side
                && slot.slot_index == slot_index
        }) {
            slot.state = CardSlotState::Populated { hand_index };
            return true;
        }

        false
    }

    pub fn next_available_local_slot(&self, location_index: usize) -> Option<usize> {
        (0..CARD_SLOT_ROW_COUNT).find(|slot_index| {
            self.can_place_local(location_index, CardSlotSide::LocalPlayer, *slot_index)
        })
    }

    pub fn place_next_local(&mut self, location_index: usize, hand_index: usize) -> Option<usize> {
        let slot_index = self.next_available_local_slot(location_index)?;
        self.place_local(
            location_index,
            CardSlotSide::LocalPlayer,
            slot_index,
            hand_index,
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
}

/// HUMAN: Runtime gameplay state for hand cards as they move between hand and board.
/// AI: This is intentionally small until full deck/turn models exist.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct CardStateModel {
    states: Vec<CardState>,
}

impl Default for CardStateModel {
    fn default() -> Self {
        Self {
            states: vec![CardState::Hand; super::STARTING_HAND_CARD_COUNT],
        }
    }
}

impl CardStateModel {
    pub fn with_size(card_count: usize) -> Self {
        Self {
            states: vec![CardState::Hand; card_count],
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn reset_to_size(&mut self, card_count: usize) {
        *self = Self::with_size(card_count);
    }

    pub fn state(&self, hand_index: usize) -> Option<CardState> {
        self.states.get(hand_index).copied()
    }

    pub fn is_draggable(&self, hand_index: usize) -> bool {
        self.state(hand_index) == Some(CardState::Hand)
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

    pub fn place_in_location(&mut self, hand_index: usize) -> bool {
        self.set_state(hand_index, CardState::Location)
    }

    fn set_state(&mut self, hand_index: usize, state: CardState) -> bool {
        let Some(card_state) = self.states.get_mut(hand_index) else {
            return false;
        };
        *card_state = state;
        true
    }
}

pub fn card_slot_rect(
    location_index: usize,
    side: CardSlotSide,
    slot_index: usize,
) -> CardSlotRect {
    let location_left = card_slot_location_left(location_index).unwrap_or(CARD_SLOT_LOCATION_LEFTS[0]);
    let column = slot_index % 2;
    let row = slot_index / 2;
    let top = match side {
        CardSlotSide::LocalPlayer => CARD_SLOT_LOCAL_TOP_Y,
        CardSlotSide::Opponent => CARD_SLOT_OPPONENT_TOP_Y,
    };

    CardSlotRect::new(
        location_left + column as f32 * CARD_SLOT_COLUMN_OFFSET,
        top + row as f32 * CARD_SLOT_ROW_OFFSET,
        CARD_SLOT_GAME_VIEW_WIDTH,
        CARD_SLOT_GAME_VIEW_HEIGHT,
    )
}

fn card_slot_location_left(location_index: usize) -> Option<f32> {
    CARD_SLOT_LOCATION_LEFTS.get(location_index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_has_three_locations_and_twenty_four_slots() {
        let board = CardSlotBoardModel::default();

        assert_eq!(board.slot_count(), CARD_SLOT_TOTAL_COUNT);
        assert_eq!(
            board.local_direct_placement_count(),
            CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT
        );
        for location_index in 0..CARD_SLOT_LOCATION_COUNT {
            for side in [CardSlotSide::Opponent, CardSlotSide::LocalPlayer] {
                for slot_index in 0..CARD_SLOT_ROW_COUNT {
                    assert!(board.slot(location_index, side, slot_index).is_some());
                }
            }
        }
    }

    #[test]
    fn slot_rects_match_debug_drawn_reference_lines() {
        let board = CardSlotBoardModel::default();

        assert_eq!(
            board.slot_rect(1, CardSlotSide::Opponent, 0),
            Some(CardSlotRect::new(548.0, 44.0, 92.0, 90.0))
        );
        assert_eq!(
            board.slot_rect(1, CardSlotSide::Opponent, 3),
            Some(CardSlotRect::new(640.0, 134.0, 92.0, 90.0))
        );
        assert_eq!(
            board.slot_rect(2, CardSlotSide::LocalPlayer, 3),
            Some(CardSlotRect::new(824.0, 522.0, 92.0, 90.0))
        );
        assert_eq!(
            board.local_slots_area_rect(0),
            Some(CardSlotRect::new(364.0, 432.0, 184.0, 180.0))
        );
        assert_eq!(
            board.location_area_rect(0),
            Some(CardSlotRect::new(364.0, 224.0, 184.0, 208.0))
        );
        assert_eq!(
            board.location_area_rect(1),
            Some(CardSlotRect::new(548.0, 224.0, 184.0, 208.0))
        );
        assert_eq!(
            board.location_area_rect(2),
            Some(CardSlotRect::new(732.0, 224.0, 184.0, 208.0))
        );
        assert_eq!(board.location_area_rect(99), None);
    }

    #[test]
    fn only_empty_local_slots_accept_direct_placement() {
        let mut board = CardSlotBoardModel::default();

        assert!(board.can_place_local(0, CardSlotSide::LocalPlayer, 0));
        assert!(!board.can_place_local(0, CardSlotSide::Opponent, 0));
        assert!(board.place_local(0, CardSlotSide::LocalPlayer, 0, 2));
        assert!(!board.can_place_local(0, CardSlotSide::LocalPlayer, 0));
        assert_eq!(board.populated_count(), 1);
    }

    #[test]
    fn valid_local_placement_covers_all_twelve_local_slots() {
        let mut board = CardSlotBoardModel::default();
        let mut placed = 0;

        for location_index in 0..CARD_SLOT_LOCATION_COUNT {
            for slot_index in 0..CARD_SLOT_ROW_COUNT {
                assert!(board.place_local(
                    location_index,
                    CardSlotSide::LocalPlayer,
                    slot_index,
                    placed
                ));
                placed += 1;
            }
        }

        assert_eq!(placed, CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT);
        assert_eq!(
            board.populated_count(),
            CARD_SLOT_LOCAL_DIRECT_PLACEMENT_COUNT
        );
    }

    #[test]
    fn next_available_local_slot_uses_upper_left_upper_right_lower_left_lower_right_order() {
        let mut board = CardSlotBoardModel::default();

        assert_eq!(board.next_available_local_slot(1), Some(0));
        assert_eq!(board.place_next_local(1, 10), Some(0));
        assert_eq!(board.place_next_local(1, 11), Some(1));
        assert_eq!(board.place_next_local(1, 12), Some(2));
        assert_eq!(board.place_next_local(1, 13), Some(3));
        assert_eq!(board.next_available_local_slot(1), None);
        assert!(!board.location_has_available_local_slot(1));
    }

    #[test]
    fn opponent_populated_and_missing_slots_reject_placement() {
        let mut board = CardSlotBoardModel::default();

        assert!(!board.place_local(0, CardSlotSide::Opponent, 0, 0));
        assert!(board.place_local(0, CardSlotSide::LocalPlayer, 0, 0));
        assert!(!board.place_local(0, CardSlotSide::LocalPlayer, 0, 1));
        assert!(!board.place_local(99, CardSlotSide::LocalPlayer, 0, 1));
        assert_eq!(board.populated_count(), 1);
    }

    #[test]
    fn card_state_allows_drag_only_from_hand() {
        let mut states = CardStateModel::default();

        assert!(states.is_draggable(0));
        assert!(states.begin_drag(0));
        assert_eq!(states.state(0), Some(CardState::Dragging));
        assert!(!states.is_draggable(0));
        assert!(!states.begin_drag(0));
        assert!(states.return_to_hand(0));
        assert!(states.is_draggable(0));
        assert!(states.place_in_location(0));
        assert_eq!(states.state(0), Some(CardState::Location));
        assert!(!states.is_draggable(0));
    }
}
