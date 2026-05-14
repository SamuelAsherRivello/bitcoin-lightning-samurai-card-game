use bevy::prelude::*;

use crate::runtime::resources::{DeckEditableZoneModel, DeckEditorTabModel};

/// HUMAN: Root marker for DeckScreen-specific UI content.
/// AI: Rebuild this root when DeckScreenModel changes mode, tab, or modal state.
#[derive(Component, Debug, Default)]
pub struct DeckScreenContentRoot;

/// HUMAN: View marker for a rendered deck tile.
/// AI: DeckViewBundle renders the existing card back plus deck name.
#[derive(Component, Debug)]
pub struct DeckView {
    pub deck_name: String,
}

impl DeckView {
    pub fn new(deck_name: impl Into<String>) -> Self {
        Self {
            deck_name: deck_name.into(),
        }
    }
}

/// HUMAN: Selectable button for the single Deck 01 deck tile.
/// AI: New Deck is visual-only for this feature.
#[derive(Component, Debug, Default)]
pub struct DeckScreenDeckTileButton;

/// HUMAN: Selectable DeckScreen card tile.
/// AI: Source zone and index determine modal action enablement.
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct DeckScreenCardTileButton {
    pub card_id: String,
    pub zone: DeckEditableZoneModel,
    pub index: usize,
}

impl DeckScreenCardTileButton {
    pub fn new(card_id: impl Into<String>, zone: DeckEditableZoneModel, index: usize) -> Self {
        Self {
            card_id: card_id.into(),
            zone,
            index,
        }
    }
}

/// HUMAN: World-space card view rendered inside a DeckScreen editor grid.
/// AI: Zone metadata lets selection and future editor actions identify the source card.
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct DeckScreenCardView {
    pub card_id: String,
    pub zone: DeckEditableZoneModel,
    pub index: usize,
}

impl DeckScreenCardView {
    pub fn new(card_id: impl Into<String>, zone: DeckEditableZoneModel, index: usize) -> Self {
        Self {
            card_id: card_id.into(),
            zone,
            index,
        }
    }
}

/// HUMAN: World-space background or border behind a DeckScreen card grid.
/// AI: Keep this below CardViewBundle roots so panel frames never cover selected cards.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeckScreenGridBackdrop {
    pub zone: DeckEditableZoneModel,
    pub role: DeckScreenGridBackdropRole,
}

impl DeckScreenGridBackdrop {
    pub const fn new(zone: DeckEditableZoneModel, role: DeckScreenGridBackdropRole) -> Self {
        Self { zone, role }
    }
}

/// HUMAN: Specific rectangle role for a DeckScreen card grid backdrop.
/// AI: Tests use this to verify deck and library panel boundaries match.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckScreenGridBackdropRole {
    Fill,
    Top,
    Bottom,
    Left,
    Right,
}

/// HUMAN: Button for switching DeckScreen available-card tabs.
/// AI: Shop renders empty until a future purchase feature.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeckScreenTabButton {
    pub tab: DeckEditorTabModel,
}

impl DeckScreenTabButton {
    pub const fn new(tab: DeckEditorTabModel) -> Self {
        Self { tab }
    }
}

/// HUMAN: Button action in the DeckScreen card modal.
/// AI: TransferOut stays disabled in this feature.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckScreenModalActionButton {
    MoveToDeck,
    MoveToLibrary,
    TransferOut,
    Back,
}

/// HUMAN: Root marker for the DeckScreen fullscreen modal.
/// AI: Presence means lower DeckScreen and top-nav input is blocked.
#[derive(Component, Debug, Default)]
pub struct DeckScreenModalRoot;
