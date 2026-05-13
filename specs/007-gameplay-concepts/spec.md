# Feature Specification: Gameplay Concepts

**Feature Branch**: `007-gameplay-concepts`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Move broad game architecture context out of 006. The app is a game. The game has a session called a Game and in that Game there are two players. Players may be human or CPU. Each player has locations where cards are placed as well as a hand containing cards that are not placed. All player cards, placed and unplaced, come from that player's deck. Both players have unique decks, but cards come from the same card series. The card series has a specific cardback shared across all. A future Table Top will be where cards sit face up and face down. The current prototype only iterates on 006 for now."

## Clarifications

### Session 2026-05-10

- Q: Which GameScene details should the concept focus on first? → A: DesertWorld background, three UI locations, local player hand, and RoundUI.
- Q: Which UI framework should GameScene use for the initial HUD and overlays? → A: Built-in Bevy UI.
- Q: What is the scene hierarchy? → A: App has one AppScene, and AppScene dynamically owns either GameScene, DeckScene, or DebugScene.
- Q: What dimensional model should GameScene use? → A: GameScene mixes 2D/UI overlays with 3D Card instances rendered in front of the local player hand.
- Q: Which layout orientation should GameScene target first? → A: Landscape first; portrait/mobile later.
- Q: How should subscene lighting be owned? → A: Each dynamic subscene owns one light.

### Session 2026-05-12

- Q: Which lower-right GameScene control should be preserved? → A: Keep the existing lower-right `End Round` button.
- Q: Which lower-left GameScene controls should be added? → A: Add `Restart` above `Undo`, with the Undo button also showing current energy as `Energy current/max`.
- Q: How does the local player's round flow work? → A: A 12-card deck deals a fixed number of cards each round, the player may move hand cards to locations by spending card energy, and `End Round` advances through round `6/6`.
- Q: What does Undo restore? → A: Undo only returns cards moved from hand to locations during the current round; it does not undo previous rounds or card deals.
- Q: What does Restart restore? → A: Restart clears the active GameScene play state and starts a fresh game at round `1/6`.
- Q: When does the local player receive cards? → A: At the start of every round 1 through 6, GameScene deals the requested number of cards from the near player's remaining deck into the near player's hand; deal eligibility is not gated by card energy.
- Q: How do locations display and affect cards? → A: Each location has centered title/body text, starts closed until its reveal round, and only applies its open ability to cards currently placed there.
- Q: Are cards immovable after placement? → A: Cards placed during the current round may be dragged back to the player hand area and inserted anywhere in hand during that same round; placed cards lock when the round ends.

### Session 2026-05-13

- Q: How is a winner calculated when players tie on one or more locations? → A: First compare how many locations each team wins; if tied, compare total power points across all locations; if still tied, compare most cards played in the game.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Preserve Gameplay Vocabulary (Priority: P1)

A designer or developer can reference a single future-facing concept document for the game's high-level entities without forcing those systems into the current card presentation bundle.

**Why this priority**: The project needs shared language for future gameplay work while keeping current visual prototype scope small.

**Independent Test**: Read this spec and verify it defines the conceptual terms Game, Player, Deck, CardSeries, CardDefinition, CardInstance, hand, placed cards, shared locations, and Table Top without requiring implementation in `006-card-bundle`.

**Acceptance Scenarios**:

1. **Given** a future gameplay task references a Game, **When** this spec is consulted, **Then** Game means a two-player session.
2. **Given** a future card task references CardSeries, **When** this spec is consulted, **Then** CardSeries owns the shared card back for its cards.
3. **Given** the current `006-card-bundle` prototype is reviewed, **When** this spec is consulted, **Then** these gameplay concepts remain future context rather than required current behavior.

---

### User Story 2 - Describe Future Hidden Card Flow (Priority: P2)

A designer can describe how cards move from deck to hand to shared board locations, including face-down and reveal timing concepts, without implementing those systems yet.

**Why this priority**: Card flipping in `006` is a visual proof of future hidden information and reveal behavior.

**Independent Test**: Review the concept relationships and confirm they describe hidden placement and reveal timing at a high level without specifying round rules.

**Acceptance Scenarios**:

1. **Given** a card is in a future player's hand, **When** it is placed later, **Then** it may be represented face down before reveal.
2. **Given** a future card is revealed, **When** its front is shown, **Then** the card front comes from its card definition while the back comes from its card series.
3. **Given** future board play is designed, **When** locations are referenced, **Then** they are shared board spaces contested by both players rather than player-owned locations.
4. **Given** a future game reaches final scoring, **When** each shared location has both teams' placed-card power totals, **Then** the winner is calculated from location wins, then total power points, then most cards played.

---

### User Story 3 - Clarify Prototype Scene Layering (Priority: P2)

A developer can understand how the current prototype separates persistent app-level tools from the active Deck scene while future gameplay concepts are still being defined.

**Why this priority**: The Deck should remain an iterated prototype scene, while app-level debug tools stay available across current and future scenes.

**Independent Test**: Launch or inspect the runtime and verify startup creates a persistent AppScene first, then AppScene opens GameScene by default, with DeckScene available through the scene shortcut.

**Acceptance Scenarios**:

1. **Given** the app starts, **When** startup systems run, **Then** AppScene is created before GameScene and owns GameScene as its active child scene.
2. **Given** AppScene is active, **When** future scenes are opened or reloaded, **Then** AppScene keeps debug UI and debug support that should persist across scenes.
3. **Given** DeckScene is active, **When** it is reloaded, **Then** the Deck camera, card, and card-facing presentation entities are replaced without recreating the persistent debug UI.
4. **Given** either GameScene, DeckScene, or DebugScene is active, **When** the player presses `S`, **Then** AppScene replaces its active child scene with the next scene.

---

### User Story 4 - Introduce Game Scene Table Top (Priority: P2)

A designer or developer can see the first gameplay-facing scene concept: a world background with three location shapes laid out on top for future round play.

**Why this priority**: GameScene is the first bridge from the Deck prototype toward actual game presentation without implementing full card play yet.

**Independent Test**: Launch or inspect GameScene and verify it has its own camera, a DesertWorld world background that fills the scene view, three visible UI locations with open/closed text, a local player hand area, and RoundUI.

**Acceptance Scenarios**:

1. **Given** GameScene is active, **When** it is rendered, **Then** the world is represented by a full-scene DesertWorld World Background image showing a top-down desert table surface.
2. **Given** GameScene is active, **When** the table top is inspected, **Then** exactly three locations are laid out on top of the World Background using Bevy UI.
3. **Given** locations are inspected, **When** their state is described, **Then** each location supports closed and open states with distinct text and visual treatment.
4. **Given** round flow is described, **When** rounds advance from 1 through 6, **Then** the left, middle, and right locations open on rounds 1, 2, and 3 respectively, and no new locations open on rounds 4, 5, or 6.
5. **Given** GameScene is active, **When** the player HUD is inspected, **Then** a local player hand area is reserved near the bottom of the screen.
6. **Given** GameScene is active, **When** RoundUI is inspected, **Then** it dynamically displays `End Round` and the current round as `1/6`.
7. **Given** GameScene is active, **When** the scene structure is inspected, **Then** the World Background, locations, hand area, and RoundUI are built from 2D/UI elements while a 3D card front is rendered centered in the local player hand area.
8. **Given** a location is closed, **When** it is displayed, **Then** its title text reads `Closed Until Round X` and its body text is blank.
9. **Given** a location is open, **When** it is displayed, **Then** its two-line title and three-line body are centered horizontally and show that location's title and ability text.

---

### User Story 5 - Play Local Round Progression (Priority: P1)

A human near player can play through six local rounds by receiving cards from a 12-card deck, spending round energy to move hand cards to locations, undoing only current-round placements, ending rounds, and restarting the game from a clean state.

**Why this priority**: GameScene needs a complete local interaction loop before future CPU behavior, scoring, or advanced card effects can be evaluated.

**Independent Test**: Launch or inspect GameScene and verify the lower-right End Round button remains present, lower-left Restart and Undo controls are present, cards deal from the near player's deck into the local hand at the start of every round 1 through 6 according to the round schedule, energy increases according to the round schedule, Undo only affects current-round placements, and Restart returns the game to round `1/6`.

**Acceptance Scenarios**:

1. **Given** GameScene starts a fresh game, **When** round 1 begins, **Then** the near player has a 12-card deck order built from the available card definitions and one card from the near player's deck animates from below the center of the screen into the local hand.
2. **Given** cards are dealt into the local hand, **When** each card enters, **Then** it lines up to the right of existing hand cards and the full hand group recenters within the hand area.
3. **Given** more than four or five cards are in hand, **When** the centered hand group exceeds the hand area's width, **Then** the cards may extend outside that width without changing the centering rule.
4. **Given** the player moves a card from hand to a location, **When** the card is placed during the current round, **Then** the player's available energy is reduced by that card's energy value.
5. **Given** the player moves a card to an open location with an ability, **When** the card is added, **Then** the location ability is applied to that card immediately.
6. **Given** at least one card has moved from hand to a location during the current round, **When** the player presses Undo, **Then** only cards moved during the current round return from locations to the hand, their location ability effects are removed, and prior-round placements remain unchanged.
7. **Given** no cards have moved from hand to a location during the current round, **When** GameScene renders the lower-left controls, **Then** the Undo button is disabled or visually greyed out.
8. **Given** a card was moved from hand to a location during the current round, **When** the player drags that card back to the player hand area, **Then** the card may return to hand during that same round.
9. **Given** a current-round placed card is dragged over the hand area, **When** the player moves it before, between, or after existing hand cards, **Then** the hand cards shift on the x axis to show the insertion gap and the player may release it into that hand order.
10. **Given** a card remained placed when the round ended, **When** a later round begins, **Then** that placed card can no longer be moved by drag.
11. **Given** the player presses End Round on rounds 1 through 5, **When** the next round starts, **Then** the round indicator advances, current-round placed cards lock, the next round's requested cards are dealt from deck to hand, and the next round's energy is granted.
12. **Given** the player presses End Round on round `6/6`, **When** the round resolves, **Then** no additional cards are dealt.
13. **Given** the player presses Restart at any time, **When** the restart completes, **Then** locations, hand, round, energy, deck state, and lower-left control state reset to a fresh round `1/6` game.
14. **Given** rounds 1 through 6 begin, **When** the near player's deck has enough remaining cards for the Round Progression count, **Then** GameScene deals that many cards from the deck to the hand regardless of those cards' energy values.

### Edge Cases

- If this spec mentions future systems, it should not imply they are part of the current `006-card-bundle` implementation.
- If behavior outside the local near-player round loop is required, such as CPU rounds, scoring implementation, location ownership UI, or card abilities, it should be defined by a later spec.
- If future card fronts include non-character content, the CardDefinition concept should still apply.
- If future scenes need different cameras, each scene should own its own camera rather than relying on AppScene for scene-specific presentation.
- If scene switching is inspected, AppScene should have exactly one active child scene: GameScene, DeckScene, or DebugScene.
- If the final World Background image is not yet available, GameScene may use a generated DesertWorld placeholder until final art direction is supplied.
- If location state art is not final, closed locations should use a dynamic red outline and open locations should use a dynamic green outline.
- If the app is viewed on portrait/mobile screens, the current feature may defer portrait-specific layout to a future spec.
- If the near player's deck has fewer remaining cards than a round requests, GameScene should deal only the remaining deck cards and continue the round without creating extra cards.
- If the player attempts to move a card whose energy cost is greater than available energy, the card should remain in hand and available energy should not change.
- If Undo returns current-round cards to hand, the hand group should recenter using the same hand layout rule as card dealing.
- If Undo returns a card from an open location, the location ability effect applied to that card should be removed before the card is placed back in hand.
- If a player manually drags a current-round placed card back to hand, the card's location ability effect and placement energy deduction should be removed just as they are during Undo.
- If a current-round placed card is dragged over the hand area but released without a clear insertion gap, it should return to its current location slot and remain undoable for the current round.
- If a card is placed in a closed location, no location ability should be applied until the location opens; when that location opens, its ability should apply immediately to cards already there.
- If End Round advances the round, every current-round placed card still in a location should become locked and no longer draggable.
- If Restart is pressed while card movement or deal animation is in progress, the completed state should be a clean fresh game, not a partial mix of old and new state.

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
- **FR-014**: This feature MUST NOT require changes to the `006-card-bundle` prototype behavior.
- **FR-015**: The current prototype MUST create AppScene as the persistent app-level scene before opening GameScene.
- **FR-016**: AppScene MUST own debug UI and debug support intended to persist across multiple prototype or gameplay scenes.
- **FR-017**: DeckScene MUST own the current Deck camera, card presentation, and card-facing UI/presentation entities.
- **FR-018**: Reloading DeckScene MUST NOT recreate AppScene debug UI.
- **FR-019**: GameScene MUST own its own camera, separate from DeckScene's camera.
- **FR-020**: The app MUST start on GameScene instead of DeckScene.
- **FR-021**: Pressing `S` MUST switch between GameScene and DeckScene.
- **FR-022**: AppScene MUST own exactly one active child scene at a time: either GameScene, DeckScene, or DebugScene.
- **FR-023**: GameScene MUST include a World Background that fills the scene view and faces the camera.
- **FR-024**: GameScene MUST lay out exactly three locations on top of the World Background.
- **FR-025**: Each GameScene location MUST support closed and open visual states.
- **FR-026**: Round location flow MUST open the left location on round 1, middle location on round 2, and right location on round 3, with no new location opening on rounds 4, 5, or 6.
- **FR-027**: The initial world MUST be `DesertWorld`.
- **FR-028**: DesertWorld MUST be represented by the generated top-down desert World Background image at `bevy/crates/game/assets/worlds/desert_world/world_background.png`, leaving usable screen space for GameScene UI overlays.
- **FR-029**: GameScene location, hand, and RoundUI overlays MUST use built-in Bevy UI unless a later requirement exceeds built-in Bevy UI capabilities.
- **FR-030**: GameScene MUST create three location UI instances from the Location Definition Values table.
- **FR-031**: GameScene locations MUST use a shared generated location image for the initial location art, with title and body text overlaid dynamically in UI.
- **FR-032**: Closed locations MUST render a dynamic red outline, and open locations MUST render a dynamic green outline.
- **FR-033**: GameScene MUST reserve a local player hand UI area near the bottom of the screen.
- **FR-034**: GameScene MUST include RoundUI that dynamically renders `End Round` and the current round fraction, starting at `1/6`.
- **FR-035**: World Background MAY vary by world, but locations, local player hand, and RoundUI MUST NOT vary by world.
- **FR-036**: GameScene MUST support hybrid 2D/3D rendering: World Background, locations, hand area, and RoundUI are 2D/UI elements, while 3D Card instances render in front of the local player hand area.
- **FR-037**: The initial GameScene layout MUST target landscape screens.
- **FR-038**: Portrait/mobile layout MAY be deferred to a future feature.
- **FR-039**: GameScene MUST render one 3D card front centered within the local player hand area as the initial hybrid rendering proof.
- **FR-040**: Each dynamic subscene, GameScene, DeckScene, or DebugScene, MUST own exactly one scene light.
- **FR-041**: GameScene MUST keep the lower-right End Round button.
- **FR-042**: GameScene MUST add two lower-left controls: `Restart` above `Undo`.
- **FR-043**: The Undo control MUST show current and maximum energy above its action text, formatted as `Energy current/max` on one line and `Undo` on the next line.
- **FR-044**: A fresh near-player game MUST start at round `1/6`.
- **FR-045**: The near human player MUST start each fresh game with a deck of exactly 12 cards.
- **FR-046**: The near player's 12-card deck MUST contain three CardInstances of each definition in the initial master card list, then randomize their order for the fresh game.
- **FR-047**: The initial master card list MUST contain exactly the card definitions in the Card Definition Values table.
- **FR-048**: At the start of every round 1 through 6, GameScene MUST deal cards from the near player's remaining deck to the near player's hand according to the Round Progression table.
- **FR-049**: Dealt cards MUST originate from screen x-position `screen_width / 2` and from a y-position below the bottom of the screen before animating into the hand area.
- **FR-050**: Each dealt card MUST animate into the local player hand and line up to the right of any existing hand cards.
- **FR-051**: After each deal, move, undo, or restart state change, the local hand card group MUST be centered within the hand area.
- **FR-052**: If the local hand group exceeds the hand area's width, GameScene MAY allow the card group to extend outside that width while preserving centered group alignment.
- **FR-053**: At the start of each round, GameScene MUST set the near player's maximum and available energy according to the Round Progression table.
- **FR-054**: Moving a card from hand to a location MUST deduct available energy equal to that card's energy value.
- **FR-055**: GameScene MUST prevent a card move from hand to location when that card's energy value is greater than the near player's available energy.
- **FR-056**: GameScene MUST track the set of cards moved from hand to locations during the current round.
- **FR-057**: Undo MUST return only cards moved from hand to locations during the current round.
- **FR-058**: Undo MUST NOT return cards moved during previous rounds.
- **FR-059**: Undo MUST be disabled or visually greyed out when no cards have moved from hand to locations during the current round.
- **FR-060**: Pressing End Round on rounds 1 through 5 MUST advance to the next round and trigger that round's card deal and energy grant.
- **FR-061**: Pressing End Round on round `6/6` MUST be allowed and MUST NOT deal additional cards afterward.
- **FR-062**: Restart MUST be available at any time during GameScene play.
- **FR-063**: Restart MUST clear the active GameScene play state, including hand cards, placed cards, current-round move history, deck progress, round, energy, and control enablement.
- **FR-064**: After Restart completes, GameScene MUST be in a fresh round `1/6` state with a newly randomized 12-card near-player deck.
- **FR-065**: GameScene MUST preserve the near player's randomized remaining deck order when selecting cards to deal.
- **FR-066**: GameScene MUST NOT gate round-start card dealing by card energy value.
- **FR-067**: If the near player's deck has fewer remaining cards than the round's requested deal count, GameScene MUST deal only the remaining deck cards and continue the round.
- **FR-068**: With the initial 12-card near-player deck and the Round Progression table, GameScene MUST deal cards on every round from round 1 through round 6.
- **FR-069**: Each location MUST render exactly two text areas: title and body.
- **FR-070**: Location title text MUST be horizontally centered, positioned at 30% from the top of the location area, use the larger location title font, and support two lines.
- **FR-071**: Location body text MUST be horizontally centered below the title area and support three lines.
- **FR-072**: When a location is closed, its title MUST read `Closed Until Round X`, where X is the round when that location opens, and its body MUST be blank.
- **FR-073**: When a location is open, its title MUST show the location title and its body MUST show the location ability text.
- **FR-074**: A location with no defined ability MUST show `(No Ability)` in its open body text.
- **FR-075**: Only open locations MUST apply location ability effects to cards.
- **FR-076**: When a card is added to an open location, or when a location opens with cards already there, that location's ability MUST apply to affected cards immediately.
- **FR-077**: When a card is removed from a location, including by Undo, the location ability effect from that location MUST be removed from that card.
- **FR-078**: `Fortress Gate` MUST add `+2 Energy` to each card placed there while it is open.
- **FR-079**: `Bamboo Crossing` MUST add `-2 Energy` to each card placed there while it is open.
- **FR-080**: Location ability effects MUST update the affected card's effective energy value without changing the card definition's base energy value.
- **FR-081**: A card moved from hand to a location during the current round MUST remain manually movable back to the player hand area until the round ends.
- **FR-082**: A card that remains placed when End Round advances the round MUST become locked and MUST NOT be movable by drag in later rounds.
- **FR-083**: While a current-round placed card is dragged over the hand area, existing hand cards MUST shift on the x axis to show the candidate insertion gap before, between, or after hand cards.
- **FR-084**: Releasing a current-round placed card over a valid hand insertion gap MUST return it to hand at that selected hand order and recenter the full hand group.
- **FR-085**: Returning a current-round placed card to hand by manual drag MUST remove that card's location ability effect and restore the energy spent by that placement.
- **FR-086**: Undo MUST return all cards still recorded in the current-round move history to hand, including cards that were moved to any location during that round and have not already been manually returned.
- **FR-087**: Future final scoring MUST calculate each team's power score at each shared location from that team's placed cards at that location.
- **FR-088**: A team MUST win a shared location when its location power score is higher than the other team's score at that location.
- **FR-089**: If both teams have the same power score at a shared location, that location MUST count as tied and MUST NOT increase either team's location-win count.
- **FR-090**: Future final winner calculation MUST first compare total locations won by each team.
- **FR-091**: If total locations won are tied, including cases where one or more locations are tied on power, final winner calculation MUST compare each team's total power points across all locations.
- **FR-092**: If total locations won and total power points are both tied, final winner calculation MUST compare the total number of cards each team played during the game.
- **FR-093**: If locations won, total power points, and total cards played are all tied, the game result MUST be a draw unless a later spec defines another tie-breaker.

### Round Progression

| Round | Cards Dealt From Player Deck | Energy Maximum | Energy Available At Round Start |
| ----- | ---------------------------- | -------------- | ------------------------------- |
| 1 | 1 | 1 | 1 |
| 2 | 2 | 2 | 2 |
| 3 | 3 | 3 | 3 |
| 4 | 1 | 4 | 4 |
| 5 | 1 | 5 | 5 |
| 6 | 1 | 6 | 6 |

### Card Definition Values

| Card | Power | Energy |
| ---- | ----- | ------ |
| kage | 1 | 1 |
| sister | 2 | 1 |
| Lord | 3 | 2 |
| test | 4 | 3 |

### Location Definition Values

| Slot | Opens On Round | Title | Body | Ability Effect |
| ---- | -------------- | ----- | ---- | -------------- |
| Left | 1 | Fortress Gate | +2 Energy to each card here | Add +2 to each placed card's effective energy |
| Middle | 2 | Bamboo Crossing | -2 Energy to each card here | Add -2 to each placed card's effective energy |
| Right | 3 | Normal | (No Ability) | No ability effect |

### Final Winner Calculation

| Step | Rule | Winner When |
| ---- | ---- | ----------- |
| 1 | Compare locations won | One team has won more of the three shared locations |
| 2 | Compare total power points | Location wins are tied and one team has more combined power across all locations |
| 3 | Compare cards played | Location wins and total power are tied and one team played more cards during the game |
| 4 | Draw | Locations won, total power, and cards played are all tied |

### Key Entities

- **AppScene**: The persistent app-level scene loaded at startup for debug UI and cross-scene debug support; dynamically owns exactly one active child scene.
- **GameScene**: The default gameplay-facing child scene owned by AppScene; owns its 2D/UI World Background, three location placeholders, local player hand area, and RoundUI.
- **GameScene**: The active gameplay presentation within GameScene where the near human player's hand, locations, round controls, energy, undo, restart, and card dealing are visible.
- **DeckScene**: A toggleable prototype scene; owns the Deck camera, card presentation, and card-facing presentation UI.
- **World Background**: A full-scene image or placeholder plane facing the GameScene camera, representing the table top/world.
- **DesertWorld**: The first World Background theme, represented as a top-down desert image.
- **Location**: One of three GameScene board shapes placed over the World Background, with closed and open states, title/body text, and an optional ability.
- **Location Ability**: An effect owned by an open location and applied immediately to each card placed there until that card leaves the location.
- **Closed Location**: A location that has not reached its open round; displays `Closed Until Round X`, no body, and applies no ability.
- **Open Location**: A location that has reached its open round; displays its own title and ability body and applies its ability to placed cards.
- **Fortress Gate**: The left location, opened on round 1, with `+2 Energy to each card here`.
- **Bamboo Crossing**: The middle location, opened on round 2, with `-2 Energy to each card here`.
- **Normal Location**: The right location, opened on round 3, with `(No Ability)`.
- **Local Player Hand**: A bottom-screen GameScene UI area where 3D Card instances can render in front of the 2D UI.
- **RoundUI**: A GameScene UI element that shows `End Round` and the current round fraction.
- **End Round Button**: The lower-right GameScene control that advances round progression.
- **Restart Button**: The upper lower-left GameScene control that resets the active play state to a fresh round `1/6` game.
- **Undo Button**: The lower lower-left GameScene control that displays energy and returns only current-round moved cards from locations to hand.
- **Energy**: The near player's round resource, with available and maximum values reset by round and spent when cards move from hand to locations.
- **Round**: The six-step local progression from `1/6` through `6/6`, with fixed card deal and energy values per round.
- **Game**: A future two-player match session.
- **Player**: A future participant in a Game, controlled by a human or CPU.
- **Deck**: A future player-owned collection of card instances available during a Game.
- **Deal Selection**: The start-of-round process that moves the requested number of cards from the near player's remaining deck order into the hand.
- **Hand**: A future player-owned zone containing unplaced cards.
- **Placed Cards**: Future cards a player has committed to board locations.
- **Table Top**: A future board surface where cards can sit face up or face down.
- **Shared Location**: A future board-owned space contested by both players.
- **Location Power Score**: The sum of one team's placed-card power values at one shared location during final scoring.
- **Location Win**: A shared location result awarded to the team with the higher Location Power Score at that location.
- **Tied Location**: A shared location where both teams have the same Location Power Score; it awards no Location Win.
- **Total Power Points**: The sum of one team's placed-card power values across all shared locations, used as the first final winner tie-breaker after Location Wins.
- **Cards Played**: The count of cards a team played during the game, used as the second final winner tie-breaker after Location Wins and Total Power Points.
- **CardSeries**: A shared card collection identity that owns the common CardBack used across cards in that series.
- **CardDefinition**: Static card data and front content.
- **CardInstance**: A runtime copy of a CardDefinition with owner and match state.
- **CardFront**: The specific visible front content for a CardDefinition.
- **CardBack**: The shared back visual for a CardSeries.
- **Current-Round Move History**: The set of cards moved from hand to locations since the current round began, used as the only scope of Undo.
- **Current-Round Placed Card**: A card moved from hand to a location during the current round; it remains eligible to drag back to the hand area until End Round.
- **Locked Placed Card**: A card that remained placed after End Round advanced the round; it is no longer movable by drag.
- **Hand Insertion Gap**: The temporary space shown before, between, or after hand cards while a current-round placed card is being returned to hand.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer can identify the future ownership of Game, Player, Deck, Hand, CardSeries, CardDefinition, CardInstance, Table Top, and Shared Location from this spec without reading implementation code.
- **SC-002**: In scope review, this spec introduces no current implementation requirements for `006-card-bundle`.
- **SC-003**: In terminology review, CardBack is consistently owned by CardSeries and CardFront is consistently owned by CardDefinition.
- **SC-004**: A reviewer can identify that AppScene persists debug UI while GameScene and DeckScene each own their own scene camera.
- **SC-005**: A reviewer can identify that GameScene starts by default and `S` cycles AppScene's active child between GameScene and DeckScene.
- **SC-006**: A reviewer can identify the three GameScene locations and their round 1, 2, and 3 open order.
- **SC-007**: A reviewer can identify that DesertWorld is the only initial world and only the World Background changes by world.
- **SC-008**: A reviewer can identify the required dynamic text and outline behavior for initial GameScene location UI.
- **SC-009**: A reviewer can identify that built-in Bevy UI is the selected UI layer for GameScene overlays.
- **SC-010**: A reviewer can identify that the initial GameScene layout targets landscape and defers portrait/mobile layout.
- **SC-011**: A reviewer can verify that GameScene renders one 3D card front centered over the 2D local player hand area.
- **SC-012**: A reviewer can verify that GameScene has one lower-right End Round control and two lower-left controls ordered Restart above Undo.
- **SC-013**: A reviewer can verify that a fresh game deals cards from the near player's deck to the near player's hand at the start of every round; with the initial 12-card deck this yields 1 card on round 1, 2 cards on round 2, 3 cards on round 3, and 1 card on each of rounds 4, 5, and 6.
- **SC-014**: A reviewer can verify that round-start energy maximums and available values progress as `1, 2, 3, 4, 5, 6`.
- **SC-015**: A reviewer can verify that Undo is disabled before any current-round placement and only returns current-round placements after cards have been moved.
- **SC-016**: A reviewer can verify that Restart returns the active GameScene state to a clean round `1/6` game with a new 12-card deck.
- **SC-017**: A reviewer can verify that the near player's deck card values match the Card Definition Values table and contain exactly three randomized instances of each listed card definition.
- **SC-018**: A reviewer can verify that closed locations show `Closed Until Round X`, open on rounds 1, 2, and 3 from left to right, and show their own title/body when open.
- **SC-019**: A reviewer can verify that Fortress Gate adds +2 effective energy to cards placed there while open, Bamboo Crossing adds -2 effective energy to cards placed there while open, and Normal applies no ability.
- **SC-020**: A reviewer can verify that Undo removes current-round location ability effects from returned cards.
- **SC-021**: A reviewer can verify that a card placed during the current round can be dragged back to hand, inserted before, between, or after existing hand cards, and has its energy/location effect restored or removed appropriately.
- **SC-022**: A reviewer can verify that cards still placed when End Round advances become locked and cannot be dragged in later rounds.
- **SC-023**: A reviewer can identify final winner calculation order as locations won, then total power points, then total cards played, with tied locations awarding no location win.

## Assumptions

- This spec now combines gameplay vocabulary with concrete near-player GameScene round progression.
- The `006-card-bundle` prototype remains separate and should not be changed by this spec.
- AppScene is loaded first and stays resident while GameScene is opened immediately after startup as AppScene's active child scene.
- DeckScene may be reloaded independently during prototype iteration.
- DeckScene remains available as a prototype view even though it is no longer the startup scene.
- The first World Background art target is DesertWorld, a top-down desert surface.
- Location, local player hand, and RoundUI should be implemented as world-independent UI overlays, with 3D Card instances layered in front of the local player hand.
- Current GameScene layout targets landscape screens.
- The game is inspired by Marvel Snap pacing and structure, but this spec does not clone or define exact Marvel Snap rules.
- Future gameplay specs may refine or replace these concepts when concrete mechanics are designed.
- The lower-right button referenced by the user is the existing End Round button.
- The near player is the local human player for this round progression.
- Undo restores cards and energy for current-round placements because the move being undone includes the related energy deduction.
- Manual same-round return to hand restores the same placement energy and location effect as Undo, but only for the card being returned.
- Deal selection preserves the randomized remaining deck order and is independent of card energy; round energy only controls placement affordability.
- Location ability effects modify card runtime effective energy only; card definition energy remains the immutable base value.
