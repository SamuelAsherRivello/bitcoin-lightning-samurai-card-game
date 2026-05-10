# Feature Specification: Card Flip

**Feature Branch**: `006-card-flip`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Update the Card UI add a button called Flip. When clicked, it animates the card to rotate from FrontFace (the current) to BackFace. The prototype shows one card only. Pressing `T` changes the active card front content; if the card back is visible, that front change is not obvious until the card is flipped face up again. A card front is specific to the active card definition and currently the existing front entries represent character cards; future cards may have other front content types. The CardBack lives in the CardStructure folder, is not specific to an individual card front, and is shared by the card series. The card back should be one shared design whose art style and color palette match the existing card fronts. The domain is superheroes: epic, big, and bold, but the back should have no words, no characters, and no clear symbology; it should be an abstract pattern that could be used across all fronts for the game. Use the provided trading card game back cover Pinterest board as inspiration for card-back composition, while moving away from medieval fantasy into a superhero theme. Future tabletop, box cover art, and main menu may fit this theme, but they are not designed in this feature. When clicking flip, the card will animate from its current angle to the current angle plus 180 in the y. The x and z rotation stay the same (continuing to be fed from the mouse position as it is now, if it is. However, you can't simply spin it. You need to recognize that at 90 degrees in the y (assuming facing directly at camera is 0 degrees, when the card is thinnest, the graphics need to change. CardFront = the current multi layer front for the active card definition. CardBack = one shared superhero-pattern card-series back."

## Clarifications

### Session 2026-05-10

- Q: What should happen if `Flip` is clicked while a flip animation is already in progress? -> A: Reverse direction from current progress.
- Q: What should CardBack look like for this feature? -> A: One shared superhero-pattern back matching the current front palette, with no words, characters, or clear symbols.
- Q: What does `T` change during this prototype? -> A: `T` changes the active card front definition; if CardBack is visible, the change is hidden until the card is flipped face up.
- Q: What owns CardBack conceptually? -> A: The card series owns the shared CardBack; individual card definitions own their CardFront content.
- Q: What is `CardBrowser` in this feature? -> A: `CardBrowser` remains the current prototype entry point, but it is not intended to be a final user-facing game surface.
- Q: What is `Card UI` in this feature? -> A: `Card UI` is a temporary prototype control surface, separate from DebugHUD, and is not final user-facing game UI.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Flip Card From Front To Back (Priority: P1)

A reviewer opens the current `CardBrowser` prototype entry point, uses the temporary Card UI, and clicks a new `Flip` button to animate the visible card from its current front-face presentation to the card backface.

**Why this priority**: The feature's primary value is direct reviewer control over seeing the card's two sides while preserving the single-card inspection flow.

**Independent Test**: Launch the `CardBrowser` prototype entry point, click `Flip` in the Card UI while the current front face is visible, and verify the card animates to a back-facing state without changing scene scope.

**Acceptance Scenarios**:

1. **Given** the current card front face is visible, **When** the reviewer clicks `Flip`, **Then** the card begins a smooth rotation toward its back face.
2. **Given** the flip animation completes from the front face, **When** the reviewer observes the card, **Then** the displayed face is the deck backface rather than the multi-layer front artwork.
3. **Given** the temporary Card UI is visible, **When** the reviewer looks for flip control, **Then** a clearly labeled `Flip` button is available in the Card UI.

---

### User Story 2 - Preserve Pointer-Driven Inspection During Flip (Priority: P1)

A reviewer can continue moving the pointer while flipping the card, and the card retains the current pointer-driven x and z orientation behavior while adding the flip rotation around y.

**Why this priority**: The flip should extend the existing inspection behavior rather than replacing the card's interactive tilt feel.

**Independent Test**: Move the pointer to several positions before and during a flip, click `Flip`, and verify the card keeps responding to pointer-driven inspection on the non-flip axes while the side transition progresses.

**Acceptance Scenarios**:

1. **Given** the card is tilted by pointer position, **When** the reviewer clicks `Flip`, **Then** the flip starts from the card's current apparent angle rather than snapping to a neutral orientation.
2. **Given** the flip is animating, **When** the reviewer moves the pointer, **Then** pointer-driven orientation continues to feed the card's x and z rotation behavior.
3. **Given** the flip completes, **When** the reviewer continues moving the pointer, **Then** the backface remains inspectable with the same pointer-driven feel as the front face.

---

### User Story 3 - Swap Face Graphics At Edge-On Point (Priority: P1)

A reviewer sees a believable face transition: the current multi-layer front remains visible until the card reaches its thinnest edge-on point, then the deck backface becomes visible for the second half of the flip.

**Why this priority**: A card flip reads incorrectly if the backface appears too early or the front artwork is simply spun through the full rotation.

**Independent Test**: Start a flip from a directly front-facing state and observe the card around the halfway point, verifying that the visual side changes at the edge-on moment.

**Acceptance Scenarios**:

1. **Given** the card starts with the front face aimed at the camera at 0 degrees y, **When** the flip reaches approximately 90 degrees y, **Then** the card appears thinnest and the visible graphics switch from CardFront to CardBack.
2. **Given** the flip has not yet reached the edge-on midpoint, **When** the reviewer observes the card, **Then** the CardFront graphics remain the active visible side.
3. **Given** the flip has passed the edge-on midpoint, **When** the reviewer observes the card, **Then** the CardBack graphics are the active visible side.

---

### User Story 4 - Use Shared Card-Series Back Design (Priority: P2)

A reviewer sees a single card-series back design that is independent of the active card front and visually belongs with the existing superhero card fronts.

**Why this priority**: The backface belongs to the card series rather than one specific card definition, so changing the active card front should not produce incompatible backs or mismatched art direction.

**Independent Test**: Toggle available card fronts with `T`, flip the card, and verify the same abstract superhero-pattern backface appears regardless of active front content.

**Acceptance Scenarios**:

1. **Given** any available card front is active, **When** the card is flipped to the back, **Then** the same card-series backface is shown.
2. **Given** the backface is visible, **When** the active card front changes, **Then** the backface remains the same card-series visual.
3. **Given** the prototype assets are inspected, **When** locating the backface artwork, **Then** it is treated as CardStructure/card-series presentation content rather than individual card-front content.
4. **Given** the backface is visible, **When** the reviewer evaluates its art direction, **Then** it reads as bold superhero-game pattern art with a palette compatible with the existing card fronts, without words, characters, or clear symbols.

---

### User Story 5 - Change Hidden Front While Face Down (Priority: P2)

A reviewer can press `T` while the card back is visible to change the active card front definition without revealing that change until the card is flipped face up.

**Why this priority**: This validates the future hidden-information model where a face-down card can change or be selected without exposing its front content.

**Independent Test**: Flip the card face down, press `T`, confirm the visible back remains unchanged, then flip face up and confirm the front content has changed.

**Acceptance Scenarios**:

1. **Given** CardBack is visible, **When** the reviewer presses `T`, **Then** the visible card remains CardBack and does not reveal the new CardFront.
2. **Given** CardBack is visible and `T` has changed the active card front, **When** the reviewer flips the card face up, **Then** the newly selected CardFront is visible.
3. **Given** CardFront is visible, **When** the reviewer presses `T`, **Then** the visible CardFront changes immediately because the card is face up.

### Edge Cases

- If the reviewer clicks `Flip` while a flip is already in progress, the card should reverse direction from the current animation progress without jitter, teleporting, or ending in an invalid face state.
- If the card is tilted strongly by pointer input, the face swap should still occur at the flip animation's edge-on midpoint rather than at an unrelated pointer tilt angle.
- If the reviewer flips from the backface, the same control should animate back toward the front face using the same midpoint face-swap rule.
- If the app loses focus or pointer movement pauses during a flip, the animation should continue smoothly or settle cleanly according to the existing app behavior.
- If the backface texture fails to load, the card should show a clear fallback rather than disappearing.
- If the temporary Card UI is hidden or disabled by existing debug controls, the flip state should remain valid and the card should not change faces unexpectedly.
- If `T` is pressed while CardBack is visible, the card should keep showing CardBack until a flip reveals CardFront.
- If future tabletop behavior is introduced later, this prototype's flip behavior should remain card-level behavior and should not imply tabletop placement or multi-card UI in this feature.
- If the final game entry point is introduced later, `CardBrowser` should remain understood as a prototype/developer browsing surface rather than the final user-facing game UI.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The Card UI MUST include a button labeled `Flip`.
- **FR-001a**: `CardBrowser` MUST remain the current project entry point for this feature's one-card prototype workflow.
- **FR-001b**: Card UI MUST remain a temporary prototype control surface separate from DebugHUD.
- **FR-002**: Activating `Flip` while the card front face is visible MUST animate the card to its back face.
- **FR-003**: Activating `Flip` while the card back face is visible MUST animate the card back to its front face.
- **FR-004**: Each flip activation MUST rotate the card's flip orientation by 180 degrees around the y axis from its current flip angle.
- **FR-005**: Pointer-driven inspection MUST continue to control the existing non-flip card orientation behavior during and after the flip, including x and z behavior where currently supported.
- **FR-006**: The flip animation MUST start from the card's current apparent orientation without snapping to a neutral angle.
- **FR-007**: The visible card side MUST change at the edge-on midpoint of the flip, equivalent to approximately 90 degrees of y rotation from a directly front-facing state.
- **FR-008**: Before the midpoint during a front-to-back flip, the active visible side MUST be CardFront.
- **FR-009**: After the midpoint during a front-to-back flip, the active visible side MUST be CardBack.
- **FR-010**: Before the midpoint during a back-to-front flip, the active visible side MUST be CardBack.
- **FR-011**: After the midpoint during a back-to-front flip, the active visible side MUST be CardFront.
- **FR-012**: CardFront MUST be the current multi-layer front face presentation from `005-card-polish`, applied to the active card definition.
- **FR-013**: CardBack MUST be one shared rectangular superhero-pattern backface for this feature phase.
- **FR-014**: CardBack MUST be treated as card-series/CardStructure visual content, not as individual card-front content.
- **FR-015**: The CardBack artwork MUST be stored with shared CardStructure assets.
- **FR-016**: Pressing `T` MUST change the active card front definition without changing the CardBack visual during this feature phase.
- **FR-017**: The card MUST maintain one centered inspectable card and MUST NOT introduce tabletop placement, gameplay, deck browsing, hand UI, location UI, multiple-card layout, dragging, scoring, turns, rules, or menu flow.
- **FR-018**: Repeated or mid-animation `Flip` activations MUST reverse direction from the current animation progress and resolve to a valid front-facing or back-facing state without visual corruption.
- **FR-019**: The card MUST remain visible and preserve its established proportions throughout the flip.
- **FR-020**: CardBack MUST use an art style and color palette compatible with the existing CardFront artwork.
- **FR-021**: CardBack MUST communicate a superhero-game tone that feels epic, big, and bold without depicting words, readable letters, characters, logos, or clear icon-like symbols.
- **FR-022**: CardBack MAY use abstract pattern language, border rhythm, energy shapes, foil-like contrast, and composition inspiration from trading-card back-cover references, but MUST NOT copy a specific referenced design.
- **FR-023**: CardBack MUST NOT define future tabletop, game box cover art, main menu art, or broader brand assets.
- **FR-024**: When CardBack is visible and `T` changes the active card front definition, the visible card MUST remain face down until the reviewer flips it face up.
- **FR-025**: When CardFront is visible and `T` changes the active card front definition, the visible front content MUST update to the active card definition.
- **FR-026**: Broader gameplay concepts such as Game, Player, Deck, hand, placed cards, shared board locations, and Table Top MUST remain in `007-gameplay-concepts` rather than becoming implementation scope for this feature.
- **FR-027**: This feature MUST NOT promote `CardBrowser` into final user-facing game UI; it remains a prototype/developer entry point.
- **FR-028**: This feature MUST NOT promote Card UI into final user-facing game UI and MUST NOT merge it with DebugHUD.

### Key Entities

- **Card UI**: The temporary prototype control surface that gains the `Flip` button; separate from DebugHUD and not intended as final user-facing game UI.
- **DebugHUD**: The existing debug overlay/control surface for diagnostics and debug toggles; separate from Card UI.
- **CardBrowser**: The current prototype entry point for viewing and inspecting one card; not intended to be the final user-facing game surface.
- **Flip Button**: The Card UI action that toggles the card between front-facing and back-facing presentation through animation.
- **CardSeries**: The shared set or collection identity that owns the common CardBack used across cards from that series.
- **CardDefinition**: Static card-front identity and content; currently represented by the prototype's switchable front entries.
- **CardFront**: The current front face for the active CardDefinition, including the multi-layer CardStructure and artwork presentation from `005-card-polish`.
- **CardBack**: The CardSeries-level backface, represented for this feature phase as one shared superhero-pattern rectangular visual that is not specific to an individual CardDefinition.
- **Flip Orientation**: The card's side-selection rotation state around the y axis, advancing by 180 degrees for each flip.
- **Face Swap Midpoint**: The edge-on point of a flip, approximately 90 degrees from front-facing, where active graphics switch between CardFront and CardBack.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of tested front-to-back attempts, clicking `Flip` starts a visible flip animation within one frame of UI activation.
- **SC-002**: In 100% of tested front-to-back attempts from a neutral front-facing state, CardFront remains visible before the edge-on midpoint and CardBack is visible after the midpoint.
- **SC-003**: In 100% of tested back-to-front attempts from a neutral back-facing state, CardBack remains visible before the edge-on midpoint and CardFront is visible after the midpoint.
- **SC-004**: In pointer movement tests during flip, the card continues responding to pointer-driven inspection without snapping to neutral in 100% of tested attempts.
- **SC-005**: In active-card-front toggle tests with at least two front entries available, CardBack remains visually unchanged in 100% of tested backface views.
- **SC-006**: In repeated-click tests, the card always settles into a valid front or back state within the documented animation duration and never disappears, jitters indefinitely, or shows both faces at once.
- **SC-007**: In scope review, the feature adds only CardBrowser one-card prototype flip behavior, active front switching, and backface presentation; no tabletop placement, gameplay, deck browsing, hand UI, location UI, multiple-card layout, dragging, scoring, turns, rules, or menu flow is present.
- **SC-008**: In art-direction review, at least 4 out of 5 reviewers identify CardBack as compatible with the existing superhero card-front palette and tone.
- **SC-009**: In content review, CardBack contains no words, readable letters, characters, logos, or clear icon-like symbols in 100% of inspected backface views.
- **SC-010**: In hidden-front tests, pressing `T` while CardBack is visible keeps CardBack visible in 100% of attempts, and the changed CardFront becomes visible only after flipping face up.

## Assumptions

- The initial visible side is CardFront.
- "Current angle plus 180 in the y" refers to the flip orientation layered onto the existing card inspection orientation.
- The existing pointer-driven inspection behavior remains the source of non-flip tilt during the feature.
- CardBack is one shared superhero-pattern rectangular visual, located with CardStructure assets and owned by the card series.
- The current switchable front entries stand in for CardDefinitions during this prototype; future CardDefinitions may include non-character front content.
- `CardBrowser` is currently the only entry point to the project, but it is not intended to be a final user-facing game surface.
- Card UI is temporary and separate from DebugHUD; this feature should not merge those surfaces or imply either is final game UI.
- The Pinterest board is used only as composition inspiration for trading-card backs; the game should translate that inspiration into a superhero tone rather than a medieval fantasy tone.
- Future tabletop, box cover art, and main menu visuals may share the eventual CardBack theme, but they are out of scope for this feature.
- The broader Game, Player, Deck, hand, placed card, shared location, and Table Top model is recorded in `007-gameplay-concepts`.
- Hidden placement and reveal timing are future gameplay concepts; this feature only proves that a card front can change while the shared back remains visible.
- The face swap is judged against flip animation progress, not absolute world rotation after pointer tilt is applied.
- The same `Flip` button toggles both front-to-back and back-to-front.
