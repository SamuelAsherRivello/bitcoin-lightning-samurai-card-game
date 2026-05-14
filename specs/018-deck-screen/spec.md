# Feature Specification: DeckScreen Meta Game UI

**Feature Branch**: `018-deck-screen`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: Implement the DeckScreen mockups from `documentation/mockups/meta-game-mockups.html` only, including the top navigation shown there. The top navigation should be designed as reusable for other pages, but this feature mounts it only on DeckScreen.

## User Scenarios & Testing

### User Story 1 - Navigate With Shared Top Navigation (Priority: P1)

As a player, I need a top navigation bar on DeckScreen so I can see where I am and later use the same navigation pattern on other screens.

**Why this priority**: The mockups show top navigation as the persistent screen-level entry point, and DeckScreen must identify `My Decks` as selected.

**Independent Test**: Open DeckScreen and verify the top nav shows `Play Game`, `My Decks`, `Settings`, and `Debug`, with `My Decks` selected.

**Acceptance Scenarios**:

1. **Given** DeckScreen is active, **When** the screen renders, **Then** the top navigation appears in the safe area with `My Decks` selected.
2. **Given** the top navigation is rendered, **When** a disabled or unimplemented destination is activated, **Then** DeckScreen remains stable and no deck data changes.

---

### User Story 2 - Browse Decks (Priority: P1)

As a player, I need a DeckScreen deck-selection state so I can see my decks, create a new deck entry, and open an existing deck for editing.

**Why this priority**: This is the DeckScreen entry point after navigation.

**Independent Test**: Open DeckScreen and verify `New Deck` plus the single `Deck 01` deck tile are readable inside the safe area.

**Acceptance Scenarios**:

1. **Given** persisted decks exist, **When** DeckScreen is active, **Then** the deck-selection grid shows `New Deck` plus the `Deck 01` tile.
2. **Given** the user activates an existing deck tile, **When** the selection resolves, **Then** DeckScreen changes to the selected deck editor.

---

### User Story 3 - Edit Selected Deck From Library (Priority: P1)

As a player, I need a selected deck editor with deck cards and available library cards so I can understand what is in my deck and what can be added.

**Why this priority**: This is the main DeckScreen editing workflow.

**Independent Test**: Select `Deck 01` and verify the split editor shows a left `Deck 01` card grid and right `Not In Deck` library grid with `Library` selected.

**Acceptance Scenarios**:

1. **Given** `Deck 01` is selected, **When** the editor opens, **Then** the left grid shows current deck cards and explicit empty slots.
2. **Given** `Deck 01` is selected, **When** the editor opens, **Then** the right grid shows owned library cards not currently in the deck.

---

### User Story 4 - View Shop Offers (Priority: P2)

As a player, I need the `Shop` tab in the editor so I can see cards available for purchase without confusing them with owned library cards.

**Why this priority**: Shop is visible in the mockup but depends on the editor shell.

**Independent Test**: Switch from `Library` to `Shop` and verify the shop is empty while the selected deck stays unchanged.

**Acceptance Scenarios**:

1. **Given** the selected deck editor is open, **When** the user activates `Shop`, **Then** `Shop` is selected and the shop panel shows no cards.

---

### User Story 5 - Inspect And Move A Card In Modal (Priority: P2)

As a player, I need a DeckScreen-specific fullscreen card overlay so I can inspect a card and move it between `Deck 01` and `Library` without interacting with lower UI.

**Why this priority**: The modal is the visible interaction model for card actions and must be separate from gameplay selected-card inspection.

**Independent Test**: Select a deck or library card, verify the lower editor dims, actions are visible, lower UI is blocked, and `Back` closes the modal without changing deck data.

**Acceptance Scenarios**:

1. **Given** a real card tile is selected, **When** the modal opens, **Then** a large card preview appears with `Move To Deck 01`, `Move To Library`, `Transfer Out`, and `Back` actions.
2. **Given** the modal is open, **When** the user activates an enabled move action, **Then** the card appears in exactly one editable zone and visible lists update immediately.

## Edge Cases

| Edge Case | Expected Handling |
| --------- | ----------------- |
| No persisted decks | Show `New Deck`; avoid corrupting player deck collection. |
| Empty selected deck | Show empty-slot visuals. |
| Full selected deck | Disable `Move To Deck 01`. |
| Duplicate card entry | Prevent one persisted entry from appearing in both deck and library. |
| Shop unavailable | Show shop affordances without ownership mutation. |
| Modal open | Block top nav and lower editor activation until modal closes. |
| Resize | Keep top nav, grids, and modal inside the 16:10 safe area. |

## Requirements

| ID | Requirement |
| -- | ----------- |
| FR-001 | DeckScreen MUST be represented as `AppScene` plus `DeckScene`, while user-facing debug/navigation text refers to `DeckScreen`. |
| FR-002 | A reusable top navigation model/view MUST support `Play Game`, `My Decks`, `Settings`, and `Debug` destinations. |
| FR-003 | This feature MUST mount the top navigation only on DeckScreen and mark `My Decks` selected. |
| FR-004 | DeckScreen MUST provide a deck-selection state with `New Deck` and the existing `Deck 01` deck tile. |
| FR-005 | Selecting an existing deck tile MUST open a selected deck editor without leaving DeckScreen. |
| FR-006 | The selected deck editor MUST render a split layout with `Deck 01` containing 12 cards and `Not In Deck` initially empty because all library cards are already present in the deck. |
| FR-007 | The deck grid MUST show exactly 12 real card tiles for `Deck 01`; empty slots are not expected while the deck contains 12 cards. |
| FR-008 | The `Not In Deck` panel MUST support `Library` and `Shop` tab states. |
| FR-009 | Shop view MUST be empty in this feature; shop cards and purchase settlement come later. |
| FR-010 | Selecting a real card tile MUST open a DeckScreen-specific fullscreen card modal, not the gameplay selected-card modal. |
| FR-011 | The modal MUST dim lower content, block lower interactions including top nav, show a large card preview, and show an action rail. |
| FR-012 | Modal actions MUST include `Move To Deck 01`, `Move To Library`, `Transfer Out`, and `Back`; move actions and Back MUST work, while `Transfer Out` MUST remain disabled. |
| FR-013 | Enabled move actions MUST update visible deck/library membership immediately and persist through existing player deck collection storage. |
| FR-014 | DeckScreen MUST not read from or mutate active gameplay deck, hand, slot, or round state. |
| FR-015 | All DeckScreen UI MUST stay inside the aspect-ratio-safe 16:10 area on Windows desktop and browser WebGPU. |
| FR-016 | Automated tests MUST cover top-nav selection, DeckScreen state transitions, membership derivation, modal action enablement, and input blocking where practical. |

## Key Entities

| Entity | Description |
| ------ | ----------- |
| `TopNavigationModel` | Reusable navigation destination and selected-destination state. |
| `TopNavigationDestination` | Destination enum for Play Game, My Decks, Settings, and Debug. |
| `DeckScreenModel` | Screen state: deck selection, selected deck editor, tab, and selected modal card. |
| `DeckSummaryModel` | Display data for one deck tile. |
| `DeckEditableCardModel` | Render-facing editable card entry with zone and ownership/source metadata. |
| `DeckScreenCardModalModel` | Transient modal state for selected card preview and action enablement. |
| `PlayerDeckCollectionModel` | Existing persisted deck collection read and updated by DeckScreen. |

## Success Criteria

| ID | Measurable Outcome |
| -- | ------------------ |
| SC-001 | DeckScreen shows the top nav with `My Decks` selected in one navigation action. |
| SC-002 | A user can open `Deck 01` and distinguish deck cards, empty slots, library cards, and shop cards without overlapping UI. |
| SC-003 | Selecting a real card opens the DeckScreen modal and lower content cannot activate until the modal closes. |
| SC-004 | Moving a card between `Deck 01` and `Library` updates both visible lists immediately and is restored after restart. |

## Assumptions

| Topic | Assumption |
| ----- | ---------- |
| Mockup scope | Only the four `DeckScreen` mockups define DeckScreen behavior; other mockups are ignored. |
| Top navigation scope | Top navigation is designed reusable but mounted only on DeckScreen in this feature. |
| Purchase scope | Shop purchase settlement is out of scope. |
| Transfer scope | `Transfer Out` MUST be visible and disabled until a future transfer feature. |
| Persistence | Existing `PlayerDeckCollectionModel` remains the storage source. |
| Layout | Visible positions derive from the existing aspect-ratio-safe game view. |

## Deck View Constraint

| Requirement | Detail |
| ----------- | ------ |
| DeckViewBundle | Implementation MUST create a DeckViewBundle that renders a deck tile using the existing card back asset and the deck name only. |
