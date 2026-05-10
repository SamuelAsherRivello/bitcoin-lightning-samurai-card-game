# Feature Specification: Gameplay Concepts

**Feature Branch**: `007-gameplay-concepts`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Move broad game architecture context out of 006. The app is a game. The game has a session called a Game and in that Game there are two players. Players may be human or CPU. Each player has locations where cards are placed as well as a hand containing cards that are not placed. All player cards, placed and unplaced, come from that player's deck. Both players have unique decks, but cards come from the same card series. The card series has a specific cardback shared across all. A future Table Top will be where cards sit face up and face down. The current prototype only iterates on 006 for now."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Preserve Gameplay Vocabulary (Priority: P1)

A designer or developer can reference a single future-facing concept document for the game's high-level entities without forcing those systems into the current card flip prototype.

**Why this priority**: The project needs shared language for future gameplay work while keeping current visual prototype scope small.

**Independent Test**: Read this spec and verify it defines the conceptual terms Game, Player, Deck, CardSeries, CardDefinition, CardInstance, hand, placed cards, shared locations, and Table Top without requiring implementation in `006-card-flip`.

**Acceptance Scenarios**:

1. **Given** a future gameplay task references a Game, **When** this spec is consulted, **Then** Game means a two-player session.
2. **Given** a future card task references CardSeries, **When** this spec is consulted, **Then** CardSeries owns the shared card back for its cards.
3. **Given** the current `006-card-flip` prototype is reviewed, **When** this spec is consulted, **Then** these gameplay concepts remain future context rather than required current behavior.

---

### User Story 2 - Describe Future Hidden Card Flow (Priority: P2)

A designer can describe how cards move from deck to hand to shared board locations, including face-down and reveal timing concepts, without implementing those systems yet.

**Why this priority**: Card flipping in `006` is a visual proof of future hidden information and reveal behavior.

**Independent Test**: Review the concept relationships and confirm they describe hidden placement and reveal timing at a high level without specifying turn rules.

**Acceptance Scenarios**:

1. **Given** a card is in a future player's hand, **When** it is placed later, **Then** it may be represented face down before reveal.
2. **Given** a future card is revealed, **When** its front is shown, **Then** the card front comes from its card definition while the back comes from its card series.
3. **Given** future board play is designed, **When** locations are referenced, **Then** they are shared board spaces contested by both players rather than player-owned locations.

### Edge Cases

- If this spec mentions future systems, it should not imply they are part of the current `006-card-flip` implementation.
- If a future feature creates Table Top behavior, it should define the concrete visual and input details then.
- If future card fronts include non-character content, the CardDefinition concept should still apply.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The project MUST use `Game` to mean a future two-player session.
- **FR-002**: A future Game MUST contain exactly two players.
- **FR-003**: A future Player MAY be controlled by a human or CPU.
- **FR-004**: Each future Player MUST have a unique deck.
- **FR-005**: Each future Player MUST have a hand containing that player's unplaced cards.
- **FR-006**: Each future Player MUST have placed cards that originated from that player's deck.
- **FR-007**: Both players' decks MAY contain cards from the same CardSeries.
- **FR-008**: A CardSeries MUST own the shared CardBack used by cards from that series.
- **FR-009**: A CardDefinition MUST represent static card front content and gameplay data such as title, cost, power, artwork, and abilities.
- **FR-010**: A CardInstance MUST represent a runtime copy of a CardDefinition with dynamic match state such as owner, modifiers, reveal status, and location.
- **FR-011**: Future shared board locations MUST be owned by the board/Table Top, not by individual players.
- **FR-012**: Future players MAY place cards into shared board locations.
- **FR-013**: Future hidden placement and reveal timing MUST preserve a clean separation between CardFront, CardBack, gameplay logic, and reveal state.
- **FR-014**: This feature MUST NOT require implementation of gameplay systems, Table Top UI, hand UI, deck browsing, turns, scoring, abilities, CPU behavior, or location control.

### Key Entities

- **Game**: A future two-player match session.
- **Player**: A future participant in a Game, controlled by a human or CPU.
- **Deck**: A future player-owned collection of card instances available during a Game.
- **Hand**: A future player-owned zone containing unplaced cards.
- **Placed Cards**: Future cards a player has committed to board locations.
- **Table Top**: A future board surface where cards can sit face up or face down.
- **Shared Location**: A future board-owned space contested by both players.
- **CardSeries**: A shared card collection identity that owns the common CardBack used across cards in that series.
- **CardDefinition**: Static card data and front content.
- **CardInstance**: A runtime copy of a CardDefinition with owner and match state.
- **CardFront**: The specific visible front content for a CardDefinition.
- **CardBack**: The shared back visual for a CardSeries.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer can identify the future ownership of Game, Player, Deck, Hand, CardSeries, CardDefinition, CardInstance, Table Top, and Shared Location from this spec without reading implementation code.
- **SC-002**: In scope review, this spec introduces no current implementation requirements for `006-card-flip`.
- **SC-003**: In terminology review, CardBack is consistently owned by CardSeries and CardFront is consistently owned by CardDefinition.

## Assumptions

- This spec is a concept-holding feature for future planning, not the active implementation target.
- The current active implementation remains `006-card-flip`.
- The game is inspired by Marvel Snap pacing and structure, but this spec does not clone or define exact Marvel Snap rules.
- Future gameplay specs may refine or replace these concepts when concrete mechanics are designed.
