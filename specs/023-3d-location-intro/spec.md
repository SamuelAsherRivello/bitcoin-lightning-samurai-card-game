# Feature Specification: 3D Location Intro

**Feature Branch**: `[023-3d-location-intro]`  
**Created**: 2026-05-17  
**Status**: Draft  
**Input**: User description: "Make a new plan without interrupting the current dirty files. The new spec is that we want the location to be a 3d asset called `location_bundle`. It will be a 3d rectangle with the background graphic on it. Then 2 point_views on top, and a colored border. The result will be the same look, but I want it 3d. Then when the game starts you first fade in location 01 from 0% opacity to 100% opacity and simultaneously shrink it from 150% of its destination size to 100% of its destination size. Do that over 1 second with easeout. Then wait 0.5 second. Then do the same for location 2. Then wait 0.5 second. Then do the same for location 3. The location must be in front of the world background, but below all cards. That relates to the depth or distance from camera."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Show Locations As 3D Bundles (Priority: P1)

A player sees the same three readable shared locations, but each location is presented as a 3D rectangular surface named `location_bundle` with the existing background graphic, two point views, and a colored border.

**Why this priority**: The feature exists to change the location presentation from flat UI-like rendering to a 3D board object while preserving the current visual meaning.

**Independent Test**: Start the game scene and verify each of the three shared locations still appears in its expected destination position, still shows its background graphic, still has top and bottom point views, and still has a colored border.

**Acceptance Scenarios**:

1. **Given** the game scene is visible, **When** the locations finish appearing, **Then** exactly three `location_bundle` presentations are visible across the board.
2. **Given** a location is visible, **When** the player inspects it, **Then** the location background graphic fills a 3D rectangular surface.
3. **Given** a location is visible, **When** the player reads its score areas, **Then** the two point views are visible on top of the location presentation.
4. **Given** a location is visible, **When** the player compares its open or closed state, **Then** the colored border remains visible and communicates the current state.

---

### User Story 2 - Reveal Locations In Sequence (Priority: P1)

A player entering the game sees the three locations arrive one at a time: location 01 appears first, location 02 appears after a short pause, and location 03 appears after the next short pause.

**Why this priority**: The requested start-of-game reveal sequence gives locations more presence and makes the new 3D presentation noticeable without changing gameplay rules.

**Independent Test**: Start a fresh game scene and time the location reveal sequence from the first visible animation through the third location reaching its final state.

**Acceptance Scenarios**:

1. **Given** the game scene starts, **When** the location intro begins, **Then** location 01 fades from 0% opacity to 100% opacity while shrinking from 150% of its destination size to 100% over 1 second with ease-out timing.
2. **Given** location 01 has reached full opacity and destination size, **When** 0.5 seconds pass, **Then** location 02 starts the same 1-second fade-and-shrink animation.
3. **Given** location 02 has reached full opacity and destination size, **When** 0.5 seconds pass, **Then** location 03 starts the same 1-second fade-and-shrink animation.
4. **Given** all three location animations have completed, **When** the player views the board, **Then** all locations remain at full opacity and destination size.

---

### User Story 3 - Preserve Board Layering (Priority: P1)

A player sees the world background behind the locations and all cards above the locations, so the new 3D location treatment does not obscure card play or board readability.

**Why this priority**: The location depth relationship is required for the new 3D presentation to integrate with the existing board, cards, and world background.

**Independent Test**: Start the game scene after location intro completion and verify that the world background never appears in front of a location, and cards never appear behind or inside the location surface when they should be readable above it.

**Acceptance Scenarios**:

1. **Given** a location is visible, **When** the world background is also visible, **Then** the location appears in front of the world background.
2. **Given** a card is in a hand or board position that visually overlaps a location, **When** the scene renders, **Then** the card appears above the location.
3. **Given** a location contains point views and a border, **When** cards are visible above the board, **Then** those location details remain below card presentation but readable when unobstructed.

### Edge Cases

| Edge Case | Expected Behavior |
| --------- | ----------------- |
| The game scene is restarted during the intro sequence | The sequence restarts from location 01 at 0% opacity and 150% size. |
| The game scene is entered after assets have already been cached or preloaded | The intro still plays with the same timing and final states. |
| A location background graphic is slow to become available | The location should not flash as an incomplete full-opacity rectangle; it should either wait for the needed visual or present a controlled placeholder during the same intro contract. |
| Point values change during or immediately after the intro | The point views remain attached to their location and show the current values without detaching or drifting. |
| A card overlaps a location during or after the intro | The card remains visually above the location. |
| A viewport or safe-area size changes before all three locations finish animating | Destination positions and sizes remain aligned with the current board layout. |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game scene MUST present each shared location as a 3D rectangular asset named `location_bundle`.
- **FR-002**: Each `location_bundle` MUST display the location's background graphic on the rectangular surface.
- **FR-003**: Each `location_bundle` MUST include exactly two point views, representing the two player totals already used by the location point system.
- **FR-004**: The two point views MUST appear on top of the location surface and remain readable at the location's final destination size.
- **FR-005**: Each `location_bundle` MUST include a colored border that remains visible at the final destination size.
- **FR-006**: The colored border MUST continue to communicate the same location state meaning as the current location border.
- **FR-007**: The 3D location presentation MUST preserve the existing final look of the board as closely as practical, except for the location surface now having 3D presence.
- **FR-008**: When the game scene starts, location 01 MUST begin at 0% opacity and 150% of its destination size.
- **FR-009**: Location 01 MUST animate to 100% opacity and 100% destination size over 1 second.
- **FR-010**: Location 01's animation timing MUST feel ease-out, with faster movement near the start and slower movement near the end.
- **FR-011**: Location 02 MUST wait until location 01 completes and then wait an additional 0.5 seconds before starting.
- **FR-012**: Location 02 MUST use the same 0% to 100% opacity and 150% to 100% size animation over 1 second with ease-out timing.
- **FR-013**: Location 03 MUST wait until location 02 completes and then wait an additional 0.5 seconds before starting.
- **FR-014**: Location 03 MUST use the same 0% to 100% opacity and 150% to 100% size animation over 1 second with ease-out timing.
- **FR-015**: The completed intro sequence MUST leave all three locations at 100% opacity and 100% destination size.
- **FR-016**: The location presentation MUST appear in front of the world background.
- **FR-017**: The location presentation MUST appear below all cards.
- **FR-018**: Location depth ordering MUST remain stable during the intro animation and after completion.
- **FR-019**: The feature MUST preserve the existing three-location board layout and destination positions.
- **FR-020**: The feature MUST preserve existing card readability, card interaction, and card layering above locations.
- **FR-021**: The feature MUST preserve existing point view meaning, including top and bottom location totals.
- **FR-022**: The feature MUST preserve existing location background identity and state coloring.
- **FR-023**: The feature MUST NOT change scoring rules, card placement rules, round rules, deck behavior, or location ability behavior.

### Key Entities

| Entity | Description |
| ------ | ----------- |
| **Location Bundle** | The 3D rectangular presentation of one shared location, including its background graphic, two point views, colored border, opacity, scale, destination size, and depth relationship to world and cards. |
| **Location Background Graphic** | The existing visual image associated with a location, now presented on the location bundle's rectangular surface. |
| **Location Point Views** | The two readable point displays shown on top of a location bundle, matching the existing top and bottom total meanings. |
| **Location Border** | The colored outline or frame treatment that communicates the location's current state. |
| **Location Intro Sequence** | The staged game-start animation that reveals location 01, waits, reveals location 02, waits, and reveals location 03. |
| **Location Destination State** | The final opacity, scale, size, position, and depth ordering for a location after its intro animation completes. |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On a fresh game scene start, location 01 reaches full opacity and destination size in 0.5 seconds, with a tolerance of 0.1 seconds.
- **SC-002**: Location 02 starts 0.5 seconds after location 01 completes, with a tolerance of 0.1 seconds.
- **SC-003**: Location 03 starts 0.5 seconds after location 02 completes, with a tolerance of 0.1 seconds.
- **SC-004**: After the sequence completes, a tester can identify all three locations, their background graphics, two point views, and colored borders within 5 seconds.
- **SC-005**: In visual review, every location appears in front of the world background and below cards in all normal game-scene states tested.
- **SC-006**: Existing card play, card readability, point totals, and location state meaning remain unchanged from the player's perspective.
- **SC-007**: At least 4 out of 5 testers describe the final location presentation as visually equivalent to the prior location layout but with clear 3D presence.

## Assumptions

| Assumption | Rationale |
| ---------- | --------- |
| The feature applies to the main game scene's three shared locations. | The request refers to location 01, 02, and 03 at game start. |
| `location_bundle` is the required durable presentation name for the new location object. | The user explicitly named the desired 3D asset. |
| The current top and bottom point view meanings remain unchanged. | The user asked for the same look, only made 3D. |
| The current colored border state meaning remains unchanged. | The user asked for a colored border without changing gameplay meaning. |
| The intro sequence plays when a game scene starts or restarts, not every time any asset reloads. | The requested behavior is tied to game start. |
| The 150% starting size scales around each location's destination center. | This preserves destination alignment while creating the requested shrink-in effect. |
| Destination positions derive from the existing aspect-ratio-safe board layout. | Project guidance requires visible 2D and 3D positions to derive from the safe game view. |
| Existing world, card, scoring, round, and deck behavior are out of scope. | The request is visual presentation and start-of-game animation only. |
