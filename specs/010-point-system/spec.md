# Feature Specification: Point System

**Feature Branch**: `010-point-system`  
**Created**: 2026-05-11  
**Status**: Draft  
**Input**: User description: "Add a new spec for 010-point-system. Build from the existing gameplay concepts, especially 007, and define the points system for cards, locations, etc."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Define Card Point Values (Priority: P1)

A designer can define each card with a clear cost and power value so future gameplay, UI, and card balancing work use the same point vocabulary.

**Why this priority**: Cards are the smallest gameplay unit. Their point fields must be defined before location totals, control, or match outcome can be reasoned about.

**Independent Test**: Review this spec and verify that card cost and card power are distinct values with separate meanings, and that every playable card can expose both values.

**Acceptance Scenarios**:

1. **Given** a card definition is reviewed, **When** its point values are inspected, **Then** it has a cost value and a power value.
2. **Given** a card is placed at a location, **When** location totals are calculated, **Then** the card's power contributes to its owner's total at that location.
3. **Given** a card has a cost value, **When** score or location control is calculated, **Then** cost does not directly add to that score or control total.
4. **Given** a card is rendered, **When** its point values are visible, **Then** its cost and power each use their own point view with an artful background and dynamic foreground text.

---

### User Story 2 - Show Location Totals (Priority: P1)

A player can look at the three shared locations and understand each player's current point total at each location.

**Why this priority**: The game scene already reserves top and bottom location numbers. Those numbers need a shared meaning before the board becomes playable.

**Independent Test**: Inspect a location during a match state and confirm that it shows one total for each player, derived from cards at that location.

**Acceptance Scenarios**:

1. **Given** a location contains revealed cards from both players, **When** the location is displayed, **Then** the top number shows the opponent total and the bottom number shows the local player total.
2. **Given** a player has no revealed cards at a location, **When** the location total is displayed, **Then** that player's total is `0` before modifiers.
3. **Given** a card's power changes because of a modifier, **When** the location total is refreshed, **Then** the displayed total reflects the modified power.
4. **Given** a location total is displayed, **When** its power total is visible, **Then** it uses a power point view with an artful background and dynamic foreground text.
5. **Given** one player has up to four revealed cards at a location, **When** the location total is calculated, **Then** the displayed total is the sum of those cards' effective power values plus any resolved location-level modifiers.

---

### User Story 3 - Determine Location Control (Priority: P2)

A player can tell which player is currently winning each shared location based on the compared point totals.

**Why this priority**: Location control is the bridge between card points and overall match outcome.

**Independent Test**: Create match states where the local player leads, the opponent leads, totals are tied, and no cards are present; verify the control result for each location.

**Acceptance Scenarios**:

1. **Given** the local player's total is higher than the opponent total at a location, **When** control is evaluated, **Then** the local player controls that location.
2. **Given** the opponent total is higher than the local player total at a location, **When** control is evaluated, **Then** the opponent controls that location.
3. **Given** both totals are equal at a location, **When** control is evaluated, **Then** neither player controls that location.
4. **Given** both totals are `0` at an empty location, **When** control is evaluated, **Then** neither player controls that location.

---

### User Story 4 - Determine Match Outcome (Priority: P2)

A player can understand who is winning or who won the match by comparing controlled locations, with a clear tiebreaker.

**Why this priority**: A point system needs a complete outcome rule so later turn, ability, CPU, and end-game work can integrate against one model.

**Independent Test**: Review or simulate end-of-match states with different controlled-location counts and ties; verify the expected winner or draw.

**Acceptance Scenarios**:

1. **Given** one player controls more locations than the other after final scoring, **When** the match result is evaluated, **Then** that player wins.
2. **Given** both players control the same number of locations after final scoring, **When** the match result is evaluated, **Then** the player with the higher total power across all locations wins.
3. **Given** both players control the same number of locations and have equal total power across all locations, **When** the match result is evaluated, **Then** the match is a draw.

### Edge Cases

| Edge Case | Expected Behavior |
| --------- | ----------------- |
| Negative card power or modifiers produce a negative location total | The location total may be negative unless a later balancing spec forbids negative values. |
| A cost or power value is outside `-99` through `99` | The point view contract does not guarantee readable rendering outside this range; future balancing or validation may constrain source values. |
| More than four cards would be placed for one player at one location | The location capacity is four cards per player, so the extra placement is invalid unless a later card or location rule explicitly changes capacity. |
| A card is unrevealed | Its power does not contribute to visible location totals until it is revealed, unless a later card ability explicitly says otherwise. |
| A location is unrevealed | Its name/body may remain hidden, but placed revealed cards still contribute to the visible player totals unless a later location rule says otherwise. |
| Both players have equal totals at one or more locations | Equal totals create no controller for those locations. |
| A card moves between locations | Its power stops contributing to the old location and starts contributing to the new location after the move is resolved. |
| A modifier affects a card or location total multiple times | The final displayed total uses the resolved current match state, not a visual accumulation of intermediate values. |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The point system MUST define card `cost` as the value used by future play-budget rules to determine whether a card can be played.
- **FR-002**: The point system MUST define card `power` as the base value a revealed card contributes to its owner's total at its current location.
- **FR-003**: Card cost MUST NOT directly contribute to location totals, location control, or match outcome.
- **FR-004**: Every playable card definition MUST be able to expose a displayable cost value and a displayable power value.
- **FR-005**: A card instance MUST be able to hold resolved match-state changes that affect its effective power separately from the card definition's base power.
- **FR-006**: The point system MUST define `CostPointModel` as the cost value model for a card or other future cost-bearing entity.
- **FR-007**: The point system MUST define `CostPointView` as the rendering appearance of a `CostPointModel` for a given entity.
- **FR-008**: Every playable card MUST have a `CostPointView`.
- **FR-009**: `CostPointView` MUST combine an artful background with dynamic foreground text.
- **FR-010**: `CostPointView` MUST be able to render numeric values from `-99` through `99`.
- **FR-011**: The point system MUST define `PowerPointModel` as the power value model for a card, location total, or other future power-bearing entity.
- **FR-012**: The point system MUST define `PowerPointView` as the rendering appearance of a `PowerPointModel` for a given entity.
- **FR-013**: Every playable card MUST have a `PowerPointView`.
- **FR-014**: Every shared location total MUST use a `PowerPointView`.
- **FR-015**: `PowerPointView` MUST combine an artful background with dynamic foreground text.
- **FR-016**: `PowerPointView` MUST be able to render numeric values from `-99` through `99`.
- **FR-017**: The game MUST support three shared board locations for point comparison, consistent with `007-gameplay-concepts`.
- **FR-018**: Each shared location MUST maintain a local player total and an opponent total.
- **FR-019**: Each shared location MUST hold up to four cards per player by default.
- **FR-020**: A player's location total MUST equal the sum of effective power for that player's revealed cards at that location, up to the default four-card capacity, plus any resolved location-level modifiers.
- **FR-021**: The top location number MUST represent the remote opponent total from the local player's point of view.
- **FR-022**: The bottom location number MUST represent the local player total from the local player's point of view.
- **FR-023**: A location MUST be controlled by the player with the higher resolved total at that location.
- **FR-024**: A tied location MUST have no controller.
- **FR-025**: An empty location with equal zero totals MUST have no controller.
- **FR-026**: The match winner MUST be the player controlling more locations after final scoring.
- **FR-027**: If controlled-location count is tied after final scoring, the match winner MUST be the player with higher total power across all locations.
- **FR-028**: If controlled-location count and total power across all locations are both tied after final scoring, the match result MUST be a draw.
- **FR-029**: The point system MUST support rounds 1 through 6 as the current match pacing model, while keeping concrete energy, draw, and turn-resolution rules out of this feature unless later clarified.
- **FR-030**: The point system MUST preserve the separation between CardDefinition base data, CardInstance match state, shared Location state, point models, point views, and future ability or modifier rules.
- **FR-031**: The point system MUST NOT require implementation of full deckbuilding, card drawing, CPU strategy, complete turn rules, card abilities, or final production UI in this feature.
- **FR-032**: Future location and card abilities MAY modify effective card power, location totals, card capacity, or control checks, but those modifiers MUST resolve into the same total and control model defined by this spec.

### Key Entities

| Entity | Description |
| ------ | ----------- |
| **Card Cost** | A card definition value used by future play-budget rules. It is displayed with the card but does not directly score points. |
| **Card Power** | A card definition value that becomes the card's base scoring contribution when the card is revealed at a location. |
| **CostPointModel** | The model for a cost point value on a card or future cost-bearing entity. |
| **CostPointView** | The rendering appearance of a CostPointModel for a specific entity, with an artful background and dynamic foreground text that supports values from `-99` through `99`. |
| **PowerPointModel** | The model for a power point value on a card, location total, or future power-bearing entity. |
| **PowerPointView** | The rendering appearance of a PowerPointModel for a specific entity, with an artful background and dynamic foreground text that supports values from `-99` through `99`. |
| **Effective Power** | A card instance's current scoring contribution after match-state modifiers are applied. |
| **Location Card Capacity** | The default number of cards one player may have at one shared location: four cards. |
| **Location Total** | One player's resolved point total at one shared location, typically the sum of up to four revealed cards plus resolved modifiers. |
| **Location Control** | The current result of comparing local and opponent totals at a location. |
| **Total Power Across Locations** | The sum of a player's resolved location totals across all three shared locations, used as the match tiebreaker. |
| **Match Outcome** | The final win, loss, or draw result after comparing controlled locations and then total power. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer can correctly explain the difference between card cost and card power after reading this spec once.
- **SC-002**: A reviewer can inspect any location state and identify the local player total, opponent total, and controller in under 10 seconds.
- **SC-003**: Given 10 sample location states, reviewers can determine the correct controller for at least 9 of them using only the rules in this spec.
- **SC-004**: Given 10 sample end-of-match states, reviewers can determine the correct match outcome for at least 9 of them using only the rules in this spec.
- **SC-005**: Given point values from `-99` through `99`, reviewers can confirm that cost and power point displays have a defined view contract for every value in that range.
- **SC-006**: The point system can be described without referencing implementation details, asset paths, rendering systems, or specific code modules.

## Assumptions

| Assumption | Rationale |
| ---------- | --------- |
| The local player's perspective controls top and bottom number meaning. | 007 already describes top and bottom location numbers without assigning them; local-bottom and opponent-top matches common board readability. |
| The top location number is the remote player's total and the bottom location number is the local player's total. | The provided Marvel Snap location example uses the top value for the remote player and bottom value for the local player. |
| Card cost is a future play-budget value rather than a scoring value. | 007 lists cost and power separately in CardDefinition gameplay data. |
| Card power contributes only after reveal by default. | 007 ties card instances to reveal status and hidden placement. |
| Cost and power values share a display range of `-99` through `99`. | The requested point views must support negative and positive dynamic values for cards and locations. |
| Each location can hold four cards per player by default. | The provided location example describes summing the power of up to four cards at a location. |
| The match uses three shared locations and six rounds. | 007 already defines three shared locations and a 1 through 6 round display. |
| Controlled locations decide the winner before total power tiebreaking. | This gives location control primary importance while preserving a deterministic tiebreaker. |
| CPU behavior, card draw, energy growth, and ability text are separate future features. | The user asked for point-system concepts, not a full playable ruleset. |
