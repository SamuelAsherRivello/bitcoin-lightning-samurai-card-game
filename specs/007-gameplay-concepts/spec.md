# Feature Specification: Gameplay Concepts

**Feature Branch**: `007-gameplay-concepts`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Move broad game architecture context out of 006. The app is a game. The game has a session called a Game and in that Game there are two players. Players may be human or CPU. Each player has locations where cards are placed as well as a hand containing cards that are not placed. All player cards, placed and unplaced, come from that player's deck. Both players have unique decks, but cards come from the same card series. The card series has a specific cardback shared across all. A future Table Top will be where cards sit face up and face down. The current prototype only iterates on 006 for now."

## Clarifications

### Session 2026-05-10

- Q: Which GameScene details should the concept focus on first? → A: DesertWorld background, three UI locations, local player hand, and TurnUI.
- Q: Which UI framework should GameScene use for the initial HUD and overlays? → A: Built-in Bevy UI.
- Q: What is the scene hierarchy? → A: App has one AppScene, and AppScene dynamically owns either GameScene, DeckBuilderScene, or DebugSettingsScene.
- Q: What dimensional model should GameScene use? → A: GameScene mixes 2D/UI overlays with 3D Card instances rendered in front of the local player hand.
- Q: Which layout orientation should GameScene target first? → A: Landscape first; portrait/mobile later.
- Q: How should subscene lighting be owned? → A: Each dynamic subscene owns one light.

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

---

### User Story 3 - Clarify Prototype Scene Layering (Priority: P2)

A developer can understand how the current prototype separates persistent app-level tools from the active Deck Builder scene while future gameplay concepts are still being defined.

**Why this priority**: The Deck Builder should remain an iterated prototype scene, while app-level debug tools stay available across current and future scenes.

**Independent Test**: Launch or inspect the runtime and verify startup creates a persistent AppScene first, then AppScene opens GameScene by default, with DeckBuilderScene available through the scene shortcut.

**Acceptance Scenarios**:

1. **Given** the app starts, **When** startup systems run, **Then** AppScene is created before GameScene and owns GameScene as its active child scene.
2. **Given** AppScene is active, **When** future scenes are opened or reloaded, **Then** AppScene keeps debug UI and debug support that should persist across scenes.
3. **Given** DeckBuilderScene is active, **When** it is reloaded, **Then** the Deck Builder camera, card, and card-facing presentation entities are replaced without recreating the persistent debug UI.
4. **Given** either GameScene, DeckBuilderScene, or DebugSettingsScene is active, **When** the player presses `S`, **Then** AppScene replaces its active child scene with the next scene.

---

### User Story 4 - Introduce Game Scene Table Top (Priority: P2)

A designer or developer can see the first gameplay-facing scene concept: a world background with three location shapes laid out on top for future round play.

**Why this priority**: GameScene is the first bridge from the Deck Builder prototype toward actual game presentation without implementing full card play yet.

**Independent Test**: Launch or inspect GameScene and verify it has its own camera, a DesertWorld world background that fills the scene view, three visible UI location placeholders, a local player hand area, and TurnUI.

**Acceptance Scenarios**:

1. **Given** GameScene is active, **When** it is rendered, **Then** the world is represented by a full-scene DesertWorld World Background image showing a top-down desert table surface.
2. **Given** GameScene is active, **When** the table top is inspected, **Then** exactly three locations are laid out on top of the World Background using Bevy UI.
3. **Given** locations are inspected, **When** their future reveal state is described, **Then** each location supports unrevealed and revealed states with distinct visual treatment.
4. **Given** future round flow is described, **When** rounds advance from 1 through 6, **Then** the left, middle, and right locations reveal on rounds 1, 2, and 3 respectively, and no new locations reveal on rounds 4, 5, or 6.
5. **Given** GameScene is active, **When** the player HUD is inspected, **Then** a local player hand area is reserved near the bottom of the screen.
6. **Given** GameScene is active, **When** TurnUI is inspected, **Then** it dynamically displays `End Turn` and the current round as `1/6`.
7. **Given** GameScene is active, **When** the scene structure is inspected, **Then** the World Background, locations, hand area, and TurnUI are built from 2D/UI elements while a 3D card front is rendered centered in the local player hand area.

### Edge Cases

- If this spec mentions future systems, it should not imply they are part of the current `006-card-flip` implementation.
- If a future feature creates Table Top behavior, it should define the concrete visual and input details then.
- If future card fronts include non-character content, the CardDefinition concept should still apply.
- If future scenes need different cameras, each scene should own its own camera rather than relying on AppScene for scene-specific presentation.
- If scene switching is inspected, AppScene should have exactly one active child scene: GameScene, DeckBuilderScene, or DebugSettingsScene.
- If the final World Background image is not yet available, GameScene may use a generated DesertWorld placeholder until final art direction is supplied.
- If location reveal-state art is not final, unrevealed locations should use a dynamic red outline and revealed locations should use a dynamic green outline.
- If the app is viewed on portrait/mobile screens, the current feature may defer portrait-specific layout to a future spec.

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
- **FR-015**: The current prototype MUST create AppScene as the persistent app-level scene before opening GameScene.
- **FR-016**: AppScene MUST own debug UI and debug support intended to persist across multiple prototype or gameplay scenes.
- **FR-017**: DeckBuilderScene MUST own the current Deck Builder camera, card presentation, and card-facing UI/presentation entities.
- **FR-018**: Reloading DeckBuilderScene MUST NOT recreate AppScene debug UI.
- **FR-019**: GameScene MUST own its own camera, separate from DeckBuilderScene's camera.
- **FR-020**: The app MUST start on GameScene instead of DeckBuilderScene.
- **FR-021**: Pressing `S` MUST switch between GameScene and DeckBuilderScene.
- **FR-022**: AppScene MUST own exactly one active child scene at a time: either GameScene, DeckBuilderScene, or DebugSettingsScene.
- **FR-023**: GameScene MUST include a World Background that fills the scene view and faces the camera.
- **FR-024**: GameScene MUST lay out exactly three locations on top of the World Background.
- **FR-025**: Each GameScene location MUST support unrevealed and revealed visual states.
- **FR-026**: Future round reveal flow MUST reveal the left location on round 1, middle location on round 2, and right location on round 3, with no location reveal on rounds 4, 5, or 6.
- **FR-027**: The initial world MUST be `DesertWorld`.
- **FR-028**: DesertWorld MUST be represented by the generated top-down desert World Background image at `bevy/crates/game/assets/worlds/desert_world/world_background.png`, leaving usable screen space for GameScene UI overlays.
- **FR-029**: GameScene location, hand, and TurnUI overlays MUST use built-in Bevy UI unless a later requirement exceeds built-in Bevy UI capabilities.
- **FR-030**: GameScene MUST create three location UI instances from the same initial location definition: title `Normal`, body `This is body`, top number `0`, and bottom number `0`.
- **FR-031**: GameScene locations MUST use a shared generated location image for the initial Normal location art, with the four text values overlaid dynamically in UI.
- **FR-032**: Unrevealed locations MUST render a dynamic red outline, and revealed locations MUST render a dynamic green outline.
- **FR-033**: GameScene MUST reserve a local player hand UI area near the bottom of the screen.
- **FR-034**: GameScene MUST include TurnUI that dynamically renders `End Turn` and the current round fraction, starting at `1/6`.
- **FR-035**: World Background MAY vary by world, but locations, local player hand, and TurnUI MUST NOT vary by world.
- **FR-036**: GameScene MUST support hybrid 2D/3D rendering: World Background, locations, hand area, and TurnUI are 2D/UI elements, while 3D Card instances render in front of the local player hand area.
- **FR-037**: The initial GameScene layout MUST target landscape screens.
- **FR-038**: Portrait/mobile layout MAY be deferred to a future feature.
- **FR-039**: GameScene MUST render one 3D card front centered within the local player hand area as the initial hybrid rendering proof.
- **FR-040**: Each dynamic subscene, GameScene, DeckBuilderScene, or DebugSettingsScene, MUST own exactly one scene light.

### Key Entities

- **AppScene**: The persistent app-level scene loaded at startup for debug UI and cross-scene debug support; dynamically owns exactly one active child scene.
- **GameScene**: The default gameplay-facing child scene owned by AppScene; owns its 2D/UI World Background, three location placeholders, local player hand area, and TurnUI.
- **DeckBuilderScene**: A toggleable prototype scene; owns the Deck Builder camera, card presentation, and card-facing presentation UI.
- **World Background**: A full-scene image or placeholder plane facing the GameScene camera, representing the table top/world.
- **DesertWorld**: The first World Background theme, represented as a top-down desert image.
- **Location**: One of three GameScene board shapes placed over the World Background, with unrevealed and revealed states.
- **Normal Location**: The first location definition, with title `Normal`, body `This is body`, top number `0`, and bottom number `0`.
- **Local Player Hand**: A bottom-screen GameScene UI area where 3D Card instances can render in front of the 2D UI.
- **TurnUI**: A GameScene UI element that shows `End Turn` and the current round fraction.
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
- **SC-004**: A reviewer can identify that AppScene persists debug UI while GameScene and DeckBuilderScene each own their own scene camera.
- **SC-005**: A reviewer can identify that GameScene starts by default and `S` cycles AppScene's active child between GameScene and DeckBuilderScene.
- **SC-006**: A reviewer can identify the three GameScene locations and their future round reveal order.
- **SC-007**: A reviewer can identify that DesertWorld is the only initial world and only the World Background changes by world.
- **SC-008**: A reviewer can identify the required dynamic text and outline behavior for initial GameScene location UI.
- **SC-009**: A reviewer can identify that built-in Bevy UI is the selected UI layer for GameScene overlays.
- **SC-010**: A reviewer can identify that the initial GameScene layout targets landscape and defers portrait/mobile layout.
- **SC-011**: A reviewer can verify that GameScene renders one 3D card front centered over the 2D local player hand area.

## Assumptions

- This spec is a concept-holding feature for future planning, not the active implementation target.
- The current active implementation remains `006-card-flip`.
- AppScene is loaded first and stays resident while GameScene is opened immediately after startup as AppScene's active child scene.
- DeckBuilderScene may be reloaded independently during prototype iteration.
- DeckBuilderScene remains available as a prototype view even though it is no longer the startup scene.
- The first World Background art target is DesertWorld, a top-down desert surface.
- Location, local player hand, and TurnUI should be implemented as world-independent UI overlays, with 3D Card instances layered in front of the local player hand.
- Current GameScene layout targets landscape screens.
- The game is inspired by Marvel Snap pacing and structure, but this spec does not clone or define exact Marvel Snap rules.
- Future gameplay specs may refine or replace these concepts when concrete mechanics are designed.
