use std::collections::HashMap;

use bevy::prelude::Resource;

use crate::runtime::resources::{
    DeckModel, GORO_TAKESHI_CARD_MODEL_ID, KAGE_REN_CARD_MODEL_ID, LORD_DAICHI_CARD_MODEL_ID,
    PlayerDeckCollectionModel, SISTER_HOTARU_CARD_MODEL_ID, YOKAI_PLACEHOLDER_CARD_MODEL_ID,
};

pub const DECK_SCREEN_DECK_NAME: &str = "Deck 01";
pub const DECK_SCREEN_CARD_COUNT: usize = 12;

/// HUMAN: Current DeckScreen presentation mode.
/// AI: Keep this screen-local and separate from ActiveView scene identity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DeckScreenMode {
    #[default]
    DeckSelection,
    Editor,
}

/// HUMAN: Active tab for the DeckScreen available-card panel.
/// AI: Shop intentionally renders empty until a future purchase feature.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DeckEditorTabModel {
    #[default]
    Library,
    Shop,
}

/// HUMAN: Editable source zone for one DeckScreen card entry.
/// AI: Zone drives modal action enablement and persistence mutations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckEditableZoneModel {
    Deck,
    Library,
    Shop,
}

/// HUMAN: Enabled action flags for the DeckScreen card modal.
/// AI: Transfer out remains disabled until a future ownership-transfer feature.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeckScreenModalActionsModel {
    pub move_to_deck: bool,
    pub move_to_library: bool,
    pub transfer_out: bool,
    pub back: bool,
}

/// HUMAN: Modal state for inspecting and moving a DeckScreen card.
/// AI: This is not the gameplay selected-card modal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeckScreenCardModalModel {
    pub card_id: String,
    pub source_zone: DeckEditableZoneModel,
    pub source_index: usize,
    pub actions: DeckScreenModalActionsModel,
}

/// HUMAN: Screen-local state for DeckScreen.
/// AI: Holds meta-game UI state only; do not store active gameplay deck/hand data here.
#[derive(Resource, Clone, Debug, Default, Eq, PartialEq)]
pub struct DeckScreenModel {
    pub mode: DeckScreenMode,
    pub editor_tab: DeckEditorTabModel,
    pub modal: Option<DeckScreenCardModalModel>,
    pub needs_rebuild: bool,
}

impl DeckScreenModel {
    pub fn open_editor(&mut self) {
        self.mode = DeckScreenMode::Editor;
        self.needs_rebuild = true;
    }

    pub fn open_deck_selection(&mut self) {
        self.mode = DeckScreenMode::DeckSelection;
        self.modal = None;
        self.needs_rebuild = true;
    }

    pub fn select_tab(&mut self, tab: DeckEditorTabModel) {
        if self.editor_tab != tab {
            self.editor_tab = tab;
            self.needs_rebuild = true;
        }
    }

    pub fn open_modal(
        &mut self,
        card_id: impl Into<String>,
        source_zone: DeckEditableZoneModel,
        source_index: usize,
        deck_cards: &[String],
    ) {
        let card_id = card_id.into();
        self.modal = Some(DeckScreenCardModalModel {
            actions: modal_actions_for(source_zone, &card_id, deck_cards),
            card_id,
            source_zone,
            source_index,
        });
        self.needs_rebuild = true;
    }

    pub fn close_modal(&mut self) {
        if self.modal.is_some() {
            self.modal = None;
            self.needs_rebuild = true;
        }
    }

    pub fn take_rebuild_request(&mut self) -> bool {
        let should_rebuild = self.needs_rebuild;
        self.needs_rebuild = false;
        should_rebuild
    }
}

pub fn deck_screen_full_card_pool() -> Vec<String> {
    [
        KAGE_REN_CARD_MODEL_ID,
        LORD_DAICHI_CARD_MODEL_ID,
        SISTER_HOTARU_CARD_MODEL_ID,
        YOKAI_PLACEHOLDER_CARD_MODEL_ID,
        GORO_TAKESHI_CARD_MODEL_ID,
    ]
    .into_iter()
    .cycle()
    .take(DECK_SCREEN_CARD_COUNT)
    .map(str::to_string)
    .collect()
}

pub fn deck_screen_deck_cards(collection: &PlayerDeckCollectionModel) -> Vec<String> {
    collection
        .primary_deck()
        .map(|deck| {
            deck.cards
                .iter()
                .take(DECK_SCREEN_CARD_COUNT)
                .cloned()
                .collect()
        })
        .filter(|cards: &Vec<String>| !cards.is_empty())
        .unwrap_or_else(deck_screen_full_card_pool)
}

pub fn deck_screen_library_cards(deck_cards: &[String]) -> Vec<String> {
    let mut remaining_counts = card_counts(deck_cards);
    let mut library_cards = Vec::new();

    for card_id in deck_screen_full_card_pool() {
        match remaining_counts.get_mut(&card_id) {
            Some(count) if *count > 0 => *count -= 1,
            _ => library_cards.push(card_id),
        }
    }

    library_cards
}

pub fn ensure_deck_screen_collection(collection: &mut PlayerDeckCollectionModel) {
    if collection.players.is_empty() {
        collection.players.push(Default::default());
    }
    if collection.players[0].decks.is_empty() {
        collection.players[0]
            .decks
            .push(DeckModel::with_name_and_cards(
                DECK_SCREEN_DECK_NAME,
                deck_screen_full_card_pool(),
            ));
    }
    collection.players[0].decks[0].name = DECK_SCREEN_DECK_NAME.to_string();
    if collection.players[0].decks[0].cards.is_empty() {
        collection.players[0].decks[0].cards = deck_screen_full_card_pool();
    }
    collection.players[0].decks[0]
        .cards
        .truncate(DECK_SCREEN_CARD_COUNT);
}

pub fn move_deck_card_to_library(
    collection: &mut PlayerDeckCollectionModel,
    deck_index: usize,
) -> Option<String> {
    ensure_deck_screen_collection(collection);
    let deck = &mut collection.players[0].decks[0];
    (deck_index < deck.cards.len()).then(|| deck.cards.remove(deck_index))
}

pub fn move_library_card_to_deck(
    collection: &mut PlayerDeckCollectionModel,
    card_id: &str,
) -> bool {
    ensure_deck_screen_collection(collection);
    let deck_cards = collection.players[0].decks[0].cards.clone();
    let library_cards = deck_screen_library_cards(&deck_cards);
    if deck_cards.len() >= DECK_SCREEN_CARD_COUNT || !library_cards.iter().any(|id| id == card_id) {
        return false;
    }

    collection.players[0].decks[0]
        .cards
        .push(card_id.to_string());
    true
}

pub fn modal_actions_for(
    source_zone: DeckEditableZoneModel,
    card_id: &str,
    deck_cards: &[String],
) -> DeckScreenModalActionsModel {
    let library_cards = deck_screen_library_cards(deck_cards);
    DeckScreenModalActionsModel {
        move_to_deck: source_zone == DeckEditableZoneModel::Library
            && deck_cards.len() < DECK_SCREEN_CARD_COUNT
            && library_cards.iter().any(|id| id == card_id),
        move_to_library: source_zone == DeckEditableZoneModel::Deck,
        transfer_out: false,
        back: true,
    }
}

fn card_counts(cards: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_deck_has_empty_library() {
        let deck = deck_screen_full_card_pool();

        assert_eq!(deck.len(), DECK_SCREEN_CARD_COUNT);
        assert!(deck_screen_library_cards(&deck).is_empty());
    }

    #[test]
    fn moving_card_to_library_makes_it_available_to_move_back() {
        let mut collection = PlayerDeckCollectionModel::default();
        ensure_deck_screen_collection(&mut collection);

        let removed = move_deck_card_to_library(&mut collection, 0).unwrap();
        let deck_cards = deck_screen_deck_cards(&collection);
        let library = deck_screen_library_cards(&deck_cards);

        assert_eq!(deck_cards.len(), DECK_SCREEN_CARD_COUNT - 1);
        assert_eq!(library, vec![removed.clone()]);
        assert!(
            modal_actions_for(DeckEditableZoneModel::Library, &removed, &deck_cards).move_to_deck
        );
        assert!(move_library_card_to_deck(&mut collection, &removed));
        assert!(deck_screen_library_cards(&deck_screen_deck_cards(&collection)).is_empty());
    }

    #[test]
    fn transfer_out_is_disabled_and_back_is_enabled() {
        let actions = modal_actions_for(
            DeckEditableZoneModel::Deck,
            KAGE_REN_CARD_MODEL_ID,
            &deck_screen_full_card_pool(),
        );

        assert!(actions.move_to_library);
        assert!(!actions.move_to_deck);
        assert!(!actions.transfer_out);
        assert!(actions.back);
    }
}
