# Feature Specification: Deck List and Game Deck Persistence

**Feature Branch**: `013-card-deck-feature`
**Created**: 2026-05-12
**Status**: Draft
**Input**: User description: "Update the Deck Builder scene to show deck collections and cards, persist deck data, and have GameScene draw and consume cards from a copied player deck per game session."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Manage Deck Builder View Layout (Priority: P1)

A player opens the Deck Builder and sees two clear areas: a left deck list and a right deck contents panel. The deck list includes one selectable deck named `Deck01`, and only deck cards are shown on the right. The existing card UI overlay is not shown in this screen.

**Why this priority**: This is the first visible workflow users touch when preparing decks and it replaces the old single-card layout.

**Independent Test**: Launch Deck Builder from a fresh run and verify the two-area layout renders with one deck button and exactly one set of card visuals on the right for that deck, with no card inspection UI shown.

**Acceptance Scenarios**:

1. **Given** Deck Builder is active, **When** the scene renders, **Then** the left side contains a deck list with a button labeled `Deck01`.
2. **Given** `Deck01` is selected, **When** the deck panel loads, **Then** the right side displays all cards in that deck.
3. **Given** the player opens the Deck Builder scene, **When** they look at the main panel, **Then** no card inspection UI is visible there.

---

### User Story 2 - Build and Persist Default Player Deck (Priority: P1)

A player has one saved deck for `Player 01` named `Deck01`, stored with persistent data. If no deck has been saved yet, the app creates it automatically from a hardcoded base set of 4 card types repeated to make 12 cards in a random order.

**Why this priority**: Gameplay cannot be reliable without a canonical persisted deck source to consume from each game instance.

**Independent Test**: Remove saved deck data and restart. Open Deck Builder; verify `Player 01` has `Deck01` with 12 cards generated from 4 base cards in a randomized order, and confirm changes are preserved after restart.

**Acceptance Scenarios**:

1. **Given** there is no saved deck collection, **When** the app loads, **Then** a default `Player 01` deck collection is created with one deck named `Deck01` and 12 cards.
2. **Given** a player has 12 card entries in `Deck01`, **When** they reopen Deck Builder, **Then** the same deck and card count are loaded from disk.
3. **Given** the deck list is rendered, **When** the same player opens the scene in two separate sessions, **Then** it shows the same persisted deck identity and card count.

---

### User Story 3 - Consume Deck into Game Hand and Reset on New Game (Priority: P1)

When a new game is created, exactly 5 cards are taken from a fresh in-memory copy of `Player 01`'s deck and rendered as the player hand. Cards moved from deck to hand no longer remain in that active game deck. When a new game starts again, the game must be rebuilt from a fresh copy so play always starts from the full persisted deck state.

**Why this priority**: This drives consistent game state and ensures repeated play does not accumulate stale cards in a session.

**Independent Test**: Start a new game, verify 5 cards are drawn and rendered, play one card to hand state transition rules, then restart the game and verify the next game starts from full deck again with a fresh 5-card hand.

**Acceptance Scenarios**:

1. **Given** a game starts, **When** the game deck model is initialized, **Then** it is a copy of `Player 01`'s persisted deck and not a direct reference to the persistent source.
2. **Given** the copied game deck has 12 cards, **When** the game hand is created, **Then** 5 random cards from that copy are rendered into hand and those 5 are removed from the active game deck.
3. **Given** a hand card is moved from deck to hand in the active game, **When** the move completes, **Then** that card is not present in the active game deck.
4. **Given** the player ends or restarts the session, **When** a new game is created again, **Then** it again uses a fresh copy and again starts with 5 cards in hand from a 12-card deck.

### Edge Cases

| Edge Case | Expected Behavior |
| --------- | ---------------- |
| Persisted deck data is missing or unreadable | The app generates a valid default `Player 01` deck with `Deck01` and 12 cards, then saves it. |
| Persisted deck has an unexpected card entry | The feature rejects invalid entries and keeps only valid known card entries, then reports the deck count and re-saves a corrected state. |
| Game starts when `Deck01` is incomplete (fewer than 12 cards) | The game uses the existing card count, fills hand from what is available, and does not create duplicate cards. |
| More than 12 cards are somehow persisted | The game copy uses persisted ordering and still consumes cards from that full list; Deck Builder continues to reflect the full persisted list. |
| User opens Deck Builder during or after an active game | Deck Builder uses the persisted player deck, while active game continues to use the previously copied game deck.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Deck Builder MUST split its main content into a left deck list region and a right deck contents region.
- **FR-002**: The left deck list MUST include a selectable deck named exactly `Deck01` for `Player 01`.
- **FR-003**: The right region MUST show all cards in the selected deck.
- **FR-004**: Deck Builder MUST NOT display the Card UI overlay controls used in the gameplay scene.
- **FR-005**: The player deck model data MUST be represented using names that end with `Model`.
- **FR-006**: The persisted deck collection MUST represent at least one player (`Player 01`) with one deck named `Deck01`.
- **FR-007**: The persisted `Deck01` content MUST contain 12 cards for Player 01 by default.
- **FR-008**: Default deck population MUST use a hardcoded set of 4 known card types and repeat them to create 12 cards.
- **FR-009**: Default deck population order MUST be randomized so the initial card sequence is not deterministic.
- **FR-010**: All deck collection changes MUST be saved to disk and reloaded across runs.
- **FR-011**: On game initialization, the game deck MUST be initialized from a copy of the persisted player deck state.
- **FR-012**: Game setup MUST render five cards in the hand from the active game deck by taking the first five of a randomized sequence.
- **FR-013**: The game deck used for active gameplay MUST mutate when cards are moved to hand; moved cards MUST not remain in the active game deck.
- **FR-014**: Card availability in an active game MUST reflect deck depletion after each hand assignment.
- **FR-015**: Starting a new game MUST reset active game deck consumption and not reuse leftovers from the previous game state.
- **FR-016**: Deck Builder MUST read from the persisted deck collection, not directly from the active game deck.
- **FR-017**: Reordering cards in the deck list UI MUST not change gameplay consumption rules unless explicitly persisted.
- **FR-018**: The feature scope MUST be limited to Player 01 deck list and gameplay deck initialization/consumption; no additional player-management UI is introduced in this feature.

### Key Entities

| Entity | Description |
| ------ | ----------- |
| **Player Deck Collection Model** | Stores all saved deck data by player identity (including Player 01) and supports reload and save behavior. |
| **Player Deck Model** | Represents one deck entry (for example `Deck01`) including deck name and ordered card list. |
| **Deck Card Model** | A persisted card entry in a deck, including a stable card identity from the available card types. |
| **Game Deck Model** | Per-game copy of the player deck used for active gameplay draw and consumption. |
| **Game Hand Model** | Per-game list of cards currently presented as playable hand cards. |
| **Deck List View Model** | The deck-list region state required for selecting a deck and displaying its cards. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of Deck Builder launches after the change, users can see both deck list and deck contents panels, with `Deck01` available in the left panel.
- **SC-002**: In at least 1 automated startup test, when no deck file exists, the system creates and persists a 12-card `Deck01` for Player 01 using 4 card types repeated in random order.
- **SC-003**: In 95% of replayable save/restart checks, Deck Builder shows the same persisted `Deck01` deck identity and card count after relaunch.
- **SC-004**: In all new game starts, the hand must display 5 cards when enough cards exist and the consumed cards must be removed from the active game deck.
- **SC-005**: After card moves to hand, the active deck size must decrease by exactly the number moved and never increase during that game session.
- **SC-006**: In 100% of game restarts for the same persisted deck state, the next game starts from a fresh copy and again provides 5 cards without carrying over previous active-game consumption state.

## Assumptions

- Only one player identity is in scope for this feature: `Player 01`.
- Existing card catalog contains at least 4 known card types suitable for deck population.
- A “random order” means the UI and draw order should not be fixed across fresh generation, but deterministic reproduction is acceptable for testing when persisted state is intentionally frozen.
- Deck persistence is expected at app-level save files already used by the project, and this feature uses that same persistence mechanism.
- Reordering logic in Deck Builder is deferred unless later features explicitly add editing operations beyond default generation and persistence.
- Gameplay scope for this feature ends at deck initialization and deck-to-hand consumption; full deck editing, shuffling controls, and multiplayer deck ownership management are intentionally out of scope.
- The existing camera/background and non-deck-builder runtime behaviors remain unchanged.
