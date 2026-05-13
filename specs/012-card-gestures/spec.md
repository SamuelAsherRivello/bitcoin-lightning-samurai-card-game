# Feature Specification: Card Gestures

**Feature Branch**: `012-card-gestures`  
**Created**: 2026-05-11  
**Status**: Draft  
**Input**: User description: "Card gestures for the game view. Mouse input should also mean mobile pointer input such as tap, press, swipe, and drag. Clicking a hand card should no longer switch to the Deck scene. A click selects the card in the game view, moves it to the center at the same large inspection position used by the Deck scene, around 90% of screen height, and clicking that selected card returns it to its hand position. Dragging a hand card after moving a few pixels should not count as a click. Dragging allows the card to move into empty slots in the local player's side of the three location areas. Each location has four slots above and four below, and the local human player may only allocate into the twelve closest slots below the three locations. Dropping onto an empty valid slot snaps the card there while preserving card aspect ratio and fitting the slot."

## Clarifications

### Session 2026-05-12

- Q: Are placed cards always immovable? -> A: No. Cards placed during the current round may be dragged back to the player hand area and reinserted into the hand during that same round; after the round ends, placed cards become locked and cannot be moved by drag.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Inspect a Hand Card In Place (Priority: P1)

A player can tap or click a card in their hand to inspect it at the center of the game view without leaving the game view. The enlarged card appears in the same center inspection position used by the Deck view, sized to nearly fill the available height while preserving its aspect ratio.

**Why this priority**: The current click behavior exits the game view, which interrupts gameplay. In-place inspection is the minimum gesture change needed before hand cards can become playable.

**Independent Test**: Launch the game view, click or tap one card in the hand area, and verify that the game view remains active while that card animates to the center inspection position at the expected large size.

**Acceptance Scenarios**:

1. **Given** the game view is active and a card is visible in the hand area, **When** the player clicks or taps the card without dragging, **Then** the game view remains active and the card becomes the selected inspected card.
2. **Given** a hand card is selected for inspection, **When** its movement completes, **Then** it is centered in the screen at the same inspection position as the Deck view and is approximately 90% of the safe visible height.
3. **Given** a hand card is selected for inspection, **When** the player clicks or taps that enlarged selected card, **Then** the card returns to its original hand position and hand-sized presentation.
4. **Given** a hand card is clicked or tapped for inspection, **When** the interaction resolves, **Then** the Deck view is not opened and no other user-facing GameScene gesture opens it.

---

### User Story 2 - Distinguish Click From Drag (Priority: P1)

A player can start an input on a hand card and either inspect it with a click/tap or drag it by moving beyond a small movement threshold. Once the movement threshold is crossed, the gesture is treated as a drag and must not also trigger card inspection.

**Why this priority**: The same card surface supports both inspection and play preparation. The game must reliably separate deliberate clicks from drag gestures to prevent accidental mode changes.

**Independent Test**: Press on a hand card, release without meaningful movement, and verify inspection. Repeat while moving beyond the threshold before release and verify the drag path is used without opening inspection.

**Acceptance Scenarios**:

1. **Given** the player presses a hand card and releases within the click movement threshold, **When** the gesture ends, **Then** the gesture is treated as a click or tap.
2. **Given** the player presses a hand card and moves beyond the click movement threshold before release, **When** the gesture continues, **Then** the gesture is treated as a drag, the card grows to 150% of its hand size over 0.25 seconds with ease-out motion, and the original pointer-to-card-center offset remains consistent while dragging.
3. **Given** a gesture has become a drag, **When** the player releases the pointer, **Then** the card inspection behavior is not triggered by that same gesture.
4. **Given** the player uses a touch screen, **When** the player taps, presses, or drags a card, **Then** the same click-versus-drag rules apply as they do for mouse input.
5. **Given** a card is already being pressed or dragged, **When** another pointer press starts on a different hand card, **Then** the second press is ignored until the first pointer gesture resolves.
6. **Given** a card is not in `Hand` state and was not placed during the current round, **When** the player presses or drags it, **Then** the card is not eligible to start a drag gesture.

---

### User Story 3 - Drag a Hand Card to a Valid Local Slot (Priority: P1)

A player can drag a card from the hand area into any empty slot on the local player's side of the three location areas. The card snaps into the chosen slot, shrinks to fit that slot, and preserves its aspect ratio.

**Why this priority**: Dragging cards from hand to local location slots is the first direct gameplay placement gesture.

**Independent Test**: Start with at least one empty local slot under a location, drag a hand card into that slot, and verify that the card is placed there at slot size without leaving the game view.

**Acceptance Scenarios**:

1. **Given** the game view has three location areas with four local-player slots below each location, **When** the player drags a hand card over the local-player slots area for a location that has at least one empty slot and releases, **Then** the card snaps into the next automatically assigned empty slot for that location.
2. **Given** a hand card is dropped into a valid location slots area, **When** placement completes, **Then** the card is sized to fit the assigned slot while maintaining its card aspect ratio.
3. **Given** a card has been placed into a local-player slot, **When** the location area is inspected, **Then** the slot is no longer empty and the card occupies that slot.
4. **Given** a card is placed into a local-player slot, **When** the gesture resolves, **Then** the game view remains active.
5. **Given** a location has at least one empty local-player slot while the player is dragging a hand card, **When** the game view shows placement affordances, **Then** a light blue DropTargetHint rectangle outlines that location's entire local-player slots area.
6. **Given** a drag resolves successfully, **When** the card finishes placement, **Then** the card's final location is the assigned destination location card slot.
7. **Given** one or more cards are assigned to a location card slot for a player side, **When** the game view updates that location, **Then** that side's visible location power points equal the sum of the assigned cards' power values.
8. **Given** a card was placed into a location during the current round, **When** the player drags it back to the player hand area, **Then** it returns to the hand instead of remaining locked at the location.
9. **Given** a current-round placed card is dragged over the hand area, **When** the player moves it between hand cards, **Then** the hand cards shift along the x axis to show an insertion gap to the left, middle, or right of existing cards.
10. **Given** a current-round placed card is released over a valid hand insertion gap, **When** the gesture resolves, **Then** the card becomes a hand card at that chosen hand order and the hand group recenters.
11. **Given** a placed card belongs to a previous round, **When** the player presses or drags it, **Then** it cannot start a drag and remains locked in its location slot.

---

### User Story 4 - Reject Invalid Drag Targets (Priority: P2)

A player cannot place a card into opponent slots, populated slots, or empty space outside valid local-player location slots. Invalid drops do not lose the card; the card returns to its source position.

**Why this priority**: Placement constraints protect the board state from accidental or illegal allocations once drag placement exists.

**Independent Test**: Drag a hand card onto an opponent slot, a populated local slot, and empty board space, then verify that none of those drops creates a placement.

**Acceptance Scenarios**:

1. **Given** a hand card is dragged over one of the twelve opponent-side slots above the locations, **When** the player releases it, **Then** the card is not placed there.
2. **Given** a hand card is dragged over a populated local-player slot, **When** the player releases it, **Then** the card is not placed there.
3. **Given** a hand card is dragged over empty space that is not a valid slot, **When** the player releases it, **Then** the card is not placed there.
4. **Given** a drop target is invalid, **When** the drag resolves, **Then** the card returns to its source position and source size.
5. **Given** a drag is cancelled by an invalid release, **When** the card finishes returning, **Then** the card's final location is its original hand card slot.

### Edge Cases

| Edge Case | Expected Behavior |
| --------- | ----------------- |
| A click begins on one hand card while another card is already selected for inspection | The previous selected card returns to its source position before or while the newly selected card becomes the selected inspected card. |
| The player starts dragging a hand card while another card is selected for inspection | The selected inspected card returns to its source position so the board has a single active card gesture focus. |
| A second hand card press begins while one card is already pressed or dragged | The second press is ignored; only zero or one card can be in a draggable pointer gesture at a time. |
| The player presses a hand card, moves exactly at the click movement threshold, then releases | The boundary is handled consistently by the documented threshold rule and does not trigger both click and drag. |
| The player releases a dragged card between two valid empty local slots | The card snaps only if one slot is clearly selected by the drop target rules; otherwise it returns to its source position. |
| The player releases a dragged card inside a location's local-player slots area with multiple empty slots | The game assigns the first empty slot in this order: upper left, upper right, lower left, lower right. |
| The player releases a dragged card inside a full location's local-player slots area | The location is not a valid drop target and the card returns to its source position. |
| The target local slot becomes populated while the card is being dragged | The drop is rejected and the dragged card returns to its source position. |
| The pointer leaves the visible application area during a drag | The drag remains controlled by the last valid pointer state, and releasing outside a valid slot returns the card to its source position. |
| The window or safe area changes while a card is selected or being dragged | Card positions and sizes are recalculated from the current safe visible area while preserving card aspect ratio. |
| All twelve local-player slots are populated | Dragging from hand cannot place a card until an empty local-player slot exists. |
| A current-round placed card is dragged back over the hand area with no clear insertion gap | The card returns to its current-round location slot and remains part of the current-round move history. |
| The round ends while cards are placed in locations | Those placed cards become locked and cannot start future drag gestures. |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game view MUST treat mouse, touch, stylus, and equivalent screen interactions as pointer input for card gestures.
- **FR-002**: The game view MUST support card gestures for cards visible in the local player's hand area.
- **FR-003**: Clicking or tapping a local hand card without crossing the drag movement threshold MUST select that card for inspection in the game view.
- **FR-004**: Selecting a hand card for inspection MUST NOT switch to, open, or otherwise activate the Deck view.
- **FR-005**: A selected inspected card MUST move to the center inspection position used by the Deck view while the game view remains active.
- **FR-006**: A selected inspected card MUST be approximately 90% of the current safe visible height, unless constrained by width, while preserving card aspect ratio.
- **FR-007**: Clicking or tapping the selected inspected card MUST return it to its source hand position and hand-sized presentation.
- **FR-008**: Card movement between hand, selected inspection, drag, and slot positions MUST be animated smoothly rather than teleporting.
- **FR-009**: The system MUST define a movement threshold that separates click or tap gestures from drag gestures.
- **FR-010**: Once pointer movement crosses the drag threshold for a hand card, that gesture MUST be treated as a drag and MUST NOT also trigger card inspection.
- **FR-011**: Dragging a hand card MUST let the player preview movement over the game view while preserving card aspect ratio and preserving the pointer-to-card-center offset from the start of the drag.
- **FR-012**: The board MUST present three location areas from left to right.
- **FR-013**: Each location area MUST have four opponent-side slots above the location and four local-player slots below the location.
- **FR-014**: The local human player MUST be able to place cards only into the twelve local-player slots below the three locations.
- **FR-015**: The local human player MUST NOT be able to place hand cards into the twelve opponent-side slots above the locations by direct drag.
- **FR-016**: A local-player slot MUST have an empty or populated state.
- **FR-017**: A dragged hand card released over an empty local-player slot MUST snap into that slot.
- **FR-018**: A dragged hand card released inside a location's local-player slots area MUST be assigned to the first empty slot in this order: upper left, upper right, lower left, lower right.
- **FR-019**: A card snapped into a local-player slot MUST mark that slot as populated.
- **FR-020**: A dragged hand card released over a populated local-player slot MUST NOT be placed there.
- **FR-021**: A dragged hand card released outside a valid empty local-player slot MUST return to its source position and source size.
- **FR-022**: A card gesture MUST maintain a single clear active card focus so click, selected inspection, and drag states do not conflict.
- **FR-023**: Card gestures MUST remain inside the aspect-ratio safe game view area for visible hand cards, selected inspection, and local slot placement.
- **FR-024**: This feature MUST NOT require full round rules, energy costs, CPU placement, opponent interaction, card reveal rules, scoring changes, or completed gameplay resolution.
- **FR-025**: This feature MUST leave the existing Deck view implementation unchanged, even though users will no longer have a GameScene gesture path to reach it after this feature.
- **FR-026**: When a hand-card drag begins, the card MUST animate to 150% of its hand size over 0.25 seconds with ease-out motion.
- **FR-027**: While dragging, each location with at least one empty local-player slot MUST show a user-facing light blue DropTargetHint rectangle over its entire local-player slots area.
- **FR-028**: A full location's local-player slots area MUST NOT show as an available DropTargetHint and MUST reject drops.
- **FR-029**: The game MUST guard against concurrent hand-card drag gestures so only zero or one card can be pressed for drag or actively dragged at a time.
- **FR-030**: After a successful or cancelled drag resolves, the card's final legal destination MUST be either its original hand card slot or the assigned destination location card slot.
- **FR-031**: Each game card MUST have a gameplay card state that distinguishes `Hand`, `Dragging`, current-round placed location cards, and locked location cards.
- **FR-032**: Location card slot position and size data MUST live in the runtime slot data model and be reused by debug drawing, drop target hit testing, and placement transforms.
- **FR-033**: Each visible location-side power total MUST be derived from the cards currently assigned to that location's player-side card slots.
- **FR-034**: A location card placed during the current round MUST be eligible to start a drag back toward the player hand area during that same round.
- **FR-035**: A location card from a previous round MUST NOT be eligible to start a drag gesture.
- **FR-036**: While a current-round placed card is dragged over the player hand area, the hand layout MUST shift existing cards along the x axis to show the current insertion gap.
- **FR-037**: Releasing a current-round placed card over a valid hand insertion gap MUST return it to the hand at that chosen hand order and recenter the hand group.

### Key Entities

| Entity | Description |
| ------ | ----------- |
| **Pointer Input** | A generalized user input source covering mouse click, mouse movement, drag, touch tap, press, swipe, stylus, and equivalent pointer interactions. |
| **Hand Card** | A card visible in the local player's hand area and eligible for click-to-inspect or drag-to-place gestures. |
| **Hand Card Slot** | The original hand position and size assigned to a hand card before its active gesture starts. |
| **Card State** | The runtime gameplay state for a card, including `Hand`, `Dragging`, current-round placed location, and locked location. |
| **Selected Inspected Card** | The one active card enlarged in the center of the game view for in-place inspection. |
| **Gesture Threshold** | The movement boundary used to decide whether a press-and-release is a click or tap, or whether it becomes a drag. |
| **Drag Gesture** | A pointer gesture that begins on a hand card and crosses the movement threshold before release. |
| **Location Area** | One of the three shared board areas arranged from left to right. |
| **Opponent-Side Slot** | One of four slots above a location area; not a valid direct placement target for the local human player in this feature. |
| **Local-Player Slot** | One of four slots below a location area; a valid direct placement target only when empty. |
| **Location Card Slot** | The destination local-player slot assigned after a successful drag release. |
| **Current-Round Placed Card** | A card moved from hand to a location during the current round; it may still be dragged back to the player hand area before End Round locks it. |
| **Locked Placed Card** | A card in a location from a previous round; it cannot be moved by drag. |
| **Hand Insertion Gap** | The temporary visual gap shown between, before, or after hand cards while returning a current-round placed card to the hand. |
| **Slot State** | Whether a slot is empty and can accept a card, or populated and cannot accept another directly dragged card. |
| **Source Position** | The card's position and size before the active gesture began, used when returning from selected inspection or rejected drag. |
| **DropTargetHint** | A user-facing light blue rectangle outlining the full local-player slots area for a location that can currently accept a dragged card. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of click or tap inspection tests on hand cards, the game view remains active and the Deck view is not opened.
- **SC-002**: In 95% of ordinary tap or click attempts on hand cards, reviewers can select and unselect a card without accidentally starting a drag.
- **SC-003**: In 95% of ordinary drag attempts on hand cards, reviewers can drag beyond the threshold without accidentally triggering card inspection.
- **SC-004**: A selected inspected card reaches a centered inspection presentation using approximately 90% of safe visible height within 500 ms of the click or tap in normal test conditions.
- **SC-005**: In placement tests across all twelve local-player slots, a dragged card successfully snaps into each empty slot when released over that slot.
- **SC-006**: In invalid placement tests covering opponent-side slots, populated local-player slots, and empty board space, 100% of invalid drops are rejected without losing the card.
- **SC-007**: In all selected, dragged, and placed presentations, card aspect ratio remains visibly consistent with the source card.
- **SC-008**: A reviewer can identify which of the twenty-four board slots are valid for local human direct placement in under 10 seconds after seeing the board.
- **SC-009**: In current-round return tests, reviewers can drag a card placed this round back to the hand area, choose an insertion gap, and see the hand recenter after release.
- **SC-010**: In prior-round lock tests, reviewers cannot drag a card that remained placed after End Round.

## Assumptions

| Assumption | Rationale |
| ---------- | --------- |
| "Mouse input" in the feature request means generalized pointer input. | The user explicitly said mouse movement and mouse click should also work for mobile tap, press, swipe, and related gestures. |
| The local human player is always seated at the near or bottom side of the screen. | The request states the player is in the near position and that the bottom side is always the human player. |
| The valid direct placement slots are the four bottom slots for each of the three locations, totaling twelve slots. | The request identifies three location areas and four closest slots per location. |
| The center inspection position should match the already approved Deck inspection pose as closely as the game view layout allows. | The request wants the exact same position as the Deck scene while staying in the game view. |
| Invalid drag drops return the card to its source position. | This preserves the card and follows common card-game drag behavior when a target cannot accept a card. |
| Full gameplay legality beyond slot emptiness and local-player ownership is outside this feature. | The request is focused on gestures and slot allocation, not round, cost, reveal, or scoring rules. |
| Desktop-only iteration is acceptable, but the gesture vocabulary must be designed so touch/mobile pointer input can use the same behavior. | The user emphasized mobile-compatible meaning for mouse input without asking for a complete mobile platform release. |
| Deck view behavior remains as-is. | The feature removes user reachability from GameScene hand-card gestures, but does not modify the existing Deck view itself. |
