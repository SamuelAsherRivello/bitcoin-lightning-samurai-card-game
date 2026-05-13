# Feature Specification: Opponent Modes and Two-Player Resolution

**Feature Branch**: `014-opponent`  
**Created**: 2026-05-12  
**Status**: Draft  
**Input**: User description: "014 opponent. Introduce the idea of an opponent. Add a mode button above restart that cycles between Human versus CPU and CPU versus CPU. Each game always has two players. Human versus CPU keeps the bottom near player as the human with a 12-card deck, bottom hand, bottom location slots, restart, and next flow. The top far player has its own deck and off-screen hand, plays to top location slots, and in Human versus CPU acts automatically. CPU should move zero or more cards after round start, with believable 0.5 to 1 second pacing between moves or decisions, then signal next when done. CPU versus CPU plays automatically from start through the end of round 6 without human interaction. Gameplay advances only when both near and far players have gone next. After round six, evaluate all three locations from left to right by power total; whoever wins two or more locations wins the game, with no tie possible. Show final winner text above the mode button with `Status:` and the winning player/controller."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Choose Match Mode (Priority: P1)

A player can see and change the active match mode from the lower-left game controls before or during a game, using the two supported modes only: `Human versus CPU` and `CPU versus CPU`.

**Why this priority**: The feature must expose opponent behavior clearly before two-player turn flow can be understood or tested.

**Independent Test**: Launch GameView and verify a mode button appears above Restart, shows `Mode:` on the first line, defaults to `Human versus CPU` when no saved mode exists, loads the last saved mode when one exists, cycles between the two supported labels on activation, saves the selected mode, and never presents any other mode.

**Acceptance Scenarios**:

1. **Given** GameView is visible, **When** the lower-left controls render, **Then** the mode button appears above the Restart button and uses the same visual style family as Restart.
2. **Given** the mode button is visible, **When** it is inspected, **Then** it displays `Mode:` on the first line and the current mode label on the next line.
3. **Given** the current mode is `Human versus CPU`, **When** the player activates the mode button, **Then** the mode changes to `CPU versus CPU`.
4. **Given** the current mode is `CPU versus CPU`, **When** the player activates the mode button, **Then** the mode changes to `Human versus CPU`.
5. **Given** the mode button is activated repeatedly, **When** modes cycle, **Then** only `Human versus CPU` and `CPU versus CPU` are available.
6. **Given** no saved mode preference exists, **When** GameView starts, **Then** the active mode defaults to `Human versus CPU`.
7. **Given** the player changes the mode, **When** the selected mode changes, **Then** the game saves the selected mode to disk.
8. **Given** a saved mode preference exists, **When** the game starts later, **Then** GameView loads and uses that saved mode.

---

### User Story 2 - Play Human Versus CPU Rounds (Priority: P1)

In `Human versus CPU`, the bottom near player remains the local human player, the top far player is a CPU opponent, and each round advances only after both players are ready for the next round.

**Why this priority**: This is the first complete two-player loop and defines how the existing solo flow becomes a contested game.

**Independent Test**: Start a `Human versus CPU` game, verify both players have independent 12-card copies of the same master deck, the near player can play bottom slots and press Next, the far CPU plays top slots automatically, and the round waits until both players have signaled next.

**Acceptance Scenarios**:

1. **Given** a fresh `Human versus CPU` game starts, **When** turn 1 begins, **Then** the near human player and far CPU player each have an independent 12-card copy of the same master deck.
2. **Given** a round begins, **When** cards are dealt, **Then** each player receives cards from that player's own deck into that player's own hand.
3. **Given** the near player receives cards, **When** the hand is displayed, **Then** the near player's hand appears at the bottom of the screen and remains interactable by the human player.
4. **Given** the far player receives cards, **When** the far hand is represented, **Then** the far player's hand remains off screen and is not directly interactable by the human player.
5. **Given** the near player moves cards to locations, **When** cards are placed, **Then** they occupy only the bottom slots for those locations.
6. **Given** the far CPU moves cards to locations, **When** cards are placed, **Then** they occupy only the top slots for those locations.
7. **Given** the near player presses Next before the far CPU is ready, **When** the far CPU is still thinking or moving cards, **Then** the round does not advance yet.
8. **Given** the far CPU is ready before the near player presses Next, **When** the near player has not signaled readiness, **Then** the round does not advance yet.
9. **Given** both near and far players have signaled readiness on rounds 1 through 5, **When** the game resolves readiness, **Then** the next round starts.
10. **Given** both near and far players have signaled readiness on round 6, **When** the game resolves readiness, **Then** no new round starts and final winner evaluation begins.

---

### User Story 3 - Show Believable CPU Opponent Actions (Priority: P2)

In `Human versus CPU`, the far CPU waits briefly after each round starts, then CPU Brain Level 1 chooses legal drag moves with the goal of winning the game while using seeded randomness when multiple acceptable moves are available.

**Why this priority**: The opponent should feel like another participant instead of an instant hidden state change.

**Independent Test**: Start multiple `Human versus CPU` rounds with a known CPU Brain random seed and verify the far CPU uses CPU Brain Level 1, chooses deterministic legal affordable moves for that seed, favors moves likely to improve victory chances, waits 0.5 to 1 second between visible moves or decisions, and signals ready after it has no energy or no legal affordable moves.

**Acceptance Scenarios**:

1. **Given** a `Human versus CPU` round starts, **When** the far CPU takes a turn with at least one move, **Then** the first CPU card move occurs after a 0.5 to 1 second thinking delay.
2. **Given** the far CPU has selected multiple card moves, **When** the moves are shown, **Then** they appear one card at a time with a 0.5 to 1 second delay between each move.
3. **Given** the far CPU has no legal affordable moves or no energy remaining, **When** its 0.5 to 1 second thinking delay completes, **Then** it signals readiness without placing a card.
4. **Given** the far CPU has no more selected moves, **When** its final move or skip completes, **Then** it signals readiness for the next round.
5. **Given** the far CPU has signaled readiness, **When** the near player continues interacting in the same round, **Then** the CPU does not make additional moves until a later round begins.
6. **Given** the far CPU has placed a card, **When** that placement is complete, **Then** the CPU does not undo or reconsider that move during the turn.

---

### User Story 4 - Simulate CPU Versus CPU (Priority: P2)

In `CPU versus CPU`, both players are CPU-controlled by CPU Brain logic, both use independent copies of the same 12-card master deck plus the same two-player hand, slot, readiness, seeded Brain randomness, and scoring rules, and the game plays automatically from start through the end of round 6 without human interaction.

**Why this priority**: This mode validates that the game is truly modeled as two players instead of one local player plus special-case opponent behavior.

**Independent Test**: Switch to `CPU versus CPU`, start or restart a game with known CPU Brain seeds, and verify both top and bottom players are CPU-controlled by CPU Brain Level 1, each can move legal affordable cards to their own slots with deterministic seeded choices, CPU decisions are paced by 0.5 to 1 second delays, rounds advance after both CPU players are ready, and the game reaches a final winner without human input.

**Acceptance Scenarios**:

1. **Given** the active mode is `CPU versus CPU`, **When** a fresh game starts, **Then** both near and far players are controlled by CPU Brain Level 1.
2. **Given** a `CPU versus CPU` round begins, **When** both CPU players take their turns, **Then** each player chooses legal affordable moves intended to improve that player's chance to win and may use randomness when multiple acceptable moves are available.
3. **Given** one CPU player is ready and the other is still thinking or moving, **When** readiness is checked, **Then** the round does not advance.
4. **Given** both CPU players are ready on rounds 1 through 5, **When** readiness is checked, **Then** the next round starts.
5. **Given** both CPU players are ready on round 6, **When** readiness is checked, **Then** final winner evaluation begins.
6. **Given** the active mode is `CPU versus CPU`, **When** the user takes no action after a fresh game starts, **Then** both CPU players continue making decisions and signaling readiness until a winner is declared after round 6.

---

### User Story 5 - Resolve Winner After Round Six (Priority: P1)

After round six completes, the game compares each location's top and bottom power totals from left to right and declares the player who wins two or more locations as the match winner.

**Why this priority**: Two-player play needs a clear end state and visible outcome.

**Independent Test**: Play or simulate a full six-round game and verify all three locations are evaluated by power totals, the winner is the player with at least two location wins, a final winner is always declared with no tie result, and a `Status:` text above the mode button identifies the winning side and controller type.

**Acceptance Scenarios**:

1. **Given** both players are ready on round 6, **When** the game resolves the final round, **Then** it evaluates location 1, then location 2, then location 3.
2. **Given** a location is evaluated, **When** the near player's bottom-slot total power is greater than the far player's top-slot total power, **Then** the near player wins that location.
3. **Given** a location is evaluated, **When** the far player's top-slot total power is greater than the near player's bottom-slot total power, **Then** the far player wins that location.
4. **Given** all three locations have been evaluated, **When** one player has won two or more locations, **Then** that player is declared the match winner.
5. **Given** final winner evaluation completes, **When** the result is shown, **Then** the game presents either the near player or the far player as winner and never presents a tied match result.
6. **Given** final winner evaluation completes in any mode, **When** the lower-left controls are visible, **Then** a status text above the mode button displays `Status:` plus the winning side and controller type, such as `Status: Winner is Player 1 (CPU)`.

---

### User Story 6 - Reveal Current-Turn Placements After Next (Priority: P1)

In `Human versus CPU`, each player can see their own hand and own current-turn dragged cards as card fronts, while the opposing player's hand and current-turn placed cards remain face down until both players mark Next. When the turn ends, all current-turn placed cards flip face up and stay visible for the rest of the game.

**Why this priority**: Hidden information is fundamental to two-player play and defines what the CPU Brain may know when choosing moves.

**Independent Test**: Start a `Human versus CPU` turn, drag a near-player card to a bottom slot, observe that the human still sees that card's front while the CPU Brain cannot inspect it, observe the far CPU hand and current-turn placements as face-down to the human, then end the turn and verify all current-turn placed cards reveal face up permanently.

**Acceptance Scenarios**:

1. **Given** the near human has cards in hand, **When** the human views those hand cards, **Then** they appear as card fronts to the human player.
2. **Given** the far CPU has cards in hand, **When** the human views the far-player hand area, **Then** the far hand is represented as face-down cards.
3. **Given** the near human drags a card to a location during the current turn, **When** the human views that placed card before the turn ends, **Then** the human still sees its card front.
4. **Given** the near human has placed a card during the current turn, **When** the CPU Brain chooses moves before the turn ends, **Then** the CPU Brain treats that near-player current-turn card as hidden and cannot use its card identity or values.
5. **Given** the far CPU places a card during the current turn, **When** the human views that placed card before the turn ends, **Then** the card appears face down.
6. **Given** both players mark Next, **When** the turn ends, **Then** all current-turn placed cards for both players immediately flip face up.
7. **Given** cards were revealed at the end of a prior turn, **When** later turns begin, **Then** those cards remain face up permanently and may be used as known information for future choices.
8. **Given** any CPU-owned card is rendered in hand or at a location, **When** the mouse cursor moves over or near that card, **Then** the CPU-owned card does not respond to cursor hover and does not rotate toward the mouse cursor.

### Edge Cases

| Edge Case | Expected Behavior |
| --------- | ---------------- |
| The player changes mode during an active game | The active game restarts cleanly in the selected mode at turn `1/6` so controller ownership, decks, readiness, and visible slots do not mix old and new mode state. |
| Restart is pressed while a CPU is thinking or moving cards | All pending CPU actions are cancelled and a fresh game starts at turn `1/6` in the current mode. |
| The game is in `CPU versus CPU` and the user provides no input | CPU controllers continue paced decisions, readiness, turn advancement, and final winner resolution automatically. |
| The near human presses Next before making any card moves | The near player is marked ready, and the round waits for the far player if needed. |
| A CPU has no energy remaining or no legal affordable moves | The CPU still signals readiness after its thinking delay. |
| Both players press or signal readiness with no card moves | The round advances normally if it is rounds 1 through 5, or resolves the winner if it is round 6. |
| A player has fewer cards remaining than the round requests | That player receives only the remaining cards from that player's deck and continues the round. |
| A player's hand has cards but no legal moves | That player may signal readiness without placing cards. |
| A location has equal top and bottom power during final evaluation | The game applies the deterministic no-tie resolution rule defined for tied locations so the overall match can never end in a tie. |
| A player wins the first two evaluated locations | The game may still evaluate and present the third location result, but the match winner remains the player with two or more location wins. |
| CPU movement animation would overlap a round transition | The round waits for the CPU action sequence to finish and readiness to be signaled before advancing. |
| A card was placed during the current turn but the placing player can see its front | That visibility is private to the owning player/controller until the turn ends. |
| A card was placed in a prior turn | It remains face up forever for both players and for CPU Brain knowledge. |
| The cursor moves over a CPU-owned card | The card remains passive; only human-owned interactable cards may use cursor-responsive hover or rotation behavior. |
| A CPU has already dispatched a legal move | The move is final; the CPU does not use Undo or reconsideration behavior. |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: GameView MUST include a mode button positioned above Restart in the lower-left control group.
- **FR-002**: The mode button MUST visually match the Restart button style family closely enough that users understand both are game controls.
- **FR-003**: The mode button MUST render `Mode:` on its first line and the current mode label on its second line.
- **FR-004**: The only supported modes MUST be `Human versus CPU` and `CPU versus CPU`.
- **FR-005**: Activating the mode button MUST cycle between the supported modes.
- **FR-006**: Changing mode during an active game MUST start a clean new game in the selected mode at turn `1/6`.
- **FR-006A**: If no saved mode preference exists, the default selected mode MUST be `Human versus CPU`.
- **FR-006B**: When the selected mode changes, the game MUST save the selected mode to disk.
- **FR-006C**: When the game starts and a saved mode preference exists, the game MUST load and use the saved mode preference.
- **FR-007**: Every game MUST contain exactly two players: a near bottom player and a far top player.
- **FR-008**: The near player MUST use bottom-screen hand presentation and bottom location slots.
- **FR-009**: The far player MUST use an off-screen hand and top location slots.
- **FR-010**: Each player MUST have an independent game deck for each fresh game.
- **FR-011**: For this feature, each fresh player deck MUST be an independent 12-card copy of the same master deck.
- **FR-012**: Cards dealt to a player MUST come only from that player's own remaining game deck.
- **FR-013**: At the start of each turn, each player MUST receive cards from that player's own deck according to the existing round card-deal schedule.
- **FR-014**: A card moved by the near player MUST be placed only into that player's bottom slots at a location.
- **FR-015**: A card moved by the far player MUST be placed only into that player's top slots at a location.
- **FR-016**: In `Human versus CPU`, the near player MUST be controlled by human input.
- **FR-017**: In `Human versus CPU`, the far player MUST be controlled by CPU Brain logic.
- **FR-018**: In `CPU versus CPU`, both near and far players MUST be controlled by CPU Brain logic.
- **FR-019**: The human-controlled near player MUST be able to signal readiness using the existing end-of-turn control, relabeled or interpreted as Next.
- **FR-020**: CPU-controlled players MUST be able to signal readiness without exposing an on-screen button to the human player.
- **FR-021**: A turn MUST advance only after both players have signaled readiness.
- **FR-022**: On turns 1 through 5, resolving both players' readiness MUST start the next turn.
- **FR-023**: On turn 6, resolving both players' readiness MUST start final winner evaluation and MUST NOT deal additional cards.
- **FR-024**: CPU-controlled player decisions MUST be made by CPU Brain logic implemented as authored game code and MUST NOT use a runtime generative AI model.
- **FR-024A**: Every CPU-controlled move or non-move readiness decision MUST be paced by a 0.5 to 1 second delay so CPU players feel human-like rather than instantaneous.
- **FR-025**: A CPU-controlled player's first move, if any, MUST occur after a 0.5 to 1 second delay from the start of that player's turn opportunity.
- **FR-026**: If a CPU-controlled player makes multiple card moves in one turn, those moves MUST be shown one card at a time with a 0.5 to 1 second delay between moves.
- **FR-027**: After a CPU-controlled player has no energy remaining or no legal affordable moves, it MUST signal readiness for the turn.
- **FR-028**: A CPU-controlled player that has already signaled readiness for the current turn MUST NOT make additional moves until a later turn begins.
- **FR-029**: Restart MUST remain available in `Human versus CPU`.
- **FR-030**: Restart MUST start a clean fresh game at turn `1/6` using the currently selected mode.
- **FR-031**: Restart MUST clear both players' hands, placed cards, decks, readiness state, pending CPU actions, turn state, and winner state.
- **FR-032**: Final winner evaluation MUST occur only after both players are ready on turn 6.
- **FR-033**: Final winner evaluation MUST compare exactly three locations from left to right.
- **FR-034**: For each location, the near player's location score MUST be the total power of cards in that location's bottom slots.
- **FR-035**: For each location, the far player's location score MUST be the total power of cards in that location's top slots.
- **FR-036**: The player with the higher location score MUST win that location.
- **FR-037**: If a location's top and bottom power totals are equal, the game MUST apply a deterministic location tie-break rule so that exactly one player wins that location.
- **FR-038**: The match winner MUST be the player who wins at least two of the three locations.
- **FR-039**: Final match results MUST never present a tied match.
- **FR-040**: Winner presentation MUST identify whether the near player or far player won.
- **FR-040A**: GameView MUST show a `Status:` text above the mode button after final winner evaluation in any mode.
- **FR-040B**: Final winner status text MUST identify the winning player number and controller type, for example `Status: Winner is Player 1 (CPU)`.
- **FR-041**: Existing near-player card placement, energy, undo, lock, location ability, and deck-deal rules from Spec 007 MUST continue to apply to human near-player play unless explicitly changed by this spec.
- **FR-042**: CPU card placement MUST respect the same legal-placement constraints as a human-controlled player for deck ownership, hand ownership, location slots, available card movement, and round readiness.
- **FR-043**: The only CPU Brain level in this feature MUST be `CpuBrainLevel = 1`.
- **FR-044**: `CpuBrainLevel = 1` MUST choose legal affordable drag-equivalent moves during a turn until it has no energy remaining or no legal affordable moves.
- **FR-044A**: CPU Brain's goal MUST be to win the game.
- **FR-044B**: When CPU Brain can evaluate legal moves, it SHOULD prefer a move that is more likely to improve the CPU player's chance to win.
- **FR-044C**: When two or more acceptable legal moves are available at the same decision point, CPU Brain MAY choose among them randomly.
- **FR-044D**: CPU Brain randomness MUST support an injectable or configurable random seed so tests can produce deterministic results.
- **FR-045**: No turn timer MUST be imposed on the human player; the human player may take as long as desired before signaling readiness.
- **FR-046**: CPU Brain details MUST remain hidden from the user-facing mode button and match result labels.
- **FR-047**: Each player MUST have exactly one controller: either `PlayerController` or `CpuController`.
- **FR-048**: `PlayerController` MUST dispatch human mouse, keyboard, and tap choices to the shared game logic.
- **FR-049**: `CpuController` MUST use its CPU Brain to dispatch CPU choices to the same shared game logic used by the player controller.
- **FR-050**: CPU Brain knowledge MUST NOT include unrevealed cards in any player's deck.
- **FR-051**: CPU Brain knowledge MAY include the CPU player's current hand card identities and values.
- **FR-052**: CPU Brain knowledge MAY include all open locations and their abilities.
- **FR-053**: CPU Brain knowledge MAY include all revealed location slots on both sides.
- **FR-054**: CPU Brain knowledge MUST NOT include opposing current-turn placed card identities or values until those cards are revealed at the end of the turn.
- **FR-055**: A human player's own hand cards MUST appear as card fronts to that human player.
- **FR-056**: In `Human versus CPU`, the CPU player's hand MUST appear to the human player as face-down cards.
- **FR-057**: A player's cards placed during the current turn MUST be hidden from the opposing player/controller until the turn ends.
- **FR-058**: A player's own cards placed during the current turn MAY still appear as card fronts to that player.
- **FR-059**: When both players have marked Next and the turn ends, all current-turn placed cards MUST immediately reveal face up.
- **FR-060**: Once a placed card has been revealed at the end of a turn, it MUST remain face up permanently for the rest of the game.
- **FR-061**: Undo MUST remain a human-player reconsideration action only.
- **FR-062**: `CpuController` and CPU Brain MUST NOT dispatch Undo choices.
- **FR-063**: Once CPU Brain dispatches a legal card placement choice, that CPU placement MUST be treated as final unless a full game Restart or mode change resets the game.
- **FR-064**: In `CPU versus CPU`, a fresh game MUST progress from turn `1/6` through final winner status without requiring human card movement, Next input, or other gameplay input.
- **FR-065**: CPU-owned cards MUST NOT respond to mouse cursor hover, drag affordances, or cursor-facing rotation effects.
- **FR-066**: Cursor-responsive card hover and rotation behavior MUST apply only to human-owned cards that are currently interactable by the local player.

### Key Entities

| Entity | Description |
| ------ | ----------- |
| **Match Mode** | The selected control mode for a game, limited to `Human versus CPU` and `CPU versus CPU`. |
| **Mode Preference** | The saved user preference for the last selected Match Mode, loaded at game startup and updated when mode changes. |
| **Mode Button** | A lower-left GameView control above Restart that shows and cycles the active Match Mode. |
| **Status Text** | A lower-left GameView text area above the Mode button that reports final match status such as the winning player and controller type. |
| **Near Player** | The bottom player, using bottom hand presentation and bottom location slots; human-controlled in `Human versus CPU` and CPU-controlled in `CPU versus CPU`. |
| **Far Player** | The top opponent, using off-screen hand representation and top location slots; CPU-controlled in both supported modes. |
| **PlayerController** | The human controller for a player; dispatches mouse, keyboard, and tap choices to shared game logic. |
| **CpuController** | The CPU controller for a player; uses CPU Brain to dispatch choices to shared game logic. |
| **Player Deck** | A player-owned 12-card game deck copied or initialized for a fresh game and consumed only by that player. |
| **Master Deck** | The current shared 12-card deck template copied independently for each player at fresh game start; future features may allow different player decks. |
| **Player Hand** | A player-owned set of unplaced cards; visible and interactive at the bottom for the near human player, off screen for the far player. |
| **Location Slot Side** | The top or bottom placement area at each location that separates far-player cards from near-player cards. |
| **Readiness State** | Per-player turn state indicating whether that player has signaled readiness to advance. |
| **CPU Brain** | Authored game-code logic that controls a CPU player without using runtime generative AI. |
| **CPU Brain Knowledge** | The subset of match information available to CPU Brain: its hand, open locations and abilities, and revealed slots on both sides; it excludes unrevealed deck cards and opposing current-turn hidden placements. |
| **CpuBrainLevel** | CPU Brain difficulty or intelligence level; only level `1` is supported in this feature. |
| **CPU Brain Seed** | A deterministic random seed supplied to CPU Brain for repeatable move selection in tests and controlled simulations. |
| **CPU Turn Plan** | The CPU Brain's selected sequence of card moves for the current turn before it signals readiness. |
| **Hidden Current-Turn Placement** | A card placed into a location during the current turn that is private to its owner and face down to the opposing player/controller until the turn ends. |
| **Revealed Placement** | A placed card that has flipped face up at end of turn and remains visible to both players and CPU Brain thereafter. |
| **Card Render Interaction** | The visual input responsiveness of a rendered card; CPU-owned cards are passive, while local human-owned interactable cards may respond to cursor hover or rotation. |
| **Location Score** | The total card power for one player on one location side at final evaluation. |
| **Location Winner** | The player who wins one location after power comparison and any required tie-break. |
| **Match Winner** | The player who wins two or more locations after turn 6 resolves. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of GameView inspections, the mode button appears above Restart and displays the active mode in a two-line `Mode:` format.
- **SC-002**: In 100% of mode cycling tests, only `Human versus CPU` and `CPU versus CPU` are reachable.
- **SC-002A**: In 100% of startup tests without a saved mode preference, GameView starts in `Human versus CPU`.
- **SC-002B**: In 100% of save/load tests, changing mode persists that mode and the next game startup loads the same selected mode.
- **SC-003**: In 100% of fresh games, exactly two players are present and each player has a separate deck, hand, readiness state, and location slot side.
- **SC-004**: In `Human versus CPU`, a tester can play at least one full six-turn game where the near player uses bottom slots and the far CPU uses top slots.
- **SC-005**: In at least 95% of observed CPU turns, CPU moves and readiness decisions are separated by 0.5 to 1 second delays.
- **SC-005A**: In 100% of seeded CPU Brain tests, the same starting state and same seed produce the same move sequence.
- **SC-006**: In 100% of readiness tests, the game remains on the current turn until both players have signaled readiness.
- **SC-007**: In 100% of restart tests, the game returns to turn `1/6`, clears both players' transient game state, and keeps the selected mode.
- **SC-008**: In 100% of completed six-turn games, final evaluation checks three locations from left to right and declares exactly one match winner.
- **SC-009**: In 100% of winner checks, the declared match winner is the player who wins at least two of the three locations.
- **SC-010**: In 100% of final result presentations, the game identifies either the near player or the far player as winner and never shows a tied match.
- **SC-011**: In 100% of current-turn visibility checks, each player can see their own current-turn placed cards while the opposing controller cannot inspect those card identities or values until end-of-turn reveal.
- **SC-012**: In 100% of end-of-turn reveal checks, all current-turn placed cards flip face up immediately and remain face up in later turns.
- **SC-013**: In 100% of `CPU versus CPU` autoplay tests, a fresh game reaches final winner status after round 6 without human gameplay input.
- **SC-014**: In 100% of completed-game UI inspections, a `Status:` text above the mode button identifies the winning player number and controller type.
- **SC-015**: In 100% of CPU-owned card rendering checks, moving the mouse cursor over CPU-owned cards does not trigger hover, drag affordance, or cursor-facing rotation behavior.

## Assumptions

- `Human versus CPU` is the default mode for startup only when no saved mode preference exists.
- The existing lower-right End Turn control from Spec 007 may be relabeled to Next, or may retain its current label while serving the readiness function, as long as the player can clearly signal readiness.
- The phrase "turn" in this spec maps to the existing six-step round flow from Spec 007; turn `1/6` through `6/6` are the visible progression states.
- Both players use independent copies of the same 12-card master deck and the same round card-deal schedule from Spec 007; future specs may allow different player decks.
- CPU decision quality is intentionally simple for this feature, but `CpuBrainLevel = 1` still has the goal of winning and may use seeded randomness to choose among multiple acceptable legal moves.
- Far-player hand details are off screen in this feature, but far-player placed cards at top location slots are visible once played.
- The deterministic tied-location rule may be simple, such as awarding the tied location by a stable player order or another fixed rule, because the user requirement is that final games never tie.
- Location ownership and scoring are based on card power totals, not card energy.
- Existing location abilities continue to affect card runtime values as already specified, but final winner evaluation uses each card's final effective power value where power modifiers exist.
- Visible mode labels remain exactly `Human versus CPU` and `CPU versus CPU`, even though the internal controller logic is named CPU Brain.
- CPU Brain is a hidden detail of each CPU player and should not be exposed as a selectable mode or visible gameplay label in this feature.
- Controllers dispatch choices to game logic; game rules should not be duplicated separately for human and CPU choices.
- Current-turn card placement uses hidden information even if the owning human sees that card's front for usability.
- Future specs may add cards that remain face down for longer, but this feature reveals all current-turn placements when the turn ends.
- Undo is retained for human reconsideration; CPU players are assumed not to change their mind during a turn.
